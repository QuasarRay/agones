use std::collections::BTreeMap;

use agones_macros::AgonesStringEnum;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{FleetStatus, IntOrPercent, ObjectMeta, ValidationError, ValidationErrorKind};

pub const DEFAULT_INTERVAL_SYNC_SECONDS: i32 = 30;

mod base64_bytes {
    use super::*;

    pub fn serialize<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&BASE64.encode(value))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        BASE64.decode(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum FleetAutoscalerPolicyType {
    #[agones(value = "Buffer")]
    Buffer,
    #[agones(value = "Webhook")]
    Webhook,
    #[agones(value = "Counter")]
    Counter,
    #[agones(value = "List")]
    List,
    #[agones(value = "Schedule")]
    Schedule,
    #[agones(value = "Chain")]
    Chain,
    #[agones(value = "Wasm")]
    Wasm,
    #[agones(unknown)]
    Unknown(String),
}

impl Default for FleetAutoscalerPolicyType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum FleetAutoscalerSyncType {
    #[agones(value = "FixedInterval")]
    FixedInterval,
    #[agones(unknown)]
    Unknown(String),
}

impl Default for FleetAutoscalerSyncType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferPolicy {
    pub max_replicas: i32,
    pub min_replicas: i32,
    pub buffer_size: IntOrPercent,
}

impl Default for BufferPolicy {
    fn default() -> Self {
        Self {
            max_replicas: 0,
            min_replicas: 0,
            buffer_size: IntOrPercent::Int(0),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReference {
    pub namespace: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "base64_bytes::serialize",
        deserialize_with = "base64_bytes::deserialize"
    )]
    pub ca_bundle: Vec<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterPolicy {
    pub key: String,
    pub max_capacity: i64,
    pub min_capacity: i64,
    pub buffer_size: IntOrPercent,
}

impl Default for CounterPolicy {
    fn default() -> Self {
        Self {
            key: String::new(),
            max_capacity: 0,
            min_capacity: 0,
            buffer_size: IntOrPercent::Int(0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPolicy {
    pub key: String,
    pub max_capacity: i64,
    pub min_capacity: i64,
    pub buffer_size: IntOrPercent,
}

impl Default for ListPolicy {
    fn default() -> Self {
        Self {
            key: String::new(),
            max_capacity: 0,
            min_capacity: 0,
            buffer_size: IntOrPercent::Int(0),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Between {
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub end: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePeriod {
    pub timezone: String,
    pub start_cron: String,
    pub duration: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchedulePolicy {
    pub between: Between,
    pub active_period: ActivePeriod,
    pub policy: FleetAutoscalerPolicy,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainEntry {
    pub id: String,
    #[serde(flatten)]
    pub policy: FleetAutoscalerPolicy,
}

pub type ChainPolicy = Vec<ChainEntry>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmFrom {
    pub url: Option<UrlConfiguration>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmPolicy {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub function: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub config: BTreeMap<String, String>,
    pub from: WasmFrom,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hash: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetAutoscalerPolicy {
    #[serde(rename = "type")]
    pub policy_type: FleetAutoscalerPolicyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffer: Option<BufferPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<UrlConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counter: Option<CounterPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list: Option<ListPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<SchedulePolicy>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chain: ChainPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wasm: Option<WasmPolicy>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedIntervalSync {
    pub seconds: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetAutoscalerSync {
    #[serde(rename = "type")]
    pub sync_type: FleetAutoscalerSyncType,
    pub fixed_interval: FixedIntervalSync,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetAutoscalerSpec {
    pub fleet_name: String,
    pub policy: FleetAutoscalerPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<FleetAutoscalerSync>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetAutoscalerStatus {
    pub current_replicas: i32,
    pub desired_replicas: i32,
    #[serde(default)]
    pub last_scale_time: Option<String>,
    pub able_to_scale: bool,
    pub scaling_limited: bool,
    pub last_applied_policy: FleetAutoscalerPolicyType,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetAutoscaler {
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: FleetAutoscalerSpec,
    #[serde(default)]
    pub status: FleetAutoscalerStatus,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetAutoscalerList {
    #[serde(default)]
    pub items: Vec<FleetAutoscaler>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetAutoscaleRequest {
    pub uid: String,
    pub name: String,
    pub namespace: String,
    pub status: FleetStatus,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetAutoscaleResponse {
    pub uid: String,
    pub scale: bool,
    pub replicas: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetAutoscaleReview {
    pub request: Option<FleetAutoscaleRequest>,
    pub response: Option<FleetAutoscaleResponse>,
}


fn validation_required(path: impl Into<String>, detail: impl Into<String>) -> ValidationError {
    ValidationError {
        kind: ValidationErrorKind::Required,
        path: path.into(),
        bad_value: None,
        detail: detail.into(),
    }
}

fn validation_invalid(
    path: impl Into<String>,
    bad_value: impl Into<String>,
    detail: impl Into<String>,
) -> ValidationError {
    ValidationError {
        kind: ValidationErrorKind::Invalid,
        path: path.into(),
        bad_value: Some(bad_value.into()),
        detail: detail.into(),
    }
}

fn percent_value(value: &IntOrPercent) -> Result<i32, ()> {
    match value {
        IntOrPercent::Int(value) => Ok(*value),
        IntOrPercent::String(value) => value
            .strip_suffix('%')
            .ok_or(())
            .and_then(|value| value.parse::<i32>().map_err(|_| ())),
    }
}

fn validate_buffer_value(
    buffer_size: &IntOrPercent,
    min: i64,
    max: i64,
    path: &str,
    min_field: &str,
    max_field: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    match buffer_size {
        IntOrPercent::Int(buffer) => {
            let buffer = i64::from(*buffer);
            if buffer <= 0 {
                errors.push(validation_invalid(
                    format!("{path}.bufferSize"),
                    buffer.to_string(),
                    "must be greater than or equal to 0",
                ));
            }
            if max < buffer {
                errors.push(validation_invalid(
                    format!("{path}.{max_field}"),
                    max.to_string(),
                    format!("{max_field} should be bigger than or equal to bufferSize"),
                ));
            }
            if min != 0 && min < buffer {
                errors.push(validation_invalid(
                    format!("{path}.{min_field}"),
                    min.to_string(),
                    format!("{min_field} should be bigger than or equal to bufferSize"),
                ));
            }
        }
        IntOrPercent::String(value) => {
            let valid = percent_value(buffer_size)
                .is_ok_and(|percent| (1..=99).contains(&percent));
            if !valid {
                errors.push(validation_invalid(
                    format!("{path}.bufferSize"),
                    value.clone(),
                    "bufferSize should be between 1% and 99%",
                ));
            }
            if min < 1 {
                errors.push(validation_invalid(
                    format!("{path}.{min_field}"),
                    min.to_string(),
                    "must be greater than or equal to 0",
                ));
            }
        }
    }

    errors
}

impl BufferPolicy {
    pub fn validate(&self, path: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.min_replicas > self.max_replicas {
            errors.push(validation_invalid(
                format!("{path}.minReplicas"),
                self.min_replicas.to_string(),
                "minReplicas should be smaller than maxReplicas",
            ));
        }
        errors.extend(validate_buffer_value(
            &self.buffer_size,
            i64::from(self.min_replicas),
            i64::from(self.max_replicas),
            path,
            "minReplicas",
            "maxReplicas",
        ));
        errors
    }
}

impl CounterPolicy {
    pub fn validate(&self, path: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.min_capacity > self.max_capacity {
            errors.push(validation_invalid(
                format!("{path}.minCapacity"),
                self.min_capacity.to_string(),
                "minCapacity should be smaller than maxCapacity",
            ));
        }
        errors.extend(validate_buffer_value(
            &self.buffer_size,
            self.min_capacity,
            self.max_capacity,
            path,
            "minCapacity",
            "maxCapacity",
        ));
        if matches!(self.buffer_size, IntOrPercent::String(_)) && self.min_capacity < 1 {
            if let Some(error) = errors
                .iter_mut()
                .find(|error| error.path == format!("{path}.minCapacity"))
            {
                if error.detail == "must be greater than or equal to 0" {
                    error.bad_value = Some(match &self.buffer_size {
                        IntOrPercent::String(value) => value.clone(),
                        IntOrPercent::Int(value) => value.to_string(),
                    });
                    error.detail =
                        " when bufferSize in percentage format is used, minCapacity should be more than 0"
                            .to_owned();
                }
            }
        }
        errors
    }
}

impl ListPolicy {
    pub fn validate(&self, path: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.min_capacity > self.max_capacity {
            errors.push(validation_invalid(
                format!("{path}.minCapacity"),
                self.min_capacity.to_string(),
                "minCapacity should be smaller than maxCapacity",
            ));
        }
        errors.extend(validate_buffer_value(
            &self.buffer_size,
            self.min_capacity,
            self.max_capacity,
            path,
            "minCapacity",
            "maxCapacity",
        ));
        if matches!(self.buffer_size, IntOrPercent::String(_)) && self.min_capacity < 1 {
            if let Some(error) = errors
                .iter_mut()
                .find(|error| error.path == format!("{path}.minCapacity"))
            {
                if error.detail == "must be greater than or equal to 0" {
                    error.bad_value = Some(match &self.buffer_size {
                        IntOrPercent::String(value) => value.clone(),
                        IntOrPercent::Int(value) => value.to_string(),
                    });
                    error.detail =
                        " when bufferSize in percentage format is used, minCapacity should be more than 0"
                            .to_owned();
                }
            }
        }
        errors
    }
}

impl FixedIntervalSync {
    pub fn validate(&self, path: &str) -> Vec<ValidationError> {
        if self.seconds <= 0 {
            vec![validation_invalid(
                format!("{path}.seconds"),
                self.seconds.to_string(),
                "must be greater than or equal to 0",
            )]
        } else {
            Vec::new()
        }
    }
}

impl FleetAutoscalerPolicy {
    /// Dependency-free subset of Agones policy validation. Webhook, Schedule,
    /// Chain and Wasm validation is layered separately because those policies
    /// require URL/PEM/timezone/cron parsing and runtime feature gates.
    pub fn validate_core(&self, path: &str) -> Vec<ValidationError> {
        match self.policy_type {
            FleetAutoscalerPolicyType::Buffer => self
                .buffer
                .as_ref()
                .map_or_else(
                    || {
                        vec![validation_required(
                            format!("{path}.buffer"),
                            "buffer policy config params are missing",
                        )]
                    },
                    |buffer| buffer.validate(&format!("{path}.buffer")),
                ),
            FleetAutoscalerPolicyType::Counter => self
                .counter
                .as_ref()
                .map_or_else(
                    || {
                        vec![validation_required(
                            format!("{path}.counter"),
                            "counter policy config params are missing",
                        )]
                    },
                    |counter| counter.validate(&format!("{path}.counter")),
                ),
            FleetAutoscalerPolicyType::List => self
                .list
                .as_ref()
                .map_or_else(
                    || {
                        vec![validation_required(
                            format!("{path}.list"),
                            "list policy config params are missing",
                        )]
                    },
                    |list| list.validate(&format!("{path}.list")),
                ),
            _ => Vec::new(),
        }
    }
}

impl FleetAutoscaler {
    pub fn apply_defaults(&mut self) {
        let sync = self.spec.sync.get_or_insert_with(FleetAutoscalerSync::default);
        if sync.sync_type.as_str().is_empty() {
            sync.sync_type = FleetAutoscalerSyncType::FixedInterval;
        }
        if sync.fixed_interval.seconds == 0 {
            sync.fixed_interval.seconds = DEFAULT_INTERVAL_SYNC_SECONDS;
        }
    }

    pub fn validate_core(&self) -> Vec<ValidationError> {
        let mut errors = self.spec.policy.validate_core("spec.policy");
        if let Some(sync) = &self.spec.sync {
            errors.extend(sync.fixed_interval.validate("spec.sync.fixedInterval"));
        }
        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_policy_validation_matches_integer_and_percentage_invariants() {
        let valid = BufferPolicy {
            max_replicas: 10,
            min_replicas: 5,
            buffer_size: IntOrPercent::Int(5),
        };
        assert!(valid.validate("spec.policy.buffer").is_empty());

        let invalid = BufferPolicy {
            max_replicas: 10,
            min_replicas: 11,
            buffer_size: IntOrPercent::Int(0),
        };
        let errors = invalid.validate("spec.policy.buffer");
        assert!(errors.iter().any(|error| {
            error.path == "spec.policy.buffer.minReplicas"
                && error.detail == "minReplicas should be smaller than maxReplicas"
        }));
        assert!(errors.iter().any(|error| {
            error.path == "spec.policy.buffer.bufferSize"
                && error.detail == "must be greater than or equal to 0"
        }));

        let invalid_percent = BufferPolicy {
            max_replicas: 10,
            min_replicas: 0,
            buffer_size: IntOrPercent::String("100%".into()),
        };
        let errors = invalid_percent.validate("spec.policy.buffer");
        assert!(errors.iter().any(|error| {
            error.path == "spec.policy.buffer.bufferSize"
                && error.detail == "bufferSize should be between 1% and 99%"
        }));
        assert!(errors.iter().any(|error| error.path == "spec.policy.buffer.minReplicas"));
    }

    #[test]
    fn counter_and_list_validation_share_original_capacity_rules() {
        let counter = CounterPolicy {
            key: "players".into(),
            max_capacity: 10,
            min_capacity: 3,
            buffer_size: IntOrPercent::Int(5),
        };
        assert!(!counter.validate("spec.policy.counter").is_empty());
        assert_eq!(
            counter.validate("spec.policy.counter")[0].path,
            "spec.policy.counter.minCapacity"
        );

        let list = ListPolicy {
            key: "rooms".into(),
            max_capacity: 10,
            min_capacity: 5,
            buffer_size: IntOrPercent::Int(5),
        };
        assert!(list.validate("spec.policy.list").is_empty());

        let percent = CounterPolicy {
            key: "players".into(),
            max_capacity: 10,
            min_capacity: 0,
            buffer_size: IntOrPercent::String("20%".into()),
        };
        let errors = percent.validate("spec.policy.counter");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.policy.counter.minCapacity");
        assert_eq!(
            errors[0].detail,
            " when bufferSize in percentage format is used, minCapacity should be more than 0"
        );
        assert_eq!(errors[0].bad_value.as_deref(), Some("20%"));
    }

    #[test]
    fn missing_selected_policy_configuration_is_required() {
        for (policy_type, expected_path) in [
            (FleetAutoscalerPolicyType::Buffer, "spec.policy.buffer"),
            (FleetAutoscalerPolicyType::Counter, "spec.policy.counter"),
            (FleetAutoscalerPolicyType::List, "spec.policy.list"),
        ] {
            let policy = FleetAutoscalerPolicy {
                policy_type,
                ..Default::default()
            };
            let errors = policy.validate_core("spec.policy");
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].kind, ValidationErrorKind::Required);
            assert_eq!(errors[0].path, expected_path);
        }
    }

    #[test]
    fn core_dispatch_preserves_unknown_and_external_policy_noop_behavior() {
        for policy_type in [
            FleetAutoscalerPolicyType::Webhook,
            FleetAutoscalerPolicyType::Schedule,
            FleetAutoscalerPolicyType::Chain,
            FleetAutoscalerPolicyType::Wasm,
            FleetAutoscalerPolicyType::from("FuturePolicy"),
        ] {
            let policy = FleetAutoscalerPolicy {
                policy_type,
                ..Default::default()
            };
            assert!(policy.validate_core("spec.policy").is_empty());
        }
    }

    #[test]
    fn fixed_interval_validation_matches_original_zero_and_negative_behavior() {
        for seconds in [0, -1] {
            let errors =
                FixedIntervalSync { seconds }.validate("spec.sync.fixedInterval");
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].path, "spec.sync.fixedInterval.seconds");
            assert_eq!(
                errors[0].detail,
                "must be greater than or equal to 0"
            );
        }
        assert!(FixedIntervalSync { seconds: 1 }
            .validate("spec.sync.fixedInterval")
            .is_empty());
    }

    #[test]
    fn fleet_autoscaler_core_validation_composes_policy_and_sync() {
        let autoscaler = FleetAutoscaler {
            spec: FleetAutoscalerSpec {
                policy: FleetAutoscalerPolicy {
                    policy_type: FleetAutoscalerPolicyType::Buffer,
                    buffer: Some(BufferPolicy {
                        max_replicas: 10,
                        min_replicas: 5,
                        buffer_size: IntOrPercent::Int(5),
                    }),
                    ..Default::default()
                },
                sync: Some(FleetAutoscalerSync {
                    sync_type: FleetAutoscalerSyncType::FixedInterval,
                    fixed_interval: FixedIntervalSync { seconds: 30 },
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(autoscaler.validate_core().is_empty());
    }

    #[test]
    fn fleet_autoscaler_defaults_match_original_go_behavior() {
        let mut autoscaler = FleetAutoscaler::default();
        assert!(autoscaler.spec.sync.is_none());

        autoscaler.apply_defaults();
        let sync = autoscaler.spec.sync.as_ref().unwrap();
        assert_eq!(sync.sync_type, FleetAutoscalerSyncType::FixedInterval);
        assert_eq!(
            sync.fixed_interval.seconds,
            DEFAULT_INTERVAL_SYNC_SECONDS
        );

        let once = autoscaler.clone();
        autoscaler.apply_defaults();
        assert_eq!(autoscaler, once);
    }

    #[test]
    fn defaults_preserve_explicit_sync_values() {
        let mut autoscaler = FleetAutoscaler {
            spec: FleetAutoscalerSpec {
                sync: Some(FleetAutoscalerSync {
                    sync_type: FleetAutoscalerSyncType::FixedInterval,
                    fixed_interval: FixedIntervalSync { seconds: 15 },
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        autoscaler.apply_defaults();
        assert_eq!(
            autoscaler.spec.sync.unwrap().fixed_interval.seconds,
            15
        );
    }

    #[test]
    fn kubernetes_time_zero_values_serialize_as_null() {
        let between = serde_json::to_value(Between::default()).unwrap();
        assert!(between["start"].is_null());
        assert!(between["end"].is_null());

        let status = serde_json::to_value(FleetAutoscalerStatus::default()).unwrap();
        assert!(status["lastScaleTime"].is_null());
    }

    #[test]
    fn int_or_percent_wire_shape_is_shared_with_fleet() {
        let buffer = BufferPolicy {
            max_replicas: 10,
            min_replicas: 2,
            buffer_size: IntOrPercent::String("20%".into()),
        };
        let json = serde_json::to_value(&buffer).unwrap();
        assert_eq!(json["bufferSize"], "20%");

        let buffer = BufferPolicy {
            buffer_size: IntOrPercent::Int(5),
            ..Default::default()
        };
        let json = serde_json::to_value(&buffer).unwrap();
        assert_eq!(json["bufferSize"], 5);
    }

    #[test]
    fn ca_bundle_uses_go_compatible_base64_wire_format() {
        let config = UrlConfiguration {
            ca_bundle: b"certificate-bytes".to_vec(),
            ..Default::default()
        };
        let json = serde_json::to_value(&config).unwrap();
        assert_eq!(json["caBundle"], BASE64.encode(b"certificate-bytes"));

        let round_trip: UrlConfiguration = serde_json::from_value(json).unwrap();
        assert_eq!(round_trip.ca_bundle, b"certificate-bytes");
    }

    #[test]
    fn chain_entries_inline_policy_fields_like_go_json() {
        let entry = ChainEntry {
            id: "weekends".into(),
            policy: FleetAutoscalerPolicy {
                policy_type: FleetAutoscalerPolicyType::Buffer,
                buffer: Some(BufferPolicy {
                    max_replicas: 10,
                    buffer_size: IntOrPercent::Int(5),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        let json = serde_json::to_value(&entry).unwrap();
        assert_eq!(json["id"], "weekends");
        assert_eq!(json["type"], "Buffer");
        assert_eq!(json["buffer"]["maxReplicas"], 10);
        assert!(json.get("policy").is_none());
    }

    #[test]
    fn unknown_policy_and_sync_types_remain_lossless() {
        let policy = FleetAutoscalerPolicyType::from("FuturePolicy");
        assert_eq!(policy.as_str(), "FuturePolicy");
        assert_eq!(serde_json::to_string(&policy).unwrap(), r#""FuturePolicy""#);

        let sync = FleetAutoscalerSyncType::from("FutureSync");
        assert_eq!(sync.as_str(), "FutureSync");
        assert_eq!(serde_json::to_string(&sync).unwrap(), r#""FutureSync""#);
    }
}
