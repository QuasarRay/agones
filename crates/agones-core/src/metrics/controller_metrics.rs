use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use opentelemetry::metrics::{Counter, Histogram, Meter, ObservableGauge};
use summer_opentelemetry::{global, KeyValue};

pub const FLEET_ROLLOUT_PERCENT: &str = "fleet_rollout_percent";
pub const FLEET_REPLICA_COUNT_NAME: &str = "fleets_replicas_count";
pub const FLEET_AUTOSCALER_BUFFER_LIMIT_NAME: &str = "fleet_autoscalers_buffer_limits";
pub const FLEET_AUTOSCALER_BUFFER_SIZE_NAME: &str = "fleet_autoscalers_buffer_size";
pub const FLEET_AUTOSCALER_CURRENT_REPLICA_COUNT_NAME: &str =
    "fleet_autoscalers_current_replicas_count";
pub const FLEET_AUTOSCALERS_DESIRED_REPLICA_COUNT_NAME: &str =
    "fleet_autoscalers_desired_replicas_count";
pub const FLEET_AUTOSCALERS_ABLE_TO_SCALE_NAME: &str = "fleet_autoscalers_able_to_scale";
pub const FLEET_AUTOSCALERS_LIMITED_NAME: &str = "fleet_autoscalers_limited";
pub const FLEET_COUNTERS_NAME: &str = "fleet_counters";
pub const FLEET_LISTS_NAME: &str = "fleet_lists";
pub const GAME_SERVERS_COUNT_NAME: &str = "gameservers_count";
pub const GAME_SERVERS_TOTAL_NAME: &str = "gameservers_total";
pub const GAME_SERVERS_ALLOCATIONS_TOTAL_NAME: &str = "gameservers_allocations_total";
pub const GAME_SERVERS_PLAYER_CONNECTED_TOTAL_NAME: &str = "gameserver_player_connected_total";
pub const GAME_SERVERS_PLAYER_CAPACITY_TOTAL_NAME: &str = "gameserver_player_capacity_total";
pub const NODE_COUNT_NAME: &str = "nodes_count";
pub const GAME_SERVERS_NODE_COUNT_NAME: &str = "gameservers_node_count";
pub const GAME_SERVER_STATE_DURATION_NAME: &str = "gameserver_state_duration";

pub const KEY_NAME: &str = "name";
pub const KEY_NAMESPACE: &str = "namespace";
pub const KEY_FLEET_NAME: &str = "fleet_name";
pub const KEY_TYPE: &str = "type";
pub const KEY_EMPTY: &str = "empty";
pub const KEY_COUNTER: &str = "counter";
pub const KEY_LIST: &str = "list";

pub const STATE_DURATION_SECONDS: &[f64] = &[
    0.0, 1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1024.0,
    2048.0, 4096.0, 8192.0, 16384.0,
];

pub const GAME_SERVERS_PER_NODE_BUCKETS: &[f64] = &[
    0.00001, 1.00001, 2.00001, 3.00001, 4.00001, 5.00001, 6.00001, 7.00001,
    8.00001, 9.00001, 10.00001, 11.00001, 12.00001, 13.00001, 14.00001,
    15.00001, 16.00001, 32.00001, 40.00001, 50.00001, 60.00001, 70.00001,
    80.00001, 90.00001, 100.00001, 110.00001, 120.00001,
];

pub const FLEET_AUTOSCALER_VIEWS: &[&str] = &[
    FLEET_AUTOSCALER_BUFFER_LIMIT_NAME,
    FLEET_AUTOSCALER_BUFFER_SIZE_NAME,
    FLEET_AUTOSCALER_CURRENT_REPLICA_COUNT_NAME,
    FLEET_AUTOSCALERS_DESIRED_REPLICA_COUNT_NAME,
    FLEET_AUTOSCALERS_ABLE_TO_SCALE_NAME,
    FLEET_AUTOSCALERS_LIMITED_NAME,
];

