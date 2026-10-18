//! Ordinary source-contract implementation witnesses, not extracted properties.
use agones_core::{
    AgonesBuildVersion, CanParseFeatureGates, FeatureGateContext, GameServer,
    GameServerDefaultingContext, VERSION_ANNOTATION,
};
use serde_json::{Value, json};

pub fn replay() -> Value {
    let data: Value = serde_json::from_str(include_str!(
        "../../../verification/agones/gameserver-defaulting-witnesses.json"
    ))
    .unwrap();
    let mut output = Vec::new();
    for case in data["cases"].as_array().unwrap() {
        let mut gs: GameServer = serde_json::from_value(case["input"].clone()).unwrap();
        let gates = FeatureGateContext::default()
            .parse_feature_gates(case["feature_flags"].as_str().unwrap())
            .unwrap();
        let mut context = GameServerDefaultingContext::from_feature_gates(&gates);
        context.agones_build_version = AgonesBuildVersion::new(b"dev".to_vec());
        for _ in 0..case["iterations"].as_u64().unwrap() {
            gs.apply_defaults_with_context(&context);
        }
        if case["mutate_spec_after_defaults"].as_bool().unwrap() {
            gs.spec
                .counters
                .as_mut()
                .unwrap()
                .get_mut("n")
                .unwrap()
                .count = 99;
            gs.spec.lists.as_mut().unwrap().get_mut("l").unwrap().values[0] = "changed".into();
        }
        let observed = json!({
            "state":gs.status.state.as_ref().map_or("", |value| value.as_str()),
            "container":gs.spec.container,"scheduling":gs.spec.scheduling.as_ref().map_or("", |value| value.as_str()),
            "port_policy":gs.spec.ports.first().and_then(|p|p.port_policy.as_ref()).map_or("",|v|v.as_str()),
            "protocol":gs.spec.ports.first().and_then(|p|p.protocol.as_ref()).map_or("",|v|v.as_str()),
            "health":gs.spec.health,"sdk":gs.spec.sdk_server,
            "eviction_spec":gs.spec.eviction.as_ref().and_then(|e|e.safe.as_ref()).map_or("",|v|v.as_str()),
            "eviction_status":gs.status.eviction.as_ref().and_then(|e|e.safe.as_ref()).map_or("",|v|v.as_str()),
            "players":gs.status.players,"counters":gs.status.counters,"lists":gs.status.lists,
            "counters_nil":gs.status.counters.is_none(),"lists_nil":gs.status.lists.is_none(),
            "finalizers":gs.metadata.finalizers,"version":gs.metadata.annotations.get(VERSION_ANNOTATION).unwrap(),
        });
        assert_eq!(
            observed, case["expected"],
            "literal witness {}",
            case["name"]
        );
        output.push(json!({"name":case["name"],"observed":observed,"object":gs}));
    }
    assert_eq!(output.len(), 14);
    Value::Array(output)
}

#[allow(dead_code)]
fn main() {
    println!("{}", serde_json::to_string_pretty(&replay()).unwrap());
}
