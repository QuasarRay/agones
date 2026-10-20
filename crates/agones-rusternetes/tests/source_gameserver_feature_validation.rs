//! Replay the complete selected Go table through the actual Rusternetes consumer.
use agones_core::{
    CanParseFeatureGates, FeatureGateContext, GameServer, GameServerDefaultingContext,
    ValidationErrorKind,
};
use agones_rusternetes::{NoopApiHooks, validate_game_server_creation_with_context};
use serde_json::{Value, json};

#[test]
fn every_original_validation_fixture_reaches_the_actual_adapter_with_exact_error_elements() {
    let ir: Value = serde_json::from_str(include_str!(
        "../../../verification/agones/gameserver-feature-validation.ir.json"
    ))
    .unwrap();
    let cases = ir["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 5);
    let mut assertions = 0;
    for case in cases {
        let mut gs: GameServer = serde_json::from_value(case["before"].clone()).unwrap();
        let presence = &case["fixture_presence"];
        if presence["counters_present"] == true && gs.spec.counters.is_none() {
            gs.spec.counters = Some(Default::default());
        }
        if presence["lists_present"] == true && gs.spec.lists.is_none() {
            gs.spec.lists = Some(Default::default());
        }
        assert_eq!(serde_json::to_value(&gs).unwrap(), case["before"]);
        assert_eq!(
            gs.spec.counters.is_some(),
            presence["counters_present"].as_bool().unwrap()
        );
        assert_eq!(
            gs.spec.lists.is_some(),
            presence["lists_present"].as_bool().unwrap()
        );
        assert_eq!(
            gs.spec.players.is_some(),
            presence["players_present"].as_bool().unwrap()
        );
        assert_eq!(
            gs.spec.ports.is_nil(),
            presence["ports_nil"].as_bool().unwrap()
        );
        let flags = FeatureGateContext::default()
            .parse_feature_gates(case["feature_flags"].as_str().unwrap());
        assert!(flags.is_ok(), "{}", case["assertions"][0]["id"]);
        assertions += 1;
        let context = GameServerDefaultingContext::from_feature_gates(&flags.unwrap());
        let actual =
            validate_game_server_creation_with_context(&context, &gs, &NoopApiHooks).unwrap();
        let mut actual = actual
            .iter()
            .map(|error| {
                json!({"Type":match error.kind {
                ValidationErrorKind::Invalid => "FieldValueInvalid",
                ValidationErrorKind::Forbidden => "FieldValueForbidden",
                ValidationErrorKind::Required => "FieldValueRequired",
                ValidationErrorKind::Duplicate => "FieldValueDuplicate",
                ValidationErrorKind::NotSupported => "FieldValueNotSupported",
                ValidationErrorKind::TooLong => "FieldValueTooLong",
            },"Field":error.path,"BadValue":error.bad_value,"Detail":error.detail,
            "Origin":"","CoveredByDeclarative":false})
                .to_string()
            })
            .collect::<Vec<_>>();
        let mut expected = case["expected_errors"]
            .as_array()
            .map(|errors| errors.iter().map(Value::to_string).collect::<Vec<_>>())
            .unwrap_or_default();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected, "{}", case["assertions"][1]["id"]);
        assertions += 1;
    }
    assert_eq!(
        assertions, 10,
        "every qualified source occurrence must execute"
    );
}
