use std::time::Duration;
use prometheus::{
    Gauge, GaugeVec, Histogram, HistogramOpts, HistogramVec, IntCounter, IntCounterVec, Opts,
    Registry,
};

const SEC: [f64; 12] = [0., .01, .025, .05, .075, .1, .25, .5, .75, 1., 2., 3.];
const NUM: [f64; 7] = [0., 10., 50., 100., 150., 250., 300.];

fn cv(n: &str, h: &str, l: &[&str]) -> prometheus::Result<IntCounterVec> {
    IntCounterVec::new(Opts::new(format!("agones_{n}"), h), l)
}
fn gv(n: &str, h: &str, l: &[&str]) -> prometheus::Result<GaugeVec> {
    GaugeVec::new(Opts::new(format!("agones_{n}"), h), l)
}
fn hv(n: &str, h: &str, b: &[f64], l: &[&str]) -> prometheus::Result<HistogramVec> {
    HistogramVec::new(HistogramOpts::new(format!("agones_{n}"), h).buckets(b.to_vec()), l)
}
macro_rules! reg { ($r:expr;$($m:expr),+$(,)?) => { $($r.register(Box::new($m.clone()))?;)+ }; }

#[derive(Clone)]
pub struct ClientMetrics {
    http: (IntCounterVec, HistogramVec),
    cache: (IntCounterVec, HistogramVec, HistogramVec, IntCounterVec, IntCounterVec, HistogramVec, HistogramVec, GaugeVec),
    queue: (GaugeVec, IntCounterVec, HistogramVec, HistogramVec, IntCounterVec, GaugeVec, GaugeVec),
}

impl ClientMetrics {
    pub fn register(r: &Registry) -> prometheus::Result<Self> {
        let http = (
            cv("k8s_client_http_request_total", "The total of HTTP requests to the Kubernetes API by status code", &["verb", "status_code"])?,
            hv("k8s_client_http_request_duration_seconds", "The distribution of HTTP requests latencies to the Kubernetes API by status code", &SEC, &["verb", "endpoint"])?,
        );
        let cache = (
            cv("k8s_client_cache_list_total", "The total number of list operations for client-go caches", &[])?,
            hv("k8s_client_cache_list_duration_seconds", "Duration of a Kubernetes list API call in seconds", &SEC, &[])?,
            hv("k8s_client_cache_list_items", "Count of items in a list from the Kubernetes API.", &NUM, &[])?,
            cv("k8s_client_cache_watches_total", "The total number of watch operations for client-go caches", &[])?,
            cv("k8s_client_cache_short_watches_total", "The total number of short watch operations for client-go caches", &[])?,
            hv("k8s_client_cache_watch_duration_seconds", "Duration of watches on the Kubernetes API.", &SEC, &[])?,
            hv("k8s_client_cache_watch_events", "Number of items in watches on the Kubernetes API.", &NUM, &[])?,
            gv("k8s_client_cache_last_resource_version", "Last resource version from the Kubernetes API.", &[])?,
        );
        let queue = (
            gv("k8s_client_workqueue_depth", "Current depth of the work queue.", &["queue_name"])?,
            cv("k8s_client_workqueue_items_total", "Total number of items added to the work queue.", &["queue_name"])?,
            hv("k8s_client_workqueue_latency_seconds", "How long an item stays in the work queue.", &SEC, &["queue_name"])?,
            hv("k8s_client_workqueue_work_duration_seconds", "How long processing an item from the work queue takes.", &SEC, &["queue_name"])?,
            cv("k8s_client_workqueue_retries_total", "Total number of items retried to the work queue.", &["queue_name"])?,
            gv("k8s_client_workqueue_longest_running_processor", "How long the longest running workqueue processor has been running in microseconds.", &["queue_name"])?,
            gv("k8s_client_workqueue_unfinished_work_seconds", "How long unfinished work has been sitting in the workqueue in seconds.", &["queue_name"])?,
        );
        reg!(r; http.0, http.1, cache.0, cache.1, cache.2, cache.3, cache.4, cache.5,
            cache.6, cache.7, queue.0, queue.1, queue.2, queue.3, queue.4, queue.5, queue.6);
        Ok(Self { http, cache, queue })
    }

    pub fn request_result(&self, verb: &str, code: &str) {
        self.http.0.with_label_values(&[verb, code]).inc();
    }
    pub fn request_latency(&self, verb: &str, endpoint: &str, d: Duration) {
        self.http.1.with_label_values(&[verb, endpoint]).observe(d.as_secs_f64());
    }
    pub fn list(&self, d: Duration, items: usize) {
        self.cache.0.with_label_values(&[]).inc();
        self.cache.1.with_label_values(&[]).observe(d.as_secs_f64());
        self.cache.2.with_label_values(&[]).observe(items as f64);
    }
    pub fn watch(&self, d: Duration, events: usize) {
        self.cache.3.with_label_values(&[]).inc();
        self.cache.5.with_label_values(&[]).observe(d.as_secs_f64());
        self.cache.6.with_label_values(&[]).observe(events as f64);
    }
    pub fn short_watch(&self) { self.cache.4.with_label_values(&[]).inc(); }
    pub fn resource_version(&self, v: f64) { self.cache.7.with_label_values(&[]).set(v); }
    pub fn queue(&self, name: &str) -> QueueMetrics {
        QueueMetrics(
            self.queue.0.with_label_values(&[name]), self.queue.1.with_label_values(&[name]),
            self.queue.2.with_label_values(&[name]), self.queue.3.with_label_values(&[name]),
            self.queue.4.with_label_values(&[name]), self.queue.5.with_label_values(&[name]),
            self.queue.6.with_label_values(&[name]),
        )
    }
}

pub struct QueueMetrics(Gauge, IntCounter, Histogram, Histogram, IntCounter, Gauge, Gauge);

impl QueueMetrics {
    pub fn depth_inc(&self) { self.0.inc(); }
    pub fn depth_dec(&self) { self.0.dec(); }
    pub fn add(&self) { self.1.inc(); }
    pub fn latency(&self, d: Duration) { self.2.observe(d.as_secs_f64()); }
    pub fn work(&self, d: Duration) { self.3.observe(d.as_secs_f64()); }
    pub fn retry(&self) { self.4.inc(); }
    pub fn longest(&self, v: f64) { self.5.set(v); }
    pub fn unfinished(&self, seconds: f64) { self.6.set(seconds); }
}
