//! Source queue mechanics and concrete owned delay/clock effects.
use std::{collections::VecDeque, sync::{Arc, Mutex}, time::Duration};
use cgp::prelude::*;
use derive_deftly::Deftly;
use futures::future::BoxFuture;
use tokio::{sync::{mpsc, Notify}, time::Instant};
use tokio_util::sync::CancellationToken;
use crate::{errors::BoxError, utility_runtime::UtilityRuntimeContext, utility_scope::CanSpawnUtilityTask,
    utility_queue_limiters::SourceRateLimiter, utility_queue_values::{ItemMap, QueueItem, QueueKey, QueueObject, meta_namespace_key},
    utility_declarations::derive_deftly_template_UtilityQueueProviders};

pub type SourceDuration = i64;
pub fn signed_nanos(after: Instant, before: Instant) -> i64 {
    if after >= before { after.duration_since(before).as_nanos().min(i64::MAX as u128) as i64 }
    else { let nanos = before.duration_since(after).as_nanos(); if nanos > i64::MAX as u128 { i64::MIN } else { -(nanos as i64) } }
}
pub fn add_signed_duration(time: Instant, duration: i64) -> Instant {
    if duration >= 0 { time + Duration::from_nanos(duration as u64) }
    else { time - Duration::from_nanos(duration.unsigned_abs()) }
}

#[cgp_component(SourceQueueClock)]
pub trait CanReadSourceQueueClock: Sync {
    fn source_queue_now(&self) -> Instant;
    fn source_queue_sleep(&self, duration: SourceDuration) -> BoxFuture<'_, ()>;
    fn source_queue_sleep_until(&self, deadline: Instant) -> BoxFuture<'_, ()>;
}
#[cgp_new_provider]
impl<Context: Sync> SourceQueueClock<Context> for ReadTokioSourceQueueClock {
    fn source_queue_now(_context: &Context) -> Instant { Instant::now() }
    fn source_queue_sleep(_context: &Context, duration: i64) -> BoxFuture<'_, ()> {
        Box::pin(async move { if duration > 0 { tokio::time::sleep(Duration::from_nanos(duration as u64)).await; } })
    }
    fn source_queue_sleep_until(_context: &Context, deadline: Instant) -> BoxFuture<'_, ()> { Box::pin(tokio::time::sleep_until(deadline)) }
}

