//! The source gateway mux: owned route table, fixed JSONPb codec and HTTP consumer.
//! Handler tables are the original application's HTTP routes, not effect providers.
use std::{collections::{BTreeMap, HashMap}, fmt, pin::Pin, sync::{Arc, RwLock}};
use cgp::prelude::*;
use bytes::Bytes;
use derive_deftly::Deftly;
use futures::{future::BoxFuture, Stream, StreamExt};
use prost_reflect::{DynamicMessage, MessageDescriptor};
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, utility_http::{header, set_header, HttpHandler, HttpRequest, ResponseWriter},
    utility_runtime::UtilityRuntimeContext,
    utility_gateway_json::{GatewayDecoder, GatewayJsonPb, GatewayValue},
    utility_declarations::derive_deftly_template_UtilityGatewayProviders};

const ERRORS: Errors = Errors::from_package("github.com/grpc-ecosystem/grpc-gateway/v2/runtime");
pub type PathParameters = BTreeMap<String, String>;
pub type GatewayHandler = Arc<dyn for<'a> Fn(&'a GatewayContext, &'a mut ResponseWriter,
    &'a mut HttpRequest, &'a GatewayCall) -> BoxFuture<'a, ()> + Send + Sync>;
pub type GatewayUnaryHandler = Arc<dyn for<'a> Fn(&'a GatewayContext, &'a GatewayCall,
    DynamicMessage) -> BoxFuture<'a, Result<GatewayValue, GatewayStatus>> + Send + Sync>;
pub type GatewayResponseStream = Pin<Box<dyn Stream<Item = Result<GatewayValue, GatewayStatus>> + Send>>;

#[derive(Clone)]
pub struct GatewayCall {
    pub path_parameters: PathParameters,
    pub pattern: GatewayPattern,
    pub form: Option<BTreeMap<String, Vec<String>>>,
    pub cancelled: CancellationToken,
}

/// The int opcode format emitted by the pinned grpc-gateway generator.
#[derive(Clone, Debug)]
pub struct GatewayPattern { operations: Vec<(i32, usize)>, pool: Vec<String>, variables: Vec<String>,
    stack_size: usize, tail_length: usize, verb: String }
impl GatewayPattern {
    pub fn new(version: i32, operations: &[i32], pool: Vec<String>, verb: String) -> Result<Self, BoxError> {
        let invalid = || ERRORS.New("invalid pattern");
        if version != 1 || operations.len() % 2 != 0 { return Err(invalid()); }
        let mut result = Self { operations: Vec::new(), pool, variables: Vec::new(), stack_size: 0, tail_length: 0, verb };
        let mut stack = 0_usize;
        let mut deep_seen = false;
        for pair in operations.chunks_exact(2) {
            let code = pair[0];
            let mut operand = pair[1] as usize;
            match code {
                0 => continue,
                1 => { stack += 1; if deep_seen { result.tail_length += 1; } },
                2 => {
                    if pair[1] < 0 || operand >= result.pool.len() { return Err(invalid()); }
                    stack += 1; if deep_seen { result.tail_length += 1; }
                }
                3 => { if deep_seen { return Err(invalid()); } deep_seen = true; stack += 1; },
                4 => { if pair[1] <= 0 { return Err(invalid()); } stack = stack.checked_sub(operand).ok_or_else(invalid)? + 1; },
                5 => {
                    if pair[1] < 0 || operand >= result.pool.len() { return Err(invalid()); }
                    result.variables.push(result.pool[operand].clone());
                    operand = result.variables.len() - 1;
                    stack = stack.checked_sub(1).ok_or_else(invalid)?;
                }
                _ => return Err(invalid()),
            }
            result.stack_size = result.stack_size.max(stack);
            result.operations.push((code, operand));
        }
        Ok(result)
    }
    pub fn verb(&self) -> &str { &self.verb }
    pub fn matches(&self, components: &[String], verb: &str) -> Option<PathParameters> {
        let mut components = components.to_vec();
        if self.verb != verb {
            if !self.verb.is_empty() { return None; }
            if let Some(last) = components.last_mut() { last.push(':'); last.push_str(verb); }
            else { components.push(format!(":{verb}")); }
        }
        let mut position = 0;
        let mut stack = Vec::with_capacity(self.stack_size);
        let mut captures = vec![String::new(); self.variables.len()];
        for (code, operand) in &self.operations {
            match code {
                1 | 2 => {
                    let component = components.get(position)?;
                    if *code == 2 && component != &self.pool[*operand] { return None; }
                    stack.push(component.clone()); position += 1;
                }
                3 => {
                    let end = components.len().checked_sub(self.tail_length)?;
                    if end < position { return None; }
                    stack.push(components[position..end].join("/")); position = end;
                }
                4 => { let start = stack.len() - operand; let value = stack[start..].join("/"); stack.truncate(start); stack.push(value); },
                5 => { captures[*operand] = stack.pop()?; },
                _ => unreachable!("validated pattern opcode"),
            }
        }
        if position < components.len() { return None; }
        Some(self.variables.iter().cloned().zip(captures).collect())
    }
}
impl fmt::Display for GatewayPattern {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stack: Vec<String> = Vec::new();
        for (code, operand) in &self.operations {
            match code {
                1 => stack.push("*".into()), 2 => stack.push(self.pool[*operand].clone()), 3 => stack.push("**".into()),
                4 => { let start = stack.len() - operand; let value = stack[start..].join("/"); stack.truncate(start); stack.push(value); },
                5 => { let value = stack.last_mut().expect("validated pattern capture"); *value = format!("{{{}={value}}}", self.variables[*operand]); },
                _ => unreachable!("validated pattern opcode"),
            }
        }
        write!(out, "/{}", stack.join("/"))?;
        if !self.verb.is_empty() { write!(out, ":{}", self.verb)?; }
        Ok(())
    }
}

