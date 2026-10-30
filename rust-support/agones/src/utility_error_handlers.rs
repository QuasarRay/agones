//! Source apimachinery ErrorHandlers data, argument ownership and dispatch.
// Source: pinned vendor/k8s.io/apimachinery/pkg/util/runtime/runtime.go and
// Agones pkg/util/runtime/runtime.go. Kubernetes Authors / Contributors to
// Agones, Apache-2.0. Passive callback data stays in the existing CGP runtime.
use std::{any::Any, sync::{Arc, Mutex}, time::{Duration, Instant}};
use bytes::Bytes;
use cgp::prelude::*;
use derive_deftly::Deftly;
use futures::future::BoxFuture;
use tokio_util::sync::CancellationToken;
use crate::{utility_callback_slice::SourceCallbackSlice, utility_error_slice::SourceRetainedError,
    utility_format_copy::SourceFmtValueCopy, utility_format_arguments::{SourcePrintfValue, SourcePrintfNil},
    utility_format_methods::{SourcePrintfInterfaces, SourcePrintfRecursion, write_source_value}, utility_format_scan::SourcePrintfResolvedCounts,
    utility_format_print::SourcePrintfMode, utility_http::{HeaderAllocationProfile, SourceUnicodeProfile, SourceSliceAllocationError},
    utility_logging::{SourceErrorBackoff, HasSourceLogger, CanEmitSourceLog, CanEmitSourceErrorLog, LogLevel},
    utility_runtime::UtilityRuntimeContext, logfields::LogEntry,
    utility_declarations::derive_deftly_template_SourceCallbackSlice};

