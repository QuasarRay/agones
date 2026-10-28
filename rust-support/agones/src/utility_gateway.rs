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
    utility_runtime::UtilityRuntimeContext, utility_http::{SourceValues, CanUppercaseSourceString},
    utility_gateway_json::{GatewayDecoder, GatewayJsonPb, GatewayValue},
    utility_declarations::derive_deftly_template_UtilityGatewayProviders};

#[path = "utility_gateway_pattern.rs"]
mod pattern_support;
#[path = "utility_gateway_strings.rs"]
mod string_support;
pub use pattern_support::{PatternWordProfile, PatternConstruction, InvalidPatternError, is_invalid_pattern,
    PatternDiagnostic, PatternLogProfile, SourcePatternLogger, HasSourcePatternLogger, SourcePatternLoggerGetterComponent,
    CanEmitSourcePatternDiagnostic, SourcePatternDiagnosticEmitter, SourcePatternDiagnosticEmitterComponent, EmitSourcePatternDiagnostic,
    CanConstructSourceGatewayPattern, SourceGatewayPatternConstructor, SourceGatewayPatternConstructorComponent, ConstructSourceGatewayPattern};
pub use string_support::{PatternStringFailure, CanCompleteSourcePatternString, SourcePatternStringCompletion,
    SourcePatternStringCompletionComponent, CompleteSourcePatternString};
use string_support::{join_components, concat_strings, format_string_parts};

const ERRORS: Errors = Errors::from_package("github.com/grpc-ecosystem/grpc-gateway/v2/runtime");
pub type PathParameters = HashMap<Bytes, Bytes>;
pub type GatewayHandler = Arc<dyn for<'a> Fn(&'a GatewayContext, &'a mut ResponseWriter,
    &'a mut HttpRequest, &'a GatewayCall) -> BoxFuture<'a, ()> + Send + Sync>;
pub type GatewayUnaryHandler = Arc<dyn for<'a> Fn(&'a GatewayContext, &'a GatewayCall,
    DynamicMessage) -> BoxFuture<'a, Result<GatewayValue, GatewayStatus>> + Send + Sync>;
pub type GatewayResponseStream = Pin<Box<dyn Stream<Item = Result<GatewayValue, GatewayStatus>> + Send>>;

#[derive(Clone)]
pub struct GatewayCall {
    pub path_parameters: PathParameters,
    pub pattern: GatewayPattern,
    pub form: Option<SourceValues>,
    pub cancelled: CancellationToken,
}

/// The int opcode format emitted by the pinned grpc-gateway generator.
/// The caller's []string pool remains a shared slice view; capture names are
/// copied string values, as in NewPattern. Registered Pattern copies retain
/// the same pool backing without another routing or lifecycle owner.
#[derive(Clone, Debug)]
pub struct GatewayPattern { operations: Vec<(i64, usize)>, pool: crate::utility_http::HeaderValues, variables: Vec<Bytes>,
    stack_size: usize, tail_length: usize, verb: Bytes, word_profile: PatternWordProfile }
