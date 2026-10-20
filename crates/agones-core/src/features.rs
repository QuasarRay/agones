//! Agones feature-gate configuration, independent of the environment or runtime.
//!
//! The defaults and query semantics follow `pkg/util/runtime/features.go` in
//! the pinned reference. Configuration is owned by CGP contexts, avoiding the
//! reference's process-global mutable map while preserving atomic replacement.

use std::{error::Error, fmt};

use cgp::prelude::*;
use comprende::c;
use derive_deftly::Deftly;

#[path = "go_quote.rs"]
mod go_quote;
pub use go_quote::quote_feature_error_bytes;

pub const FEATURE_GATE_FLAG: &str = "feature-gates";
pub const FEATURE_GATE_ENV: &str = "FEATURE_GATES";

pub const AGONES_FEATURE_DEFAULTS: &[(&str, bool)] = &[
    ("AutopilotPassthroughPort", true),
    ("CountsAndLists", true),
    ("DisableResyncOnSDKServer", true),
    ("Example", false),
    ("FleetAutoscaleRequestMetaData", true),
    ("GKEAutopilotExtendedDurationPods", true),
    ("PlayerAllocationFilter", false),
    ("PlayerTracking", false),
    ("PortPolicyNone", true),
    ("PortRanges", true),
    ("ProcessorAllocator", false),
    ("RollingUpdateFix", true),
    ("ScheduledAutoscaler", true),
    ("SidecarContainers", true),
    ("WasmAutoscaler", false),
];

// Give default contexts one stable allocation identity. References to a const
// alone may be materialized in different allocations by the compiler.
static CANONICAL_FEATURE_DEFAULTS: &[(&str, bool)] = AGONES_FEATURE_DEFAULTS;

/// Mirrors an initialized or not-yet-initialized reference feature map.
/// `Default` is empty, as Go's map is nil before ParseFeatures/EnableAllFeatures.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FeatureGates(Vec<(String, bool)>);

impl FeatureGates {
    pub fn enabled(&self, feature: &str) -> bool {
        self.0
            .binary_search_by(|(name, _)| name.as_str().cmp(feature))
            .map(|index| self.0[index].1)
            .unwrap_or(false)
    }

    fn from_defaults(defaults: &[(&str, bool)]) -> Self {
        // The canonical source table is already sorted and unique. Preserve
        // its order directly; custom contexts retain the map-like fallback.
        // Pointer identity is only a fast path, never a semantic precondition.
        if std::ptr::eq(defaults, CANONICAL_FEATURE_DEFAULTS) {
            return Self(c![
                ((*name).to_owned(), *enabled)
                for (name, enabled) in defaults
            ]);
        }
        // The production dictionary has fifteen entries. Sorted owned entries
        // preserve map lookup/encoding behavior without general tree machinery.
        // Repeated default names keep their last value, as map collection did.
        let mut values: Vec<(String, bool)> = Vec::with_capacity(defaults.len());
        for (name, enabled) in defaults {
            match values.binary_search_by(|(key, _)| key.as_str().cmp(name)) {
                Ok(index) => values[index].1 = *enabled,
                Err(index) => values.insert(index, ((*name).to_owned(), *enabled)),
            }
        }
        Self(values)
    }

    pub fn encode(&self) -> String {
        // url::form_urlencoded uses '*' unescaped and escapes '~', whereas
        // Go's QueryEscape does the reverse. Keep the source encoding here.
        c![
            format!("{}={value}", query_escape(key))
            for (key, value) in &self.0
        ]
        .join("&")
    }

    /// Parse into a temporary value; a rejected configuration leaves the
    /// currently active feature map intact, including previously enabled gates.
    pub fn replace<C: CanParseFeatureGates>(
        &mut self,
        context: &C,
        query: &str,
    ) -> Result<(), FeatureGateError> {
        *self = context.parse_feature_gates(query)?;
        Ok(())
    }

