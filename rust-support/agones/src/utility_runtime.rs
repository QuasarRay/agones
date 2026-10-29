//! Shared utility resources, constructed and consumed through CGP.
use std::{io::Write, sync::{Arc, Weak}};
use cgp::prelude::*;
use derive_deftly::Deftly;
use crate::{errors::BoxError, utility_logging::{SourceLogger, SourceErrorBackoff}, utility_scope::UtilityTaskScope,
    utility_klog_daemon::CanControlSourceKlogFlushDaemon,
    utility_declarations::derive_deftly_template_UtilityRuntimeProviders};

#[derive(Clone, HasField, Deftly)]
#[derive_deftly(UtilityRuntimeProviders)]
pub struct UtilityRuntimeContext {
    pub source_logger: Arc<SourceLogger>,
    pub source_klog_state: Arc<crate::utility_klog::SourceKlogState>,
    pub source_error_backoff: Arc<SourceErrorBackoff>,
    pub source_error_handler_state: Arc<crate::utility_error_handlers::SourceErrorHandlerState>,
    pub utility_task_scope: Weak<UtilityTaskScope>,
    pub source_signal_state: Arc<crate::utility_signals::SourceSignalState>,
    pub source_query_settings: Arc<crate::utility_http::SourceQuerySettings>,
    pub source_unicode_profile: crate::utility_http::SourceUnicodeProfile,
    pub source_pattern_logger: Arc<crate::utility_gateway::SourcePatternLogger>,
    pub source_crash_state: Arc<crate::utility_crash::SourceCrashState>,
}

/// The parent lifecycle owns this resource phase and awaits shutdown. Children
/// borrow/clone only the context; they cannot retain their own JoinSet owner.
pub struct UtilityRuntimeOwner { pub context: UtilityRuntimeContext, tasks: Arc<UtilityTaskScope> }
impl UtilityRuntimeOwner {
    pub fn new(output: impl Write + Send + 'static) -> Self {
        let tasks = Arc::new(UtilityTaskScope::reporting());
        let backoff = Arc::new(SourceErrorBackoff::default());
        let profile = if usize::BITS == 32 { crate::utility_http::HeaderAllocationProfile::GO126_32 }
            else { crate::utility_http::HeaderAllocationProfile::GO126_64 };
        let error_handlers = Arc::new(crate::utility_error_handlers::SourceErrorHandlerState::agones_defaults(Arc::clone(&backoff), profile));
        Self { context: UtilityRuntimeContext {
            source_logger: Arc::new(SourceLogger::new(output)), source_error_backoff: backoff,
            source_klog_state: Arc::new(crate::utility_klog::SourceKlogState::new(profile)),
            source_error_handler_state: error_handlers,
            utility_task_scope: Arc::downgrade(&tasks),
            source_signal_state: Arc::default(),
            source_query_settings: Arc::default(),
            source_unicode_profile: crate::utility_http::SourceUnicodeProfile::Go1265,
            source_pattern_logger: Arc::new(crate::utility_gateway::SourcePatternLogger::stderr()),
            source_crash_state: Arc::default(),
        }, tasks }
    }
    pub fn stderr() -> Self { Self::new(std::io::stderr()) }
    pub async fn shutdown(self) -> Vec<(String, BoxError)> {
        let signal_error = self.context.source_signal_state.close().err();
        let flush_error = self.context.stop_source_klog_flush_daemon().err();
        let mut errors = self.tasks.cancel_and_join().await;
        if let Some(error) = signal_error { errors.push(("process signal cleanup".into(), error)); }
        if let Some(error) = flush_error { errors.push(("klog flush daemon cleanup".into(), Box::new(error))); }
        errors
    }
}
impl Drop for UtilityRuntimeOwner {
    fn drop(&mut self) {
        self.context.source_klog_state.output.flush_daemon.request_stop();
        if let Err(error) = self.context.source_signal_state.close() { eprintln!("process signal cleanup: {error}"); }
    }
}
