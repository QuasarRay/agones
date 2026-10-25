use std::{
    collections::HashMap, error::Error, fmt,
    sync::{Arc, Mutex, MutexGuard, atomic::{AtomicBool, AtomicUsize, Ordering as O}},
    time::{Duration, Instant},
};
use futures::future::BoxFuture;
use tokio::sync::{mpsc, watch, Mutex as AsyncMutex};

pub type BoxError = Box<dyn Error + Send + Sync>;
pub type Handler = Arc<dyn Fn(watch::Receiver<bool>, String) -> BoxFuture<'static, Result<(), BoxError>> + Send + Sync>;
pub type ConflictCheck = fn(&(dyn Error + Send + Sync)) -> bool;

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

#[derive(Debug)]
pub struct TraceError(pub Option<BoxError>);
impl fmt::Display for TraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.as_ref().map_or_else(|| f.write_str("<nil>"), |e| e.fmt(f))
    }
}
impl Error for TraceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}
pub fn is_trace_error(mut e: &(dyn Error + 'static)) -> bool {
    loop {
        if e.is::<TraceError>() { return true; }
        match e.source() { Some(x) => e = x, None => return false }
    }
}

pub trait RateLimiter: Send + Sync + 'static {
    fn delay(&self, failures: u32) -> Duration;
}
impl<F> RateLimiter for F where F: Fn(u32) -> Duration + Send + Sync + 'static {
    fn delay(&self, n: u32) -> Duration { self(n) }
}
pub fn fast_rate_limiter(max: Duration) -> impl RateLimiter {
    move |n| if n < 5 { Duration::from_millis(200) } else { max }
}
pub fn constant_rate_limiter(delay: Duration) -> impl RateLimiter {
    move |_| delay
}

pub struct DefaultRateLimiter(Mutex<(f64, Instant)>);
impl Default for DefaultRateLimiter {
    fn default() -> Self {
        Self(Mutex::new((100.0, Instant::now())))
    }
}
impl RateLimiter for DefaultRateLimiter {
    fn delay(&self, n: u32) -> Duration {
        let exp = Duration::from_millis((5u64 << n.min(18)).min(1_000_000));
        let mut b = lock(&self.0);
        let now = Instant::now();
        b.0 = (b.0 + now.duration_since(b.1).as_secs_f64() * 10.0).min(100.0);
        b.1 = now;
        let global = if b.0 >= 1.0 {
            b.0 -= 1.0;
            Duration::ZERO
        } else {
            let d = Duration::from_secs_f64((1.0 - b.0) / 10.0);
            b.0 -= 1.0;
            d
        };
        exp.max(global)
    }
}

#[derive(Clone, Copy)]
enum State { Queued, Processing(bool) }