    pub fn replace_bytes<C: CanParseFeatureGates>(
        &mut self,
        context: &C,
        query: &[u8],
    ) -> Result<(), FeatureGateError> {
        *self = context.parse_feature_gates_bytes(query)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FeatureGateError {
    InvalidEscape(Vec<u8>),
    SemicolonSeparator,
    UnknownFeature(Vec<u8>),
    InvalidBool { feature: String, value: Vec<u8> },
}

impl fmt::Display for FeatureGateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The reference's package-scoped Errors wrapper prefixes every error.
        f.write_str("agones.dev/agones/pkg/util/runtime: ")?;
        match self {
            Self::InvalidEscape(value) => {
                f.write_str("error parsing query string for feature gates: invalid URL escape ")?;
                f.write_str(&quote_feature_error_bytes(value))
            }
            Self::SemicolonSeparator => f.write_str(
                "error parsing query string for feature gates: invalid semicolon separator in query",
            ),
            Self::UnknownFeature(value) => {
                f.write_str("Feature Gate ")?;
                f.write_str(&quote_feature_error_bytes(value))?;
                f.write_str(" is not a valid Feature Gate")
            }
            Self::InvalidBool { feature, value } => {
                f.write_str("error parsing bool value from flag ")?;
                f.write_str(feature)?;
                f.write_str(" : strconv.ParseBool: parsing ")?;
                f.write_str(&quote_feature_error_bytes(value))?;
                f.write_str(": invalid syntax")
            }
        }
    }
}

impl Error for FeatureGateError {}

#[cgp_getter]
pub trait HasFeatureGateDefaults {
    fn feature_gate_defaults(&self) -> &[(&'static str, bool)];
}

#[cgp_component(FeatureGateParser)]
pub trait CanParseFeatureGates {
    fn parse_feature_gates(&self, query: &str) -> Result<FeatureGates, FeatureGateError>;
    fn parse_feature_gates_bytes(&self, query: &[u8]) -> Result<FeatureGates, FeatureGateError>;
    fn all_features_enabled(&self) -> FeatureGates;
}

#[derive(Clone, Copy, Debug, HasField, Deftly)]
#[derive_deftly(FieldwiseDefault)]
pub struct FeatureGateContext {
    #[deftly(default = "CANONICAL_FEATURE_DEFAULTS")]
    pub feature_gate_defaults: &'static [(&'static str, bool)],
}

#[cgp_new_provider]
impl<Context: HasFeatureGateDefaults> FeatureGateParser<Context> for ParseAgonesFeatureGates {
    fn parse_feature_gates(
        context: &Context,
        query: &str,
    ) -> Result<FeatureGates, FeatureGateError> {
        Self::parse_feature_gates_bytes(context, query.as_bytes())
    }

    fn parse_feature_gates_bytes(
        context: &Context,
        query: &[u8],
    ) -> Result<FeatureGates, FeatureGateError> {
        let values = parse_query(query)?;
        let mut features = FeatureGates::from_defaults(context.feature_gate_defaults());

        // Go map iteration may choose any failing key. Sorted iteration chooses
        // one member of that permitted error set deterministically.
        for (key, value) in values {
            let index = features
                .0
                .binary_search_by(|(name, _)| name.as_bytes().cmp(&key))
                .map_err(|_| FeatureGateError::UnknownFeature(key))?;
            let enabled =
                parse_feature_bool(&value).ok_or_else(|| FeatureGateError::InvalidBool {
                    feature: features.0[index].0.clone(),
                    value,
                })?;
            features.0[index].1 = enabled;
        }
        Ok(features)
    }

    fn all_features_enabled(context: &Context) -> FeatureGates {
        let mut features = FeatureGates::from_defaults(context.feature_gate_defaults());
        c!(*enabled = true; for (_, enabled) in &mut features.0);
        features
    }
}

delegate_and_check_components! {
    FeatureGateContext {
        FeatureGateDefaultsGetterComponent: UseField<Symbol!("feature_gate_defaults")>,
        FeatureGateParserComponent: ParseAgonesFeatureGates,
    }
}

/// The exact accepted byte strings of Go strconv.ParseBool, with no trimming.
pub fn parse_feature_bool(value: &[u8]) -> Option<bool> {
    match value {
        b"1" | b"t" | b"T" | b"TRUE" | b"true" | b"True" => Some(true),
        b"0" | b"f" | b"F" | b"FALSE" | b"false" | b"False" => Some(false),
        _ => None,
    }
}

