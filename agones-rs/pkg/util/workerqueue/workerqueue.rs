// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/workerqueue/workerqueue.go. Copyright Contributors to Agones a Series
// of LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
use std::{error::Error, fmt, sync::{Arc, Mutex, RwLock}};
use derive_deftly::Deftly;
use futures::{future::BoxFuture, FutureExt};
use tokio_util::sync::CancellationToken;
use crate::{errors::{BoxError, Errors}, logfields::{LogEntry, ResourceType}, runtime,
    utility_runtime::UtilityRuntimeContext, utility_scope::CanSpawnUtilityTask,
    utility_crash::{CanHandleSourceCrash, SourceCrashContext, SourcePanicHandlers},
    utility_logging::{CanEmitSourceLog, LogLevel}, utility_admission::Status,
    utility_queue::{SourceQueueContext, CanUseSourceQueue, CanReadSourceQueueKey, CanReadSourceQueueClock, CanDecideSourceQueueDelay,
        CanCallSourceQueueHandler, CanClassifySourceQueueError},
    utility_queue_limiters::{self, SourceRateLimiter, ItemFastSlowRateLimiter, DefaultControllerRateLimiter},
    utility_queue_values::{QueueItem, QueueKey, QueueObject},
    utility_declarations::derive_deftly_template_SourceErrorOrigin};

