use agones_core::{
    ActivePeriod, Between, BufferPolicy, CanParseFeatureGates, FeatureGateContext, Fleet,
    FleetAutoscalerPolicy, FleetAutoscalerPolicyType, IntOrPercent, SchedulePolicy,
};
use agones_rusternetes::{
    CanComputeFleetAutoscalerDesiredSizeAt, CreatingGameServerController,
    FleetAutoscalerController, FleetAutoscalerInventory, SdkSidecarConfig,
};
use cgp::prelude::*;
use rusternetes_storage::MemoryStorage;

#[derive(HasField)]
struct MinimalFeatureContext {
    feature_gate_defaults: &'static [(&'static str, bool)],
}

delegate_and_check_components! {
    MinimalFeatureContext {
        agones_core::FeatureGateDefaultsGetterComponent: UseField<Symbol!("feature_gate_defaults")>,
        agones_core::FeatureGateParserComponent: agones_core::ParseAgonesFeatureGates,
    }
}

#[test]
fn the_same_portable_provider_runs_in_a_minimal_downstream_context() {
    let minimal = MinimalFeatureContext {
        feature_gate_defaults: &[("Example", true), ("Test", false)],
    };
    let production = FeatureGateContext {
        feature_gate_defaults: minimal.feature_gate_defaults,
    };
    for gates in [
        minimal.parse_feature_gates("Example=0&Test=1").unwrap(),
        production.parse_feature_gates("Example=0&Test=1").unwrap(),
    ] {
        assert!(!gates.enabled("Example"));
        assert!(gates.enabled("Test"));
        assert_eq!(gates.encode(), "Example=false&Test=true");
    }
}

#[test]
fn default_production_constructors_use_the_cgp_source_defaults() {
    let server = CreatingGameServerController::new(MemoryStorage::new());
    assert!(server.pod_build_features.sidecar_containers);
    let autoscaler = FleetAutoscalerController::new(MemoryStorage::new());
    assert!(autoscaler.autoscaler_features.scheduled_autoscaler);
    assert!(!autoscaler.autoscaler_features.wasm_autoscaler);
    assert!(autoscaler.fleet_autoscale_request_metadata);
    assert!(
        agones_rusternetes::FleetAutoscalerAdmissionContext::default()
            .autoscaler_features
            .scheduled_autoscaler
    );
    let sdk = SdkSidecarConfig::new("sdk:test", 1000);
    let gates = FeatureGateContext::default()
        .parse_feature_gates(&sdk.feature_gates)
        .unwrap();
    assert!(gates.enabled("SidecarContainers"));
    assert!(gates.enabled("CountsAndLists"));
    assert!(!gates.enabled("PlayerTracking"));
}

#[test]
fn configured_game_server_context_uses_source_sidecar_default_and_override() {
    let config = FeatureGateContext::default();
    let controller =
        CreatingGameServerController::from_feature_configuration(MemoryStorage::new(), &config, "")
            .unwrap();
    assert!(controller.pod_build_features.sidecar_containers);
    let controller = CreatingGameServerController::from_feature_configuration(
        MemoryStorage::new(),
        &config,
        "SidecarContainers=false",
    )
    .unwrap();
    assert!(!controller.pod_build_features.sidecar_containers);
    assert!(
        CreatingGameServerController::from_feature_configuration(
            MemoryStorage::new(),
            &config,
            "SidecarContainers=invalid",
        )
        .is_err()
    );
}

#[test]
fn configured_gates_reach_sdk_environment_in_either_builder_order() {
    let config = FeatureGateContext::default();
    let gates = config
        .parse_feature_gates("SidecarContainers=0&PlayerTracking=1")
        .unwrap();
    let early = CreatingGameServerController::new(MemoryStorage::new())
        .with_sdk_sidecar_config(SdkSidecarConfig::new("sdk:test", 1000))
        .with_feature_gates(&gates);
    let late = CreatingGameServerController::new(MemoryStorage::new())
        .with_feature_gates(&gates)
        .with_sdk_sidecar_config(SdkSidecarConfig::new("sdk:test", 1000));
    for controller in [early, late] {
        let sdk = agones_rusternetes::build_sdk_sidecar(
            &agones_core::GameServer::default(),
            controller.sdk_sidecar.as_ref().unwrap(),
        )
        .unwrap();
        let environment = sdk.env.unwrap();
        let query = environment
            .iter()
            .find(|value| value.name == "FEATURE_GATES")
            .unwrap()
            .value
            .as_ref()
            .unwrap();
        assert_eq!(config.parse_feature_gates(query).unwrap(), gates);
    }
}

#[test]
fn configured_autoscaler_context_preserves_independent_gate_values() {
    let config = FeatureGateContext::default();
    let controller =
        FleetAutoscalerController::from_feature_configuration(MemoryStorage::new(), &config, "")
            .unwrap();
    assert!(controller.autoscaler_features.scheduled_autoscaler);
    assert!(!controller.autoscaler_features.wasm_autoscaler);
    assert!(controller.fleet_autoscale_request_metadata);
    let controller = FleetAutoscalerController::from_feature_configuration(
        MemoryStorage::new(),
        &config,
        "ScheduledAutoscaler=0&WasmAutoscaler=1&FleetAutoscaleRequestMetaData=false",
    )
    .unwrap();
    assert!(!controller.autoscaler_features.scheduled_autoscaler);
    assert!(controller.autoscaler_features.wasm_autoscaler);
    assert!(!controller.fleet_autoscale_request_metadata);
    let gates = config.all_features_enabled();
    let controller =
        FleetAutoscalerController::new(MemoryStorage::new()).with_feature_gates(&gates);
    assert!(controller.autoscaler_features.scheduled_autoscaler);
    assert!(controller.autoscaler_features.wasm_autoscaler);
    assert!(controller.fleet_autoscale_request_metadata);
}

#[tokio::test]
async fn parsed_feature_gate_controls_the_actual_cgp_policy_dispatch() {
    let config = FeatureGateContext::default();
    let now = chrono::DateTime::parse_from_rfc3339("2021-01-01T00:30:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let policy = FleetAutoscalerPolicy {
        policy_type: FleetAutoscalerPolicyType::Schedule,
        schedule: Some(Box::new(SchedulePolicy {
            between: Between {
                start: Some("2021-01-01T00:00:00Z".into()),
                end: Some("2021-01-01T01:00:00Z".into()),
            },
            active_period: ActivePeriod::default(),
            policy: FleetAutoscalerPolicy {
                policy_type: FleetAutoscalerPolicyType::Buffer,
                buffer: Some(BufferPolicy {
                    buffer_size: IntOrPercent::Int(1),
                    min_replicas: 3,
                    max_replicas: 10,
                }),
                ..Default::default()
            },
        })),
        ..Default::default()
    };
    let mut fleet = Fleet::default();
    fleet.status.replicas = 2;
    for (query, enabled) in [("", true), ("ScheduledAutoscaler=0", false)] {
        let controller = FleetAutoscalerController::from_feature_configuration(
            MemoryStorage::new(),
            &config,
            query,
        )
        .unwrap();
        let decision = controller
            .compute_fleet_autoscaler_desired_size_at(
                &policy,
                &fleet,
                &FleetAutoscalerInventory::default(),
                "gate-test@0",
                now,
            )
            .await;
        if enabled {
            assert_eq!(decision.unwrap().replicas, 3);
        } else {
            assert!(
                decision
                    .unwrap_err()
                    .to_string()
                    .contains("feature flag ScheduledAutoscaler is enabled")
            );
        }
    }
}
