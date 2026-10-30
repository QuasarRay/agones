//! Agones HTTP/RPC bindings over native Rust HTTP, protobuf JSON and streams.
//! CGP selects codec, dispatch and RPC effects. The gateway owns an application
//! route table; the runtime task scope owns every mounted request future.
use std::{collections::BTreeMap, future::Future, pin::Pin, sync::{Arc, Mutex, RwLock}};
use axum::body::{Body, BodyDataStream};
use base64::{engine::general_purpose::{STANDARD, STANDARD_NO_PAD}, Engine};
use bytes::{Bytes, BytesMut};
use cgp::prelude::*;
use derive_deftly::Deftly;
use futures::{future::BoxFuture, Stream, StreamExt};
use hyper::{header::{HeaderMap, HeaderName, HeaderValue}, Method};
use prost::Message;
use prost_reflect::{DeserializeOptions, DescriptorPool, DynamicMessage, Kind, MessageDescriptor,
    ReflectMessage, SerializeOptions, Value as ProtoValue};
use regex::bytes::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Value};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use tonic::metadata::{Ascii, Binary, MetadataKey, MetadataMap, MetadataValue};
use crate::{errors::{BoxError, Errors}, logfields::LogEntry,
    utility_declarations::derive_deftly_template_UtilityGatewayProviders,
    utility_logging::{CanEmitSourceLog, HasSourceLogger, LogLevel},
    utility_runtime::UtilityRuntimeContext,
    utility_web::{set_header, HttpHandler, HttpRequest, ResponseWriter, SourceMux}};
pub use crate::utility_gateway_routes::{BodyBinding, GatewayRouteSpec, RpcMode};

pub type PathParameters = BTreeMap<&'static str, String>;
pub type GatewayHandler = Arc<dyn for<'a> Fn(&'a GatewayContext, &'a mut ResponseWriter,
    &'a mut HttpRequest, &'a GatewayCall) -> BoxFuture<'a, ()> + Send + Sync>;
pub type GatewayUnaryHandler = Arc<dyn for<'a> Fn(&'a GatewayContext, &'a GatewayCall,
    DynamicMessage) -> BoxFuture<'a, Result<GatewayReply, GatewayStatus>> + Send + Sync>;
pub type GatewayServerStreamHandler = Arc<dyn for<'a> Fn(&'a GatewayContext, &'a GatewayCall,
    DynamicMessage) -> BoxFuture<'a, Result<GatewayStreamReply, GatewayStatus>> + Send + Sync>;
pub type GatewayRequestStream = Pin<Box<dyn Stream<Item = Result<DynamicMessage, GatewayStatus>> + Send>>;
pub type GatewayClientStreamHandler = Arc<dyn for<'a> Fn(&'a GatewayContext, &'a GatewayCall,
    GatewayRequestStream) -> BoxFuture<'a, Result<GatewayReply, GatewayStatus>> + Send + Sync>;
pub type GatewayResponseStream = Pin<Box<dyn Stream<Item = Result<GatewayValue, GatewayStatus>> + Send>>;

