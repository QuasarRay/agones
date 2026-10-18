//! Concrete production replay for raw Go-string boundary witnesses.
use agones_core::{CanParseFeatureGates, FeatureGateContext, FeatureGateError};
use serde::Serialize;

#[derive(Serialize)]
struct Observation {
    id: &'static str,
    error_class: &'static str,
    example: bool,
    encoded: String,
}

fn main() {
    let context = FeatureGateContext::default();
    let mut gates = context.parse_feature_gates("").unwrap();
    let cases: &[(&str, &[u8], &str)] = &[
        (
            "ignored-raw-duplicate",
            b"Example=true&Example=\xff",
            "NONE",
        ),
        ("invalid-raw-bool", b"Example=\xff", "INVALID_BOOL"),
        ("unknown-raw-key", b"\xff=true", "UNKNOWN_FEATURE"),
        (
            "invalid-escape-in-duplicate",
            b"Example=true&Example=\xff%QQ",
            "INVALID_QUERY",
        ),
    ];
    let mut observations = Vec::new();
    for &(id, input, expected) in cases {
        let result = gates.replace_bytes(&context, input);
        let error_class = match result {
            Ok(()) => "NONE",
            Err(FeatureGateError::InvalidBool { .. }) => "INVALID_BOOL",
            Err(FeatureGateError::UnknownFeature(_)) => "UNKNOWN_FEATURE",
            Err(FeatureGateError::InvalidEscape(_) | FeatureGateError::SemicolonSeparator) => {
                "INVALID_QUERY"
            }
        };
        assert_eq!(error_class, expected, "{id}");
        assert!(
            gates.enabled("Example"),
            "{id}: rejection must retain the prior state"
        );
        observations.push(Observation {
            id,
            error_class,
            example: gates.enabled("Example"),
            encoded: gates.encode(),
        });
    }
    println!("{}", serde_json::to_string_pretty(&observations).unwrap());
}