struct Inner<R> {
    tx: mpsc::UnboundedSender<String>,
    rx: AsyncMutex<mpsc::UnboundedReceiver<String>>,
    states: Mutex<HashMap<String, State>>,
    failures: Mutex<HashMap<String, u32>>,
    limiter: R,
    handler: Handler,
    conflict: ConflictCheck,
    workers: AtomicUsize,
    running: AtomicUsize,
    closed: AtomicBool,
}
pub struct WorkerQueue<R: RateLimiter = DefaultRateLimiter>(Arc<Inner<R>>);
impl<R: RateLimiter> Clone for WorkerQueue<R> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl WorkerQueue<DefaultRateLimiter> {
    pub fn new(handler: Handler, conflict: ConflictCheck) -> Self {
        Self::with_rate_limiter(handler, conflict, DefaultRateLimiter::default())
    }
}
impl<R: RateLimiter> WorkerQueue<R> {
    pub fn with_rate_limiter(handler: Handler, conflict: ConflictCheck, limiter: R) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self(Arc::new(Inner {
            tx, rx: AsyncMutex::new(rx), states: Mutex::default(),
            failures: Mutex::default(), limiter, handler, conflict,
            workers: AtomicUsize::new(0), running: AtomicUsize::new(0),
            closed: AtomicBool::new(false),
        }))
    }

    fn add(&self, key: String) {
        if self.0.closed.load(O::Acquire) { return; }
        let mut s = lock(&self.0.states);
        match s.get_mut(&key) {
            Some(State::Queued) => {}
            Some(State::Processing(dirty)) => *dirty = true,
            None => {
                s.insert(key.clone(), State::Queued);
                let _ = self.0.tx.send(key);
            }
        }
    }

    fn retry_delay(&self, key: &str) -> Duration {
        let mut f = lock(&self.0.failures);
        let n = f.get(key).copied().unwrap_or(0);
        f.insert(key.into(), n.saturating_add(1));
        self.0.limiter.delay(n)
    }

    pub fn enqueue(&self, key: impl Into<String>) {
        let key = key.into();
        let delay = self.retry_delay(&key);
        self.enqueue_after(key, delay);
    }
    pub fn enqueue_immediately(&self, key: impl Into<String>) { self.add(key.into()); }
    pub fn enqueue_after(&self, key: impl Into<String>, delay: Duration) {
        let (q, key) = (self.clone(), key.into());
        tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            q.add(key);
        });
    }

    async fn next(&self) -> Option<String> {
        let key = self.0.rx.lock().await.recv().await?;
        lock(&self.0.states).insert(key.clone(), State::Processing(false));
        Some(key)
    }

    fn done(&self, key: &str, forget: bool) {
        if forget { lock(&self.0.failures).remove(key); }
        let mut s = lock(&self.0.states);
        if matches!(s.get(key), Some(State::Processing(true)))
            && !self.0.closed.load(O::Acquire)
        {
            s.insert(key.into(), State::Queued);
            let _ = self.0.tx.send(key.into());
        } else {
            s.remove(key);
        }
    }

    async fn process(&self, stop: watch::Receiver<bool>, key: String) {
        match (self.0.handler)(stop, key.clone()).await {
            Ok(()) => self.done(&key, true),
            Err(e) => {
                let quiet = (self.0.conflict)(e.as_ref()) || is_trace_error(e.as_ref());
                eprintln!("{} workerqueue {key}: {e}", if quiet { "trace" } else { "error" });
                let delay = self.retry_delay(&key);
                self.enqueue_after(key.clone(), delay);
                self.done(&key, false);
            }
        }
    }

    async fn worker(&self, mut stop: watch::Receiver<bool>) {
        self.0.running.fetch_add(1, O::AcqRel);
        struct Guard<'a>(&'a AtomicUsize);
        impl Drop for Guard<'_> {
            fn drop(&mut self) { self.0.fetch_sub(1, O::AcqRel); }
        }
        let _guard = Guard(&self.0.running);
        loop {
            if *stop.borrow() { break; }
            let key = tokio::select! {
                r = stop.changed() => if r.is_err() { break } else { continue },
                k = self.next() => match k { Some(k) => k, None => break },
            };
            self.process(stop.clone(), key).await;
        }
    }

    pub async fn run(&self, mut stop: watch::Receiver<bool>, workers: usize) {
        self.0.workers.store(workers, O::Release);
        self.0.closed.store(false, O::Release);
        for _ in 0..workers {
            let (q, s) = (self.clone(), stop.clone());
            tokio::spawn(async move { q.worker(s).await; });
        }
        while !*stop.borrow() && stop.changed().await.is_ok() {}
        self.0.closed.store(true, O::Release);
    }

    pub fn healthy(&self) -> Result<(), String> {
        let (want, got) = (
            self.0.workers.load(O::Acquire),
            self.0.running.load(O::Acquire),
        );
        (want == got).then_some(())
            .ok_or_else(|| format!("want {want} worker goroutine(s), got {got}"))
    }
    pub fn run_count(&self) -> usize { self.0.running.load(O::Acquire) }
}