pub const FLEET_VIEWS: &[&str] = &[
    FLEET_ROLLOUT_PERCENT,
    FLEET_REPLICA_COUNT_NAME,
    GAME_SERVERS_COUNT_NAME,
    GAME_SERVERS_TOTAL_NAME,
    GAME_SERVERS_PLAYER_CONNECTED_TOTAL_NAME,
    GAME_SERVERS_PLAYER_CAPACITY_TOTAL_NAME,
    GAME_SERVER_STATE_DURATION_NAME,
    FLEET_COUNTERS_NAME,
    FLEET_LISTS_NAME,
    FLEET_AUTOSCALER_BUFFER_LIMIT_NAME,
    FLEET_AUTOSCALER_BUFFER_SIZE_NAME,
    FLEET_AUTOSCALER_CURRENT_REPLICA_COUNT_NAME,
    FLEET_AUTOSCALERS_DESIRED_REPLICA_COUNT_NAME,
    FLEET_AUTOSCALERS_ABLE_TO_SCALE_NAME,
    FLEET_AUTOSCALERS_LIMITED_NAME,
];

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Metric {
    FleetRollout,
    FleetReplicas,
    FasBufferLimits,
    FasBufferSize,
    FasCurrentReplicas,
    FasDesiredReplicas,
    FasAbleToScale,
    FasLimited,
    FleetCounters,
    FleetLists,
    GameServerCount,
    GameServerTotal,
    GameServerAllocationsTotal,
    GameServerPlayerConnected,
    GameServerPlayerCapacity,
    NodesCount,
    GameServersPerNode,
    GameServerStateDuration,
}

impl Metric {
    // Compact aliases used by controller.rs.
    pub const O: Self = Self::FleetRollout;
    pub const R: Self = Self::FleetReplicas;
    pub const AB: Self = Self::FasBufferLimits;
    pub const AS: Self = Self::FasBufferSize;
    pub const AC: Self = Self::FasCurrentReplicas;
    pub const AD: Self = Self::FasDesiredReplicas;
    pub const AA: Self = Self::FasAbleToScale;
    pub const AL: Self = Self::FasLimited;
    pub const C: Self = Self::FleetCounters;
    pub const L: Self = Self::FleetLists;
    pub const G: Self = Self::GameServerCount;
    pub const T: Self = Self::GameServerTotal;
    pub const X: Self = Self::GameServerAllocationsTotal;
    pub const P: Self = Self::GameServerPlayerConnected;
    pub const Q: Self = Self::GameServerPlayerCapacity;
    pub const N: Self = Self::NodesCount;
    pub const NG: Self = Self::GameServersPerNode;
    pub const D: Self = Self::GameServerStateDuration;

    fn name(self) -> &'static str {
        match self {
            Self::FleetRollout => FLEET_ROLLOUT_PERCENT,
            Self::FleetReplicas => FLEET_REPLICA_COUNT_NAME,
            Self::FasBufferLimits => FLEET_AUTOSCALER_BUFFER_LIMIT_NAME,
            Self::FasBufferSize => FLEET_AUTOSCALER_BUFFER_SIZE_NAME,
            Self::FasCurrentReplicas => FLEET_AUTOSCALER_CURRENT_REPLICA_COUNT_NAME,
            Self::FasDesiredReplicas => FLEET_AUTOSCALERS_DESIRED_REPLICA_COUNT_NAME,
            Self::FasAbleToScale => FLEET_AUTOSCALERS_ABLE_TO_SCALE_NAME,
            Self::FasLimited => FLEET_AUTOSCALERS_LIMITED_NAME,
            Self::FleetCounters => FLEET_COUNTERS_NAME,
            Self::FleetLists => FLEET_LISTS_NAME,
            Self::GameServerCount => GAME_SERVERS_COUNT_NAME,
            Self::GameServerTotal => GAME_SERVERS_TOTAL_NAME,
            Self::GameServerAllocationsTotal => GAME_SERVERS_ALLOCATIONS_TOTAL_NAME,
            Self::GameServerPlayerConnected => GAME_SERVERS_PLAYER_CONNECTED_TOTAL_NAME,
            Self::GameServerPlayerCapacity => GAME_SERVERS_PLAYER_CAPACITY_TOTAL_NAME,
            Self::NodesCount => NODE_COUNT_NAME,
            Self::GameServersPerNode => GAME_SERVERS_NODE_COUNT_NAME,
            Self::GameServerStateDuration => GAME_SERVER_STATE_DURATION_NAME,
        }
    }

    fn tags(self) -> &'static [&'static str] {
        match self {
            Self::FleetRollout | Self::FleetReplicas => &[KEY_NAME, KEY_TYPE, KEY_NAMESPACE],
            Self::FasBufferLimits | Self::FasBufferSize => {
                &[KEY_NAME, KEY_TYPE, KEY_FLEET_NAME, KEY_NAMESPACE]
            }
            Self::FasCurrentReplicas
            | Self::FasDesiredReplicas
            | Self::FasAbleToScale
            | Self::FasLimited => &[KEY_NAME, KEY_FLEET_NAME, KEY_NAMESPACE],
            Self::FleetCounters => &[KEY_NAME, KEY_NAMESPACE, KEY_TYPE, KEY_COUNTER],
            Self::FleetLists => &[KEY_NAME, KEY_NAMESPACE, KEY_TYPE, KEY_LIST],
            Self::GameServerCount | Self::GameServerTotal | Self::GameServerStateDuration => {
                &[KEY_TYPE, KEY_FLEET_NAME, KEY_NAMESPACE]
            }
            Self::GameServerAllocationsTotal => &[KEY_FLEET_NAME, KEY_NAMESPACE],
            Self::GameServerPlayerConnected | Self::GameServerPlayerCapacity => {
                &[KEY_FLEET_NAME, KEY_NAME, KEY_NAMESPACE]
            }
            Self::NodesCount => &[KEY_EMPTY],
            Self::GameServersPerNode => &[],
        }
    }

    fn is_gauge(self) -> bool {
        !matches!(
            self,
            Self::GameServerTotal
                | Self::GameServerAllocationsTotal
                | Self::GameServersPerNode
                | Self::GameServerStateDuration
        )
    }
}

