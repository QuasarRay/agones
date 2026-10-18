use std::{collections::{BTreeMap, BTreeSet, HashMap}, str::FromStr};

use agones_macros::{agones_invariant, AgonesStringEnum};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use chrono_tz::Tz;
use cron::Schedule;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use x509_parser::pem::Pem;

use crate::{Fleet, FleetStatus, IntOrPercent, ObjectMeta, ValidationError, ValidationErrorKind};
use crate::gameserver::{GameServer, SchedulingStrategy};
use crate::gameserverset::{NodeCount, sort_game_servers_by_strategy};

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
    group = "autoscaling.agones.dev",
    version = "v1",
    kind = "FleetAutoscaler",
    plural = "fleetautoscalers"
)]
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


fn go_duration_nanos(value: &str) -> Result<i64, String> {
    if !parse_go_duration(value) {
        return Err(format!("invalid duration {value}"));
    }

    let mut rest = value;
    let mut negative = false;
    if let Some(sign) = rest.as_bytes().first().copied() {
        if sign == b'-' || sign == b'+' {
            negative = sign == b'-';
            rest = &rest[1..];
        }
    }
    if rest == "0" {
        return Ok(0);
    }

    fn unit_nanos(unit: &str) -> Option<i128> {
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

    let mut total = 0i128;
    while !rest.is_empty() {
        let bytes = rest.as_bytes();
        let mut index = 0usize;
        let mut integer = 0i128;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            integer = integer * 10 + i128::from(bytes[index] - b'0');
            index += 1;
        }
        rest = &rest[index..];

        let mut fraction = 0i128;
        let mut fraction_scale = 1i128;
        if rest.starts_with('.') {
            rest = &rest[1..];
            let bytes = rest.as_bytes();
            let mut fraction_index = 0usize;
            while fraction_index < bytes.len()
                && bytes[fraction_index].is_ascii_digit()
            {
                if fraction_index < 18 {
                    fraction = fraction * 10
                        + i128::from(bytes[fraction_index] - b'0');
                    fraction_scale *= 10;
                }
                fraction_index += 1;
            }
            rest = &rest[fraction_index..];
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
        let unit = unit_nanos(&rest[..unit_end])
            .ok_or_else(|| format!("invalid duration {value}"))?;
        rest = &rest[unit_end..];

        total += integer * unit;
        if fraction_scale > 1 {
            total += fraction * unit / fraction_scale;
        }
    }

    if negative {
        total = -total;
    }
    i64::try_from(total).map_err(|_| format!("invalid duration {value}"))
}

fn schedule_cron_expression(value: &str) -> Result<Schedule, String> {
    let expression = match value {
        "@yearly" | "@annually" => "0 0 0 1 1 *".to_owned(),
        "@monthly" => "0 0 0 1 * *".to_owned(),
        "@weekly" => "0 0 0 * * 0".to_owned(),
        "@daily" | "@midnight" => "0 0 0 * * *".to_owned(),
        "@hourly" => "0 0 * * * *".to_owned(),
        value if value.starts_with("@every ") => {
            return Err("@every is an interval schedule".into());
        }
        value => format!("0 {value}"),
    };
    Schedule::from_str(&expression).map_err(|error| error.to_string())
}

/// Deterministic port of upstream isScheduleActive.
///
/// Validation is expected to run before this function, but unlike the Go
/// helper this returns malformed schedule input as an error rather than relying
/// on ignored parser errors.
pub fn is_fleet_autoscaler_schedule_active(
    schedule: &SchedulePolicy,
    current_time: DateTime<Utc>,
) -> Result<bool, String> {
    if let Some(start) = schedule.between.start.as_deref() {
        let start = DateTime::parse_from_rfc3339(start)
            .map_err(|error| error.to_string())?
            .with_timezone(&Utc);
        if current_time < start {
            return Ok(false);
        }
    }

    if let Some(end) = schedule.between.end.as_deref() {
        let end = DateTime::parse_from_rfc3339(end)
            .map_err(|error| error.to_string())?
            .with_timezone(&Utc);
        if current_time > end {
            return Ok(false);
        }
    }

    if schedule.active_period.start_cron.is_empty() {
        return Ok(true);
    }

    let timezone = if schedule.active_period.timezone.is_empty() {
        Tz::UTC
    } else {
        Tz::from_str(&schedule.active_period.timezone)
            .map_err(|error| error.to_string())?
    };

    let duration = if schedule.active_period.duration.is_empty() {
        ChronoDuration::hours(2_540_400)
    } else {
        ChronoDuration::nanoseconds(go_duration_nanos(
            &schedule.active_period.duration,
        )?)
    };
    let localized_now = current_time.with_timezone(&timezone);
    let current_minus_duration = localized_now - duration;

    let cron_start = if let Some(every) = schedule
        .active_period
        .start_cron
        .strip_prefix("@every ")
    {
        let every = ChronoDuration::nanoseconds(go_duration_nanos(
            every.trim(),
        )?);
        current_minus_duration + every
    } else {
        schedule_cron_expression(&schedule.active_period.start_cron)?
            .after(&current_minus_duration)
            .next()
            .ok_or_else(|| "cron schedule has no next occurrence".to_owned())?
    };
    let cron_end = cron_start + duration;
    let cron_delta = ChronoDuration::seconds(-90);

    Ok(
        localized_now > cron_start + cron_delta
            && localized_now < cron_end,
    )
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


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FleetAutoscalerScaleDecision {
    pub replicas: i32,
    pub limited: bool,
}

fn invalid_int_or_percent_error(value: &str) -> String {
    format!(
        "invalid value for IntOrString: invalid value \"{value}\": strconv.Atoi: parsing \"{value}\": invalid syntax"
    )
}

fn autoscaler_percent_value(value: &str) -> Result<i32, String> {
    let Some(percent) = value.strip_suffix('%') else {
        return Err(invalid_int_or_percent_error(value));
    };
    percent.parse::<i32>().map_err(|_| {
        invalid_int_or_percent_error(percent)
    })
}

/// Pure port of FleetAutoscaler's applyBufferPolicy.
///
/// Integer buffers request allocated plus buffer replicas. Percentage buffers
/// solve the desired fleet size from the opposite side: if the ready buffer is
/// P%, Allocated must occupy (100-P)% of the eventual fleet. The result is
/// rounded upward exactly like Go's math.Ceil and then clamped to the policy's
/// replica bounds.
pub fn apply_buffer_autoscaler_policy(
    policy: &BufferPolicy,
    fleet: &Fleet,
) -> Result<FleetAutoscalerScaleDecision, String> {
    let mut replicas = match &policy.buffer_size {
        IntOrPercent::Int(buffer) => fleet
            .status
            .allocated_replicas
            .wrapping_add(*buffer),
        IntOrPercent::String(value) => {
            let buffer_percent = autoscaler_percent_value(value)?;
            let denominator = 100i32.wrapping_sub(buffer_percent);
            let desired = (f64::from(
                fleet.status.allocated_replicas.wrapping_mul(100),
            ) / f64::from(denominator))
                .ceil();
            desired as i32
        }
    };

    let mut scaling_in_limited = false;
    let mut scaling_out_limited = false;

    if replicas < policy.min_replicas {
        replicas = policy.min_replicas;
        scaling_in_limited = true;
    }
    if replicas > policy.max_replicas {
        replicas = policy.max_replicas;
        scaling_out_limited = true;
    }

    Ok(FleetAutoscalerScaleDecision {
        replicas,
        limited: scaling_in_limited || scaling_out_limited,
    })
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CapacityPolicyKind {
    Counter,
    List,
}

impl CapacityPolicyKind {
    fn label(self) -> &'static str {
        match self {
            Self::Counter => "Counter",
            Self::List => "List",
        }
    }
}

#[derive(Clone, Debug)]
struct CapacityPolicyInput<'a> {
    kind: CapacityPolicyKind,
    key: &'a str,
    count: i64,
    capacity: i64,
    aggregate_count: i64,
    aggregate_capacity: i64,
    aggregate_allocated_count: i64,
    min_capacity: i64,
    max_capacity: i64,
    buffer_size: &'a IntOrPercent,
}

fn is_capacity_limited(
    aggregate_capacity: i64,
    min_capacity: i64,
    max_capacity: i64,
) -> (bool, i8) {
    if aggregate_capacity < min_capacity {
        return (true, 1);
    }
    if aggregate_capacity > max_capacity {
        return (true, -1);
    }
    (false, 0)
}

fn sorted_deletable_game_servers(
    fleet: &Fleet,
    game_servers: &[GameServer],
    node_counts: &HashMap<String, NodeCount>,
) -> Vec<GameServer> {
    let mut deletable: Vec<_> = game_servers
        .iter()
        .filter(|game_server| game_server.is_deletable())
        .cloned()
        .collect();
    let strategy = fleet
        .spec
        .scheduling
        .clone()
        .unwrap_or(SchedulingStrategy::Packed);
    sort_game_servers_by_strategy(
        &strategy,
        &mut deletable,
        node_counts,
        &fleet.spec.priorities,
    );
    deletable
}

fn game_server_capacity(
    game_server: &GameServer,
    kind: CapacityPolicyKind,
    key: &str,
) -> Option<(i64, i64)> {
    match kind {
        CapacityPolicyKind::Counter => game_server
            .status
            .counters
            .as_ref()
            .and_then(|counters| counters.get(key))
            .map(|counter| (counter.count, counter.capacity)),
        CapacityPolicyKind::List => game_server
            .status
            .lists
            .as_ref()
            .and_then(|lists| lists.get(key))
            .map(|list| (list.values.len() as i64, list.capacity)),
    }
}

fn scale_up_limited(
    mut replicas: i32,
    capacity: i64,
    mut aggregate_capacity: i64,
    min_capacity: i64,
) -> Result<FleetAutoscalerScaleDecision, String> {
    if capacity == 0 {
        return Err("cannot scale up as Capacity is equal to 0".into());
    }
    if capacity < 0 {
        return Err("cannot scale up as Capacity is less than 0".into());
    }

    while aggregate_capacity < min_capacity {
        aggregate_capacity = aggregate_capacity.wrapping_add(capacity);
        replicas = replicas.wrapping_add(1);
    }

    Ok(FleetAutoscalerScaleDecision {
        replicas,
        limited: true,
    })
}

fn scale_down_limited(
    fleet: &Fleet,
    game_servers: &[GameServer],
    node_counts: &HashMap<String, NodeCount>,
    kind: CapacityPolicyKind,
    key: &str,
    mut replicas: i32,
    mut aggregate_capacity: i64,
    max_capacity: i64,
) -> FleetAutoscalerScaleDecision {
    for game_server in sorted_deletable_game_servers(
        fleet,
        game_servers,
        node_counts,
    ) {
        if aggregate_capacity <= max_capacity {
            break;
        }

        if let Some((_, capacity)) =
            game_server_capacity(&game_server, kind, key)
        {
            aggregate_capacity =
                aggregate_capacity.wrapping_sub(capacity);
        }
        replicas = replicas.wrapping_sub(1);
    }

    if replicas < 1 {
        replicas = 1;
    }

    FleetAutoscalerScaleDecision {
        replicas,
        limited: true,
    }
}

fn scale_limited(
    scale: i8,
    fleet: &Fleet,
    game_servers: &[GameServer],
    node_counts: &HashMap<String, NodeCount>,
    input: &CapacityPolicyInput<'_>,
    replicas: i32,
) -> Result<FleetAutoscalerScaleDecision, String> {
    match scale {
        1 => scale_up_limited(
            replicas,
            input.capacity,
            input.aggregate_capacity,
            input.min_capacity,
        ),
        -1 => Ok(scale_down_limited(
            fleet,
            game_servers,
            node_counts,
            input.kind,
            input.key,
            replicas,
            input.aggregate_capacity,
            input.max_capacity,
        )),
        0 => Ok(FleetAutoscalerScaleDecision {
            replicas,
            limited: false,
        }),
        _ => Err(
            "cannot scale due to error in scaleLimited function".into(),
        ),
    }
}

fn scale_capacity_up(
    replicas: i32,
    input: &CapacityPolicyInput<'_>,
    available_capacity: i64,
    buffer: i64,
) -> Result<FleetAutoscalerScaleDecision, String> {
    let replica_capacity =
        input.capacity.wrapping_sub(input.count);
    if replica_capacity <= 0 {
        return Err(
            "cannot scale up as adding additional replicas does not increase available Capacity"
                .into(),
        );
    }

    let mut additional_replicas =
        ((buffer.wrapping_sub(available_capacity)) as f64
            / replica_capacity as f64)
            .ceil() as i32;

    let prospective_capacity = input
        .aggregate_capacity
        .wrapping_add(
            i64::from(additional_replicas)
                .wrapping_mul(input.capacity),
        );
    let (limited, _) = is_capacity_limited(
        prospective_capacity,
        input.min_capacity,
        input.max_capacity,
    );

    if limited {
        if input.capacity == 0 {
            return Err(
                "cannot scale up as Capacity is equal to 0".into(),
            );
        }
        additional_replicas = (input
            .max_capacity
            .wrapping_sub(input.aggregate_capacity)
            / input.capacity) as i32;
    }

    Ok(FleetAutoscalerScaleDecision {
        replicas: replicas.wrapping_add(additional_replicas),
        limited,
    })
}

fn scale_capacity_down(
    fleet: &Fleet,
    game_servers: &[GameServer],
    node_counts: &HashMap<String, NodeCount>,
    input: &CapacityPolicyInput<'_>,
    mut replicas: i32,
    mut aggregate_count: i64,
    mut aggregate_capacity: i64,
    buffer: i64,
) -> FleetAutoscalerScaleDecision {
    if aggregate_capacity == input.min_capacity {
        return FleetAutoscalerScaleDecision {
            replicas,
            limited: true,
        };
    }

    for game_server in sorted_deletable_game_servers(
        fleet,
        game_servers,
        node_counts,
    ) {
        replicas = replicas.wrapping_sub(1);
        let Some((count, capacity)) =
            game_server_capacity(&game_server, input.kind, input.key)
        else {
            continue;
        };

        aggregate_count = aggregate_count.wrapping_sub(count);
        aggregate_capacity =
            aggregate_capacity.wrapping_sub(capacity);

        let available_capacity =
            aggregate_capacity.wrapping_sub(aggregate_count);
        if available_capacity < buffer {
            return FleetAutoscalerScaleDecision {
                replicas: replicas.wrapping_add(1),
                limited: false,
            };
        }
        if aggregate_capacity < input.min_capacity {
            return FleetAutoscalerScaleDecision {
                replicas: replicas.wrapping_add(1),
                limited: true,
            };
        }
        if available_capacity == buffer {
            return FleetAutoscalerScaleDecision {
                replicas,
                limited: false,
            };
        }
        if aggregate_capacity == input.min_capacity {
            return FleetAutoscalerScaleDecision {
                replicas,
                limited: true,
            };
        }
    }

    if replicas < 1 {
        replicas = 1;
    }
    FleetAutoscalerScaleDecision {
        replicas,
        limited: false,
    }
}

fn apply_capacity_autoscaler_policy(
    input: CapacityPolicyInput<'_>,
    fleet: &Fleet,
    game_servers: &[GameServer],
    node_counts: &HashMap<String, NodeCount>,
) -> Result<FleetAutoscalerScaleDecision, String> {
    let (limited, scale) = is_capacity_limited(
        input.aggregate_capacity,
        input.min_capacity,
        input.max_capacity,
    );
    let replicas = fleet.status.replicas;

    let buffer = match input.buffer_size {
        IntOrPercent::Int(buffer) => i64::from(*buffer),
        IntOrPercent::String(value) => {
            let buffer_percent =
                i64::from(autoscaler_percent_value(value)?);
            if input.aggregate_allocated_count <= 0
                && input.capacity >= input.min_capacity
            {
                return Ok(FleetAutoscalerScaleDecision {
                    replicas: 1,
                    limited: true,
                });
            }

            let desired_capacity = ((input
                .aggregate_allocated_count
                .wrapping_mul(100)) as f64
                / (100i64.wrapping_sub(buffer_percent)) as f64)
                .ceil() as i64;
            desired_capacity
                .wrapping_sub(input.aggregate_allocated_count)
        }
    };

    let available_capacity = input
        .aggregate_capacity
        .wrapping_sub(input.aggregate_count);

    if available_capacity == buffer {
        if limited {
            return scale_limited(
                scale,
                fleet,
                game_servers,
                node_counts,
                &input,
                replicas,
            );
        }
        return Ok(FleetAutoscalerScaleDecision {
            replicas,
            limited: false,
        });
    }

    if available_capacity < buffer {
        if limited {
            return scale_limited(
                scale,
                fleet,
                game_servers,
                node_counts,
                &input,
                replicas,
            );
        }
        return scale_capacity_up(
            replicas,
            &input,
            available_capacity,
            buffer,
        );
    }

    if limited && scale == 1 {
        return scale_limited(
            scale,
            fleet,
            game_servers,
            node_counts,
            &input,
            replicas,
        );
    }

    Ok(scale_capacity_down(
        fleet,
        game_servers,
        node_counts,
        &input,
        replicas,
        input.aggregate_count,
        input.aggregate_capacity,
        buffer,
    ))
}

/// Pure port of FleetAutoscaler's CounterPolicy calculation.
pub fn apply_counter_autoscaler_policy(
    policy: &CounterPolicy,
    fleet: &Fleet,
    game_servers: &[GameServer],
    node_counts: &HashMap<String, NodeCount>,
) -> Result<FleetAutoscalerScaleDecision, String> {
    let counter = fleet
        .spec
        .template
        .spec
        .counters
        .as_ref()
        .and_then(|counters| counters.get(&policy.key))
        .ok_or_else(|| {
            format!(
                "cannot apply CounterPolicy as Counter key {} does not exist in the Fleet Spec",
                policy.key
            )
        })?;
    let aggregate = fleet
        .status
        .counters
        .get(&policy.key)
        .ok_or_else(|| {
            format!(
                "cannot apply CounterPolicy as Counter key {} does not exist in the Fleet Status",
                policy.key
            )
        })?;

    apply_capacity_autoscaler_policy(
        CapacityPolicyInput {
            kind: CapacityPolicyKind::Counter,
            key: &policy.key,
            count: counter.count,
            capacity: counter.capacity,
            aggregate_count: aggregate.count,
            aggregate_capacity: aggregate.capacity,
            aggregate_allocated_count: aggregate.allocated_count,
            min_capacity: policy.min_capacity,
            max_capacity: policy.max_capacity,
            buffer_size: &policy.buffer_size,
        },
        fleet,
        game_servers,
        node_counts,
    )
}

/// Pure port of FleetAutoscaler's ListPolicy calculation.
pub fn apply_list_autoscaler_policy(
    policy: &ListPolicy,
    fleet: &Fleet,
    game_servers: &[GameServer],
    node_counts: &HashMap<String, NodeCount>,
) -> Result<FleetAutoscalerScaleDecision, String> {
    let list = fleet
        .spec
        .template
        .spec
        .lists
        .as_ref()
        .and_then(|lists| lists.get(&policy.key))
        .ok_or_else(|| {
            format!(
                "cannot apply ListPolicy as List key {} does not exist in the Fleet Spec",
                policy.key
            )
        })?;
    let aggregate = fleet
        .status
        .lists
        .get(&policy.key)
        .ok_or_else(|| {
            format!(
                "cannot apply ListPolicy as List key {} does not exist in the Fleet Status",
                policy.key
            )
        })?;

    apply_capacity_autoscaler_policy(
        CapacityPolicyInput {
            kind: CapacityPolicyKind::List,
            key: &policy.key,
            count: list.values.len() as i64,
            capacity: list.capacity,
            aggregate_count: aggregate.count,
            aggregate_capacity: aggregate.capacity,
            aggregate_allocated_count: aggregate.allocated_count,
            min_capacity: policy.min_capacity,
            max_capacity: policy.max_capacity,
            buffer_size: &policy.buffer_size,
        },
        fleet,
        game_servers,
        node_counts,
    )
}

#[agones_invariant]
fn capacity_scale_up_never_decreases_replicas(
    input: (i16, i16, i16, i16, i16),
) -> bool {
    let (replicas, capacity, count, available, buffer) = input;
    if replicas < 0
        || capacity <= count
        || count < 0
        || available < 0
        || buffer <= available
    {
        return true;
    }

    let policy = CapacityPolicyInput {
        kind: CapacityPolicyKind::Counter,
        key: "c",
        count: i64::from(count),
        capacity: i64::from(capacity),
        aggregate_count: 0,
        aggregate_capacity: i64::from(available),
        aggregate_allocated_count: 0,
        min_capacity: 0,
        max_capacity: i64::MAX,
        buffer_size: &IntOrPercent::Int(i32::from(buffer)),
    };

    scale_capacity_up(
        i32::from(replicas),
        &policy,
        i64::from(available),
        i64::from(buffer),
    )
    .is_ok_and(|decision| decision.replicas >= i32::from(replicas))
}

#[agones_invariant]
fn buffer_policy_integer_decision_stays_bounded(
    input: (i16, i16, i16, i16),
) -> bool {
    let (allocated, buffer, min_replicas, max_replicas) = input;
    if allocated < 0
        || buffer <= 0
        || min_replicas < 0
        || max_replicas < min_replicas
    {
        return true;
    }

    let mut fleet = Fleet::default();
    fleet.status.allocated_replicas = i32::from(allocated);
    let policy = BufferPolicy {
        buffer_size: IntOrPercent::Int(i32::from(buffer)),
        min_replicas: i32::from(min_replicas),
        max_replicas: i32::from(max_replicas),
    };

    let Ok(decision) = apply_buffer_autoscaler_policy(&policy, &fleet) else {
        return false;
    };

    decision.replicas >= policy.min_replicas
        && decision.replicas <= policy.max_replicas
}

#[agones_invariant]
fn buffer_integer_valid_region_is_accepted(input: (i32, i32, i32)) -> bool {
    let (min_replicas, max_replicas, buffer_size) = input;
    if buffer_size <= 0
        || min_replicas > max_replicas
        || max_replicas < buffer_size
        || (min_replicas != 0 && min_replicas < buffer_size)
    {
        return true;
    }

    BufferPolicy {
        max_replicas,
        min_replicas,
        buffer_size: IntOrPercent::Int(buffer_size),
    }
    .validate("spec.policy.buffer")
    .is_empty()
}

#[agones_invariant]
fn capacity_integer_valid_region_is_accepted(input: (i64, i64, i32, bool)) -> bool {
    let (min_capacity, max_capacity, buffer_size, list_policy) = input;
    let buffer_size_i64 = i64::from(buffer_size);
    if buffer_size <= 0
        || min_capacity > max_capacity
        || max_capacity < buffer_size_i64
        || (min_capacity != 0 && min_capacity < buffer_size_i64)
    {
        return true;
    }

    if list_policy {
        ListPolicy {
            key: String::new(),
            max_capacity,
            min_capacity,
            buffer_size: IntOrPercent::Int(buffer_size),
        }
        .validate("spec.policy.list")
        .is_empty()
    } else {
        CounterPolicy {
            key: String::new(),
            max_capacity,
            min_capacity,
            buffer_size: IntOrPercent::Int(buffer_size),
        }
        .validate("spec.policy.counter")
        .is_empty()
    }
}

#[agones_invariant]
fn fixed_interval_positive_seconds_are_accepted(seconds: i32) -> bool {
    seconds <= 0
        || FixedIntervalSync { seconds }
            .validate("spec.sync.fixedInterval")
            .is_empty()
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


    fn schedule_with(
        start: Option<&str>,
        end: Option<&str>,
        cron: &str,
        duration: &str,
    ) -> SchedulePolicy {
        SchedulePolicy {
            between: Between {
                start: start.map(str::to_owned),
                end: end.map(str::to_owned),
            },
            active_period: ActivePeriod {
                timezone: "UTC".into(),
                start_cron: cron.into(),
                duration: duration.into(),
            },
            ..Default::default()
        }
    }

    #[test]
    fn schedule_active_respects_between_bounds_and_empty_cron() {
        let now = DateTime::parse_from_rfc3339(
            "2021-01-01T00:30:00Z",
        )
        .unwrap()
        .with_timezone(&Utc);

        assert!(
            is_fleet_autoscaler_schedule_active(
                &schedule_with(
                    Some("2021-01-01T00:00:00Z"),
                    Some("2021-01-01T01:00:00Z"),
                    "",
                    "1h",
                ),
                now,
            )
            .unwrap()
        );
        assert!(
            !is_fleet_autoscaler_schedule_active(
                &schedule_with(
                    Some("2021-01-01T00:31:00Z"),
                    None,
                    "",
                    "1h",
                ),
                now,
            )
            .unwrap()
        );
        assert!(
            !is_fleet_autoscaler_schedule_active(
                &schedule_with(
                    None,
                    Some("2021-01-01T00:29:00Z"),
                    "",
                    "1h",
                ),
                now,
            )
            .unwrap()
        );
    }

    #[test]
    fn schedule_active_ports_standard_cron_duration_windows() {
        let cases = [
            (
                "2020-12-26T08:30:00Z",
                schedule_with(
                    None,
                    Some("2021-01-01T00:00:00Z"),
                    "* * * * *",
                    "48h",
                ),
                true,
            ),
            (
                "2021-01-02T00:00:00Z",
                schedule_with(
                    Some("2021-01-01T00:00:00Z"),
                    None,
                    "* * * * *",
                    "1h",
                ),
                true,
            ),
            (
                "2021-03-14T02:00:00Z",
                schedule_with(
                    Some("2021-03-13T00:00:00Z"),
                    Some("2021-03-15T00:00:00Z"),
                    "* 2 * * *",
                    "1h",
                ),
                true,
            ),
            (
                "2023-12-12T03:49:00Z",
                schedule_with(
                    Some("2022-12-31T23:59:59Z"),
                    Some("2023-03-02T00:00:00Z"),
                    "* 0 * 3 *",
                    "",
                ),
                false,
            ),
        ];

        for (now, schedule, expected) in cases {
            let now = DateTime::parse_from_rfc3339(now)
                .unwrap()
                .with_timezone(&Utc);
            assert_eq!(
                is_fleet_autoscaler_schedule_active(&schedule, now)
                    .unwrap(),
                expected,
            );
        }
    }

    #[test]
    fn schedule_active_supports_timezone_and_cron_aliases() {
        let schedule = SchedulePolicy {
            active_period: ActivePeriod {
                timezone: "America/New_York".into(),
                start_cron: "@hourly".into(),
                duration: "30m".into(),
            },
            ..Default::default()
        };
        let now = DateTime::parse_from_rfc3339(
            "2021-01-01T05:15:00Z",
        )
        .unwrap()
        .with_timezone(&Utc);
        assert!(
            is_fleet_autoscaler_schedule_active(&schedule, now)
                .unwrap()
        );
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



    fn counter_policy_fixture(
        replicas: i32,
        count: i64,
        capacity: i64,
        aggregate_count: i64,
        aggregate_capacity: i64,
    ) -> Fleet {
        use crate::gameserver::CounterStatus;
        use crate::gameserverset::AggregatedCounterStatus;

        let mut fleet = Fleet::default();
        fleet.status.replicas = replicas;
        fleet.spec.template.spec.counters = Some(BTreeMap::from([(
            "rooms".into(),
            CounterStatus { count, capacity },
        )]));
        fleet.status.counters.insert(
            "rooms".into(),
            AggregatedCounterStatus {
                count: aggregate_count,
                capacity: aggregate_capacity,
                ..Default::default()
            },
        );
        fleet
    }

    #[test]
    fn counter_policy_ports_zero_scale_up_same_and_limit_cases() {
        let policy = CounterPolicy {
            key: "rooms".into(),
            min_capacity: 10,
            max_capacity: 100,
            buffer_size: IntOrPercent::Int(10),
        };
        let empty_nodes = HashMap::new();

        let zero = apply_counter_autoscaler_policy(
            &policy,
            &counter_policy_fixture(0, 0, 7, 0, 0),
            &[],
            &empty_nodes,
        )
        .unwrap();
        assert_eq!(
            zero,
            FleetAutoscalerScaleDecision {
                replicas: 2,
                limited: true,
            }
        );

        let up = apply_counter_autoscaler_policy(
            &policy,
            &counter_policy_fixture(10, 0, 7, 68, 70),
            &[],
            &empty_nodes,
        )
        .unwrap();
        assert_eq!(
            up,
            FleetAutoscalerScaleDecision {
                replicas: 12,
                limited: false,
            }
        );

        let same = apply_counter_autoscaler_policy(
            &policy,
            &counter_policy_fixture(10, 0, 7, 60, 70),
            &[],
            &empty_nodes,
        )
        .unwrap();
        assert_eq!(
            same,
            FleetAutoscalerScaleDecision {
                replicas: 10,
                limited: false,
            }
        );

        let limited = apply_counter_autoscaler_policy(
            &CounterPolicy {
                max_capacity: 100,
                ..policy.clone()
            },
            &counter_policy_fixture(14, 0, 7, 98, 98),
            &[],
            &empty_nodes,
        )
        .unwrap();
        assert_eq!(
            limited,
            FleetAutoscalerScaleDecision {
                replicas: 14,
                limited: true,
            }
        );
    }

    #[test]
    fn counter_policy_uses_per_replica_available_capacity() {
        let decision = apply_counter_autoscaler_policy(
            &CounterPolicy {
                key: "rooms".into(),
                min_capacity: 10,
                max_capacity: 100,
                buffer_size: IntOrPercent::Int(25),
            },
            &counter_policy_fixture(3, 7, 10, 21, 30),
            &[],
            &HashMap::new(),
        )
        .unwrap();

        assert_eq!(
            decision,
            FleetAutoscalerScaleDecision {
                replicas: 9,
                limited: false,
            }
        );
    }

    #[test]
    fn counter_policy_requires_matching_spec_and_status_keys() {
        let policy = CounterPolicy {
            key: "rooms".into(),
            min_capacity: 10,
            max_capacity: 100,
            buffer_size: IntOrPercent::Int(10),
        };
        let mut missing_spec =
            counter_policy_fixture(10, 0, 7, 31, 70);
        missing_spec.spec.template.spec.counters =
            Some(BTreeMap::new());
        assert!(
            apply_counter_autoscaler_policy(
                &policy,
                &missing_spec,
                &[],
                &HashMap::new(),
            )
            .unwrap_err()
            .contains("does not exist in the Fleet Spec")
        );

        let mut missing_status =
            counter_policy_fixture(10, 0, 7, 31, 70);
        missing_status.status.counters.clear();
        assert!(
            apply_counter_autoscaler_policy(
                &policy,
                &missing_status,
                &[],
                &HashMap::new(),
            )
            .unwrap_err()
            .contains("does not exist in the Fleet Status")
        );
    }

    fn list_policy_fixture(
        replicas: i32,
        template_values: usize,
        template_capacity: i64,
        aggregate_count: i64,
        aggregate_capacity: i64,
        allocated_count: i64,
    ) -> Fleet {
        use crate::gameserver::ListStatus;
        use crate::gameserverset::AggregatedListStatus;

        let mut fleet = Fleet::default();
        fleet.status.replicas = replicas;
        fleet.spec.template.spec.lists = Some(BTreeMap::from([(
            "gamers".into(),
            ListStatus {
                capacity: template_capacity,
                values: (0..template_values)
                    .map(|index| format!("v{index}"))
                    .collect(),
            },
        )]));
        fleet.status.lists.insert(
            "gamers".into(),
            AggregatedListStatus {
                count: aggregate_count,
                capacity: aggregate_capacity,
                allocated_count,
                ..Default::default()
            },
        );
        fleet
    }

    #[test]
    fn list_policy_ports_integer_and_percentage_scale_up() {
        let integer = apply_list_autoscaler_policy(
            &ListPolicy {
                key: "gamers".into(),
                min_capacity: 10,
                max_capacity: 100,
                buffer_size: IntOrPercent::Int(5),
            },
            &list_policy_fixture(10, 2, 3, 29, 30, 0),
            &[],
            &HashMap::new(),
        )
        .unwrap();
        assert_eq!(
            integer,
            FleetAutoscalerScaleDecision {
                replicas: 14,
                limited: false,
            }
        );

        let percentage = apply_list_autoscaler_policy(
            &ListPolicy {
                key: "gamers".into(),
                min_capacity: 10,
                max_capacity: 50,
                buffer_size: IntOrPercent::String("10%".into()),
            },
            &list_policy_fixture(11, 1, 3, 30, 30, 29),
            &[],
            &HashMap::new(),
        )
        .unwrap();
        assert_eq!(
            percentage,
            FleetAutoscalerScaleDecision {
                replicas: 13,
                limited: false,
            }
        );
    }

    #[test]
    fn list_policy_scale_down_ignores_allocated_game_servers() {
        use crate::gameserver::{
            GameServerState, GameServerStatus, ListStatus,
            SchedulingStrategy,
        };

        let mut fleet =
            list_policy_fixture(3, 0, 60, 55, 180, 55);
        fleet.spec.scheduling = Some(SchedulingStrategy::Packed);

        let allocated = GameServer {
            metadata: ObjectMeta {
                name: "allocated".into(),
                ..Default::default()
            },
            status: GameServerStatus {
                state: Some(GameServerState::Allocated),
                node_name: "n2".into(),
                lists: Some(BTreeMap::from([(
                    "gamers".into(),
                    ListStatus {
                        capacity: 60,
                        values: (0..55)
                            .map(|index| format!("p{index}"))
                            .collect(),
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        let ready = |name: &str| GameServer {
            metadata: ObjectMeta {
                name: name.into(),
                ..Default::default()
            },
            status: GameServerStatus {
                state: Some(GameServerState::Ready),
                node_name: "n1".into(),
                lists: Some(BTreeMap::from([(
                    "gamers".into(),
                    ListStatus {
                        capacity: 60,
                        values: Vec::new().into(),
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        let game_servers =
            vec![allocated, ready("ready-1"), ready("ready-2")];
        let node_counts = HashMap::from([
            (
                "n1".into(),
                NodeCount {
                    ready: 2,
                    allocated: 0,
                },
            ),
            (
                "n2".into(),
                NodeCount {
                    ready: 0,
                    allocated: 1,
                },
            ),
        ]);

        let decision = apply_list_autoscaler_policy(
            &ListPolicy {
                key: "gamers".into(),
                min_capacity: 120,
                max_capacity: 300,
                buffer_size: IntOrPercent::Int(120),
            },
            &fleet,
            &game_servers,
            &node_counts,
        )
        .unwrap();

        assert_eq!(
            decision,
            FleetAutoscalerScaleDecision {
                replicas: 3,
                limited: false,
            }
        );
    }

    #[test]
    fn buffer_policy_scales_up_from_integer_buffer() {
        let mut fleet = Fleet::default();
        fleet.spec.replicas = 50;
        fleet.status.replicas = 50;
        fleet.status.allocated_replicas = 40;
        fleet.status.ready_replicas = 10;

        let decision = apply_buffer_autoscaler_policy(
            &BufferPolicy {
                buffer_size: IntOrPercent::Int(20),
                min_replicas: 0,
                max_replicas: 100,
            },
            &fleet,
        )
        .unwrap();

        assert_eq!(
            decision,
            FleetAutoscalerScaleDecision {
                replicas: 60,
                limited: false,
            }
        );
    }

    #[test]
    fn buffer_policy_reports_min_and_max_limiting() {
        let mut fleet = Fleet::default();
        fleet.status.allocated_replicas = 40;

        let min_limited = apply_buffer_autoscaler_policy(
            &BufferPolicy {
                buffer_size: IntOrPercent::Int(20),
                min_replicas: 65,
                max_replicas: 100,
            },
            &fleet,
        )
        .unwrap();
        assert_eq!(
            min_limited,
            FleetAutoscalerScaleDecision {
                replicas: 65,
                limited: true,
            }
        );

        let max_limited = apply_buffer_autoscaler_policy(
            &BufferPolicy {
                buffer_size: IntOrPercent::Int(20),
                min_replicas: 0,
                max_replicas: 55,
            },
            &fleet,
        )
        .unwrap();
        assert_eq!(
            max_limited,
            FleetAutoscalerScaleDecision {
                replicas: 55,
                limited: true,
            }
        );
    }

    #[test]
    fn buffer_percentage_solves_future_fleet_size_with_ceil() {
        let mut fleet = Fleet::default();
        fleet.status.allocated_replicas = 50;

        let decision = apply_buffer_autoscaler_policy(
            &BufferPolicy {
                buffer_size: IntOrPercent::String("20%".into()),
                min_replicas: 0,
                max_replicas: 100,
            },
            &fleet,
        )
        .unwrap();
        assert_eq!(decision.replicas, 63);
        assert!(!decision.limited);

        fleet.status.allocated_replicas = 1;
        let small = apply_buffer_autoscaler_policy(
            &BufferPolicy {
                buffer_size: IntOrPercent::String("10%".into()),
                min_replicas: 0,
                max_replicas: 10,
            },
            &fleet,
        )
        .unwrap();
        assert_eq!(small.replicas, 2);
    }

    #[test]
    fn buffer_policy_invalid_string_preserves_upstream_error_shape() {
        let mut fleet = Fleet::default();
        fleet.status.allocated_replicas = 1;

        let error = apply_buffer_autoscaler_policy(
            &BufferPolicy {
                buffer_size: IntOrPercent::String("asd".into()),
                min_replicas: 0,
                max_replicas: 10,
            },
            &fleet,
        )
        .unwrap_err();

        assert_eq!(
            error,
            "invalid value for IntOrString: invalid value \"asd\": strconv.Atoi: parsing \"asd\": invalid syntax"
        );
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
