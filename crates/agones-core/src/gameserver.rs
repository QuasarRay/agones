use std::collections::{BTreeMap, BTreeSet};
use std::net::IpAddr;

use agones_macros::AgonesStringEnum;
use serde::{Deserialize, Serialize};

pub const GROUP_NAME: &str = "agones.dev";
pub const DEFAULT_PORT_RANGE: &str = "default";
pub const VERSION_ANNOTATION: &str = "agones.dev/sdk-version";
pub const FINALIZER_NAME: &str = "agones.dev/controller";
pub const POD_SAFE_TO_EVICT_ANNOTATION: &str = "cluster-autoscaler.kubernetes.io/safe-to-evict";
pub const DEV_ADDRESS_ANNOTATION: &str = "agones.dev/dev-address";
pub const DEFAULT_SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

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
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub generate_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
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
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<ContainerRestartPolicy>,
    #[serde(default)]
    pub tty: bool,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    #[serde(default)]
    pub containers: Vec<Container>,
    #[serde(default)]
    pub init_containers: Vec<Container>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: PodSpec,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationErrorKind {
    Required,
    Forbidden,
    Invalid,
    TooLong,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationError {
    pub kind: ValidationErrorKind,
    pub path: String,
    pub bad_value: Option<String>,
    pub detail: String,
}

impl ValidationError {
    fn required(path: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            kind: ValidationErrorKind::Required,
            path: path.into(),
            bad_value: None,
            detail: detail.into(),
        }
    }

    fn forbidden(path: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            kind: ValidationErrorKind::Forbidden,
            path: path.into(),
            bad_value: None,
            detail: detail.into(),
        }
    }

    fn invalid(
        path: impl Into<String>,
        bad_value: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind: ValidationErrorKind::Invalid,
            path: path.into(),
            bad_value: Some(bad_value.into()),
            detail: detail.into(),
        }
    }

    fn too_long(path: impl Into<String>, bad_value: impl Into<String>, max: usize) -> Self {
        Self {
            kind: ValidationErrorKind::TooLong,
            path: path.into(),
            bad_value: Some(bad_value.into()),
            detail: format!("must be no more than {max} bytes"),
        }
    }
}

fn qualified_name_error(value: &str) -> Option<&'static str> {
    let name = value.rsplit('/').next().unwrap_or(value);
    if name.len() > 63 {
        return Some("name part must be no more than 63 bytes");
    }
    if name.is_empty() {
        return Some(
            "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character",
        );
    }
    let mut chars = name.chars();
    let first = chars.next().unwrap();
    let last = name.chars().last().unwrap();
    if !first.is_ascii_alphanumeric()
        || !last.is_ascii_alphanumeric()
        || !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return Some(
            "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character",
        );
    }
    None
}

pub(crate) fn validate_object_meta(meta: &ObjectMeta, path: &str) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for (key, value) in &meta.labels {
        if let Some(detail) = qualified_name_error(key) {
            errors.push(ValidationError::invalid(
                format!("{path}.labels"),
                key.as_str(),
                detail,
            ));
        }
        if value.len() > 63 {
            errors.push(ValidationError::invalid(
                format!("{path}.labels"),
                value.as_str(),
                "must be no more than 63 bytes",
            ));
        }
    }
    for key in meta.annotations.keys() {
        if let Some(detail) = qualified_name_error(key) {
            errors.push(ValidationError::invalid(
                format!("{path}.annotations"),
                key.as_str(),
                detail,
            ));
        }
    }
    errors
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
    /// Resource quantity validation and API hook validation belong to the
    /// Rusternetes integration layer because they depend on runtime/API types.
    pub fn validate(&self, dev_address: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

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
                        format!("spec.ports[{index}].hostPort"),
                        DEV_ADDRESS_ANNOTATION,
                    ));
                }
                if port.port_policy != Some(PortPolicy::Static) {
                    errors.push(ValidationError::required(
                        format!("spec.ports[{index}].portPolicy"),
                        ERR_PORT_POLICY_STATIC,
                    ));
                }
            }
            errors.extend(validate_object_meta(
                &self.template.metadata,
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
            errors.push(ValidationError::invalid(
                "spec.container",
                self.container.as_str(),
                format!("Could not find a container named {}", self.container),
            ));
        }

        for (index, port) in self.ports.iter().enumerate() {
            let prefix = format!("spec.ports[{index}]");
            if matches!(
                port.port_policy.as_ref(),
                Some(PortPolicy::Dynamic | PortPolicy::Static)
            ) && port.container_port <= 0
            {
                errors.push(ValidationError::required(
                    format!("{prefix}.containerPort"),
                    ERR_CONTAINER_PORT_REQUIRED,
                ));
            }

            if port.port_policy == Some(PortPolicy::Passthrough) && port.container_port > 0 {
                errors.push(ValidationError::required(
                    format!("{prefix}.containerPort"),
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
                    format!("{prefix}.hostPort"),
                    ERR_HOST_PORT,
                ));
            }

            if let Some(container) = port.container.as_deref() {
                if !self.container.is_empty() && !self.has_container(container, true) {
                    errors.push(ValidationError::invalid(
                        format!("{prefix}.container"),
                        container,
                        ERR_CONTAINER_NAME_INVALID,
                    ));
                }
            }
        }

        errors.extend(validate_object_meta(
            &self.template.metadata,
            "spec.template.metadata",
        ));
        errors
    }
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