pub const workFx: i64 = 1_000_000_000;
#[derive(Debug)]
pub struct traceError { err: Option<BoxError> }
impl fmt::Display for traceError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { match &self.err { Some(error) => error.fmt(out), None => out.write_str("<nil>") } }
}
// Source traceError has no Unwrap. Do not expose err through Error::source.
impl Error for traceError {}
pub fn NewTraceError(err: Option<BoxError>) -> BoxError { Box::new(traceError { err }) }
pub fn isTraceError(err: &(dyn Error + 'static)) -> bool { crate::errors::contains_error::<traceError>(err) }

#[derive(Debug)]
pub struct ApiStatusError { pub status: Status }
impl fmt::Display for ApiStatusError { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str(&self.status.message) } }
impl Error for ApiStatusError {}
impl crate::runtime::SourceLogType for ApiStatusError { const SOURCE_LOG_TYPE: &'static str = "*errors.StatusError"; }
impl crate::utility_error_view::SourceApiStatus for ApiStatusError { fn source_api_status(&self) -> Status { self.status.clone() } }
pub fn isConflict(error: &(dyn Error + 'static)) -> bool {
    let Some((reason, code)) = crate::errors::first_error_view(error, &|error| {
        if let Some(error) = error.downcast_ref::<ApiStatusError>() { return Some((error.status.reason.clone(), error.status.code)); }
        let error = error.downcast_ref::<crate::utility_error_view::ApiStatusEnvelope>()?;
        // reasonAndCodeForError calls Status separately for Reason and Code.
        Some((error.status().reason, error.status().code))
    }) else { return false; };
    const KNOWN_REASONS: &[&str] = &["Unauthorized", "Forbidden", "NotFound", "AlreadyExists", "Conflict", "Gone", "Invalid",
        "ServerTimeout", "StoreReadError", "Timeout", "TooManyRequests", "BadRequest", "MethodNotAllowed", "NotAcceptable",
        "RequestEntityTooLarge", "UnsupportedMediaType", "InternalError", "Expired", "ServiceUnavailable"];
    reason == "Conflict" || !KNOWN_REASONS.contains(&reason.as_str()) && code == 409
}

pub type Handler = Arc<dyn Fn(CancellationToken, QueueKey) -> BoxFuture<'static, Result<(), BoxError>> + Send + Sync>;
#[derive(Default)]
struct WorkerCounts { workers: i64, running: i64 }
#[derive(Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "agones.dev/agones/pkg/util/workerqueue", source_struct = "WorkerQueue", source_log_type = "*workerqueue.WorkerQueue")]
pub struct WorkerQueue<Limiter: SourceRateLimiter = DefaultControllerRateLimiter> {
    logger: LogEntry,
    errs: Errors,
    keyName: String,
    queue: SourceQueueContext<Limiter>,
    pub SyncHandler: RwLock<Option<Handler>>,
    mu: Mutex<WorkerCounts>,
}
pub fn FastRateLimiter(maxDelay: i64) -> ItemFastSlowRateLimiter { ItemFastSlowRateLimiter::new(200_000_000, maxDelay, 5) }
pub fn ConstantRateLimiter(maxDelay: i64) -> ItemFastSlowRateLimiter { ItemFastSlowRateLimiter::new(200_000_000, maxDelay, 0) }
pub fn NewWorkerQueue(context: &UtilityRuntimeContext, handler: Option<Handler>, logger: Option<&LogEntry>, keyName: &ResourceType, queueName: &str) -> Result<Arc<WorkerQueue>, BoxError> {
    NewWorkerQueueWithRateLimiter(context, handler, logger, keyName, queueName, utility_queue_limiters::default_controller_limiter())
}
pub fn NewWorkerQueueWithRateLimiter<Limiter: SourceRateLimiter>(context: &UtilityRuntimeContext, handler: Option<Handler>, logger: Option<&LogEntry>, keyName: &ResourceType, queueName: &str, rateLimiter: Limiter) -> Result<Arc<WorkerQueue<Limiter>>, BoxError> {
    let logger = logger.expect("nil logrus.Entry in NewWorkerQueueWithRateLimiter").with_field("queue", queueName);
    let queue = SourceQueueContext::new(context, rateLimiter, queueName)?;
    Ok(Arc::new(WorkerQueue { logger, errs: Errors::FromStruct::<WorkerQueue<Limiter>>(None, None), keyName: keyName.to_string(), queue,
        SyncHandler: RwLock::new(handler), mu: Mutex::default() }))
}
impl<Limiter: SourceRateLimiter> WorkerQueue<Limiter> {
    async fn key_for(&self, object: &dyn QueueObject) -> Option<QueueKey> {
        match self.queue.source_queue_key(object) {
            Ok(key) => Some(key),
            Err(error) => {
                let error = self.errs.Wrap(Some(error), "Error creating key for object").expect("non-nil key error");
                runtime::HandleError(&self.queue.source_queue_runtime, Some(&object.log_object(&self.logger)), Some(error.as_ref())).await; None
            }
        }
    }
    pub async fn Enqueue(&self, object: &dyn QueueObject) {
        if let Some(key) = self.key_for(object).await {
            let item = QueueItem::string(key);
            self.queue.source_queue_runtime.emit_source_log(&item.with_log_field(&self.logger, &self.keyName), LogLevel::Trace, "Enqueuing");
            self.queue.add_rate_limited(item).await;
        }
    }
    pub async fn EnqueueImmediately(&self, object: &dyn QueueObject) {
        if let Some(key) = self.key_for(object).await {
            let item = QueueItem::string(key);
            self.queue.source_queue_runtime.emit_source_log(&item.with_log_field(&self.logger, &self.keyName), LogLevel::Trace, "Enqueuing immediately");
            self.queue.source_queue_add(item);
        }
    }
    pub async fn EnqueueAfter(&self, object: &dyn QueueObject, duration: i64) {
        if let Some(key) = self.key_for(object).await {
            let item = QueueItem::string(key);
            self.queue.source_queue_runtime.emit_source_log(&item.with_log_field(&self.logger, &self.keyName).with_field("duration", duration), LogLevel::Trace, "Enqueueing after duration");
            self.queue.source_queue_add_after(item, duration).await;
        }
    }
    async fn runWorker(&self, context: CancellationToken) { while self.processNextWorkItem(context.clone()).await {} }
    async fn processNextWorkItem(&self, context: CancellationToken) -> bool {
        let Some(object) = self.queue.source_queue_get().await else { return false; };
        struct Done<'a, L: SourceRateLimiter> { queue: &'a SourceQueueContext<L>, object: &'a QueueItem }
        impl<L: SourceRateLimiter> Drop for Done<'_, L> { fn drop(&mut self) { self.queue.source_queue_done(self.object); } }
        let _done = Done { queue: &self.queue, object: &object };
        self.queue.source_queue_runtime.emit_source_log(&object.with_log_field(&self.logger, &self.keyName), LogLevel::Debug, "Processing");
        let Some(key) = object.string_key().cloned() else {
            let error = self.errs.New(format!("expected string in queue, but got {}", object.source_type()));
            runtime::HandleError(&self.queue.source_queue_runtime, Some(&object.with_log_field(&self.logger, &self.keyName)), Some(error.as_ref())).await;
            self.queue.source_queue_forget(&object); return true;
        };
        let handler = self.SyncHandler.read().unwrap_or_else(|error| error.into_inner()).clone().unwrap_or_else(||
            crate::utility_panic::unwind_source_runtime_error(crate::utility_panic::SourceRuntimeErrorString::new("invalid memory address or nil pointer dereference")));
        match self.queue.source_queue_call_handler(&handler, context, key).await {
            Err(error) => {
                if self.queue.source_queue_expected_error(error.as_ref()) {
                    self.queue.source_queue_runtime.emit_source_log(&object.with_log_field(&self.logger, &self.keyName), LogLevel::Trace, &error.to_string());
                } else {
                    runtime::HandleError(&self.queue.source_queue_runtime, Some(&object.with_log_field(&self.logger, &self.keyName)), Some(error.as_ref())).await;
                }
                // Do not Forget a failed item. Done runs after AddRateLimited,
                // including its bounded-channel wait and immediate dirty path.
                self.queue.add_rate_limited(object.clone()).await;
            }
            Ok(()) => self.queue.source_queue_forget(&object),
        }
        true
    }
    pub async fn Run(self: &Arc<Self>, context: CancellationToken, workers: i64) -> Result<(), BoxError> {
        self.setWorkerCount(workers);
        let runtime = &self.queue.source_queue_runtime;
        runtime.emit_source_log(&self.logger.with_field("workers", workers), LogLevel::Info, "Starting workers...");
        let owner_stop = runtime.utility_child_token()?;
        let worker_context = context.child_token();
        for index in 0..workers {
            let (queue, child_context) = (Arc::clone(self), worker_context.clone());
            if let Err(error) = runtime.spawn_utility_task(&format!("{} source workerqueue worker {index}", self.queue.source_queue_name), Box::pin(async move { queue.run(child_context).await; Ok(()) })) {
                worker_context.cancel(); self.queue.source_queue_shutdown(); return Err(error);
            }
        }
        tokio::select! { _ = context.cancelled() => (), _ = owner_stop.cancelled() => () }
        worker_context.cancel(); runtime.emit_source_log(&self.logger, LogLevel::Info, "...shutting down workers");
        self.queue.source_queue_shutdown();
        // Source Run returns after shutdown, without joining the workers.
        // The CGP parent owns those tasks and awaits their separate drain.
        Ok(())
    }
    async fn run(self: Arc<Self>, context: CancellationToken) {
        self.inc();
        struct Running<L: SourceRateLimiter>(Arc<WorkerQueue<L>>);
        impl<L: SourceRateLimiter> Drop for Running<L> { fn drop(&mut self) { self.0.dec(); } }
        let _running = Running(Arc::clone(&self));
        loop {
            if context.is_cancelled() { return; }
            if let Err(payload) = std::panic::AssertUnwindSafe(self.runWorker(context.clone())).catch_unwind().await {
                self.queue.source_queue_runtime.handle_source_crash(SourceCrashContext::new(context.clone()),
                    crate::utility_panic::SourceRecoveredPanic::new(payload), &SourcePanicHandlers::nil());
            }
            // Sliding Until prepares its delay after f/HandleCrash, then
            // selects cancellation. It rechecks stop before the next f.
            let delay = self.queue.source_queue_sleep(workFx);
            tokio::select! { _ = context.cancelled() => return, _ = delay => () }
        }
    }
    pub fn Healthy(&self) -> Result<(), BoxError> {
        let counts = self.mu.lock().unwrap_or_else(|error| error.into_inner());
        if counts.workers != counts.running { return Err(Errors::default().New(format!("want {} worker goroutine(s), got {}", counts.workers, counts.running))); } Ok(())
    }
    pub fn RunCount(&self) -> i64 { self.mu.lock().unwrap_or_else(|error| error.into_inner()).running }
    fn setWorkerCount(&self, workers: i64) { self.mu.lock().unwrap_or_else(|error| error.into_inner()).workers = workers; }
    fn inc(&self) { let mut counts = self.mu.lock().unwrap_or_else(|error| error.into_inner()); counts.running = counts.running.wrapping_add(1); }
    fn dec(&self) { let mut counts = self.mu.lock().unwrap_or_else(|error| error.into_inner()); counts.running = counts.running.wrapping_sub(1); }
}
