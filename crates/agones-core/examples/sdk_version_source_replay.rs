//! Replay exact source fixtures; credit only the selected version observations.
use agones_core::{
    AgonesBuildVersionContext, CanParseFeatureGates, FeatureFlags, FeatureGateContext, Fleet,
    GameServer, VERSION_ANNOTATION,
};

pub fn replay() -> serde_json::Value {
    let ir: serde_json::Value = serde_json::from_str(include_str!(
        "../../../verification/agones/sdk-version.ir.json"
    ))
    .unwrap();
    assert_eq!(ir["source_mapped_version_assertions"], 19);
    let cases = ir["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 18);
    let mut observed = Vec::new();
    for case in cases {
        let expected = case["expected_version"].as_str().unwrap();
        let context = AgonesBuildVersionContext::from_bytes(expected.as_bytes());
        let assertions = case["assertion_ids"].as_array().unwrap();
        match case["kind"].as_str().unwrap() {
            "GAME_SERVER_VERSION" => {
                let mut gs: GameServer =
                    serde_json::from_value(case["input_json"].clone()).unwrap();
                let gates = FeatureGateContext::default()
                    .parse_feature_gates(case["feature_flags"].as_str().unwrap())
                    .unwrap();
                gs.apply_defaults_with_version_context(
                    &context,
                    FeatureFlags {
                        player_tracking: gates.enabled("PlayerTracking"),
                    },
                );
                let value = gs.metadata.annotations.get(VERSION_ANNOTATION).unwrap();
                assert_eq!(value, expected);
                assert_eq!(assertions.len(), 1);
                observed.push(serde_json::json!({"assertion":assertions[0],"observed":value}));
            }
            "FLEET_VERSION" => {
                let mut fleet: Fleet = serde_json::from_value(case["input_json"].clone()).unwrap();
                assert_eq!(assertions.len(), 2);
                for assertion in assertions {
                    fleet.apply_defaults_with_version_context(&context);
                    let value = fleet.metadata.annotations.get(VERSION_ANNOTATION).unwrap();
                    assert_eq!(value, expected);
                    observed.push(serde_json::json!({"assertion":assertion,"observed":value}));
                }
            }
            kind => panic!("unsupported source SDK case: {kind}"),
        }
    }
    assert_eq!(observed.len(), 19);
    serde_json::Value::Array(observed)
}

#[allow(dead_code)]
fn main() {
    println!("{}", serde_json::to_string_pretty(&replay()).unwrap());
}
