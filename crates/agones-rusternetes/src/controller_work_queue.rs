use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use futures::StreamExt;
use futures::channel::mpsc::{
    UnboundedReceiver, UnboundedSender, unbounded,
};
use futures::lock::Mutex as AsyncMutex;

/// Stable key contract shared by controller work queues.
pub(crate) trait ControllerWorkItem: Clone {
    fn work_key(&self) -> String;
}

/// Retry policy abstraction for controller queues.
///
/// Keeping this independent of the queue mechanics lets the main GameServer
/// controller use Agones' custom fast/slow limiter while auxiliary controllers
/// reuse client-go's default exponential limiter behavior.
pub(crate) trait ControllerRetryPolicy: Clone {
    fn delay(&self, retries_before_this_retry: u32) -> Duration;
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct FastSlowRetryPolicy {
    pub fast_retries: u32,
    pub fast_delay: Duration,
    pub slow_delay: Duration,
}

impl ControllerRetryPolicy for FastSlowRetryPolicy {
    fn delay(&self, retries_before_this_retry: u32) -> Duration {
        if retries_before_this_retry < self.fast_retries {
            self.fast_delay
        } else {
            self.slow_delay
        }
    }
}

pub(crate) const DEFAULT_CONTROLLER_RETRY_BASE_DELAY_MS: u64 = 5;
pub(crate) const DEFAULT_CONTROLLER_RETRY_MAX_DELAY_MS: u64 = 1_000_000;
pub(crate) const DEFAULT_CONTROLLER_RETRY_BUCKET_QPS: f64 = 10.0;
pub(crate) const DEFAULT_CONTROLLER_RETRY_BUCKET_BURST: f64 = 100.0;

/// Per-item portion of client-go DefaultTypedControllerRateLimiter:
/// min(5ms * 2^failures, 1000s).
pub(crate) const fn default_controller_exponential_delay_ms(
    retries_before_this_retry: u32,
) -> u64 {
    if retries_before_this_retry >= 28 {
        return DEFAULT_CONTROLLER_RETRY_MAX_DELAY_MS;
    }

    let scaled =
        DEFAULT_CONTROLLER_RETRY_BASE_DELAY_MS << retries_before_this_retry;
    if scaled > DEFAULT_CONTROLLER_RETRY_MAX_DELAY_MS {
        DEFAULT_CONTROLLER_RETRY_MAX_DELAY_MS
    } else {
        scaled
    }
}

#[derive(Debug)]
struct RetryTokenBucket {
    tokens: f64,
    last: Instant,
}

impl RetryTokenBucket {
    fn new() -> Self {
        Self {
            tokens: DEFAULT_CONTROLLER_RETRY_BUCKET_BURST,
            last: Instant::now(),
        }
    }

    fn reserve_delay(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last).as_secs_f64();
        self.last = now;
        self.tokens = (
            self.tokens + elapsed * DEFAULT_CONTROLLER_RETRY_BUCKET_QPS
        )
            .min(DEFAULT_CONTROLLER_RETRY_BUCKET_BURST);

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            return Duration::ZERO;
        }

        let deficit = 1.0 - self.tokens;
        self.tokens -= 1.0;
        Duration::from_secs_f64(
            deficit / DEFAULT_CONTROLLER_RETRY_BUCKET_QPS,
        )
    }
}

/// Approximation of client-go DefaultTypedControllerRateLimiter using the same
/// per-item exponential schedule plus its 10qps / burst-100 global retry
/// bucket. Normal controller operation stays in the exact zero-bucket-delay
/// region; retry floods are additionally globally throttled.
#[derive(Clone, Debug)]
pub(crate) struct DefaultControllerRetryPolicy {
    bucket: Arc<Mutex<RetryTokenBucket>>,
}

impl Default for DefaultControllerRetryPolicy {
    fn default() -> Self {
        Self {
            bucket: Arc::new(Mutex::new(RetryTokenBucket::new())),
        }
    }
}

