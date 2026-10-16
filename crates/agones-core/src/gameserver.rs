use std::collections::BTreeMap;

use agones_macros::AgonesStringEnum;
use serde::{Deserialize, Serialize};

pub const GROUP_NAME: &str = "agones.dev";
pub const DEFAULT_PORT_RANGE: &str = "default";
pub const VERSION_ANNOTATION: &str = "agones.dev/sdk-version";
pub const FINALIZER_NAME: &str = "agones.dev/controller";
pub const POD_SAFE_TO_EVICT_ANNOTATION: &str = "cluster-autoscaler.kubernetes.io/safe-to-evict";
pub const DEFAULT_SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum GameServerState {
    #[agones(value = "PortAllocation")]
    PortAllocation,
    #[agones(value = "Creating")]
    Creating,
    #[agones(value = "Starting")]
    Starting,
    #[agones(value = "Scheduled")]
    Scheduled,
    #[agones(value = "RequestReady")]
    RequestReady,
    #[agones(value = "Ready")]
    Ready,
    #[agones(value = "Shutdown")]
    Shutdown,
    #[agones(value = "Error")]
    Error,
    #[agones(value = "Unhealthy")]
    Unhealthy,
    #[agones(value = "Reserved")]
    Reserved,
    #[agones(value = "Allocated")]
    Allocated,
    #[agones(unknown)]
    Unknown(String),
}

