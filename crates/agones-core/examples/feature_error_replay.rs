//! Ordinary feature-error witnesses shared by the replay command and test.
use agones_core::{CanParseFeatureGates, FeatureGateContext};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Family {
    schema_version: u32,
    cases: Vec<Case>,
}
#[derive(Deserialize)]
struct Case {
    id: String,
    query_hex: String,
    expected_error: String,
}
#[derive(Serialize)]
pub struct Observation {
    id: String,
    error: String,
}

pub fn replay_feature_error_witnesses() -> Vec<Observation> {
    let family: Family = serde_json::from_str(include_str!(
        "../../../verification/agones/feature-error-witnesses.json"
    ))
    .unwrap();
    assert_eq!(family.schema_version, 1);
    assert_eq!(
        family.cases.len(),
        19,
        "nonempty exact ordinary witness selection"
    );
    let context = FeatureGateContext::default();
    family
        .cases
        .into_iter()
        .map(|case| {
            assert_eq!(case.query_hex.len() % 2, 0);
            let bytes = (0..case.query_hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&case.query_hex[i..i + 2], 16).unwrap())
                .collect::<Vec<_>>();
            let error = context
                .parse_feature_gates_bytes(&bytes)
                .unwrap_err()
                .to_string();
            assert_eq!(error, case.expected_error, "{}", case.id);
            Observation { id: case.id, error }
        })
        .collect()
}

#[allow(dead_code)] // Also included by the ordinary integration test.
fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&replay_feature_error_witnesses()).unwrap()
    );
}
