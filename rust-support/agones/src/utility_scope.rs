//! Child-task ownership used by the statically composed utility effects.
//! Contexts hold a Weak reference so a pending child cannot retain its own
//! supervisor. The application owner cancels and joins before releasing it.
use std::sync::{Mutex, Weak, atomic::{AtomicBool, Ordering}};
use futures::future::BoxFuture;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use cgp::prelude::*;
use crate::errors::{BoxError, Errors};

pub struct UtilityTaskScope {
    closing: AtomicBool,
    cancel: CancellationToken,
    tasks: Mutex<JoinSet<(String, Result<(), BoxError>)>>,
    completed_errors: Mutex<Vec<(String, BoxError)>>,
}
impl Default for UtilityTaskScope {
    fn default() -> Self {
        Self { closing: AtomicBool::new(false), cancel: CancellationToken::new(), tasks: Mutex::new(JoinSet::new()), completed_errors: Mutex::new(Vec::new()) }
    }
}
impl UtilityTaskScope {
    pub fn child_token(&self) -> CancellationToken { self.cancel.child_token() }
    pub fn spawn(&self, name: &str, future: BoxFuture<'static, Result<(), BoxError>>) -> Result<(), BoxError> {
        self.spawn_or_return(name, future)
            .map_err(|_future| Errors::default().New("utility task scope is shutting down"))
    }
    /// Return ownership on rejection so a caller transferring live resources
    /// can still cancel and join them before returning.
    pub fn spawn_or_return(&self, name: &str, future: BoxFuture<'static, Result<(), BoxError>>)
        -> Result<(), BoxFuture<'static, Result<(), BoxError>>>
    {
        let mut tasks = self.tasks.lock().unwrap_or_else(|error| error.into_inner());
        if self.closing.load(Ordering::Acquire) {
            return Err(future);
        }
        // Long-lived servers must reap completed request tasks rather than
        // retain one JoinSet entry per request until process shutdown.
        while let Some(result) = tasks.try_join_next() {
            match result {
                Ok((name, Err(error))) => self.completed_errors.lock().unwrap_or_else(|error| error.into_inner()).push((name, error)),
                Err(error) => self.completed_errors.lock().unwrap_or_else(|error| error.into_inner()).push(("utility child".into(), Box::new(error))),
                Ok((_, Ok(()))) => (),
            }
        }
        let name = name.to_owned();
        tasks.spawn(async move { (name, future.await) });
        Ok(())
    }
    pub async fn cancel_and_join(&self) -> Vec<(String, BoxError)> {
        let mut tasks = {
            let mut owned = self.tasks.lock().unwrap_or_else(|error| error.into_inner());
            self.closing.store(true, Ordering::Release);
            self.cancel.cancel();
            std::mem::replace(&mut *owned, JoinSet::new())
        };
        let mut errors = std::mem::take(&mut *self.completed_errors.lock().unwrap_or_else(|error| error.into_inner()));
        while let Some(result) = tasks.join_next().await {
            match result {
                Ok((name, Err(error))) => errors.push((name, error)),
                Err(error) => errors.push(("utility child".into(), Box::new(error) as BoxError)),
                Ok((_, Ok(()))) => (),
            }
        }
        errors
    }
}
impl Drop for UtilityTaskScope {
    fn drop(&mut self) {
        self.cancel.cancel();
        self.tasks.get_mut().unwrap_or_else(|error| error.into_inner()).abort_all();
    }
}

#[cgp_getter]
pub trait HasUtilityTaskScope { fn utility_task_scope(&self) -> &Weak<UtilityTaskScope>; }
#[cgp_component(UtilityTaskSpawner)]
pub trait CanSpawnUtilityTask {
    fn utility_child_token(&self) -> Result<CancellationToken, BoxError>;
    fn spawn_utility_task(&self, name: &str, future: BoxFuture<'static, Result<(), BoxError>>) -> Result<(), BoxError>;
}
#[cgp_new_provider]
impl<Context: HasUtilityTaskScope> UtilityTaskSpawner<Context> for SpawnOwnedUtilityTask {
    fn utility_child_token(context: &Context) -> Result<CancellationToken, BoxError> {
        context.utility_task_scope().upgrade().map(|scope| scope.child_token())
            .ok_or_else(|| Errors::default().New("utility task owner has been released"))
    }
    fn spawn_utility_task(context: &Context, name: &str, future: BoxFuture<'static, Result<(), BoxError>>) -> Result<(), BoxError> {
        context.utility_task_scope().upgrade()
            .ok_or_else(|| Errors::default().New("utility task owner has been released"))?.spawn(name, future)
    }
}