trait ErrorArgumentData: Send + Sync {
    fn original(&self) -> &dyn Any;
    fn source_type(&self) -> Option<&str>;
    fn is_nil(&self) -> bool;
    fn methods(&self) -> SourcePrintfInterfaces<'_>;
    fn write(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Option<Result<(), SourceSliceAllocationError>>;
}
struct RetainedErrorArgument { error: SourceRetainedError }
impl crate::utility_format_methods::SourceErrorMethod for RetainedErrorArgument {
    fn source_error_message(&self) -> std::borrow::Cow<'_, [u8]> {
        crate::utility_error_format::source_error_message(self.error.original()).unwrap_or_else(||
            std::panic::panic_any(crate::utility_klog_dispatch::SourceKlogFormatError::Unbound {
                source_type: self.source_type().map(str::to_owned), operation: "retained klog error Error method" }))
    }
}
impl ErrorArgumentData for RetainedErrorArgument {
    fn original(&self) -> &dyn Any { &self.error }
    fn source_type(&self) -> Option<&str> { crate::utility_error_view::source_error_type(self.error.original()) }
    fn is_nil(&self) -> bool { false }
    fn methods(&self) -> SourcePrintfInterfaces<'_> {
        use crate::utility_format_methods::SourcePrintfMethodViews;
        let mut error = self.error.original();
        loop {
            if let Some(retained) = error.downcast_ref::<SourceRetainedError>() { error = retained.original(); continue; }
            if let Some(runtime) = error.downcast_ref::<crate::utility_panic::SourceRuntimeErrorEnvelope>() { error = runtime.error(); continue; }
            if let Some(shared) = error.downcast_ref::<crate::utility_error_alias::SourceSharedError>() { return shared.source_methods(); }
            break;
        }
        macro_rules! declared {
            ($($ty:ty),+) => { $(if let Some(value) = error.downcast_ref::<$ty>() { return value.source_methods(); })+ };
        }
        if let Some(methods) = crate::utility_fs_error::methods(error) { return methods; }
        declared!(crate::utility_panic::SourceRuntimeErrorString, crate::utility_panic::SourceNilMapAssignment,
            crate::utility_panic::SourceBoundsError, crate::utility_error_nil::SourceNilStatusError,
            crate::utility_error_nil::SourceNilTraceError, crate::source::workerqueue::traceError,
            crate::utility_time_json_error::SourceTimeMarshalerError, crate::utility_logr_context::SourceLogrNotFoundError);
        if self.source_type().is_none() { return SourcePrintfInterfaces::default(); }
        SourcePrintfInterfaces { error: Some(self),
            error_methods: crate::utility_error_query::canonical_query_methods(error).unwrap_or_default(),
            ..SourcePrintfInterfaces::default() }
    }
    fn write(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Option<Result<(), SourceSliceAllocationError>>
    {
        self.source_type()?;
        Some(crate::utility_format_arguments::write_source_error_value(self.error.original(), verb, state, profile, mode, recursion, output))
    }
}
struct TypedErrorArgument<Value>(Value);
impl<Value: SourcePrintfValue + Send + Sync + 'static> ErrorArgumentData for TypedErrorArgument<Value> {
    fn original(&self) -> &dyn Any { &self.0 }
    fn source_type(&self) -> Option<&str> { Some(self.0.source_type()) }
    fn is_nil(&self) -> bool { self.0.source_is_nil() }
    fn methods(&self) -> SourcePrintfInterfaces<'_> { self.0.source_methods() }
    fn write(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion,
        output: &mut Vec<u8>) -> Option<Result<(), SourceSliceAllocationError>>
    { Some(write_source_value(&self.0, verb, state, profile, mode, recursion, output)) }
}
struct NativeErrorArgument<Value>(Value);
impl<Value: Any + Send + Sync> ErrorArgumentData for NativeErrorArgument<Value> {
    fn original(&self) -> &dyn Any { &self.0 }
    fn source_type(&self) -> Option<&str> { None }
    fn is_nil(&self) -> bool { false }
    fn methods(&self) -> SourcePrintfInterfaces<'_> { SourcePrintfInterfaces::default() }
    fn write(&self, _: char, _: &SourcePrintfResolvedCounts, _: SourceUnicodeProfile, _: SourcePrintfMode, _: SourcePrintfRecursion,
        _: &mut Vec<u8>) -> Option<Result<(), SourceSliceAllocationError>> { None }
}
/// Per-argument interface data retains an actual compiled value. Copying the
/// interface retains its data; source value/receiver copies are supplied by
/// that concrete declaration. There is no name-to-type/object/provider table.
#[derive(Default)]
pub struct SourceErrorArgument { data: Option<Arc<dyn ErrorArgumentData>> }
impl SourceErrorArgument {
    pub fn from_error(error: Option<SourceRetainedError>) -> Self {
        Self { data: error.map(|error| Arc::new(RetainedErrorArgument { error }) as Arc<dyn ErrorArgumentData>) }
    }
    pub fn from_value<Value: SourcePrintfValue + Send + Sync + 'static>(value: Value) -> Self {
        Self { data: Some(Arc::new(TypedErrorArgument(value))) }
    }
    pub fn copy_value<Value: SourcePrintfValue + SourceFmtValueCopy + Send + Sync + 'static>(value: &Value) -> Self {
        Self::from_value(value.source_fmt_copy())
    }
    /// Transport any supplied native value even when its source formatting
    /// binding is not yet authored. None from try_write reports that exact gap;
    /// it never substitutes a fabricated type name, Debug string or success.
    pub fn from_native<Value: Any + Send + Sync>(value: Value) -> Self {
        Self { data: Some(Arc::new(NativeErrorArgument(value))) }
    }
    pub fn is_nil(&self) -> bool { self.data.as_ref().is_none_or(|data| data.is_nil()) }
    pub fn downcast_ref<Value: Any>(&self) -> Option<&Value> { self.data.as_ref()?.original().downcast_ref() }
    pub fn source_type(&self) -> Option<&str> { self.data.as_ref().and_then(|data| data.source_type()) }
    pub fn source_methods(&self) -> SourcePrintfInterfaces<'_> {
        self.data.as_ref().map_or_else(SourcePrintfInterfaces::default, |data| data.methods())
    }
    pub fn try_write(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        output: &mut Vec<u8>) -> Option<Result<(), SourceSliceAllocationError>>
    { self.try_write_recursive(verb, state, profile, SourcePrintfMode::Sprintf, SourcePrintfRecursion::default(), output) }
    pub fn try_write_recursive(&self, verb: char, state: &SourcePrintfResolvedCounts, profile: SourceUnicodeProfile,
        mode: SourcePrintfMode, recursion: SourcePrintfRecursion, output: &mut Vec<u8>) -> Option<Result<(), SourceSliceAllocationError>>
    {
        match &self.data { Some(data) => data.write(verb, state, profile, mode, recursion, output),
            None => Some(write_source_value(&SourcePrintfNil, verb, state, profile, mode, recursion, output)) }
    }
}
impl SourceFmtValueCopy for SourceErrorArgument {
    fn source_fmt_copy(&self) -> Self { Self { data: self.data.as_ref().map(Arc::clone) } }
}
#[derive(Deftly)]
#[derive_deftly(SourceCallbackSlice)]
#[deftly(source_slice_words = "2", source_slice_value = "SourceErrorArgument")]
pub struct SourceErrorArguments { slice: SourceCallbackSlice<SourceErrorArgument> }

