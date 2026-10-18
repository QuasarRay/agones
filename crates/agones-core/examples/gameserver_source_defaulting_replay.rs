//! Replay the gated exact source table through the real composed defaulter.
use agones_core::{
    CanParseFeatureGates, FeatureGateContext, GameServer, GameServerDefaultingContext,
    VERSION_ANNOTATION,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn guard_selected(guard: &str, expected: &Value) -> bool {
    if let Some(inner) = guard.strip_prefix("!(").and_then(|s| s.strip_suffix(')')) {
        return !guard_selected(inner, expected);
    }
    let field = |name| {
        expected
            .get(name)
            .expect("missing source conditional context")
    };
    match guard {
        "test.expected.alphaPlayerCapacity != nil" => !field("alpha_capacity").is_null(),
        "len(test.expected.evictionSafeSpec) > 0" => {
            !field("eviction_spec").as_str().unwrap().is_empty()
        }
        "len(test.expected.evictionSafeStatus) > 0" => {
            !field("eviction_status").as_str().unwrap().is_empty()
        }
        "test.expected.counterSpec != nil" => !field("counter_spec").is_null(),
        "test.expected.listSpec != nil" => !field("list_spec").is_null(),
        unknown => panic!("unsupported source guard: {unknown}"),
    }
}

pub fn replay() -> Value {
    let ir: Value = serde_json::from_str(include_str!(
        "../../../verification/agones/gameserver-defaulting.ir.json"
    ))
    .unwrap();
    assert_eq!(ir["case_count"], 17);
    assert_eq!(ir["source_assertion_occurrences"], 304);
    assert_eq!(
        ir["source_actions"],
        json!([
            "err := runtime.ParseFeatures(test.featureFlags)",
            "test.gameServer.ApplyDefaults()",
            "spec := test.gameServer.Spec"
        ])
    );
    let sites: BTreeMap<_, _> = ir["source_assertion_sites"]
        .as_array()
        .unwrap()
        .iter()
        .map(|site| (site["line"].as_u64().unwrap(), site))
        .collect();
    assert_eq!(sites.len(), 23);
    let mut observed = Vec::new();
    for case in ir["cases"].as_array().unwrap() {
        let selected_lines = sites
            .values()
            .filter(|site| {
                site["guards"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|g| guard_selected(g.as_str().unwrap(), &case["expected_context"]))
            })
            .map(|site| site["line"].as_u64().unwrap())
            .collect::<Vec<_>>();
        let captured_lines = case["assertions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a["line"].as_u64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            selected_lines, captured_lines,
            "conditional assertion selection {}",
            case["id"]
        );
        let mut gs: GameServer = serde_json::from_value(case["before"].clone()).unwrap();
        let parsed = FeatureGateContext::default()
            .parse_feature_gates(case["feature_flags"].as_str().unwrap());
        let mut applied = false;
        for assertion in case["assertions"].as_array().unwrap() {
            let site = sites[&assertion["line"].as_u64().unwrap()];
            let kind = assertion["kind"].as_str().unwrap();
            assert_eq!(assertion["kind"], site["kind"]);
            if kind != "NoError" && !applied {
                let context =
                    GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap());
                gs.apply_defaults_with_context(&context);
                applied = true;
            }
            let actual = match site["actual_expression"].as_str().unwrap() {
                "err" => match &parsed {
                    Ok(_) => Value::Null,
                    Err(error) => json!(error.to_string()),
                },
                "test.gameServer.Annotations[VersionAnnotation]" => {
                    json!(gs.metadata.annotations.get(VERSION_ANNOTATION).unwrap())
                }
                "test.gameServer.ObjectMeta.Finalizers" => json!(gs.metadata.finalizers),
                "spec.Container" => json!(gs.spec.container),
                "spec.Ports[0].Protocol" => json!(
                    gs.spec.ports[0]
                        .protocol
                        .as_ref()
                        .map_or("", |p| p.as_str())
                ),
                "spec.Ports[0].Range" => json!(gs.spec.ports[0].range),
                "test.gameServer.Status.State" => {
                    json!(gs.status.state.as_ref().map_or("", |s| s.as_str()))
                }
                "test.gameServer.Spec.Scheduling" => {
                    json!(gs.spec.scheduling.as_ref().map_or("", |s| s.as_str()))
                }
                "test.gameServer.Spec.Health" => json!(gs.spec.health),
                "test.gameServer.Spec.SdkServer" => json!(gs.spec.sdk_server),
                "test.gameServer.Status.Players.Capacity" => {
                    json!(gs.status.players.as_ref().unwrap().capacity)
                }
                "test.gameServer.Spec.Players" => json!(gs.spec.players),
                "test.gameServer.Status.Players" => json!(gs.status.players),
                "spec.Eviction.Safe" => json!(
                    gs.spec
                        .eviction
                        .as_ref()
                        .unwrap()
                        .safe
                        .as_ref()
                        .map_or("", |s| s.as_str())
                ),
                "spec.Eviction" => json!(gs.spec.eviction),
                "test.gameServer.Status.Eviction.Safe" => json!(
                    gs.status
                        .eviction
                        .as_ref()
                        .unwrap()
                        .safe
                        .as_ref()
                        .map_or("", |s| s.as_str())
                ),
                "test.gameServer.Status.Eviction" => json!(gs.status.eviction),
                "test.gameServer.Status.Counters" => json!(gs.status.counters),
                "test.gameServer.Spec.Counters" => json!(gs.spec.counters),
                "test.gameServer.Status.Lists" => json!(gs.status.lists),
                "test.gameServer.Spec.Lists" => json!(gs.spec.lists),
                expression => panic!("unsupported source observation: {expression}"),
            };
            let expected = &assertion["expected"];
            match kind {
                "Equal" => assert_eq!(&actual, expected, "{}", assertion["id"]),
                "Contains" => assert!(
                    actual.as_array().unwrap().contains(expected),
                    "{}",
                    assertion["id"]
                ),
                "Nil" | "NoError" => {
                    assert!(expected.is_null());
                    assert!(actual.is_null(), "{}", assertion["id"]);
                }
                kind => panic!("unsupported source relation: {kind}"),
            }
            assert_eq!(
                actual, assertion["observed"],
                "source observation {}",
                assertion["id"]
            );
            observed.push(json!({"id":assertion["id"],"observed":actual}));
        }
        assert!(applied);
        assert_eq!(
            serde_json::to_value(gs).unwrap(),
            case["after"],
            "whole source object {}",
            case["id"]
        );
    }
    assert_eq!(observed.len(), 304);
    Value::Array(observed)
}

#[allow(dead_code)]
fn main() {
    println!("{}", serde_json::to_string_pretty(&replay()).unwrap());
}
