//! Shared utility resources, constructed and consumed through CGP.
use std::{io::Write, sync::{Arc, Weak}};
use cgp::prelude::*;
use derive_deftly::Deftly;
use crate::{errors::BoxError, utility_logging::{SourceLogger, SourceErrorBackoff}, utility_scope::UtilityTaskScope,
    utility_declarations::derive_deftly_template_UtilityRuntimeProviders};

#[derive(Clone, HasField, Deftly)]
#[derive_deftly(UtilityRuntimeProviders)]
pub struct UtilityRuntimeContext {
    pub source_logger: Arc<SourceLogger>,
    pub source_error_backoff: Arc<SourceErrorBackoff>,
    pub utility_task_scope: Weak<UtilityTaskScope>,
}

/// The parent lifecycle owns this resource phase and awaits shutdown. Children
/// borrow/clone only the context; they cannot retain their own JoinSet owner.
pub struct UtilityRuntimeOwner { pub context: UtilityRuntimeContext, tasks: Arc<UtilityTaskScope> }
impl UtilityRuntimeOwner {
    pub fn new(output: impl Write + Send + 'static) -> Self {
        let tasks = Arc::new(UtilityTaskScope::default());
        Self { context: UtilityRuntimeContext {
            source_logger: Arc::new(SourceLogger::new(output)), source_error_backoff: Arc::default(),
            utility_task_scope: Arc::downgrade(&tasks),
        }, tasks }
    }
    pub fn stderr() -> Self { Self::new(std::io::stderr()) }
    pub async fn shutdown(self) -> Vec<(String, BoxError)> { self.tasks.cancel_and_join().await }
}
