use agones_core::{
    CanValidateResourceQuantities, Container, GameServerDefaultingContext, ResourceQuantity,
};
use cgp::prelude::*;
use serde_json::json;

struct QuantityOnlyContext;
delegate_and_check_components! {
    QuantityOnlyContext {
        agones_core::ResourceQuantityValidatorComponent: agones_core::ValidateAgonesResourceQuantities,
    }
}
use std::{cmp::Ordering, collections::BTreeMap};

fn quantity(text: &str) -> ResourceQuantity {
    text.parse().unwrap()
}

#[test]
fn quantities_keep_exact_units_rounding_caps_and_source_spelling() {
    for (input, output) in [
        ("1.5", "1500m"),
        ("1.5Gi", "1536Mi"),
        ("0.1m", "100u"),
        ("0.1n", "1n"),
        ("-0.1n", "-1n"),
        ("0.0000000011", "2n"),
        ("1.1Ki", "1126400m"),
        ("0.5Ki", "512"),
        ("99999999999999999999Ei", "9223372036854775807"),
        ("-99999999999999999999Ei", "-9223372036854775807"),
        ("9223372036854775808", "9223372036854775808"),
        ("+0009Ki", "+0009Ki"),
        ("+0008Ki", "8Ki"),
        ("0001", "0001"),
        ("1e4096", "10e4095"),
        ("1e4294967296", "1e4294967296"),
        ("1000E", "1"),
        ("+", "0"),
        (".", "0"),
        ("m", "0"),
    ] {
        assert_eq!(quantity(input).to_string(), output, "{input}");
    }
    for input in [
        "", "NaN", "inf", "1K", "1ki", "1µ", "1e+", " 1", "1 ", "1 m", "1..2",
    ] {
        assert!(input.parse::<ResourceQuantity>().is_err(), "{input}");
    }
}

#[test]
fn quantity_comparison_preserves_independent_integer_and_unit_relations() {
    for integer in [0_i64, 1, 7, 999, 1000, 9007199254740992, i64::MAX - 1] {
        let lesser = quantity(&integer.to_string());
        let greater = quantity(&(i128::from(integer) + 1).to_string());
        assert_eq!(lesser.compare(&greater), Ordering::Less);
        assert_eq!(greater.compare(&lesser), Ordering::Greater);
    }
    for (left, right) in [
        ("1", "1000m"),
        ("1Ki", "1024"),
        ("1Mi", "1048576"),
        ("1u", "1000n"),
        ("1.5Gi", "1536Mi"),
        ("-0.1n", "-1n"),
        ("1e4294967296", "1"),
        ("0.0000000001", "1n"),
    ] {
        assert_eq!(
            quantity(left).compare(&quantity(right)),
            Ordering::Equal,
            "{left} {right}"
        );
    }
}

