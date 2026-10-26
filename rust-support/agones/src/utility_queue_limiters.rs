//! Source client-go limiter state. All Go durations and counters are signed
//! 64-bit values; failure increments wrap rather than saturate.
#![allow(non_snake_case)]
use std::{sync::Mutex, time::Duration};
use tokio::time::Instant;
use crate::utility_queue::{CanReadSourceQueueClock, SourceDuration};
use crate::utility_queue_values::{ItemMap, QueueItem};

pub trait SourceRateLimiter: Send + Sync + 'static {
    fn When<Clock: CanReadSourceQueueClock>(&self, clock: &Clock, item: &QueueItem) -> SourceDuration;
    fn Forget(&self, item: &QueueItem);
    fn NumRequeues(&self, item: &QueueItem) -> i64;
}
pub struct ItemFastSlowRateLimiter { failures: Mutex<ItemMap<i64>>, fast: i64, slow: i64, max_fast: i64 }
impl ItemFastSlowRateLimiter {
    pub fn new(fast: SourceDuration, slow: SourceDuration, max_fast: i64) -> Self { Self { failures: Mutex::default(), fast, slow, max_fast } }
}
impl SourceRateLimiter for ItemFastSlowRateLimiter {
    fn When<Clock: CanReadSourceQueueClock>(&self, _clock: &Clock, item: &QueueItem) -> i64 {
        let mut failures = self.failures.lock().unwrap_or_else(|error| error.into_inner());
        let count = failures.get(item).copied().unwrap_or(0).wrapping_add(1); failures.insert(item.clone(), count);
        if count <= self.max_fast { self.fast } else { self.slow }
    }
    fn Forget(&self, item: &QueueItem) { self.failures.lock().unwrap_or_else(|error| error.into_inner()).remove(item); }
    fn NumRequeues(&self, item: &QueueItem) -> i64 { self.failures.lock().unwrap_or_else(|error| error.into_inner()).get(item).copied().unwrap_or(0) }
}
pub struct ItemExponentialRateLimiter { failures: Mutex<ItemMap<i64>>, base: i64, max: i64 }
impl ItemExponentialRateLimiter { pub fn new(base: i64, max: i64) -> Self { Self { failures: Mutex::default(), base, max } } }
fn go_float_to_i64(value: f64) -> i64 {
    // amd64 Go conversion profile. The default controller's positive 5ms
    // powers never hit the exact 2^63 boundary before the overflow guard.
    // Other source target/conversion profiles remain a recorded boundary.
    if !value.is_finite() || value >= 9_223_372_036_854_775_808.0 || value < -9_223_372_036_854_775_808.0 { i64::MIN }
    else { value.trunc() as i64 }
}
impl SourceRateLimiter for ItemExponentialRateLimiter {
    fn When<Clock: CanReadSourceQueueClock>(&self, _clock: &Clock, item: &QueueItem) -> i64 {
        let mut failures = self.failures.lock().unwrap_or_else(|error| error.into_inner());
        let exponent = failures.get(item).copied().unwrap_or(0); failures.insert(item.clone(), exponent.wrapping_add(1));
        let backoff = self.base as f64 * 2.0f64.powf(exponent as f64);
        if backoff > i64::MAX as f64 { return self.max; }
        go_float_to_i64(backoff).min(self.max)
    }
    fn Forget(&self, item: &QueueItem) { self.failures.lock().unwrap_or_else(|error| error.into_inner()).remove(item); }
    fn NumRequeues(&self, item: &QueueItem) -> i64 { self.failures.lock().unwrap_or_else(|error| error.into_inner()).get(item).copied().unwrap_or(0) }
}
struct Bucket { tokens: f64, last: Option<Instant> }
pub struct BucketRateLimiter { state: Mutex<Bucket>, qps: f64, burst: i64 }
impl BucketRateLimiter { pub fn new(qps: f64, burst: i64) -> Self { Self { state: Mutex::new(Bucket { tokens: burst as f64, last: None }), qps, burst } } }
impl SourceRateLimiter for BucketRateLimiter {
    fn When<Clock: CanReadSourceQueueClock>(&self, clock: &Clock, _item: &QueueItem) -> i64 {
        let now = clock.source_queue_now(); // Reserve samples before taking mu.
        let ready_at = {
            let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
            if self.qps == f64::MAX { Some(now) } // rate.Inf bypasses burst/token updates.
            else {
                let elapsed = state.last.map_or(i64::MAX, |last| crate::utility_queue::signed_nanos(now, last).max(0));
                let delta = if self.qps <= 0.0 { 0.0 } else { Duration::from_nanos(elapsed as u64).as_secs_f64() * self.qps };
                let tokens = (state.tokens + delta).min(self.burst as f64) - 1.0;
                let wait = if tokens < 0.0 {
                    if self.qps <= 0.0 { i64::MAX } else {
                        let duration = (-tokens / self.qps) * 1_000_000_000.0;
                        if duration > i64::MAX as f64 { i64::MAX } else { go_float_to_i64(duration) }
                    }
                } else { 0 };
                if self.burst < 1 { None }
                else {
                    state.tokens = tokens; state.last = Some(now);
                    Some(crate::utility_queue::add_signed_duration(now, wait))
                }
            }
        };
        // Reservation.Delay makes its own second time.Now call after unlock.
        let after = clock.source_queue_now();
        ready_at.map_or(i64::MAX, |ready_at| crate::utility_queue::signed_nanos(ready_at, after).max(0))
    }
    fn Forget(&self, _item: &QueueItem) {}
    fn NumRequeues(&self, _item: &QueueItem) -> i64 { 0 }
}
pub struct MaxOfRateLimiter<A, B>(pub A, pub B);
impl<A: SourceRateLimiter, B: SourceRateLimiter> SourceRateLimiter for MaxOfRateLimiter<A, B> {
    fn When<Clock: CanReadSourceQueueClock>(&self, clock: &Clock, item: &QueueItem) -> i64 {
        let first = self.0.When(clock, item); let second = self.1.When(clock, item); 0.max(first).max(second)
    }
    fn Forget(&self, item: &QueueItem) { self.0.Forget(item); self.1.Forget(item); }
    fn NumRequeues(&self, item: &QueueItem) -> i64 { 0.max(self.0.NumRequeues(item)).max(self.1.NumRequeues(item)) }
}
pub type DefaultControllerRateLimiter = MaxOfRateLimiter<ItemExponentialRateLimiter, BucketRateLimiter>;
pub fn default_controller_limiter() -> DefaultControllerRateLimiter { MaxOfRateLimiter(ItemExponentialRateLimiter::new(5_000_000, 1_000_000_000_000), BucketRateLimiter::new(10.0, 100)) }
