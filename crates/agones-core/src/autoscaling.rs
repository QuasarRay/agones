use std::{collections::{BTreeMap, BTreeSet}, str::FromStr};

use agones_macros::AgonesStringEnum;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use cron::Schedule;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use x509_parser::pem::Pem;

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
fn validation_duplicate(path: impl Into<String>, detail: impl Into<String>) -> ValidationError {
    ValidationError {
        kind: ValidationErrorKind::Duplicate,
        path: path.into(),
        bad_value: None,
        detail: detail.into(),
    }
}

fn validation_forbidden(path: impl Into<String>, detail: impl Into<String>) -> ValidationError {
    ValidationError {
        kind: ValidationErrorKind::Forbidden,
        path: path.into(),
        bad_value: None,
        detail: detail.into(),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AutoscalerFeatures {
    pub scheduled_autoscaler: bool,
    pub wasm_autoscaler: bool,
}

fn ca_bundle_has_valid_certificate(bytes: &[u8]) -> bool {
    Pem::iter_from_buffer(bytes)
        .filter_map(Result::ok)
        .filter(|pem| pem.label == "CERTIFICATE")
        .any(|pem| pem.parse_x509().is_ok())
}

fn go_url_parse_succeeds(value: &str) -> bool {
    if Url::parse(value).is_ok() {
        return true;
    }
    // net/url.Parse accepts relative references; parse them against a dummy
    // base so the Rust validator accepts the same common relative forms.
    Url::parse("http://agones.invalid/")
        .ok()
        .and_then(|base| base.join(value).ok())
        .is_some()
}

impl UrlConfiguration {
    pub fn validate(&self, path: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if self.service.is_none() && self.url.is_none() {
            errors.push(validation_required(path, "url should be provided"));
        }
        if self.service.is_some() && self.url.is_some() {
            errors.push(validation_duplicate(
                format!("{path}.url"),
                "service and url cannot be used simultaneously",
            ));
        }
        if !self.ca_bundle.is_empty() && !ca_bundle_has_valid_certificate(&self.ca_bundle) {
            errors.push(validation_invalid(
                format!("{path}.caBundle"),
                BASE64.encode(&self.ca_bundle),
                "CA Bundle is not valid",
            ));
        }
        if let Some(url) = &self.url {
            if !go_url_parse_succeeds(url) {
                errors.push(validation_invalid(
                    format!("{path}.url"),
                    url.clone(),
                    "url is not valid",
                ));
            }
        }

        errors
    }
}

impl WasmPolicy {
    pub fn validate(
        &self,
        path: &str,
        features: AutoscalerFeatures,
    ) -> Vec<ValidationError> {
        if !features.wasm_autoscaler {
            return vec![validation_forbidden(
                path,
                "feature WasmAutoscaler must be enabled",
            )];
        }

        let from_path = format!("{path}.from");
        let Some(url) = &self.from.url else {
            return vec![validation_required(
                from_path,
                "wasm from configuration is missing",
            )];
        };
        url.validate(&format!("{from_path}.url"))
    }
}

fn go_timezone_exists(value: &str) -> bool {
    value.is_empty()
        || value == "UTC"
        || value == "Local"
        || Tz::from_str(value).is_ok()
}

fn parse_standard_cron(value: &str) -> bool {
    let mut spec = value;

    // robfig/cron extracts TZ= and CRON_TZ= before parsing the standard
    // five-field expression. Agones then rejects TZ usage separately, so a
    // syntactically valid timezone-prefixed cron must parse successfully here.
    if spec.starts_with("TZ=") || spec.starts_with("CRON_TZ=") {
        let Some(space) = spec.find(' ') else {
            return false;
        };
        let Some(eq) = spec.find('=') else {
            return false;
        };
        if !go_timezone_exists(&spec[eq + 1..space]) {
            return false;
        }
        spec = spec[space..].trim();
    }

    match spec {
        "@yearly" | "@annually" | "@monthly" | "@weekly" | "@daily" | "@midnight"
        | "@hourly" => true,
        _ if spec.starts_with("@every ") => {
            parse_go_duration(spec.trim_start_matches("@every ").trim())
        }
        _ => {
            let fields: Vec<_> = spec.split_whitespace().collect();
            fields.len() == 5
                && Schedule::from_str(&format!("0 {spec}")).is_ok()
        }
    }
}

fn parse_go_duration(value: &str) -> bool {
    const LIMIT: u64 = 1u64 << 63;

    fn unit_nanos(unit: &str) -> Option<u64> {
        match unit {
            "ns" => Some(1),
            "us" | "µs" | "μs" => Some(1_000),
            "ms" => Some(1_000_000),
            "s" => Some(1_000_000_000),
            "m" => Some(60 * 1_000_000_000),
            "h" => Some(60 * 60 * 1_000_000_000),
            _ => None,
        }
    }

    let mut rest = value;
    let mut negative = false;
    if let Some(first) = rest.as_bytes().first().copied() {
        if first == b'-' || first == b'+' {
            negative = first == b'-';
            rest = &rest[1..];
        }
    }

    // Go time.ParseDuration special-cases signed and unsigned zero.
    if rest == "0" {
        return true;
    }
    if rest.is_empty() {
        return false;
    }

    let mut total = 0u64;
    while !rest.is_empty() {
        let first = rest.as_bytes()[0];
        if first != b'.' && !first.is_ascii_digit() {
            return false;
        }

        let bytes = rest.as_bytes();
        let mut index = 0usize;
        let mut integer = 0u64;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            if integer > LIMIT / 10 {
                return false;
            }
            integer = integer * 10 + u64::from(bytes[index] - b'0');
            if integer > LIMIT {
                return false;
            }
            index += 1;
        }
        let pre = index > 0;
        rest = &rest[index..];

        let mut fraction = 0u64;
        let mut scale = 1f64;
        let mut post = false;
        if rest.starts_with('.') {
            rest = &rest[1..];
            let bytes = rest.as_bytes();
            let mut fraction_index = 0usize;
            let mut overflow = false;
            while fraction_index < bytes.len() && bytes[fraction_index].is_ascii_digit() {
                post = true;
                if !overflow {
                    if fraction > (LIMIT - 1) / 10 {
                        overflow = true;
                    } else {
                        let next =
                            fraction * 10 + u64::from(bytes[fraction_index] - b'0');
                        if next > LIMIT {
                            overflow = true;
                        } else {
                            fraction = next;
                            scale *= 10.0;
                        }
                    }
                }
                fraction_index += 1;
            }
            rest = &rest[fraction_index..];
        }

        if !pre && !post {
            return false;
        }

        let bytes = rest.as_bytes();
        let mut unit_end = 0usize;
        while unit_end < bytes.len() {
            let byte = bytes[unit_end];
            if byte == b'.' || byte.is_ascii_digit() {
                break;
            }
            unit_end += 1;
        }
        if unit_end == 0 {
            return false;
        }

        let Some(unit) = unit_nanos(&rest[..unit_end]) else {
            return false;
        };
        rest = &rest[unit_end..];

        if integer > LIMIT / unit {
            return false;
        }
        let mut term = integer * unit;
        if fraction > 0 {
            term += ((fraction as f64) * (unit as f64 / scale)) as u64;
            if term > LIMIT {
                return false;
            }
        }

        // This deliberately wraps like Go's uint64 accumulator before the
        // post-addition LIMIT check.
        total = total.wrapping_add(term);
        if total > LIMIT {
            return false;
        }
    }

    if negative {
        true
    } else {
        total <= LIMIT - 1
    }
}

impl SchedulePolicy {
    pub fn validate_at(
        &self,
        path: &str,
        features: AutoscalerFeatures,
        now: DateTime<Utc>,
    ) -> Vec<ValidationError> {
        if !features.scheduled_autoscaler {
            return vec![validation_forbidden(
                path,
                "feature ScheduledAutoscaler must be enabled",
            )];
        }

        let mut errors = Vec::new();

        if !self.active_period.timezone.is_empty()
            && Tz::from_str(&self.active_period.timezone).is_err()
        {
            errors.push(validation_invalid(
                format!("{path}.activePeriod.timezone"),
                self.active_period.timezone.clone(),
                format!(
                    "Error parsing timezone: unknown time zone {}\n",
                    self.active_period.timezone
                ),
            ));
        }

        if let Some(end) = &self.between.end {
            if let Ok(end_time) = DateTime::parse_from_rfc3339(end) {
                let end_time = end_time.with_timezone(&Utc);
                let mut end_before_now = false;
                if end_time < now {
                    errors.push(validation_invalid(
                        format!("{path}.between.end"),
                        end.clone(),
                        "end time must be after the current time",
                    ));
                    end_before_now = true;
                }

                if !end_before_now {
                    if let Some(start) = &self.between.start {
                        if let Ok(start_time) = DateTime::parse_from_rfc3339(start) {
                            if end_time < start_time.with_timezone(&Utc) {
                                errors.push(validation_invalid(
                                    format!("{path}.between"),
                                    format!("start={start},end={end}"),
                                    "start time must be before end time",
                                ));
                            }
                        }
                    }
                }
            }
        }

        if !self.active_period.start_cron.is_empty() {
            if !parse_standard_cron(&self.active_period.start_cron) {
                errors.push(validation_invalid(
                    format!("{path}.activePeriod.startCron"),
                    self.active_period.start_cron.clone(),
                    "invalid startCron",
                ));
            }
            if self.active_period.start_cron.contains("TZ") {
                errors.push(validation_invalid(
                    format!("{path}.activePeriod.startCron"),
                    self.active_period.start_cron.clone(),
                    "CRON_TZ or TZ used in activePeriod is not supported, please use the .schedule.timezone field to specify a timezone",
                ));
            }
        }

        if !self.active_period.duration.is_empty()
            && !parse_go_duration(&self.active_period.duration)
        {
            // The Go implementation accidentally records StartCron as the bad
            // value for a duration parse error; preserve that observable shape.
            errors.push(validation_invalid(
                format!("{path}.activePeriod.duration"),
                self.active_period.start_cron.clone(),
                "invalid duration",
            ));
        }

        errors
    }
}

impl FleetAutoscalerPolicy {
    fn has_chain_entry_policy(&self) -> bool {
        self.buffer.is_some()
            || self.webhook.is_some()
            || self.counter.is_some()
            || self.list.is_some()
            || self.schedule.is_some()
            || self.wasm.is_some()
    }

    pub fn validate_at(
        &self,
        path: &str,
        features: AutoscalerFeatures,
        now: DateTime<Utc>,
    ) -> Vec<ValidationError> {
        match self.policy_type {
            FleetAutoscalerPolicyType::Buffer
            | FleetAutoscalerPolicyType::Counter
            | FleetAutoscalerPolicyType::List => self.validate_core(path),
            FleetAutoscalerPolicyType::Webhook | FleetAutoscalerPolicyType::Wasm => {
                self.validate_webhook_wasm(path, features)
            }
            FleetAutoscalerPolicyType::Schedule => self.schedule.as_ref().map_or_else(
                || {
                    vec![validation_required(
                        format!("{path}.schedule"),
                        "schedule policy config params are missing",
                    )]
                },
                |schedule| schedule.validate_at(&format!("{path}.schedule"), features, now),
            ),
            FleetAutoscalerPolicyType::Chain => {
                if !features.scheduled_autoscaler {
                    return vec![validation_forbidden(
                        format!("{path}.chain"),
                        "feature ScheduledAutoscaler must be enabled",
                    )];
                }

                let chain_path = format!("{path}.chain");
                let mut errors = Vec::new();
                let mut seen = BTreeSet::new();
                for (index, entry) in self.chain.iter().enumerate() {
                    if !seen.insert(entry.id.clone()) {
                        errors.push(validation_invalid(
                            chain_path.clone(),
                            entry.id.clone(),
                            "id of chain entry must be unique",
                        ));
                    }

                    if entry.policy.policy_type.as_str().is_empty()
                        || !entry.policy.has_chain_entry_policy()
                    {
                        errors.push(validation_required(
                            format!("{chain_path}[{index}]"),
                            "valid policy is missing",
                        ));
                    }

                    errors.extend(entry.policy.validate_at(
                        &format!("{chain_path}[{index}].policy"),
                        features,
                        now,
                    ));
                }
                errors
            }
            FleetAutoscalerPolicyType::Unknown(_) => Vec::new(),
        }
    }
}

impl FleetAutoscalerPolicy {
    pub fn validate_webhook_wasm(
        &self,
        path: &str,
        features: AutoscalerFeatures,
    ) -> Vec<ValidationError> {
        match self.policy_type {
            FleetAutoscalerPolicyType::Webhook => self.webhook.as_ref().map_or_else(
                || {
                    vec![validation_required(
                        format!("{path}.webhook"),
                        "webhook policy config params are missing",
                    )]
                },
                |webhook| webhook.validate(&format!("{path}.webhook")),
            ),
            FleetAutoscalerPolicyType::Wasm => self.wasm.as_ref().map_or_else(
                || {
                    vec![validation_required(
                        format!("{path}.wasm"),
                        "wasm policy config params are missing",
                    )]
                },
                |wasm| wasm.validate(&format!("{path}.wasm"), features),
            ),
            _ => Vec::new(),
        }
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

    pub fn validate_at(
        &self,
        features: AutoscalerFeatures,
        now: DateTime<Utc>,
    ) -> Vec<ValidationError> {
        let mut errors = self.spec.policy.validate_at("spec.policy", features, now);
        if let Some(sync) = &self.spec.sync {
            errors.extend(sync.fixed_interval.validate("spec.sync.fixedInterval"));
        }
        errors
    }

    pub fn validate(&self, features: AutoscalerFeatures) -> Vec<ValidationError> {
        self.validate_at(features, Utc::now())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-10-17T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    fn scheduled_features() -> AutoscalerFeatures {
        AutoscalerFeatures {
            scheduled_autoscaler: true,
            ..Default::default()
        }
    }

    #[test]
    fn schedule_feature_gate_short_circuits_other_validation() {
        let schedule = SchedulePolicy {
            active_period: ActivePeriod {
                timezone: "Not/AZone".into(),
                start_cron: "invalid cron".into(),
                duration: "invalid".into(),
            },
            ..Default::default()
        };
        let errors = schedule.validate_at(
            "spec.policy.schedule",
            AutoscalerFeatures::default(),
            test_now(),
        );
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, ValidationErrorKind::Forbidden);
        assert_eq!(
            errors[0].detail,
            "feature ScheduledAutoscaler must be enabled"
        );
    }

    #[test]
    fn schedule_validation_covers_timezone_and_time_window_rules() {
        let schedule = SchedulePolicy {
            between: Between {
                start: Some("2026-10-18T00:00:00Z".into()),
                end: Some("2026-10-17T12:00:00Z".into()),
            },
            active_period: ActivePeriod {
                timezone: "Not/AZone".into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let errors = schedule.validate_at(
            "spec.policy.schedule",
            scheduled_features(),
            test_now(),
        );
        assert!(errors.iter().any(|error| {
            error.path == "spec.policy.schedule.activePeriod.timezone"
        }));
        assert!(errors.iter().any(|error| {
            error.path == "spec.policy.schedule.between"
                && error.detail == "start time must be before end time"
        }));

        let expired = SchedulePolicy {
            between: Between {
                end: Some("2026-10-16T23:59:59Z".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        let errors = expired.validate_at(
            "spec.policy.schedule",
            scheduled_features(),
            test_now(),
        );
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.policy.schedule.between.end");
        assert_eq!(errors[0].detail, "end time must be after the current time");
    }

    #[test]
    fn schedule_validation_covers_standard_cron_tz_and_go_duration() {
        let valid = SchedulePolicy {
            active_period: ActivePeriod {
                timezone: "UTC".into(),
                start_cron: "* * * * 1-5".into(),
                duration: "2h45m".into(),
            },
            ..Default::default()
        };
        assert!(valid
            .validate_at(
                "spec.policy.schedule",
                scheduled_features(),
                test_now(),
            )
            .is_empty());

        for duration in [
            "0",
            "+0",
            "-0",
            "300ms",
            "-1.5h",
            "2h45m",
            "1µs",
            ".5s",
            "1.s",
            "9223372036854775807ns",
            "-9223372036854775808ns",
        ] {
            assert!(parse_go_duration(duration), "{duration}");
        }
        for duration in [
            "",
            "5",
            "h",
            "1x",
            ".s",
            "9223372036854775808ns",
        ] {
            assert!(!parse_go_duration(duration), "{duration}");
        }

        assert!(parse_standard_cron(
            "CRON_TZ=America/New_York * * * * *"
        ));
        assert!(parse_standard_cron("TZ=UTC @every 0"));

        let tz_only = SchedulePolicy {
            active_period: ActivePeriod {
                start_cron: "CRON_TZ=America/New_York * * * * *".into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let errors = tz_only.validate_at(
            "spec.policy.schedule",
            scheduled_features(),
            test_now(),
        );
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].detail,
            "CRON_TZ or TZ used in activePeriod is not supported, please use the .schedule.timezone field to specify a timezone"
        );

        let invalid = SchedulePolicy {
            active_period: ActivePeriod {
                start_cron: "CRON_TZ=UTC * * * * *".into(),
                duration: "not-duration".into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let errors = invalid.validate_at(
            "spec.policy.schedule",
            scheduled_features(),
            test_now(),
        );
        assert_eq!(errors.len(), 2);
        assert!(errors.iter().any(|error| {
            error.path == "spec.policy.schedule.activePeriod.startCron"
                && error.detail.contains("CRON_TZ or TZ")
        }));
        assert!(errors.iter().any(|error| {
            error.path == "spec.policy.schedule.activePeriod.duration"
                && error.bad_value.as_deref() == Some("CRON_TZ=UTC * * * * *")
        }));
    }

    #[test]
    fn chain_validation_preserves_duplicate_missing_and_recursive_policy_rules() {
        let valid_buffer = || FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Buffer,
            buffer: Some(BufferPolicy {
                max_replicas: 10,
                min_replicas: 5,
                buffer_size: IntOrPercent::Int(5),
            }),
            ..Default::default()
        };

        let chain = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Chain,
            chain: vec![
                ChainEntry {
                    id: "duplicate".into(),
                    policy: valid_buffer(),
                },
                ChainEntry {
                    id: "duplicate".into(),
                    policy: FleetAutoscalerPolicy::default(),
                },
            ],
            ..Default::default()
        };

        let errors = chain.validate_at(
            "spec.policy",
            scheduled_features(),
            test_now(),
        );
        assert!(errors.iter().any(|error| {
            error.path == "spec.policy.chain"
                && error.detail == "id of chain entry must be unique"
        }));
        assert!(errors.iter().any(|error| {
            error.path == "spec.policy.chain[1]"
                && error.detail == "valid policy is missing"
        }));

        let disabled = chain.validate_at(
            "spec.policy",
            AutoscalerFeatures::default(),
            test_now(),
        );
        assert_eq!(disabled.len(), 1);
        assert_eq!(disabled[0].kind, ValidationErrorKind::Forbidden);
        assert_eq!(disabled[0].path, "spec.policy.chain");
    }

    #[test]
    fn schedule_policy_does_not_recursively_validate_nested_policy() {
        let schedule = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Schedule,
            schedule: Some(Box::new(SchedulePolicy {
                policy: FleetAutoscalerPolicy {
                    policy_type: FleetAutoscalerPolicyType::Buffer,
                    buffer: None,
                    ..Default::default()
                },
                ..Default::default()
            })),
            ..Default::default()
        };
        assert!(schedule
            .validate_at("spec.policy", scheduled_features(), test_now())
            .is_empty());
    }

    #[test]
    fn full_fleet_autoscaler_validation_uses_policy_dispatch_and_sync() {
        let autoscaler = FleetAutoscaler {
            spec: FleetAutoscalerSpec {
                policy: FleetAutoscalerPolicy {
                    policy_type: FleetAutoscalerPolicyType::Schedule,
                    schedule: Some(Box::new(SchedulePolicy::default())),
                    ..Default::default()
                },
                sync: Some(FleetAutoscalerSync {
                    sync_type: FleetAutoscalerSyncType::FixedInterval,
                    fixed_interval: FixedIntervalSync { seconds: 0 },
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        let errors = autoscaler.validate_at(scheduled_features(), test_now());
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.sync.fixedInterval.seconds");
    }

    #[test]
    fn webhook_configuration_validation_matches_original_presence_rules() {
        let missing = UrlConfiguration::default();
        let errors = missing.validate("spec.policy.webhook");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, ValidationErrorKind::Required);
        assert_eq!(errors[0].path, "spec.policy.webhook");

        let both = UrlConfiguration {
            service: Some(ServiceReference {
                namespace: "default".into(),
                name: "allocator".into(),
                ..Default::default()
            }),
            url: Some("https://example.com/scale".into()),
            ..Default::default()
        };
        let errors = both.validate("spec.policy.webhook");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, ValidationErrorKind::Duplicate);
        assert_eq!(errors[0].path, "spec.policy.webhook.url");

        let service = UrlConfiguration {
            service: Some(ServiceReference {
                namespace: "default".into(),
                name: "allocator".into(),
                path: Some("/scale".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(service.validate("spec.policy.webhook").is_empty());
    }

    #[test]
    fn webhook_configuration_rejects_invalid_url_and_ca_bundle() {
        let invalid_url = UrlConfiguration {
            url: Some("http://[::1".into()),
            ..Default::default()
        };
        let errors = invalid_url.validate("spec.policy.webhook");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.policy.webhook.url");

        let invalid_ca = UrlConfiguration {
            url: Some("https://example.com/scale".into()),
            ca_bundle: b"not a certificate".to_vec(),
            ..Default::default()
        };
        let errors = invalid_ca.validate("spec.policy.webhook");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.policy.webhook.caBundle");
        assert_eq!(errors[0].detail, "CA Bundle is not valid");
    }

    #[test]
    fn webhook_url_parser_accepts_relative_references_like_go_net_url() {
        let config = UrlConfiguration {
            url: Some("/scale".into()),
            ..Default::default()
        };
        assert!(config.validate("spec.policy.webhook").is_empty());
    }

    #[test]
    fn wasm_validation_preserves_feature_gate_and_from_requirements() {
        let wasm = WasmPolicy::default();
        let errors = wasm.validate("spec.policy.wasm", AutoscalerFeatures::default());
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, ValidationErrorKind::Forbidden);
        assert_eq!(
            errors[0].detail,
            "feature WasmAutoscaler must be enabled"
        );

        let enabled = AutoscalerFeatures {
            wasm_autoscaler: true,
            ..Default::default()
        };
        let errors = wasm.validate("spec.policy.wasm", enabled);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, ValidationErrorKind::Required);
        assert_eq!(errors[0].path, "spec.policy.wasm.from");

        let wasm = WasmPolicy {
            from: WasmFrom {
                url: Some(UrlConfiguration {
                    url: Some("http://example.com/wasm-module".into()),
                    ..Default::default()
                }),
            },
            ..Default::default()
        };
        assert!(wasm.validate("spec.policy.wasm", enabled).is_empty());
    }

    #[test]
    fn policy_external_dispatch_requires_selected_configuration() {
        let webhook = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Webhook,
            ..Default::default()
        };
        let errors = webhook.validate_webhook_wasm(
            "spec.policy",
            AutoscalerFeatures::default(),
        );
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.policy.webhook");

        let wasm = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Wasm,
            ..Default::default()
        };
        let errors = wasm.validate_webhook_wasm(
            "spec.policy",
            AutoscalerFeatures {
                wasm_autoscaler: true,
                ..Default::default()
            },
        );
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.policy.wasm");
    }

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
