//! Agones retry policy over native Rust durations, maps and monotonic clocks.
//! Per-key retry selection and aggregate burst reservations are application
//! policy. Tokio supplies time; this module does not model a Go rate limiter.
#![allow(non_snake_case)]
use std::{collections::HashMap, num::NonZeroU32, sync::Mutex, time::Duration};
use tokio::time::Instant;
use crate::{errors::{BoxError, Errors}, utility_queue::CanReadSourceQueueClock, utility_queue_values::QueueKey};

pub trait SourceRateLimiter: Send + Sync + 'static {
    fn When<Clock: CanReadSourceQueueClock>(&self, clock: &Clock, key: &QueueKey) -> Result<Duration, BoxError>;
    fn Forget(&self, key: &QueueKey);
    fn NumRequeues(&self, key: &QueueKey) -> usize;
}
pub struct ItemFastSlowRateLimiter {
    failures: Mutex<HashMap<QueueKey, usize>>, fast: Duration, slow: Duration, max_fast: usize,
}
impl ItemFastSlowRateLimiter {
    pub fn new(fast: Duration, slow: Duration, max_fast: usize) -> Self { Self { failures: Mutex::default(), fast, slow, max_fast } }
}
impl SourceRateLimiter for ItemFastSlowRateLimiter {
    fn When<Clock: CanReadSourceQueueClock>(&self, _clock: &Clock, key: &QueueKey) -> Result<Duration, BoxError> {
        let mut failures = self.failures.lock().unwrap_or_else(|error| error.into_inner());
        let count = failures.entry(key.clone()).or_default();
        *count = count.saturating_add(1);
        Ok(if *count <= self.max_fast { self.fast } else { self.slow })
    }
    fn Forget(&self, key: &QueueKey) { self.failures.lock().unwrap_or_else(|error| error.into_inner()).remove(key); }
    fn NumRequeues(&self, key: &QueueKey) -> usize { self.failures.lock().unwrap_or_else(|error| error.into_inner()).get(key).copied().unwrap_or_default() }
}
struct RetryState { attempts: usize, next: Duration }
pub struct ItemExponentialRateLimiter { failures: Mutex<HashMap<QueueKey, RetryState>>, base: Duration, max: Duration }
impl ItemExponentialRateLimiter {
    pub fn new(base: Duration, max: Duration) -> Self { Self { failures: Mutex::default(), base: base.min(max), max } }
}
impl SourceRateLimiter for ItemExponentialRateLimiter {
    fn When<Clock: CanReadSourceQueueClock>(&self, _clock: &Clock, key: &QueueKey) -> Result<Duration, BoxError> {
        let mut failures = self.failures.lock().unwrap_or_else(|error| error.into_inner());
        let retry = failures.entry(key.clone()).or_insert(RetryState { attempts: 0, next: self.base });
        let delay = retry.next;
        retry.attempts = retry.attempts.saturating_add(1);
        retry.next = delay.checked_mul(2).unwrap_or(self.max).min(self.max);
        Ok(delay)
    }
    fn Forget(&self, key: &QueueKey) { self.failures.lock().unwrap_or_else(|error| error.into_inner()).remove(key); }
    fn NumRequeues(&self, key: &QueueKey) -> usize { self.failures.lock().unwrap_or_else(|error| error.into_inner()).get(key).map_or(0, |retry| retry.attempts) }
}

/// Reserve aggregate work at a fixed refill interval, allowing the configured
/// initial burst. The virtual next slot is the sole mutable reservation state;
/// callers combine this delay with per-key retry policy before enqueueing.
pub struct BucketRateLimiter { next: Mutex<Option<Instant>>, interval: Duration, burst_allowance: Duration }
impl BucketRateLimiter {
    pub fn new(interval: Duration, burst: NonZeroU32) -> Result<Self, BoxError> {
        if interval.is_zero() { return Err(Errors::default().New("queue refill interval must be positive")); }
        let burst_allowance = interval.checked_mul(burst.get() - 1)
            .ok_or_else(|| Errors::default().New("queue burst window exceeds Duration range"))?;
        Ok(Self { next: Mutex::new(None), interval, burst_allowance })
    }
}
impl SourceRateLimiter for BucketRateLimiter {
    fn When<Clock: CanReadSourceQueueClock>(&self, clock: &Clock, _key: &QueueKey) -> Result<Duration, BoxError> {
        let now = clock.source_queue_now();
        let mut next = self.next.lock().unwrap_or_else(|error| error.into_inner());
        let slot = (*next).map_or(now, |slot| slot.max(now));
        let following = slot.checked_add(self.interval).ok_or_else(|| Errors::default().New("queue reservation exceeds monotonic clock range"))?;
        *next = Some(following);
        Ok(slot.saturating_duration_since(now).saturating_sub(self.burst_allowance))
    }
    fn Forget(&self, _key: &QueueKey) {}
    fn NumRequeues(&self, _key: &QueueKey) -> usize { 0 }
}
pub struct MaxOfRateLimiter<A, B>(pub A, pub B);
impl<A: SourceRateLimiter, B: SourceRateLimiter> SourceRateLimiter for MaxOfRateLimiter<A, B> {
    fn When<Clock: CanReadSourceQueueClock>(&self, clock: &Clock, key: &QueueKey) -> Result<Duration, BoxError> {
        let first = self.0.When(clock, key)?;
        let second = self.1.When(clock, key)?;
        Ok(first.max(second))
    }
    fn Forget(&self, key: &QueueKey) { self.0.Forget(key); self.1.Forget(key); }
    fn NumRequeues(&self, key: &QueueKey) -> usize { self.0.NumRequeues(key).max(self.1.NumRequeues(key)) }
}
pub type DefaultControllerRateLimiter = MaxOfRateLimiter<ItemExponentialRateLimiter, BucketRateLimiter>;
pub fn default_controller_limiter() -> DefaultControllerRateLimiter {
    let bucket = BucketRateLimiter::new(Duration::from_millis(100), NonZeroU32::new(100).expect("positive default burst"))
        .expect("valid default queue refill policy");
    MaxOfRateLimiter(ItemExponentialRateLimiter::new(Duration::from_millis(5), Duration::from_secs(1000)), bucket)
}
