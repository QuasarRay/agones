use std::process::Command;

use agones_core::{
    FEATURE_GATE_ENV, FeatureGateInputContext, Fleet, GameServer, GameServerSet, PlayersSpec,
};
use agones_rusternetes::{
    FleetValidationAdmission, GameServerCreationValidationAdmission,
    GameServerSetCreationValidationAdmission, GameServerSetUpdateValidationAdmission, NoopApiHooks,
    validate_fleet_with_context, validate_game_server_creation_with_context,
    validate_game_server_set_creation_with_context, validate_game_server_with_context,
};
use rusternetes_common::admission::{
    AdmissionController, AdmissionRequest, AdmissionResponse, Operation, UserInfo,
};
use serde_json::{Value, json};

fn objects(players: bool) -> [(&'static str, Value); 3] {
    let mut gs: GameServer = serde_json::from_value(json!({
        "metadata":{"name":"game"},
        "spec":{"template":{"spec":{"containers":[{"name":"game","image":"game:v1"}]}}}
    }))
    .unwrap();
    gs.apply_defaults();
    gs.spec.players = players.then_some(PlayersSpec::default());
    let mut fleet = Fleet::default();
    fleet.spec.template.spec = gs.spec.clone();
    fleet.apply_defaults();
    let mut set = GameServerSet::default();
    set.spec.template.spec = gs.spec.clone();
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
            username: "validation-witness".into(),
            uid: String::new(),
            groups: vec![],
        },
    }
}

async fn check_admissions(admissions: &[Box<dyn AdmissionController>], enabled: bool) {
    assert_eq!(admissions.len(), 3);
    for players in [false, true] {
        for (admission, (kind, object)) in admissions.iter().zip(objects(players)) {
            assert!(admission.supports_operation(&Operation::Create));
            assert_eq!(
                admission.supports_operation(&Operation::Update),
                kind == "Fleet"
            );
            let mut request = request(kind, object);
            let response = admission.admit(&request).await;
            if !enabled && players {
                let AdmissionResponse::Deny(detail) = &response else {
                    panic!("{kind} must forbid players: {response:?}");
                };
                let path = if kind == "GameServer" {
                    "spec.players"
                } else {
                    "spec.template.spec.players"
                };
                assert_eq!(
                    detail,
                    &format!(
                        "{path}Forbidden: Value cannot be set unless feature flag PlayerTracking is enabled"
                    )
                );
            } else {
                assert_eq!(response, AdmissionResponse::Allow, "{kind}");
            }
            if kind == "Fleet" {
                request.operation = Operation::Update;
                assert_eq!(admission.admit(&request).await, response);
            }
            request.object = json!("malformed resource");
            assert!(matches!(
                admission.admit(&request).await,
                AdmissionResponse::Deny(_)
            ));
            request.kind = "Pod".into();
            assert_eq!(admission.admit(&request).await, AdmissionResponse::Allow);
        }
    }
}

#[tokio::test]
async fn explicit_feature_input_reaches_all_actual_validation_admissions() {
    for enabled in [false, true] {
        let input = FeatureGateInputContext::from_parsed_flag(Some(
            format!("PlayerTracking={enabled}").into(),
        ));
        let admissions: Vec<Box<dyn AdmissionController>> = vec![
            Box::new(
                GameServerCreationValidationAdmission::from_feature_inputs(&input, NoopApiHooks)
                    .unwrap(),
            ),
            Box::new(FleetValidationAdmission::from_feature_inputs(&input, NoopApiHooks).unwrap()),
            Box::new(
                GameServerSetCreationValidationAdmission::from_feature_inputs(&input, NoopApiHooks)
                    .unwrap(),
            ),
        ];
        check_admissions(&admissions, enabled).await;
    }
}

#[test]
fn direct_adapter_validation_accepts_the_same_explicit_feature_context() {
    for enabled in [false, true] {
        let context = agones_core::GameServerDefaultingContext {
            game_server_feature_flags: agones_core::FeatureFlags {
                player_tracking: enabled,
            },
            ..Default::default()
        };
        let [(_, gs), (_, fleet), (_, set)] = objects(true);
        let gs = serde_json::from_value(gs).unwrap();
        let fleet = serde_json::from_value(fleet).unwrap();
        let set = serde_json::from_value(set).unwrap();
        for (result, path) in [
            (
                validate_game_server_with_context(&context, &gs),
                "spec.players",
            ),
            (
                validate_game_server_creation_with_context(&context, &gs, &NoopApiHooks),
                "spec.players",
            ),
            (
                validate_fleet_with_context(&context, &fleet, &NoopApiHooks),
                "spec.template.spec.players",
            ),
            (
                validate_game_server_set_creation_with_context(&context, &set, &NoopApiHooks),
                "spec.template.spec.players",
            ),
        ] {
            let errors = result.unwrap();
            if enabled {
                assert!(errors.is_empty());
            } else {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].path, path);
                assert_eq!(errors[0].kind, agones_core::ValidationErrorKind::Forbidden);
                assert_eq!(errors[0].bad_value, None);
            }
        }
    }
}