/// Callback context transport retains caller-supplied native data unchanged.
/// Background has no cancellation channel. This does not implement a general
/// Go context.Value/Deadline evaluator or the still-required full klog context.
#[derive(Clone)]
pub struct SourceErrorContext {
    stop: Option<CancellationToken>,
    data: Option<Arc<dyn Any + Send + Sync>>,
    source_nil: bool,
    logr_lookup: Option<crate::utility_logr_context::SourceLogrContextLookup>,
}
impl SourceErrorContext {
    pub fn background() -> Self { Self { stop: None, data: None, source_nil: false, logr_lookup: None } }
    pub fn nil() -> Self { Self { source_nil: true, ..Self::background() } }
    pub fn from_stop(stop: CancellationToken) -> Self { Self { stop: Some(stop), ..Self::background() } }
    pub fn logr_value(&self) -> crate::utility_logr_context::SourceLogrContextValue {
        if self.source_nil { crate::utility_error_nil::nil_error_receiver(); }
        self.logr_lookup.as_ref().map_or(crate::utility_logr_context::SourceLogrContextValue::Nil, |lookup| lookup())
    }
    pub fn with_logr_lookup(mut self, lookup: crate::utility_logr_context::SourceLogrContextLookup) -> Self {
        self.logr_lookup = Some(lookup); self
    }
    pub fn with_logr_value(mut self, value: crate::utility_logr_context::SourceLogrContextValue) -> Self {
        if self.source_nil { crate::utility_format_methods::SourcePrintfPanic::raise("cannot create context from nil parent"); }
        self.logr_lookup = Some(Arc::new(move || value.source_fmt_copy())); self
    }
    pub fn with_native_data<Value: Any + Send + Sync>(mut self, data: Arc<Value>) -> Self { self.data = Some(data); self }
    pub fn native_data<Value: Any>(&self) -> Option<&Value> { self.data.as_ref()?.downcast_ref() }
    pub fn is_cancelled(&self) -> bool {
        if self.source_nil { crate::utility_error_nil::nil_error_receiver(); }
        self.stop.as_ref().is_some_and(CancellationToken::is_cancelled)
    }
    pub async fn cancelled(&self) {
        if self.source_nil { crate::utility_error_nil::nil_error_receiver(); }
        match &self.stop { Some(stop) => stop.cancelled().await, None => std::future::pending::<()>().await }
    }
}
pub struct SourceErrorReport {
    pub context: SourceErrorContext,
    pub error: Option<SourceRetainedError>,
    pub message: Bytes,
    pub keys_and_values: SourceErrorArguments,
}
impl SourceFmtValueCopy for SourceErrorReport {
    fn source_fmt_copy(&self) -> Self { Self { context: self.context.clone(),
        error: self.error.as_ref().map(SourceFmtValueCopy::source_fmt_copy), message: self.message.clone(),
        keys_and_values: self.keys_and_values.source_fmt_copy() } }
}
pub type SourceErrorCallback = Arc<dyn for<'a> Fn(&'a UtilityRuntimeContext, SourceErrorReport) -> BoxFuture<'a, ()> + Send + Sync>;
#[derive(Default)]
pub enum SourceErrorHandler {
    #[default] Nil,
    KlogError,
    LogrusError,
    Backoff(Arc<SourceErrorBackoff>),
    Callback(SourceErrorCallback),
}
pub fn source_backoff_error(min_period: i64) -> SourceErrorHandler {
    SourceErrorHandler::Backoff(Arc::new(SourceErrorBackoff::new(min_period)))
}
impl SourceFmtValueCopy for SourceErrorHandler {
    fn source_fmt_copy(&self) -> Self {
        match self { Self::Nil => Self::Nil, Self::KlogError => Self::KlogError, Self::LogrusError => Self::LogrusError,
            Self::Backoff(counter) => Self::Backoff(Arc::clone(counter)), Self::Callback(callback) => Self::Callback(Arc::clone(callback)) }
    }
}
impl SourceErrorHandler {
    fn call<'a>(&'a self, runtime: &'a UtilityRuntimeContext, report: SourceErrorReport) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            match self {
                Self::Nil => crate::utility_error_nil::nil_error_receiver(),
                Self::KlogError => {
                    let logger = runtime.source_klog_state.from_context(runtime, &report.context).with_call_depth(3);
                    let logger = runtime.source_klog_state.logger_with_name(&logger, b"UnhandledError");
                    logger.error(runtime, report.error, &report.message, report.keys_and_values);
                },
                Self::LogrusError => {
                    // Agones replaces ErrorHandlers[0]; it ignores ctx/msg/kv.
                    let logger = LogEntry::from_logger(Arc::clone(runtime.source_logger()));
                    match report.error { Some(error) => runtime.emit_source_error_log(&logger, LogLevel::Error, b"", &error),
                        None => runtime.emit_source_log(&logger, LogLevel::Error, "<nil>") }
                },
                Self::Backoff(counter) => {
                    let now = runtime.source_error_instant();
                    let delay = counter.next_delay(now, || runtime.source_error_instant());
                    runtime.wait_source_error_delay(delay).await;
                },
                Self::Callback(callback) => callback(runtime, report).await,
            }
        })
    }
}
#[derive(Deftly)]
#[derive_deftly(SourceCallbackSlice)]
#[deftly(source_slice_words = "1", source_slice_value = "SourceErrorHandler")]
pub struct SourceErrorHandlers { slice: SourceCallbackSlice<SourceErrorHandler> }

