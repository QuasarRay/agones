use std::collections::BTreeMap;

use agones_core::{
    AllocationOverflow, CanValidateObjectMetadata, Container, DeploymentStrategyType, FeatureFlags,
    Fleet, GameServer, GameServerSet, MetaPatch, ObjectMetadataValidator,
    ObjectMetadataValidatorComponent, ValidationError, ValidationErrorKind,
};
use cgp::prelude::*;

struct MetadataOnlyContext;
delegate_and_check_components! {
    MetadataOnlyContext {
        ObjectMetadataValidatorComponent: agones_core::ValidateAgonesObjectMetadata,
    }
}

const NAME_FORMAT: &str = "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')";
const KEY_FORMAT: &str = "a valid label key must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]') with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName')";
const DNS_FORMAT: &str = "prefix part a lowercase RFC 1123 subdomain must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character (e.g. 'example.com', regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?(\\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*')";
const VALUE_FORMAT: &str = "a valid label must be an empty string or consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyValue',  or 'my_value',  or '12345', regex used for validation is '(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9])?')";

fn invalid(path: &str, bad_value: &str, detail: &str, origin: &str) -> ValidationError {
    ValidationError {
        kind: ValidationErrorKind::Invalid,
        path: path.into(),
        bad_value: Some(bad_value.into()),
        detail: detail.into(),
        origin: origin.into(),
        covered_by_declarative: false,
    }
}

fn key_error(key: &str, detail: &str) -> ValidationError {
    invalid("metadata.labels", key, detail, "format=k8s-label-key")
}

fn validate_label(key: &str, value: &str) -> Vec<ValidationError> {
    MetadataOnlyContext.validate_object_metadata(
        &BTreeMap::from([(key.into(), value.into())]),
        &BTreeMap::new(),
        "metadata",
    )
}

fn validate_annotation(key: &str, value: &str) -> Vec<ValidationError> {
    MetadataOnlyContext.validate_object_metadata(
        &BTreeMap::new(),
        &BTreeMap::from([(key.into(), value.into())]),
        "metadata",
    )
}

#[test]
fn qualified_label_key_checks_every_prefix_name_and_slash_condition() {
    for key in [
        "A_0..b-Z".into(),
        "9".into(),
        "example.com/My_Name-0".into(),
        "a".repeat(63),
        format!("{}/z", "a".repeat(253)),
        format!("{}.com/z", "a".repeat(64)),
    ] {
        assert!(validate_label(&key, "").is_empty(), "{key:?}");
    }
    let cases = [
        ("".into(), vec!["name part must be non-empty", NAME_FORMAT]),
        (
            "/".into(),
            vec![
                "prefix part must be non-empty",
                "name part must be non-empty",
                NAME_FORMAT,
            ],
        ),
        ("/valid".into(), vec!["prefix part must be non-empty"]),
        ("a/b/c".into(), vec![KEY_FORMAT]),
        ("///".into(), vec![KEY_FORMAT]),
        ("EXAMPLE.com/z".into(), vec![DNS_FORMAT]),
        ("a..b/z".into(), vec![DNS_FORMAT]),
        ("a.-b/z".into(), vec![DNS_FORMAT]),
        ("a-/z".into(), vec![DNS_FORMAT]),
        ("é/z".into(), vec![DNS_FORMAT]),
        (
            "a/".into(),
            vec!["name part must be non-empty", NAME_FORMAT],
        ),
        ("-bad".into(), vec![NAME_FORMAT]),
        ("bad\0".into(), vec![NAME_FORMAT]),
        (
            "a".repeat(64),
            vec!["name part must be no more than 63 bytes"],
        ),
        (
            "_".repeat(64),
            vec!["name part must be no more than 63 bytes", NAME_FORMAT],
        ),
        (
            format!("{}/{}", "_".repeat(254), "_".repeat(64)),
            vec![
                "prefix part must be no more than 253 bytes",
                DNS_FORMAT,
                "name part must be no more than 63 bytes",
                NAME_FORMAT,
            ],
        ),
    ];
    for (key, details) in cases {
        assert_eq!(
            validate_label(&key, ""),
            details
                .into_iter()
                .map(|d| key_error(&key, d))
                .collect::<Vec<_>>(),
            "{key:?}"
        );
    }
}

#[test]
fn label_value_length_and_format_errors_are_independent_and_follow_key_errors() {
    for value in ["".into(), "0".into(), "a.._-Z".into(), "Z".repeat(63)] {
        assert!(validate_label("valid", &value).is_empty());
    }
    for (value, details) in [
        ("a".repeat(64), vec!["must be no more than 63 bytes"]),
        (
            "_".repeat(64),
            vec!["must be no more than 63 bytes", VALUE_FORMAT],
        ),
        (
            "é".repeat(32),
            vec!["must be no more than 63 bytes", VALUE_FORMAT],
        ),
        ("a\nb".into(), vec![VALUE_FORMAT]),
        ("_".into(), vec![VALUE_FORMAT]),
    ] {
        let mut expected = vec![
            key_error("", "name part must be non-empty"),
            key_error("", NAME_FORMAT),
        ];
        expected.extend(
            details
                .into_iter()
                .map(|detail| invalid("metadata.labels", &value, detail, "format=k8s-label-value")),
        );
        assert_eq!(validate_label("", &value), expected);
    }
}