#[derive(Clone)]
struct Route { pattern: GatewayPattern, handler: GatewayHandler }
#[derive(Default)]
pub struct GatewayState { routes: RwLock<HashMap<Bytes, Vec<Route>>> }
#[cgp_getter]
pub trait HasGatewayState { fn gateway_state(&self) -> &Arc<GatewayState>; }
#[cgp_getter]
pub trait HasGatewayRuntime { fn gateway_runtime(&self) -> &UtilityRuntimeContext; }

#[derive(Clone, HasField, Deftly)]
#[derive_deftly(UtilityGatewayProviders)]
pub struct GatewayContext { gateway_state: Arc<GatewayState>, gateway_runtime: UtilityRuntimeContext }
impl GatewayContext {
    pub fn handle(&self, method: impl AsRef<[u8]>, pattern: GatewayPattern, handler: GatewayHandler) {
        let mut routes = self.gateway_state().routes.write().unwrap_or_else(|error| error.into_inner());
        routes.entry(Bytes::copy_from_slice(method.as_ref())).or_default().insert(0, Route { pattern, handler });
    }
    /// Generated handlers can use handle directly to perform their own path,
    /// query, body and metadata mapping before calling a source RPC.
    pub fn handle_unary(&self, method: impl AsRef<[u8]>, pattern: GatewayPattern, input: MessageDescriptor, handler: GatewayUnaryHandler) {
        let route: GatewayHandler = Arc::new(move |context, writer, request, call| {
            let (input, handler) = (input.clone(), Arc::clone(&handler));
            Box::pin(async move {
                let mut message = DynamicMessage::new(input);
                let mut decoder = GatewayDecoder::new(request);
                if let Err(error) = context.decode_gateway(&mut decoder, request, &mut message).await {
                    let _ = context.write_gateway_error(writer, &GatewayStatus::new(3, error.to_string())).await;
                    return;
                }
                match context.call_gateway_unary(&handler, call, message).await {
                    Ok(value) => { let _ = context.forward_gateway_message(writer, &value).await; },
                    Err(status) => { let _ = context.write_gateway_error(writer, &status).await; },
                }
            })
        });
        self.handle(method, pattern, route);
    }
    /// Mount this actual handler in the existing source HTTP/TLS mux.
    pub fn as_http_handler(&self) -> HttpHandler {
        let context = self.clone();
        Arc::new(move |_runtime, writer, request| {
            let context = context.clone();
            Box::pin(async move { context.serve_gateway(writer, request).await; })
        })
    }
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
    fn decode_gateway<'a>(&'a self, decoder: &'a mut GatewayDecoder, request: &'a mut HttpRequest,
        target: &'a mut DynamicMessage) -> BoxFuture<'a, Result<bool, BoxError>>;
}
#[cgp_new_provider]
impl<Context: Sync> GatewayCodec<Context> for UseSourceGatewayJsonPb {
    fn marshal_gateway(_context: &Context, value: &GatewayValue) -> Result<Vec<u8>, BoxError> { GatewayJsonPb.marshal(value) }
    fn decode_gateway<'a>(_context: &'a Context, decoder: &'a mut GatewayDecoder, request: &'a mut HttpRequest,
        target: &'a mut DynamicMessage) -> BoxFuture<'a, Result<bool, BoxError>> { Box::pin(decoder.decode(request, target)) }
}
#[cgp_component(GatewayHandlerCaller)]
pub trait CanCallGatewayHandler: Sync {
    fn call_gateway_handler<'a>(&'a self, handler: &'a GatewayHandler, writer: &'a mut ResponseWriter,
        request: &'a mut HttpRequest, call: &'a GatewayCall) -> BoxFuture<'a, ()>;
    fn call_gateway_unary<'a>(&'a self, handler: &'a GatewayUnaryHandler, call: &'a GatewayCall,
        input: DynamicMessage) -> BoxFuture<'a, Result<GatewayValue, GatewayStatus>>;
}
#[cgp_new_provider]
impl GatewayHandlerCaller<GatewayContext> for CallSourceGatewayHandler {
    fn call_gateway_handler<'a>(context: &'a GatewayContext, handler: &'a GatewayHandler, writer: &'a mut ResponseWriter,
        request: &'a mut HttpRequest, call: &'a GatewayCall) -> BoxFuture<'a, ()> { handler(context, writer, request, call) }
    fn call_gateway_unary<'a>(context: &'a GatewayContext, handler: &'a GatewayUnaryHandler, call: &'a GatewayCall,
        input: DynamicMessage) -> BoxFuture<'a, Result<GatewayValue, GatewayStatus>> { handler(context, call, input) }
}
#[cgp_component(GatewayDispatcher)]
pub trait CanServeGateway: Sync {
    fn serve_gateway<'a>(&'a self, writer: &'a mut ResponseWriter, request: &'a mut HttpRequest) -> BoxFuture<'a, ()>;
}
#[cgp_new_provider]
impl GatewayDispatcher<GatewayContext> for DispatchSourceGateway {
    fn serve_gateway<'a>(context: &'a GatewayContext, writer: &'a mut ResponseWriter, request: &'a mut HttpRequest) -> BoxFuture<'a, ()> {
        Box::pin(dispatch(context, writer, request))
    }
}
async fn dispatch(context: &GatewayContext, writer: &mut ResponseWriter, request: &mut HttpRequest) {
    if !request.path.starts_with('/') { let _ = context.write_gateway_error(writer, &routing_status(400)).await; return; }
    let mut form = None;
    if is_path_length_fallback(request) {
        let method = header(&request.headers, "X-HTTP-Method-Override").filter(|value| !value.is_empty()).map(|value| String::from_utf8_lossy(&value).into_owned());
        if let Some(method) = method {
            match parse_form(request).await {
                Ok(value) => form = Some(value),
                Err(error) => { let _ = context.write_gateway_error(writer, &GatewayStatus::new(3, error.to_string())).await; return; },
            }
            request.set_method(method.to_uppercase());
        }
    }
    let path: Vec<_> = request.path[1..].split('/').map(str::to_owned).collect();
    let (same, others) = {
        let requested_method = request.method();
        let routes = context.gateway_state().routes.read().unwrap_or_else(|error| error.into_inner());
        (routes.get(&requested_method).cloned().unwrap_or_default(),
            routes.iter().filter(|(method, _)| **method != requested_method).map(|(method, routes)| (method.clone(), routes.clone())).collect::<Vec<_>>())
    };
    for route in same {
        let (parts, verb, empty_verb_path) = split_verb(&route.pattern, &path);
        if empty_verb_path { let _ = context.write_gateway_error(writer, &routing_status(404)).await; return; }
        if let Some(parameters) = route.pattern.matches(&parts, &verb) {
            let call = GatewayCall { path_parameters: parameters, pattern: route.pattern, form, cancelled: request.cancelled.clone() };
            context.call_gateway_handler(&route.handler, writer, request, &call).await;
            return;
        }
    }
    for (method, routes) in others {
        for route in routes {
            let (parts, verb, _) = split_verb(&route.pattern, &path);
            if let Some(parameters) = route.pattern.matches(&parts, &verb) {
                if method.as_ref() == b"GET" && is_path_length_fallback(request) {
                    if form.is_none() {
                        match parse_form(request).await {
                            Ok(value) => form = Some(value),
                            Err(error) => { let _ = context.write_gateway_error(writer, &GatewayStatus::new(3, error.to_string())).await; return; },
                        }
                    }
                    let call = GatewayCall { path_parameters: parameters, pattern: route.pattern, form, cancelled: request.cancelled.clone() };
                    context.call_gateway_handler(&route.handler, writer, request, &call).await;
                } else { let _ = context.write_gateway_error(writer, &routing_status(405)).await; }
                return;
            }
        }
    }
    let _ = context.write_gateway_error(writer, &routing_status(404)).await;
}
fn split_verb(pattern: &GatewayPattern, path: &[String]) -> (Vec<String>, String, bool) {
    let mut parts = path.to_vec();
    let mut verb = String::new();
    let last = parts.last_mut().expect("split path contains at least one component");
    if !pattern.verb().is_empty() {
        if let Some(prefix) = last.strip_suffix(&format!(":{}", pattern.verb())) {
            if prefix.is_empty() { return (parts, verb, true); }
            let prefix = prefix.to_owned(); verb = pattern.verb().to_owned(); *last = prefix;
        }
    }
    (parts, verb, false)
}
fn is_path_length_fallback(request: &HttpRequest) -> bool {
    request.method().as_ref() == b"POST" && header(&request.headers, "Content-Type").as_deref() == Some(b"application/x-www-form-urlencoded".as_slice())
}
async fn parse_form(request: &mut HttpRequest) -> Result<BTreeMap<String, Vec<String>>, BoxError> {
    // ParseForm's default max is ten MiB. Body values precede query values.
    const LIMIT: usize = 10 << 20;
    let read = request.read_body_all_limited(Some(LIMIT + 1)).await;
    if let Some(crate::utility_http::SourceReadError::Failure(error)) = read.error { return Err(error); }
    if read.data.len() > LIMIT { return Err(ERRORS.New("http: POST too large")); }
    let mut form = parse_query(&read.data)?;
    let query = request.url_log_value.get("RawQuery").and_then(serde_json::Value::as_str).unwrap_or_default();
    for (name, values) in parse_query(query.as_bytes())? { form.entry(name).or_default().extend(values); }
    Ok(form)
}
fn parse_query(query: &[u8]) -> Result<BTreeMap<String, Vec<String>>, BoxError> {
    let mut values: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for part in query.split(|byte| *byte == b'&').filter(|part| !part.is_empty()) {
        if part.contains(&b';') { return Err(ERRORS.New("invalid semicolon separator in query")); }
        let equal = part.iter().position(|byte| *byte == b'=');
        let (name, value) = equal.map_or((part, &[][..]), |index| (&part[..index], &part[index + 1..]));
        let name = unescape_query(name)?; let value = unescape_query(value)?;
        values.entry(name).or_default().push(value);
    }
    Ok(values)
}
fn unescape_query(bytes: &[u8]) -> Result<String, BoxError> {
    let mut result = Vec::with_capacity(bytes.len()); let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => result.push(b' '),
            b'%' => {
                let Some(pair) = bytes.get(index + 1..index + 3) else { return Err(ERRORS.New("invalid URL escape")); };
                let digit = |byte: u8| match byte { b'0'..=b'9' => Some(byte - b'0'), b'a'..=b'f' => Some(byte - b'a' + 10), b'A'..=b'F' => Some(byte - b'A' + 10), _ => None };
                let Some(high) = digit(pair[0]) else { return Err(ERRORS.New("invalid URL escape")); };
                let Some(low) = digit(pair[1]) else { return Err(ERRORS.New("invalid URL escape")); };
                result.push(high * 16 + low); index += 2;
            }
            byte => result.push(byte),
        }
        index += 1;
    }
    Ok(String::from_utf8_lossy(&result).into_owned())
}