/// Own only the source ErrorHandlers variable's current slice header. Active
/// ranges keep their copied header and read the current value of each cell.
pub struct SourceErrorHandlerState { handlers: Mutex<SourceErrorHandlers> }
impl SourceErrorHandlerState {
    pub fn agones_defaults(backoff: Arc<SourceErrorBackoff>, profile: HeaderAllocationProfile) -> Self {
        let state = Self::source_defaults(backoff, profile);
        state.handlers().set(0, SourceErrorHandler::LogrusError); state
    }
    pub fn source_defaults(backoff: Arc<SourceErrorBackoff>, profile: HeaderAllocationProfile) -> Self {
        let handlers = SourceErrorHandlers::make(2, 2, profile);
        handlers.set(0, SourceErrorHandler::KlogError);
        handlers.set(1, SourceErrorHandler::Backoff(backoff));
        Self { handlers: Mutex::new(handlers) }
    }
    pub fn handlers(&self) -> SourceErrorHandlers {
        self.handlers.lock().unwrap_or_else(|poison| poison.into_inner()).source_fmt_copy()
    }
    pub fn set_handlers(&self, handlers: SourceErrorHandlers) -> SourceErrorHandlers {
        let previous = {
            let mut slot = self.handlers.lock().unwrap_or_else(|poison| poison.into_inner());
            std::mem::replace(&mut *slot, handlers)
        };
        previous
    }
}
#[cgp_getter]
pub trait HasSourceErrorHandlerState { fn source_error_handler_state(&self) -> &Arc<SourceErrorHandlerState>; }
#[cgp_component(SourceRuntimeErrorDispatcher)]
pub trait CanDispatchSourceRuntimeError: Sync {
    fn dispatch_source_runtime_error<'a>(&'a self, report: SourceErrorReport) -> BoxFuture<'a, ()>;
}
#[cgp_new_provider]
impl SourceRuntimeErrorDispatcher<UtilityRuntimeContext> for DispatchSourceRuntimeError {
    fn dispatch_source_runtime_error<'a>(runtime: &'a UtilityRuntimeContext, report: SourceErrorReport) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let handlers = runtime.source_error_handler_state().handlers();
            for index in 0..handlers.len() {
                let handler = handlers.get(index);
                handler.call(runtime, report.source_fmt_copy()).await;
            }
        })
    }
}
#[cgp_component(SourceRuntimeErrorClock)]
pub trait CanReadSourceErrorClock { fn source_error_instant(&self) -> Instant; }
#[cgp_new_provider]
impl<Context> SourceRuntimeErrorClock<Context> for ReadNativeSourceErrorClock {
    fn source_error_instant(_context: &Context) -> Instant { Instant::now() }
}
#[cgp_component(SourceRuntimeErrorDelay)]
pub trait CanWaitSourceErrorDelay: Sync { fn wait_source_error_delay(&self, nanoseconds: i64) -> BoxFuture<'_, ()>; }
#[cgp_new_provider]
impl<Context: Sync> SourceRuntimeErrorDelay<Context> for WaitNativeSourceErrorDelay {
    fn wait_source_error_delay(_context: &Context, nanoseconds: i64) -> BoxFuture<'_, ()> {
        Box::pin(async move { if nanoseconds > 0 { tokio::time::sleep(Duration::from_nanos(nanoseconds as u64)).await; } })
    }
}
/// Contextual entrypoint deliberately dispatches nil errors, unlike the
/// inherited non-contextual HandleError wrapper's early nil return.
pub async fn handle_error_with_context<Context: CanDispatchSourceRuntimeError>(runtime: &Context, report: SourceErrorReport) {
    runtime.dispatch_source_runtime_error(report).await;
}
pub async fn handle_error_with_logger(runtime: &UtilityRuntimeContext, logger: crate::utility_logr::SourceLogr,
    error: Option<SourceRetainedError>, message: Bytes, keys_and_values: SourceErrorArguments)
{
    let context = runtime.source_klog_state.new_context(SourceErrorContext::background(), logger);
    runtime.dispatch_source_runtime_error(SourceErrorReport { context, error, message, keys_and_values }).await;
}
