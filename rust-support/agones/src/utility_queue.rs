//! Agones keyed work scheduling over Rust collections and Tokio effects.
//! CGP selects clock, retry, key and queue operations; derive-deftly composes
//! the concrete context. The application scope owns the delay task and joins it.
use std::{collections::{HashMap, HashSet, VecDeque}, sync::{Arc, Mutex}, time::Duration};
use cgp::prelude::*;
use derive_deftly::Deftly;
use futures::{future::BoxFuture, StreamExt};
use tokio::{sync::{mpsc, Notify}, time::Instant};
use tokio_util::{sync::CancellationToken, time::{DelayQueue, delay_queue}};
use crate::{errors::{BoxError, Errors}, utility_runtime::UtilityRuntimeContext, utility_scope::CanSpawnUtilityTask,
    utility_queue_limiters::SourceRateLimiter, utility_queue_values::{QueueKey, QueueObject, meta_namespace_key},
    utility_declarations::derive_deftly_template_UtilityQueueProviders};

#[cgp_component(SourceQueueClock)]
pub trait CanReadSourceQueueClock: Sync {
    fn source_queue_now(&self) -> Instant;
    fn source_queue_sleep(&self, duration: Duration) -> BoxFuture<'_, ()>;
}
#[cgp_new_provider]
impl<Context: Sync> SourceQueueClock<Context> for ReadTokioSourceQueueClock {
    fn source_queue_now(_context: &Context) -> Instant { Instant::now() }
    fn source_queue_sleep(_context: &Context, duration: Duration) -> BoxFuture<'_, ()> { Box::pin(tokio::time::sleep(duration)) }
}

#[derive(Default)]
struct ReadyState { queue: VecDeque<QueueKey>, dirty: HashSet<QueueKey>, processing: HashSet<QueueKey>, shutting_down: bool }
pub struct SourceQueueState { ready: Mutex<ReadyState>, readable: Notify, delays: mpsc::Sender<WaitEntry>, stop: CancellationToken }
impl SourceQueueState {
    pub fn add(&self, key: QueueKey) {
        let mut state = self.ready.lock().unwrap_or_else(|error| error.into_inner());
        if state.shutting_down || !state.dirty.insert(key.clone()) { return; }
        if !state.processing.contains(&key) { state.queue.push_back(key); self.readable.notify_one(); }
    }
    pub fn done(&self, key: &QueueKey) {
        let mut state = self.ready.lock().unwrap_or_else(|error| error.into_inner());
        if !state.processing.remove(key) { return; }
        // A key dirtied during processing is queued once more, including when
        // shutdown started after that update. New Add calls are then rejected.
        if state.dirty.contains(key) { state.queue.push_back(key.clone()); self.readable.notify_one(); }
    }
    pub async fn get(&self) -> Option<QueueKey> {
        loop {
            let readable = self.readable.notified(); tokio::pin!(readable); readable.as_mut().enable();
            {
                let mut state = self.ready.lock().unwrap_or_else(|error| error.into_inner());
                if let Some(key) = state.queue.pop_front() {
                    state.processing.insert(key.clone()); state.dirty.remove(&key); return Some(key);
                }
                if state.shutting_down { return None; }
            }
            readable.await;
        }
    }
    pub fn shutdown(&self) {
        let mut state = self.ready.lock().unwrap_or_else(|error| error.into_inner());
        state.shutting_down = true; self.stop.cancel(); self.readable.notify_waiters();
    }
    pub fn shutting_down(&self) -> bool { self.ready.lock().unwrap_or_else(|error| error.into_inner()).shutting_down }
    pub fn len(&self) -> usize { self.ready.lock().unwrap_or_else(|error| error.into_inner()).queue.len() }
}

#[cgp_getter]
pub trait HasSourceQueueState { fn source_queue_state(&self) -> &Arc<SourceQueueState>; }
#[cgp_getter]
pub trait HasSourceQueueRuntime { fn source_queue_runtime(&self) -> &UtilityRuntimeContext; }
pub trait HasSourceQueueLimiter { type Limiter: SourceRateLimiter; fn source_queue_limiter(&self) -> &Self::Limiter; }

