// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/runtime/runtime.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::Arc;
use crate::{errors::BoxError, logfields::LogEntry,
    utility_logging::{CanHandleSourceRuntimeError, HasSourceLogger, LogLevel}};

pub const sourceKey: &str = "source";
pub trait SourceLogType { const SOURCE_LOG_TYPE: &'static str; }

// Initialization is owned by UtilityRuntimeOwner: it installs the selected
// JSON field map and the replaced error logger while retaining the inherited
// second backoff handler. No process-global logger is duplicated in children.
pub fn SetLevel<Context: HasSourceLogger>(context: &Context, level: LogLevel) {
    context.source_logger().set_level(level);
}
pub async fn HandleError<Context: CanHandleSourceRuntimeError>(context: &Context, logger: Option<&LogEntry>, error: Option<&(dyn std::error::Error + Send + Sync)>) {
    context.handle_source_runtime_error(logger, error).await;
}
pub fn Must(error: Option<BoxError>) { if let Some(error) = error { std::panic::panic_any(error); } }
pub fn NewLoggerWithSource<Context: HasSourceLogger>(context: &Context, source: &str) -> LogEntry {
    LogEntry::from_logger(Arc::clone(context.source_logger())).with_field(sourceKey, source)
}
pub fn NewLoggerWithType<Context: HasSourceLogger, Object: SourceLogType + ?Sized>(context: &Context, _object: Option<&Object>) -> LogEntry {
    NewLoggerWithSource(context, Object::SOURCE_LOG_TYPE)
}
pub fn NewLoggerForNilInterface<Context: HasSourceLogger>(context: &Context) -> LogEntry { NewLoggerWithSource(context, "<nil>") }
