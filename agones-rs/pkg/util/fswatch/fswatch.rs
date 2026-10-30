// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/fswatch/fswatch.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case)]
use std::{path::Path, pin::Pin, sync::Arc, time::Duration};
use futures::future::BoxFuture;
use tokio::time::{Instant, Sleep};
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, logfields::LogEntry,
    utility_fswatch::{CanOpenFileWatcher, NativeWatch, OpenedWatch, WatchEvents},
    utility_scope::CanSpawnUtilityTask, utility_logging::{CanEmitSourceLog, LogLevel}};

pub type ProcessEvent = Arc<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync>;
pub type ErrorEvent = Arc<dyn Fn(BoxError) -> BoxFuture<'static, ()> + Send + Sync>;

pub struct WatchHandle { watch: Arc<NativeWatch> }
impl WatchHandle { pub async fn cancel(&self) -> Result<(), BoxError> { self.watch.cancel().await } }
impl Drop for WatchHandle { fn drop(&mut self) { let _ = self.watch.begin_close(); } }
struct BatchOwner(Arc<NativeWatch>);
impl Drop for BatchOwner { fn drop(&mut self) { let _ = self.0.begin_close(); } }

pub async fn Watch<Context>(context: &Context, logger: &LogEntry, path: &Path, batchFor: Duration, processEvent: ProcessEvent) -> Result<WatchHandle, BoxError>
where Context: CanOpenFileWatcher + CanEmitSourceLog + CanSpawnUtilityTask + Clone + Send + Sync + 'static,
{
    let logger = logger.with_field("path", path.to_string_lossy().into_owned());
    let OpenedWatch { watch, events } = context.open_file_watcher(path).await?;
    let runtime = context.clone();
    // Agones' callback captures its successful construction error rather than
    // its argument. Preserve the source error:null field with native Option.
    let on_error: ErrorEvent = Arc::new(move |_error| {
        let (runtime, logger) = (runtime.clone(), logger.clone());
        Box::pin(async move { runtime.emit_source_log(&logger.with_error(None), LogLevel::Error, "error watching path"); })
    });
    let owner = BatchOwner(Arc::clone(&watch));
    let callbacks = Box::pin(async move {
        let result = batchWatch(batchFor, events, owner.0.cancel.clone(), processEvent, on_error).await;
        let close = owner.0.cancel().await;
        match (result, close) {
            (Err(error), Err(cleanup)) => Err(Errors::default().Wrap(Some(error), &format!("watch cleanup failed: {cleanup}"))
                .expect("watch callback error is present")),
            (Err(error), _) | (_, Err(error)) => Err(error),
            (Ok(()), Ok(())) => Ok(()),
        }
    });
    if let Err(callbacks) = context.spawn_utility_task_or_return("filesystem watch batch callbacks", callbacks) {
        let _ = watch.begin_close();
        callbacks.await?;
        return Err(Errors::default().New("filesystem watcher owner rejected its callback task"));
    }
    Ok(WatchHandle { watch })
}

pub async fn batchWatch(batchFor: Duration, mut events: WatchEvents, cancel: CancellationToken,
    processEvent: ProcessEvent, onError: ErrorEvent) -> Result<(), BoxError> {
    let mut timer: Option<Pin<Box<Sleep>>> = None;
    loop {
        if cancel.is_cancelled() { return Ok(()); }
        tokio::select! {
            _ = cancel.cancelled() => return Ok(()),
            _ = events.changes.notified() => {
                if timer.is_none() {
                    let deadline = Instant::now().checked_add(batchFor)
                        .ok_or_else(|| Errors::default().New("filesystem batch duration exceeds monotonic clock range"))?;
                    timer = Some(Box::pin(tokio::time::sleep_until(deadline)));
                }
            }
            _ = async { match &mut timer { Some(timer) => timer.as_mut().await, None => futures::future::pending::<()>().await } } => {
                if cancel.is_cancelled() { return Ok(()); }
                processEvent().await;
                timer = None;
            }
            error = events.errors.recv() => match error {
                Some(error) => onError(error).await,
                None if cancel.is_cancelled() => return Ok(()),
                None => return Err(Errors::default().New("filesystem watcher notification stream closed unexpectedly")),
            },
        }
    }
}