type GaugeSeries = HashMap<Vec<String>, i64>;
type GaugeStore = Arc<RwLock<HashMap<Metric, GaugeSeries>>>;

fn attrs(metric: Metric, values: &[String]) -> Vec<KeyValue> {
    let keys = metric.tags();
    assert_eq!(
        keys.len(),
        values.len(),
        "{} expects {} labels, got {}",
        metric.name(),
        keys.len(),
        values.len()
    );
    keys.iter()
        .zip(values)
        .map(|(&key, value)| KeyValue::new(key, value.clone()))
        .collect()
}

fn observable_gauge(
    meter: &Meter,
    store: &GaugeStore,
    metric: Metric,
    description: &'static str,
) -> ObservableGauge<i64> {
    let store = Arc::clone(store);
    meter
        .i64_observable_gauge(metric.name())
        .with_description(description)
        .with_unit("1")
        .with_callback(move |observer| {
            let store = store.read().unwrap_or_else(|e| e.into_inner());
            if let Some(series) = store.get(&metric) {
                for (labels, &value) in series {
                    observer.observe(value, &attrs(metric, labels));
                }
            }
        })
        .build()
}

#[derive(Clone)]
pub struct StateMetrics {
    gauges: GaugeStore,
    // Observable handles must stay alive for their callbacks to remain registered.
    _gauge_handles: Arc<Vec<ObservableGauge<i64>>>,
    total: Arc<Counter<u64>>,
    allocations: Arc<Counter<u64>>,
    per_node: Arc<Histogram<f64>>,
    state_duration: Arc<Histogram<f64>>,
}

impl Default for StateMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMetrics {
    /// Build Agones instruments on the global meter provider installed by
    /// `summer_opentelemetry::OpenTelemetryPlugin`.
    ///
    /// Construct this after Summer has initialized the plugin; otherwise the
    /// global OpenTelemetry provider may still be the no-op provider.
    pub fn new() -> Self {
        let meter = global::meter("agones.metrics");
        let gauges: GaugeStore = Arc::new(RwLock::new(HashMap::new()));

        let gauge_specs = [
            (Metric::O, "Measures the current progress of fleet rollout"),
            (Metric::R, "The number of replicas per fleet"),
            (Metric::AB, "The limits of buffer based fleet autoscalers"),
            (Metric::AS, "The buffer size of fleet autoscalers"),
            (Metric::AC, "The current replicas count as seen by autoscalers"),
            (Metric::AD, "The desired replicas count as seen by autoscalers"),
            (Metric::AA, "The fleet autoscaler can access the fleet to scale"),
            (Metric::AL, "The fleet autoscaler is capped"),
            (
                Metric::C,
                "Aggregated Counters counts and capacity across GameServers in the Fleet",
            ),
            (
                Metric::L,
                "Aggregated Lists counts and capacity across GameServers in the Fleet",
            ),
            (Metric::G, "The number of gameservers"),
            (
                Metric::P,
                "The current amount of players connected in gameservers",
            ),
            (Metric::Q, "The available player capacity per gameserver"),
            (Metric::N, "The count of nodes in the cluster"),
        ];

        let gauge_handles = gauge_specs
            .into_iter()
            .map(|(metric, description)| observable_gauge(&meter, &gauges, metric, description))
            .collect();

        let total = meter
            .u64_counter(GAME_SERVERS_TOTAL_NAME)
            .with_description("The total of gameservers")
            .with_unit("1")
            .build();

        let allocations = meter
            .u64_counter(GAME_SERVERS_ALLOCATIONS_TOTAL_NAME)
            .with_description("The total of gameserver allocations")
            .with_unit("1")
            .build();

        let per_node = meter
            .f64_histogram(GAME_SERVERS_NODE_COUNT_NAME)
            .with_description("The count of gameservers per node in the cluster")
            .with_unit("1")
            .with_boundaries(GAME_SERVERS_PER_NODE_BUCKETS.to_vec())
            .build();

        let state_duration = meter
            .f64_histogram(GAME_SERVER_STATE_DURATION_NAME)
            .with_description("The time gameserver exists in the current state in seconds")
            .with_unit("s")
            .with_boundaries(STATE_DURATION_SECONDS.to_vec())
            .build();

        Self {
            gauges,
            _gauge_handles: Arc::new(gauge_handles),
            total: Arc::new(total),
            allocations: Arc::new(allocations),
            per_node: Arc::new(per_node),
            state_duration: Arc::new(state_duration),
        }
    }