#[derive(Clone, Debug)]
pub enum GatewayValue {
    Proto(DynamicMessage),
    Json(Value),
    HttpBody { content_type: String, data: Bytes },
}
impl GatewayValue {
    fn content_type(&self) -> &str {
        match self { Self::HttpBody { content_type, .. } => content_type, _ => "application/json" }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GatewayMetadata { pub headers: MetadataMap, pub trailers: MetadataMap }
#[derive(Clone, Debug)]
pub struct GatewayReply { pub value: GatewayValue, pub metadata: GatewayMetadata }
impl From<DynamicMessage> for GatewayReply {
    fn from(message: DynamicMessage) -> Self { Self { value: GatewayValue::Proto(message), metadata: GatewayMetadata::default() } }
}
pub struct GatewayStreamReply {
    pub headers: MetadataMap,
    pub stream: GatewayResponseStream,
    /// The RPC stream fills this owned data when its native trailers arrive.
    pub trailers: Arc<Mutex<MetadataMap>>,
}
#[derive(Clone, Debug)]
pub struct GatewayStatus {
    pub code: i32,
    pub message: String,
    pub details: Vec<DynamicMessage>,
    pub http_status: Option<u16>,
    pub metadata: GatewayMetadata,
}
impl GatewayStatus {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self { code, message: message.into(), details: Vec::new(), http_status: None, metadata: GatewayMetadata::default() }
    }
    fn json(&self) -> Result<Value, BoxError> {
        let details = self.details.iter().map(detail_json).collect::<Result<Vec<_>, _>>()?;
        Ok(serde_json::json!({"code": self.code, "message": self.message, "details": details}))
    }
}
impl From<tonic::Status> for GatewayStatus {
    fn from(status: tonic::Status) -> Self {
        let mut result = Self::new(status.code() as i32, status.message());
        result.metadata.headers = status.metadata().clone();
        // Tonic retains the encoded google.rpc.Status in grpc-status-details-bin.
        // Decode it through a descriptor when one is supplied by the caller.
        result
    }
}
pub fn http_status_from_code(code: i32) -> u16 {
    match code { 0 => 200, 1 => 499, 2 | 13 | 15 => 500, 3 | 9 | 11 => 400, 4 => 504, 5 => 404,
        6 | 10 => 409, 7 => 403, 8 => 429, 12 => 501, 14 => 503, 16 => 401, _ => 500 }
}

#[derive(Clone, Debug)]
pub struct GatewayCall {
    pub route: GatewayRouteSpec,
    pub path_parameters: PathParameters,
    pub metadata: MetadataMap,
    pub cancelled: CancellationToken,
    pub deadline: Option<Instant>,
    pub accepts_trailers: bool,
    form: Option<Vec<(String, String)>>,
}
impl GatewayCall {
    pub fn grpc_request<T>(&self, message: T) -> tonic::Request<T> {
        let mut request = tonic::Request::new(message);
        *request.metadata_mut() = self.metadata.clone();
        if let Some(deadline) = self.deadline { request.set_timeout(deadline.saturating_duration_since(Instant::now())); }
        request
    }
    pub async fn run<T>(&self, future: impl Future<Output = Result<T, GatewayStatus>>) -> Result<T, GatewayStatus> {
        let timeout = async {
            match self.deadline { Some(deadline) => tokio::time::sleep_until(deadline).await, None => futures::future::pending().await }
        };
        tokio::select! {
            result = future => result,
            _ = self.cancelled.cancelled() => Err(GatewayStatus::new(1, "request cancelled")),
            _ = timeout => Err(GatewayStatus::new(4, "context deadline exceeded")),
        }
    }
}

struct ProtoJson<'a>(&'a DynamicMessage);
impl Serialize for ProtoJson<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let options = SerializeOptions::new().use_proto_field_name(true).skip_default_fields(false);
        self.0.serialize_with_options(serializer, &options)
    }
}
fn detail_json(detail: &DynamicMessage) -> Result<Value, BoxError> {
    if detail.descriptor().full_name() == "google.protobuf.Any" { return Ok(serde_json::to_value(ProtoJson(detail))?); }
    let descriptor = detail.descriptor();
    let any = descriptor.parent_pool().get_message_by_name("google.protobuf.Any")
        .ok_or_else(|| Errors::default().New("google.protobuf.Any descriptor is missing"))?;
    let mut envelope = DynamicMessage::new(any);
    envelope.try_set_field_by_name("type_url", ProtoValue::String(format!("type.googleapis.com/{}", descriptor.full_name())))?;
    envelope.try_set_field_by_name("value", ProtoValue::Bytes(detail.encode_to_vec().into()))?;
    Ok(serde_json::to_value(ProtoJson(&envelope))?)
}
fn decode_message(descriptor: MessageDescriptor, bytes: &[u8]) -> Result<DynamicMessage, BoxError> {
    if bytes.trim_ascii().is_empty() { return Ok(DynamicMessage::new(descriptor)); }
    let mut decoder = serde_json::Deserializer::from_slice(bytes);
    let options = DeserializeOptions::new().deny_unknown_fields(false);
    Ok(DynamicMessage::deserialize_with_options(descriptor, &mut decoder, &options)?)
}
fn encode_value(value: &GatewayValue) -> Result<Vec<u8>, BoxError> {
    match value {
        GatewayValue::Proto(message) => Ok(serde_json::to_vec(&ProtoJson(message))?),
        GatewayValue::Json(value) => Ok(serde_json::to_vec(value)?),
        GatewayValue::HttpBody { data, .. } => Ok(data.to_vec()),
    }
}

