// Generated from gated typed SDK IR by feature_case_codegen --sdk-version. Do not edit.
fn source_sdk_fleet_fixture() -> agones_core::Fleet {
    let mut fleet = agones_core::Fleet::default();
    fleet.spec.scheduling = Some(agones_core::SchedulingStrategy::Unknown(String::new()));
    fleet
}
fn source_sdk_assertion_000(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/fleet_test.go::TestFleetApplyDefaults::$root::line-112"
    );
}
fn source_sdk_assertion_001(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/fleet_test.go::TestFleetApplyDefaults::$root::line-121"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(32))]
fn source_sdk_fleet_case() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let context = agones_core::AgonesBuildVersionContext::from_bytes("dev".as_bytes()); let mut fleet = source_sdk_fleet_fixture(); fleet.apply_defaults_with_version_context(&context); source_sdk_assertion_000(fleet.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev")); fleet.apply_defaults_with_version_context(&context); source_sdk_assertion_001(fleet.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev")); #[cfg(kani)] { kani::cover!(true, "source SDK case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/fleet_test.go::TestFleetApplyDefaults::$root"); } });
}
#[cfg(not(kani))]
#[test]
fn source_sdk_typed_fleet_fixture_matches_exact_capture() {
    let captured: agones_core::Fleet = serde_json::from_str("{\"metadata\":{},\"spec\":{\"replicas\":0,\"scheduling\":\"\",\"strategy\":{},\"template\":{\"metadata\":{},\"spec\":{\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":null}}}}},\"status\":{\"allocatedReplicas\":0,\"allocations\":0,\"readyReplicas\":0,\"replicas\":0,\"reservedReplicas\":0}}").unwrap();
    assert_eq!(source_sdk_fleet_fixture(), captured);
}
