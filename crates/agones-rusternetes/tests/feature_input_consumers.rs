use std::process::Command;

use agones_core::{CanLoadFeatureGates, FEATURE_GATE_ENV, FeatureGateInputContext};
use agones_rusternetes::{
    CreatingGameServerController, FleetAutoscalerAdmissionContext, FleetAutoscalerController,
    SdkSidecarConfig, build_sdk_sidecar,
};
use rusternetes_storage::MemoryStorage;

#[test]
fn real_process_inputs_reach_default_and_explicit_production_consumers() {
    let executable = std::env::current_exe().unwrap();
    for case in [
        "environment",
        "changed-flag",
        "empty-flag",
        "override",
        "invalid",
    ] {
        let mut child = Command::new(&executable);
        child.args(["--exact", "production_input_child", "--ignored", "--nocapture"])
            .env("AGONES_PRODUCTION_INPUT_CASE", case)
            .env(FEATURE_GATE_ENV, if case == "invalid" {
                "SidecarContainers=invalid"
            } else {
                "SidecarContainers=false&ScheduledAutoscaler=false&FleetAutoscaleRequestMetaData=false&PlayerTracking=true"
            });
        let result = child.output().unwrap();
        let stdout = String::from_utf8_lossy(&result.stdout);
        assert!(
            result.status.success(),
            "{case}: {stdout}\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(
            stdout.contains("1 passed; 0 failed; 0 ignored"),
            "{case}: empty child selection: {stdout}"
        );
    }
}

#[test]
#[ignore = "runs only as the isolated production-input child"]
fn production_input_child() {
    let case = std::env::var("AGONES_PRODUCTION_INPUT_CASE").unwrap();
    let flag = match case.as_str() {
        "changed-flag" => {
            Some("SidecarContainers=true&ScheduledAutoscaler=true&PlayerTracking=false".into())
        }
        "empty-flag" => Some("".into()),
        _ => None,
    };
    let mut context = FeatureGateInputContext::from_parsed_flag(flag);
    if case == "override" {
        context.feature_gate_input_options.override_value = Some(b"PlayerTracking=false".to_vec());
        context.feature_gate_input_options.flag_value = Some(b"PlayerTracking=true".to_vec());
    }
    if case == "invalid" {
        let errors = [
            context
                .load_feature_gates()
                .err()
                .expect("invalid process input"),
            CreatingGameServerController::from_feature_inputs(MemoryStorage::new(), &context)
                .err()
                .expect("invalid controller input"),
            FleetAutoscalerController::from_feature_inputs(MemoryStorage::new(), &context)
                .err()
                .expect("invalid autoscaler input"),
            FleetAutoscalerAdmissionContext::from_feature_inputs(&context)
                .err()
                .expect("invalid admission input"),
            SdkSidecarConfig::from_feature_inputs("sdk:test", 1000, &context)
                .err()
                .expect("invalid SDK input"),
        ];
        for error in errors {
            assert_eq!(
                error.to_string(),
                "agones.dev/agones/pkg/util/runtime: error parsing bool value from flag SidecarContainers : strconv.ParseBool: parsing \"invalid\": invalid syntax"
            );
        }
        return;
    }
    let environment_selected = case == "environment";
    let server = CreatingGameServerController::from_feature_inputs(MemoryStorage::new(), &context)
        .unwrap()
        .with_sdk_sidecar_config(
            SdkSidecarConfig::from_feature_inputs("sdk:test", 1000, &context).unwrap(),
        );
    let autoscaler =
        FleetAutoscalerController::from_feature_inputs(MemoryStorage::new(), &context).unwrap();
    let admission = FleetAutoscalerAdmissionContext::from_feature_inputs(&context).unwrap();
    assert_eq!(
        server.pod_build_features.sidecar_containers,
        !environment_selected
    );
    assert_eq!(
        autoscaler.autoscaler_features.scheduled_autoscaler,
        !environment_selected
    );
    assert_eq!(
        autoscaler.fleet_autoscale_request_metadata,
        !environment_selected
    );
    assert_eq!(
        admission.autoscaler_features.scheduled_autoscaler,
        !environment_selected
    );
    let gates = server.feature_gates.as_ref().unwrap();
    assert_eq!(gates.enabled("PlayerTracking"), environment_selected);
    let sdk = build_sdk_sidecar(
        &agones_core::GameServer::default(),
        server.sdk_sidecar.as_ref().unwrap(),
    )
    .unwrap();
    let sdk_env = sdk.env.unwrap();
    assert_eq!(
        sdk_env
            .iter()
            .find(|value| value.name == FEATURE_GATE_ENV)
            .unwrap()
            .value
            .as_deref(),
        Some(gates.encode().as_str())
    );

    // The ordinary constructors are real entry paths too; they must read the
    // process configuration rather than silently initialize an empty query.
    let default_server = CreatingGameServerController::new(MemoryStorage::new());
    assert!(!default_server.pod_build_features.sidecar_containers);
    assert!(
        !FleetAutoscalerController::new(MemoryStorage::new())
            .autoscaler_features
            .scheduled_autoscaler
    );
    assert!(
        !FleetAutoscalerAdmissionContext::default()
            .autoscaler_features
            .scheduled_autoscaler
    );
    assert!(
        SdkSidecarConfig::new("sdk:test", 1000)
            .feature_gates
            .contains("PlayerTracking=true")
    );
}
