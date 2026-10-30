// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/runtime/runtime.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::Arc;
use crate::{errors::BoxError, logfields::LogEntry,
    utility_logging::{CanHandleSourceRuntimeError, HasSourceLogger, CanEmitSourceLog, CanEmitSourceErrorLog, LogLevel},
    utility_client_config::{CanReadSourceClientConfiguration, RusternetesClientConfiguration}};

pub const sourceKey: &str = "source";
pub trait SourceLogType { const SOURCE_LOG_TYPE: &'static str; }
const errs: crate::errors::Errors = crate::errors::Errors::from_package("agones.dev/agones/pkg/util/runtime");

// The lifecycle owns a native JSON sink, filter and error backoff. Child
// contexts share those resources without a process-global callback registry.
pub fn SetLevel<Context: HasSourceLogger>(context: &Context, level: LogLevel) {
    context.source_logger().set_level(level);
}
pub async fn HandleError<Context: CanHandleSourceRuntimeError>(context: &Context, logger: Option<&LogEntry>, error: Option<&(dyn std::error::Error + Send + Sync + 'static)>) {
    context.handle_source_runtime_error(logger, error).await;
}
pub fn Must<T, E: std::fmt::Debug>(result: Result<T, E>) -> T { result.expect("Agones initialization failed") }
pub fn NewLoggerWithSource<Context: HasSourceLogger>(context: &Context, source: &str) -> LogEntry {
    LogEntry::from_logger(Arc::clone(context.source_logger())).with_field(sourceKey, source)
}
pub fn NewLoggerWithType<Context: HasSourceLogger, Object: ?Sized>(context: &Context, _object: Option<&Object>) -> LogEntry {
    NewLoggerWithSource(context, std::any::type_name::<Object>())
}

/// The source wildcard HTTPBodyMarshaler over JSONPb with UseProtoNames,
/// EmitUnpopulated and DiscardUnknown is consumed by the concrete mux/codec.
pub fn NewServerMux<Context: crate::utility_gateway::CanCreateSourceServerMux>(context: &Context) -> crate::utility_gateway::GatewayContext {
    context.new_source_server_mux()
}

pub async fn InClusterBuildConfig<Context>(context: &Context, logger: Option<&LogEntry>, kubeconfigPath: &str) -> Result<RusternetesClientConfiguration, BoxError>
where Context: CanReadSourceClientConfiguration + CanEmitSourceLog + CanEmitSourceErrorLog + HasSourceLogger,
{
    match context.source_in_cluster_config().await {
        Ok(config) => return Ok(config),
        Err(error) => {
            let fallback = LogEntry::from_logger(Arc::clone(context.source_logger()));
            let logger = logger.unwrap_or(&fallback).with_error(Some(error.as_ref()));
            context.emit_source_error_log(&logger, LogLevel::Warn,
                "Error creating inClusterConfig, trying to build config from flags: ", error.as_ref());
        }
    }
    if kubeconfigPath.is_empty() {
        let logger = LogEntry::from_logger(Arc::clone(context.source_logger()));
        context.emit_source_log(&logger, LogLevel::Warn, "No kubeconfigPath provided. Attempting to use a default configuration.");
    }
    context.source_deferred_client_config(kubeconfigPath).await
}