impl GameServer {
    pub fn get_dev_address(&self) -> Option<&str> {
        self.metadata
            .annotations
            .get(DEV_ADDRESS_ANNOTATION)
            .map(String::as_str)
    }

    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.metadata.name.len() > 63 {
            errors.push(ValidationError::too_long(
                "metadata.name",
                self.metadata.name.as_str(),
                63,
            ));
        }
        errors.extend(self.spec.validate(self.get_dev_address().unwrap_or("")));
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
        list.values = merge_remove_duplicates(&list.values, values);
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

fn truncate_list(capacity: i64, values: &mut Vec<String>) {
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
                            ..Default::default()
                        }],
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
                        ],
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
        assert_eq!(encoded, json);
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
            ],
            template: PodTemplateSpec {
                spec: PodSpec {
                    containers: vec![
                        Container { name: "testing".into(), ..Default::default() },
                        Container { name: "anothertest".into(), ..Default::default() },
                    ],
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

        let dev = GameServerSpec {
            ports: vec![GameServerPort {
                name: "main".into(),
                container_port: 7777,
                port_policy: Some(PortPolicy::Static),
                ..Default::default()
            }],
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
                ],
                template: PodTemplateSpec {
                    spec: PodSpec {
                        containers: vec![Container {
                            name: "my_image".into(),
                            ..Default::default()
                        }],
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
        }];
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
            ],
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
                        values: vec!["game0".into()],
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
                        values: vec!["a".into(), "b".into(), "c".into()],
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
                        values: vec!["a".into()],
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

    #[test]
    fn bolero_counter_updates_never_leave_count_outside_capacity() {
        bolero::check!().for_each(|bytes: &[u8]| {
            let capacity = i64::from(bytes.first().copied().unwrap_or(0));
            let count = i64::from(bytes.get(1).copied().unwrap_or(0)).min(capacity);
            let amount = i64::from(bytes.get(2).copied().unwrap_or(0));
            let mut gs = GameServer {
                status: GameServerStatus {
                    counters: Some(BTreeMap::from([(
                        "c".into(),
                        CounterStatus { count, capacity },
                    )])),
                    ..Default::default()
                },
                ..Default::default()
            };
            gs.update_count("c", GAME_SERVER_PRIORITY_INCREMENT, amount)
                .unwrap();
            let result = gs.status.counters.as_ref().unwrap()["c"].count;
            assert!(result >= 0);
            assert!(result <= capacity);
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


#[cfg(kani)]
mod tracking_verification {
    use super::*;

    #[kani::proof]
    fn safe_add_never_overflows_for_nonnegative_inputs() {
        let x: i64 = kani::any();
        let y: i64 = kani::any();
        kani::assume(x >= 0);
        kani::assume(y >= 0);
        let result = safe_add(x, y);
        assert!(result >= x);
        assert!(result <= i64::MAX);
        if x <= i64::MAX - y {
            assert_eq!(result, x + y);
        } else {
            assert_eq!(result, i64::MAX);
        }
    }

    #[kani::proof]
    fn counter_increment_is_bounded_by_nonnegative_capacity() {
        let capacity: i64 = kani::any();
        kani::assume(capacity >= 0);
        let count: i64 = kani::any();
        kani::assume(count >= 0 && count <= capacity);
        let amount: i64 = kani::any();
        kani::assume(amount >= 0);
        kani::assume(count.checked_add(amount).is_some());

        let mut gs = GameServer {
            status: GameServerStatus {
                counters: Some(BTreeMap::from([(
                    "c".into(),
                    CounterStatus { count, capacity },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        gs.update_count("c", GAME_SERVER_PRIORITY_INCREMENT, amount)
            .unwrap();
        let result = gs.status.counters.as_ref().unwrap()["c"].count;
        assert!(result >= 0);
        assert!(result <= capacity);
    }

    #[kani::proof]
    fn counter_decrement_is_bounded_by_zero_and_capacity() {
        let capacity: i64 = kani::any();
        kani::assume(capacity >= 0);
        let count: i64 = kani::any();
        kani::assume(count >= 0 && count <= capacity);
        let amount: i64 = kani::any();
        kani::assume(amount >= 0);
        kani::assume(count.checked_sub(amount).is_some());

        let mut gs = GameServer {
            status: GameServerStatus {
                counters: Some(BTreeMap::from([(
                    "c".into(),
                    CounterStatus { count, capacity },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        gs.update_count("c", GAME_SERVER_PRIORITY_DECREMENT, amount)
            .unwrap();
        let result = gs.status.counters.as_ref().unwrap()["c"].count;
        assert!(result >= 0);
        assert!(result <= capacity);
    }
}