#[test]
fn annotation_validation_uses_simple_lowercase_but_retains_original_bad_value() {
    for key in [
        "EXAMPLE.COM/MyName".into(),
        "K.example/İ".into(),
        "İ".repeat(63),
    ] {
        assert!(validate_annotation(&key, "\0\n💠").is_empty(), "{key:?}");
    }
    assert_eq!(
        validate_label("EXAMPLE.COM/MyName", ""),
        vec![key_error("EXAMPLE.COM/MyName", DNS_FORMAT)]
    );
    let key = "İ".repeat(32);
    assert_eq!(
        validate_label(&key, ""),
        vec![
            key_error(&key, "name part must be no more than 63 bytes"),
            key_error(&key, NAME_FORMAT)
        ]
    );
    let key = "ẞ".repeat(22); // 66 original bytes, 44 bytes after simple lowercase.
    assert_eq!(
        validate_annotation(&key, ""),
        vec![invalid(
            "metadata.annotations",
            &key,
            NAME_FORMAT,
            "format=k8s-label-key"
        )]
    );
    let key = format!("{}/", "A".repeat(254));
    let expected = [
        "prefix part must be no more than 253 bytes",
        "name part must be non-empty",
        NAME_FORMAT,
    ];
    assert_eq!(
        validate_annotation(&key, ""),
        expected
            .into_iter()
            .map(|d| invalid("metadata.annotations", &key, d, "format=k8s-label-key"))
            .collect::<Vec<_>>()
    );
}

fn annotation_too_long(path: &str) -> ValidationError {
    ValidationError {
        kind: ValidationErrorKind::TooLong,
        path: path.into(),
        bad_value: Some("<value omitted>".into()),
        detail: "may not be more than 262144 bytes".into(),
        origin: String::new(),
        covered_by_declarative: false,
    }
}

#[test]
fn annotation_limit_counts_all_original_key_and_value_bytes_and_follows_key_errors() {
    let limit = agones_core::TOTAL_ANNOTATION_SIZE_LIMIT_BYTES as usize;
    for (key, key_bytes) in [("key", 3), ("İ", 2)] {
        assert!(validate_annotation(key, &"x".repeat(limit - key_bytes)).is_empty());
        assert_eq!(
            validate_annotation(key, &"x".repeat(limit - key_bytes + 1)),
            vec![annotation_too_long("metadata.annotations")]
        );
    }
    let annotations = BTreeMap::from([
        ("a".into(), "é".repeat(65535)),
        ("b".into(), "\0".repeat(131072)),
    ]);
    assert!(
        MetadataOnlyContext
            .validate_object_metadata(&BTreeMap::new(), &annotations, "metadata")
            .is_empty()
    );
    let mut annotations = annotations;
    annotations.get_mut("b").unwrap().push('\0');
    assert_eq!(
        MetadataOnlyContext.validate_object_metadata(&BTreeMap::new(), &annotations, "metadata"),
        vec![annotation_too_long("metadata.annotations")]
    );
    let mut expected = vec![
        invalid(
            "metadata.annotations",
            "",
            "name part must be non-empty",
            "format=k8s-label-key",
        ),
        invalid(
            "metadata.annotations",
            "",
            NAME_FORMAT,
            "format=k8s-label-key",
        ),
    ];
    expected.push(annotation_too_long("metadata.annotations"));
    assert_eq!(validate_annotation("", &"x".repeat(limit + 1)), expected);
}

