// Generated from gated typed GameServer IR by feature_case_codegen --gameserver-defaulting. Do not edit.
use agones_core::CanParseFeatureGates;
fn source_gs_fixture_000() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = Some(std::collections::BTreeMap::from([(
        "games".to_owned(),
        agones_core::CounterStatus {
            count: 1i64,
            capacity: 100i64,
        },
    )]));
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0000(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-405"
    );
}
fn source_gs_assertion_0001(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-409"
    );
}
fn source_gs_assertion_0002(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-412"
    );
}
fn source_gs_assertion_0003(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-413"
    );
}
fn source_gs_assertion_0004(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-414"
    );
}
fn source_gs_assertion_0005(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-415"
    );
}
fn source_gs_assertion_0006(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-416"
    );
}
fn source_gs_assertion_0007(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-417"
    );
}
fn source_gs_assertion_0008(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-418"
    );
}
fn source_gs_assertion_0009(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-419"
    );
}
fn source_gs_assertion_0010(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-423"
    );
}
fn source_gs_assertion_0011(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-424"
    );
}
fn source_gs_assertion_0012(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-427"
    );
}
fn source_gs_assertion_0013(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-432"
    );
}
fn source_gs_assertion_0014(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-437"
    );
}
fn source_gs_assertion_0015(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-445"
    );
}
fn source_gs_assertion_0016(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_000() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_000(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("CountsAndLists=true");source_gs_assertion_0000(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0001(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0002(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0003(gs.spec.container == "testing");source_gs_assertion_0004(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0005(gs.spec.ports[0].range == "default");source_gs_assertion_0006(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0007(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0008(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0009(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0010(gs.spec.players.is_none());source_gs_assertion_0011(gs.status.players.is_none());source_gs_assertion_0012(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0013(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0014(gs.status.counters == Some(std::collections::BTreeMap::from([("games".to_owned(), agones_core::CounterStatus { count: 1i64, capacity: 100i64 }),])));source_gs_assertion_0015(gs.spec.lists.is_none());source_gs_assertion_0016(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_001() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = Some(std::collections::BTreeMap::from([(
        "players".to_owned(),
        agones_core::ListStatus {
            capacity: 100i64,
            values: vec!["foo".to_owned(), "bar".to_owned()].into(),
        },
    )]));
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0017(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-405"
    );
}
fn source_gs_assertion_0018(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-409"
    );
}
fn source_gs_assertion_0019(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-412"
    );
}
fn source_gs_assertion_0020(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-413"
    );
}
fn source_gs_assertion_0021(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-414"
    );
}
fn source_gs_assertion_0022(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-415"
    );
}
fn source_gs_assertion_0023(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-416"
    );
}
fn source_gs_assertion_0024(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-417"
    );
}
fn source_gs_assertion_0025(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-418"
    );
}
fn source_gs_assertion_0026(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-419"
    );
}
fn source_gs_assertion_0027(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-423"
    );
}
fn source_gs_assertion_0028(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-424"
    );
}
fn source_gs_assertion_0029(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-427"
    );
}
fn source_gs_assertion_0030(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-432"
    );
}
fn source_gs_assertion_0031(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-439"
    );
}
fn source_gs_assertion_0032(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-440"
    );
}
fn source_gs_assertion_0033(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists::line-443"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_001() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_001(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("CountsAndLists=true");source_gs_assertion_0017(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0018(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0019(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0020(gs.spec.container == "testing");source_gs_assertion_0021(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0022(gs.spec.ports[0].range == "default");source_gs_assertion_0023(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0024(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0025(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0026(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0027(gs.spec.players.is_none());source_gs_assertion_0028(gs.status.players.is_none());source_gs_assertion_0029(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0030(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0031(gs.spec.counters.is_none());source_gs_assertion_0032(gs.status.counters.is_none());source_gs_assertion_0033(gs.status.lists == Some(std::collections::BTreeMap::from([("players".to_owned(), agones_core::ListStatus { capacity: 100i64, values: vec!["foo".to_owned(),"bar".to_owned(),].into() }),])));#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_002() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([(
        "cluster-autoscaler.kubernetes.io/safe-to-evict".to_owned(),
        "false".to_owned(),
    )]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0034(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-405"
    );
}
fn source_gs_assertion_0035(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-409"
    );
}
fn source_gs_assertion_0036(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-412"
    );
}
fn source_gs_assertion_0037(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-413"
    );
}
fn source_gs_assertion_0038(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-414"
    );
}
fn source_gs_assertion_0039(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-415"
    );
}
fn source_gs_assertion_0040(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-416"
    );
}
fn source_gs_assertion_0041(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-417"
    );
}
fn source_gs_assertion_0042(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-418"
    );
}
fn source_gs_assertion_0043(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-419"
    );
}
fn source_gs_assertion_0044(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-423"
    );
}
fn source_gs_assertion_0045(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-424"
    );
}
fn source_gs_assertion_0046(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-427"
    );
}
fn source_gs_assertion_0047(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-432"
    );
}
fn source_gs_assertion_0048(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-439"
    );
}
fn source_gs_assertion_0049(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-440"
    );
}
fn source_gs_assertion_0050(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-445"
    );
}
fn source_gs_assertion_0051(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_002() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_002(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0034(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0035(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0036(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0037(gs.spec.container == "testing");source_gs_assertion_0038(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0039(gs.spec.ports[0].range == "default");source_gs_assertion_0040(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0041(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0042(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0043(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0044(gs.spec.players.is_none());source_gs_assertion_0045(gs.status.players.is_none());source_gs_assertion_0046(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0047(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0048(gs.spec.counters.is_none());source_gs_assertion_0049(gs.status.counters.is_none());source_gs_assertion_0050(gs.spec.lists.is_none());source_gs_assertion_0051(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_003() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: Some(agones_core::PortPolicy::from("Static")),
        protocol: Some(agones_core::Protocol::from("TCP")),
        container: None,
        container_port: 777i32,
        host_port: 777i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0052(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-405"
    );
}
fn source_gs_assertion_0053(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-409"
    );
}
fn source_gs_assertion_0054(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-412"
    );
}
fn source_gs_assertion_0055(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-413"
    );
}
fn source_gs_assertion_0056(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-414"
    );
}
fn source_gs_assertion_0057(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-415"
    );
}
fn source_gs_assertion_0058(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-416"
    );
}
fn source_gs_assertion_0059(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-417"
    );
}
fn source_gs_assertion_0060(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-418"
    );
}
fn source_gs_assertion_0061(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-419"
    );
}
fn source_gs_assertion_0062(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-423"
    );
}
fn source_gs_assertion_0063(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-424"
    );
}
fn source_gs_assertion_0064(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-427"
    );
}
fn source_gs_assertion_0065(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-432"
    );
}
fn source_gs_assertion_0066(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-439"
    );
}
fn source_gs_assertion_0067(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-440"
    );
}
fn source_gs_assertion_0068(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-445"
    );
}
fn source_gs_assertion_0069(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_003() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_003(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0052(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0053(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0054(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0055(gs.spec.container == "testing");source_gs_assertion_0056(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "TCP");source_gs_assertion_0057(gs.spec.ports[0].range == "default");source_gs_assertion_0058(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "Creating");source_gs_assertion_0059(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0060(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0061(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0062(gs.spec.players.is_none());source_gs_assertion_0063(gs.status.players.is_none());source_gs_assertion_0064(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0065(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0066(gs.spec.counters.is_none());source_gs_assertion_0067(gs.status.counters.is_none());source_gs_assertion_0068(gs.spec.lists.is_none());source_gs_assertion_0069(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_004() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "testing2".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "default".to_owned(),
        port_policy: Some(agones_core::PortPolicy::from("Static")),
        protocol: Some(agones_core::Protocol::from("TCP")),
        container: None,
        container_port: 0i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 12i32,
        failure_threshold: 10i32,
        initial_delay_seconds: 11i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: Some(agones_core::SdkServerLogLevel::from("Info")),
        grpc_port: 9357i32,
        http_port: 9358i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![
        agones_core::Container {
            name: "testing".to_owned(),
            image: "testing/image".to_owned(),
            ..Default::default()
        },
        agones_core::Container {
            name: "testing2".to_owned(),
            image: "testing/image2".to_owned(),
            ..Default::default()
        },
    ]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = Some(agones_core::GameServerState::from("TestState"));
    value
}
fn source_gs_assertion_0070(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-405"
    );
}
fn source_gs_assertion_0071(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-409"
    );
}
fn source_gs_assertion_0072(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-412"
    );
}
fn source_gs_assertion_0073(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-413"
    );
}
fn source_gs_assertion_0074(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-414"
    );
}
fn source_gs_assertion_0075(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-415"
    );
}
fn source_gs_assertion_0076(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-416"
    );
}
fn source_gs_assertion_0077(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-417"
    );
}
fn source_gs_assertion_0078(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-418"
    );
}
fn source_gs_assertion_0079(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-419"
    );
}
fn source_gs_assertion_0080(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-423"
    );
}
fn source_gs_assertion_0081(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-424"
    );
}
fn source_gs_assertion_0082(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-427"
    );
}
fn source_gs_assertion_0083(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-432"
    );
}
fn source_gs_assertion_0084(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-439"
    );
}
fn source_gs_assertion_0085(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-440"
    );
}
fn source_gs_assertion_0086(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-445"
    );
}
fn source_gs_assertion_0087(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_004() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_004(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0070(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0071(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0072(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0073(gs.spec.container == "testing2");source_gs_assertion_0074(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "TCP");source_gs_assertion_0075(gs.spec.ports[0].range == "default");source_gs_assertion_0076(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "TestState");source_gs_assertion_0077(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0078(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 12i32, failure_threshold: 10i32, initial_delay_seconds: 11i32 });source_gs_assertion_0079(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0080(gs.spec.players.is_none());source_gs_assertion_0081(gs.status.players.is_none());source_gs_assertion_0082(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0083(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0084(gs.spec.counters.is_none());source_gs_assertion_0085(gs.status.counters.is_none());source_gs_assertion_0086(gs.spec.lists.is_none());source_gs_assertion_0087(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_005() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0088(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-405"
    );
}
fn source_gs_assertion_0089(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-409"
    );
}
fn source_gs_assertion_0090(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-412"
    );
}
fn source_gs_assertion_0091(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-413"
    );
}
fn source_gs_assertion_0092(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-414"
    );
}
fn source_gs_assertion_0093(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-415"
    );
}
fn source_gs_assertion_0094(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-416"
    );
}
fn source_gs_assertion_0095(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-417"
    );
}
fn source_gs_assertion_0096(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-418"
    );
}
fn source_gs_assertion_0097(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-419"
    );
}
fn source_gs_assertion_0098(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-423"
    );
}
fn source_gs_assertion_0099(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-424"
    );
}
fn source_gs_assertion_0100(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-427"
    );
}
fn source_gs_assertion_0101(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-432"
    );
}
fn source_gs_assertion_0102(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-439"
    );
}
fn source_gs_assertion_0103(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-440"
    );
}
fn source_gs_assertion_0104(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-445"
    );
}
fn source_gs_assertion_0105(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_005() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_005(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0088(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0089(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0090(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0091(gs.spec.container == "testing");source_gs_assertion_0092(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0093(gs.spec.ports[0].range == "default");source_gs_assertion_0094(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0095(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0096(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0097(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0098(gs.spec.players.is_none());source_gs_assertion_0099(gs.status.players.is_none());source_gs_assertion_0100(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0101(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0102(gs.spec.counters.is_none());source_gs_assertion_0103(gs.status.counters.is_none());source_gs_assertion_0104(gs.spec.lists.is_none());source_gs_assertion_0105(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_006() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: Some(agones_core::PortPolicy::from("Passthrough")),
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0106(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-405"
    );
}
fn source_gs_assertion_0107(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-409"
    );
}
fn source_gs_assertion_0108(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-412"
    );
}
fn source_gs_assertion_0109(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-413"
    );
}
fn source_gs_assertion_0110(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-414"
    );
}
fn source_gs_assertion_0111(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-415"
    );
}
fn source_gs_assertion_0112(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-416"
    );
}
fn source_gs_assertion_0113(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-417"
    );
}
fn source_gs_assertion_0114(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-418"
    );
}
fn source_gs_assertion_0115(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-419"
    );
}
fn source_gs_assertion_0116(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-423"
    );
}
fn source_gs_assertion_0117(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-424"
    );
}
fn source_gs_assertion_0118(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-427"
    );
}
fn source_gs_assertion_0119(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-432"
    );
}
fn source_gs_assertion_0120(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-439"
    );
}
fn source_gs_assertion_0121(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-440"
    );
}
fn source_gs_assertion_0122(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-445"
    );
}
fn source_gs_assertion_0123(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_006() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_006(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0106(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0107(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0108(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0109(gs.spec.container == "testing");source_gs_assertion_0110(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0111(gs.spec.ports[0].range == "default");source_gs_assertion_0112(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0113(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0114(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0115(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0116(gs.spec.players.is_none());source_gs_assertion_0117(gs.status.players.is_none());source_gs_assertion_0118(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0119(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0120(gs.spec.counters.is_none());source_gs_assertion_0121(gs.status.counters.is_none());source_gs_assertion_0122(gs.spec.lists.is_none());source_gs_assertion_0123(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_007() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = Some(agones_core::Eviction {
        safe: Some(agones_core::EvictionSafe::from("Always")),
    });
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0124(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-405"
    );
}
fn source_gs_assertion_0125(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-409"
    );
}
fn source_gs_assertion_0126(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-412"
    );
}
fn source_gs_assertion_0127(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-413"
    );
}
fn source_gs_assertion_0128(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-414"
    );
}
fn source_gs_assertion_0129(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-415"
    );
}
fn source_gs_assertion_0130(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-416"
    );
}
fn source_gs_assertion_0131(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-417"
    );
}
fn source_gs_assertion_0132(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-418"
    );
}
fn source_gs_assertion_0133(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-419"
    );
}
fn source_gs_assertion_0134(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-423"
    );
}
fn source_gs_assertion_0135(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-424"
    );
}
fn source_gs_assertion_0136(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-427"
    );
}
fn source_gs_assertion_0137(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-432"
    );
}
fn source_gs_assertion_0138(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-439"
    );
}
fn source_gs_assertion_0139(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-440"
    );
}
fn source_gs_assertion_0140(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-445"
    );
}
fn source_gs_assertion_0141(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_007() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_007(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0124(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0125(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0126(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0127(gs.spec.container == "testing");source_gs_assertion_0128(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0129(gs.spec.ports[0].range == "default");source_gs_assertion_0130(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0131(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0132(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0133(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0134(gs.spec.players.is_none());source_gs_assertion_0135(gs.status.players.is_none());source_gs_assertion_0136(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Always");source_gs_assertion_0137(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Always");source_gs_assertion_0138(gs.spec.counters.is_none());source_gs_assertion_0139(gs.status.counters.is_none());source_gs_assertion_0140(gs.spec.lists.is_none());source_gs_assertion_0141(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_008() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([(
        "cluster-autoscaler.kubernetes.io/safe-to-evict".to_owned(),
        "true".to_owned(),
    )]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0142(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-405"
    );
}
fn source_gs_assertion_0143(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-409"
    );
}
fn source_gs_assertion_0144(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-412"
    );
}
fn source_gs_assertion_0145(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-413"
    );
}
fn source_gs_assertion_0146(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-414"
    );
}
fn source_gs_assertion_0147(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-415"
    );
}
fn source_gs_assertion_0148(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-416"
    );
}
fn source_gs_assertion_0149(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-417"
    );
}
fn source_gs_assertion_0150(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-418"
    );
}
fn source_gs_assertion_0151(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-419"
    );
}
fn source_gs_assertion_0152(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-423"
    );
}
fn source_gs_assertion_0153(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-424"
    );
}
fn source_gs_assertion_0154(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-427"
    );
}
fn source_gs_assertion_0155(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-432"
    );
}
fn source_gs_assertion_0156(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-439"
    );
}
fn source_gs_assertion_0157(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-440"
    );
}
fn source_gs_assertion_0158(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-445"
    );
}
fn source_gs_assertion_0159(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_008() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_008(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0142(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0143(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0144(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0145(gs.spec.container == "testing");source_gs_assertion_0146(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0147(gs.spec.ports[0].range == "default");source_gs_assertion_0148(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0149(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0150(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0151(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0152(gs.spec.players.is_none());source_gs_assertion_0153(gs.status.players.is_none());source_gs_assertion_0154(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0155(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Always");source_gs_assertion_0156(gs.spec.counters.is_none());source_gs_assertion_0157(gs.status.counters.is_none());source_gs_assertion_0158(gs.spec.lists.is_none());source_gs_assertion_0159(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_009() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = Some(agones_core::Eviction {
        safe: Some(agones_core::EvictionSafe::from("Never")),
    });
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0160(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-405"
    );
}
fn source_gs_assertion_0161(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-409"
    );
}
fn source_gs_assertion_0162(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-412"
    );
}
fn source_gs_assertion_0163(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-413"
    );
}
fn source_gs_assertion_0164(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-414"
    );
}
fn source_gs_assertion_0165(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-415"
    );
}
fn source_gs_assertion_0166(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-416"
    );
}
fn source_gs_assertion_0167(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-417"
    );
}
fn source_gs_assertion_0168(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-418"
    );
}
fn source_gs_assertion_0169(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-419"
    );
}
fn source_gs_assertion_0170(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-423"
    );
}
fn source_gs_assertion_0171(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-424"
    );
}
fn source_gs_assertion_0172(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-427"
    );
}
fn source_gs_assertion_0173(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-432"
    );
}
fn source_gs_assertion_0174(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-439"
    );
}
fn source_gs_assertion_0175(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-440"
    );
}
fn source_gs_assertion_0176(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-445"
    );
}
fn source_gs_assertion_0177(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_009() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_009(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0160(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0161(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0162(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0163(gs.spec.container == "testing");source_gs_assertion_0164(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0165(gs.spec.ports[0].range == "default");source_gs_assertion_0166(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0167(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0168(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0169(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0170(gs.spec.players.is_none());source_gs_assertion_0171(gs.status.players.is_none());source_gs_assertion_0172(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0173(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0174(gs.spec.counters.is_none());source_gs_assertion_0175(gs.status.counters.is_none());source_gs_assertion_0176(gs.spec.lists.is_none());source_gs_assertion_0177(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_010() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = Some(agones_core::Eviction {
        safe: Some(agones_core::EvictionSafe::from("OnUpgrade")),
    });
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0178(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-405"
    );
}
fn source_gs_assertion_0179(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-409"
    );
}
fn source_gs_assertion_0180(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-412"
    );
}
fn source_gs_assertion_0181(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-413"
    );
}
fn source_gs_assertion_0182(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-414"
    );
}
fn source_gs_assertion_0183(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-415"
    );
}
fn source_gs_assertion_0184(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-416"
    );
}
fn source_gs_assertion_0185(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-417"
    );
}
fn source_gs_assertion_0186(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-418"
    );
}
fn source_gs_assertion_0187(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-419"
    );
}
fn source_gs_assertion_0188(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-423"
    );
}
fn source_gs_assertion_0189(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-424"
    );
}
fn source_gs_assertion_0190(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-427"
    );
}
fn source_gs_assertion_0191(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-432"
    );
}
fn source_gs_assertion_0192(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-439"
    );
}
fn source_gs_assertion_0193(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-440"
    );
}
fn source_gs_assertion_0194(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-445"
    );
}
fn source_gs_assertion_0195(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_010() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_010(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0178(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0179(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0180(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0181(gs.spec.container == "testing");source_gs_assertion_0182(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0183(gs.spec.ports[0].range == "default");source_gs_assertion_0184(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0185(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0186(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0187(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0188(gs.spec.players.is_none());source_gs_assertion_0189(gs.status.players.is_none());source_gs_assertion_0190(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "OnUpgrade");source_gs_assertion_0191(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "OnUpgrade");source_gs_assertion_0192(gs.spec.counters.is_none());source_gs_assertion_0193(gs.status.counters.is_none());source_gs_assertion_0194(gs.spec.lists.is_none());source_gs_assertion_0195(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_011() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: true,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0196(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-405"
    );
}
fn source_gs_assertion_0197(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-409"
    );
}
fn source_gs_assertion_0198(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-412"
    );
}
fn source_gs_assertion_0199(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-413"
    );
}
fn source_gs_assertion_0200(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-414"
    );
}
fn source_gs_assertion_0201(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-415"
    );
}
fn source_gs_assertion_0202(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-416"
    );
}
fn source_gs_assertion_0203(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-417"
    );
}
fn source_gs_assertion_0204(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-418"
    );
}
fn source_gs_assertion_0205(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-419"
    );
}
fn source_gs_assertion_0206(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-423"
    );
}
fn source_gs_assertion_0207(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-424"
    );
}
fn source_gs_assertion_0208(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-427"
    );
}
fn source_gs_assertion_0209(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-432"
    );
}
fn source_gs_assertion_0210(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-439"
    );
}
fn source_gs_assertion_0211(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-440"
    );
}
fn source_gs_assertion_0212(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-445"
    );
}
fn source_gs_assertion_0213(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_011() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_011(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0196(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0197(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0198(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0199(gs.spec.container == "testing");source_gs_assertion_0200(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0201(gs.spec.ports[0].range == "default");source_gs_assertion_0202(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0203(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0204(gs.spec.health == agones_core::Health { disabled: true, period_seconds: 0i32, failure_threshold: 0i32, initial_delay_seconds: 0i32 });source_gs_assertion_0205(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0206(gs.spec.players.is_none());source_gs_assertion_0207(gs.status.players.is_none());source_gs_assertion_0208(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0209(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0210(gs.spec.counters.is_none());source_gs_assertion_0211(gs.status.counters.is_none());source_gs_assertion_0212(gs.spec.lists.is_none());source_gs_assertion_0213(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_012() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = Some(agones_core::Eviction {
        safe: Some(agones_core::EvictionSafe::from("Always")),
    });
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([(
        "cluster-autoscaler.kubernetes.io/safe-to-evict".to_owned(),
        "false".to_owned(),
    )]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0214(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-405"
    );
}
fn source_gs_assertion_0215(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-409"
    );
}
fn source_gs_assertion_0216(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-412"
    );
}
fn source_gs_assertion_0217(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-413"
    );
}
fn source_gs_assertion_0218(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-414"
    );
}
fn source_gs_assertion_0219(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-415"
    );
}
fn source_gs_assertion_0220(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-416"
    );
}
fn source_gs_assertion_0221(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-417"
    );
}
fn source_gs_assertion_0222(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-418"
    );
}
fn source_gs_assertion_0223(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-419"
    );
}
fn source_gs_assertion_0224(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-423"
    );
}
fn source_gs_assertion_0225(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-424"
    );
}
fn source_gs_assertion_0226(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-427"
    );
}
fn source_gs_assertion_0227(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-432"
    );
}
fn source_gs_assertion_0228(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-439"
    );
}
fn source_gs_assertion_0229(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-440"
    );
}
fn source_gs_assertion_0230(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-445"
    );
}
fn source_gs_assertion_0231(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_012() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_012(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0214(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0215(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0216(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0217(gs.spec.container == "testing");source_gs_assertion_0218(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0219(gs.spec.ports[0].range == "default");source_gs_assertion_0220(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0221(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0222(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0223(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0224(gs.spec.players.is_none());source_gs_assertion_0225(gs.status.players.is_none());source_gs_assertion_0226(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Always");source_gs_assertion_0227(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Always");source_gs_assertion_0228(gs.spec.counters.is_none());source_gs_assertion_0229(gs.status.counters.is_none());source_gs_assertion_0230(gs.spec.lists.is_none());source_gs_assertion_0231(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_013() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: Some(agones_core::SdkServerLogLevel::from("Debug")),
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0232(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-405"
    );
}
fn source_gs_assertion_0233(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-409"
    );
}
fn source_gs_assertion_0234(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-412"
    );
}
fn source_gs_assertion_0235(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-413"
    );
}
fn source_gs_assertion_0236(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-414"
    );
}
fn source_gs_assertion_0237(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-415"
    );
}
fn source_gs_assertion_0238(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-416"
    );
}
fn source_gs_assertion_0239(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-417"
    );
}
fn source_gs_assertion_0240(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-418"
    );
}
fn source_gs_assertion_0241(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-419"
    );
}
fn source_gs_assertion_0242(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-423"
    );
}
fn source_gs_assertion_0243(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-424"
    );
}
fn source_gs_assertion_0244(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-427"
    );
}
fn source_gs_assertion_0245(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-432"
    );
}
fn source_gs_assertion_0246(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-439"
    );
}
fn source_gs_assertion_0247(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-440"
    );
}
fn source_gs_assertion_0248(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-445"
    );
}
fn source_gs_assertion_0249(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_013() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_013(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0232(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0233(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0234(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0235(gs.spec.container == "testing");source_gs_assertion_0236(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0237(gs.spec.ports[0].range == "default");source_gs_assertion_0238(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0239(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0240(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0241(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Debug")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0242(gs.spec.players.is_none());source_gs_assertion_0243(gs.status.players.is_none());source_gs_assertion_0244(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0245(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0246(gs.spec.counters.is_none());source_gs_assertion_0247(gs.status.counters.is_none());source_gs_assertion_0248(gs.spec.lists.is_none());source_gs_assertion_0249(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_014() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0250(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-405"
    );
}
fn source_gs_assertion_0251(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-409"
    );
}
fn source_gs_assertion_0252(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-412"
    );
}
fn source_gs_assertion_0253(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-413"
    );
}
fn source_gs_assertion_0254(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-414"
    );
}
fn source_gs_assertion_0255(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-415"
    );
}
fn source_gs_assertion_0256(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-416"
    );
}
fn source_gs_assertion_0257(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-417"
    );
}
fn source_gs_assertion_0258(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-418"
    );
}
fn source_gs_assertion_0259(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-419"
    );
}
fn source_gs_assertion_0260(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-423"
    );
}
fn source_gs_assertion_0261(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-424"
    );
}
fn source_gs_assertion_0262(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-427"
    );
}
fn source_gs_assertion_0263(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-432"
    );
}
fn source_gs_assertion_0264(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-439"
    );
}
fn source_gs_assertion_0265(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-440"
    );
}
fn source_gs_assertion_0266(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-445"
    );
}
fn source_gs_assertion_0267(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_014() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_014(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0250(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0251(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0252(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0253(gs.spec.container == "testing");source_gs_assertion_0254(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0255(gs.spec.ports[0].range == "default");source_gs_assertion_0256(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0257(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0258(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0259(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0260(gs.spec.players.is_none());source_gs_assertion_0261(gs.status.players.is_none());source_gs_assertion_0262(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0263(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0264(gs.spec.counters.is_none());source_gs_assertion_0265(gs.status.counters.is_none());source_gs_assertion_0266(gs.spec.lists.is_none());source_gs_assertion_0267(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_015() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: Some(agones_core::PortPolicy::from("Static")),
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: None,
        grpc_port: 0i32,
        http_port: 0i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0268(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-405"
    );
}
fn source_gs_assertion_0269(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-409"
    );
}
fn source_gs_assertion_0270(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-412"
    );
}
fn source_gs_assertion_0271(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-413"
    );
}
fn source_gs_assertion_0272(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-414"
    );
}
fn source_gs_assertion_0273(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-415"
    );
}
fn source_gs_assertion_0274(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-416"
    );
}
fn source_gs_assertion_0275(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-417"
    );
}
fn source_gs_assertion_0276(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-418"
    );
}
fn source_gs_assertion_0277(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-419"
    );
}
fn source_gs_assertion_0278(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-423"
    );
}
fn source_gs_assertion_0279(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-424"
    );
}
fn source_gs_assertion_0280(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-427"
    );
}
fn source_gs_assertion_0281(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-432"
    );
}
fn source_gs_assertion_0282(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-439"
    );
}
fn source_gs_assertion_0283(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-440"
    );
}
fn source_gs_assertion_0284(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-445"
    );
}
fn source_gs_assertion_0285(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_015() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_015(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0268(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0269(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0270(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0271(gs.spec.container == "testing");source_gs_assertion_0272(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0273(gs.spec.ports[0].range == "default");source_gs_assertion_0274(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "Creating");source_gs_assertion_0275(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0276(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0277(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Info")), grpc_port: 9357i32, http_port: 9358i32 });source_gs_assertion_0278(gs.spec.players.is_none());source_gs_assertion_0279(gs.status.players.is_none());source_gs_assertion_0280(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0281(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0282(gs.spec.counters.is_none());source_gs_assertion_0283(gs.status.counters.is_none());source_gs_assertion_0284(gs.spec.lists.is_none());source_gs_assertion_0285(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
fn source_gs_fixture_016() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "".to_owned(),
        range: "".to_owned(),
        port_policy: None,
        protocol: None,
        container: None,
        container_port: 999i32,
        host_port: 0i32,
    }]
    .into();
    value.spec.health = agones_core::Health {
        disabled: false,
        period_seconds: 0i32,
        failure_threshold: 0i32,
        initial_delay_seconds: 0i32,
    };
    value.spec.sdk_server = agones_core::SdkServer {
        log_level: Some(agones_core::SdkServerLogLevel::from("Error")),
        grpc_port: 19357i32,
        http_port: 19358i32,
    };
    value.spec.scheduling = None;
    value.spec.eviction = None;
    value.spec.counters = None;
    value.spec.lists = None;
    value.spec.template.spec.containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_assertion_0286(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-405"
    );
}
fn source_gs_assertion_0287(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-409"
    );
}
fn source_gs_assertion_0288(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-412"
    );
}
fn source_gs_assertion_0289(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-413"
    );
}
fn source_gs_assertion_0290(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-414"
    );
}
fn source_gs_assertion_0291(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-415"
    );
}
fn source_gs_assertion_0292(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-416"
    );
}
fn source_gs_assertion_0293(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-417"
    );
}
fn source_gs_assertion_0294(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-418"
    );
}
fn source_gs_assertion_0295(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-419"
    );
}
fn source_gs_assertion_0296(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-423"
    );
}
fn source_gs_assertion_0297(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-424"
    );
}
fn source_gs_assertion_0298(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-427"
    );
}
fn source_gs_assertion_0299(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-432"
    );
}
fn source_gs_assertion_0300(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-439"
    );
}
fn source_gs_assertion_0301(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-440"
    );
}
fn source_gs_assertion_0302(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-445"
    );
}
fn source_gs_assertion_0303(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server::line-446"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(64))]
fn source_gs_case_016() {
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_fixture_016(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("");source_gs_assertion_0286(parsed.is_ok());let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap()); gs.apply_defaults_with_context(&context);source_gs_assertion_0287(gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some("dev"));source_gs_assertion_0288(gs.metadata.finalizers.iter().any(|value| value == "agones.dev/controller"));source_gs_assertion_0289(gs.spec.container == "testing");source_gs_assertion_0290(gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == "UDP");source_gs_assertion_0291(gs.spec.ports[0].range == "default");source_gs_assertion_0292(gs.status.state.as_ref().map_or("", |v| v.as_str()) == "PortAllocation");source_gs_assertion_0293(gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == "Packed");source_gs_assertion_0294(gs.spec.health == agones_core::Health { disabled: false, period_seconds: 5i32, failure_threshold: 3i32, initial_delay_seconds: 5i32 });source_gs_assertion_0295(gs.spec.sdk_server == agones_core::SdkServer { log_level: Some(agones_core::SdkServerLogLevel::from("Error")), grpc_port: 19357i32, http_port: 19358i32 });source_gs_assertion_0296(gs.spec.players.is_none());source_gs_assertion_0297(gs.status.players.is_none());source_gs_assertion_0298(gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0299(gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == "Never");source_gs_assertion_0300(gs.spec.counters.is_none());source_gs_assertion_0301(gs.status.counters.is_none());source_gs_assertion_0302(gs.spec.lists.is_none());source_gs_assertion_0303(gs.status.lists.is_none());#[cfg(kani)] { kani::cover!(true, "source GameServer case reached all assertions: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server"); }std::mem::forget(gs); std::mem::forget(parsed); std::mem::forget(context); });
}
#[cfg(not(kani))]
#[test]
fn source_gs_typed_fixtures_and_complete_outputs_match_capture() {
    {
        let mut value = source_gs_fixture_000();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"counters\":{\"games\":{\"capacity\":100,\"count\":1}},\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("CountsAndLists=true")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"counters\":{\"games\":{\"capacity\":100,\"count\":1}},\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"counters\":{\"games\":{\"capacity\":100,\"count\":1}},\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Counters"
        );
    }
    {
        let mut value = source_gs_fixture_001();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"lists\":{\"players\":{\"capacity\":100,\"values\":[\"foo\",\"bar\"]}},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("CountsAndLists=true")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"lists\":{\"players\":{\"capacity\":100,\"values\":[\"foo\",\"bar\"]}},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"lists\":{\"players\":{\"capacity\":100,\"values\":[\"foo\",\"bar\"]}},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::CountsAndLists=true, Lists"
        );
    }
    {
        let mut value = source_gs_fixture_002();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{\"annotations\":{\"cluster-autoscaler.kubernetes.io/safe-to-evict\":\"false\"}},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{\"annotations\":{\"cluster-autoscaler.kubernetes.io/safe-to-evict\":\"false\"}},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::Nothing inferred from safe-to-evict=false"
        );
    }
    {
        let mut value = source_gs_fixture_003();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":777,\"hostPort\":777,\"portPolicy\":\"Static\",\"protocol\":\"TCP\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":777,\"hostPort\":777,\"portPolicy\":\"Static\",\"protocol\":\"TCP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"Creating\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::convert from legacy single port to multiple"
        );
    }
    {
        let mut value = source_gs_fixture_004();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"container\":\"testing2\",\"health\":{\"failureThreshold\":10,\"initialDelaySeconds\":11,\"periodSeconds\":12},\"ports\":[{\"portPolicy\":\"Static\",\"protocol\":\"TCP\",\"range\":\"default\"}],\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}},{\"image\":\"testing/image2\",\"name\":\"testing2\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"TestState\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing2\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":10,\"initialDelaySeconds\":11,\"periodSeconds\":12},\"ports\":[{\"container\":\"testing2\",\"portPolicy\":\"Static\",\"protocol\":\"TCP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}},{\"image\":\"testing/image2\",\"name\":\"testing2\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"TestState\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are already set"
        );
    }
    {
        let mut value = source_gs_fixture_005();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults are eviction.safe: Never"
        );
    }
    {
        let mut value = source_gs_fixture_006();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":999,\"portPolicy\":\"Passthrough\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Passthrough\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::defaults on passthrough"
        );
    }
    {
        let mut value = source_gs_fixture_007();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"eviction\":{\"safe\":\"Always\"},\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Always\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Always\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always"
        );
    }
    {
        let mut value = source_gs_fixture_008();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{\"annotations\":{\"cluster-autoscaler.kubernetes.io/safe-to-evict\":\"true\"}},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{\"annotations\":{\"cluster-autoscaler.kubernetes.io/safe-to-evict\":\"true\"}},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Always\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Always inferred from safe-to-evict=true"
        );
    }
    {
        let mut value = source_gs_fixture_009();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"eviction\":{\"safe\":\"Never\"},\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: Never"
        );
    }
    {
        let mut value = source_gs_fixture_010();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"eviction\":{\"safe\":\"OnUpgrade\"},\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"OnUpgrade\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"OnUpgrade\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::eviction.safe: OnUpgrade"
        );
    }
    {
        let mut value = source_gs_fixture_011();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{\"disabled\":true},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"disabled\":true},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::health is disabled"
        );
    }
    {
        let mut value = source_gs_fixture_012();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"eviction\":{\"safe\":\"Always\"},\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{\"annotations\":{\"cluster-autoscaler.kubernetes.io/safe-to-evict\":\"false\"}},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Always\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{\"annotations\":{\"cluster-autoscaler.kubernetes.io/safe-to-evict\":\"false\"}},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Always\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::safe-to-evict=false AND eviction.safe: Always => eviction.safe: Always"
        );
    }
    {
        let mut value = source_gs_fixture_013();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{\"logLevel\":\"Debug\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Debug\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set Debug logging level"
        );
    }
    {
        let mut value = source_gs_fixture_014();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on a very simple gameserver"
        );
    }
    {
        let mut value = source_gs_fixture_015();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":999,\"portPolicy\":\"Static\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Static\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"Creating\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set basic defaults on static gameserver"
        );
    }
    {
        let mut value = source_gs_fixture_016();
        let input: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":999}],\"sdkServer\":{\"grpcPort\":19357,\"httpPort\":19358,\"logLevel\":\"Error\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
        let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
        assert_eq!(
            value, decoded,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server"
        );
        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            input,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server"
        );
        let parsed = agones_core::FeatureGateContext::default()
            .parse_feature_gates("")
            .unwrap();
        value.apply_defaults_with_context(
            &agones_core::GameServerDefaultingContext::from_feature_gates(&parsed),
        );
        let expected: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"testing\",\"containerPort\":999,\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":19357,\"httpPort\":19358,\"logLevel\":\"Error\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            expected,
            "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerApplyDefaults::set gRPC and HTTP ports on SDK Server"
        );
    }
}
