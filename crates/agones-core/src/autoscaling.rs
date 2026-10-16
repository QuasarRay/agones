use std::collections::BTreeMap;

use agones_macros::AgonesStringEnum;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{FleetStatus, IntOrPercent, ObjectMeta};

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
    pub start: String,
    pub end: String,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