fn parse_query(query: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>, FeatureGateError> {
    let mut values: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    let mut error = None;
    for field in query
        .split(|byte| *byte == b'&')
        .filter(|field| !field.is_empty())
    {
        if field.contains(&b';') {
            error = Some(FeatureGateError::SemicolonSeparator);
            continue;
        }
        let (key, value) = match field.iter().position(|byte| *byte == b'=') {
            Some(index) => (&field[..index], &field[index + 1..]),
            None => (field, &[][..]),
        };
        let decoded = unescape(key).and_then(|key| unescape(value).map(|value| (key, value)));
        match decoded {
            Ok((key, value)) => {
                // Keep byte order and the first value of every decoded key.
                if let Err(index) = values.binary_search_by(|(name, _)| name.cmp(&key)) {
                    values.insert(index, (key, value));
                }
            }
            Err(err) if error.is_none() => error = Some(err),
            Err(_) => {}
        }
    }
    if let Some(error) = error {
        Err(error)
    } else {
        Ok(values)
    }
}

fn unescape(bytes: &[u8]) -> Result<Vec<u8>, FeatureGateError> {
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => decoded.push(b' '),
            b'%' => {
                let digit = |byte| match byte {
                    b'0'..=b'9' => Some(byte - b'0'),
                    b'a'..=b'f' => Some(byte - b'a' + 10),
                    b'A'..=b'F' => Some(byte - b'A' + 10),
                    _ => None,
                };
                let number = bytes
                    .get(i + 1)
                    .copied()
                    .and_then(digit)
                    .zip(bytes.get(i + 2).copied().and_then(digit));
                let Some((high, low)) = number else {
                    return Err(FeatureGateError::InvalidEscape(
                        bytes[i..bytes.len().min(i + 3)].to_vec(),
                    ));
                };
                decoded.push(high * 16 + low);
                i += 2;
            }
            byte => decoded.push(byte),
        }
        i += 1;
    }
    Ok(decoded)
}

fn query_escape(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(char::from(byte))
            }
            b' ' => encoded.push('+'),
            _ => {
                const HEX: &[u8; 16] = b"0123456789ABCDEF";
                encoded.push('%');
                encoded.push(char::from(HEX[usize::from(byte >> 4)]));
                encoded.push(char::from(HEX[usize::from(byte & 15)]));
            }
        }
    }
    encoded
}

