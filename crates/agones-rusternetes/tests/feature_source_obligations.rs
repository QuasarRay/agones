//! Exact assertions translated after the feature's implementation gate.
use agones_core::FeatureGateEnvironmentReaderComponent;
use agones_core::{
    CanLoadFeatureGates, CanParseFeatureGates, FeatureGateInputOptions, FeatureGates,
};
use cgp::prelude::*;
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
    #[serde(default)]
    key: String,
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
    SetEnv,
    BindFlags,
    ParseFlags,
    SetEnvKeyReplacer,
    BindEnv,
    ParseEnv,
}

#[derive(HasField)]
struct SourceInputContext {
    feature_gate_defaults: &'static [(&'static str, bool)],
    feature_gate_input_options: FeatureGateInputOptions,
    environment: Option<Vec<u8>>,
}

#[cgp_new_provider]
impl agones_core::FeatureGateEnvironmentReader<SourceInputContext> for ReadSourceInputEnvironment {
    fn read_feature_gate_environment(context: &SourceInputContext) -> Option<Vec<u8>> {
        context.environment.clone()
    }
}

delegate_and_check_components! {
    SourceInputContext {
        agones_core::FeatureGateDefaultsGetterComponent: UseField<Symbol!("feature_gate_defaults")>,
        agones_core::FeatureGateInputOptionsGetterComponent: UseField<Symbol!("feature_gate_input_options")>,
        agones_core::FeatureGateEnvironmentReaderComponent: ReadSourceInputEnvironment,
        agones_core::FeatureGateParserComponent: agones_core::ParseAgonesFeatureGates,
        agones_core::FeatureGateConfigurationLoaderComponent: agones_core::LoadAgonesFeatureGates,
    }
}

#[test]
fn exact_source_feature_gate_assertions() {
    let family: SourceFamily = serde_json::from_str(include_str!(
        "../../../verification/agones/feature-gates.ir.json"
    ))
    .unwrap();
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
        let mut context = SourceInputContext {
            feature_gate_defaults: &[("Example", true), ("Test", false)],
            feature_gate_input_options: FeatureGateInputOptions::default(),
            environment: None,
        };
        let (mut flags_bound, mut flags_parsed, mut key_replaced, mut environment_bound) =
            (false, false, false, false);
        let mut last_result = Ok(());
        let mut observed_assertions = 0;
        for step in case.steps {
            match step.kind {
                StepKind::SetEnv => {
                    assert_eq!(step.key, agones_core::FEATURE_GATE_ENV, "{}", step.origin);
                    context.environment = Some(step.value.into_bytes());
                    // A modeled fixture effect. Actual OS input behavior is
                    // separately exercised by isolated process replay.
                    last_result = Ok(());
                }
                StepKind::BindFlags => flags_bound = true,
                StepKind::ParseFlags => {
                    assert!(flags_bound);
                    flags_parsed = true;
                }
                StepKind::SetEnvKeyReplacer => {
                    assert_eq!((step.key.as_str(), step.value.as_str()), ("-", "_"));
                    key_replaced = true;
                }
                StepKind::BindEnv => {
                    assert!(key_replaced);
                    assert_eq!(step.key.replace('-', "_").to_uppercase(), step.value);
                    assert_eq!(step.value, agones_core::FEATURE_GATE_ENV);
                    environment_bound = true;
                    // The production CGP graph statically binds this exact
                    // key; no runtime name-registration failure is possible.
                    last_result = Ok(());
                }
                StepKind::ParseEnv => {
                    assert!(flags_parsed && environment_bound);
                    last_result = context.load_feature_gates().map(|loaded| features = loaded);
                }
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
    assert_eq!(cases, 6, "nonempty exact source case selection");
    assert_eq!(assertions, 22, "all supported source assertions must run");
    assert_eq!(
        unsupported, 0,
        "the input feature is now implemented and gated; formal effect boundaries remain separate"
    );
}
