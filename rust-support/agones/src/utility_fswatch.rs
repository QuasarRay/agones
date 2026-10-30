//! Filesystem notifications supplied by notify, owned through CGP capabilities.
//! Application tasks belong to UtilityTaskScope. The library owns its native
//! backend; callback destruction acknowledges the end of that backend lifetime.
use std::{path::Path, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, time::Duration};
use cgp::prelude::*;
use futures::future::BoxFuture;
use notify::{Config, Event, EventHandler, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{AccessKind, AccessMode};
use tokio::sync::{mpsc, Notify};
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, utility_scope::CanSpawnUtilityTask};

#[derive(Default)]
struct BackendCompletion { closed: AtomicBool, changed: Notify }
impl BackendCompletion {
    fn finish(&self) { self.closed.store(true, Ordering::Release); self.changed.notify_waiters(); }
    async fn wait(&self) {
        loop {
            let changed = self.changed.notified(); tokio::pin!(changed); changed.as_mut().enable();
            if self.closed.load(Ordering::Acquire) { return; }
            changed.await;
        }
    }
}
struct EventBridge {
    changes: Arc<Notify>, errors: mpsc::UnboundedSender<BoxError>,
    cancel: CancellationToken, completion: Arc<BackendCompletion>,
}
impl EventHandler for EventBridge {
    fn handle_event(&mut self, event: notify::Result<Event>) {
        if self.cancel.is_cancelled() { return; }
        match event {
            Ok(Event { kind: EventKind::Access(AccessKind::Close(AccessMode::Write)), .. }) => self.changes.notify_one(),
            // Reading the certificates must not trigger another reload.
            Ok(Event { kind: EventKind::Access(_), .. }) => (),
            Ok(_) => self.changes.notify_one(),
            Err(error) => { let _ = self.errors.send(Box::new(error)); }
        }
    }
}
impl Drop for EventBridge { fn drop(&mut self) { self.completion.finish(); } }
struct BackendState { watcher: Option<RecommendedWatcher>, close_error: Option<String> }
pub struct NativeWatch {
    backend: Mutex<BackendState>,
    pub cancel: CancellationToken,
    completion: Arc<BackendCompletion>,
}
impl NativeWatch {
    pub fn begin_close(&self) -> Result<(), BoxError> {
        self.cancel.cancel();
        let mut backend = self.backend.lock().unwrap_or_else(|error| error.into_inner());
        if let Some(watcher) = backend.watcher.take() {
            // A native backend that has already failed can panic in its Drop
            // implementation. Keep cleanup idempotent and report that failure
            // through the owned lifetime task, including during unwinding.
            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(watcher))).is_err() {
                backend.close_error = Some("filesystem watcher destructor panicked".into());
            }
        }
        match &backend.close_error { Some(error) => Err(Errors::default().New(error.clone())), None => Ok(()) }
    }
    pub async fn cancel(&self) -> Result<(), BoxError> {
        let result = self.begin_close();
        self.completion.wait().await;
        result
    }
}
impl Drop for NativeWatch { fn drop(&mut self) { let _ = self.begin_close(); } }
pub struct WatchEvents { pub changes: Arc<Notify>, pub errors: mpsc::UnboundedReceiver<BoxError> }
pub struct OpenedWatch { pub watch: Arc<NativeWatch>, pub events: WatchEvents }
struct LifetimeOwner(Arc<NativeWatch>);
impl Drop for LifetimeOwner { fn drop(&mut self) { let _ = self.0.begin_close(); } }

#[cgp_component(FileWatcherOpener)]
pub trait CanOpenFileWatcher: Sync {
    fn open_file_watcher<'a>(&'a self, path: &'a Path) -> BoxFuture<'a, Result<OpenedWatch, BoxError>>;
}
#[cgp_new_provider]
impl<Context> FileWatcherOpener<Context> for OpenNotifyFileWatcher
where Context: CanSpawnUtilityTask + Sync,
{
    fn open_file_watcher<'a>(context: &'a Context, path: &'a Path) -> BoxFuture<'a, Result<OpenedWatch, BoxError>> {
        Box::pin(async move {
            let cancel = context.utility_child_token()?;
            if cancel.is_cancelled() { return Err(Errors::default().New("filesystem watcher owner is closing")); }
            let completion = Arc::new(BackendCompletion::default());
            let changes = Arc::new(Notify::new());
            let (errors, error_receiver) = mpsc::unbounded_channel();
            let bridge = EventBridge { changes: Arc::clone(&changes), errors,
                cancel: cancel.clone(), completion: Arc::clone(&completion) };
            // Native backends ignore the polling interval. The library's
            // portable polling fallback also checks cancellation promptly.
            let config = Config::default().with_poll_interval(Duration::from_secs(1));
            let watcher = match RecommendedWatcher::new(bridge, config) {
                Ok(watcher) => watcher,
                Err(error) => { completion.wait().await; return Err(Box::new(error) as BoxError); }
            };
            let watch = Arc::new(NativeWatch { backend: Mutex::new(BackendState { watcher: Some(watcher), close_error: None }), cancel, completion });
            let added = watch.backend.lock().unwrap_or_else(|error| error.into_inner()).watcher.as_mut()
                .expect("new native watcher").watch(path, RecursiveMode::NonRecursive);
            if let Err(error) = added {
                if let Err(cleanup) = watch.cancel().await {
                    return Err(Errors::default().Wrap(Some(Box::new(error)), &format!("watch startup cleanup failed: {cleanup}"))
                        .expect("watch registration error is present"));
                }
                return Err(Box::new(error) as BoxError);
            }
            // The guard is captured before the future is polled so rejection
            // or abort still requests backend shutdown through native RAII.
            let owner = LifetimeOwner(Arc::clone(&watch));
            let lifetime = Box::pin(async move {
                tokio::select! {
                    _ = owner.0.cancel.cancelled() => (),
                    _ = owner.0.completion.wait() => (),
                }
                let unexpected = !owner.0.cancel.is_cancelled();
                owner.0.cancel().await?;
                if unexpected { Err(Errors::default().New("filesystem watcher backend stopped unexpectedly")) } else { Ok(()) }
            });
            if let Err(lifetime) = context.spawn_utility_task_or_return("filesystem watcher backend lifetime", lifetime) {
                let _ = watch.begin_close();
                lifetime.await?;
                return Err(Errors::default().New("filesystem watcher owner rejected its lifetime task"));
            }
            if watch.cancel.is_cancelled() {
                watch.cancel().await?;
                return Err(Errors::default().New("filesystem watcher owner is closing"));
            }
            Ok(OpenedWatch { watch, events: WatchEvents { changes, errors: error_receiver } })
        })
    }
}