impl GameServerState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Shutdown | Self::Error | Self::Unhealthy)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum PortPolicy {
    #[agones(value = "Static")]
    Static,
    #[agones(value = "Dynamic")]
    Dynamic,
    #[agones(value = "Passthrough")]
    Passthrough,
    #[agones(value = "None")]
    None,
    #[agones(unknown)]
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum EvictionSafe {
    #[agones(value = "Always")]
    Always,
    #[agones(value = "OnUpgrade")]
    OnUpgrade,
    #[agones(value = "Never")]
    Never,
    #[agones(unknown)]
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum SdkServerLogLevel {
    #[agones(value = "Info")]
    Info,
    #[agones(value = "Debug")]
    Debug,
    #[agones(value = "Error")]
    Error,
    #[agones(value = "Trace")]
    Trace,
    #[agones(unknown)]
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum Protocol {
    #[agones(value = "TCP")]
    Tcp,
    #[agones(value = "UDP")]
    Udp,
    #[agones(value = "TCPUDP")]
    TcpUdp,
    #[agones(unknown)]
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum SchedulingStrategy {
    #[agones(value = "Packed")]
    Packed,
    #[agones(value = "Distributed")]
    Distributed,
    #[agones(unknown)]
    Unknown(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMeta {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    #[serde(default)]
    pub containers: Vec<Container>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: PodSpec,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub period_seconds: i32,
    #[serde(default)]
    pub failure_threshold: i32,
    #[serde(default)]
    pub initial_delay_seconds: i32,
}

impl Health {
    fn apply_defaults(&mut self) {
        if self.disabled {
            return;
        }
        if self.period_seconds <= 0 {
            self.period_seconds = 5;
        }
        if self.failure_threshold <= 0 {
            self.failure_threshold = 3;
        }
        if self.initial_delay_seconds <= 0 {
            self.initial_delay_seconds = 5;
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerPort {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub range: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port_policy: Option<PortPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default)]
    pub container_port: i32,
    #[serde(default)]
    pub host_port: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
}

impl GameServerPort {
    pub fn status(&self) -> GameServerStatusPort {
        GameServerStatusPort {
            name: self.name.clone(),
            port: if self.port_policy == Some(PortPolicy::None) {
                self.container_port
            } else {
                self.host_port
            },
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkServer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<SdkServerLogLevel>,
    #[serde(default)]
    pub grpc_port: i32,
    #[serde(default)]
    pub http_port: i32,
}

impl SdkServer {
    fn apply_defaults(&mut self) {
        if self.log_level.is_none() {
            self.log_level = Some(SdkServerLogLevel::Info);
        }
        if self.grpc_port == 0 {
            self.grpc_port = 9357;
        }
        if self.http_port == 0 {
            self.http_port = 9358;
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eviction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safe: Option<EvictionSafe>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayersSpec {
    #[serde(default)]
    pub initial_capacity: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatus {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub capacity: i64,
    #[serde(default)]
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterStatus {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub capacity: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStatus {
    #[serde(default)]
    pub capacity: i64,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerSpec {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container: String,
    #[serde(default)]
    pub ports: Vec<GameServerPort>,
    #[serde(default)]
    pub health: Health,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<SchedulingStrategy>,
    #[serde(default)]
    pub sdk_server: SdkServer,
    #[serde(default)]
    pub template: PodTemplateSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub players: Option<PlayersSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<BTreeMap<String, CounterStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lists: Option<BTreeMap<String, ListStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eviction: Option<Eviction>,
}

impl GameServerSpec {
    pub fn apply_defaults(&mut self) {
        if self.template.spec.containers.len() == 1 {
            self.container = self.template.spec.containers[0].name.clone();
        }

        let default_container = self.container.clone();
        for port in &mut self.ports {
            if port.port_policy.is_none() {
                port.port_policy = Some(PortPolicy::Dynamic);
            }
            if port.range.is_empty() {
                port.range = DEFAULT_PORT_RANGE.to_owned();
            }
            if port.protocol.is_none() {
                port.protocol = Some(Protocol::Udp);
            }
            if port.container.as_deref().is_none_or(str::is_empty) {
                port.container = Some(default_container.clone());
            }
        }

        self.health.apply_defaults();

        let eviction = self.eviction.get_or_insert_with(Eviction::default);
        if eviction.safe.is_none() {
            eviction.safe = Some(EvictionSafe::Never);
        }

        if self.scheduling.is_none() {
            self.scheduling = Some(SchedulingStrategy::Packed);
        }
        self.sdk_server.apply_defaults();
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerStatusPort {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub port: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<GameServerState>,
    #[serde(default)]
    pub ports: Vec<GameServerStatusPort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub players: Option<PlayerStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<BTreeMap<String, CounterStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lists: Option<BTreeMap<String, ListStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eviction: Option<Eviction>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FeatureFlags {
    pub player_tracking: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServer {
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: GameServerSpec,
    #[serde(default)]
    pub status: GameServerStatus,
}

impl GameServer {
    pub fn apply_defaults(&mut self) {
        self.apply_defaults_with(DEFAULT_SDK_VERSION, FeatureFlags::default());
    }

    pub fn apply_defaults_with(&mut self, sdk_version: &str, features: FeatureFlags) {
        self.metadata
            .annotations
            .insert(VERSION_ANNOTATION.to_owned(), sdk_version.to_owned());
        self.metadata.finalizers.push(FINALIZER_NAME.to_owned());

        self.spec.apply_defaults();
        self.apply_status_defaults(features);
    }

    fn apply_status_defaults(&mut self, features: FeatureFlags) {
        if self.status.state.is_none() {
            self.status.state = Some(GameServerState::Creating);
            if self.has_port_policy(&PortPolicy::Dynamic)
                || self.has_port_policy(&PortPolicy::Passthrough)
            {
                self.status.state = Some(GameServerState::PortAllocation);
            }
        }

        if features.player_tracking {
            let players = self.status.players.get_or_insert_with(PlayerStatus::default);
            if let Some(spec) = &self.spec.players {
                players.capacity = spec.initial_capacity;
            }
        }

        self.status.eviction = self.spec.eviction.clone();
        if self
            .spec
            .template
            .metadata
            .annotations
            .get(POD_SAFE_TO_EVICT_ANNOTATION)
            .is_some_and(|value| value == "true")
        {
            self.status
                .eviction
                .get_or_insert_with(Eviction::default)
                .safe = Some(EvictionSafe::Always);
        }

        self.status.counters = self.spec.counters.clone();
        self.status.lists = self.spec.lists.clone();
    }

    pub fn has_port_policy(&self, policy: &PortPolicy) -> bool {
        self.spec
            .ports
            .iter()
            .any(|port| port.port_policy.as_ref() == Some(policy))
    }

    pub fn count_ports(&self, predicate: impl Fn(&PortPolicy) -> bool) -> usize {
        self.spec
            .ports
            .iter()
            .filter_map(|port| port.port_policy.as_ref())
            .filter(|policy| predicate(policy))
            .count()
    }

    pub fn count_ports_for_range(
        &self,
        name: &str,
        predicate: impl Fn(&PortPolicy) -> bool,
    ) -> usize {
        self.spec
            .ports
            .iter()
            .filter(|port| port.range == name)
            .filter_map(|port| port.port_policy.as_ref())
            .filter(|policy| predicate(policy))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_game_server() -> GameServer {
        GameServer {
            spec: GameServerSpec {
                ports: vec![GameServerPort {
                    container_port: 999,
                    ..Default::default()
                }],
                template: PodTemplateSpec {
                    spec: PodSpec {
                        containers: vec![Container {
                            name: "testing".into(),
                            image: "testing/image".into(),
                        }],
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn apply_defaults_matches_basic_go_gameserver_invariants() {
        let mut gs = simple_game_server();
        gs.apply_defaults_with("test-version", FeatureFlags::default());

        assert_eq!(gs.metadata.annotations[VERSION_ANNOTATION], "test-version");
        assert!(gs.metadata.finalizers.iter().any(|f| f == FINALIZER_NAME));
        assert_eq!(gs.spec.container, "testing");
        assert_eq!(gs.spec.ports[0].protocol, Some(Protocol::Udp));
        assert_eq!(gs.spec.ports[0].range, DEFAULT_PORT_RANGE);
        assert_eq!(gs.spec.ports[0].port_policy, Some(PortPolicy::Dynamic));
        assert_eq!(gs.spec.ports[0].container.as_deref(), Some("testing"));
        assert_eq!(gs.status.state, Some(GameServerState::PortAllocation));
        assert_eq!(gs.spec.scheduling, Some(SchedulingStrategy::Packed));
        assert_eq!(
            gs.spec.health,
            Health {
                disabled: false,
                period_seconds: 5,
                failure_threshold: 3,
                initial_delay_seconds: 5,
            }
        );
        assert_eq!(
            gs.spec.sdk_server,
            SdkServer {
                log_level: Some(SdkServerLogLevel::Info),
                grpc_port: 9357,
                http_port: 9358,
            }
        );
        assert_eq!(
            gs.spec.eviction.as_ref().and_then(|e| e.safe.clone()),
            Some(EvictionSafe::Never)
        );
        assert_eq!(
            gs.status.eviction.as_ref().and_then(|e| e.safe.clone()),
            Some(EvictionSafe::Never)
        );
    }

    #[test]
    fn static_policy_starts_in_creating() {
        let mut gs = simple_game_server();
        gs.spec.ports[0].port_policy = Some(PortPolicy::Static);
        gs.apply_defaults();
        assert_eq!(gs.status.state, Some(GameServerState::Creating));
    }

    #[test]
    fn passthrough_policy_starts_in_port_allocation() {
        let mut gs = simple_game_server();
        gs.spec.ports[0].port_policy = Some(PortPolicy::Passthrough);
        gs.apply_defaults();
        assert_eq!(gs.status.state, Some(GameServerState::PortAllocation));
    }

    #[test]
    fn existing_values_and_unknown_state_are_preserved() {
        let mut gs = GameServer {
            spec: GameServerSpec {
                container: "testing2".into(),
                ports: vec![GameServerPort {
                    range: DEFAULT_PORT_RANGE.into(),
                    port_policy: Some(PortPolicy::Static),
                    protocol: Some(Protocol::Tcp),
                    ..Default::default()
                }],
                health: Health {
                    disabled: false,
                    period_seconds: 12,
                    failure_threshold: 10,
                    initial_delay_seconds: 11,
                },
                template: PodTemplateSpec {
                    spec: PodSpec {
                        containers: vec![
                            Container { name: "testing".into(), image: "testing/image".into() },
                            Container { name: "testing2".into(), image: "testing/image2".into() },
                        ],
                    },
                    ..Default::default()
                },
                sdk_server: SdkServer {
                    log_level: Some(SdkServerLogLevel::Info),
                    grpc_port: 9357,
                    http_port: 9358,
                },
                ..Default::default()
            },
            status: GameServerStatus {
                state: Some(GameServerState::from("TestState")),
                ..Default::default()
            },
            ..Default::default()
        };

        gs.apply_defaults();
        assert_eq!(gs.spec.container, "testing2");
        assert_eq!(gs.spec.ports[0].protocol, Some(Protocol::Tcp));
        assert_eq!(gs.status.state.as_ref().map(AsRef::as_ref), Some("TestState"));
        assert_eq!(gs.spec.health.period_seconds, 12);
        assert_eq!(gs.spec.health.failure_threshold, 10);
        assert_eq!(gs.spec.health.initial_delay_seconds, 11);
    }

    #[test]
    fn disabled_health_is_not_populated() {
        let mut gs = simple_game_server();
        gs.spec.health.disabled = true;
        gs.apply_defaults();
        assert_eq!(gs.spec.health, Health { disabled: true, ..Default::default() });
    }

    #[test]
    fn safe_to_evict_true_only_overrides_status() {
        let mut gs = simple_game_server();
        gs.spec.template.metadata.annotations.insert(
            POD_SAFE_TO_EVICT_ANNOTATION.into(),
            "true".into(),
        );
        gs.apply_defaults();

        assert_eq!(gs.spec.eviction.unwrap().safe, Some(EvictionSafe::Never));
        assert_eq!(gs.status.eviction.unwrap().safe, Some(EvictionSafe::Always));
    }

    #[test]
    fn counters_and_lists_are_copied_into_status() {
        let mut gs = simple_game_server();
        gs.spec.counters = Some(BTreeMap::from([(
            "games".into(),
            CounterStatus { count: 1, capacity: 100 },
        )]));
        gs.spec.lists = Some(BTreeMap::from([(
            "players".into(),
            ListStatus { capacity: 100, values: vec!["foo".into(), "bar".into()] },
        )]));
        gs.apply_defaults();
        assert_eq!(gs.status.counters, gs.spec.counters);
        assert_eq!(gs.status.lists, gs.spec.lists);
    }

    #[test]
    fn player_tracking_feature_initializes_status_capacity() {
        let mut gs = simple_game_server();
        gs.spec.players = Some(PlayersSpec { initial_capacity: 42 });
        gs.apply_defaults_with(
            DEFAULT_SDK_VERSION,
            FeatureFlags { player_tracking: true },
        );
        assert_eq!(gs.status.players.unwrap().capacity, 42);
    }

    #[test]
    fn port_status_matches_go_none_policy_behavior() {
        let mut port = GameServerPort {
            name: "game".into(),
            container_port: 7777,
            host_port: 7000,
            port_policy: Some(PortPolicy::None),
            ..Default::default()
        };
        assert_eq!(port.status().port, 7777);
        port.port_policy = Some(PortPolicy::Static);
        assert_eq!(port.status().port, 7000);
    }

    #[test]
    fn string_types_round_trip_unknown_values_without_loss() {
        let state = GameServerState::from("FutureState");
        assert_eq!(state.as_str(), "FutureState");
        assert_eq!(GameServerState::known_values()[0], "PortAllocation");
    }

    #[test]
    fn bolero_health_defaulting_preserves_positive_invariant() {
        bolero::check!().for_each(|bytes: &[u8]| {
            let signed = |index: usize| i32::from(bytes.get(index).copied().unwrap_or(0)) - 128;
            let mut health = Health {
                disabled: false,
                period_seconds: signed(0),
                failure_threshold: signed(1),
                initial_delay_seconds: signed(2),
            };
            health.apply_defaults();
            assert!(health.period_seconds > 0);
            assert!(health.failure_threshold > 0);
            assert!(health.initial_delay_seconds > 0);
        });
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn health_defaulting_makes_enabled_thresholds_positive() {
        let mut health = Health {
            disabled: false,
            period_seconds: kani::any(),
            failure_threshold: kani::any(),
            initial_delay_seconds: kani::any(),
        };
        health.apply_defaults();
        assert!(health.period_seconds > 0);
        assert!(health.failure_threshold > 0);
        assert!(health.initial_delay_seconds > 0);
    }
}
