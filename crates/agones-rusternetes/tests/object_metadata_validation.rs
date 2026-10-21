use std::collections::BTreeMap;

use agones_core::{
    Container, Fleet, GameServer, GameServerSet, ObjectMeta, ValidationError, ValidationErrorKind,
};
use agones_rusternetes::{
    FleetValidationAdmission, GameServerCreationValidationAdmission,
    GameServerSetCreationValidationAdmission, NoopApiHooks, validate_fleet,
    validate_game_server_creation, validate_game_server_set_creation,
};
use rusternetes_common::admission::{
    AdmissionController, AdmissionRequest, AdmissionResponse, Operation, UserInfo,
};
use serde_json::Value;

fn objects(metadata: ObjectMeta) -> [(&'static str, Value); 3] {
    let mut clean = GameServer::default();
    clean.metadata.name = "game".into();
    clean.spec.template.spec.containers = vec![Container {
        name: "game".into(),
        image: "game:v1".into(),
        ..Default::default()
    }]
    .into();
    clean.apply_defaults();
    let mut gs = clean.clone();
    gs.spec.template.metadata = metadata.clone();
    let mut fleet = Fleet::default();
    fleet.metadata.name = "fleet".into();
    fleet.spec.template.spec = clean.spec.clone();
    fleet.spec.template.metadata = metadata.clone();
    fleet.apply_defaults();
    let mut set = GameServerSet::default();
    set.metadata.name = "set".into();
    set.spec.template.spec = clean.spec;
    set.spec.template.metadata = metadata;
    [
        ("GameServer", serde_json::to_value(gs).unwrap()),
        ("Fleet", serde_json::to_value(fleet).unwrap()),
        ("GameServerSet", serde_json::to_value(set).unwrap()),
    ]
}

fn request(kind: &str, object: Value) -> AdmissionRequest {
    AdmissionRequest {
        operation: Operation::Create,
        kind: kind.into(),
        namespace: Some("default".into()),
        name: "game".into(),
        object,
        old_object: None,
        user_info: UserInfo {
            username: "metadata-witness".into(),
            uid: String::new(),
            groups: vec![],
        },
    }
}

fn typed_errors(kind: &str, object: &Value) -> Vec<ValidationError> {
    match kind {
        "GameServer" => validate_game_server_creation(
            &serde_json::from_value(object.clone()).unwrap(),
            &NoopApiHooks,
        ),
        "Fleet" => validate_fleet(
            &serde_json::from_value(object.clone()).unwrap(),
            &NoopApiHooks,
        ),
        "GameServerSet" => validate_game_server_set_creation(
            &serde_json::from_value(object.clone()).unwrap(),
            &NoopApiHooks,
        ),
        _ => unreachable!(),
    }
    .unwrap()
}

fn admissions() -> [Box<dyn AdmissionController>; 3] {
    [
        Box::new(GameServerCreationValidationAdmission::new(NoopApiHooks)),
        Box::new(FleetValidationAdmission::new(NoopApiHooks)),
        Box::new(GameServerSetCreationValidationAdmission::new(NoopApiHooks)),
    ]
}

#[tokio::test]
async fn actual_rusternetes_admission_and_typed_routes_enforce_metadata_size_and_case_policy() {
    let limit = agones_core::TOTAL_ANNOTATION_SIZE_LIMIT_BYTES as usize;
    for too_large in [false, true] {
        let metadata = ObjectMeta {
            labels: BTreeMap::from([("example.com/My_Name".into(), "A._-z".into())]),
            annotations: BTreeMap::from([(
                "K.example/İ".into(),
                "\0".repeat(limit - "K.example/İ".len() + usize::from(too_large)),
            )]),
            ..Default::default()
        };
        for (admission, (kind, object)) in admissions().into_iter().zip(objects(metadata)) {
            let errors = typed_errors(kind, &object);
            let response = admission.admit(&request(kind, object)).await;
            if too_large {
                assert_eq!(
                    errors,
                    [ValidationError {
                        kind: ValidationErrorKind::TooLong,
                        path: "spec.template.metadata.annotations".into(),
                        bad_value: Some("<value omitted>".into()),
                        detail: "may not be more than 262144 bytes".into(),
                        origin: String::new(),
                        covered_by_declarative: false,
                    }]
                );
                assert_eq!(response, AdmissionResponse::Deny("spec.template.metadata.annotationsTooLong: <value omitted>: may not be more than 262144 bytes".into()), "{kind}");
            } else {
                assert!(errors.is_empty(), "{kind}: {errors:?}");
                assert_eq!(response, AdmissionResponse::Allow, "{kind}");
            }
        }
    }
}

#[tokio::test]
async fn rusternetes_routes_preserve_every_error_for_one_invalid_key_and_value() {
    let key = "_".repeat(64);
    let metadata = ObjectMeta {
        labels: BTreeMap::from([(key.clone(), key.clone())]),
        ..Default::default()
    };
    for (admission, (kind, object)) in admissions().into_iter().zip(objects(metadata)) {
        let errors = typed_errors(kind, &object);
        assert_eq!(errors.len(), 4, "{kind}: {errors:?}");
        assert_eq!(
            errors.iter().map(|e| e.origin.as_str()).collect::<Vec<_>>(),
            [
                "format=k8s-label-key",
                "format=k8s-label-key",
                "format=k8s-label-value",
                "format=k8s-label-value"
            ]
        );
        assert_eq!(errors[0].detail, "name part must be no more than 63 bytes");
        assert_eq!(errors[2].detail, "must be no more than 63 bytes");
        assert!(errors.iter().all(|e| e.kind == ValidationErrorKind::Invalid
            && e.path == "spec.template.metadata.labels"
            && e.bad_value.as_deref() == Some(&key)
            && !e.covered_by_declarative));
        let AdmissionResponse::Deny(message) = admission.admit(&request(kind, object)).await else {
            panic!("{kind} must reject invalid metadata");
        };
        assert_eq!(
            message
                .matches("spec.template.metadata.labelsInvalid")
                .count(),
            4
        );
    }
}