#[test]
fn resource_wire_decode_is_exact_before_validation_including_null_and_escapes() {
    let container: Container = serde_json::from_str(r#"{"resources":{"limits":{"cpu":9007199254740993,"memory":"1.5Gi","zero":null,"tiny":"0.1n"}}}"#).unwrap();
    assert_eq!(
        serde_json::to_value(&container.resources).unwrap()["limits"],
        json!({
            "cpu":"9007199254740993", "memory":"1536Mi", "zero":"0", "tiny":"1n"
        })
    );
    let quantity: ResourceQuantity = serde_json::from_str("9007199254740993.000000001").unwrap();
    assert_eq!(quantity.to_string(), "9007199254740993000000001n");
    for wire in [r#""invalid""#, r#""\u0031""#, "true", "{}", "[]"] {
        assert!(
            serde_json::from_str::<ResourceQuantity>(wire).is_err(),
            "{wire}"
        );
    }
    for value in [json!({"cpu":"invalid"}), json!({"cpu":true}), json!(["1"])] {
        assert!(
            serde_json::from_value::<Container>(json!({"resources":{"requests":value}})).is_err()
        );
    }
}

#[test]
fn cgp_resource_policy_preserves_phases_paths_and_all_error_values() {
    let limits = BTreeMap::from([("cpu".into(), quantity("-1.5"))]);
    let requests = BTreeMap::from([("cpu".into(), quantity("-1.25"))]);
    let errors = GameServerDefaultingContext::default().validate_resource_quantities(
        &limits,
        &requests,
        "spec.resources",
    );
    assert_eq!(
        QuantityOnlyContext.validate_resource_quantities(&limits, &requests, "spec.resources"),
        errors
    );
    assert_eq!(errors.len(), 3);
    assert_eq!(
        errors
            .iter()
            .map(|e| (e.path.as_str(), e.bad_value.as_deref(), e.detail.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (
                "spec.resources.limits[cpu]",
                Some("-1500m"),
                "must be greater than or equal to 0"
            ),
            (
                "spec.resources.requests[cpu]",
                Some("-1250m"),
                "must be greater than or equal to 0"
            ),
            (
                "spec.resources.requests",
                Some("-1250m"),
                "must be less than or equal to cpu limit of -1500m"
            ),
        ]
    );
}

#[test]
fn direct_spec_and_all_resource_validators_share_the_core_quantity_policy() {
    use agones_core::{Fleet, GameServer, GameServerSet};
    let mut gs: GameServer = serde_json::from_value(json!({
        "metadata":{"name":"game"}, "spec":{"template":{"spec":{"containers":[{
            "name":"game", "image":"test:v1", "resources":{
                "requests":{"cpu":"9007199254740993"}, "limits":{"cpu":"9007199254740992"}
            }
        }]}}}
    }))
    .unwrap();
    gs.apply_defaults();
    let expected = "spec.template.spec.containers[0].resources.requests";
    for errors in [
        gs.validate(),
        gs.spec.validate(""),
        gs.spec
            .validate_resource_quantities_with_context(&QuantityOnlyContext, "spec"),
    ] {
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, expected);
        assert_eq!(errors[0].bad_value.as_deref(), Some("9007199254740993"));
    }
    // The source development branch returns before resource policy checks.
    assert!(gs.spec.validate("127.0.0.1").is_empty());
    let mut fleet = Fleet::default();
    fleet.spec.template.spec = gs.spec.clone();
    fleet.apply_defaults();
    let mut set = GameServerSet::default();
    set.spec.template.spec = gs.spec;
    for errors in [fleet.validate(), set.validate()] {
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].path,
            "spec.template.spec.template.spec.containers[0].resources.requests"
        );
    }
}

#[test]
fn decoded_resource_policy_retains_numeric_value_when_source_string_is_lossy() {
    let mut gs: agones_core::GameServer = serde_json::from_value(json!({
        "metadata":{"name":"game"}, "spec":{"template":{"spec":{"containers":[{
            "name":"game", "image":"test:v1", "resources":{
                "requests":{"cpu":"1000E"}, "limits":{"cpu":"2"}
            }
        }]}}}
    }))
    .unwrap();
    gs.apply_defaults();
    let errors = gs.validate();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].bad_value.as_deref(), Some("1"));
    assert_eq!(
        errors[0].detail,
        "must be less than or equal to cpu limit of 2"
    );
}

#[test]
fn comparison_preserves_the_confirmed_source_scale_overflow_exception() {
    for (left, right) in [
        ("1e2147483647", "1m"),
        ("1m", "1e2147483647"),
        ("-1e2147483647", "1u"),
        ("1e2147483647", "-1n"),
        ("1e2147483647", "0m"),
    ] {
        let (left, right) = (quantity(left), quantity(right));
        assert!(std::panic::catch_unwind(|| left.compare(&right)).is_err());
    }
    assert_eq!(
        quantity("1e2147483647").compare(&quantity("2e2147483647")),
        Ordering::Less
    );
}

#[test]
fn template_equality_uses_typed_numeric_values_and_shared_quantity_rounding() {
    let template = |value: &str| -> agones_core::GameServerTemplateSpec {
        serde_json::from_value(json!({"spec":{"template":{"spec":{"containers":[{
            "name":"game", "image":"test:v1", "resources":{"requests":{"cpu":value}}
        }]}}}}))
        .unwrap()
    };
    assert!(!agones_core::game_server_templates_semantically_equal(
        &template("1000E"),
        &template("1")
    ));
    assert!(agones_core::game_server_templates_semantically_equal(
        &template("0.1n"),
        &template("1n")
    ));
    assert!(agones_core::game_server_templates_semantically_equal(
        &template("99999999999999999999Ei"),
        &template("9223372036854775807")
    ));
}