#[cgp_component(SourceQueueHandlerCaller)]
pub trait CanCallSourceQueueHandler: Sync {
    fn source_queue_call_handler<'a>(&'a self, handler: &'a crate::source::workerqueue::Handler, stop: CancellationToken, key: QueueKey) -> BoxFuture<'a, Result<(), BoxError>>;
}
#[cgp_new_provider]
impl<Context: Sync> SourceQueueHandlerCaller<Context> for CallSourceQueueHandler {
    fn source_queue_call_handler<'a>(_context: &'a Context, handler: &'a crate::source::workerqueue::Handler, stop: CancellationToken, key: QueueKey) -> BoxFuture<'a, Result<(), BoxError>> { handler(stop, key) }
}
#[cgp_component(SourceQueueErrorClassifier)]
pub trait CanClassifySourceQueueError { fn source_queue_expected_error(&self, error: &(dyn std::error::Error + Send + Sync + 'static)) -> bool; }
#[cgp_new_provider]
impl<Context> SourceQueueErrorClassifier<Context> for ClassifySourceQueueError {
    fn source_queue_expected_error(_context: &Context, error: &(dyn std::error::Error + Send + Sync + 'static)) -> bool {
        crate::source::workerqueue::isConflict(error) || crate::source::workerqueue::isTraceError(error)
    }
}

#[cgp_component(SourceQueueDelayDecider)]
pub trait CanDecideSourceQueueDelay {
    fn source_queue_delay(&self, key: &QueueKey) -> Result<Duration, BoxError>;
    fn source_queue_forget(&self, key: &QueueKey);
    fn source_queue_num_requeues(&self, key: &QueueKey) -> usize;
}
#[cgp_new_provider]
impl<Context: HasSourceQueueLimiter + CanReadSourceQueueClock> SourceQueueDelayDecider<Context> for UseOwnedSourceQueueLimiter {
    fn source_queue_delay(context: &Context, key: &QueueKey) -> Result<Duration, BoxError> { context.source_queue_limiter().When(context, key) }
    fn source_queue_forget(context: &Context, key: &QueueKey) { context.source_queue_limiter().Forget(key); }
    fn source_queue_num_requeues(context: &Context, key: &QueueKey) -> usize { context.source_queue_limiter().NumRequeues(key) }
}
#[cgp_component(SourceQueueKeyReader)]
pub trait CanReadSourceQueueKey { fn source_queue_key(&self, object: &dyn QueueObject) -> Result<QueueKey, BoxError>; }
#[cgp_new_provider]
impl<Context> SourceQueueKeyReader<Context> for ReadSourceMetadataKey {
    fn source_queue_key(_context: &Context, object: &dyn QueueObject) -> Result<QueueKey, BoxError> { meta_namespace_key(object) }
}
#[cgp_component(SourceQueueOperations)]
pub trait CanUseSourceQueue: Sync {
    fn source_queue_add(&self, key: QueueKey);
    fn source_queue_add_after(&self, key: QueueKey, delay: Duration) -> BoxFuture<'_, Result<(), BoxError>>;
    fn source_queue_get(&self) -> BoxFuture<'_, Option<QueueKey>>;
    fn source_queue_done(&self, key: &QueueKey);
    fn source_queue_shutdown(&self);
}
#[cgp_new_provider]
impl<Context: HasSourceQueueState + CanReadSourceQueueClock + Sync> SourceQueueOperations<Context> for UseOwnedSourceQueue {
    fn source_queue_add(context: &Context, key: QueueKey) { context.source_queue_state().add(key); }
    fn source_queue_add_after(context: &Context, key: QueueKey, delay: Duration) -> BoxFuture<'_, Result<(), BoxError>> {
        Box::pin(async move {
            let queue = context.source_queue_state();
            if queue.shutting_down() { return Ok(()); }
            if delay.is_zero() { queue.add(key); return Ok(()); }
            let ready_at = context.source_queue_now().checked_add(delay)
                .ok_or_else(|| Errors::default().New("queue delay exceeds monotonic clock range"))?;
            let entry = WaitEntry { key, ready_at };
            tokio::select! {
                biased;
                _ = queue.stop.cancelled() => Ok(()),
                result = queue.delays.send(entry) => result.map_err(|_| Errors::default().New("queue delay task is unavailable")),
            }
        })
    }
    fn source_queue_get(context: &Context) -> BoxFuture<'_, Option<QueueKey>> { Box::pin(context.source_queue_state().get()) }
    fn source_queue_done(context: &Context, key: &QueueKey) { context.source_queue_state().done(key); }
    fn source_queue_shutdown(context: &Context) { context.source_queue_state().shutdown(); }
}

#[derive(HasField, Deftly)]
#[derive_deftly(UtilityQueueProviders)]
pub struct SourceQueueContext<Limiter: SourceRateLimiter> {
    pub source_queue_state: Arc<SourceQueueState>,
    pub source_queue_runtime: UtilityRuntimeContext,
    pub source_queue_name: Arc<str>,
    limiter: Arc<Limiter>,
}
impl<Limiter: SourceRateLimiter> Clone for SourceQueueContext<Limiter> {
    fn clone(&self) -> Self { Self { source_queue_state: Arc::clone(&self.source_queue_state), source_queue_runtime: self.source_queue_runtime.clone(), source_queue_name: Arc::clone(&self.source_queue_name), limiter: Arc::clone(&self.limiter) } }
}
impl<Limiter: SourceRateLimiter> HasSourceQueueLimiter for SourceQueueContext<Limiter> {
    type Limiter = Limiter;
    fn source_queue_limiter(&self) -> &Limiter { &self.limiter }
}
impl<Limiter: SourceRateLimiter> SourceQueueContext<Limiter> {
    pub fn new(runtime: &UtilityRuntimeContext, limiter: Limiter, queue_name: &str) -> Result<Self, BoxError> {
        let stop = runtime.utility_child_token()?;
        let (delays, receiver) = mpsc::channel(1000);
        let context = Self { source_queue_state: Arc::new(SourceQueueState { ready: Mutex::default(), readable: Notify::new(), delays, stop }),
            source_queue_runtime: runtime.clone(), source_queue_name: Arc::from(queue_name), limiter: Arc::new(limiter) };
        let actor = DelayLoop { context: context.clone(), receiver };
        runtime.spawn_utility_task(&format!("{queue_name} delayed workqueue"), Box::pin(actor.run()))?;
        Ok(context)
    }
    pub async fn add_rate_limited(&self, key: QueueKey) -> Result<(), BoxError> {
        let delay = self.source_queue_delay(&key)?;
        self.source_queue_add_after(key, delay).await
    }
}

struct WaitEntry { key: QueueKey, ready_at: Instant }
struct PendingDelay { handle: delay_queue::Key, ready_at: Instant }
// DelayQueue supplies the timer wheel. Long representable deadlines retain
// their exact policy deadline while waking at bounded native timer intervals.
const TIMER_HORIZON: Duration = Duration::from_secs(24 * 60 * 60);
fn timer_deadline(now: Instant, due: Instant) -> Instant { now.checked_add(TIMER_HORIZON).unwrap_or(due).min(due) }
struct DelayLoop<Limiter: SourceRateLimiter> { context: SourceQueueContext<Limiter>, receiver: mpsc::Receiver<WaitEntry> }
impl<Limiter: SourceRateLimiter> Drop for DelayLoop<Limiter> { fn drop(&mut self) { self.context.source_queue_state.shutdown(); } }
impl<Limiter: SourceRateLimiter> DelayLoop<Limiter> {
    async fn run(mut self) -> Result<(), BoxError> {
        let mut timers = DelayQueue::new();
        let mut waiting: HashMap<QueueKey, PendingDelay> = HashMap::new();
        loop {
            if self.context.source_queue_state.shutting_down() { return Ok(()); }
            tokio::select! {
                _ = self.context.source_queue_state.stop.cancelled() => return Ok(()),
                Some(expired) = timers.next(), if !timers.is_empty() => {
                    let key = expired.into_inner();
                    let pending = waiting.remove(&key).expect("expired resource key has one pending deadline");
                    let now = self.context.source_queue_now();
                    if pending.ready_at <= now { self.context.source_queue_add(key); }
                    else {
                        if timers.is_empty() { timers = DelayQueue::new(); }
                        let handle = timers.insert_at(key.clone(), timer_deadline(now, pending.ready_at));
                        waiting.insert(key, PendingDelay { handle, ready_at: pending.ready_at });
                    }
                }
                request = self.receiver.recv() => match request {
                    Some(entry) => self.accept(entry, &mut timers, &mut waiting),
                    None => return Ok(()),
                },
            }
        }
    }
    fn accept(&self, entry: WaitEntry, timers: &mut DelayQueue<QueueKey>, waiting: &mut HashMap<QueueKey, PendingDelay>) {
        let now = self.context.source_queue_now();
        if entry.ready_at <= now { self.context.source_queue_add(entry.key); return; }
        if let Some(pending) = waiting.get_mut(&entry.key) {
            if entry.ready_at < pending.ready_at {
                pending.ready_at = entry.ready_at;
                timers.reset_at(&pending.handle, timer_deadline(now, entry.ready_at));
            }
        } else {
            if timers.is_empty() { *timers = DelayQueue::new(); }
            let handle = timers.insert_at(entry.key.clone(), timer_deadline(now, entry.ready_at));
            waiting.insert(entry.key, PendingDelay { handle, ready_at: entry.ready_at });
        }
    }
}