#[derive(Clone)]
struct Route { spec: GatewayRouteSpec, method: Method, matcher: Regex, input: MessageDescriptor, handler: GatewayHandler }
#[derive(Default)]
pub struct GatewayState { routes: RwLock<Vec<Route>> }
#[cgp_getter]
pub trait HasGatewayState { fn gateway_state(&self) -> &Arc<GatewayState>; }
#[cgp_getter]
pub trait HasGatewayRuntime { fn gateway_runtime(&self) -> &UtilityRuntimeContext; }
#[derive(Clone, HasField, Deftly)]
#[derive_deftly(UtilityGatewayProviders)]
pub struct GatewayContext { gateway_state: Arc<GatewayState>, gateway_runtime: UtilityRuntimeContext }
impl GatewayContext {
    pub fn handle(&self, spec: GatewayRouteSpec, descriptors: &DescriptorPool, handler: GatewayHandler) -> Result<(), BoxError> {
        let service = descriptors.get_service_by_name(spec.service)
            .ok_or_else(|| Errors::default().New(format!("missing RPC service {}", spec.service)))?;
        let method = service.methods().find(|method| method.name() == spec.rpc)
            .ok_or_else(|| Errors::default().New(format!("missing RPC {}", spec.rpc_path())))?;
        let expected_mode = if method.is_client_streaming() { RpcMode::ClientStream }
            else if method.is_server_streaming() { RpcMode::ServerStream } else { RpcMode::Unary };
        if expected_mode != spec.mode { return Err(Errors::default().New(format!("RPC streaming mode differs for {}", spec.rpc_path()))); }
        // regex supplies native matching. The current Agones bindings have at
        // most one single-segment capture, including the beta action suffixes.
        let expression = match spec.capture {
            Some(binding) => format!("(?-u)^{}([^/]+){}$", regex::escape(binding.prefix), regex::escape(binding.suffix)),
            None => format!("^{}$", regex::escape(spec.path)),
        };
        let route = Route { spec, method: Method::from_bytes(spec.method.as_bytes())?,
            matcher: Regex::new(&expression)?, input: method.input(), handler };
        self.gateway_state.routes.write().unwrap_or_else(|error| error.into_inner()).insert(0, route);
        Ok(())
    }
    pub fn handle_unary(&self, spec: GatewayRouteSpec, descriptors: &DescriptorPool, handler: GatewayUnaryHandler) -> Result<(), BoxError> {
        if spec.mode != RpcMode::Unary { return Err(Errors::default().New("unary handler requires a unary route")); }
        self.handle(spec, descriptors, Arc::new(move |context, writer, request, call| {
            let handler = Arc::clone(&handler);
            Box::pin(async move {
                let result = call.run(async {
                    let input = context.decode_request(request, call).await.map_err(invalid_argument)?;
                    context.call_gateway_unary(&handler, call, input).await
                }).await;
                match result {
                    Ok(reply) => { let _ = context.forward_gateway_reply(writer, &reply, call.accepts_trailers).await; }
                    Err(status) => { let _ = context.write_gateway_error(writer, &status, call.accepts_trailers).await; }
                }
            })
        }))
    }
    pub fn handle_server_stream(&self, spec: GatewayRouteSpec, descriptors: &DescriptorPool, handler: GatewayServerStreamHandler) -> Result<(), BoxError> {
        if spec.mode != RpcMode::ServerStream { return Err(Errors::default().New("server stream handler requires a server stream route")); }
        self.handle(spec, descriptors, Arc::new(move |context, writer, request, call| {
            let handler = Arc::clone(&handler);
            Box::pin(async move {
                let result = call.run(async {
                    let input = context.decode_request(request, call).await.map_err(invalid_argument)?;
                    context.call_gateway_server_stream(&handler, call, input).await
                }).await;
                match result {
                    Ok(mut reply) => { let _ = context.forward_gateway_stream(writer, &mut reply, call).await; }
                    Err(status) => { let _ = context.write_gateway_error(writer, &status, call.accepts_trailers).await; }
                }
            })
        }))
    }
    pub fn handle_client_stream(&self, spec: GatewayRouteSpec, descriptors: &DescriptorPool, handler: GatewayClientStreamHandler) -> Result<(), BoxError> {
        if spec.mode != RpcMode::ClientStream { return Err(Errors::default().New("client stream handler requires a client stream route")); }
        self.handle(spec, descriptors, Arc::new(move |context, writer, request, call| {
            let handler = Arc::clone(&handler);
            Box::pin(async move {
                let descriptor = match context.input_descriptor(call) {
                    Ok(descriptor) => descriptor,
                    Err(error) => { let _ = context.write_gateway_error(writer, &GatewayStatus::new(13, error.to_string()), call.accepts_trailers).await; return; }
                };
                let stream = request_stream(request.take_body(), descriptor, call.cancelled.clone());
                let result = call.run(context.call_gateway_client_stream(&handler, call, stream)).await;
                match result {
                    Ok(reply) => { let _ = context.forward_gateway_reply(writer, &reply, call.accepts_trailers).await; }
                    Err(status) => { let _ = context.write_gateway_error(writer, &status, call.accepts_trailers).await; }
                }
            })
        }))
    }
    fn input_descriptor(&self, call: &GatewayCall) -> Result<MessageDescriptor, BoxError> {
        self.gateway_state.routes.read().unwrap_or_else(|error| error.into_inner()).iter()
            .find(|route| route.spec.service == call.route.service && route.spec.rpc == call.route.rpc)
            .map(|route| route.input.clone()).ok_or_else(|| Errors::default().New("RPC route was not registered"))
    }
    async fn decode_request(&self, request: &mut HttpRequest, call: &GatewayCall) -> Result<DynamicMessage, BoxError> {
        let descriptor = self.input_descriptor(call)?;
        let bytes = if call.route.body == BodyBinding::None { Bytes::new() } else { request.read_body().await? };
        let mut message = match call.route.body {
            BodyBinding::All => self.decode_gateway(descriptor.clone(), &bytes)?,
            BodyBinding::None => DynamicMessage::new(descriptor.clone()),
            BodyBinding::Field(name) => {
                let field = descriptor.get_field_by_name(name).or_else(|| descriptor.get_field_by_json_name(name))
                    .ok_or_else(|| Errors::default().New(format!("missing body field {name}")))?;
                let Kind::Message(body_descriptor) = field.kind() else { return Err(Errors::default().New("Agones body binding must be a message")); };
                let mut message = DynamicMessage::new(descriptor.clone());
                message.try_set_field(&field, ProtoValue::Message(self.decode_gateway(body_descriptor.clone(), &bytes)?))?;
                if call.route.infer_list_mask {
                    let paths = list_body_mask(&body_descriptor, &bytes)?;
                    set_field_mask(&mut message, &paths)?;
                }
                message
            }
        };
        // Annotation precedence: the URL identity wins over the JSON body.
        for (name, value) in &call.path_parameters { set_path_string(&mut message, name, value)?; }
        if call.route.infer_list_mask {
            let mut query: Vec<_> = url::form_urlencoded::parse(request.uri().query().unwrap_or_default().as_bytes()).into_owned().collect();
            if let Some(form) = &call.form { query.extend(form.iter().cloned()); }
            if let Some((_, value)) = query.iter().find(|(name, _)| matches!(name.as_str(), "update_mask" | "updateMask")) {
                set_field_mask(&mut message, &value.split(',').map(str::to_owned).collect::<Vec<_>>())?;
            }
        }
        Ok(message)
    }
    pub fn as_http_handler(&self) -> HttpHandler {
        let context = self.clone();
        Arc::new(move |_runtime, writer, request| {
            let context = context.clone(); Box::pin(async move { context.serve_gateway(writer, request).await; })
        })
    }
    pub fn mount(&self, mux: &SourceMux, prefix: &str) { mux.handle(prefix, self.as_http_handler()); }
}
fn invalid_argument(error: BoxError) -> GatewayStatus { GatewayStatus::new(3, error.to_string()) }
fn set_path_string(message: &mut DynamicMessage, path: &str, text: &str) -> Result<(), BoxError> {
    let (name, rest) = path.split_once('.').map_or((path, None), |(name, rest)| (name, Some(rest)));
    let descriptor = message.descriptor();
    let field = descriptor.get_field_by_name(name).or_else(|| descriptor.get_field_by_json_name(name))
        .ok_or_else(|| Errors::default().New(format!("missing path field {path}")))?;
    let value = match rest {
        None => ProtoValue::String(text.to_owned()),
        Some(rest) => {
            let Kind::Message(descriptor) = field.kind() else { return Err(Errors::default().New(format!("path field {path} is not a message"))); };
            let mut child = match message.get_field(&field).as_ref() {
                ProtoValue::Message(child) => child.clone(), _ => DynamicMessage::new(descriptor),
            };
            set_path_string(&mut child, rest, text)?; ProtoValue::Message(child)
        }
    };
    message.try_set_field(&field, value)?; Ok(())
}
fn list_body_mask(descriptor: &MessageDescriptor, bytes: &[u8]) -> Result<Vec<String>, BoxError> {
    if bytes.trim_ascii().is_empty() { return Ok(Vec::new()); }
    let mut decoder = serde_json::Deserializer::from_slice(bytes);
    let value = Value::deserialize(&mut decoder)?;
    let Some(object) = value.as_object() else { return Ok(Vec::new()); };
    // The Agones beta List has scalar/repeated fields only. Descriptor names
    // keep aliases canonical and reject unknown body fields in this mask path.
    let mut paths = object.keys().map(|name| descriptor.get_field_by_name(name).or_else(|| descriptor.get_field_by_json_name(name))
        .map(|field| field.name().to_owned()).ok_or_else(|| Errors::default().New(format!("could not find field {name:?} in {:?}", descriptor.full_name()))))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort(); Ok(paths)
}
fn set_field_mask(message: &mut DynamicMessage, paths: &[String]) -> Result<(), BoxError> {
    let field = message.descriptor().get_field_by_name("update_mask")
        .ok_or_else(|| Errors::default().New("missing update_mask field"))?;
    let Kind::Message(descriptor) = field.kind() else { return Err(Errors::default().New("update_mask is not a message")); };
    let mut mask = DynamicMessage::new(descriptor);
    mask.try_set_field_by_name("paths", ProtoValue::List(paths.iter().cloned().map(ProtoValue::String).collect()))?;
    message.try_set_field(&field, ProtoValue::Message(mask))?; Ok(())
}