impl Default for GatewayPattern { fn default() -> Self { Self::zero(PatternWordProfile::Go64) } }
impl GatewayPattern {
    /// A Go zero Pattern under an explicit source int ABI. The profile is
    /// retained on rejected construction and every registered Pattern copy.
    pub fn zero(profile: PatternWordProfile) -> Self {
        Self { operations: Vec::new(), pool: crate::utility_http::HeaderValues::nil(), variables: Vec::new(),
            stack_size: 0, tail_length: 0, verb: Bytes::new(), word_profile: profile }
    }
    pub fn word_profile(&self) -> PatternWordProfile { self.word_profile }
    pub fn new(context: &impl CanEmitSourcePatternDiagnostic, profile: PatternWordProfile, version: i64,
        operations: &[i64], pool: crate::utility_http::HeaderValues, verb: impl AsRef<[u8]>) -> PatternConstruction {
        use PatternDiagnostic::*;
        let version = profile.integer(version);
        if version != 1 { return pattern_support::reject_pattern(context, profile, UnsupportedVersion(version)); }
        if operations.len() % 2 != 0 { return pattern_support::reject_pattern(context, profile, OddCodeCount(profile.integer(operations.len() as i64))); }
        let mut result = Self { operations: Vec::new(), pool, variables: Vec::new(), stack_size: 0, tail_length: 0,
            verb: Bytes::copy_from_slice(verb.as_ref()), word_profile: profile };
        let (mut stack, mut maxstack, mut tail) = (0_i64, 0_i64, 0_i64);
        let mut deep_seen = false;
        for pair in operations.chunks_exact(2) {
            let code = profile.integer(pair[0]); let source_operand = profile.integer(pair[1]);
            let mut operand = source_operand as usize;
            match code {
                0 => continue,
                1 => { if deep_seen { tail = profile.add(tail, 1); } stack = profile.add(stack, 1); },
                2 => {
                    if source_operand < 0 || operand >= result.pool.len() { return pattern_support::reject_pattern(context, profile, LiteralIndex(source_operand)); }
                    if deep_seen { tail = profile.add(tail, 1); } stack = profile.add(stack, 1);
                }
                3 => {
                    if deep_seen { return pattern_support::reject_pattern(context, profile, DuplicatePushM); }
                    deep_seen = true; stack = profile.add(stack, 1);
                }
                4 => {
                    if source_operand <= 0 { return pattern_support::reject_pattern(context, profile, ConcatSize(source_operand)); }
                    stack = profile.sub(stack, source_operand);
                    if stack < 0 { return pattern_support::reject_pattern(context, profile, StackUnderflow); }
                    stack = profile.add(stack, 1);
                }
                5 => {
                    if source_operand < 0 || operand >= result.pool.len() { return pattern_support::reject_pattern(context, profile, CaptureIndex(source_operand)); }
                    result.variables.push(result.pool.get(operand).expect("validated source literal index"));
                    operand = result.variables.len() - 1;
                    stack = profile.sub(stack, 1);
                    if stack < 0 { return pattern_support::reject_pattern(context, profile, StackUnderflow); }
                }
                _ => return pattern_support::reject_pattern(context, profile, InvalidOpcode(code)),
            }
            if maxstack < stack { maxstack = stack; }
            result.operations.push((code, operand));
        }
        result.stack_size = maxstack as usize; result.tail_length = tail as usize;
        PatternConstruction { pattern: result, error: None }
    }
    pub fn verb(&self) -> &[u8] { &self.verb }
    pub fn matches(&self, context: &impl CanCompleteSourcePatternString, components: &[Bytes], verb: &Bytes) -> Option<PathParameters> {
        // The source borrows the incoming slice unless it must append a
        // mismatching verb. Copying eagerly changes its allocation path.
        let mut components = std::borrow::Cow::Borrowed(components);
        if self.verb != *verb {
            if !self.verb.is_empty() { return None; }
            if components.is_empty() {
                components = std::borrow::Cow::Owned(vec![context.complete_source_pattern_string(
                    concat_strings(self.word_profile, &[Bytes::from_static(b":"), verb.clone()]))]);
            } else {
                let last = components.to_mut().last_mut().expect("nonempty source components");
                *last = context.complete_source_pattern_string(concat_strings(self.word_profile, &[last.clone(), Bytes::from_static(b":"), verb.clone()]));
            }
        }
        let mut position = 0;
        let mut stack = Vec::with_capacity(self.stack_size);
        let mut captures = vec![Bytes::new(); self.variables.len()];
        for (code, operand) in &self.operations {
            match code {
                1 | 2 => {
                    let component = components.get(position)?;
                    if *code == 2 && component != &self.pool.get(*operand).expect("validated source literal index") { return None; }
                    stack.push(component.clone()); position += 1;
                }
                3 => {
                    let end = components.len().checked_sub(self.tail_length)?;
                    if end < position { return None; }
                    stack.push(context.complete_source_pattern_string(join_components(self.word_profile, &components[position..end]))); position = end;
                }
                4 => { let start = stack.len() - operand; let value = context.complete_source_pattern_string(join_components(self.word_profile, &stack[start..])); stack.truncate(start); stack.push(value); },
                5 => { captures[*operand] = stack.pop()?; },
                _ => unreachable!("validated pattern opcode"),
            }
        }
        if position < components.len() { return None; }
        // Duplicate capture names overwrite in source variable order. Map
        // iteration does not acquire a prescribed lexicographic priority.
        Some(self.variables.iter().cloned().zip(captures).collect())
    }
    /// Source Pattern.String result. Display below is a text compatibility
    /// view; routing/callback data and this result retain arbitrary Go bytes.
    pub fn string_bytes(&self, context: &impl CanCompleteSourcePatternString) -> Bytes {
        context.complete_source_pattern_string(self.try_string_bytes())
    }
    /// Explicit Rust result projection; source callers use string_bytes so
    /// source panics/throws pass through the selected production capability.
    pub fn try_string_bytes(&self) -> Result<Bytes, PatternStringFailure> {
        let mut stack: Vec<Bytes> = Vec::new();
        for (code, operand) in &self.operations {
            match code {
                1 => stack.push(Bytes::from_static(b"*")),
                2 => stack.push(self.pool.get(*operand).expect("validated source literal index")),
                3 => stack.push(Bytes::from_static(b"**")),
                4 => { let start = stack.len() - operand; let value = join_components(self.word_profile, &stack[start..])?; stack.truncate(start); stack.push(value); },
                5 => {
                    let value = stack.last_mut().expect("validated pattern capture");
                    *value = format_string_parts(self.word_profile, &[b"{", &self.variables[*operand], b"=", value.as_ref(), b"}"])?;
                }
                _ => unreachable!("validated pattern opcode"),
            }
        }
        let segments = join_components(self.word_profile, &stack)?;
        if !self.verb.is_empty() { return format_string_parts(self.word_profile, &[b"/", &segments, b":", &self.verb]); }
        concat_strings(self.word_profile, &[Bytes::from_static(b"/"), segments])
    }
}
impl fmt::Display for GatewayPattern {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.try_string_bytes().map_err(|_| fmt::Error)?;
        out.write_str(&String::from_utf8_lossy(&value))
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
    pub fn new_pattern(&self, profile: PatternWordProfile, version: i64, operations: &[i64], pool: crate::utility_http::HeaderValues, verb: &[u8]) -> PatternConstruction {
        self.gateway_runtime().construct_source_gateway_pattern(profile, version, operations, pool, verb)
    }
    pub fn must_pattern(&self, construction: PatternConstruction) -> GatewayPattern { self.gateway_runtime().require_source_gateway_pattern(construction) }
    pub fn handle_compiled(&self, profile: PatternWordProfile, method: impl AsRef<[u8]>, operations: &[i64], pool: crate::utility_http::HeaderValues, verb: &[u8], handler: GatewayHandler) {
        let construction = self.new_pattern(profile, 1, operations, pool, verb);
        self.handle(method, self.must_pattern(construction), handler);
    }

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
    // The source captures URL.Path before ParseForm can call a user Reader.
    let path = request.path();
    if !path.starts_with(b"/") { let _ = context.write_gateway_error(writer, &routing_status(400)).await; return; }
    let mut form = None;
    if is_path_length_fallback(request) {
        let method = header(&request.headers, "X-HTTP-Method-Override").filter(|value| !value.is_empty());
        if let Some(method) = method {
            match parse_form(request).await {
                Ok(value) => form = Some(value),
                Err(error) => { let _ = context.write_gateway_error(writer, &GatewayStatus::new(3, error.to_string())).await; return; },
            }
            request.set_method(context.gateway_runtime().uppercase_source_string(&method));
        }
    }
    let path: Vec<_> = path[1..].split(|byte| *byte == b'/').map(Bytes::copy_from_slice).collect();
    let (same, others) = {
        let requested_method = request.method();
        let routes = context.gateway_state().routes.read().unwrap_or_else(|error| error.into_inner());
        (routes.get(&requested_method).cloned().unwrap_or_default(),
            routes.iter().filter(|(method, _)| **method != requested_method).map(|(method, routes)| (method.clone(), routes.clone())).collect::<Vec<_>>())
    };
    for route in same {
        let (parts, verb, empty_verb_path) = split_verb(&route.pattern, &path);
        if empty_verb_path { let _ = context.write_gateway_error(writer, &routing_status(404)).await; return; }
        if let Some(parameters) = route.pattern.matches(context.gateway_runtime(), &parts, &verb) {
            let call = GatewayCall { path_parameters: parameters, pattern: route.pattern, form, cancelled: request.cancelled.clone() };
            context.call_gateway_handler(&route.handler, writer, request, &call).await;
            return;
        }
    }
    for (method, routes) in others {
        for route in routes {
            let (parts, verb, _) = split_verb(&route.pattern, &path);
            if let Some(parameters) = route.pattern.matches(context.gateway_runtime(), &parts, &verb) {
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
fn split_verb(pattern: &GatewayPattern, path: &[Bytes]) -> (Vec<Bytes>, Bytes, bool) {
    let mut parts = path.to_vec();
    let mut verb = Bytes::new();
    let last = parts.last_mut().expect("split path contains at least one component");
    if !pattern.verb().is_empty() {
        let mut suffix = vec![b':']; suffix.extend_from_slice(pattern.verb());
        if last.ends_with(&suffix) {
            let prefix_len = last.len() - suffix.len();
            if prefix_len == 0 { return (parts, verb, true); }
            let prefix = last.slice(..prefix_len); verb = last.slice(prefix_len+1..); *last = prefix;
        }
    }
    (parts, verb, false)
}

fn is_path_length_fallback(request: &HttpRequest) -> bool {
    request.method().as_ref() == b"POST" && header(&request.headers, "Content-Type").as_deref() == Some(b"application/x-www-form-urlencoded".as_slice())
}
async fn parse_form(request: &mut HttpRequest) -> Result<SourceValues, BoxError> { request.parse_gateway_form().await }

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
