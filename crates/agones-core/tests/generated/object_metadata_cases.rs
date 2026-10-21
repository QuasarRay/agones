// Generated from gated complete object-metadata source IR; do not edit.
use agones_core::ObjectMetadataValidatorComponent;
use cgp::prelude::*;
struct SourceMetadataContext;
delegate_and_check_components! { SourceMetadataContext { ObjectMetadataValidatorComponent: agones_core::ValidateAgonesObjectMetadata, } }
#[cfg(not(kani))]
fn source_metadata_native_projection(
    labels: &std::collections::BTreeMap<String, String>,
    annotations: &std::collections::BTreeMap<String, String>,
    expected_labels: &str,
    expected_annotations: &str,
    actual: &[agones_core::ValidationError],
    reference: &[agones_core::ValidationError],
) {
    let expected_labels: Option<std::collections::BTreeMap<String, String>> =
        serde_json::from_str(expected_labels).unwrap();
    let expected_annotations: Option<std::collections::BTreeMap<String, String>> =
        serde_json::from_str(expected_annotations).unwrap();
    assert_eq!(labels, &expected_labels.unwrap_or_default());
    assert_eq!(annotations, &expected_annotations.unwrap_or_default());
    assert_eq!(actual, reference);
}
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
fn source_metadata_assertion_011(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-315"
    );
}
fn source_metadata_assertion_012(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-320"
    );
}
fn source_metadata_assertion_013(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-325"
    );
}
fn source_metadata_assertion_014(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-331"
    );
}
fn source_metadata_assertion_015(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-332"
    );
}
fn source_metadata_assertion_016(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-339"
    );
}
fn source_metadata_assertion_017(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-340"
    );
}
fn source_metadata_assertion_018(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-345"
    );
}
fn source_metadata_assertion_019(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-346"
    );
}
fn source_metadata_assertion_020(relation: bool) {
    assert!(
        relation,
        "agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate::line-347"
    );
}
fn source_metadata_program_000(retain_observations: bool) {
    let context = SourceMetadataContext;
    let input = agones_core::AllocationOverflow::default();
    let actual = input.validate_with_context(&context, "spec.allocationOverflow");
    let reference: Vec<agones_core::ValidationError> = vec![];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &input.labels,
            &input.annotations,
            "null",
            "null",
            &actual,
            &reference,
        );
    }
    source_metadata_assertion_000((reference.len() == 0) && (actual.len() == 0));
    #[cfg(kani)]
    {
        kani::cover!(
            true,
            "source metadata validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::empty"
        );
    }
    if retain_observations {
        std::mem::forget((input, context, actual, reference));
    }
}
fn source_metadata_program_001(retain_observations: bool) {
    let context = SourceMetadataContext;
    let mut input = agones_core::AllocationOverflow::default();
    input
        .labels
        .insert("$$$foobar".to_owned(), "stuff".to_owned());
    let actual = input.validate_with_context(&context, "spec.allocationOverflow");
    let reference: Vec<agones_core::ValidationError> = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.allocationOverflow.labels".to_owned(), bad_value: Some("$$$foobar".to_owned()), detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &input.labels,
            &input.annotations,
            "{\"$$$foobar\":\"stuff\"}",
            "null",
            &actual,
            &reference,
        );
    }
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
    #[cfg(kani)]
    {
        kani::cover!(
            true,
            "source metadata validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad label name"
        );
    }
    if retain_observations {
        std::mem::forget((input, context, actual, reference));
    }
}
fn source_metadata_program_002(retain_observations: bool) {
    let context = SourceMetadataContext;
    let mut input = agones_core::AllocationOverflow::default();
    input
        .labels
        .insert("valid".to_owned(), "$$$NOPE".to_owned());
    let actual = input.validate_with_context(&context, "spec.allocationOverflow");
    let reference: Vec<agones_core::ValidationError> = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.allocationOverflow.labels".to_owned(), bad_value: Some("$$$NOPE".to_owned()), detail: "a valid label must be an empty string or consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyValue',  or 'my_value',  or '12345', regex used for validation is '(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9])?')".to_owned(), origin: "format=k8s-label-value".to_owned(), covered_by_declarative: false },];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &input.labels,
            &input.annotations,
            "{\"valid\":\"$$$NOPE\"}",
            "null",
            &actual,
            &reference,
        );
    }
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
    #[cfg(kani)]
    {
        kani::cover!(
            true,
            "source metadata validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad label value"
        );
    }
    if retain_observations {
        std::mem::forget((input, context, actual, reference));
    }
}
fn source_metadata_program_003(retain_observations: bool) {
    let context = SourceMetadataContext;
    let mut input = agones_core::AllocationOverflow::default();
    input
        .annotations
        .insert("$$$foobar".to_owned(), "stuff".to_owned());
    let actual = input.validate_with_context(&context, "spec.allocationOverflow");
    let reference: Vec<agones_core::ValidationError> = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.allocationOverflow.annotations".to_owned(), bad_value: Some("$$$foobar".to_owned()), detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &input.labels,
            &input.annotations,
            "null",
            "{\"$$$foobar\":\"stuff\"}",
            &actual,
            &reference,
        );
    }
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
    #[cfg(kani)]
    {
        kani::cover!(
            true,
            "source metadata validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::bad annotation name"
        );
    }
    if retain_observations {
        std::mem::forget((input, context, actual, reference));
    }
}
fn source_metadata_program_004(retain_observations: bool) {
    let context = SourceMetadataContext;
    let mut input = agones_core::AllocationOverflow::default();
    input
        .labels
        .insert("still.valid".to_owned(), "check-me-out".to_owned());
    input.labels.insert("valid".to_owned(), "yes".to_owned());
    input.annotations.insert(
        "icando-this".to_owned(),
        "yes, I can do all kinds of things here $$$".to_owned(),
    );
    let actual = input.validate_with_context(&context, "spec.allocationOverflow");
    let reference: Vec<agones_core::ValidationError> = vec![];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &input.labels,
            &input.annotations,
            "{\"still.valid\":\"check-me-out\",\"valid\":\"yes\"}",
            "{\"icando-this\":\"yes, I can do all kinds of things here $$$\"}",
            &actual,
            &reference,
        );
    }
    source_metadata_assertion_010((reference.len() == 0) && (actual.len() == 0));
    #[cfg(kani)]
    {
        kani::cover!(
            true,
            "source metadata validation case: agones.dev/agones/pkg/apis/agones/v1::pkg/apis/agones/v1/common_test.go::TestAllocationOverflowValidate::valid full"
        );
    }
    if retain_observations {
        std::mem::forget((input, context, actual, reference));
    }
}
fn source_metadata_program_005(retain_observations: bool) {
    let context = SourceMetadataContext;
    let mut mp = agones_core::MetaPatch::default();
    let actual_000 = mp.validate_with_context(&context, "spec.metadata");
    let reference_000: Vec<agones_core::ValidationError> = vec![];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &mp.labels,
            &mp.annotations,
            "null",
            "null",
            &actual_000,
            &reference_000,
        );
    }
    source_metadata_assertion_011((reference_000.len() == 0) && (actual_000.len() == 0));
    mp.labels = std::collections::BTreeMap::new();
    mp.annotations = std::collections::BTreeMap::new();
    let actual_001 = mp.validate_with_context(&context, "spec.metadata");
    let reference_001: Vec<agones_core::ValidationError> = vec![];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &mp.labels,
            &mp.annotations,
            "{}",
            "{}",
            &actual_001,
            &reference_001,
        );
    }
    source_metadata_assertion_012((reference_001.len() == 0) && (actual_001.len() == 0));
    mp.labels.insert("foo".to_owned(), "bar".to_owned());
    mp.annotations.insert("bar".to_owned(), "foo".to_owned());
    let actual_002 = mp.validate_with_context(&context, "spec.metadata");
    let reference_002: Vec<agones_core::ValidationError> = vec![];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &mp.labels,
            &mp.annotations,
            "{\"foo\":\"bar\"}",
            "{\"bar\":\"foo\"}",
            &actual_002,
            &reference_002,
        );
    }
    source_metadata_assertion_013((reference_002.len() == 0) && (actual_002.len() == 0));
    let mut invalid_0 = mp.clone();
    invalid_0.labels.insert("$$$$".to_owned(), "no".to_owned());
    let actual_003 = invalid_0.validate_with_context(&context, "spec.metadata");
    let reference_003: Vec<agones_core::ValidationError> = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.metadata.labels".to_owned(), bad_value: Some("$$$$".to_owned()), detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &invalid_0.labels,
            &invalid_0.annotations,
            "{\"$$$$\":\"no\",\"foo\":\"bar\"}",
            "{\"bar\":\"foo\"}",
            &actual_003,
            &reference_003,
        );
    }
    source_metadata_assertion_014((reference_003.len() == 1) && (actual_003.len() == 1));
    source_metadata_assertion_015(
        (reference_003
            .get(0)
            .is_some_and(|error| error.path == "spec.metadata.labels"))
            && (actual_003
                .get(0)
                .is_some_and(|error| error.path == "spec.metadata.labels")),
    );
    let mut invalid_1 = mp.clone();
    invalid_1
        .annotations
        .insert("$$$$".to_owned(), "no".to_owned());
    let actual_004 = invalid_1.validate_with_context(&context, "spec.metadata");
    let reference_004: Vec<agones_core::ValidationError> = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.metadata.annotations".to_owned(), bad_value: Some("$$$$".to_owned()), detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &invalid_1.labels,
            &invalid_1.annotations,
            "{\"foo\":\"bar\"}",
            "{\"$$$$\":\"no\",\"bar\":\"foo\"}",
            &actual_004,
            &reference_004,
        );
    }
    source_metadata_assertion_016((reference_004.len() == 1) && (actual_004.len() == 1));
    source_metadata_assertion_017(
        (reference_004
            .get(0)
            .is_some_and(|error| error.path == "spec.metadata.annotations"))
            && (actual_004
                .get(0)
                .is_some_and(|error| error.path == "spec.metadata.annotations")),
    );
    invalid_1.labels.insert("$$$$".to_owned(), "no".to_owned());
    let actual_005 = invalid_1.validate_with_context(&context, "spec.metadata");
    let reference_005: Vec<agones_core::ValidationError> = vec![agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.metadata.labels".to_owned(), bad_value: Some("$$$$".to_owned()), detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path: "spec.metadata.annotations".to_owned(), bad_value: Some("$$$$".to_owned()), detail: "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')".to_owned(), origin: "format=k8s-label-key".to_owned(), covered_by_declarative: false },];
    #[cfg(not(kani))]
    {
        source_metadata_native_projection(
            &invalid_1.labels,
            &invalid_1.annotations,
            "{\"$$$$\":\"no\",\"foo\":\"bar\"}",
            "{\"$$$$\":\"no\",\"bar\":\"foo\"}",
            &actual_005,
            &reference_005,
        );
    }
    source_metadata_assertion_018((reference_005.len() == 2) && (actual_005.len() == 2));
    source_metadata_assertion_019(
        (reference_005
            .get(0)
            .is_some_and(|error| error.path == "spec.metadata.labels"))
            && (actual_005
                .get(0)
                .is_some_and(|error| error.path == "spec.metadata.labels")),
    );
    source_metadata_assertion_020(
        (reference_005
            .get(1)
            .is_some_and(|error| error.path == "spec.metadata.annotations"))
            && (actual_005
                .get(1)
                .is_some_and(|error| error.path == "spec.metadata.annotations")),
    );
    #[cfg(kani)]
    {
        kani::cover!(
            true,
            "source metadata validation case: agones.dev/agones/pkg/apis/allocation/v1::pkg/apis/allocation/v1/gameserverallocation_test.go::TestMetaPatchValidate"
        );
    }
    if retain_observations {
        std::mem::forget((
            context,
            mp,
            actual_000,
            reference_000,
            actual_001,
            reference_001,
            actual_002,
            reference_002,
            invalid_0,
            actual_003,
            reference_003,
            invalid_1,
            actual_004,
            reference_004,
            actual_005,
            reference_005,
        ));
    }
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_metadata_case_000() {
    bolero::check!()
        .with_type::<()>()
        .with_iterations(1)
        .for_each(|_| {
            source_metadata_program_000(true);
        });
}
#[cfg(not(kani))]
#[test]
fn source_metadata_native_replay_000() {
    source_metadata_program_000(false);
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_metadata_case_001() {
    bolero::check!()
        .with_type::<()>()
        .with_iterations(1)
        .for_each(|_| {
            source_metadata_program_001(true);
        });
}
#[cfg(not(kani))]
#[test]
fn source_metadata_native_replay_001() {
    source_metadata_program_001(false);
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_metadata_case_002() {
    bolero::check!()
        .with_type::<()>()
        .with_iterations(1)
        .for_each(|_| {
            source_metadata_program_002(true);
        });
}
#[cfg(not(kani))]
#[test]
fn source_metadata_native_replay_002() {
    source_metadata_program_002(false);
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_metadata_case_003() {
    bolero::check!()
        .with_type::<()>()
        .with_iterations(1)
        .for_each(|_| {
            source_metadata_program_003(true);
        });
}
#[cfg(not(kani))]
#[test]
fn source_metadata_native_replay_003() {
    source_metadata_program_003(false);
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_metadata_case_004() {
    bolero::check!()
        .with_type::<()>()
        .with_iterations(1)
        .for_each(|_| {
            source_metadata_program_004(true);
        });
}
#[cfg(not(kani))]
#[test]
fn source_metadata_native_replay_004() {
    source_metadata_program_004(false);
}
#[test]
#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(128))]
fn source_metadata_case_005() {
    bolero::check!()
        .with_type::<()>()
        .with_iterations(1)
        .for_each(|_| {
            source_metadata_program_005(true);
        });
}
#[cfg(not(kani))]
#[test]
fn source_metadata_native_replay_005() {
    source_metadata_program_005(false);
}
