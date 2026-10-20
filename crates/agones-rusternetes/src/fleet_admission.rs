use async_trait::async_trait;
use agones_core::{Fleet, ValidationError};
use rusternetes_common::admission::{
    AdmissionController, AdmissionRequest, AdmissionResponse, Operation,
};

use crate::{
    AdapterError, ApiHooks, admission_json_patch,
    validate_game_server_resources_at,
};
use crate::gameserver_admission::format_validation_errors;

/// Upstream Fleet CREATE mutation.
///
/// Malformed JSON deliberately falls through without a patch so OpenAPI schema
/// validation can report the user-facing error, matching creationMutationHandler.
pub fn mutate_fleet_creation(
    object: &serde_json::Value,
) -> Result<AdmissionResponse, AdapterError> {
    mutate_fleet_creation_with_version_context(
        &agones_core::AgonesBuildVersionContext::default(),
        object,
    )
}

pub fn mutate_fleet_creation_with_version_context<C: agones_core::CanGetAgonesVersionAnnotation>(
    context: &C,
    object: &serde_json::Value,
) -> Result<AdmissionResponse, AdapterError> {
    let Ok(mut fleet) = serde_json::from_value::<Fleet>(object.clone()) else {
        return Ok(AdmissionResponse::Allow);
    };

    fleet.apply_defaults_with_version_context(context);
    let mutated = serde_json::to_value(fleet)?;
    let patches = admission_json_patch(object, &mutated);

    if patches.is_empty() {
        Ok(AdmissionResponse::Allow)
    } else {
        Ok(AdmissionResponse::AllowWithPatch(patches))
    }
}

/// Full upstream Fleet validation used for both CREATE and UPDATE.
///
/// Fleet::validate owns the runtime-independent strategy, nested GameServerSpec,
/// template metadata, and AllocationOverflow checks. The Rusternetes layer adds
/// the Kubernetes resource-quantity and cloud-product hook checks that upstream
/// reaches through validateGSSpec/APIHooks.
pub fn validate_fleet<H>(
    fleet: &Fleet,
    hooks: &H,
) -> Result<Vec<ValidationError>, AdapterError>
where
    H: ApiHooks,
{
    validate_fleet_with_context(
        &agones_core::GameServerDefaultingContext::default(),
        fleet,
        hooks,
    )
}

pub fn validate_fleet_with_context<C, H>(
    context: &C,
    fleet: &Fleet,
    hooks: &H,
) -> Result<Vec<ValidationError>, AdapterError>
where
    C: agones_core::CanValidateGameServerFeatures,
    H: ApiHooks,
{
    // Upstream Fleet.Validate mutates the nested GameServerSpec by applying
    // defaults through validateGSSpec. Validate a clone so admission remains a
    // pure decision over the supplied object.
    let mut validated = fleet.clone();
    let mut errors = validated.validate_with_context(context);

    let mut game_server_spec = fleet.spec.template.spec.clone();
    game_server_spec.apply_defaults();

    errors.extend(validate_game_server_resources_at(
        &game_server_spec,
        "spec.template.spec",
    )?);
    errors.extend(hooks.validate_game_server_spec_at(
        &game_server_spec,
        "spec.template.spec",
    ));
    errors.extend(hooks.validate_scheduling(
        fleet.spec.scheduling.as_ref(),
        "spec.scheduling",
    ));

    Ok(errors)
}

pub fn validate_fleet_object<H>(
    object: &serde_json::Value,
    hooks: &H,
) -> Result<AdmissionResponse, AdapterError>
where
    H: ApiHooks,
{
    validate_fleet_object_with_context(
        &agones_core::GameServerDefaultingContext::default(),
        object,
        hooks,
    )
}

