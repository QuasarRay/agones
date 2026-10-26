// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/fswatch/fswatch.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case)]
use std::{path::Path, pin::Pin, sync::Arc, time::Duration};
use futures::future::BoxFuture;
use tokio::time::{Instant, Sleep};
use tokio_util::sync::CancellationToken;
use crate::{errors::BoxError, logfields::LogEntry,
    utility_fswatch::{CanOpenFileWatcher, NativeWatch, Rendezvous}, utility_runtime::UtilityRuntimeContext,
    utility_scope::CanSpawnUtilityTask, utility_logging::{CanEmitSourceLog, LogLevel}};

pub type ProcessEvent = Arc<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync>;
pub type ErrorEvent = Arc<dyn Fn(Option<BoxError>) -> BoxFuture<'static, ()> + Send + Sync>;

pub struct WatchHandle { watch: Arc<NativeWatch> }
impl WatchHandle {
    pub async fn cancel(&self) { self.watch.cancel().await; }
}
impl Drop for WatchHandle { fn drop(&mut self) { self.watch.cancel_on_owner_drop(); } }
struct BatchOwner(Arc<NativeWatch>);
impl Drop for BatchOwner { fn drop(&mut self) { self.0.cancel_on_owner_drop(); } }

pub fn Watch(context: &UtilityRuntimeContext, logger: &LogEntry, path: &Path, batchFor: i64, processEvent: ProcessEvent) -> Result<WatchHandle, BoxError> {
    let logger = logger.with_field("path", path.to_string_lossy().into_owned());
    let watch = context.open_file_watcher(path)?;
    let runtime = context.clone();
    // The source callback captures the successful NewWatcher err (nil), not
    // its callback argument. Retain that observable error:null log field.
    let on_error: ErrorEvent = Arc::new(move |_ignored| {
        let (runtime, logger) = (runtime.clone(), logger.clone());
        Box::pin(async move { runtime.emit_source_log(&logger.with_error(None), LogLevel::Error, "error watching path"); })
    });
    let owner = BatchOwner(Arc::clone(&watch));
    context.spawn_utility_task("fswatch batch callbacks", Box::pin(async move {
        batchWatch(batchFor, &owner.0.events, &owner.0.errors, owner.0.cancel.clone(), processEvent, on_error).await;
        Ok(())
    }))?;
    Ok(WatchHandle { watch })
}

pub async fn batchWatch(batchFor: i64, events: &Rendezvous<()>, errors: &Rendezvous<BoxError>, cancelChan: CancellationToken,
    processEvent: ProcessEvent, onError: ErrorEvent) {
    // Go 1.26 Reset discards a stale timer tick. A disabled Option represents
    // the original nil timerCh until the first event; no callback runs early.
    let mut timer: Option<Pin<Box<Sleep>>> = None;
    loop {
        tokio::select! {
            _ = events.recv() => {
                if timer.is_none() {
                    let delay = Duration::from_nanos(batchFor.max(0) as u64);
                    timer = Some(Box::pin(tokio::time::sleep_until(Instant::now() + delay)));
                }
            }
            _ = async { match &mut timer { Some(timer) => timer.as_mut().await, None => futures::future::pending::<()>().await } } => {
                processEvent().await;
                timer = None; // after the callback, as in the source
            }
            error = errors.recv() => onError(error).await,
            _ = cancelChan.cancelled() => return,
        }
    }
}
