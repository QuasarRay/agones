//! Child-task ownership used by the statically composed utility effects.
//! Contexts hold a Weak reference so a pending child cannot retain its own
//! supervisor. The application owner cancels and joins before releasing it.
use std::{sync::{Arc, Mutex, Weak, atomic::{AtomicBool, Ordering}}, panic::AssertUnwindSafe};
use futures::{future::BoxFuture, FutureExt};
use tokio::{task::JoinSet, sync::Notify};
use tokio_util::sync::CancellationToken;
use cgp::prelude::*;
use crate::errors::{BoxError, Errors};
use crate::utility_runtime_panic::CanHandleSourceUtilityPanic;

pub type UtilityBlockingTask = Box<dyn FnOnce() -> Result<(), BoxError> + Send + 'static>;

pub struct UtilityTaskScope {
    closing: AtomicBool,
    cancel: CancellationToken,
    tasks: Mutex<JoinSet<(String, Result<(), BoxError>)>>,
    completed_errors: Mutex<Vec<(String, BoxError)>>,
    failures: Option<Arc<FirstFailure>>,
    runtime: Option<tokio::runtime::Handle>,
}
#[derive(Clone, Debug)]
pub enum ChildFailureKind { ReturnedError, Panic }
#[derive(Clone, Debug)]
pub struct ChildFailure { pub task: String, pub kind: ChildFailureKind }
#[derive(Default)]
struct FirstFailure { value: Mutex<Option<ChildFailure>>, changed: Notify }
impl FirstFailure {
    fn report(&self, failure: ChildFailure) {
        let mut first = self.value.lock().unwrap_or_else(|error| error.into_inner());
        if first.is_none() { *first = Some(failure); self.changed.notify_waiters(); }
    }
    async fn wait(&self, stop: CancellationToken) -> Result<ChildFailure, BoxError> {
        loop {
            let changed = self.changed.notified();
            if let Some(first) = self.value.lock().unwrap_or_else(|error| error.into_inner()).clone() { return Ok(first); }
            tokio::select! {
                _ = changed => (),
                _ = stop.cancelled() => return Err(Errors::default().New("utility task owner is shutting down")),
            }
        }
    }
}
impl Default for UtilityTaskScope {
    fn default() -> Self {
        Self { closing: AtomicBool::new(false), cancel: CancellationToken::new(), tasks: Mutex::new(JoinSet::new()),
            completed_errors: Mutex::new(Vec::new()), failures: None, runtime: tokio::runtime::Handle::try_current().ok() }
    }
}
impl UtilityTaskScope {
    pub fn reporting() -> Self { Self { failures: Some(Arc::default()), ..Self::default() } }
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
        self.reap_completed(&mut tasks);
        let name = name.to_owned();
        let failures = self.failures.clone();
        tasks.spawn(async move {
            match AssertUnwindSafe(future).catch_unwind().await {
                Ok(result) => {
                    if let (Some(failures), Err(_)) = (&failures, &result) {
                        // Observation does not call an error's user-supplied
                        // Display/Error method or duplicate its owned value.
                        failures.report(ChildFailure { task: name.clone(), kind: ChildFailureKind::ReturnedError });
                    }
                    (name, result)
                }
                Err(payload) => {
                    if let Some(failures) = failures {
                        failures.report(ChildFailure { task: name, kind: ChildFailureKind::Panic });
                    }
                    // Retain the original panic payload in the JoinError. The
                    // CGP parent is awakened immediately and owns fatal policy.
                    std::panic::resume_unwind(payload)
                }
            }
        });
        Ok(())
    }
    /// Synchronous source effects may wait for work which must keep running
    /// after their timeout. Keep that work in the same parent-owned JoinSet.
    /// Rejection returns the closure without executing or losing ownership.
    pub fn spawn_blocking_or_return(&self, name: &str, work: UtilityBlockingTask) -> Result<(), UtilityBlockingTask> {
        let mut tasks = self.tasks.lock().unwrap_or_else(|error| error.into_inner());
        if self.closing.load(Ordering::Acquire) { return Err(work); }
        // Logging may originate on an ordinary native thread. Use the
        // application's retained runtime handle instead of requiring a Tokio
        // entry guard on the calling thread. No runtime is created here.
        let Some(runtime) = self.runtime.clone().or_else(|| tokio::runtime::Handle::try_current().ok()) else { return Err(work); };
        self.reap_completed(&mut tasks);
        let name = name.to_owned();
        let failures = self.failures.clone();
        tasks.spawn_blocking_on(move || {
            match std::panic::catch_unwind(AssertUnwindSafe(work)) {
                Ok(result) => {
                    if let (Some(failures), Err(_)) = (&failures, &result) {
                        failures.report(ChildFailure { task: name.clone(), kind: ChildFailureKind::ReturnedError });
                    }
                    (name, result)
                }
                Err(payload) => {
                    if let Some(failures) = failures {
                        failures.report(ChildFailure { task: name, kind: ChildFailureKind::Panic });
                    }
                    std::panic::resume_unwind(payload)
                }
            }
        }, &runtime);
        Ok(())
    }
    fn reap_completed(&self, tasks: &mut JoinSet<(String, Result<(), BoxError>)>) {
        // Long-lived servers must reap completed work rather than retain one
        // JoinSet entry per call until process shutdown.
        while let Some(result) = tasks.try_join_next() {
            match result {
                Ok((name, Err(error))) if self.failures.is_some() => self.completed_errors.lock().unwrap_or_else(|error| error.into_inner()).push((name, error)),
                Err(error) if self.failures.is_some() => self.completed_errors.lock().unwrap_or_else(|error| error.into_inner()).push(("utility child".into(), Box::new(error))),
                Ok((_, Err(_))) | Err(_) => (),
                Ok((_, Ok(()))) => (),
            }
        }
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
                Ok((name, Err(error))) if self.failures.is_some() => errors.push((name, error)),
                Err(error) if self.failures.is_some() => errors.push(("utility child".into(), Box::new(error) as BoxError)),
                Ok((_, Err(_))) | Err(_) => (),
                Ok((_, Ok(()))) => (),
            }
        }
        errors
    }
}

