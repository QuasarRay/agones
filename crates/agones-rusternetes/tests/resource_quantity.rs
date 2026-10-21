use agones_rusternetes::validate_resource_requirements;
use rusternetes_common::types::ResourceRequirements;
use serde_json::json;

#[test]
fn resource_quantities_compare_integer_bits_above_f64_precision() {
    let resources: ResourceRequirements = serde_json::from_value(json!({
        "requests":{"cpu":"9007199254740993"},
        "limits":{"cpu":"9007199254740992"}
    }))
    .unwrap();
    let errors = validate_resource_requirements(&resources, "resources").unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].path, "resources.requests");
    assert_eq!(errors[0].bad_value.as_deref(), Some("9007199254740993"));
    assert_eq!(
        errors[0].detail,
        "must be less than or equal to cpu limit of 9007199254740992"
    );
}

#[test]
fn resource_quantities_report_the_source_string_value() {
    let resources: ResourceRequirements = serde_json::from_value(json!({
        "requests":{"cpu":"-1.5"}
    }))
    .unwrap();
    let errors = validate_resource_requirements(&resources, "resources").unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].bad_value.as_deref(), Some("-1500m"));
}

#[test]
fn native_resource_requirements_reject_unparseable_values() {
    let resources: ResourceRequirements = serde_json::from_value(json!({
        "requests":{"cpu":"invalid"}
    }))
    .unwrap();
    let error = validate_resource_requirements(&resources, "resources").unwrap_err();
    assert!(
        error
            .to_string()
            .starts_with("resources.requests[cpu]: quantities must match")
    );
}

#[test]
fn direct_core_values_keep_null_quantity_as_zero_during_native_conversion() {
    let mut gs = agones_core::GameServer::default();
    let mut container = agones_core::Container {
        image: "test:v1".into(),
        ..Default::default()
    };
    container.resources = serde_json::from_value(json!({
        "requests":{"cpu":null}, "limits":{"cpu":"-1n"}
    }))
    .unwrap();
    gs.spec.template.spec.containers = vec![container].into();
    let errors = agones_rusternetes::validate_game_server_resources(&gs.spec).unwrap();
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0].bad_value.as_deref(), Some("-1n"));
    assert_eq!(errors[1].bad_value.as_deref(), Some("0"));
    assert_eq!(
        errors[1].detail,
        "must be less than or equal to cpu limit of -1n"
    );
}

#[tokio::test]
async fn actual_admissions_validate_exact_quantities_in_all_container_paths() {
    use agones_core::{Fleet, GameServer, GameServerSet};
    use agones_rusternetes::{
        FleetValidationAdmission, GameServerCreationValidationAdmission,
        GameServerSetCreationValidationAdmission, NoopApiHooks,
    };
    use rusternetes_common::admission::{
        AdmissionController, AdmissionRequest, AdmissionResponse, Operation, UserInfo,
    };

    for container_type in ["regular", "init", "restartable"] {
        let mut gs: GameServer = serde_json::from_value(json!({
            "metadata":{"name":"game"},
            "spec":{"template":{"spec":{"containers":[{"name":"game","image":"game:v1"}]}}}
        }))
        .unwrap();
        let mut container = json!({"name":"quantities","image":"test:v1","resources":{
            "requests":{"cpu":"9007199254740993"}, "limits":{"cpu":"9007199254740992"}
        }});
        let segment = if container_type == "regular" {
            gs.spec.template.spec.containers[0].resources =
                serde_json::from_value(container["resources"].clone()).unwrap();
            "containers"
        } else {
            if container_type == "restartable" {
                container["restartPolicy"] = json!("Always");
            }
            gs.spec.template.spec.init_containers =
                vec![serde_json::from_value(container).unwrap()];
            "initContainers"
        };
        gs.apply_defaults();
        let mut fleet = Fleet::default();
        fleet.spec.template.spec = gs.spec.clone();
        fleet.apply_defaults();
        let mut set = GameServerSet::default();
        set.spec.template.spec = gs.spec.clone();
        let admissions: [Box<dyn AdmissionController>; 3] = [
            Box::new(GameServerCreationValidationAdmission::from_context(
                agones_core::GameServerDefaultingContext::default(),
                NoopApiHooks,
            )),
            Box::new(FleetValidationAdmission::from_context(
                agones_core::GameServerDefaultingContext::default(),
                NoopApiHooks,
            )),
            Box::new(GameServerSetCreationValidationAdmission::from_context(
                agones_core::GameServerDefaultingContext::default(),
                NoopApiHooks,
            )),
        ];
        let objects = [
            ("GameServer", serde_json::to_value(gs).unwrap()),
            ("Fleet", serde_json::to_value(fleet).unwrap()),
            ("GameServerSet", serde_json::to_value(set).unwrap()),
        ];
        for (admission, (kind, mut object)) in admissions.into_iter().zip(objects) {
            let spec = if kind == "GameServer" {
                "spec"
            } else {
                "spec.template.spec"
            };
            let path = format!("{spec}.template.spec.{segment}[0].resources.requests");
            let mut request = AdmissionRequest {
                operation: Operation::Create,
                kind: kind.into(),
                namespace: Some("default".into()),
                name: "game".into(),
                object: object.clone(),
                old_object: None,
                user_info: UserInfo {
                    username: "quantity-witness".into(),
                    uid: String::new(),
                    groups: vec![],
                },
            };
            let AdmissionResponse::Deny(detail) = admission.admit(&request).await else {
                panic!("{kind} {container_type} allowed an excessive request")
            };
            assert!(detail.contains(&path), "{detail}");
            assert!(
                detail.contains("9007199254740993") && detail.contains("limit of 9007199254740992"),
                "{detail}"
            );
            if kind == "Fleet" {
                request.operation = Operation::Update;
                assert_eq!(
                    admission.admit(&request).await,
                    AdmissionResponse::Deny(detail)
                );
            }
            let pointer = if kind == "GameServer" {
                "/spec"
            } else {
                "/spec/template/spec"
            };
            object
                .pointer_mut(&format!(
                    "{pointer}/template/spec/{segment}/0/resources/requests/cpu"
                ))
                .unwrap()
                .clone_from(&json!("1000E"));
            object
                .pointer_mut(&format!(
                    "{pointer}/template/spec/{segment}/0/resources/limits/cpu"
                ))
                .unwrap()
                .clone_from(&json!("2"));
            request.object = object.clone();
            request.operation = Operation::Create;
            let AdmissionResponse::Deny(detail) = admission.admit(&request).await else {
                panic!("{kind} lost the decoded numeric quantity")
            };
            assert!(
                detail.contains(&format!(
                    "{path}Invalid: 1: must be less than or equal to cpu limit of 2"
                )),
                "{detail}"
            );
            // Quantity decoding precedes validation, including the dev-address early return.
            object
                .pointer_mut(&format!(
                    "{pointer}/template/spec/{segment}/0/resources/requests/cpu"
                ))
                .unwrap()
                .clone_from(&json!("invalid"));
            if kind == "GameServer" {
                object["metadata"]["annotations"]["agones.dev/dev-address"] = json!("127.0.0.1");
            }
            request.object = object;
            request.operation = Operation::Create;
            let AdmissionResponse::Deny(detail) = admission.admit(&request).await else {
                panic!("{kind} accepted malformed wire quantity")
            };
            assert!(detail.contains("quantities must match"), "{detail}");
        }
    }
}
