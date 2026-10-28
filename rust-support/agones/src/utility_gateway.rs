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
#[path = "utility_gateway_bounds.rs"]
mod bounds_support;
#[path = "utility_gateway_storage.rs"]
mod storage_support;
#[path = "utility_gateway_strings.rs"]
mod string_support;
pub use pattern_support::{PatternWordProfile, PatternSourceProfile, PatternConstruction, PatternSentinelError,
    InvalidPatternError, PatternNotMatchError, is_invalid_pattern, is_pattern_not_match,
    PatternDiagnostic, PatternLogProfile, SourcePatternLogger, HasSourcePatternLogger, SourcePatternLoggerGetterComponent,
    CanEmitSourcePatternDiagnostic, SourcePatternDiagnosticEmitter, SourcePatternDiagnosticEmitterComponent, EmitSourcePatternDiagnostic,
    CanConstructSourceGatewayPattern, SourceGatewayPatternConstructor, SourceGatewayPatternConstructorComponent, ConstructSourceGatewayPattern};
pub use bounds_support::{PatternBoundsCode, PatternBoundsError};
pub use storage_support::PatternInputDomainError;
pub use string_support::{PatternStringFailure, PatternOperationFailure, CanCompleteSourcePatternString, SourcePatternStringCompletion,
    SourcePatternStringCompletionComponent, CompleteSourcePatternString};
use string_support::{join_components, concat_strings, format_string_parts};

const ERRORS: Errors = Errors::from_package("github.com/grpc-ecosystem/grpc-gateway/v2/runtime");
pub type PathParameters = HashMap<Bytes, Bytes>;
pub type PatternMatchResult = Result<PathParameters, PatternNotMatchError>;
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
pub struct GatewayPattern { operations: storage_support::PatternOperations, pool: crate::utility_http::HeaderValues, variables: crate::utility_http::HeaderValues,
    stack_size: i64, tail_length: i64, verb: Bytes, source_profile: PatternSourceProfile }