pub fn validate_fleet_object_with_context<C, H>(
    context: &C,
    object: &serde_json::Value,
    hooks: &H,
) -> Result<AdmissionResponse, AdapterError>
where
    C: agones_core::CanValidateGameServerFeatures,
    H: ApiHooks,
{
    let fleet: Fleet =
        serde_json::from_value(object.clone()).map_err(|error| {
            AdapterError::Hook(format!(
                "error unmarshalling Fleet json after schema validation: {object}: {error}"
            ))
        })?;

    let errors = validate_fleet_with_context(context, &fleet, hooks)?;
    if errors.is_empty() {
        Ok(AdmissionResponse::Allow)
    } else {
        Ok(AdmissionResponse::Deny(format_validation_errors(
            &errors,
        )))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FleetCreationMutationAdmission;

#[async_trait]
impl AdmissionController for FleetCreationMutationAdmission {
    fn name(&self) -> &str {
        "AgonesFleetCreationMutation"
    }

    fn supports_operation(&self, operation: &Operation) -> bool {
        matches!(operation, Operation::Create)
    }

    async fn admit(&self, request: &AdmissionRequest) -> AdmissionResponse {
        if request.kind != "Fleet" {
            return AdmissionResponse::Allow;
        }

        mutate_fleet_creation(&request.object)
            .unwrap_or_else(|error| AdmissionResponse::Deny(error.to_string()))
    }
}

#[derive(Clone, Debug)]
pub struct FleetValidationAdmission<H, C = agones_core::GameServerDefaultingContext> {
    hooks: H,
    validation_context: C,
}

impl<H> FleetValidationAdmission<H> {
    pub fn new(hooks: H) -> Self {
        Self::from_feature_inputs(&agones_core::FeatureGateInputContext::default(), hooks)
            .expect("invalid Agones feature gate configuration")
    }

    pub fn from_feature_inputs<I: agones_core::CanLoadFeatureGates>(
        input: &I,
        hooks: H,
    ) -> Result<Self, agones_core::FeatureGateError> {
        let gates = input.load_feature_gates()?;
        Ok(Self::from_context(
            agones_core::GameServerDefaultingContext::from_feature_gates(&gates),
            hooks,
        ))
    }
}

impl<H, C> FleetValidationAdmission<H, C> {
    pub fn from_context(validation_context: C, hooks: H) -> Self {
        Self { hooks, validation_context }
    }
}

#[async_trait]
impl<H, C> AdmissionController for FleetValidationAdmission<H, C>
where
    C: agones_core::CanValidateGameServerFeatures + Send + Sync,
    H: ApiHooks + Send + Sync,
{
    fn name(&self) -> &str {
        "AgonesFleetValidation"
    }

    fn supports_operation(&self, operation: &Operation) -> bool {
        matches!(operation, Operation::Create | Operation::Update)
    }

    async fn admit(&self, request: &AdmissionRequest) -> AdmissionResponse {
        if request.kind != "Fleet" {
            return AdmissionResponse::Allow;
        }

        validate_fleet_object_with_context(&self.validation_context, &request.object, &self.hooks)
            .unwrap_or_else(|error| AdmissionResponse::Deny(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container, DeploymentStrategyType, FleetSpec, GameServerPort,
        GameServerTemplateSpec, IntOrPercent, ObjectMeta, PodSpec,
        PodTemplateSpec, PortPolicy, Protocol, RollingUpdateDeployment,
        SchedulingStrategy, ValidationErrorKind,
    };
    use rusternetes_common::admission::PatchOp;

    use crate::NoopApiHooks;

    use super::*;

    fn fixture() -> Fleet {
        Fleet {
            metadata: ObjectMeta {
                name: "fleet".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: FleetSpec {
                replicas: 3,
                template: GameServerTemplateSpec {
                    spec: agones_core::GameServerSpec {
                        ports: vec![GameServerPort {
                            container_port: 7777,
                            port_policy: Some(PortPolicy::Static),
                            protocol: Some(Protocol::Udp),
                            ..Default::default()
                        }].into(),
                        template: PodTemplateSpec {
                            spec: PodSpec {
                                containers: vec![Container {
                                    name: "game".into(),
                                    image: "game:v1".into(),
                                    ..Default::default()
                                }].into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn has_patch(
        response: &AdmissionResponse,
        op: PatchOp,
        path: &str,
    ) -> bool {
        let AdmissionResponse::AllowWithPatch(patches) = response else {
            return false;
        };
        patches.iter().any(|patch| patch.op == op && patch.path == path)
    }

    #[test]
    fn creation_mutation_applies_upstream_fleet_defaults() {
        let response = mutate_fleet_creation(
            &serde_json::to_value(fixture()).unwrap(),
        )
        .unwrap();

        assert!(response.is_allowed());
        assert!(has_patch(
            &response,
            PatchOp::Add,
            "/spec/strategy/type",
        ));
        assert!(has_patch(
            &response,
            PatchOp::Add,
            "/spec/scheduling",
        ));
        assert!(
            response
                .patches()
                .unwrap()
                .iter()
                .any(|patch| {
                    patch.path
                        == "/metadata/annotations/agones.dev~1sdk-version"
                        || patch.path == "/metadata/annotations"
                })
        );
    }

    #[test]
    fn creation_mutation_falls_through_on_malformed_object() {
        assert!(matches!(
            mutate_fleet_creation(&serde_json::json!("WRONG DATA")).unwrap(),
            AdmissionResponse::Allow
        ));
    }

    #[test]
    fn validation_accepts_defaulted_valid_fleet() {
        let mut fleet = fixture();
        fleet.apply_defaults();

        assert!(matches!(
            validate_fleet_object(
                &serde_json::to_value(fleet).unwrap(),
                &NoopApiHooks,
            )
            .unwrap(),
            AdmissionResponse::Allow
        ));
    }

    #[test]
    fn validation_rebases_nested_gameserver_and_resource_errors() {
        let mut fleet = fixture();
        fleet.apply_defaults();
        fleet
            .spec
            .template
            .spec
            .template
            .spec
            .containers
            .push(Container {
                name: "sidecar".into(),
                image: "sidecar:v1".into(),
                ..Default::default()
            });
        fleet.spec.template.spec.container = "missing".into();
        fleet.spec.template.spec.template.spec.containers[0]
            .resources
            .insert(
                "requests".into(),
                serde_json::json!({
                    "cpu": "-1"
                }),
            );

        let response = validate_fleet_object(
            &serde_json::to_value(fleet).unwrap(),
            &NoopApiHooks,
        )
        .unwrap();

        let AdmissionResponse::Deny(reason) = response else {
            panic!("invalid Fleet should be denied");
        };
        assert!(reason.contains("spec.template.spec.container"));
        assert!(reason.contains(
            "spec.template.spec.template.spec.containers[0].resources.requests[cpu]"
        ));
    }

    #[test]
    fn validation_preserves_rolling_update_constraints() {
        let mut fleet = fixture();
        fleet.apply_defaults();
        fleet.spec.strategy.strategy_type =
            Some(DeploymentStrategyType::RollingUpdate);
        fleet.spec.strategy.rolling_update =
            Some(RollingUpdateDeployment {
                max_unavailable: Some(IntOrPercent::String("0%".into())),
                max_surge: Some(IntOrPercent::Int(0)),
            });

        let response = validate_fleet_object(
            &serde_json::to_value(fleet).unwrap(),
            &NoopApiHooks,
        )
        .unwrap();

        let AdmissionResponse::Deny(reason) = response else {
            panic!("invalid rolling update should be denied");
        };
        assert!(reason.contains(
            "spec.strategy.rollingUpdate.maxUnavailable"
        ));
        assert!(reason.contains("spec.strategy.rollingUpdate.maxSurge"));
    }

    #[derive(Clone, Copy)]
    struct Hook;

    impl ApiHooks for Hook {
        fn validate_game_server_spec_at(
            &self,
            _spec: &agones_core::GameServerSpec,
            path: &str,
        ) -> Vec<ValidationError> {
            vec![ValidationError {
                kind: ValidationErrorKind::Invalid,
                path: format!("{path}.hook"),
                bad_value: None,
                detail: "gameserver hook".into(),
            }]
        }

        fn validate_scheduling(
            &self,
            _scheduling: Option<&SchedulingStrategy>,
            path: &str,
        ) -> Vec<ValidationError> {
            vec![ValidationError {
                kind: ValidationErrorKind::Invalid,
                path: path.into(),
                bad_value: None,
                detail: "scheduling hook".into(),
            }]
        }
    }

    #[test]
    fn validation_invokes_both_upstream_api_hook_boundaries() {
        let mut fleet = fixture();
        fleet.apply_defaults();
        let errors = validate_fleet(&fleet, &Hook).unwrap();

        assert!(errors.iter().any(|error| {
            error.path == "spec.template.spec.hook"
                && error.detail == "gameserver hook"
        }));
        assert!(errors.iter().any(|error| {
            error.path == "spec.scheduling"
                && error.detail == "scheduling hook"
        }));
    }

    #[test]
    fn same_validation_adapter_handles_create_and_update() {
        let validation = FleetValidationAdmission::new(NoopApiHooks);

        assert!(validation.supports_operation(&Operation::Create));
        assert!(validation.supports_operation(&Operation::Update));
        assert!(!validation.supports_operation(&Operation::Delete));

        let mutation = FleetCreationMutationAdmission;
        assert!(mutation.supports_operation(&Operation::Create));
        assert!(!mutation.supports_operation(&Operation::Update));
    }

    #[test]
    fn update_is_full_validation_not_gameserverset_style_immutability() {
        let mut fleet = fixture();
        fleet.apply_defaults();
        fleet.spec.replicas = 10;
        fleet.spec.scheduling = Some(SchedulingStrategy::Distributed);

        // Upstream Fleet UPDATE reuses creationValidationHandler and does not
        // compare old_object or freeze spec.template.
        assert!(matches!(
            validate_fleet_object(
                &serde_json::to_value(fleet).unwrap(),
                &NoopApiHooks,
            )
            .unwrap(),
            AdmissionResponse::Allow
        ));
    }

    #[test]
    fn malformed_validation_object_is_denied_after_schema_boundary() {
        let error = validate_fleet_object(
            &serde_json::json!("WRONG DATA"),
            &NoopApiHooks,
        )
        .unwrap_err();

        assert!(error.to_string().contains("unmarshalling Fleet"));
    }
}