#[test]
fn same_complete_policy_reaches_templates_overflow_and_allocation_patch() {
    let labels = BTreeMap::from([("EXAMPLE.com/x".into(), "ok".into())]);
    let mut gs = GameServer::default();
    gs.spec.template.spec.containers = vec![Container {
        name: "game".into(),
        ..Default::default()
    }]
    .into();
    gs.spec.template.metadata.labels = labels.clone();
    gs.apply_defaults();
    assert_eq!(
        gs.validate(),
        vec![invalid(
            "spec.template.metadata.labels",
            "EXAMPLE.com/x",
            DNS_FORMAT,
            "format=k8s-label-key"
        )]
    );
    // The development-address branch still validates metadata.
    assert_eq!(gs.spec.validate("127.0.0.1"), gs.validate());

    let mut fleet = Fleet::default();
    fleet.metadata.name = "fleet".into();
    fleet.spec.strategy.strategy_type = Some(DeploymentStrategyType::Recreate);
    fleet.spec.template.spec = gs.spec.clone();
    fleet.spec.template.metadata.labels = labels.clone();
    let paths = fleet
        .validate()
        .into_iter()
        .map(|e| e.path)
        .collect::<Vec<_>>();
    assert_eq!(
        paths,
        [
            "spec.template.spec.template.metadata.labels",
            "spec.template.metadata.labels"
        ]
    );
    let mut set = GameServerSet::default();
    set.metadata.name = "set".into();
    set.spec.template = fleet.spec.template.clone();
    assert_eq!(
        set.validate()
            .into_iter()
            .map(|e| e.path)
            .collect::<Vec<_>>(),
        paths
    );

    let overflow = AllocationOverflow {
        labels: labels.clone(),
        ..Default::default()
    };
    let patch = MetaPatch {
        labels,
        ..Default::default()
    };
    assert_eq!(
        overflow.validate_with_context(&MetadataOnlyContext, "patch"),
        patch.validate_with_context(&MetadataOnlyContext, "patch")
    );
    assert_eq!(patch.validate("patch"), overflow.validate("patch"));
    assert_eq!(patch.validate("patch").len(), 1);
}

#[derive(HasField)]
struct ObserveContext {
    paths: std::cell::RefCell<Vec<String>>,
    game_server_feature_flags: FeatureFlags,
}
#[cgp_new_provider]
impl ObjectMetadataValidator<ObserveContext> for ObserveMetadata {
    fn validate_object_metadata(
        context: &ObserveContext,
        _labels: &BTreeMap<String, String>,
        _annotations: &BTreeMap<String, String>,
        path: &str,
    ) -> Vec<ValidationError> {
        context.paths.borrow_mut().push(path.into());
        Vec::new()
    }
}
delegate_and_check_components! {
    ObserveContext {
        ObjectMetadataValidatorComponent: ObserveMetadata,
        agones_core::ResourceQuantityValidatorComponent: agones_core::ValidateAgonesResourceQuantities,
        agones_core::GameServerFeatureFlagsGetterComponent: UseField<Symbol!("game_server_feature_flags")>,
        agones_core::GameServerFeatureValidatorComponent: agones_core::ValidateAgonesGameServerFeatures,
    }
}

#[test]
fn consumer_composition_preserves_every_selected_metadata_provider_call() {
    let context = ObserveContext {
        paths: Default::default(),
        game_server_feature_flags: Default::default(),
    };
    let mut fleet = Fleet::default();
    fleet.spec.allocation_overflow = Some(AllocationOverflow::default());
    fleet.validate_with_context(&context);
    assert_eq!(
        *context.paths.borrow(),
        [
            "spec.template.metadata",
            "spec.template.metadata",
            "spec.allocationOverflow"
        ]
    );
    context.paths.borrow_mut().clear();
    let mut set = GameServerSet::default();
    set.spec.allocation_overflow = Some(AllocationOverflow::default());
    set.validate_with_context(&context);
    assert_eq!(
        *context.paths.borrow(),
        [
            "spec.template.metadata",
            "spec.template.metadata",
            "spec.allocationOverflow"
        ]
    );
    context.paths.borrow_mut().clear();
    let mut gs = GameServer::default();
    gs.spec
        .template
        .metadata
        .labels
        .insert("bad/invalid/key".into(), "bad value".into());
    gs.validate_with_context(&context);
    gs.spec.validate_with_context(&context, "127.0.0.1");
    MetaPatch::default().validate_with_context(&context, "前缀\0.patch");
    agones_core::GameServerAllocation::default().validate_with_context(&context);
    assert_eq!(
        *context.paths.borrow(),
        [
            "spec.template.metadata",
            "spec.template.metadata",
            "前缀\0.patch",
            "spec.metadata"
        ]
    );
}

#[test]
fn pinned_unicode_simple_lowercase_matches_every_scalar() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../verification/agones/unicode-simple-lowercase-table.json"
    ))
    .unwrap();
    assert_eq!(char::UNICODE_VERSION, (17, 0, 0));
    let mappings: BTreeMap<u32, u32> = fixture["mappings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|pair| {
            (
                pair[0].as_u64().unwrap() as u32,
                pair[1].as_u64().unwrap() as u32,
            )
        })
        .collect();
    assert_eq!(mappings.len(), 1488);
    let mut visits = 0;
    for n in 0..=0x10ffffu32 {
        if let Some(ch) = char::from_u32(n) {
            assert_eq!(
                ch.to_lowercase().next().unwrap() as u32,
                *mappings.get(&n).unwrap_or(&n),
                "U+{n:04X}"
            );
            visits += 1;
        }
    }
    assert_eq!(visits, 1_112_064);
}
