//! Exact assertions translated after the feature's implementation gate.
use agones_core::{CanParseFeatureGates, FeatureGateContext, FeatureGates};
use serde::Deserialize;

#[derive(Deserialize)]
struct SourceFamily {
    cases: Vec<SourceCase>,
}

#[derive(Deserialize)]
struct SourceCase {
    id: String,
    disposition: String,
    steps: Vec<Step>,
    assertion_count: usize,
}

#[derive(Deserialize)]
struct Step {
    kind: StepKind,
    #[serde(default)]
    value: String,
    enabled: bool,
    origin: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum StepKind {
    Parse,
    ParseEncoded,
    EnableAll,
    AssertNoError,
    AssertError,
    AssertErrorContains,
    AssertEnabled,
    AssertEncoding,
}

#[test]
fn exact_source_feature_gate_assertions() {
    let family: SourceFamily = serde_json::from_str(include_str!(
        "../../../verification/agones/feature-gates.ir.json"
    ))
    .unwrap();
    let context = FeatureGateContext {
        feature_gate_defaults: &[("Example", true), ("Test", false)],
    };
    let mut cases = 0;
    let mut assertions = 0;
    let mut unsupported = 0;
    for case in family.cases {
        if case.disposition == "UNSUPPORTED" {
            unsupported += 1;
            eprintln!("OPEN source obligation: {}", case.id);
            continue;
        }
        assert_eq!(case.disposition, "READY", "{}", case.id);
        cases += 1;
        let mut features = FeatureGates::default();
        let mut last_result = Ok(());
        let mut observed_assertions = 0;
        for step in case.steps {
            match step.kind {
                StepKind::Parse => last_result = features.replace(&context, &step.value),
                StepKind::ParseEncoded => {
                    let encoded = features.encode();
                    last_result = features.replace(&context, &encoded);
                }
                StepKind::EnableAll => features = context.all_features_enabled(),
                StepKind::AssertNoError => {
                    observed_assertions += 1;
                    assert!(last_result.is_ok(), "{}: {last_result:?}", step.origin);
                }
                StepKind::AssertError => {
                    observed_assertions += 1;
                    assert!(last_result.is_err(), "{}", step.origin);
                }
                StepKind::AssertErrorContains => {
                    observed_assertions += 1;
                    assert!(
                        last_result
                            .as_ref()
                            .unwrap_err()
                            .to_string()
                            .contains(&step.value),
                        "{}",
                        step.origin
                    );
                }
                StepKind::AssertEnabled => {
                    observed_assertions += 1;
                    assert_eq!(
                        features.enabled(&step.value),
                        step.enabled,
                        "{}",
                        step.origin
                    );
                }
                StepKind::AssertEncoding => {
                    observed_assertions += 1;
                    assert_eq!(features.encode(), step.value, "{}", step.origin);
                }
            }
        }
        assert_eq!(observed_assertions, case.assertion_count, "{}", case.id);
        assertions += observed_assertions;
        eprintln!("REPLAYED {}: {observed_assertions} assertions", case.id);
    }
    assert_eq!(cases, 5, "nonempty exact source case selection");
    assert_eq!(assertions, 17, "all supported source assertions must run");
    assert_eq!(
        unsupported, 1,
        "environment binding remains explicit proof debt"
    );
}