#[derive(Clone, Debug)]
pub struct GatewayStatus { pub code: i32, pub message: String, pub details: Vec<DynamicMessage>, pub http_status: Option<u16> }
impl GatewayStatus {
    pub fn new(code: i32, message: impl Into<String>) -> Self { Self { code, message: message.into(), details: Vec::new(), http_status: None } }
    fn value(&self) -> GatewayValue {
        GatewayValue::Status { code: self.code, message: self.message.clone(), details: self.details.clone() }
    }
}
pub fn http_status_from_code(code: i32) -> u16 {
    match code { 0 => 200, 1 => 499, 2 | 13 | 15 => 500, 3 | 9 | 11 => 400, 4 => 504, 5 => 404,
        6 | 10 => 409, 7 => 403, 8 => 429, 12 => 501, 14 => 503, 16 => 401, _ => 500 }
}
fn routing_status(status: u16) -> GatewayStatus {
    match status { 400 => GatewayStatus::new(3, "Bad Request"), 405 => GatewayStatus::new(12, "Method Not Allowed"),
        404 => GatewayStatus::new(5, "Not Found"), _ => GatewayStatus::new(13, "Unexpected routing error") }
}
#[cgp_component(GatewayResponseWriter)]
pub trait CanWriteGatewayResponse: Sync {
    fn forward_gateway_message<'a>(&'a self, writer: &'a mut ResponseWriter, value: &'a GatewayValue) -> BoxFuture<'a, Result<(), BoxError>>;
    fn write_gateway_error<'a>(&'a self, writer: &'a mut ResponseWriter, status: &'a GatewayStatus) -> BoxFuture<'a, Result<(), BoxError>>;
    fn forward_gateway_stream<'a>(&'a self, writer: &'a mut ResponseWriter, stream: &'a mut GatewayResponseStream,
        cancelled: CancellationToken) -> BoxFuture<'a, Result<(), BoxError>>;
}
#[cgp_new_provider]
impl GatewayResponseWriter<GatewayContext> for WriteSourceGatewayResponse {
    fn forward_gateway_message<'a>(context: &'a GatewayContext, writer: &'a mut ResponseWriter, value: &'a GatewayValue) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(async move {
            set_header(&mut writer.headers, "Content-Type", GatewayJsonPb.content_type(value));
            match context.marshal_gateway(value) {
                Ok(bytes) => { writer.write(&bytes).await?; Ok(()) },
                Err(error) => context.write_gateway_error(writer, &GatewayStatus::new(2, error.to_string())).await,
            }
        })
    }
    fn write_gateway_error<'a>(context: &'a GatewayContext, writer: &'a mut ResponseWriter, status: &'a GatewayStatus) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(async move {
            writer.headers.remove("Trailer"); writer.headers.remove("Transfer-Encoding");
            set_header(&mut writer.headers, "Content-Type", "application/json");
            if status.code == 16 { set_header(&mut writer.headers, "Www-Authenticate", &status.message); }
            match context.marshal_gateway(&status.value()) {
                Ok(bytes) => { writer.write_header(status.http_status.unwrap_or_else(|| http_status_from_code(status.code))).await?; writer.write(&bytes).await?; },
                Err(_) => { writer.write_header(500).await?; writer.write(b"{\"code\": 13, \"message\": \"failed to marshal error message\"}").await?; },
            }
            Ok(())
        })
    }
    fn forward_gateway_stream<'a>(context: &'a GatewayContext, writer: &'a mut ResponseWriter, stream: &'a mut GatewayResponseStream,
        cancelled: CancellationToken) -> BoxFuture<'a, Result<(), BoxError>> {
        Box::pin(async move {
            set_header(&mut writer.headers, "Transfer-Encoding", "chunked");
            let mut wrote_header = false;
            loop {
                let value = tokio::select! { _ = cancelled.cancelled() => return Ok(()), value = stream.next() => value };
                let Some(value) = value else { return Ok(()); };
                let (bytes, error_status) = match value {
                    Ok(value) => {
                        if !wrote_header { set_header(&mut writer.headers, "Content-Type", GatewayJsonPb.content_type(&value)); }
                        let bytes = match value {
                            GatewayValue::HttpBody(body) => Ok(body.map_or_else(Vec::new, |body| body.data)),
                            GatewayValue::Null => context.marshal_gateway(&GatewayValue::Map(Some(BTreeMap::from([("error".into(), GatewayStatus::new(13, "empty response").value())])))),
                            value => context.marshal_gateway(&GatewayValue::Map(Some(BTreeMap::from([("result".into(), value)])))),
                        };
                        match bytes { Ok(bytes) => (bytes, None), Err(error) => {
                            let status = GatewayStatus::new(2, error.to_string());
                            (context.marshal_gateway(&GatewayValue::Map(Some(BTreeMap::from([("error".into(), status.value())]))))?, Some(status))
                        } }
                    }
                    Err(status) => (context.marshal_gateway(&GatewayValue::Map(Some(BTreeMap::from([("error".into(), status.value())]))))?, Some(status)),
                };
                if let Some(status) = &error_status {
                    if !wrote_header { set_header(&mut writer.headers, "Content-Type", "application/json"); writer.write_header(http_status_from_code(status.code)).await?; }
                }
                writer.write(&bytes).await?; writer.write(b"\n").await?;
                wrote_header = true;
                if error_status.is_some() { return Ok(()); }
                writer.flush().await?;
            }
        })
    }
}