struct ReadyState { queue: VecDeque<QueueItem>, dirty: ItemMap<()>, processing: ItemMap<()>, shutting_down: bool }
impl Default for ReadyState {
    fn default() -> Self { Self { queue: VecDeque::new(), dirty: ItemMap::default(), processing: ItemMap::default(), shutting_down: false } }
}
pub struct SourceQueueState { ready: Mutex<ReadyState>, readable: Notify, delays: mpsc::Sender<WaitEntry>, stop: CancellationToken }
impl SourceQueueState {
    pub fn add(&self, item: QueueItem) {
        let mut state = self.ready.lock().unwrap_or_else(|error| error.into_inner());
        if state.shutting_down { return; }
        if state.dirty.contains(&item) { return; } // source FIFO Touch is empty
        state.dirty.insert(item.clone(), ());
        if state.processing.contains(&item) { return; }
        state.queue.push_back(item); self.readable.notify_one();
    }
    pub fn done(&self, item: &QueueItem) {
        let mut state = self.ready.lock().unwrap_or_else(|error| error.into_inner());
        state.processing.remove(item);
        // A pre-shutdown dirty item is requeued even after ShutDown. Add's
        // shutdown rejection does not apply to the separate Done operation.
        if state.dirty.contains(item) { state.queue.push_back(item.clone()); self.readable.notify_one(); }
        else if state.processing.len() == 0 { self.readable.notify_one(); }
    }
    pub async fn get(&self) -> Option<QueueItem> {
        loop {
            let readable = self.readable.notified(); tokio::pin!(readable); readable.as_mut().enable();
            {
                let mut state = self.ready.lock().unwrap_or_else(|error| error.into_inner());
                if let Some(item) = state.queue.pop_front() {
                    state.processing.insert(item.clone(), ()); state.dirty.remove(&item); return Some(item);
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
    fn source_queue_delay(&self, item: &QueueItem) -> SourceDuration;
    fn source_queue_forget(&self, item: &QueueItem);
    fn source_queue_num_requeues(&self, item: &QueueItem) -> i64;
}
#[cgp_new_provider]
impl<Context: HasSourceQueueLimiter + CanReadSourceQueueClock> SourceQueueDelayDecider<Context> for UseOwnedSourceQueueLimiter {
    fn source_queue_delay(context: &Context, item: &QueueItem) -> i64 { context.source_queue_limiter().When(context, item) }
    fn source_queue_forget(context: &Context, item: &QueueItem) { context.source_queue_limiter().Forget(item); }
    fn source_queue_num_requeues(context: &Context, item: &QueueItem) -> i64 { context.source_queue_limiter().NumRequeues(item) }
}
#[cgp_component(SourceQueueKeyReader)]
pub trait CanReadSourceQueueKey { fn source_queue_key(&self, object: &dyn QueueObject) -> Result<QueueKey, BoxError>; }
#[cgp_new_provider]
impl<Context> SourceQueueKeyReader<Context> for ReadSourceMetadataKey {
    fn source_queue_key(_context: &Context, object: &dyn QueueObject) -> Result<QueueKey, BoxError> { meta_namespace_key(object) }
}
#[cgp_component(SourceQueueOperations)]
pub trait CanUseSourceQueue: Sync {
    fn source_queue_add(&self, item: QueueItem);
    fn source_queue_add_after(&self, item: QueueItem, delay: i64) -> BoxFuture<'_, ()>;
    fn source_queue_get(&self) -> BoxFuture<'_, Option<QueueItem>>;
    fn source_queue_done(&self, item: &QueueItem);
    fn source_queue_shutdown(&self);
}
#[cgp_new_provider]
impl<Context: HasSourceQueueState + CanReadSourceQueueClock + Sync> SourceQueueOperations<Context> for UseOwnedSourceQueue {
    fn source_queue_add(context: &Context, item: QueueItem) { context.source_queue_state().add(item); }
    fn source_queue_add_after(context: &Context, item: QueueItem, delay: i64) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            let queue = context.source_queue_state();
            if queue.shutting_down() { return; }
            if delay <= 0 { queue.add(item); return; }
            // The original channel has capacity 1000. A full channel blocks
            // producers, and shutdown unblocks them without a detached timer.
            let entry = WaitEntry { data: item, ready_at: add_signed_duration(context.source_queue_now(), delay), index: usize::MAX };
            tokio::select! { _ = queue.stop.cancelled() => (), _ = queue.delays.send(entry) => () }
        })
    }
    fn source_queue_get(context: &Context) -> BoxFuture<'_, Option<QueueItem>> { Box::pin(context.source_queue_state().get()) }
    fn source_queue_done(context: &Context, item: &QueueItem) { context.source_queue_state().done(item); }
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
        let actor = DelayLoop { heartbeat_first: context.source_queue_now() + Duration::from_secs(10), context: context.clone(), receiver };
        runtime.spawn_utility_task(&format!("{queue_name} delayed workqueue"), Box::pin(actor.run()))?;
        Ok(context)
    }
    pub async fn add_rate_limited(&self, item: QueueItem) { let delay = self.source_queue_delay(&item); self.source_queue_add_after(item, delay).await; }
}