struct RequestStreamState { body: BodyDataStream, buffered: BytesMut, ended: bool, descriptor: MessageDescriptor, cancelled: CancellationToken }
fn request_stream(body: Body, descriptor: MessageDescriptor, cancelled: CancellationToken) -> GatewayRequestStream {
    let state = RequestStreamState { body: body.into_data_stream(), buffered: BytesMut::new(), ended: false, descriptor, cancelled };
    Box::pin(futures::stream::try_unfold(state, |mut state| async move {
        loop {
            let mut values = serde_json::Deserializer::from_slice(&state.buffered).into_iter::<Box<RawValue>>();
            match values.next() {
                Some(Ok(raw)) => {
                    let consumed = values.byte_offset();
                    let message = decode_message(state.descriptor.clone(), raw.get().as_bytes()).map_err(invalid_argument)?;
                    let _ = state.buffered.split_to(consumed);
                    return Ok(Some((message, state)));
                }
                Some(Err(error)) if error.is_eof() && !state.ended => (),
                Some(Err(error)) => return Err(GatewayStatus::new(3, error.to_string())),
                None => { state.buffered.clear(); if state.ended { return Ok(None); } }
            }
            let next = tokio::select! {
                value = state.body.next() => value,
                _ = state.cancelled.cancelled() => return Err(GatewayStatus::new(1, "request cancelled")),
            };
            match next {
                Some(Ok(bytes)) => state.buffered.extend_from_slice(&bytes),
                Some(Err(error)) => return Err(GatewayStatus::new(3, error.to_string())),
                None => state.ended = true,
            }
        }
    }))
}