#[test]
fn invalid_feature_configuration_is_returned_by_all_admission_factories() {
    let input = FeatureGateInputContext::from_parsed_flag(Some("PlayerTracking=invalid".into()));
    let errors = [
        GameServerCreationValidationAdmission::from_feature_inputs(&input, NoopApiHooks)
            .unwrap_err(),
        FleetValidationAdmission::from_feature_inputs(&input, NoopApiHooks).unwrap_err(),
        GameServerSetCreationValidationAdmission::from_feature_inputs(&input, NoopApiHooks)
            .unwrap_err(),
    ];
    for error in errors {
        assert_eq!(
            error.to_string(),
            "agones.dev/agones/pkg/util/runtime: error parsing bool value from flag PlayerTracking : strconv.ParseBool: parsing \"invalid\": invalid syntax"
        );
    }
}

#[tokio::test]
async fn gameserverset_update_keeps_its_separate_immutable_template_policy() {
    let (_, object) = objects(true).into_iter().last().unwrap();
    let mut request = request("GameServerSet", object.clone());
    request.operation = Operation::Update;
    request.old_object = Some(object);
    // The source update path checks template immutability, without rerunning
    // the CREATE feature check against unchanged existing players.
    assert_eq!(
        GameServerSetUpdateValidationAdmission.admit(&request).await,
        AdmissionResponse::Allow
    );
}

#[tokio::test]
async fn development_admission_keeps_the_feature_check_before_resource_and_container_skips() {
    for enabled in [false, true] {
        let input = FeatureGateInputContext::from_parsed_flag(Some(
            format!("PlayerTracking={enabled}").into(),
        ));
        let admission =
            GameServerCreationValidationAdmission::from_feature_inputs(&input, NoopApiHooks)
                .unwrap();
        let object = json!({
            "metadata":{"annotations":{"agones.dev/dev-address":"127.0.0.1"}},
            "spec":{"players":{},"container":"missing","template":{"spec":{"containers":[{
                "name":"game","resources":{"requests":{"cpu":"invalid-quantity"}}
            }]}}}
        });
        let response = admission.admit(&request("GameServer", object)).await;
        assert_eq!(
            response,
            if enabled {
                AdmissionResponse::Allow
            } else {
                AdmissionResponse::Deny("spec.playersForbidden: Value cannot be set unless feature flag PlayerTracking is enabled".into())
            }
        );
    }
}

#[test]
fn default_admission_constructors_capture_real_process_configuration() {
    for enabled in [false, true] {
        let mut child = Command::new(std::env::current_exe().unwrap());
        child
            .args([
                "--exact",
                "validation_configuration_child",
                "--ignored",
                "--nocapture",
            ])
            .env("AGONES_VALIDATION_EXPECT_ENABLED", enabled.to_string());
        if enabled {
            child.env(FEATURE_GATE_ENV, "PlayerTracking=true");
        } else {
            child.env_remove(FEATURE_GATE_ENV);
        }
        let result = child.output().unwrap();
        let stdout = String::from_utf8_lossy(&result.stdout);
        assert!(
            result.status.success(),
            "{stdout}\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(
            stdout.contains("1 passed; 0 failed; 0 ignored"),
            "empty selection: {stdout}"
        );
    }
}

#[tokio::test]
#[ignore = "runs in isolated production-configuration child"]
async fn validation_configuration_child() {
    let enabled = std::env::var("AGONES_VALIDATION_EXPECT_ENABLED").unwrap() == "true";
    let admissions: Vec<Box<dyn AdmissionController>> = vec![
        Box::new(GameServerCreationValidationAdmission::new(NoopApiHooks)),
        Box::new(FleetValidationAdmission::new(NoopApiHooks)),
        Box::new(GameServerSetCreationValidationAdmission::new(NoopApiHooks)),
    ];
    check_admissions(&admissions, enabled).await;
}