impl ControllerRetryPolicy for DefaultControllerRetryPolicy {
    fn delay(&self, retries_before_this_retry: u32) -> Duration {
        let exponential = Duration::from_millis(
            default_controller_exponential_delay_ms(
                retries_before_this_retry,
            ),
        );
        let bucket = self
            .bucket
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .reserve_delay();
        exponential.max(bucket)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QueueEntryState {
    Waiting,
    Processing { dirty: bool },
}

/// Minimal client-go-style deduplicating work queue used by Agones controller
/// runtimes.
///
/// Re-enqueueing a waiting item is coalesced. Re-enqueueing an in-flight item
/// marks it dirty, causing exactly one additional processing pass after the
/// current attempt succeeds. Failures preserve retry state until success.
#[derive(Clone)]
pub(crate) struct ControllerWorkQueue<T, P>
where
    T: ControllerWorkItem,
    P: ControllerRetryPolicy,
{
    sender: UnboundedSender<T>,
    receiver: Arc<AsyncMutex<UnboundedReceiver<T>>>,
    states: Arc<Mutex<HashMap<String, QueueEntryState>>>,
    retries: Arc<Mutex<HashMap<String, u32>>>,
    retry_policy: P,
}

impl<T, P> ControllerWorkQueue<T, P>
where
    T: ControllerWorkItem,
    P: ControllerRetryPolicy,
{
    pub(crate) fn new(retry_policy: P) -> Self {
        let (sender, receiver) = unbounded();
        Self {
            sender,
            receiver: Arc::new(AsyncMutex::new(receiver)),
            states: Arc::new(Mutex::new(HashMap::new())),
            retries: Arc::new(Mutex::new(HashMap::new())),
            retry_policy,
        }
    }

    pub(crate) fn enqueue(&self, item: T) {
        let key = item.work_key();
        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        match states.get_mut(&key) {
            Some(QueueEntryState::Waiting) => {}
            Some(QueueEntryState::Processing { dirty }) => {
                *dirty = true;
            }
            None => {
                states.insert(key, QueueEntryState::Waiting);
                let _ = self.sender.unbounded_send(item);
            }
        }
    }

    pub(crate) async fn next(&self) -> Option<T> {
        let item = {
            let mut receiver = self.receiver.lock().await;
            receiver.next().await?
        };

        let key = item.work_key();
        self.states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(
                key,
                QueueEntryState::Processing { dirty: false },
            );
        Some(item)
    }

    pub(crate) fn complete_success(&self, item: &T) {
        let key = item.work_key();
        self.retries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(&key);

        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let dirty = matches!(
            states.get(&key),
            Some(QueueEntryState::Processing { dirty: true })
        );

        if dirty {
            states.insert(key, QueueEntryState::Waiting);
            let _ = self.sender.unbounded_send(item.clone());
        } else {
            states.remove(&key);
        }
    }

    pub(crate) fn record_failure(&self, item: &T) -> Duration {
        let key = item.work_key();
        let mut retries = self
            .retries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let retries_before_this_retry =
            retries.get(&key).copied().unwrap_or(0);
        retries.insert(key, retries_before_this_retry.saturating_add(1));
        self.retry_policy.delay(retries_before_this_retry)
    }

    pub(crate) fn complete_retry(&self, item: &T) {
        let key = item.work_key();
        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        if matches!(
            states.get(&key),
            Some(QueueEntryState::Processing { .. })
        ) {
            states.insert(key, QueueEntryState::Waiting);
            let _ = self.sender.unbounded_send(item.clone());
        }
    }

    #[cfg(test)]
    pub(crate) fn retry_count(&self, item: &T) -> u32 {
        self.retries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(&item.work_key())
            .copied()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_exponential_retry_schedule_matches_client_go_thresholds() {
        assert_eq!(default_controller_exponential_delay_ms(0), 5);
        assert_eq!(default_controller_exponential_delay_ms(1), 10);
        assert_eq!(default_controller_exponential_delay_ms(10), 5_120);
        assert_eq!(
            default_controller_exponential_delay_ms(18),
            DEFAULT_CONTROLLER_RETRY_MAX_DELAY_MS
        );
        assert_eq!(
            default_controller_exponential_delay_ms(u32::MAX),
            DEFAULT_CONTROLLER_RETRY_MAX_DELAY_MS
        );
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestWorkItem(&'static str);

    impl ControllerWorkItem for TestWorkItem {
        fn work_key(&self) -> String {
            self.0.to_owned()
        }
    }

    #[tokio::test]
    async fn generic_queue_deduplicates_and_requeues_dirty_work_once() {
        let queue = ControllerWorkQueue::new(FastSlowRetryPolicy {
            fast_retries: 1,
            fast_delay: Duration::from_millis(1),
            slow_delay: Duration::from_millis(2),
        });
        let item = TestWorkItem("default/server");

        queue.enqueue(item.clone());
        queue.enqueue(item.clone());
        assert_eq!(queue.next().await, Some(item.clone()));

        queue.enqueue(item.clone());
        queue.enqueue(item.clone());
        queue.complete_success(&item);
        assert_eq!(queue.next().await, Some(item.clone()));
        queue.complete_success(&item);

        assert!(
            tokio::time::timeout(Duration::from_millis(10), queue.next())
                .await
                .is_err()
        );
    }
}