impl From<&FeatureGates> for crate::AutoscalerFeatures {
    fn from(gates: &FeatureGates) -> Self {
        Self {
            scheduled_autoscaler: gates.enabled("ScheduledAutoscaler"),
            wasm_autoscaler: gates.enabled("WasmAutoscaler"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_defaults_match_distinct_reordered_and_duplicate_tables() {
        assert!(
            AGONES_FEATURE_DEFAULTS
                .windows(2)
                .all(|pair| pair[0].0 < pair[1].0)
        );
        let context = FeatureGateContext::default();
        assert!(std::ptr::eq(
            context.feature_gate_defaults,
            CANONICAL_FEATURE_DEFAULTS
        ));
        let canonical = context.parse_feature_gates("").unwrap();
        for (name, enabled) in AGONES_FEATURE_DEFAULTS {
            assert_eq!(canonical.enabled(name), *enabled);
        }
        let mut reordered = AGONES_FEATURE_DEFAULTS.to_vec();
        reordered.reverse();
        assert!(!std::ptr::eq(reordered.as_slice(), AGONES_FEATURE_DEFAULTS));
        assert_eq!(FeatureGates::from_defaults(&reordered), canonical);
        reordered.push(("PlayerTracking", true));
        reordered.push(("PlayerTracking", false));
        assert_eq!(FeatureGates::from_defaults(&reordered), canonical);
        reordered.push(("PlayerTracking", true));
        let changed = FeatureGates::from_defaults(&reordered);
        assert!(changed.enabled("PlayerTracking"));
        for (name, enabled) in AGONES_FEATURE_DEFAULTS {
            if *name != "PlayerTracking" {
                assert_eq!(changed.enabled(name), *enabled);
            }
        }
    }

    #[test]
    fn custom_default_duplicates_and_query_byte_order_preserve_map_semantics() {
        let context = FeatureGateContext {
            feature_gate_defaults: &[("Z", false), ("a~* ", false), ("A", true), ("Z", true)],
        };
        let defaults = context.parse_feature_gates("").unwrap();
        assert!(defaults.enabled("Z"));
        assert_eq!(defaults.encode(), "A=true&Z=true&a~%2A+=false");
        let gates = context
            .parse_feature_gates("a~%2A+=true&Z=0&A=false&Z=1")
            .unwrap();
        assert_eq!(gates.encode(), "A=false&Z=false&a~%2A+=true");
        assert!(!gates.enabled("missing"));
        assert_eq!(
            context.all_features_enabled().encode(),
            "A=true&Z=true&a~%2A+=true"
        );
    }

    #[test]
    fn configuration_is_atomic_and_uses_first_duplicate_value() {
        let context = FeatureGateContext::default();
        let mut gates = context
            .parse_feature_gates("WasmAutoscaler=1&WasmAutoscaler=0")
            .unwrap();
        assert!(gates.enabled("WasmAutoscaler"));
        let before = gates.clone();
        assert!(
            gates
                .replace(&context, "WasmAutoscaler=0&Example=%GG")
                .is_err()
        );
        assert_eq!(gates, before);
        assert!(gates.replace(&context, "Example=invalid").is_err());
        assert_eq!(gates, before);
        assert!(gates.replace(&context, "Foo=true").is_err());
        assert_eq!(gates, before);
        gates.replace(&context, "").unwrap();
        assert!(!gates.enabled("WasmAutoscaler"));
    }

    #[test]
    fn defaults_and_overrides_round_trip_without_global_state() {
        let context = FeatureGateContext::default();
        let gates = context.parse_feature_gates("").unwrap();
        assert_eq!(gates.0.len(), 15);
        assert_eq!(context.parse_feature_gates(&gates.encode()).unwrap(), gates);
        assert!(gates.enabled("SidecarContainers"));
        assert!(gates.enabled("ScheduledAutoscaler"));
        assert!(!gates.enabled("PlayerTracking"));
        assert!(!gates.enabled("not-a-gate"));
        assert_eq!(FeatureGates::default().encode(), "");
        assert!(!FeatureGates::default().enabled("SidecarContainers"));
        assert!(
            context
                .all_features_enabled()
                .0
                .iter()
                .all(|(_, value)| *value)
        );
        // The actual source allows false for stable gates despite its comment.
        assert!(
            !context
                .parse_feature_gates("PortRanges=false")
                .unwrap()
                .enabled("PortRanges")
        );
    }

    #[test]
    fn query_encoding_and_failure_precedence_follow_go() {
        let context = FeatureGateContext::default();
        assert!(
            context
                .parse_feature_gates("%45xample=%54rue&&")
                .unwrap()
                .enabled("Example")
        );
        assert!(matches!(
            context.parse_feature_gates("Example=TRUE;Foo=1"),
            Err(FeatureGateError::SemicolonSeparator)
        ));
        assert!(matches!(
            context.parse_feature_gates("Example=1&Example=%GG"),
            Err(FeatureGateError::InvalidEscape(_))
        ));
        assert!(matches!(
            context.parse_feature_gates("Example=%XX&bad;key=1"),
            Err(FeatureGateError::SemicolonSeparator)
        ));
        assert!(matches!(
            context.parse_feature_gates("Example=+true"),
            Err(FeatureGateError::InvalidBool { .. })
        ));
        assert_eq!(
            context.parse_feature_gates("%ff=1"),
            Err(FeatureGateError::UnknownFeature(vec![255]))
        );
        let names = FeatureGateContext {
            feature_gate_defaults: &[("a b+~*", true)],
        };
        let gates = names.parse_feature_gates("").unwrap();
        assert_eq!(gates.encode(), "a+b%2B~%2A=true");
        assert_eq!(names.parse_feature_gates(&gates.encode()).unwrap(), gates);
    }

    #[test]
    fn raw_go_strings_preserve_ignored_duplicate_bytes_and_error_values() {
        let context = FeatureGateContext::default();
        let mut gates = context
            .parse_feature_gates_bytes(b"Example=true&Example=\xff")
            .unwrap();
        assert!(gates.enabled("Example"));
        let previous = gates.clone();
        assert_eq!(
            gates.replace_bytes(&context, b"Example=\xff"),
            Err(FeatureGateError::InvalidBool {
                feature: "Example".into(),
                value: vec![255],
            })
        );
        assert_eq!(gates, previous);
        assert_eq!(
            context.parse_feature_gates_bytes(b"\xff=true"),
            Err(FeatureGateError::UnknownFeature(vec![255]))
        );
        assert_eq!(
            context.parse_feature_gates_bytes(b"Example=true&Example=\xff%QQ"),
            Err(FeatureGateError::InvalidEscape(b"%QQ".to_vec()))
        );
    }
}
