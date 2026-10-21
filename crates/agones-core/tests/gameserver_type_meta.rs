use agones_core::{GameServer, TypeMeta};
use serde_json::json;

#[test]
fn source_type_meta_survives_game_server_wire_roundtrip_and_actions() {
    for (kind, version) in [("test-kind", "custom.example/v9"), ("", ""), ("GameServer", "agones.dev/v1")] {
        let mut server: GameServer = serde_json::from_value(json!({
            "kind":kind,"apiVersion":version,"metadata":{"name":"game"},
            "spec":{"template":{"spec":{"containers":[{"name":"game","image":"test:v1"}]}}}
        })).unwrap();
        assert_eq!(server.type_meta, TypeMeta { kind:kind.into(), api_version:version.into() });
        server.apply_defaults();
        assert!(server.validate().is_empty());
        let wire = serde_json::to_value(&server).unwrap();
        for (key, value) in [("kind",kind), ("apiVersion",version)] {
            assert_eq!(wire.get(key).and_then(|v|v.as_str()), (!value.is_empty()).then_some(value));
        }
        let restored: GameServer = serde_json::from_value(wire).unwrap();
        assert_eq!(restored, server);
    }
}

#[test]
fn source_type_meta_accepts_missing_and_null_strings_and_rejects_wrong_types() {
    for wire in [json!({}),json!({"kind":null,"apiVersion":null})] {
        let server: GameServer = serde_json::from_value(wire).unwrap();
        assert_eq!(server.type_meta, TypeMeta::default());
        let value = serde_json::to_value(server).unwrap();
        assert!(value.get("kind").is_none() && value.get("apiVersion").is_none());
    }
    for wire in [json!({"kind":7}),json!({"apiVersion":true})] {
        assert!(serde_json::from_value::<GameServer>(wire).is_err());
    }
}
