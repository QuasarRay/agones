// Generated from gated typed validation IR by feature_case_codegen --gameserver-features. Do not edit.
fn source_gsv_errors_match(
    actual: &[agones_core::ValidationError],
    expected: &[agones_core::ValidationError],
) -> bool {
    actual.len() == expected.len()
        && expected.iter().all(|error| {
            actual.iter().filter(|value| *value == error).count()
                == expected.iter().filter(|value| *value == error).count()
        })
}
fn source_gsv_base_fixture_000() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.name = "dev-game".to_owned();
    value.metadata.namespace = "default".to_owned();
    value.spec.container = "testing".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "main".to_owned(),
        range: "".to_owned(),
        port_policy: Some(agones_core::PortPolicy::from("Dynamic")),
        protocol: None,
        container: Some("another-container".to_owned()),
        container_port: 7777i32,
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
fn source_gsv_fixture_000() -> agones_core::GameServer {
    let mut value = source_gsv_base_fixture_000();
    value
}
fn source_gsv_assertion_000(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::Invalid container name, container was not found::line-1479"
    );
}
fn source_gsv_assertion_001(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::Invalid container name, container was not found::line-1482"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gsv_case_000() {
    use agones_core::CanParseFeatureGates;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let gs = source_gsv_fixture_000(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates(""); source_gsv_assertion_000(parsed.is_ok()); let gates = parsed.unwrap(); let context = agones_core::GameServerDefaultingContext::from_feature_gates(&gates); let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.ports[0].container".to_owned(), bad_value: Some("another-container".to_owned()), detail: "Container must be empty or the name of a container in the pod template".to_owned() },]; source_gsv_assertion_001(source_gsv_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::Invalid container name, container was not found"); } std::mem::forget((gs, gates, context, expected, actual)); });
}
fn source_gsv_base_fixture_001() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.name = "dev-game".to_owned();
    value.metadata.namespace = "default".to_owned();
    value.spec.container = "testing".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "main".to_owned(),
        range: "".to_owned(),
        port_policy: Some(agones_core::PortPolicy::from("Dynamic")),
        protocol: None,
        container: None,
        container_port: 7777i32,
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
fn source_gsv_fixture_001() -> agones_core::GameServer {
    let mut value = source_gsv_base_fixture_001();
    value
}
fn source_gsv_assertion_002(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::Multiple container ports, OK scenario::line-1479"
    );
}
fn source_gsv_assertion_003(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::Multiple container ports, OK scenario::line-1482"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gsv_case_001() {
    use agones_core::CanParseFeatureGates;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let gs = source_gsv_fixture_001(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates(""); source_gsv_assertion_002(parsed.is_ok()); let gates = parsed.unwrap(); let context = agones_core::GameServerDefaultingContext::from_feature_gates(&gates); let actual = gs.validate_with_context(&context); let expected = vec![]; source_gsv_assertion_003(source_gsv_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::Multiple container ports, OK scenario"); } std::mem::forget((gs, gates, context, expected, actual)); });
}
fn source_gsv_base_fixture_002() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "testing".to_owned();
    value.spec.ports = agones_core::GoSlice::default();
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
fn source_gsv_fixture_002() -> agones_core::GameServer {
    let mut value = source_gsv_base_fixture_002();
    value.spec.counters = Some(std::collections::BTreeMap::new());
    value
}
fn source_gsv_assertion_004(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::CountsAndLists is enabled, Counters field specified::line-1479"
    );
}
fn source_gsv_assertion_005(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::CountsAndLists is enabled, Counters field specified::line-1482"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gsv_case_002() {
    use agones_core::CanParseFeatureGates;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let gs = source_gsv_fixture_002(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("CountsAndLists=true"); source_gsv_assertion_004(parsed.is_ok()); let gates = parsed.unwrap(); let context = agones_core::GameServerDefaultingContext::from_feature_gates(&gates); let actual = gs.validate_with_context(&context); let expected = vec![]; source_gsv_assertion_005(source_gsv_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::CountsAndLists is enabled, Counters field specified"); } std::mem::forget((gs, gates, context, expected, actual)); });
}
fn source_gsv_base_fixture_003() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "testing".to_owned();
    value.spec.ports = agones_core::GoSlice::default();
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
fn source_gsv_fixture_003() -> agones_core::GameServer {
    let mut value = source_gsv_base_fixture_003();
    value.spec.lists = Some(std::collections::BTreeMap::new());
    value
}
fn source_gsv_assertion_006(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::CountsAndLists is enabled, Lists field specified::line-1479"
    );
}
fn source_gsv_assertion_007(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::CountsAndLists is enabled, Lists field specified::line-1482"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gsv_case_003() {
    use agones_core::CanParseFeatureGates;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let gs = source_gsv_fixture_003(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates("CountsAndLists=true"); source_gsv_assertion_006(parsed.is_ok()); let gates = parsed.unwrap(); let context = agones_core::GameServerDefaultingContext::from_feature_gates(&gates); let actual = gs.validate_with_context(&context); let expected = vec![]; source_gsv_assertion_007(source_gsv_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::CountsAndLists is enabled, Lists field specified"); } std::mem::forget((gs, gates, context, expected, actual)); });
}
fn source_gsv_base_fixture_004() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "testing".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "main".to_owned(),
        range: "".to_owned(),
        port_policy: Some(agones_core::PortPolicy::from("None")),
        protocol: None,
        container: None,
        container_port: 7777i32,
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
fn source_gsv_fixture_004() -> agones_core::GameServer {
    let mut value = source_gsv_base_fixture_004();
    value.spec.lists = Some(std::collections::BTreeMap::new());
    value
}
fn source_gsv_assertion_008(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::PortPolicyNone is enabled, PortPolicy field set to None::line-1479"
    );
}
fn source_gsv_assertion_009(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::PortPolicyNone is enabled, PortPolicy field set to None::line-1482"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gsv_case_004() {
    use agones_core::CanParseFeatureGates;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let gs = source_gsv_fixture_004(); let parsed = agones_core::FeatureGateContext::default().parse_feature_gates(""); source_gsv_assertion_008(parsed.is_ok()); let gates = parsed.unwrap(); let context = agones_core::GameServerDefaultingContext::from_feature_gates(&gates); let actual = gs.validate_with_context(&context); let expected = vec![]; source_gsv_assertion_009(source_gsv_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidateFeatures::PortPolicyNone is enabled, PortPolicy field set to None"); } std::mem::forget((gs, gates, context, expected, actual)); });
}
#[cfg(not(kani))]
#[test]
fn source_gsv_multiset_relation_preserves_elements_and_multiplicity() {
    let first = agones_core::ValidationError {
        kind: agones_core::ValidationErrorKind::Invalid,
        path: "spec.first".into(),
        bad_value: None,
        detail: "detail".into(),
    };
    let mut second = first.clone();
    second.path = "spec.second".into();
    assert!(source_gsv_errors_match(
        &[first.clone(), second.clone()],
        &[second.clone(), first.clone()]
    ));
    assert!(!source_gsv_errors_match(
        &[first.clone(), first.clone()],
        &[first.clone(), second.clone()]
    ));
    assert!(!source_gsv_errors_match(&[], &[first.clone()]));
    for field in 0..4 {
        let mut wrong = first.clone();
        match field {
            0 => wrong.kind = agones_core::ValidationErrorKind::Forbidden,
            1 => wrong.path = "other".into(),
            2 => wrong.bad_value = Some("bad".into()),
            _ => wrong.detail = "other".into(),
        }
        assert!(!source_gsv_errors_match(&[first.clone()], &[wrong]));
    }
    let mut third = first.clone();
    third.detail = "third class".into();
    let records = [first, second, third];
    let mut lists = vec![Vec::<usize>::new()];
    for length in 1..=3 {
        for encoded in 0..3usize.pow(length) {
            let mut value = encoded;
            let ids = (0..length)
                .map(|_| {
                    let id = value % 3;
                    value /= 3;
                    id
                })
                .collect::<Vec<_>>();
            lists.push(ids);
        }
    }
    assert_eq!(lists.len(), 40);
    for actual_ids in &lists {
        for expected_ids in &lists {
            let mut actual_counts = [0usize; 3];
            let mut expected_counts = [0usize; 3];
            for id in actual_ids {
                actual_counts[*id] += 1;
            }
            for id in expected_ids {
                expected_counts[*id] += 1;
            }
            let actual = actual_ids
                .iter()
                .map(|id| records[*id].clone())
                .collect::<Vec<_>>();
            let expected = expected_ids
                .iter()
                .map(|id| records[*id].clone())
                .collect::<Vec<_>>();
            assert_eq!(
                source_gsv_errors_match(&actual, &expected),
                actual_counts == expected_counts
            );
        }
    }
}
#[cfg(not(kani))]
#[test]
fn source_gsv_whole_fixture_000() {
    let value = source_gsv_fixture_000();
    let raw: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"container\":\"another-container\",\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), raw);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert_eq!(value.spec.players.is_some(), false);
    assert_eq!(value.spec.ports.is_nil(), false);
}
#[cfg(not(kani))]
#[test]
fn source_gsv_whole_fixture_001() {
    let value = source_gsv_fixture_001();
    let raw: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), raw);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert_eq!(value.spec.players.is_some(), false);
    assert_eq!(value.spec.ports.is_nil(), false);
}
#[cfg(not(kani))]
#[test]
fn source_gsv_whole_fixture_002() {
    let value = source_gsv_fixture_002();
    let raw: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"container\":\"testing\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), raw);
    assert_eq!(value.spec.counters.is_some(), true);
    assert_eq!(value.spec.lists.is_some(), false);
    assert_eq!(value.spec.players.is_some(), false);
    assert_eq!(value.spec.ports.is_nil(), true);
}
#[cfg(not(kani))]
#[test]
fn source_gsv_whole_fixture_003() {
    let value = source_gsv_fixture_003();
    let raw: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"container\":\"testing\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), raw);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), true);
    assert_eq!(value.spec.players.is_some(), false);
    assert_eq!(value.spec.ports.is_nil(), true);
}
#[cfg(not(kani))]
#[test]
fn source_gsv_whole_fixture_004() {
    let value = source_gsv_fixture_004();
    let raw: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"None\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), raw);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), true);
    assert_eq!(value.spec.players.is_some(), false);
    assert_eq!(value.spec.ports.is_nil(), false);
}