struct WaitEntry { data: QueueItem, ready_at: Instant, index: usize }
#[derive(Default)]
struct WaitHeap {
    // Stable arena identities reproduce the source map's *waitFor pointers.
    // Heap swaps update the pointed-to index without reinserting map keys.
    nodes: Vec<Option<WaitEntry>>, free: Vec<usize>, heap: Vec<usize>, known: ItemMap<usize>,
}
impl WaitHeap {
    fn node(&self, id: usize) -> &WaitEntry { self.nodes[id].as_ref().expect("live wait entry") }
    fn less(&self, left: usize, right: usize) -> bool { self.node(self.heap[left]).ready_at < self.node(self.heap[right]).ready_at }
    fn swap(&mut self, left: usize, right: usize) {
        self.heap.swap(left, right);
        for index in [left, right] { self.nodes[self.heap[index]].as_mut().expect("live wait entry").index = index; }
    }
    fn up(&mut self, mut child: usize) {
        while child > 0 { let parent = (child - 1) / 2; if !self.less(child, parent) { break; } self.swap(child, parent); child = parent; }
    }
    fn down(&mut self, mut parent: usize, length: usize) {
        loop {
            let left = parent * 2 + 1; if left >= length { break; }
            let right = left + 1; let child = if right < length && self.less(right, left) { right } else { left };
            if !self.less(child, parent) { break; } self.swap(parent, child); parent = child;
        }
    }
    fn insert(&mut self, mut entry: WaitEntry) {
        if let Some(id) = self.known.get(&entry.data).copied() {
            let previous = self.nodes[id].as_mut().expect("known wait entry");
            if previous.ready_at > entry.ready_at { previous.ready_at = entry.ready_at; let index = previous.index; self.up(index); }
            return;
        }
        entry.index = self.heap.len();
        let id = match self.free.pop() { Some(id) => { self.nodes[id] = Some(entry); id }, None => { self.nodes.push(Some(entry)); self.nodes.len()-1 } };
        self.known.insert(self.node(id).data.clone(), id); self.heap.push(id); self.up(self.heap.len()-1);
    }
    fn first(&self) -> Option<&WaitEntry> { self.heap.first().map(|id| self.node(*id)) }
    fn pop_id(&mut self) -> usize {
        let length = self.heap.len()-1; self.swap(0, length); self.down(0, length);
        let id = self.heap.pop().expect("nonempty wait heap"); self.nodes[id].as_mut().expect("live wait entry").index = usize::MAX; id
    }
    fn finish_pop(&mut self, id: usize) {
        if self.known.remove(&self.node(id).data.clone()).is_some() { self.nodes[id] = None; self.free.push(id); }
        // A non-reflexive key cannot be deleted from the source map. Its
        // pointed-to node stays alive here too, rather than abusing Rust Eq.
    }
}
struct DelayLoop<Limiter: SourceRateLimiter> { context: SourceQueueContext<Limiter>, receiver: mpsc::Receiver<WaitEntry>, heartbeat_first: Instant }
impl<Limiter: SourceRateLimiter> Drop for DelayLoop<Limiter> { fn drop(&mut self) { self.context.source_queue_state.shutdown(); } }
impl<Limiter: SourceRateLimiter> DelayLoop<Limiter> {
    async fn run(mut self) -> Result<(), BoxError> {
        let mut waiting = WaitHeap::default();
        let mut heartbeat = self.heartbeat_first;
        loop {
            if self.context.source_queue_state.shutting_down() { return Ok(()); }
            let now = self.context.source_queue_now();
            while waiting.first().is_some_and(|entry| entry.ready_at <= now) {
                let id = waiting.pop_id(); self.context.source_queue_add(waiting.node(id).data.clone()); waiting.finish_pop(id);
            }
            let deadline = waiting.first().map(|entry| entry.ready_at);
            tokio::select! {
                _ = self.context.source_queue_state.stop.cancelled() => return Ok(()),
                _ = self.context.source_queue_sleep_until(heartbeat) => {
                    // RealClock's ticker retains a fixed cadence and drops
                    // missed ticks. Keep the next boundary after the last tick.
                    let now = self.context.source_queue_now();
                    let missed = now.duration_since(heartbeat).as_nanos() / 10_000_000_000u128;
                    heartbeat += Duration::from_secs(((missed + 1).min(u64::MAX as u128 / 10) as u64) * 10);
                }
                _ = async { match deadline { Some(deadline) => self.context.source_queue_sleep_until(deadline).await, None => futures::future::pending::<()>().await } } => (),
                Some(entry) = self.receiver.recv() => {
                    self.accept(entry, &mut waiting);
                    while let Ok(entry) = self.receiver.try_recv() { self.accept(entry, &mut waiting); }
                }
            }
        }
    }
    fn accept(&self, entry: WaitEntry, waiting: &mut WaitHeap) {
        if entry.ready_at > self.context.source_queue_now() { waiting.insert(entry); } else { self.context.source_queue_add(entry.data); }
    }
}