#[cgp_component(UtilityTaskFailureObserver)]
pub trait CanObserveUtilityTaskFailure {
    fn first_utility_task_failure(&self) -> BoxFuture<'_, Result<ChildFailure, BoxError>>;
}
#[cgp_new_provider]
impl<Context: HasUtilityTaskScope> UtilityTaskFailureObserver<Context> for ObserveOwnedUtilityTaskFailure {
    fn first_utility_task_failure(context: &Context) -> BoxFuture<'_, Result<ChildFailure, BoxError>> {
        let observation = context.utility_task_scope().upgrade().and_then(|scope| scope.failures.clone().map(|failures| (failures, scope.cancel.clone())));
        Box::pin(async move {
            let (failures, stop) = observation.ok_or_else(|| Errors::default().New("reporting utility task owner is unavailable"))?;
            failures.wait(stop).await
        })
    }
}
impl Drop for UtilityTaskScope {
    fn drop(&mut self) {
        self.cancel.cancel();
        // Tokio cannot abort blocking work which has already begun. Normal
        // lifecycle shutdown must await cancel_and_join; fatal process exit
        // remains a distinct source effect.
        self.tasks.get_mut().unwrap_or_else(|error| error.into_inner()).abort_all();
    }
}

#[cgp_getter]
pub trait HasUtilityTaskScope { fn utility_task_scope(&self) -> &Weak<UtilityTaskScope>; }
#[cgp_component(UtilityTaskSpawner)]
pub trait CanSpawnUtilityTask {
    fn utility_child_token(&self) -> Result<CancellationToken, BoxError>;
    fn spawn_utility_task(&self, name: &str, future: BoxFuture<'static, Result<(), BoxError>>) -> Result<(), BoxError>;
    fn spawn_utility_task_or_return(&self, name: &str, future: BoxFuture<'static, Result<(), BoxError>>)
        -> Result<(), BoxFuture<'static, Result<(), BoxError>>>;
    fn spawn_blocking_utility_task_or_return(&self, name: &str, work: UtilityBlockingTask) -> Result<(), UtilityBlockingTask>;
}
#[cgp_new_provider]
impl<Context> UtilityTaskSpawner<Context> for SpawnOwnedUtilityTask
where Context: HasUtilityTaskScope + CanHandleSourceUtilityPanic + Clone + Send + 'static,
{
    fn utility_child_token(context: &Context) -> Result<CancellationToken, BoxError> {
        context.utility_task_scope().upgrade().map(|scope| scope.child_token())
            .ok_or_else(|| Errors::default().New("utility task owner has been released"))
    }
    fn spawn_utility_task(context: &Context, name: &str, future: BoxFuture<'static, Result<(), BoxError>>) -> Result<(), BoxError> {
        context.utility_task_scope().upgrade()
            .ok_or_else(|| Errors::default().New("utility task owner has been released"))?.spawn(name, source_utility_task(context.clone(), future))
    }
    fn spawn_utility_task_or_return(context: &Context, name: &str, future: BoxFuture<'static, Result<(), BoxError>>)
        -> Result<(), BoxFuture<'static, Result<(), BoxError>>>
    {
        let future = source_utility_task(context.clone(), future);
        match context.utility_task_scope().upgrade() {
            Some(scope) => scope.spawn_or_return(name, future), None => Err(future),
        }
    }
    fn spawn_blocking_utility_task_or_return(context: &Context, name: &str, work: UtilityBlockingTask) -> Result<(), UtilityBlockingTask> {
        let parent = context.clone();
        let work: UtilityBlockingTask = Box::new(move || {
            match std::panic::catch_unwind(AssertUnwindSafe(work)) {
                Ok(result) => result,
                Err(payload) => parent.handle_source_utility_panic(crate::utility_panic::SourceRecoveredPanic::new(payload)),
            }
        });
        match context.utility_task_scope().upgrade() {
            Some(scope) => scope.spawn_blocking_or_return(name, work), None => Err(work),
        }
    }
}

fn source_utility_task<Context>(context: Context, future: BoxFuture<'static, Result<(), BoxError>>)
    -> BoxFuture<'static, Result<(), BoxError>>
where Context: CanHandleSourceUtilityPanic + Send + 'static,
{
    Box::pin(async move {
        match AssertUnwindSafe(future).catch_unwind().await {
            Ok(result) => result, // Returned errors retain their caller's policy.
            Err(payload) => context.handle_source_utility_panic(crate::utility_panic::SourceRecoveredPanic::new(payload)),
        }
    })
}