#[cgp_component(SourceServerMuxFactory)]
pub trait CanCreateSourceServerMux { fn new_source_server_mux(&self) -> GatewayContext; }
#[cgp_new_provider]
impl SourceServerMuxFactory<UtilityRuntimeContext> for CreateNativeSourceServerMux {
    fn new_source_server_mux(context: &UtilityRuntimeContext) -> GatewayContext {
        GatewayContext { gateway_state: Arc::default(), gateway_runtime: context.clone() }
    }
}
#[cgp_component(GatewayCodec)]
pub trait CanUseGatewayCodec: Sync {
    fn marshal_gateway(&self, value: &GatewayValue) -> Result<Vec<u8>, BoxError>;
    fn decode_gateway(&self, descriptor: MessageDescriptor, bytes: &[u8]) -> Result<DynamicMessage, BoxError>;
}
#[cgp_new_provider]
impl<Context: Sync> GatewayCodec<Context> for UseNativeProtobufJson {
    fn marshal_gateway(_context: &Context, value: &GatewayValue) -> Result<Vec<u8>, BoxError> { encode_value(value) }
    fn decode_gateway(_context: &Context, descriptor: MessageDescriptor, bytes: &[u8]) -> Result<DynamicMessage, BoxError> { decode_message(descriptor, bytes) }
}
#[cgp_component(GatewayHandlerCaller)]
pub trait CanCallGatewayHandler: Sync {
    fn call_gateway_handler<'a>(&'a self, handler: &'a GatewayHandler, writer: &'a mut ResponseWriter, request: &'a mut HttpRequest, call: &'a GatewayCall) -> BoxFuture<'a, ()>;
    fn call_gateway_unary<'a>(&'a self, handler: &'a GatewayUnaryHandler, call: &'a GatewayCall, input: DynamicMessage) -> BoxFuture<'a, Result<GatewayReply, GatewayStatus>>;
    fn call_gateway_server_stream<'a>(&'a self, handler: &'a GatewayServerStreamHandler, call: &'a GatewayCall, input: DynamicMessage) -> BoxFuture<'a, Result<GatewayStreamReply, GatewayStatus>>;
    fn call_gateway_client_stream<'a>(&'a self, handler: &'a GatewayClientStreamHandler, call: &'a GatewayCall, input: GatewayRequestStream) -> BoxFuture<'a, Result<GatewayReply, GatewayStatus>>;
}
#[cgp_new_provider]
impl GatewayHandlerCaller<GatewayContext> for CallNativeGatewayHandler {
    fn call_gateway_handler<'a>(context: &'a GatewayContext, handler: &'a GatewayHandler, writer: &'a mut ResponseWriter, request: &'a mut HttpRequest, call: &'a GatewayCall) -> BoxFuture<'a, ()> { handler(context, writer, request, call) }
    fn call_gateway_unary<'a>(context: &'a GatewayContext, handler: &'a GatewayUnaryHandler, call: &'a GatewayCall, input: DynamicMessage) -> BoxFuture<'a, Result<GatewayReply, GatewayStatus>> { handler(context, call, input) }
    fn call_gateway_server_stream<'a>(context: &'a GatewayContext, handler: &'a GatewayServerStreamHandler, call: &'a GatewayCall, input: DynamicMessage) -> BoxFuture<'a, Result<GatewayStreamReply, GatewayStatus>> { handler(context, call, input) }
    fn call_gateway_client_stream<'a>(context: &'a GatewayContext, handler: &'a GatewayClientStreamHandler, call: &'a GatewayCall, input: GatewayRequestStream) -> BoxFuture<'a, Result<GatewayReply, GatewayStatus>> { handler(context, call, input) }
}
#[cgp_component(GatewayDispatcher)]
pub trait CanServeGateway: Sync { fn serve_gateway<'a>(&'a self, writer: &'a mut ResponseWriter, request: &'a mut HttpRequest) -> BoxFuture<'a, ()>; }
#[cgp_new_provider]
impl GatewayDispatcher<GatewayContext> for DispatchNativeGateway {
    fn serve_gateway<'a>(context: &'a GatewayContext, writer: &'a mut ResponseWriter, request: &'a mut HttpRequest) -> BoxFuture<'a, ()> { Box::pin(dispatch(context, writer, request)) }
}
async fn dispatch(context: &GatewayContext, writer: &mut ResponseWriter, request: &mut HttpRequest) {
    let result = select_call(context, request).await;
    match result {
        Ok((handler, call)) => context.call_gateway_handler(&handler, writer, request, &call).await,
        Err(status) => { let _ = context.write_gateway_error(writer, &status, false).await; }
    }
}
async fn select_call(context: &GatewayContext, request: &mut HttpRequest) -> Result<(GatewayHandler, GatewayCall), GatewayStatus> {
    let path = request.path();
    if !path.starts_with(b"/") { return Err(GatewayStatus::new(3, "Bad Request")); }
    let form_fallback = request.method() == Method::POST && request.headers.get("content-type")
        .is_some_and(|value| value == "application/x-www-form-urlencoded");
    let mut method = request.method().clone();
    let mut form = None;
    if form_fallback {
        if let Some(value) = request.headers.get("x-http-method-override").filter(|value| !value.is_empty()) {
            method = Method::from_bytes(value.as_bytes().to_ascii_uppercase().as_slice()).map_err(|error| GatewayStatus::new(3, error.to_string()))?;
        }
        let bytes = request.read_body().await.map_err(invalid_argument)?;
        form = Some(url::form_urlencoded::parse(&bytes).into_owned().collect());
    }
    let routes = context.gateway_state.routes.read().unwrap_or_else(|error| error.into_inner()).clone();
    let mut matched_path = false;
    let mut selected = None;
    for route in &routes {
        if route.matcher.is_match(&path) {
            matched_path = true;
            if route.method == method { selected = Some(route); break; }
        }
    }
    if selected.is_none() && form_fallback {
        selected = routes.iter().find(|route| route.method == Method::GET && route.matcher.is_match(&path));
    }
    let Some(route) = selected else {
        return Err(if matched_path { GatewayStatus::new(12, "Method Not Allowed") } else { GatewayStatus::new(5, "Not Found") });
    };
    let mut path_parameters = PathParameters::new();
    if let Some(binding) = route.spec.capture {
        let captures = route.matcher.captures(&path).expect("selected route matched");
        let value = std::str::from_utf8(captures.get(1).expect("route declares its capture").as_bytes())
            .map_err(|error| GatewayStatus::new(3, error.to_string()))?;
        path_parameters.insert(binding.field, value.to_owned());
    }
    let accepts_trailers = request.headers.get_all("te").iter().filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(',')).any(|value| value.trim().eq_ignore_ascii_case("trailers"));
    Ok((Arc::clone(&route.handler), GatewayCall { route: route.spec, path_parameters,
        metadata: incoming_metadata(context, request)?, cancelled: request.cancelled.clone(),
        deadline: request_deadline(request)?, accepts_trailers, form }))
}
fn request_deadline(request: &HttpRequest) -> Result<Option<Instant>, GatewayStatus> {
    let Some(value) = request.headers.get("grpc-timeout") else { return Ok(None); };
    let invalid = || GatewayStatus::new(3, format!("invalid grpc-timeout: {}", String::from_utf8_lossy(value.as_bytes())));
    let text = value.to_str().map_err(|_| invalid())?;
    if !(2..=9).contains(&text.len()) { return Err(invalid()); }
    let (digits, unit) = text.split_at(text.len() - 1);
    if !digits.bytes().all(|byte| byte.is_ascii_digit()) { return Err(invalid()); }
    let count: u32 = digits.parse().map_err(|_| invalid())?;
    let unit = match unit { "H" => std::time::Duration::from_secs(3600), "M" => std::time::Duration::from_secs(60),
        "S" => std::time::Duration::from_secs(1), "m" => std::time::Duration::from_millis(1),
        "u" => std::time::Duration::from_micros(1), "n" => std::time::Duration::from_nanos(1), _ => return Err(invalid()) };
    if count == 0 { return Ok(None); }
    let timeout = unit.checked_mul(count).ok_or_else(invalid)?;
    Instant::now().checked_add(timeout).map(Some).ok_or_else(invalid)
}
fn incoming_metadata(context: &GatewayContext, request: &HttpRequest) -> Result<MetadataMap, GatewayStatus> {
    let mut metadata = MetadataMap::new();
    for (name, value) in &request.headers {
        let name = name.as_str();
        if matches!(name, "x-forwarded-for" | "x-forwarded-host") { continue; }
        if name == "authorization" { append_ascii(&mut metadata, name, value.as_bytes()); }
        let key = if let Some(key) = name.strip_prefix("grpc-metadata-") { Some(key.to_owned()) }
            else if permanent_header(name) { Some(format!("grpcgateway-{name}")) } else { None };
        let Some(key) = key else { continue; };
        let accepted = if key.ends_with("-bin") {
            let key = match MetadataKey::<Binary>::from_bytes(key.as_bytes()) { Ok(key) => key, Err(_) => continue };
            let bytes = STANDARD.decode(value.as_bytes()).or_else(|_| STANDARD_NO_PAD.decode(value.as_bytes()))
                .map_err(|error| GatewayStatus::new(3, format!("invalid binary header {name}: {error}")))?;
            metadata.append_bin(key, MetadataValue::from_bytes(&bytes)); true
        } else { append_ascii(&mut metadata, &key, value.as_bytes()) };
        if !accepted {
            let logger = LogEntry::from_logger(Arc::clone(context.gateway_runtime.source_logger())).with_field("header", name);
            context.gateway_runtime.emit_source_log(&logger, LogLevel::Debug, "Skipping invalid gRPC metadata header");
        }
    }
    let host = request.headers.get("x-forwarded-host").map(|value| value.as_bytes()).unwrap_or_else(|| request.host().as_bytes());
    if !host.is_empty() { append_ascii(&mut metadata, "x-forwarded-host", host); }
    let mut forwarded: Vec<_> = request.headers.get_all("x-forwarded-for").iter().filter_map(|value| value.to_str().ok()).map(str::to_owned).collect();
    if let Some(peer) = request.peer() { forwarded.push(peer.ip().to_string()); }
    if !forwarded.is_empty() { append_ascii(&mut metadata, "x-forwarded-for", forwarded.join(", ").as_bytes()); }
    Ok(metadata)
}
fn append_ascii(metadata: &mut MetadataMap, key: &str, bytes: &[u8]) -> bool {
    match (MetadataKey::<Ascii>::from_bytes(key.as_bytes()), MetadataValue::<Ascii>::try_from(bytes)) {
        (Ok(key), Ok(value)) => { metadata.append(key, value); true }, _ => false,
    }
}
fn permanent_header(name: &str) -> bool {
    matches!(name, "accept" | "accept-charset" | "accept-language" | "accept-ranges" | "authorization" |
        "cache-control" | "content-type" | "cookie" | "date" | "expect" | "from" | "host" | "if-match" |
        "if-modified-since" | "if-none-match" | "if-schedule-tag-match" | "if-unmodified-since" | "max-forwards" |
        "origin" | "pragma" | "referer" | "user-agent" | "via" | "warning")
}