    pub fn set_i64(&self, metric: Metric, labels: &[&str], value: i64) {
        assert!(metric.is_gauge(), "{} is not a gauge", metric.name());
        assert_eq!(metric.tags().len(), labels.len());

        self.gauges
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .entry(metric)
            .or_default()
            .insert(labels.iter().map(|v| (*v).to_owned()).collect(), value);
    }

    // Compatibility with the compact controller.rs API.
    pub fn s(&self, metric: Metric, labels: &[&str], value: i64) {
        self.set_i64(metric, labels, value);
    }

    pub fn increment(&self, metric: Metric, labels: &[&str]) {
        let owned: Vec<String> = labels.iter().map(|v| (*v).to_owned()).collect();
        let labels = attrs(metric, &owned);
        match metric {
            Metric::GameServerTotal => self.total.add(1, &labels),
            Metric::GameServerAllocationsTotal => self.allocations.add(1, &labels),
            _ => panic!("{} is not a Count metric", metric.name()),
        }
    }

    pub fn i(&self, metric: Metric, labels: &[&str]) {
        self.increment(metric, labels);
    }

    pub fn observe<V: Observation>(&self, metric: Metric, labels: &[&str], value: V) {
        let owned: Vec<String> = labels.iter().map(|v| (*v).to_owned()).collect();
        let labels = attrs(metric, &owned);
        match metric {
            Metric::GameServersPerNode => self.per_node.record(value.into_f64(), &labels),
            Metric::GameServerStateDuration => self.state_duration.record(value.into_f64(), &labels),
            _ => panic!("{} is not a Distribution metric", metric.name()),
        }
    }

    pub fn o<V: Observation>(&self, metric: Metric, labels: &[&str], value: V) {
        self.observe(metric, labels, value);
    }

    /// Remove currently exported LastValue series whose view names match `names`.
    ///
    /// OpenTelemetry does not expose OpenCensus' unregister/re-register operation
    /// for instruments owned by a shared MeterProvider. Counters and histograms
    /// therefore remain SDK-managed; observable gauges disappear on the next
    /// collection after their stored series are removed here.
    pub fn reset_views(&self, names: &[&str]) {
        let mut gauges = self.gauges.write().unwrap_or_else(|e| e.into_inner());
        gauges.retain(|metric, _| !names.contains(&metric.name()));
    }

    pub fn reset_fleet_views(&self) {
        self.reset_views(FLEET_VIEWS);
    }

    pub fn reset_fleet_autoscaler_views(&self) {
        self.reset_views(FLEET_AUTOSCALER_VIEWS);
    }

    // Compatibility with controller.rs.
    pub fn rf(&self) {
        self.reset_fleet_views();
    }

    pub fn ra(&self) {
        self.reset_fleet_autoscaler_views();
    }
}

pub trait Observation {
    fn into_f64(self) -> f64;
}

macro_rules! observation {
    ($($t:ty),* $(,)?) => {$(
        impl Observation for $t {
            fn into_f64(self) -> f64 { self as f64 }
        }
    )*};
}

observation!(i32, i64, isize, u32, u64, usize, f32, f64);
