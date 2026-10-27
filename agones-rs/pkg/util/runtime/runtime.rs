// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/runtime/runtime.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::Arc;
use crate::{errors::BoxError, logfields::LogEntry,
    utility_logging::{CanHandleSourceRuntimeError, HasSourceLogger, CanEmitSourceLog, LogLevel},
    utility_client_config::{CanReadSourceClientConfiguration, RusternetesClientConfiguration}};

pub const sourceKey: &str = "source";
pub trait SourceLogType { const SOURCE_LOG_TYPE: &'static str; }
const errs: crate::errors::Errors = crate::errors::Errors::from_package("agones.dev/agones/pkg/util/runtime");

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

/// The source wildcard HTTPBodyMarshaler over JSONPb with UseProtoNames,
/// EmitUnpopulated and DiscardUnknown is consumed by the concrete mux/codec.
pub fn NewServerMux<Context: crate::utility_gateway::CanCreateSourceServerMux>(context: &Context) -> crate::utility_gateway::GatewayContext {
    context.new_source_server_mux()
}

pub async fn InClusterBuildConfig<Context>(context: &Context, logger: Option<&LogEntry>, kubeconfigPath: &str) -> Result<RusternetesClientConfiguration, BoxError>
where Context: CanReadSourceClientConfiguration + CanEmitSourceLog + HasSourceLogger,
{
    match context.source_in_cluster_config().await {
        Ok(config) => return Ok(config),
        Err(error) => {
            let logger = logger.expect("nil logrus.Entry after InClusterConfig failure").with_error(Some(error.as_ref()));
            // logrus.Warning uses Sprint: a leading string argument suppresses
            // an automatically inserted space before the error argument.
            context.emit_source_log(&logger, LogLevel::Warn,
                &format!("Error creating inClusterConfig, trying to build config from flags{error}"));
        }
    }
    if kubeconfigPath.is_empty() {
        let logger = LogEntry::from_logger(Arc::clone(context.source_logger()));
        context.emit_source_log(&logger, LogLevel::Warn, "No kubeconfigPath provided. Attempting to use a default configuration.");
    }
    context.source_deferred_client_config(kubeconfigPath).await
}
