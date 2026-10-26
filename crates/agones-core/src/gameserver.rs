use std::collections::{BTreeMap, BTreeSet};
use std::net::IpAddr;

use agones_macros::{AgonesStringEnum, agones_invariant};
use serde::{Deserialize, Serialize};

pub use agones_source::pkg::apis::agones::GroupName as GROUP_NAME;
pub const DEFAULT_PORT_RANGE: &str = "default";
pub const VERSION_ANNOTATION: &str = "agones.dev/sdk-version";
pub const FINALIZER_NAME: &str = "agones.dev/controller";
pub const POD_SAFE_TO_EVICT_ANNOTATION: &str = "cluster-autoscaler.kubernetes.io/safe-to-evict";
pub const DEV_ADDRESS_ANNOTATION: &str = "agones.dev/dev-address";
pub const GAME_SERVER_READY_CONTAINER_ID_ANNOTATION: &str = "agones.dev/ready-container-id";
pub const DEFAULT_SDK_VERSION: &str = crate::AGONES_BUILD_VERSION;

pub const ERR_CONTAINER_REQUIRED: &str =
    "Container is required when using multiple containers in the pod template";
pub const ERR_HOST_PORT: &str =
    "HostPort cannot be specified with a Dynamic or Passthrough PortPolicy";
pub const ERR_PORT_POLICY_STATIC: &str = "PortPolicy must be Static";
pub const ERR_CONTAINER_PORT_REQUIRED: &str =
    "ContainerPort must be defined for Dynamic and Static PortPolicies";
pub const ERR_CONTAINER_PORT_PASSTHROUGH: &str =
    "ContainerPort cannot be specified with Passthrough PortPolicy";
pub const ERR_CONTAINER_NAME_INVALID: &str =
    "Container must be empty or the name of a container in the pod template";
pub const GAME_SERVER_PRIORITY_INCREMENT: &str = "Increment";
pub const GAME_SERVER_PRIORITY_DECREMENT: &str = "Decrement";
pub const GAME_SERVER_PRIORITY_COUNTER: &str = "Counter";
pub const GAME_SERVER_PRIORITY_LIST: &str = "List";
pub const GAME_SERVER_PRIORITY_ASCENDING: &str = "Ascending";
pub const GAME_SERVER_PRIORITY_DESCENDING: &str = "Descending";

fn is_false(value: &bool) -> bool {
    !*value
}

fn is_zero_i32(value: &i32) -> bool {
    *value == 0
}

fn is_zero_i64(value: &i64) -> bool {
    *value == 0
}

fn go_string_is_empty<T: AsRef<str>>(value: &Option<T>) -> bool {
    value.as_ref().is_none_or(|value| value.as_ref().is_empty())
}

fn optional_map_is_empty<K, V>(value: &Option<BTreeMap<K, V>>) -> bool {
    value.as_ref().is_none_or(BTreeMap::is_empty)
}

fn serialize_game_server_state<S: serde::Serializer>(
    value: &Option<GameServerState>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(value.as_ref().map_or("", GameServerState::as_str))
}

fn deserialize_game_server_state<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<GameServerState>, D::Error> {
    // State is a non-pointer Go string: missing, null and "" are one zero
    // value. Keep one canonical Rust representation while preserving every
    // nonempty known or future state.
    Ok(Option::<GameServerState>::deserialize(deserializer)?
        .filter(|state| !state.as_str().is_empty()))
}

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

