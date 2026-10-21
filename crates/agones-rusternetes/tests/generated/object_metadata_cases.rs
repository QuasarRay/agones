// Generated original AllocationOverflow cases through actual Rusternetes conversions and validation; do not edit.
fn source_metadata_assertion_000(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::empty::line-82"
    );
}
fn source_metadata_assertion_001(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad label name::line-82"
    );
}
fn source_metadata_assertion_002(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad label name::line-84::error-index-0"
    );
}
fn source_metadata_assertion_003(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad label name::line-85::error-index-0"
    );
}
fn source_metadata_assertion_004(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad label value::line-82"
    );
}
fn source_metadata_assertion_005(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad label value::line-84::error-index-0"
    );
}
fn source_metadata_assertion_006(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad label value::line-85::error-index-0"
    );
}
fn source_metadata_assertion_007(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad annotation name::line-82"
    );
}
fn source_metadata_assertion_008(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad annotation name::line-84::error-index-0"
    );
}
fn source_metadata_assertion_009(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad annotation name::line-85::error-index-0"
    );
}
fn source_metadata_assertion_010(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::valid full::line-82"
    );
}
#[test]
fn source_metadata_rusternetes_overflow_000() {
    use agones_rusternetes::RusternetesCustomResource;
    let input = agones_core::AllocationOverflow::default();
    let reference: Vec<agones_core::ValidationError> = vec![];
    let mut game = agones_core::GameServer::default();
    game.spec.template.spec.containers = vec![agones_core::Container {
        name: "game".into(),
        image: "game:v1".into(),
        ..Default::default()
    }]
    .into();
    game.apply_defaults();
    let mut fleet = agones_core::Fleet::default();
    fleet.metadata.name = "fleet".into();
    fleet.spec.template.spec = game.spec.clone();
    fleet.spec.allocation_overflow = Some(input.clone());
    fleet.apply_defaults();
    let fleet = agones_core::Fleet::from_rusternetes_custom_resource(
        fleet.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(fleet.spec.allocation_overflow.as_ref(), Some(&input));
    let mut set = agones_core::GameServerSet::default();
    set.metadata.name = "set".into();
    set.spec.template.spec = game.spec;
    set.spec.allocation_overflow = Some(input.clone());
    let set = agones_core::GameServerSet::from_rusternetes_custom_resource(
        set.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(set.spec.allocation_overflow.as_ref(), Some(&input));
    for actual in [
        agones_rusternetes::validate_fleet(&fleet, &agones_rusternetes::NoopApiHooks).unwrap(),
        agones_rusternetes::validate_game_server_set_creation(
            &set,
            &agones_rusternetes::NoopApiHooks,
        )
        .unwrap(),
    ] {
        source_metadata_assertion_000((reference.len() == 0) && (actual.len() == 0));
        assert_eq!(actual, reference);
    }
}
#[test]
fn source_metadata_rusternetes_overflow_001() {
    use agones_rusternetes::RusternetesCustomResource;
    let mut input = agones_core::AllocationOverflow::default();
    input
        .labels
        .insert("$$$foobar".to_owned(), "stuff".to_owned());
    let reference: Vec<agones_core::ValidationError> = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.allocationOverflow.labels".to_owned(), bad_value: Some("$$$foobar".to_owned()), detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },];
    let mut game = agones_core::GameServer::default();
    game.spec.template.spec.containers = vec![agones_core::Container {
        name: "game".into(),
        image: "game:v1".into(),
        ..Default::default()
    }]
    .into();
    game.apply_defaults();
    let mut fleet = agones_core::Fleet::default();
    fleet.metadata.name = "fleet".into();
    fleet.spec.template.spec = game.spec.clone();
    fleet.spec.allocation_overflow = Some(input.clone());
    fleet.apply_defaults();
    let fleet = agones_core::Fleet::from_rusternetes_custom_resource(
        fleet.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(fleet.spec.allocation_overflow.as_ref(), Some(&input));
    let mut set = agones_core::GameServerSet::default();
    set.metadata.name = "set".into();
    set.spec.template.spec = game.spec;
    set.spec.allocation_overflow = Some(input.clone());
    let set = agones_core::GameServerSet::from_rusternetes_custom_resource(
        set.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(set.spec.allocation_overflow.as_ref(), Some(&input));
    for actual in [
        agones_rusternetes::validate_fleet(&fleet, &agones_rusternetes::NoopApiHooks).unwrap(),
        agones_rusternetes::validate_game_server_set_creation(
            &set,
            &agones_rusternetes::NoopApiHooks,
        )
        .unwrap(),
    ] {
        source_metadata_assertion_001((reference.len() == 1) && (actual.len() == 1));
        source_metadata_assertion_002(
            (reference
                .get(0)
                .is_some_and(|error| error.kind == agones_core::ValidationErrorKind::Invalid))
                && (actual
                    .get(0)
                    .is_some_and(|error| error.kind == agones_core::ValidationErrorKind::Invalid)),
        );
        source_metadata_assertion_003(
            (reference
                .get(0)
                .is_some_and(|error| error.path == "spec.allocationOverflow.labels"))
                && (actual
                    .get(0)
                    .is_some_and(|error| error.path == "spec.allocationOverflow.labels")),
        );
        assert_eq!(actual, reference);
    }
}
#[test]
fn source_metadata_rusternetes_overflow_002() {
    use agones_rusternetes::RusternetesCustomResource;
    let mut input = agones_core::AllocationOverflow::default();
    input
        .labels
        .insert("valid".to_owned(), "$$$NOPE".to_owned());
    let reference: Vec<agones_core::ValidationError> = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.allocationOverflow.labels".to_owned(), bad_value: Some("$$$NOPE".to_owned()), detail: "a valid label must be an empty string or consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyValue',  or 'my_value',  or '12345', regex used for validation is '(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9])?')".to_owned(), origin: "format=k8s-label-value".to_owned(), covered_by_declarative: false },];
    let mut game = agones_core::GameServer::default();
    game.spec.template.spec.containers = vec![agones_core::Container {
        name: "game".into(),
        image: "game:v1".into(),
        ..Default::default()
    }]
    .into();
    game.apply_defaults();
    let mut fleet = agones_core::Fleet::default();
    fleet.metadata.name = "fleet".into();
    fleet.spec.template.spec = game.spec.clone();
    fleet.spec.allocation_overflow = Some(input.clone());
    fleet.apply_defaults();
    let fleet = agones_core::Fleet::from_rusternetes_custom_resource(
        fleet.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(fleet.spec.allocation_overflow.as_ref(), Some(&input));
    let mut set = agones_core::GameServerSet::default();
    set.metadata.name = "set".into();
    set.spec.template.spec = game.spec;
    set.spec.allocation_overflow = Some(input.clone());
    let set = agones_core::GameServerSet::from_rusternetes_custom_resource(
        set.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(set.spec.allocation_overflow.as_ref(), Some(&input));
    for actual in [
        agones_rusternetes::validate_fleet(&fleet, &agones_rusternetes::NoopApiHooks).unwrap(),
        agones_rusternetes::validate_game_server_set_creation(
            &set,
            &agones_rusternetes::NoopApiHooks,
        )
        .unwrap(),
    ] {
        source_metadata_assertion_004((reference.len() == 1) && (actual.len() == 1));
        source_metadata_assertion_005(
            (reference
                .get(0)
                .is_some_and(|error| error.kind == agones_core::ValidationErrorKind::Invalid))
                && (actual
                    .get(0)
                    .is_some_and(|error| error.kind == agones_core::ValidationErrorKind::Invalid)),
        );
        source_metadata_assertion_006(
            (reference
                .get(0)
                .is_some_and(|error| error.path == "spec.allocationOverflow.labels"))
                && (actual
                    .get(0)
                    .is_some_and(|error| error.path == "spec.allocationOverflow.labels")),
        );
        assert_eq!(actual, reference);
    }
}
#[test]
fn source_metadata_rusternetes_overflow_003() {
    use agones_rusternetes::RusternetesCustomResource;
    let mut input = agones_core::AllocationOverflow::default();
    input
        .annotations
        .insert("$$$foobar".to_owned(), "stuff".to_owned());
    let reference: Vec<agones_core::ValidationError> = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.allocationOverflow.annotations".to_owned(), bad_value: Some("$$$foobar".to_owned()), detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },];
    let mut game = agones_core::GameServer::default();
    game.spec.template.spec.containers = vec![agones_core::Container {
        name: "game".into(),
        image: "game:v1".into(),
        ..Default::default()
    }]
    .into();
    game.apply_defaults();
    let mut fleet = agones_core::Fleet::default();
    fleet.metadata.name = "fleet".into();
    fleet.spec.template.spec = game.spec.clone();
    fleet.spec.allocation_overflow = Some(input.clone());
    fleet.apply_defaults();
    let fleet = agones_core::Fleet::from_rusternetes_custom_resource(
        fleet.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(fleet.spec.allocation_overflow.as_ref(), Some(&input));
    let mut set = agones_core::GameServerSet::default();
    set.metadata.name = "set".into();
    set.spec.template.spec = game.spec;
    set.spec.allocation_overflow = Some(input.clone());
    let set = agones_core::GameServerSet::from_rusternetes_custom_resource(
        set.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(set.spec.allocation_overflow.as_ref(), Some(&input));
    for actual in [
        agones_rusternetes::validate_fleet(&fleet, &agones_rusternetes::NoopApiHooks).unwrap(),
        agones_rusternetes::validate_game_server_set_creation(
            &set,
            &agones_rusternetes::NoopApiHooks,
        )
        .unwrap(),
    ] {
        source_metadata_assertion_007((reference.len() == 1) && (actual.len() == 1));
        source_metadata_assertion_008(
            (reference
                .get(0)
                .is_some_and(|error| error.kind == agones_core::ValidationErrorKind::Invalid))
                && (actual
                    .get(0)
                    .is_some_and(|error| error.kind == agones_core::ValidationErrorKind::Invalid)),
        );
        source_metadata_assertion_009(
            (reference
                .get(0)
                .is_some_and(|error| error.path == "spec.allocationOverflow.annotations"))
                && (actual
                    .get(0)
                    .is_some_and(|error| error.path == "spec.allocationOverflow.annotations")),
        );
        assert_eq!(actual, reference);
    }
}
#[test]
fn source_metadata_rusternetes_overflow_004() {
    use agones_rusternetes::RusternetesCustomResource;
    let mut input = agones_core::AllocationOverflow::default();
    input
        .labels
        .insert("still.valid".to_owned(), "check-me-out".to_owned());
    input.labels.insert("valid".to_owned(), "yes".to_owned());
    input.annotations.insert(
        "icando-this".to_owned(),
        "yes, I can do all kinds of things here $$$".to_owned(),
    );
    let reference: Vec<agones_core::ValidationError> = vec![];
    let mut game = agones_core::GameServer::default();
    game.spec.template.spec.containers = vec![agones_core::Container {
        name: "game".into(),
        image: "game:v1".into(),
        ..Default::default()
    }]
    .into();
    game.apply_defaults();
    let mut fleet = agones_core::Fleet::default();
    fleet.metadata.name = "fleet".into();
    fleet.spec.template.spec = game.spec.clone();
    fleet.spec.allocation_overflow = Some(input.clone());
    fleet.apply_defaults();
    let fleet = agones_core::Fleet::from_rusternetes_custom_resource(
        fleet.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(fleet.spec.allocation_overflow.as_ref(), Some(&input));
    let mut set = agones_core::GameServerSet::default();
    set.metadata.name = "set".into();
    set.spec.template.spec = game.spec;
    set.spec.allocation_overflow = Some(input.clone());
    let set = agones_core::GameServerSet::from_rusternetes_custom_resource(
        set.to_rusternetes_custom_resource().unwrap(),
    )
    .unwrap();
    assert_eq!(set.spec.allocation_overflow.as_ref(), Some(&input));
    for actual in [
        agones_rusternetes::validate_fleet(&fleet, &agones_rusternetes::NoopApiHooks).unwrap(),
        agones_rusternetes::validate_game_server_set_creation(
            &set,
            &agones_rusternetes::NoopApiHooks,
        )
        .unwrap(),
    ] {
        source_metadata_assertion_010((reference.len() == 0) && (actual.len() == 0));
        assert_eq!(actual, reference);
    }
}
