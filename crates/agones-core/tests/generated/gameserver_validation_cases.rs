// Generated from gated typed TestGameServerValidate IR. Do not edit.
fn source_gs_validate_errors_match(
    actual: &[agones_core::ValidationError],
    expected: &[agones_core::ValidationError],
) -> bool {
    match expected {
        [] => actual.is_empty(),
        [error] => matches!(actual, [value] if value == error),
        _ => {
            actual.len() == expected.len()
                && expected.iter().all(|error| {
                    actual.iter().filter(|value| *value == error).count()
                        == expected.iter().filter(|value| *value == error).count()
                })
        }
    }
}
fn source_gs_validate_base_000() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
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
fn source_gs_validate_fixture_000() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_000();
    value
}
fn source_gs_validate_assertion_000(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Valid game server::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_000() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_000(); let context = agones_core::GameServerDefaultingContext::default(); if true { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![]; source_gs_validate_assertion_000(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Valid game server"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_001() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.spec.container = "".to_owned();
    value.spec.ports = vec![
        agones_core::GameServerPort {
            name: "main".to_owned(),
            range: "".to_owned(),
            port_policy: Some(agones_core::PortPolicy::from("Dynamic")),
            protocol: None,
            container: None,
            container_port: 0i32,
            host_port: 5001i32,
        },
        agones_core::GameServerPort {
            name: "sidecar".to_owned(),
            range: "".to_owned(),
            port_policy: Some(agones_core::PortPolicy::from("Static")),
            protocol: None,
            container: None,
            container_port: 5002i32,
            host_port: 5002i32,
        },
    ]
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
    value.spec.template.spec.containers = vec![
        agones_core::Container {
            name: "testing".to_owned(),
            image: "testing/image".to_owned(),
            ..Default::default()
        },
        agones_core::Container {
            name: "anothertest".to_owned(),
            image: "testing/image".to_owned(),
            ..Default::default()
        },
    ]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_001() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_001();
    value
}
fn source_gs_validate_assertion_001(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Invalid gs: container, containerPort, hostPort::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_001() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_001(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Required, path: "spec.container".to_owned(), bad_value: None, detail: "Container is required when using multiple containers in the pod template".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.container".to_owned(), bad_value: Some("".to_owned()), detail: "Could not find a container named ".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Required, path: "spec.ports[0].containerPort".to_owned(), bad_value: None, detail: "ContainerPort must be defined for Dynamic and Static PortPolicies".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Forbidden, path: "spec.ports[0].hostPort".to_owned(), bad_value: None, detail: "HostPort cannot be specified with a Dynamic or Passthrough PortPolicy".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_001(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Invalid gs: container, containerPort, hostPort"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_002() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.name = "dev-game".to_owned();
    value.metadata.namespace = "default".to_owned();
    value.metadata.annotations = std::collections::BTreeMap::from([(
        "agones.dev/dev-address".to_owned(),
        "invalid-ip".to_owned(),
    )]);
    value.spec.container = "".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "main".to_owned(),
        range: "".to_owned(),
        port_policy: Some(agones_core::PortPolicy::from("Static")),
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
    value.spec.template.spec.containers = agones_core::GoSlice::default();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_002() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_002();
    value
}
fn source_gs_validate_assertion_002(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::DevAddressAnnotation: Invalid IP, no host port::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_002() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_002(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "metadata.annotations.agones.dev/dev-address".to_owned(), bad_value: Some("invalid-ip".to_owned()), detail: "must be a valid IP address".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Required, path: "spec.ports[0].hostPort".to_owned(), bad_value: None, detail: "agones.dev/dev-address".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_002(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::DevAddressAnnotation: Invalid IP, no host port"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_003() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.name =
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned();
    value.type_meta.kind = "test-kind".to_owned();
    value.spec.container = "my_image".to_owned();
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
        name: "my_image".to_owned(),
        image: "foo/my_image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_003() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_003();
    value
}
fn source_gs_validate_assertion_003(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long gs name::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_003() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_003(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::TooLong, path: "metadata.name".to_owned(), bad_value: Some("<value omitted>".to_owned()), detail: "may not be more than 63 bytes".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_003(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long gs name"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_004() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.generate_name =
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned();
    value.type_meta.kind = "test-kind".to_owned();
    value.spec.container = "my_image".to_owned();
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
        name: "my_image".to_owned(),
        image: "foo/my_image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_004() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_004();
    value
}
fn source_gs_validate_assertion_004(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long gs GenerateName is not validated on agones side::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_004() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_004(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![]; source_gs_validate_assertion_004(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long gs GenerateName is not validated on agones side"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_005() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.generate_name =
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned();
    value.type_meta.kind = "test-kind".to_owned();
    value.spec.container = "my_image".to_owned();
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
        name: "my_image".to_owned(),
        image: "foo/my_image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.spec.template.metadata.labels = std::collections::BTreeMap::from([(
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
        "".to_owned(),
    )]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_005() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_005();
    value
}
fn source_gs_validate_assertion_005(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long label key is invalid::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_005() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_005(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.metadata.labels".to_owned(), bad_value: Some("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned()), detail: "name part must be no more than 63 bytes".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_005(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long label key is invalid"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_006() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.generate_name = "ok-name".to_owned();
    value.type_meta.kind = "test-kind".to_owned();
    value.spec.container = "my_image".to_owned();
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
        name: "my_image".to_owned(),
        image: "foo/my_image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.spec.template.metadata.labels = std::collections::BTreeMap::from([(
        "agones.dev/longValueKey".to_owned(),
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
    )]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_006() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_006();
    value
}
fn source_gs_validate_assertion_006(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long label value is invalid::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_006() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_006(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.metadata.labels".to_owned(), bad_value: Some("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned()), detail: "must be no more than 63 bytes".to_owned(), origin: "format=k8s-label-value".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_006(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long label value is invalid"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_007() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.generate_name = "ok-name".to_owned();
    value.type_meta.kind = "test-kind".to_owned();
    value.spec.container = "my_image".to_owned();
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
        name: "my_image".to_owned(),
        image: "foo/my_image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([(
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
    )]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_007() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_007();
    value
}
fn source_gs_validate_assertion_007(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long annotation key is invalid::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_007() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_007(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.metadata.annotations".to_owned(), bad_value: Some("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned()), detail: "name part must be no more than 63 bytes".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_007(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Long annotation key is invalid"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_008() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.generate_name = "ok-name".to_owned();
    value.type_meta.kind = "test-kind".to_owned();
    value.spec.container = "my_image".to_owned();
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
        name: "my_image".to_owned(),
        image: "foo/my_image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([(
        "agones.dev/short±Name".to_owned(),
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
    )]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_008() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_008();
    value
}
fn source_gs_validate_assertion_008(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Invalid character in annotation key::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_008() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_008(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.metadata.annotations".to_owned(), bad_value: Some("agones.dev/short±Name".to_owned()), detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_008(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Invalid character in annotation key"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_009() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.generate_name = "ok-name".to_owned();
    value.type_meta.kind = "test-kind".to_owned();
    value.spec.container = "my_image".to_owned();
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
        name: "my_image".to_owned(),
        image: "foo/my_image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([(
        "agones.dev/shortName".to_owned(),
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
    )]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_009() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_009();
    value
}
fn source_gs_validate_assertion_009(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Valid annotation key::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_009() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_009(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![]; source_gs_validate_assertion_009(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Valid annotation key"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_010() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.generate_name = "ok-name".to_owned();
    value.type_meta.kind = "test-kind".to_owned();
    value.spec.container = "my_image".to_owned();
    value.spec.ports = vec![
        agones_core::GameServerPort {
            name: "one".to_owned(),
            range: "".to_owned(),
            port_policy: Some(agones_core::PortPolicy::from("Passthrough")),
            protocol: None,
            container: None,
            container_port: 1294i32,
            host_port: 0i32,
        },
        agones_core::GameServerPort {
            name: "two".to_owned(),
            range: "".to_owned(),
            port_policy: Some(agones_core::PortPolicy::from("Passthrough")),
            protocol: None,
            container: None,
            container_port: 0i32,
            host_port: 7890i32,
        },
        agones_core::GameServerPort {
            name: "three".to_owned(),
            range: "".to_owned(),
            port_policy: Some(agones_core::PortPolicy::from("Dynamic")),
            protocol: None,
            container: None,
            container_port: 1294i32,
            host_port: 7890i32,
        },
    ]
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
        name: "my_image".to_owned(),
        image: "foo/my_image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_010() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_010();
    value
}
fn source_gs_validate_assertion_010(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Check ContainerPort and HostPort with different policies::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_010() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_010(); let context = agones_core::GameServerDefaultingContext::default(); if true { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Required, path: "spec.ports[0].containerPort".to_owned(), bad_value: None, detail: "ContainerPort cannot be specified with Passthrough PortPolicy".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Forbidden, path: "spec.ports[1].hostPort".to_owned(), bad_value: None, detail: "HostPort cannot be specified with a Dynamic or Passthrough PortPolicy".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Forbidden, path: "spec.ports[2].hostPort".to_owned(), bad_value: None, detail: "HostPort cannot be specified with a Dynamic or Passthrough PortPolicy".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_010(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Check ContainerPort and HostPort with different policies"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_011() -> agones_core::GameServer {
    let mut value = agones_core::GameServer::default();
    value.metadata.name = "dev-game".to_owned();
    value.metadata.namespace = "default".to_owned();
    value.metadata.annotations = std::collections::BTreeMap::from([(
        "agones.dev/dev-address".to_owned(),
        "127.1.1.1".to_owned(),
    )]);
    value.spec.container = "my_image".to_owned();
    value.spec.ports = vec![agones_core::GameServerPort {
        name: "main".to_owned(),
        range: "".to_owned(),
        port_policy: Some(agones_core::PortPolicy::from("Passthrough")),
        protocol: None,
        container: None,
        container_port: 7777i32,
        host_port: 7890i32,
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
        name: "my_image".to_owned(),
        image: "foo/my_image".to_owned(),
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_011() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_011();
    value
}
fn source_gs_validate_assertion_011(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::PortPolicy must be Static with HostPort specified::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_011() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_011(); let context = agones_core::GameServerDefaultingContext::default(); if true { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Required, path: "spec.ports[0].portPolicy".to_owned(), bad_value: None, detail: "PortPolicy must be Static".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_011(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::PortPolicy must be Static with HostPort specified"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_012() -> agones_core::GameServer {
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
        container_port: -4i32,
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
fn source_gs_validate_fixture_012() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_012();
    value
}
fn source_gs_validate_assertion_012(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::ContainerPort is less than zero::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_012() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_012(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Required, path: "spec.ports[0].containerPort".to_owned(), bad_value: None, detail: "ContainerPort must be defined for Dynamic and Static PortPolicies".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_012(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::ContainerPort is less than zero"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_013() -> agones_core::GameServer {
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
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "50m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_013() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_013();
    value
}
fn source_gs_validate_assertion_013(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::CPU Request > Limit::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_013() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_013(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.containers[0].resources.requests".to_owned(), bad_value: Some("50m".to_owned()), detail: "must be less than or equal to cpu limit of 30m".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_013(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::CPU Request > Limit"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_014() -> agones_core::GameServer {
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
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "-30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_014() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_014();
    value
}
fn source_gs_validate_assertion_014(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::CPU negative request::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_014() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_014(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.containers[0].resources.requests[cpu]".to_owned(), bad_value: Some("-30m".to_owned()), detail: "must be greater than or equal to 0".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_014(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::CPU negative request"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_015() -> agones_core::GameServer {
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
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "-30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_015() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_015();
    value
}
fn source_gs_validate_assertion_015(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::CPU negative limit::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_015() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_015(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.containers[0].resources.limits[cpu]".to_owned(), bad_value: Some("-30m".to_owned()), detail: "must be greater than or equal to 0".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.containers[0].resources.requests".to_owned(), bad_value: Some("30m".to_owned()), detail: "must be less than or equal to cpu limit of -30m".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_015(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::CPU negative limit"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_016() -> agones_core::GameServer {
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
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "55Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_016() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_016();
    value
}
fn source_gs_validate_assertion_016(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Memory Request > Limit::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_016() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_016(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.containers[0].resources.requests".to_owned(), bad_value: Some("55Mi".to_owned()), detail: "must be less than or equal to memory limit of 32Mi".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_016(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Memory Request > Limit"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_017() -> agones_core::GameServer {
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
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "-32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_017() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_017();
    value
}
fn source_gs_validate_assertion_017(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Memory negative request::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_017() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_017(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.containers[0].resources.requests[memory]".to_owned(), bad_value: Some("-32Mi".to_owned()), detail: "must be greater than or equal to 0".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_017(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Memory negative request"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_018() -> agones_core::GameServer {
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
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "-32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }]
    .into();
    value.spec.template.metadata.annotations = std::collections::BTreeMap::from([]);
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_018() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_018();
    value
}
fn source_gs_validate_assertion_018(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Memory negative limit::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_018() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_018(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.containers[0].resources.limits[memory]".to_owned(), bad_value: Some("-32Mi".to_owned()), detail: "must be greater than or equal to 0".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.containers[0].resources.requests".to_owned(), bad_value: Some("32Mi".to_owned()), detail: "must be less than or equal to memory limit of -32Mi".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_018(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Memory negative limit"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_019() -> agones_core::GameServer {
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
    value.spec.template.spec.init_containers = vec![agones_core::Container {
        name: "sidecar".to_owned(),
        image: "testing/image".to_owned(),
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "50m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }];
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_019() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_019();
    value
}
fn source_gs_validate_assertion_019(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init CPU Request > Limit::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_019() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_019(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.initContainers[0].resources.requests".to_owned(), bad_value: Some("50m".to_owned()), detail: "must be less than or equal to cpu limit of 30m".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_019(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init CPU Request > Limit"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_020() -> agones_core::GameServer {
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
    value.spec.template.spec.init_containers = vec![agones_core::Container {
        name: "sidecar".to_owned(),
        image: "testing/image".to_owned(),
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "-30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }];
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_020() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_020();
    value
}
fn source_gs_validate_assertion_020(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init CPU negative request::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_020() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_020(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.initContainers[0].resources.requests[cpu]".to_owned(), bad_value: Some("-30m".to_owned()), detail: "must be greater than or equal to 0".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_020(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init CPU negative request"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_021() -> agones_core::GameServer {
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
    value.spec.template.spec.init_containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "-30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }];
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_021() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_021();
    value
}
fn source_gs_validate_assertion_021(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init CPU negative limit::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_021() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_021(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.initContainers[0].resources.limits[cpu]".to_owned(), bad_value: Some("-30m".to_owned()), detail: "must be greater than or equal to 0".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.initContainers[0].resources.requests".to_owned(), bad_value: Some("30m".to_owned()), detail: "must be less than or equal to cpu limit of -30m".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_021(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init CPU negative limit"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_022() -> agones_core::GameServer {
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
    value.spec.template.spec.init_containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "55Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }];
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_022() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_022();
    value
}
fn source_gs_validate_assertion_022(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init Memory Request > Limit::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_022() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_022(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.initContainers[0].resources.requests".to_owned(), bad_value: Some("55Mi".to_owned()), detail: "must be less than or equal to memory limit of 32Mi".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_022(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init Memory Request > Limit"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_023() -> agones_core::GameServer {
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
    value.spec.template.spec.init_containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "-32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }];
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_023() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_023();
    value
}
fn source_gs_validate_assertion_023(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init Memory negative request::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_023() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_023(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.initContainers[0].resources.requests[memory]".to_owned(), bad_value: Some("-32Mi".to_owned()), detail: "must be greater than or equal to 0".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_023(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init Memory negative request"); } std::mem::forget((gs, context, actual, expected)); });
}
fn source_gs_validate_base_024() -> agones_core::GameServer {
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
    value.spec.template.spec.init_containers = vec![agones_core::Container {
        name: "testing".to_owned(),
        image: "testing/image".to_owned(),
        resources: agones_core::ContainerResourceRequirements {
            limits: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "-32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            requests: Some({
                let mut values = std::collections::BTreeMap::new();
                values.insert(
                    "cpu".to_owned(),
                    "30m".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values.insert(
                    "memory".to_owned(),
                    "32Mi".parse::<agones_core::ResourceQuantity>().unwrap(),
                );
                values
            }),
            ..Default::default()
        },
        ..Default::default()
    }];
    value.status.state = None;
    value
}
fn source_gs_validate_fixture_024() -> agones_core::GameServer {
    let mut value = source_gs_validate_base_024();
    value
}
fn source_gs_validate_assertion_024(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init Memory negative limit::line-1355"
    );
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_gs_validate_case_024() {
    use agones_core::CanApplyGameServerDefaults;
    bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { let mut gs = source_gs_validate_fixture_024(); let context = agones_core::GameServerDefaultingContext::default(); if false { context.apply_game_server_defaults(&mut gs); } let actual = gs.validate_with_context(&context); let expected = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.initContainers[0].resources.limits[memory]".to_owned(), bad_value: Some("-32Mi".to_owned()), detail: "must be greater than or equal to 0".to_owned(), origin: "".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.template.spec.initContainers[0].resources.requests".to_owned(), bad_value: Some("32Mi".to_owned()), detail: "must be less than or equal to memory limit of -32Mi".to_owned(), origin: "".to_owned(), covered_by_declarative: false },]; source_gs_validate_assertion_024(source_gs_validate_errors_match(&actual, &expected)); #[cfg(kani)] { kani::cover!(true, "source GameServer Validate case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/gameserver_test.go::TestGameServerValidate::Init Memory negative limit"); } std::mem::forget((gs, context, actual, expected)); });
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_000() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_000();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), true);
    if true {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"]},\"spec\":{\"container\":\"testing\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"Creating\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_001() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_001();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"hostPort\":5001,\"name\":\"main\",\"portPolicy\":\"Dynamic\"},{\"containerPort\":5002,\"hostPort\":5002,\"name\":\"sidecar\",\"portPolicy\":\"Static\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}},{\"image\":\"testing/image\",\"name\":\"anothertest\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{},\"spec\":{\"health\":{},\"ports\":[{\"hostPort\":5001,\"name\":\"main\",\"portPolicy\":\"Dynamic\"},{\"containerPort\":5002,\"hostPort\":5002,\"name\":\"sidecar\",\"portPolicy\":\"Static\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}},{\"image\":\"testing/image\",\"name\":\"anothertest\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_002() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_002();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/dev-address\":\"invalid-ip\"},\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Static\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":null}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/dev-address\":\"invalid-ip\"},\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Static\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":null}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_003() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_003();
    let before: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"name\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), true);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"name\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_004() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_004();
    let before: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), true);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_005() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_005();
    let before: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"labels\":{\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\":\"\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), true);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"labels\":{\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\":\"\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_006() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_006();
    let before: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"labels\":{\"agones.dev/longValueKey\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), true);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"labels\":{\"agones.dev/longValueKey\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_007() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_007();
    let before: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"annotations\":{\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), true);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"annotations\":{\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_008() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_008();
    let before: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"annotations\":{\"agones.dev/short±Name\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), true);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"annotations\":{\"agones.dev/short±Name\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_009() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_009();
    let before: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"annotations\":{\"agones.dev/shortName\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), true);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"sdkServer\":{},\"template\":{\"metadata\":{\"annotations\":{\"agones.dev/shortName\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"}},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_010() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_010();
    let before: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"ports\":[{\"containerPort\":1294,\"name\":\"one\",\"portPolicy\":\"Passthrough\"},{\"hostPort\":7890,\"name\":\"two\",\"portPolicy\":\"Passthrough\"},{\"containerPort\":1294,\"hostPort\":7890,\"name\":\"three\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if true {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"kind\":\"test-kind\",\"metadata\":{\"annotations\":{\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"],\"generateName\":\"ok-name\"},\"spec\":{\"container\":\"my_image\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"my_image\",\"containerPort\":1294,\"name\":\"one\",\"portPolicy\":\"Passthrough\",\"protocol\":\"UDP\",\"range\":\"default\"},{\"container\":\"my_image\",\"hostPort\":7890,\"name\":\"two\",\"portPolicy\":\"Passthrough\",\"protocol\":\"UDP\",\"range\":\"default\"},{\"container\":\"my_image\",\"containerPort\":1294,\"hostPort\":7890,\"name\":\"three\",\"portPolicy\":\"Dynamic\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_011() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_011();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/dev-address\":\"127.1.1.1\"},\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"my_image\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"hostPort\":7890,\"name\":\"main\",\"portPolicy\":\"Passthrough\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if true {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"annotations\":{\"agones.dev/dev-address\":\"127.1.1.1\",\"agones.dev/sdk-version\":\"dev\"},\"finalizers\":[\"agones.dev/controller\"],\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"my_image\",\"eviction\":{\"safe\":\"Never\"},\"health\":{\"failureThreshold\":3,\"initialDelaySeconds\":5,\"periodSeconds\":5},\"ports\":[{\"container\":\"my_image\",\"containerPort\":7777,\"hostPort\":7890,\"name\":\"main\",\"portPolicy\":\"Passthrough\",\"protocol\":\"UDP\",\"range\":\"default\"}],\"scheduling\":\"Packed\",\"sdkServer\":{\"grpcPort\":9357,\"httpPort\":9358,\"logLevel\":\"Info\"},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"foo/my_image\",\"name\":\"my_image\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"eviction\":{\"safe\":\"Never\"},\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"PortAllocation\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_012() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_012();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":-4,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":-4,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_013() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_013();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"50m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"50m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_014() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_014();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"-30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"-30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_015() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_015();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"-30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"-30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_016() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_016();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"55Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"55Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_017() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_017();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"-32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"-32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_018() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_018();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"-32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"-32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_019() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_019();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"sidecar\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"50m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"sidecar\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"50m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_020() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_020();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"sidecar\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"-30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"sidecar\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"-30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_021() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_021();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"-30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"-30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_022() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_022();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"55Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"55Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_023() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_023();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"-32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"-32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_whole_fixture_024() {
    use agones_core::CanApplyGameServerDefaults;
    let mut value = source_gs_validate_fixture_024();
    let before: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"-32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), before);
    assert_eq!(value.spec.counters.is_some(), false);
    assert_eq!(value.spec.lists.is_some(), false);
    assert!(!value.spec.players.is_some());
    assert_eq!(value.spec.ports.is_nil(), false);
    if false {
        agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value);
    }
    let after: serde_json::Value = serde_json::from_str("{\"metadata\":{\"name\":\"dev-game\",\"namespace\":\"default\"},\"spec\":{\"container\":\"testing\",\"health\":{},\"ports\":[{\"containerPort\":7777,\"name\":\"main\",\"portPolicy\":\"Dynamic\"}],\"sdkServer\":{},\"template\":{\"metadata\":{},\"spec\":{\"containers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{}}],\"initContainers\":[{\"image\":\"testing/image\",\"name\":\"testing\",\"resources\":{\"limits\":{\"cpu\":\"30m\",\"memory\":\"-32Mi\"},\"requests\":{\"cpu\":\"30m\",\"memory\":\"32Mi\"}}}]}}},\"status\":{\"address\":\"\",\"addresses\":null,\"nodeName\":\"\",\"players\":null,\"ports\":null,\"reservedUntil\":null,\"state\":\"\"}}").unwrap();
    assert_eq!(serde_json::to_value(&value).unwrap(), after);
}
#[cfg(not(kani))]
#[test]
fn source_gs_validate_relation_rejects_mutated_error_elements_and_multiplicity() {
    let error = agones_core::ValidationError {
        kind: agones_core::ValidationErrorKind::Invalid,
        path: "spec".into(),
        bad_value: Some("value".into()),
        detail: "detail".into(),
        origin: String::new(),
        covered_by_declarative: false,
    };
    assert!(source_gs_validate_errors_match(
        &[error.clone()],
        &[error.clone()]
    ));
    assert!(!source_gs_validate_errors_match(
        &[error.clone(), error.clone()],
        &[error.clone()]
    ));
    for field in 0..6 {
        let mut wrong = error.clone();
        match field {
            0 => wrong.kind = agones_core::ValidationErrorKind::Forbidden,
            1 => wrong.path = "other".into(),
            2 => wrong.bad_value = None,
            3 => wrong.detail = "other".into(),
            4 => wrong.origin = "format=other".into(),
            _ => wrong.covered_by_declarative = true,
        }
        assert!(!source_gs_validate_errors_match(&[error.clone()], &[wrong]));
    }
}
