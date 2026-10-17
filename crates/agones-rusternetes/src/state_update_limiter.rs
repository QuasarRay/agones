use std::error::Error;
use std::fmt;
use std::time::{Duration, Instant};

use futures::lock::Mutex;

/// Matches the shared token bucket in client-go's default controller limiter.
pub const STATE_UPDATE_RATE_PER_SECOND: f64 = 10.0;
pub const STATE_UPDATE_BURST: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateUpdateLimiterError {
    NoCapacity,
}

impl fmt::Display for StateUpdateLimiterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCapacity => {
                f.write_str("state update limiter cannot grant a token")
            }
        }
    }
}

impl Error for StateUpdateLimiterError {}

struct Bucket {
    tokens: f64,
    last_refill: Instant,
}

/// Async token bucket used immediately before terminal GameServer state writes.
///
/// Informer/watch events themselves remain unthrottled and deduplicated by the
/// work queue. Only real terminal-state writes consume a token, matching the
/// upstream Agones newStateUpdateLimiter placement.
pub struct StateUpdateLimiter {
    rate_per_second: f64,
    burst: f64,
    bucket: Mutex<Bucket>,
}

impl Default for StateUpdateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl StateUpdateLimiter {
    pub fn new() -> Self {
        Self::with_limits(STATE_UPDATE_RATE_PER_SECOND, STATE_UPDATE_BURST)
    }

    pub fn with_limits(rate_per_second: f64, burst: usize) -> Self {
        Self {
            rate_per_second,
            burst: burst as f64,
            bucket: Mutex::new(Bucket {
                tokens: burst as f64,
                last_refill: Instant::now(),
            }),
        }
    }

    pub async fn wait(&self) -> Result<(), StateUpdateLimiterError> {
        if !self.rate_per_second.is_finite()
            || self.rate_per_second <= 0.0
            || self.burst < 1.0
        {
            return Err(StateUpdateLimiterError::NoCapacity);
        }

        loop {
            let delay = {
                let mut bucket = self.bucket.lock().await;
                let now = Instant::now();
                let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
                bucket.tokens = (bucket.tokens + elapsed * self.rate_per_second)
                    .min(self.burst);
                bucket.last_refill = now;

                if bucket.tokens >= 1.0 {
                    bucket.tokens -= 1.0;
                    return Ok(());
                }

                Duration::from_secs_f64(
                    (1.0 - bucket.tokens) / self.rate_per_second,
                )
            };

            tokio::time::sleep(delay).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_limits_match_upstream_agones() {
        let limiter = StateUpdateLimiter::new();
        assert_eq!(limiter.rate_per_second, 10.0);
        assert_eq!(limiter.burst, 100.0);
    }

    #[tokio::test]
    async fn zero_capacity_limiter_fails_immediately() {
        let limiter = StateUpdateLimiter::with_limits(0.0, 0);
        assert_eq!(
            limiter.wait().await,
            Err(StateUpdateLimiterError::NoCapacity)
        );
    }
}
