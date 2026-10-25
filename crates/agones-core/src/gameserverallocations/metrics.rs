use std::time::Duration;

use agones_core::{GameServerAllocation, GameServerAllocationState, SchedulingStrategy};
use prometheus::{HistogramOpts, HistogramVec, Registry};

use super::allocator::{AllocationMetrics, AllocationOutput, AllocatorError};

const LABELS: [&str; 5] = [
    "fleet_name", "cluster_name", "is_multicluster", "status", "scheduling_strategy",
];
const LATENCY_BUCKETS: [f64; 12] =
    [0., .01, .025, .05, .075, .1, .25, .5, .75, 1., 2., 3.];
const RETRY_BUCKETS: [f64; 5] = [1., 2., 3., 4., 5.];
const FLEET: &str = "agones.dev/fleet";

#[derive(Clone)]
pub struct Metrics {
    latency: HistogramVec,
    retries: HistogramVec,
}

impl Metrics {
    pub fn register(registry: &Registry) -> prometheus::Result<Self> {
        let latency = HistogramVec::new(
            HistogramOpts::new(
                "agones_gameserver_allocations_duration_seconds",
                "The distribution of gameserver allocation requests latencies.",
            )
            .buckets(LATENCY_BUCKETS.to_vec()),
            &LABELS,
        )?;
        let retries = HistogramVec::new(
            HistogramOpts::new(
                "agones_gameserver_allocations_retry_total",
                "The count of gameserver allocation retry until it succeeds",
            )
            .buckets(RETRY_BUCKETS.to_vec()),
            &LABELS,
        )?;
        registry.register(Box::new(latency.clone()))?;
        if let Err(e) = registry.register(Box::new(retries.clone())) {
            let _ = registry.unregister(Box::new(latency.clone()));
            return Err(e);
        }
        Ok(Self { latency, retries })
    }

    pub fn unregister(&self, registry: &Registry) {
        let _ = registry.unregister(Box::new(self.latency.clone()));
        let _ = registry.unregister(Box::new(self.retries.clone()));
    }

    fn labels(
        request: &GameServerAllocation,
        response: Option<&AllocationOutput>,
        error: Option<&AllocatorError>,
    ) -> [String; 5] {
        let mut l = [
            "none".into(),
            "none".into(),
            request.spec.multi_cluster_setting.enabled.to_string(),
            "none".into(),
            request.spec.scheduling.as_ref().map_or("", SchedulingStrategy::as_str).into(),
        ];
        if error.is_some() {
            l[3] = "error".into();
        } else if let Some(AllocationOutput::Allocation(a)) = response {
            l[3] = a.status.state.as_str().into();
            if a.status.state == GameServerAllocationState::Allocated && a.status.source == "local" {
                if let Some(fleet) = a.status.metadata.as_ref()
                    .and_then(|m| m.labels.get(FLEET)).filter(|v| !v.is_empty())
                {
                    l[0] = fleet.clone();
                }
            }
        }
        l
    }
}

impl AllocationMetrics for Metrics {
    fn allocation(
        &self,
        request: &GameServerAllocation,
        response: Option<&AllocationOutput>,
        error: Option<&AllocatorError>,
        elapsed: Duration,
    ) {
        let l = Self::labels(request, response, error);
        let v = [&*l[0], &*l[1], &*l[2], &*l[3], &*l[4]];
        self.latency.with_label_values(&v).observe(elapsed.as_secs_f64());
    }

    fn retry_success(&self, retries: usize) {
        self.retries
            .with_label_values(&["", "", "", "Success", ""])
            .observe(retries as f64);
    }
}