fn metadata_headers(metadata: &MetadataMap, prefix: &str) -> Result<HeaderMap, BoxError> {
    let mut headers = HeaderMap::new();
    for (name, value) in &metadata.clone().into_headers() {
        headers.append(HeaderName::from_bytes(format!("{prefix}{}", name.as_str()).as_bytes())?, value.clone());
    }
    Ok(headers)
}
fn prepare_metadata(writer: &mut ResponseWriter, metadata: &GatewayMetadata, trailers: bool) -> Result<HeaderMap, BoxError> {
    let headers = metadata_headers(&metadata.headers, "grpc-metadata-")?;
    for name in headers.keys() { for value in headers.get_all(name) { writer.headers.append(name.clone(), value.clone()); } }
    let trailers = if trailers { metadata_headers(&metadata.trailers, "grpc-trailer-")? } else { HeaderMap::new() };
    for name in trailers.keys() { writer.headers.append(hyper::header::TRAILER, HeaderValue::from_str(name.as_str())?); }
    Ok(trailers)
}
#[cgp_component(GatewayResponseWriter)]
pub trait CanWriteGatewayResponse: Sync {
    fn forward_gateway_reply<'a>(&'a self, writer: &'a mut ResponseWriter, reply: &'a GatewayReply, trailers: bool) -> BoxFuture<'a, Result<(), BoxError>>;
    fn write_gateway_error<'a>(&'a self, writer: &'a mut ResponseWriter, status: &'a GatewayStatus, trailers: bool) -> BoxFuture<'a, Result<(), BoxError>>;
    fn forward_gateway_stream<'a>(&'a self, writer: &'a mut ResponseWriter, reply: &'a mut GatewayStreamReply, call: &'a GatewayCall) -> BoxFuture<'a, Result<(), BoxError>>;
}
#[cgp_new_provider]
impl GatewayResponseWriter<GatewayContext> for WriteNativeGatewayResponse {
    fn forward_gateway_reply<'a>(context: &'a GatewayContext, writer: &'a mut ResponseWriter, reply: &'a GatewayReply, trailers: bool) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(async move {
            let bytes = match context.marshal_gateway(&reply.value) {
                Ok(bytes) => bytes,
                Err(error) => return context.write_gateway_error(writer, &GatewayStatus::new(2, error.to_string()), trailers).await,
            };
            let trailers = prepare_metadata(writer, &reply.metadata, trailers)?;
            set_header(&mut writer.headers, "content-type", reply.value.content_type())?;
            writer.write(&bytes).await?;
            if !trailers.is_empty() { writer.write_trailers(trailers).await?; }
            Ok(())
        })
    }
    fn write_gateway_error<'a>(_context: &'a GatewayContext, writer: &'a mut ResponseWriter, status: &'a GatewayStatus, trailers: bool) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(async move {
            writer.headers.remove("trailer"); writer.headers.remove("transfer-encoding");
            set_header(&mut writer.headers, "content-type", "application/json")?;
            if status.code == 16 { set_header(&mut writer.headers, "www-authenticate", &status.message)?; }
            let trailers = prepare_metadata(writer, &status.metadata, trailers)?;
            match status.json().and_then(|value| Ok(serde_json::to_vec(&value)?)) {
                Ok(bytes) => { writer.write_header(status.http_status.unwrap_or_else(|| http_status_from_code(status.code))).await?; writer.write(&bytes).await?; }
                Err(_) => { writer.write_header(500).await?; writer.write(b"{\"code\":13,\"message\":\"failed to marshal error message\"}").await?; }
            }
            if !trailers.is_empty() { writer.write_trailers(trailers).await?; }
            Ok(())
        })
    }
    fn forward_gateway_stream<'a>(context: &'a GatewayContext, writer: &'a mut ResponseWriter, reply: &'a mut GatewayStreamReply, call: &'a GatewayCall) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(async move {
            let initial = GatewayMetadata { headers: reply.headers.clone(),
                trailers: reply.trailers.lock().unwrap_or_else(|error| error.into_inner()).clone() };
            prepare_metadata(writer, &initial, call.accepts_trailers)?;
            let mut committed = false;
            loop {
                let item = call.run(async { Ok(reply.stream.next().await) }).await;
                let item = match item { Ok(item) => item, Err(status) => Some(Err(status)) };
                let Some(item) = item else {
                    if call.accepts_trailers {
                        let trailers = reply.trailers.lock().unwrap_or_else(|error| error.into_inner()).clone();
                        let trailers = metadata_headers(&trailers, "grpc-trailer-")?;
                        if !trailers.is_empty() { writer.write_trailers(trailers).await?; }
                    }
                    return Ok(());
                };
                let value = match item {
                    Ok(value) => value,
                    Err(status) => {
                        set_header(&mut writer.headers, "content-type", "application/json")?;
                        if !committed { writer.write_header(http_status_from_code(status.code)).await?; }
                        let mut bytes = serde_json::to_vec(&serde_json::json!({"error": status.json()?}))?;
                        bytes.push(b'\n'); writer.write(&bytes).await?; return Ok(());
                    }
                };
                if !committed { set_header(&mut writer.headers, "content-type", value.content_type())?; }
                let encoded = match &value {
                    GatewayValue::HttpBody { data, .. } => Ok(data.to_vec()),
                    _ => context.marshal_gateway(&value).and_then(|bytes| {
                        let value: Box<RawValue> = serde_json::from_slice(&bytes)?;
                        Ok(serde_json::to_vec(&serde_json::json!({"result": value}))?)
                    }),
                };
                let mut bytes = match encoded {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        let status = GatewayStatus::new(2, error.to_string());
                        if !committed { writer.write_header(500).await?; }
                        let mut bytes = serde_json::to_vec(&serde_json::json!({"error": status.json()?}))?;
                        bytes.push(b'\n'); writer.write(&bytes).await?; return Ok(());
                    }
                };
                bytes.push(b'\n'); writer.write(&bytes).await?; writer.flush().await?;
                committed = true;
            }
        })
    }
}