/// Source metav1.TypeMeta, retained independently of static resource identity.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeMeta {
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub api_version: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMeta {
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub generate_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub namespace: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub labels: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub annotations: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub finalizers: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub uid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<String>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum ContainerRestartPolicy {
    #[agones(value = "Always")]
    Always,
    #[agones(value = "Never")]
    Never,
    #[agones(value = "OnFailure")]
    OnFailure,
    #[agones(unknown)]
    Unknown(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub image: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<ContainerRestartPolicy>,
    #[serde(default, skip_serializing_if = "is_false", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub tty: bool,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub resources: crate::ContainerResourceRequirements,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub containers: crate::GoSlice<Container>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub init_containers: Vec<Container>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub metadata: ObjectMeta,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub spec: PodSpec,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    #[serde(default, skip_serializing_if = "is_false", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "is_zero_i32", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub period_seconds: i32,
    #[serde(default, skip_serializing_if = "is_zero_i32", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub failure_threshold: i32,
    #[serde(default, skip_serializing_if = "is_zero_i32", deserialize_with = "crate::go_value::deserialize_go_value")]
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

#[agones_invariant]
fn health_defaulting_makes_enabled_thresholds_positive(
    input: (i32, i32, i32),
) -> bool {
    let (period_seconds, failure_threshold, initial_delay_seconds) = input;
    let mut health = Health {
        disabled: false,
        period_seconds,
        failure_threshold,
        initial_delay_seconds,
    };
    health.apply_defaults();
    health.period_seconds > 0
        && health.failure_threshold > 0
        && health.initial_delay_seconds > 0
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerPort {
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub range: String,
    #[serde(default, skip_serializing_if = "go_string_is_empty")]
    pub port_policy: Option<PortPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default, skip_serializing_if = "is_zero_i32", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub container_port: i32,
    #[serde(default, skip_serializing_if = "is_zero_i32", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub host_port: i32,
    #[serde(default, skip_serializing_if = "go_string_is_empty")]
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
    #[serde(default, skip_serializing_if = "go_string_is_empty")]
    pub log_level: Option<SdkServerLogLevel>,
    #[serde(default, skip_serializing_if = "is_zero_i32", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub grpc_port: i32,
    #[serde(default, skip_serializing_if = "is_zero_i32", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub http_port: i32,
}

impl SdkServer {
    fn apply_defaults(&mut self) {
        if go_string_is_empty(&self.log_level) {
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
    #[serde(default, skip_serializing_if = "go_string_is_empty")]
    pub safe: Option<EvictionSafe>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayersSpec {
    #[serde(default, skip_serializing_if = "is_zero_i64", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub initial_capacity: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatus {
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub count: i64,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub capacity: i64,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub ids: crate::GoSlice<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    #[serde(rename = "type")]
    pub priority_type: String,
    pub key: String,
    pub order: String,
}

/// Mirrors Agones SafeAdd, including Go's two's-complement overflow behavior
/// outside the intended nonnegative-addend domain.
pub fn safe_add(x: i64, y: i64) -> i64 {
    let threshold = i64::MAX.wrapping_sub(y);
    if x > threshold {
        i64::MAX
    } else {
        x.wrapping_add(y)
    }
}

#[agones_invariant]
fn safe_add_nonnegative_inputs_are_saturating(input: (i64, i64)) -> bool {
    let (x, y) = input;
    if x < 0 || y < 0 {
        return true;
    }

    let result = safe_add(x, y);
    result >= x
        && result <= i64::MAX
        && if x <= i64::MAX - y {
            result == x + y
        } else {
            result == i64::MAX
        }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterStatus {
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub count: i64,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub capacity: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStatus {
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub capacity: i64,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub values: crate::GoSlice<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerSpec {
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub container: String,
    #[serde(default, skip_serializing_if = "crate::GoSlice::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub ports: crate::GoSlice<GameServerPort>,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub health: Health,
    #[serde(default, skip_serializing_if = "go_string_is_empty")]
    pub scheduling: Option<SchedulingStrategy>,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub sdk_server: SdkServer,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub template: PodTemplateSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub players: Option<PlayersSpec>,
    #[serde(default, skip_serializing_if = "optional_map_is_empty")]
    pub counters: Option<BTreeMap<String, CounterStatus>>,
    #[serde(default, skip_serializing_if = "optional_map_is_empty")]
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
            if go_string_is_empty(&port.port_policy) {
                port.port_policy = Some(PortPolicy::Dynamic);
            }
            if port.range.is_empty() {
                port.range = DEFAULT_PORT_RANGE.to_owned();
            }
            if go_string_is_empty(&port.protocol) {
                port.protocol = Some(Protocol::Udp);
            }
            if port.container.as_deref().is_none_or(str::is_empty) {
                port.container = Some(default_container.clone());
            }
        }

        self.health.apply_defaults();

        let eviction = self.eviction.get_or_insert_with(Eviction::default);
        if go_string_is_empty(&eviction.safe) {
            eviction.safe = Some(EvictionSafe::Never);
        }

        if go_string_is_empty(&self.scheduling) {
            self.scheduling = Some(SchedulingStrategy::Packed);
        }
        self.sdk_server.apply_defaults();
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddress {
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub address: String,
    #[serde(rename = "type", default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub address_type: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerStatusPort {
    #[serde(default, skip_serializing_if = "String::is_empty", deserialize_with = "crate::go_value::deserialize_go_value")]
    pub name: String,
    pub port: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerStatus {
    #[serde(default, deserialize_with = "deserialize_game_server_state", serialize_with = "serialize_game_server_state")]
    pub state: Option<GameServerState>,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub ports: crate::GoSlice<GameServerStatusPort>,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub address: String,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub addresses: crate::GoSlice<NodeAddress>,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub node_name: String,
    #[serde(default)]
    pub reserved_until: Option<String>,
    #[serde(default)]
    pub players: Option<PlayerStatus>,
    #[serde(default, skip_serializing_if = "optional_map_is_empty")]
    pub counters: Option<BTreeMap<String, CounterStatus>>,
    #[serde(default, skip_serializing_if = "optional_map_is_empty")]
    pub lists: Option<BTreeMap<String, ListStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eviction: Option<Eviction>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FeatureFlags {
    pub player_tracking: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationErrorKind {
    Required,
    Duplicate,
    Forbidden,
    Invalid,
    NotSupported,
    TooLong,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationError {
    pub kind: ValidationErrorKind,
    pub path: String,
    pub bad_value: Option<String>,
    pub detail: String,
    pub origin: String,
    pub covered_by_declarative: bool,
}

impl ValidationError {
    fn required(path: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            kind: ValidationErrorKind::Required,
            path: path.into(),
            bad_value: None,
            detail: detail.into(),
            origin: String::new(),
            covered_by_declarative: false,
        }
    }

    fn forbidden(path: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            kind: ValidationErrorKind::Forbidden,
            path: path.into(),
            bad_value: None,
            detail: detail.into(),
            origin: String::new(),
            covered_by_declarative: false,
        }
    }

    pub(crate) fn invalid(
        path: impl Into<String>,
        bad_value: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind: ValidationErrorKind::Invalid,
            path: path.into(),
            bad_value: Some(bad_value.into()),
            detail: detail.into(),
            origin: String::new(),
            covered_by_declarative: false,
        }
    }

    pub(crate) fn too_long(path: impl Into<String>, _bad_value: impl Into<String>, max: isize) -> Self {
        Self {
            kind: ValidationErrorKind::TooLong,
            path: path.into(),
            bad_value: Some("<value omitted>".into()),
            detail: if max < 0 {
                "value is too long".into()
            } else {
                format!("may not be more than {max} {}", if max == 1 { "byte" } else { "bytes" })
            },
            origin: String::new(),
            covered_by_declarative: false,
        }
    }

    pub fn with_origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = origin.into();
        self
    }

    pub fn mark_covered_by_declarative(mut self) -> Self {
        self.covered_by_declarative = true;
        self
    }
}

// Build the source field.Path index directly. Successful port validation needs
// no path allocation; errors preserve the same decimal index and field text.
pub(crate) fn indexed_validation_field(path: &str, index: usize, field: &str) -> String {
    let mut result = String::from(path);
    result.push('[');
    let mut digits = [0u8; usize::BITS as usize];
    let mut start = digits.len();
    let mut remaining = index;
    loop {
        start -= 1;
        digits[start] = b'0' + (remaining % 10) as u8;
        remaining /= 10;
        if remaining == 0 {
            break;
        }
    }
    for digit in &digits[start..] {
        result.push(char::from(*digit));
    }
    result.push(']');
    result.push('.');
    result.push_str(field);
    result
}

impl GameServerSpec {
    pub fn has_container(&self, name: &str, include_sidecar: bool) -> bool {
        if self.template.spec.containers.iter().any(|c| c.name == name) {
            return true;
        }
        include_sidecar
            && self.template.spec.init_containers.iter().any(|c| {
                c.restart_policy == Some(ContainerRestartPolicy::Always) && c.name == name
            })
    }

    /// Domain-level validation directly corresponding to the Kubernetes-independent
    /// checks in Agones GameServerSpec.Validate.
    ///
    /// Resource quantities use the shared core CGP policy. API hook effects
    /// are supplied by the Rusternetes integration layer.
    pub fn validate(&self, dev_address: &str) -> Vec<ValidationError> {
        self.validate_with_context(&crate::GameServerDefaultingContext::default(), dev_address)
    }

    pub fn validate_with_context<C: crate::CanValidateGameServerFeatures + crate::CanValidateResourceQuantities + crate::CanValidateObjectMetadata>(
        &self,
        context: &C,
        dev_address: &str,
    ) -> Vec<ValidationError> {
        let mut errors = context.validate_game_server_features(self, "spec");

        if !dev_address.is_empty() {
            if dev_address.parse::<IpAddr>().is_err() {
                errors.push(ValidationError::invalid(
                    format!("metadata.annotations.{DEV_ADDRESS_ANNOTATION}"),
                    dev_address,
                    "must be a valid IP address",
                ));
            }
            for (index, port) in self.ports.iter().enumerate() {
                if port.host_port == 0 {
                    errors.push(ValidationError::required(
                        indexed_validation_field("spec.ports", index, "hostPort"),
                        DEV_ADDRESS_ANNOTATION,
                    ));
                }
                if port.port_policy != Some(PortPolicy::Static) {
                    errors.push(ValidationError::required(
                        indexed_validation_field("spec.ports", index, "portPolicy"),
                        ERR_PORT_POLICY_STATIC,
                    ));
                }
            }
            errors.extend(context.validate_object_metadata(
                &self.template.metadata.labels,
                &self.template.metadata.annotations,
                "spec.template.metadata",
            ));
            return errors;
        }

        if self.container.is_empty() && self.template.spec.containers.len() > 1 {
            errors.push(ValidationError::required(
                "spec.container",
                ERR_CONTAINER_REQUIRED,
            ));
        }

        if !self.has_container(&self.container, false) {
            let prefix = "Could not find a container named ";
            let mut detail = String::with_capacity(prefix.len() + self.container.len());
            detail.push_str(prefix);
            detail.push_str(&self.container);
            errors.push(ValidationError::invalid(
                "spec.container",
                self.container.as_str(),
                detail,
            ));
        }

        for (index, port) in self.ports.iter().enumerate() {
            if matches!(
                port.port_policy.as_ref(),
                Some(PortPolicy::Dynamic | PortPolicy::Static)
            ) && port.container_port <= 0
            {
                errors.push(ValidationError::required(
                    indexed_validation_field("spec.ports", index, "containerPort"),
                    ERR_CONTAINER_PORT_REQUIRED,
                ));
            }

            if port.port_policy == Some(PortPolicy::Passthrough) && port.container_port > 0 {
                errors.push(ValidationError::required(
                    indexed_validation_field("spec.ports", index, "containerPort"),
                    ERR_CONTAINER_PORT_PASSTHROUGH,
                ));
            }

            if port.host_port > 0
                && matches!(
                    port.port_policy.as_ref(),
                    Some(PortPolicy::Dynamic | PortPolicy::Passthrough)
                )
            {
                errors.push(ValidationError::forbidden(
                    indexed_validation_field("spec.ports", index, "hostPort"),
                    ERR_HOST_PORT,
                ));
            }

            if let Some(container) = port.container.as_deref() {
                if !self.container.is_empty() && !self.has_container(container, true) {
                    errors.push(ValidationError::invalid(
                        indexed_validation_field("spec.ports", index, "container"),
                        container,
                        ERR_CONTAINER_NAME_INVALID,
                    ));
                }
            }
        }

        errors.extend(self.validate_resource_quantities_with_context(context, "spec"));
        errors.extend(context.validate_object_metadata(
            &self.template.metadata.labels,
            &self.template.metadata.annotations,
            "spec.template.metadata",
        ));
        errors
    }
}


#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    agones_macros::AgonesApiResource,
)]
#[agones_resource(
    group = "agones.dev",
    version = "v1",
    kind = "GameServer",
    plural = "gameservers"
)]
#[serde(rename_all = "camelCase")]
pub struct GameServer {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub metadata: ObjectMeta,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub spec: GameServerSpec,
    #[serde(default, deserialize_with = "crate::go_value::deserialize_go_value")]
    pub status: GameServerStatus,
}

impl GameServer {
    pub fn apply_defaults(&mut self) {
        self.apply_defaults_with_context(&crate::GameServerDefaultingContext::default());
    }

    pub fn apply_defaults_with_context<C: crate::CanApplyGameServerDefaults>(
        &mut self,
        context: &C,
    ) {
        context.apply_game_server_defaults(self);
    }

    pub fn apply_defaults_with_version_context<C: crate::CanGetAgonesVersionAnnotation>(
        &mut self,
        context: &C,
        features: FeatureFlags,
    ) {
        self.apply_defaults_with(&context.agones_version_annotation(), features);
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
        if go_string_is_empty(&self.status.state) {
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

        if let Some(counters) = &self.spec.counters {
            self.status.counters = Some(counters.clone());
        }
        if let Some(lists) = &self.spec.lists {
            self.status.lists = Some(lists.clone());
        }
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


#[derive(Clone, Debug, PartialEq, Serialize)]
struct JsonPatchOperation {
    op: &'static str,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<serde_json::Value>,
}

impl JsonPatchOperation {
    fn test(path: impl Into<String>, value: serde_json::Value) -> Self {
        Self {
            op: "test",
            path: path.into(),
            value: Some(value),
        }
    }

    fn add(path: impl Into<String>, value: serde_json::Value) -> Self {
        Self {
            op: "add",
            path: path.into(),
            value: Some(value),
        }
    }

    fn remove(path: impl Into<String>) -> Self {
        Self {
            op: "remove",
            path: path.into(),
            value: None,
        }
    }

    fn replace(path: impl Into<String>, value: serde_json::Value) -> Self {
        Self {
            op: "replace",
            path: path.into(),
            value: Some(value),
        }
    }
}

fn json_pointer_segment(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

fn child_pointer(path: &str, key: &str) -> String {
    let escaped = json_pointer_segment(key);
    if path.is_empty() {
        format!("/{escaped}")
    } else {
        format!("{path}/{escaped}")
    }
}

fn diff_json(
    path: &str,
    old: &serde_json::Value,
    new: &serde_json::Value,
    operations: &mut Vec<JsonPatchOperation>,
) {
    if old == new {
        return;
    }

    match (old, new) {
        (serde_json::Value::Object(old), serde_json::Value::Object(new)) => {
            for key in old.keys() {
                if !new.contains_key(key) {
                    operations.push(JsonPatchOperation::remove(child_pointer(path, key)));
                }
            }
            for (key, new_value) in new {
                let child = child_pointer(path, key);
                match old.get(key) {
                    Some(old_value) => diff_json(&child, old_value, new_value, operations),
                    None => operations.push(JsonPatchOperation::add(child, new_value.clone())),
                }
            }
        }
        _ => operations.push(JsonPatchOperation::replace(path, new.clone())),
    }
}

impl GameServer {
    pub fn get_dev_address(&self) -> Option<&str> {
        self.metadata
            .annotations
            .get(DEV_ADDRESS_ANNOTATION)
            .map(String::as_str)
    }

    /// Applies the pure state transition performed by the upstream
    /// GameServer controller's syncDevelopmentGameServer path.
    ///
    /// Returns true only when the resource should be persisted.
    pub fn reconcile_development_status(&mut self) -> bool {
        if self.metadata.deletion_timestamp.is_some() {
            return false;
        }

        let Some(dev_address) = self.get_dev_address().map(str::to_owned) else {
            return false;
        };

        let state = self.status.state.clone();
        if !matches!(
            state.as_ref(),
            Some(GameServerState::Creating | GameServerState::RequestReady)
        ) {
            return false;
        }

        self.status.state = Some(GameServerState::Ready);

        if state == Some(GameServerState::Creating) {
            self.status.ports = self.spec.ports.iter().map(GameServerPort::status).collect();
            self.status.address = dev_address.clone();
            self.status.addresses = vec![NodeAddress {
                address: dev_address.clone(),
                address_type: "InternalIP".to_owned(),
            }].into();
            self.status.node_name = dev_address;
        }

        true
    }

    /// Mirrors the Static + TCPUDP normalization performed immediately before
    /// upstream GameServer Pod creation.
    ///
    /// Returns true when at least one port was split. The transformation is
    /// idempotent because transformed entries no longer use TCPUDP.
    pub fn split_static_tcp_udp_ports(&mut self) -> bool {
        let mut appended = Vec::new();

        for port in &mut self.spec.ports {
            if port.port_policy != Some(PortPolicy::Static)
                || port.protocol != Some(Protocol::TcpUdp)
            {
                continue;
            }

            let base_name = port.name.clone();
            let udp = GameServerPort {
                name: format!("{base_name}-udp"),
                port_policy: Some(PortPolicy::Static),
                container: port.container.clone(),
                container_port: port.container_port,
                host_port: port.host_port,
                protocol: Some(Protocol::Udp),
                ..Default::default()
            };

            port.name = format!("{base_name}-tcp");
            port.protocol = Some(Protocol::Tcp);
            appended.push(udp);
        }

        let changed = !appended.is_empty();
        self.spec.ports.extend(appended);
        changed
    }


    pub fn patch(&self, delta: &GameServer) -> Result<Vec<u8>, serde_json::Error> {
        let old = serde_json::to_value(self)?;
        let new = serde_json::to_value(delta)?;
        let mut operations = vec![JsonPatchOperation::test(
            "/metadata/resourceVersion",
            self.metadata
                .resource_version
                .as_ref()
                .map_or(serde_json::Value::Null, |value| {
                    serde_json::Value::String(value.clone())
                }),
        )];
        diff_json("", &old, &new, &mut operations);
        serde_json::to_vec(&operations)
    }

    pub fn validate(&self) -> Vec<ValidationError> {
        self.validate_with_context(&crate::GameServerDefaultingContext::default())
    }

    pub fn validate_with_context<C: crate::CanValidateGameServerFeatures + crate::CanValidateResourceQuantities + crate::CanValidateObjectMetadata>(
        &self,
        context: &C,
    ) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.metadata.name.len() > 63 {
            errors.push(ValidationError::too_long(
                "metadata.name",
                self.metadata.name.as_str(),
                63,
            ));
        }
        errors.extend(self.spec.validate_with_context(context, self.get_dev_address().unwrap_or("")));
        errors
    }

    pub fn is_deletable(&self) -> bool {
        if matches!(
            self.status.state.as_ref(),
            Some(GameServerState::Allocated | GameServerState::Reserved)
        ) {
            return self.metadata.deletion_timestamp.is_some();
        }
        true
    }

    pub fn is_being_deleted(&self) -> bool {
        self.metadata.deletion_timestamp.is_some()
            || self.status.state == Some(GameServerState::Shutdown)
    }

    pub fn is_before_ready(&self) -> bool {
        matches!(
            self.status.state.as_ref(),
            Some(
                GameServerState::PortAllocation
                    | GameServerState::Creating
                    | GameServerState::Starting
                    | GameServerState::Scheduled
                    | GameServerState::RequestReady
            )
        )
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.status.state.as_ref(),
            Some(
                GameServerState::Allocated
                    | GameServerState::Ready
                    | GameServerState::Reserved
            )
        )
    }

    pub fn apply_to_pod_container(
        &self,
        pod: &mut PodSpec,
        container_name: &str,
        mut apply: impl FnMut(&mut Container),
    ) -> Result<(), String> {
        if let Some(container) = pod
            .containers
            .iter_mut()
            .find(|container| container.name == container_name)
        {
            apply(container);
            return Ok(());
        }
        if let Some(container) = pod.init_containers.iter_mut().find(|container| {
            container.restart_policy == Some(ContainerRestartPolicy::Always)
                && container.name == container_name
        }) {
            apply(container);
            return Ok(());
        }
        Err(format!(
            "failed to find container named {container_name} in pod spec"
        ))
    }

    pub fn update_count(&mut self, name: &str, action: &str, amount: i64) -> Result<(), String> {
        if action != GAME_SERVER_PRIORITY_INCREMENT && action != GAME_SERVER_PRIORITY_DECREMENT {
            return Err(format!(
                "unable to UpdateCount with Name {name}, Action {action}, Amount {amount}. Allocation action must be one of {GAME_SERVER_PRIORITY_INCREMENT} or {GAME_SERVER_PRIORITY_DECREMENT}"
            ));
        }
        if amount < 0 {
            return Err(format!(
                "unable to UpdateCount with Name {name}, Action {action}, Amount {amount}. Amount must be greater than 0"
            ));
        }
        let Some(counters) = self.status.counters.as_mut() else {
            return Err(format!(
                "unable to UpdateCount with Name {name}, Action {action}, Amount {amount}. Counter not found in GameServer {}",
                self.metadata.name
            ));
        };
        let Some(counter) = counters.get_mut(name) else {
            return Err(format!(
                "unable to UpdateCount with Name {name}, Action {action}, Amount {amount}. Counter not found in GameServer {}",
                self.metadata.name
            ));
        };

        let mut count = if action == GAME_SERVER_PRIORITY_INCREMENT {
            counter.count.wrapping_add(amount)
        } else {
            counter.count.wrapping_sub(amount)
        };
        if count > counter.capacity {
            count = counter.capacity;
        }
        if count < 0 {
            count = 0;
        }
        counter.count = count;
        Ok(())
    }

    pub fn update_counter_capacity(&mut self, name: &str, capacity: i64) -> Result<(), String> {
        if capacity < 0 {
            return Err(format!(
                "unable to UpdateCounterCapacity: Name {name}, Capacity {capacity}. Capacity must be greater than or equal to 0"
            ));
        }
        let Some(counters) = self.status.counters.as_mut() else {
            return Err(format!(
                "unable to UpdateCounterCapacity: Name {name}, Capacity {capacity}. Counter not found in GameServer {}",
                self.metadata.name
            ));
        };
        let Some(counter) = counters.get_mut(name) else {
            return Err(format!(
                "unable to UpdateCounterCapacity: Name {name}, Capacity {capacity}. Counter not found in GameServer {}",
                self.metadata.name
            ));
        };
        counter.capacity = capacity;
        if counter.count > counter.capacity {
            counter.count = counter.capacity;
        }
        Ok(())
    }

    pub fn update_list_capacity(
        &mut self,
        name: &str,
        capacity: i64,
        max_capacity: i64,
    ) -> Result<(), String> {
        if capacity < 0 || capacity > max_capacity {
            return Err(format!(
                "unable to UpdateListCapacity: Name {name}, Capacity {capacity}. Capacity must be between 0 and {max_capacity}, inclusive"
            ));
        }
        let Some(lists) = self.status.lists.as_mut() else {
            return Err(format!(
                "unable to UpdateListCapacity: Name {name}, Capacity {capacity}. List not found in GameServer {}",
                self.metadata.name
            ));
        };
        let Some(list) = lists.get_mut(name) else {
            return Err(format!(
                "unable to UpdateListCapacity: Name {name}, Capacity {capacity}. List not found in GameServer {}",
                self.metadata.name
            ));
        };
        list.capacity = capacity;
        truncate_list(list.capacity, &mut list.values);
        Ok(())
    }

    pub fn append_list_values(
        &mut self,
        name: &str,
        values: Option<&[String]>,
    ) -> Result<(), String> {
        let Some(values) = values else {
            return Err(format!(
                "unable to AppendListValues: Name {name}. Values must not be nil"
            ));
        };
        let Some(lists) = self.status.lists.as_mut() else {
            return Err(format!(
                "unable to AppendListValues: Name {name}. List not found in GameServer {}",
                self.metadata.name
            ));
        };
        let Some(list) = lists.get_mut(name) else {
            return Err(format!(
                "unable to AppendListValues: Name {name}. List not found in GameServer {}",
                self.metadata.name
            ));
        };
        list.values = merge_remove_duplicates(&list.values, values).into();
        truncate_list(list.capacity, &mut list.values);
        Ok(())
    }

    pub fn delete_list_values(
        &mut self,
        name: &str,
        values: Option<&[String]>,
    ) -> Result<(), String> {
        let Some(values) = values else {
            return Err(format!(
                "unable to DeleteListValues: Name {name}. Values must not be nil"
            ));
        };
        let Some(lists) = self.status.lists.as_mut() else {
            return Err(format!(
                "unable to DeleteListValues: Name {name}. List not found in GameServer {}",
                self.metadata.name
            ));
        };
        let Some(list) = lists.get_mut(name) else {
            return Err(format!(
                "unable to DeleteListValues: Name {name}. List not found in GameServer {}",
                self.metadata.name
            ));
        };
        let deletes: BTreeSet<&str> = values.iter().map(String::as_str).collect();
        list.values.retain(|value| !deletes.contains(value.as_str()));
        Ok(())
    }
}


impl GameServer {
    pub fn compare_count_and_list_priorities(
        &self,
        priorities: &[Priority],
        other: &GameServer,
    ) -> Option<bool> {
        for priority in priorities {
            if let Some(result) = self.compare_count_and_list_priority(priority, other) {
                return Some(if priority.order == GAME_SERVER_PRIORITY_DESCENDING {
                    !result
                } else {
                    result
                });
            }
        }
        None
    }

    fn compare_count_and_list_priority(
        &self,
        priority: &Priority,
        other: &GameServer,
    ) -> Option<bool> {
        let (self_present, other_present, value_comparison) =
            match priority.priority_type.as_str() {
                GAME_SERVER_PRIORITY_COUNTER => {
                    let left = self
                        .status
                        .counters
                        .as_ref()
                        .and_then(|counters| counters.get(&priority.key));
                    let right = other
                        .status
                        .counters
                        .as_ref()
                        .and_then(|counters| counters.get(&priority.key));
                    let comparison = left.zip(right).map(|(left, right)| {
                        let left_available = left.capacity.wrapping_sub(left.count);
                        let right_available = right.capacity.wrapping_sub(right.count);
                        left_available.cmp(&right_available)
                    });
                    (left.is_some(), right.is_some(), comparison)
                }
                GAME_SERVER_PRIORITY_LIST => {
                    let left = self
                        .status
                        .lists
                        .as_ref()
                        .and_then(|lists| lists.get(&priority.key));
                    let right = other
                        .status
                        .lists
                        .as_ref()
                        .and_then(|lists| lists.get(&priority.key));
                    let comparison = left.zip(right).map(|(left, right)| {
                        let left_available =
                            left.capacity.wrapping_sub(left.values.len() as i64);
                        let right_available =
                            right.capacity.wrapping_sub(right.values.len() as i64);
                        left_available.cmp(&right_available)
                    });
                    (left.is_some(), right.is_some(), comparison)
                }
                _ => (false, false, None),
            };

        if let Some(ordering) = value_comparison {
            return match ordering {
                std::cmp::Ordering::Less => Some(true),
                std::cmp::Ordering::Greater => Some(false),
                std::cmp::Ordering::Equal => None,
            };
        }

        if (self_present && priority.order == GAME_SERVER_PRIORITY_DESCENDING)
            || (other_present && priority.order == GAME_SERVER_PRIORITY_ASCENDING)
        {
            return Some(false);
        }
        if (self_present && priority.order == GAME_SERVER_PRIORITY_ASCENDING)
            || (other_present && priority.order == GAME_SERVER_PRIORITY_DESCENDING)
        {
            return Some(true);
        }
        None
    }
}

fn truncate_list(capacity: i64, values: &mut crate::GoSlice<String>) {
    let capacity = capacity.max(0) as usize;
    if values.len() > capacity {
        values.truncate(capacity);
    }
}

pub fn merge_remove_duplicates(list1: &[String], list2: &[String]) -> Vec<String> {
    let mut unique = Vec::with_capacity(list1.len() + list2.len());
    let mut seen = BTreeSet::new();
    for value in list1.iter().chain(list2) {
        if seen.insert(value.as_str()) {
            unique.push(value.clone());
        }
    }
    unique
}


#[agones_invariant]
fn counter_updates_preserve_bounds(input: (i64, i64, i64, bool)) -> bool {
    let (capacity, count, amount, increment) = input;
    if capacity < 0 || count < 0 || count > capacity || amount < 0 {
        return true;
    }

    let arithmetic_is_defined = if increment {
        count.checked_add(amount).is_some()
    } else {
        count.checked_sub(amount).is_some()
    };
    if !arithmetic_is_defined {
        return true;
    }

    let mut game_server = GameServer {
        status: GameServerStatus {
            counters: Some(BTreeMap::from([(
                "c".into(),
                CounterStatus { count, capacity },
            )])),
            ..Default::default()
        },
        ..Default::default()
    };

    let action = if increment {
        GAME_SERVER_PRIORITY_INCREMENT
    } else {
        GAME_SERVER_PRIORITY_DECREMENT
    };
    if game_server.update_count("c", action, amount).is_err() {
        return false;
    }

    let result = game_server.status.counters.as_ref().unwrap()["c"].count;
    result >= 0 && result <= capacity
}

#[cfg(test)]
mod tests {

    #[test]
    fn creating_static_tcpudp_port_splits_like_upstream_controller() {
        let mut game_server = GameServer {
            spec: GameServerSpec {
                ports: vec![GameServerPort {
                    name: "default".into(),
                    range: "ignored-on-udp-copy".into(),
                    port_policy: Some(PortPolicy::Static),
                    container: Some("game".into()),
                    container_port: 7777,
                    host_port: 7000,
                    protocol: Some(Protocol::TcpUdp),
                }].into(),
                ..Default::default()
            },
            status: GameServerStatus {
                state: Some(GameServerState::Creating),
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(game_server.split_static_tcp_udp_ports());
        assert_eq!(game_server.spec.ports.len(), 2);

        let tcp = &game_server.spec.ports[0];
        assert_eq!(tcp.name, "default-tcp");
        assert_eq!(tcp.protocol, Some(Protocol::Tcp));
        assert_eq!(tcp.range, "ignored-on-udp-copy");

        let udp = &game_server.spec.ports[1];
        assert_eq!(udp.name, "default-udp");
        assert_eq!(udp.protocol, Some(Protocol::Udp));
        assert_eq!(udp.port_policy, Some(PortPolicy::Static));
        assert_eq!(udp.container.as_deref(), Some("game"));
        assert_eq!(udp.container_port, 7777);
        assert_eq!(udp.host_port, 7000);
        assert!(
            udp.range.is_empty(),
            "upstream appended GameServerPort does not copy Range"
        );
    }

    #[test]
    fn creating_tcpudp_split_is_idempotent() {
        let mut game_server = GameServer {
            spec: GameServerSpec {
                ports: vec![GameServerPort {
                    name: "default".into(),
                    port_policy: Some(PortPolicy::Static),
                    container_port: 7777,
                    host_port: 7000,
                    protocol: Some(Protocol::TcpUdp),
                    ..Default::default()
                }].into(),
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(game_server.split_static_tcp_udp_ports());
        let once = game_server.clone();
        assert!(!game_server.split_static_tcp_udp_ports());
        assert_eq!(game_server, once);
    }

    #[test]
    fn creating_port_normalization_ignores_tcp_and_nonstatic_tcpudp() {
        let mut game_server = GameServer {
            spec: GameServerSpec {
                ports: vec![
                    GameServerPort {
                        name: "tcp".into(),
                        port_policy: Some(PortPolicy::Static),
                        protocol: Some(Protocol::Tcp),
                        ..Default::default()
                    },
                    GameServerPort {
                        name: "dynamic".into(),
                        port_policy: Some(PortPolicy::Dynamic),
                        protocol: Some(Protocol::TcpUdp),
                        ..Default::default()
                    },
                ].into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let before = game_server.clone();

        assert!(!game_server.split_static_tcp_udp_ports());
        assert_eq!(game_server, before);
    }


    fn development_game_server(state: GameServerState) -> GameServer {
        let mut game_server = GameServer {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                annotations: BTreeMap::from([(
                    DEV_ADDRESS_ANNOTATION.to_owned(),
                    "203.0.113.7".to_owned(),
                )]),
                ..Default::default()
            },
            spec: GameServerSpec {
                ports: vec![GameServerPort {
                    name: "game".into(),
                    port_policy: Some(PortPolicy::Static),
                    container_port: 7777,
                    host_port: 7777,
                    protocol: Some(Protocol::Udp),
                    ..Default::default()
                }].into(),
                ..Default::default()
            },
            status: GameServerStatus {
                state: Some(state),
                ..Default::default()
            },
        };
        game_server.spec.apply_defaults();
        game_server
    }

    #[test]
    fn development_gameserver_creating_matches_controller_transition() {
        let mut game_server = development_game_server(GameServerState::Creating);

        assert!(game_server.reconcile_development_status());
        assert_eq!(game_server.status.state, Some(GameServerState::Ready));
        assert_eq!(
            game_server.status.ports,
            vec![GameServerStatusPort {
                name: "game".into(),
                port: 7777,
            }]
        );
        assert_eq!(game_server.status.address, "203.0.113.7");
        assert_eq!(
            game_server.status.addresses,
            vec![NodeAddress {
                address: "203.0.113.7".into(),
                address_type: "InternalIP".into(),
            }]
        );
        assert_eq!(game_server.status.node_name, "203.0.113.7");
    }

    #[test]
    fn development_gameserver_request_ready_only_advances_state() {
        let mut game_server = development_game_server(GameServerState::RequestReady);
        game_server.status.address = "preserve-address".into();
        game_server.status.node_name = "preserve-node".into();
        game_server.status.ports = vec![GameServerStatusPort {
            name: "preserve".into(),
            port: 9999,
        }].into();

        assert!(game_server.reconcile_development_status());
        assert_eq!(game_server.status.state, Some(GameServerState::Ready));
        assert_eq!(game_server.status.address, "preserve-address");
        assert_eq!(game_server.status.node_name, "preserve-node");
        assert_eq!(
            game_server.status.ports,
            vec![GameServerStatusPort {
                name: "preserve".into(),
                port: 9999,
            }]
        );
    }

    #[test]
    fn development_gameserver_allocated_is_noop() {
        let mut game_server = development_game_server(GameServerState::Allocated);
        let before = game_server.clone();

        assert!(!game_server.reconcile_development_status());
        assert_eq!(game_server, before);
    }

    #[test]
    fn deleting_development_gameserver_is_noop() {
        let mut game_server = development_game_server(GameServerState::Creating);
        game_server.metadata.deletion_timestamp = Some("2026-10-17T03:40:00Z".into());
        let before = game_server.clone();

        assert!(!game_server.reconcile_development_status());
        assert_eq!(game_server, before);
    }

    #[test]
    fn non_development_gameserver_is_noop() {
        let mut game_server = development_game_server(GameServerState::Creating);
        game_server.metadata.annotations.remove(DEV_ADDRESS_ANNOTATION);
        let before = game_server.clone();

        assert!(!game_server.reconcile_development_status());
        assert_eq!(game_server, before);
    }


    #[test]
    fn gameserver_status_network_fields_match_original_json_shape() {
        let status = GameServerStatus {
            state: Some(GameServerState::Ready),
            ports: vec![GameServerStatusPort {
                name: "game".into(),
                port: 7777,
            }].into(),
            address: "203.0.113.10".into(),
            addresses: vec![
                NodeAddress {
                    address: "10.0.0.10".into(),
                    address_type: "InternalIP".into(),
                },
                NodeAddress {
                    address: "203.0.113.10".into(),
                    address_type: "ExternalIP".into(),
                },
            ].into(),
            node_name: "node-a".into(),
            reserved_until: Some("2026-10-17T03:30:00Z".into()),
            ..Default::default()
        };

        let json = serde_json::to_value(&status).unwrap();
        assert_eq!(json["address"], "203.0.113.10");
        assert_eq!(json["addresses"][0]["address"], "10.0.0.10");
        assert_eq!(json["addresses"][0]["type"], "InternalIP");
        assert_eq!(json["addresses"][1]["type"], "ExternalIP");
        assert_eq!(json["nodeName"], "node-a");
        assert_eq!(json["reservedUntil"], "2026-10-17T03:30:00Z");

        let round_tripped: GameServerStatus = serde_json::from_value(json).unwrap();
        assert_eq!(round_tripped, status);
    }

    #[test]
    fn gameserver_status_network_fields_default_without_affecting_existing_status_logic() {
        let status: GameServerStatus = serde_json::from_value(serde_json::json!({
            "state": "Creating",
            "ports": []
        }))
        .unwrap();

        assert_eq!(status.state, Some(GameServerState::Creating));
        assert!(status.address.is_empty());
        assert!(status.addresses.is_empty());
        assert!(status.node_name.is_empty());
        assert!(status.reserved_until.is_none());
    }

    use super::*;

    #[test]
    fn indexed_validation_fields_preserve_decimal_paths() {
        for index in (0..1024).chain([usize::MAX / 10, usize::MAX - 1, usize::MAX]) {
            for field in ["hostPort", "portPolicy", "containerPort", "container"] {
                assert_eq!(
                    indexed_validation_field("spec.ports", index, field),
                    format!("spec.ports[{index}].{field}")
                );
            }
        }
    }

    fn simple_game_server() -> GameServer {
        GameServer {
            spec: GameServerSpec {
                ports: vec![GameServerPort {
                    container_port: 999,
                    ..Default::default()
                }].into(),
                template: PodTemplateSpec {
                    spec: PodSpec {
                        containers: vec![Container {
                            name: "testing".into(),
                            image: "testing/image".into(),
                            ..Default::default()
                        }].into(),
                        ..Default::default()
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
                }].into(),
                health: Health {
                    disabled: false,
                    period_seconds: 12,
                    failure_threshold: 10,
                    initial_delay_seconds: 11,
                },
                template: PodTemplateSpec {
                    spec: PodSpec {
                        containers: vec![
                            Container {
                                name: "testing".into(),
                                image: "testing/image".into(),
                                ..Default::default()
                            },
                            Container {
                                name: "testing2".into(),
                                image: "testing/image2".into(),
                                ..Default::default()
                            },
                        ].into(),
                        ..Default::default()
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
            ListStatus { capacity: 100, values: vec!["foo".into(), "bar".into()].into() },
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
    fn pod_template_preserves_unmodeled_fields() {
        let json = serde_json::json!({
            "metadata": {
                "annotations": {"example.dev/a": "b"},
                "ownerReferences": [{
                    "apiVersion": "v1",
                    "kind": "ConfigMap",
                    "name": "owner",
                    "uid": "123"
                }]
            },
            "spec": {
                "containers": [{
                    "name": "game",
                    "image": "game:v1",
                    "command": ["/game"],
                    "env": [{"name": "MODE", "value": "prod"}]
                }],
                "nodeSelector": {"accelerator": "gpu"},
                "terminationGracePeriodSeconds": 30
            }
        });
        let template: PodTemplateSpec = serde_json::from_value(json.clone()).unwrap();
        let encoded = serde_json::to_value(template).unwrap();
        // Source Go Container JSON materializes zero resources while retaining
        // the unrelated Pod/metadata fields exercised by this fixture.
        let mut expected = json;
        expected["spec"]["containers"][0]["resources"] = serde_json::json!({});
        assert_eq!(encoded, expected);
    }

    #[test]
    fn patch_matches_original_resource_version_and_replace_invariants() {
        let fixture = GameServer {
            metadata: ObjectMeta {
                name: "lucy".into(),
                resource_version: Some("1234".into()),
                ..Default::default()
            },
            spec: GameServerSpec {
                container: "goat".into(),
                ..Default::default()
            },
            ..Default::default()
        };

        let mut delta = fixture.clone();
        delta.spec.container = "bear".into();

        let patch = fixture.patch(&delta).unwrap();
        let patch = String::from_utf8(patch).unwrap();
        assert!(patch.contains(
            r#"{"op":"test","path":"/metadata/resourceVersion","value":"1234"}"#
        ));
        assert!(patch.contains(
            r#"{"op":"replace","path":"/spec/container","value":"bear"}"#
        ));
        assert!(patch.find(r#""op":"test""#).unwrap() < patch.find(r#""op":"replace""#).unwrap());
    }

    #[test]
    fn patch_escapes_json_pointer_segments_and_handles_add_remove() {
        let mut fixture = GameServer::default();
        fixture
            .metadata
            .annotations
            .insert("a/b~c".into(), "old".into());

        let mut delta = fixture.clone();
        delta.metadata.annotations.remove("a/b~c");
        delta
            .metadata
            .annotations
            .insert("new/key".into(), "value".into());

        let patch: serde_json::Value =
            serde_json::from_slice(&fixture.patch(&delta).unwrap()).unwrap();
        let operations = patch.as_array().unwrap();
        assert!(operations.iter().any(|operation| {
            operation["op"] == "remove"
                && operation["path"] == "/metadata/annotations/a~1b~0c"
        }));
        assert!(operations.iter().any(|operation| {
            operation["op"] == "add"
                && operation["path"] == "/metadata/annotations/new~1key"
                && operation["value"] == "value"
        }));
    }

    #[test]
    fn validation_ports_containers_and_dev_address_match_go_cases() {
        let spec = GameServerSpec {
            container: String::new(),
            ports: vec![
                GameServerPort {
                    name: "main".into(),
                    host_port: 5001,
                    port_policy: Some(PortPolicy::Dynamic),
                    ..Default::default()
                },
                GameServerPort {
                    name: "sidecar".into(),
                    host_port: 5002,
                    container_port: 5002,
                    port_policy: Some(PortPolicy::Static),
                    ..Default::default()
                },
            ].into(),
            template: PodTemplateSpec {
                spec: PodSpec {
                    containers: vec![
                        Container { name: "testing".into(), ..Default::default() },
                        Container { name: "anothertest".into(), ..Default::default() },
                    ].into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let errors = spec.validate("");
        assert_eq!(errors.len(), 4);
        assert_eq!(errors[0].kind, ValidationErrorKind::Required);
        assert_eq!(errors[0].path, "spec.container");
        assert_eq!(errors[1].kind, ValidationErrorKind::Invalid);
        assert_eq!(errors[2].path, "spec.ports[0].containerPort");
        assert_eq!(errors[3].path, "spec.ports[0].hostPort");

        for (name, expected_detail) in [
            ("", "Could not find a container named "),
            ("missing", "Could not find a container named missing"),
            ("🦀\n\0", "Could not find a container named 🦀\n\0"),
        ] {
            let errors = GameServerSpec {
                container: name.to_owned(),
                ..Default::default()
            }.validate("");
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].kind, ValidationErrorKind::Invalid);
            assert_eq!(errors[0].path, "spec.container");
            assert_eq!(errors[0].bad_value.as_deref(), Some(name));
            assert_eq!(errors[0].detail, expected_detail);
        }

        let dev = GameServerSpec {
            ports: vec![GameServerPort {
                name: "main".into(),
                container_port: 7777,
                port_policy: Some(PortPolicy::Static),
                ..Default::default()
            }].into(),
            ..Default::default()
        };
        let errors = dev.validate("invalid-ip");
        assert_eq!(errors.len(), 2);
        assert_eq!(
            errors[0].path,
            "metadata.annotations.agones.dev/dev-address"
        );
        assert_eq!(errors[1].path, "spec.ports[0].hostPort");
    }

    #[test]
    fn validation_enforces_passthrough_dynamic_and_dev_policy_rules() {
        let mut gs = GameServer {
            spec: GameServerSpec {
                container: "my_image".into(),
                ports: vec![
                    GameServerPort {
                        name: "one".into(),
                        port_policy: Some(PortPolicy::Passthrough),
                        container_port: 1294,
                        ..Default::default()
                    },
                    GameServerPort {
                        name: "two".into(),
                        port_policy: Some(PortPolicy::Passthrough),
                        host_port: 7890,
                        ..Default::default()
                    },
                    GameServerPort {
                        name: "three".into(),
                        port_policy: Some(PortPolicy::Dynamic),
                        host_port: 7890,
                        container_port: 1294,
                        ..Default::default()
                    },
                ].into(),
                template: PodTemplateSpec {
                    spec: PodSpec {
                        containers: vec![Container {
                            name: "my_image".into(),
                            ..Default::default()
                        }].into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gs.spec.apply_defaults();
        let errors = gs.spec.validate("");
        assert_eq!(errors.len(), 3);
        assert_eq!(errors[0].detail, ERR_CONTAINER_PORT_PASSTHROUGH);
        assert_eq!(errors[1].detail, ERR_HOST_PORT);
        assert_eq!(errors[2].detail, ERR_HOST_PORT);

        gs.metadata
            .annotations
            .insert(DEV_ADDRESS_ANNOTATION.into(), "127.1.1.1".into());
        gs.spec.ports = vec![GameServerPort {
            name: "main".into(),
            port_policy: Some(PortPolicy::Passthrough),
            host_port: 7890,
            container_port: 7777,
            ..Default::default()
        }].into();
        let errors = gs.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.ports[0].portPolicy");
        assert_eq!(errors[0].detail, ERR_PORT_POLICY_STATIC);
    }

    #[test]
    fn validation_checks_name_metadata_and_sidecar_container_semantics() {
        let mut gs = simple_game_server();
        gs.apply_defaults();
        gs.metadata.name = "f".repeat(64);
        let errors = gs.validate();
        assert!(errors.iter().any(|e| {
            e.kind == ValidationErrorKind::TooLong && e.path == "metadata.name"
        }));

        gs.metadata.name.clear();
        gs.spec
            .template
            .metadata
            .annotations
            .insert("agones.dev/short±Name".into(), "value".into());
        let errors = gs.validate();
        assert!(errors
            .iter()
            .any(|e| e.path == "spec.template.metadata.annotations"));

        gs.spec.template.metadata.annotations.clear();
        gs.spec.template.spec.init_containers.push(Container {
            name: "sdk".into(),
            restart_policy: Some(ContainerRestartPolicy::Always),
            ..Default::default()
        });
        assert!(gs.spec.has_container("sdk", true));
        assert!(!gs.spec.has_container("sdk", false));
    }

    #[test]
    fn validation_error_constructor_payload_and_metadata_match_source() {
        for (maximum, detail) in [
            (-1, "value is too long"),
            (0, "may not be more than 0 bytes"),
            (1, "may not be more than 1 byte"),
            (63, "may not be more than 63 bytes"),
        ] {
            let error = ValidationError::too_long("metadata.name", "private-value", maximum);
            assert_eq!(error, ValidationError {
                kind: ValidationErrorKind::TooLong,
                path: "metadata.name".into(),
                bad_value: Some("<value omitted>".into()),
                detail: detail.into(),
                origin: String::new(),
                covered_by_declarative: false,
            });
        }
        let error = ValidationError::invalid("spec.field", "value", "detail");
        let marked = error.clone().with_origin("format=example").mark_covered_by_declarative();
        assert_eq!(marked.kind, error.kind);
        assert_eq!(marked.path, error.path);
        assert_eq!(marked.bad_value, error.bad_value);
        assert_eq!(marked.detail, error.detail);
        assert_eq!(marked.origin, "format=example");
        assert!(marked.covered_by_declarative);
        assert_ne!(marked, error);
        let other_origin = marked.clone().with_origin("format=other");
        assert!(other_origin.covered_by_declarative);
        assert_ne!(marked, other_origin);
    }

    #[test]
    fn production_validation_retains_all_source_error_fields() {
        let mut gs = simple_game_server();
        gs.apply_defaults();
        gs.metadata.name = "n".repeat(64);
        let errors = gs.validate();
        assert_eq!(errors, vec![ValidationError {
            kind: ValidationErrorKind::TooLong,
            path: "metadata.name".into(),
            bad_value: Some("<value omitted>".into()),
            detail: "may not be more than 63 bytes".into(),
            origin: String::new(),
            covered_by_declarative: false,
        }]);
        gs.metadata.name.clear();
        gs.spec.template.metadata.annotations.insert("agones.dev/short±Name".into(), "value".into());
        let errors = gs.validate();
        assert_eq!(errors, vec![ValidationError {
            kind: ValidationErrorKind::Invalid,
            path: "spec.template.metadata.annotations".into(),
            bad_value: Some("agones.dev/short±Name".into()),
            detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".into(),
            origin: "format=k8s-label-key".into(),
            covered_by_declarative: false,
        }]);
        gs.spec.template.metadata.annotations.clear();
        gs.spec.template.metadata.labels.insert("label".into(), "v".repeat(64));
        assert_eq!(gs.validate(), vec![ValidationError {
            kind: ValidationErrorKind::Invalid,
            path: "spec.template.metadata.labels".into(),
            bad_value: Some("v".repeat(64)),
            detail: "must be no more than 63 bytes".into(),
            origin: "format=k8s-label-value".into(),
            covered_by_declarative: false,
        }]);
    }

    #[test]
    fn lifecycle_helpers_match_go_state_tables() {
        let before_ready = [
            GameServerState::PortAllocation,
            GameServerState::Creating,
            GameServerState::Starting,
            GameServerState::Scheduled,
            GameServerState::RequestReady,
        ];
        for state in before_ready {
            let gs = GameServer {
                status: GameServerStatus {
                    state: Some(state),
                    ..Default::default()
                },
                ..Default::default()
            };
            assert!(gs.is_before_ready());
        }

        for state in [
            GameServerState::Ready,
            GameServerState::Shutdown,
            GameServerState::Error,
            GameServerState::Unhealthy,
            GameServerState::Reserved,
            GameServerState::Allocated,
        ] {
            let gs = GameServer {
                status: GameServerStatus {
                    state: Some(state),
                    ..Default::default()
                },
                ..Default::default()
            };
            assert!(!gs.is_before_ready());
        }

        for state in [
            GameServerState::Ready,
            GameServerState::Reserved,
            GameServerState::Allocated,
        ] {
            let gs = GameServer {
                status: GameServerStatus {
                    state: Some(state),
                    ..Default::default()
                },
                ..Default::default()
            };
            assert!(gs.is_active());
        }
    }

    #[test]
    fn deletion_helpers_match_allocated_reserved_behavior() {
        let mut gs = GameServer {
            status: GameServerStatus {
                state: Some(GameServerState::Starting),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(gs.is_deletable());
        gs.status.state = Some(GameServerState::Allocated);
        assert!(!gs.is_deletable());
        gs.status.state = Some(GameServerState::Reserved);
        assert!(!gs.is_deletable());

        gs.metadata.deletion_timestamp = Some("now".into());
        assert!(gs.is_deletable());
        assert!(gs.is_being_deleted());

        gs.metadata.deletion_timestamp = None;
        gs.status.state = Some(GameServerState::Shutdown);
        assert!(gs.is_being_deleted());
    }

    #[test]
    fn apply_to_pod_container_supports_regular_and_restartable_init_containers() {
        let gs = simple_game_server();
        let mut pod = PodSpec {
            containers: vec![
                Container { name: "game".into(), ..Default::default() },
                Container { name: "other".into(), ..Default::default() },
            ].into(),
            init_containers: vec![
                Container {
                    name: "sdk".into(),
                    restart_policy: Some(ContainerRestartPolicy::Always),
                    ..Default::default()
                },
                Container {
                    name: "init".into(),
                    restart_policy: Some(ContainerRestartPolicy::Never),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        gs.apply_to_pod_container(&mut pod, "game", |c| c.tty = true)
            .unwrap();
        gs.apply_to_pod_container(&mut pod, "sdk", |c| c.tty = true)
            .unwrap();
        assert!(pod.containers[0].tty);
        assert!(pod.init_containers[0].tty);
        assert!(gs
            .apply_to_pod_container(&mut pod, "init", |c| c.tty = true)
            .is_err());
    }

    #[test]
    fn counter_updates_match_go_truncation_and_error_behavior() {
        let mut gs = GameServer {
            status: GameServerStatus {
                counters: Some(BTreeMap::from([
                    ("players".into(), CounterStatus { count: 0, capacity: 100 }),
                    ("bars".into(), CounterStatus { count: 99, capacity: 100 }),
                ])),
                ..Default::default()
            },
            ..Default::default()
        };

        gs.update_count("players", GAME_SERVER_PRIORITY_INCREMENT, 101)
            .unwrap();
        assert_eq!(gs.status.counters.as_ref().unwrap()["players"].count, 100);

        gs.update_count("bars", GAME_SERVER_PRIORITY_DECREMENT, 100)
            .unwrap();
        assert_eq!(gs.status.counters.as_ref().unwrap()["bars"].count, 0);

        assert!(gs.update_count("bars", "decrement", 1).is_err());
        assert!(gs
            .update_count("bars", GAME_SERVER_PRIORITY_INCREMENT, -1)
            .is_err());

        gs.update_counter_capacity("players", 50).unwrap();
        assert_eq!(
            gs.status.counters.as_ref().unwrap()["players"],
            CounterStatus { count: 50, capacity: 50 }
        );
    }

    #[test]
    fn list_updates_match_go_order_deduplication_and_capacity_behavior() {
        let mut gs = GameServer {
            status: GameServerStatus {
                lists: Some(BTreeMap::from([(
                    "games".into(),
                    ListStatus {
                        capacity: 3,
                        values: vec!["game0".into()].into(),
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };

        let values = vec![
            "game1".into(),
            "game2".into(),
            "game2".into(),
            "game1".into(),
            "game3".into(),
        ];
        gs.append_list_values("games", Some(&values)).unwrap();
        assert_eq!(
            gs.status.lists.as_ref().unwrap()["games"].values,
            vec!["game0", "game1", "game2"]
        );

        let deletes = vec!["game1".into(), "missing".into()];
        gs.delete_list_values("games", Some(&deletes)).unwrap();
        assert_eq!(
            gs.status.lists.as_ref().unwrap()["games"].values,
            vec!["game0", "game2"]
        );

        gs.update_list_capacity("games", 1, 25).unwrap();
        assert_eq!(gs.status.lists.as_ref().unwrap()["games"].capacity, 1);
        assert_eq!(
            gs.status.lists.as_ref().unwrap()["games"].values,
            vec!["game0"]
        );
        assert!(gs.update_list_capacity("games", 26, 25).is_err());
        assert!(gs.append_list_values("games", None).is_err());
        assert!(gs.delete_list_values("games", None).is_err());
    }

    #[test]
    fn merge_remove_duplicates_matches_original_ordering() {
        let one = vec!["one".into(), "two".into()];
        let two = vec!["two".into(), "one".into(), "three".into()];
        assert_eq!(
            merge_remove_duplicates(&one, &two),
            vec!["one", "two", "three"]
        );
    }


    #[test]
    fn count_and_list_priorities_match_agones_ordering() {
        let make_counter = |count, capacity| GameServer {
            status: GameServerStatus {
                counters: Some(BTreeMap::from([(
                    "rooms".into(),
                    CounterStatus { count, capacity },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };

        let low_available = make_counter(9, 10);
        let high_available = make_counter(2, 10);
        let ascending = [Priority {
            priority_type: GAME_SERVER_PRIORITY_COUNTER.into(),
            key: "rooms".into(),
            order: GAME_SERVER_PRIORITY_ASCENDING.into(),
        }];
        let descending = [Priority {
            order: GAME_SERVER_PRIORITY_DESCENDING.into(),
            ..ascending[0].clone()
        }];

        assert_eq!(
            low_available.compare_count_and_list_priorities(&ascending, &high_available),
            Some(true)
        );
        assert_eq!(
            low_available.compare_count_and_list_priorities(&descending, &high_available),
            Some(false)
        );

        let list_left = GameServer {
            status: GameServerStatus {
                lists: Some(BTreeMap::from([(
                    "maps".into(),
                    ListStatus {
                        capacity: 5,
                        values: vec!["a".into(), "b".into(), "c".into()].into(),
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        let list_right = GameServer {
            status: GameServerStatus {
                lists: Some(BTreeMap::from([(
                    "maps".into(),
                    ListStatus {
                        capacity: 5,
                        values: vec!["a".into()].into(),
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        let priority = [Priority {
            priority_type: GAME_SERVER_PRIORITY_LIST.into(),
            key: "maps".into(),
            order: GAME_SERVER_PRIORITY_ASCENDING.into(),
        }];
        assert_eq!(
            list_left.compare_count_and_list_priorities(&priority, &list_right),
            Some(true)
        );
    }

    #[test]
    fn priority_presence_beats_missing_priority_in_both_orders() {
        let with_counter = GameServer {
            status: GameServerStatus {
                counters: Some(BTreeMap::from([(
                    "players".into(),
                    CounterStatus {
                        count: 0,
                        capacity: 10,
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        let without_counter = GameServer::default();

        for order in [
            GAME_SERVER_PRIORITY_ASCENDING,
            GAME_SERVER_PRIORITY_DESCENDING,
        ] {
            let priorities = [Priority {
                priority_type: GAME_SERVER_PRIORITY_COUNTER.into(),
                key: "players".into(),
                order: order.into(),
            }];
            assert_eq!(
                with_counter
                    .compare_count_and_list_priorities(&priorities, &without_counter),
                Some(true)
            );
            assert_eq!(
                without_counter
                    .compare_count_and_list_priorities(&priorities, &with_counter),
                Some(false)
            );
        }
    }

    #[test]
    fn priority_ties_fall_through_to_next_priority() {
        let left = GameServer {
            status: GameServerStatus {
                counters: Some(BTreeMap::from([
                    ("tie".into(), CounterStatus { count: 1, capacity: 2 }),
                    ("break".into(), CounterStatus { count: 1, capacity: 2 }),
                ])),
                ..Default::default()
            },
            ..Default::default()
        };
        let right = GameServer {
            status: GameServerStatus {
                counters: Some(BTreeMap::from([
                    ("tie".into(), CounterStatus { count: 1, capacity: 2 }),
                    ("break".into(), CounterStatus { count: 0, capacity: 2 }),
                ])),
                ..Default::default()
            },
            ..Default::default()
        };
        let priorities = [
            Priority {
                priority_type: GAME_SERVER_PRIORITY_COUNTER.into(),
                key: "tie".into(),
                order: GAME_SERVER_PRIORITY_ASCENDING.into(),
            },
            Priority {
                priority_type: GAME_SERVER_PRIORITY_COUNTER.into(),
                key: "break".into(),
                order: GAME_SERVER_PRIORITY_ASCENDING.into(),
            },
        ];
        assert_eq!(
            left.compare_count_and_list_priorities(&priorities, &right),
            Some(true)
        );
    }

    #[test]
    fn safe_add_saturates_positive_overflow() {
        assert_eq!(safe_add(i64::MAX - 1, 1), i64::MAX);
        assert_eq!(safe_add(i64::MAX - 1, 2), i64::MAX);
        assert_eq!(safe_add(40, 2), 42);
    }

}