impl Default for GatewayPattern { fn default() -> Self { Self::zero(PatternWordProfile::Go64) } }
impl GatewayPattern {
    /// A Go zero Pattern under an explicit source word/heap profile. It is
    /// retained on rejected construction and every registered Pattern copy.
    pub fn zero(profile: impl Into<PatternSourceProfile>) -> Self {
        Self { operations: storage_support::PatternOperations::default(), pool: crate::utility_http::HeaderValues::nil(), variables: crate::utility_http::HeaderValues::nil(),
            stack_size: 0, tail_length: 0, verb: Bytes::new(), source_profile: profile.into() }
    }
    pub fn source_profile(&self) -> PatternSourceProfile { self.source_profile }
    pub fn word_profile(&self) -> PatternWordProfile { self.source_profile.word() }
    pub fn new(context: &(impl CanEmitSourcePatternDiagnostic + CanCompleteSourcePatternString), profile: impl Into<PatternSourceProfile>, version: i64,
        operations: &[i64], pool: crate::utility_http::HeaderValues, verb: Bytes) -> PatternConstruction {
        context.complete_source_pattern_construction(Self::try_new(context, profile.into(), version, operations, pool, verb))
    }
    /// Integer arguments are source-word bit values. Native slice/string
    /// lengths are checked before entering the source constructor body.
    pub fn try_new(context: &impl CanEmitSourcePatternDiagnostic, profile: PatternSourceProfile, version: i64,
        operations: &[i64], pool: crate::utility_http::HeaderValues, verb: Bytes) -> Result<PatternConstruction, PatternOperationFailure> {
        use PatternDiagnostic::*;
        storage_support::input_array(profile, "ops", operations.len(), (profile.allocation().word_bits() / 8) as u64)?;
        storage_support::input_strings(profile, "pool", &pool)?;
        storage_support::input_length(profile, "verb", verb.len())?;
        let word = profile.word();
        let version = word.integer(version);
        if version != 1 { return Ok(pattern_support::reject_pattern(context, profile, UnsupportedVersion(version))); }
        if operations.len() % 2 != 0 { return Ok(pattern_support::reject_pattern(context, profile, OddCodeCount(word.integer(operations.len() as i64)))); }
        let mut result = Self { operations: storage_support::PatternOperations::default(), pool, variables: crate::utility_http::HeaderValues::nil(), stack_size: 0, tail_length: 0,
            verb, source_profile: profile };
        let mut typed_operations = storage_support::PatternOperationBuilder::default();
        let (mut stack, mut maxstack, mut tail) = (0_i64, 0_i64, 0_i64);
        let mut deep_seen = false;
        for pair in operations.chunks_exact(2) {
            let code = word.integer(pair[0]); let source_operand = word.integer(pair[1]);
            let mut operand = source_operand;
            match code {
                0 => continue,
                1 => { if deep_seen { tail = word.add(tail, 1); } stack = word.add(stack, 1); },
                2 => {
                    if source_operand < 0 || source_operand as u128 >= result.pool.len() as u128 { return Ok(pattern_support::reject_pattern(context, profile, LiteralIndex(source_operand))); }
                    if deep_seen { tail = word.add(tail, 1); } stack = word.add(stack, 1);
                }
                3 => {
                    if deep_seen { return Ok(pattern_support::reject_pattern(context, profile, DuplicatePushM)); }
                    deep_seen = true; stack = word.add(stack, 1);
                }
                4 => {
                    if source_operand <= 0 { return Ok(pattern_support::reject_pattern(context, profile, ConcatSize(source_operand))); }
                    stack = word.sub(stack, source_operand);
                    if stack < 0 { return Ok(pattern_support::reject_pattern(context, profile, StackUnderflow)); }
                    stack = word.add(stack, 1);
                }
                5 => {
                    if source_operand < 0 || source_operand as u128 >= result.pool.len() as u128 { return Ok(pattern_support::reject_pattern(context, profile, CaptureIndex(source_operand))); }
                    result.variables.try_append_values(&[result.pool.get(source_operand as usize).expect("validated source literal index")], profile.allocation())?;
                    operand = word.integer((result.variables.len() - 1) as i64);
                    stack = word.sub(stack, 1);
                    if stack < 0 { return Ok(pattern_support::reject_pattern(context, profile, StackUnderflow)); }
                }
                _ => return Ok(pattern_support::reject_pattern(context, profile, InvalidOpcode(code))),
            }
            if maxstack < stack { maxstack = stack; }
            typed_operations.push(profile, (code, operand))?;
        }
        result.operations = typed_operations.finish();
        result.stack_size = maxstack; result.tail_length = tail;
        Ok(PatternConstruction { pattern: result, error: None })
    }
    pub fn verb(&self) -> &[u8] { &self.verb }
    pub fn matches(&self, context: &impl CanCompleteSourcePatternString, components: &crate::utility_http::HeaderValues, verb: &Bytes) -> Option<PathParameters> {
        self.match_source(context, components, verb).ok()
    }
    /// Canonical default-Legacy source Match/MatchAndEscape result. The
    /// dispatcher uses matches as an explicit Option projection of this.
    pub fn match_source(&self, context: &impl CanCompleteSourcePatternString, components: &crate::utility_http::HeaderValues, verb: &Bytes) -> PatternMatchResult {
        context.complete_source_pattern_match(self.try_matches(components, verb).map(|value| value.ok_or_else(pattern_support::not_match_error)))
    }
    /// Rust result projection separates a returned non-match from source
    /// panic/throw outcomes. The production entrypoint completes via CGP.
    pub fn try_matches(&self, components: &crate::utility_http::HeaderValues, verb: &Bytes) -> Result<Option<PathParameters>, PatternOperationFailure> {
        storage_support::input_strings(self.source_profile, "components", components)?;
        storage_support::input_strings(self.source_profile, "pool", &self.pool)?;
        storage_support::input_length(self.source_profile, "verb", verb.len())?;
        let word = self.source_profile.word();
        let allocation = self.source_profile.allocation();
        // Clone the source slice descriptor, retaining its length/capacity
        // and backing. Verb fallback alone copies its string elements.
        let mut components = components.clone();
        if self.verb != *verb {
            if !self.verb.is_empty() { return Ok(None); }
            if components.is_empty() {
                let value = concat_strings(self.source_profile, &[Bytes::from_static(b":"), verb.clone()])?;
                components = bounds_support::make_strings(self.source_profile, 1, 1)?;
                components.set(0, value);
            } else {
                let mut copied = crate::utility_http::HeaderValues::nil();
                copied.try_append_slice(&components, allocation)?; components = copied;
                let last = components.len() - 1;
                let value = components.get(last).expect("nonempty source components");
                components.set(last, concat_strings(self.source_profile, &[value, Bytes::from_static(b":"), verb.clone()])?);
            }
        }
        let mut position = 0_i64;
        let length = word.integer(components.len() as i64);
        let mut stack = bounds_support::make_strings(self.source_profile, 0, self.stack_size)?;
        let captures = bounds_support::make_strings(self.source_profile, self.variables.len() as i64, self.variables.len() as i64)?;
        for (code, operand) in self.operations.iter() {
            match code {
                1 | 2 => {
                    if position >= length { return Ok(None); }
                    let component = components.get(bounds_support::index(position, components.len())?).expect("checked source component index");
                    if *code == 2 {
                        let literal = self.pool.get(bounds_support::index(*operand, self.pool.len())?).expect("checked source literal index");
                        if component != literal { return Ok(None); }
                    }
                    stack.try_append_values(&[component], allocation)?; position = word.add(position, 1);
                }
                3 => {
                    if length < word.add(position, self.tail_length) { return Ok(None); }
                    let end = word.sub(length, self.tail_length);
                    let range = bounds_support::slice(position, end, components.capacity())?;
                    let values = components.reslice(range.start, range.end, components.capacity());
                    stack.try_append_values(&[join_components(self.source_profile, &values)?], allocation)?; position = end;
                }
                4 => {
                    let start = word.sub(word.integer(stack.len() as i64), *operand);
                    let prefix = bounds_support::slice(0, start, stack.capacity())?;
                    let suffix = bounds_support::slice(start, stack.len() as i64, stack.capacity())?;
                    let values = stack.reslice(suffix.start, suffix.end, stack.capacity());
                    let value = join_components(self.source_profile, &values)?;
                    stack = stack.reslice(prefix.start, prefix.end, stack.capacity()); stack.try_append_values(&[value], allocation)?;
                }
                5 => {
                    let last = word.sub(word.integer(stack.len() as i64), 1);
                    let value = stack.get(bounds_support::index(last, stack.len())?).expect("checked source stack index");
                    captures.set(bounds_support::index(*operand, captures.len())?, value);
                    let prefix = bounds_support::slice(0, last, stack.capacity())?;
                    stack = stack.reslice(prefix.start, prefix.end, stack.capacity());
                }
                _ => unreachable!("validated pattern opcode"),
            }
        }
        if position < length { return Ok(None); }
        // Duplicate capture names overwrite in source variable order. Map
        // iteration does not acquire a prescribed lexicographic priority.
        Ok(Some(self.variables.iter().zip(captures.iter()).collect()))
    }
    /// Source Pattern.String result. Display below is a text compatibility
    /// view; routing/callback data and this result retain arbitrary Go bytes.
    pub fn string_bytes(&self, context: &impl CanCompleteSourcePatternString) -> Bytes {
        context.complete_source_pattern_render(self.try_string_bytes())
    }
    /// Explicit Rust result projection; source callers use string_bytes so
    /// source panics/throws pass through the selected production capability.
    pub fn try_string_bytes(&self) -> Result<Bytes, PatternOperationFailure> {
        storage_support::input_strings(self.source_profile, "pool", &self.pool)?;
        let word = self.source_profile.word();
        let allocation = self.source_profile.allocation();
        let mut stack = crate::utility_http::HeaderValues::nil();
        for (code, operand) in self.operations.iter() {
            match code {
                1 => stack.try_append_values(&[Bytes::from_static(b"*")], allocation)?,
                2 => stack.try_append_values(&[self.pool.get(bounds_support::index(*operand, self.pool.len())?).expect("checked source literal index")], allocation)?,
                3 => stack.try_append_values(&[Bytes::from_static(b"**")], allocation)?,
                4 => {
                    let start = word.sub(word.integer(stack.len() as i64), *operand);
                    let prefix = bounds_support::slice(0, start, stack.capacity())?;
                    let suffix = bounds_support::slice(start, stack.len() as i64, stack.capacity())?;
                    let values = stack.reslice(suffix.start, suffix.end, stack.capacity());
                    let value = join_components(self.source_profile, &values)?;
                    stack = stack.reslice(prefix.start, prefix.end, stack.capacity()); stack.try_append_values(&[value], allocation)?;
                }
                5 => {
                    let name = self.variables.get(bounds_support::index(*operand, self.variables.len())?).expect("checked source capture-name index");
                    let last = bounds_support::index(word.sub(word.integer(stack.len() as i64), 1), stack.len())?;
                    let value = stack.get(last).expect("checked source stack index");
                    stack.set(last, format_string_parts(self.source_profile, &[b"{", &name, b"=", &value, b"}"])?);
                }
                _ => unreachable!("validated pattern opcode"),
            }
        }
        let segments = join_components(self.source_profile, &stack)?;
        if !self.verb.is_empty() { return Ok(format_string_parts(self.source_profile, &[b"/", &segments, b":", &self.verb])?); }
        Ok(concat_strings(self.source_profile, &[Bytes::from_static(b"/"), segments])?)
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
    pub fn new_pattern(&self, profile: impl Into<PatternSourceProfile>, version: i64, operations: &[i64], pool: crate::utility_http::HeaderValues, verb: Bytes) -> PatternConstruction {
        self.gateway_runtime().construct_source_gateway_pattern(profile.into(), version, operations, pool, verb)
    }
    pub fn must_pattern(&self, construction: PatternConstruction) -> GatewayPattern { self.gateway_runtime().require_source_gateway_pattern(construction) }
    pub fn handle_compiled(&self, profile: impl Into<PatternSourceProfile>, method: impl AsRef<[u8]>, operations: &[i64], pool: crate::utility_http::HeaderValues, verb: Bytes, handler: GatewayHandler) {
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
        let (parts, verb, empty_verb_path) = split_verb(context.gateway_runtime(), &route.pattern, &path, true);
        if empty_verb_path { let _ = context.write_gateway_error(writer, &routing_status(404)).await; return; }
        if let Some(parameters) = route.pattern.matches(context.gateway_runtime(), &parts, &verb) {
            let call = GatewayCall { path_parameters: parameters, pattern: route.pattern, form, cancelled: request.cancelled.clone() };
            context.call_gateway_handler(&route.handler, writer, request, &call).await;
            return;
        }
    }
    for (method, routes) in others {
        for route in routes {
            let (parts, verb, _) = split_verb(context.gateway_runtime(), &route.pattern, &path, false);
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
fn split_verb(context: &impl CanCompleteSourcePatternString, pattern: &GatewayPattern, path: &[Bytes], reject_empty_prefix: bool)
    -> (crate::utility_http::HeaderValues, Bytes, bool) {
    context.complete_source_pattern_input(storage_support::input_array(pattern.source_profile, "path components", path.len(), (pattern.source_profile.allocation().word_bits() / 4) as u64));
    for component in path { context.complete_source_pattern_input(storage_support::input_length(pattern.source_profile, "path component", component.len())); }
    let last = path.last().expect("split path contains at least one component");
    let mut prefix_length = None;
    if !pattern.verb().is_empty() {
        let suffix = context.complete_source_pattern_string(concat_strings(pattern.source_profile,
            &[Bytes::from_static(b":"), pattern.verb.clone()]));
        if last.ends_with(&suffix) { prefix_length = Some(last.len() - suffix.len()); }
    }
    // In the same-method loop this 404 precedes make/copy. The other-method
    // loop still makes an untrimmed copy when its matching prefix is empty.
    if reject_empty_prefix && prefix_length == Some(0) { return (crate::utility_http::HeaderValues::nil(), Bytes::new(), true); }
    let parts = context.complete_source_pattern_slice(bounds_support::make_strings(pattern.source_profile, path.len() as i64, path.len() as i64));
    for (index, value) in path.iter().enumerate() { parts.set(index, value.clone()); }
    let mut verb = Bytes::new();
    if let Some(prefix_length) = prefix_length.filter(|length| *length > 0) {
        verb = last.slice(prefix_length + 1..); parts.set(parts.len() - 1, last.slice(..prefix_length));
    }
    (parts, verb, prefix_length == Some(0))
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
