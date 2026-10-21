use agones_core::{GameServer, TypeMeta};
use agones_rusternetes::{AdapterError, RusternetesCustomResource};

#[test]
fn actual_native_conversion_retains_explicit_identity_and_supplies_wire_identity() {
    let source = GameServer { type_meta: TypeMeta { kind:"GameServer".into(), api_version:"agones.dev/v1".into() }, ..Default::default() };
    let native = source.to_rusternetes_custom_resource().unwrap();
    let restored = GameServer::from_rusternetes_custom_resource(native).unwrap();
    assert_eq!(restored, source);

    let source = GameServer::default();
    let native = source.to_rusternetes_custom_resource().unwrap();
    let restored = GameServer::from_rusternetes_custom_resource(native).unwrap();
    assert_eq!(restored.type_meta.kind, "GameServer");
    assert_eq!(restored.type_meta.api_version, "agones.dev/v1");
    assert_eq!(restored.spec, source.spec);
    assert_eq!(restored.metadata, source.metadata);
    assert_eq!(restored.status, source.status);
}

#[test]
fn actual_native_conversion_rejects_identity_conflicts_without_changing_the_source() {
    for meta in [TypeMeta { kind:"test-kind".into(), api_version:"agones.dev/v1".into() }, TypeMeta { kind:"GameServer".into(), api_version:"other.example/v2".into() }] {
        let source = GameServer { type_meta:meta.clone(), ..Default::default() };
        assert!(matches!(source.to_rusternetes_custom_resource(), Err(AdapterError::ResourceIdentity { .. })));
        assert_eq!(source.type_meta, meta);
    }
}
