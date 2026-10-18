use agones_core::{AgonesBuildVersion, FeatureFlags, GameServerDefaultingContext};
use agones_rusternetes::GameServerCreationMutationAdmission;
use rusternetes_common::admission::{
    AdmissionController, AdmissionRequest, AdmissionResponse, Operation, UserInfo,
};
use serde_json::json;

#[tokio::test]
async fn actual_mutation_admission_consumes_composed_player_and_version_dependencies() {
    let object = json!({
        "spec":{"players":{"initialCapacity":7}},
        "status":{"players":null}
    });
    for enabled in [false, true] {
        let admission =
            GameServerCreationMutationAdmission::from_context(GameServerDefaultingContext {
                agones_build_version: AgonesBuildVersion::new(b"admission-version".to_vec()),
                game_server_feature_flags: FeatureFlags {
                    player_tracking: enabled,
                },
            });
        assert!(admission.supports_operation(&Operation::Create));
        assert!(!admission.supports_operation(&Operation::Update));
        let mut request = AdmissionRequest {
            operation: Operation::Create,
            kind: "GameServer".into(),
            namespace: Some("default".into()),
            name: "ordinary-witness".into(),
            object: object.clone(),
            old_object: None,
            user_info: UserInfo {
                username: "ordinary-witness".into(),
                uid: String::new(),
                groups: vec![],
            },
        };
        let response = admission.admit(&request).await;
        let patches = response.patches().expect("real admission mutation patches");
        let metadata = patches
            .iter()
            .find(|p| p.path == "/metadata")
            .unwrap()
            .value
            .as_ref()
            .unwrap();
        assert_eq!(
            metadata["annotations"]["agones.dev/sdk-version"],
            "admission-version"
        );
        let players = patches.iter().find(|p| p.path == "/status/players");
        if enabled {
            assert_eq!(
                players.unwrap().value.as_ref().unwrap(),
                &json!({"count":0,"capacity":7,"ids":null})
            );
        } else {
            assert!(players.is_none());
        }
        request.object = json!("invalid source object shape");
        assert_eq!(admission.admit(&request).await, AdmissionResponse::Allow);
        request.kind = "Pod".into();
        assert_eq!(admission.admit(&request).await, AdmissionResponse::Allow);
    }
}
