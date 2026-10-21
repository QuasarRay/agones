use async_trait::async_trait;
use agones_core::{GameServerSet, ValidationError};
use rusternetes_common::admission::{
    AdmissionController, AdmissionRequest, AdmissionResponse, Operation,
};

use crate::{
    AdapterError, ApiHooks,
};
use crate::gameserver_admission::format_validation_errors;

/// Full upstream GameServerSet CREATE validation.
///
/// `GameServerSet::validate` owns the runtime-independent domain checks. The
/// Rusternetes layer adds the validation that upstream delegates through
/// `GameServerSpec.Validate` to Kubernetes resource parsing and cloud-product
/// API hooks. The nested spec path is preserved exactly:
/// `spec.template.spec.template.spec...`.
pub fn validate_game_server_set_creation<H>(
    game_server_set: &GameServerSet,
    hooks: &H,
) -> Result<Vec<ValidationError>, AdapterError>
where
    H: ApiHooks,
{
    validate_game_server_set_creation_with_context(
        &agones_core::GameServerDefaultingContext::default(),
        game_server_set,
        hooks,
    )
}

pub fn validate_game_server_set_creation_with_context<C, H>(
    context: &C,
    game_server_set: &GameServerSet,
    hooks: &H,
) -> Result<Vec<ValidationError>, AdapterError>
where
    C: agones_core::CanValidateGameServerFeatures + agones_core::CanValidateResourceQuantities + agones_core::CanValidateObjectMetadata,
    H: ApiHooks,
{
    let mut errors = game_server_set.validate_with_context(context);

    // validateGSSpec applies GameServer defaults before validating the nested
    // spec. GameServerSet::validate does the same for domain checks; repeat on
    // a local clone for runtime/resource and hook validation.
    let mut game_server_spec = game_server_set.spec.template.spec.clone();
    game_server_spec.apply_defaults();

    let _pod_spec: rusternetes_common::resources::PodSpec = crate::to_native(&game_server_spec.template.spec)?;
    errors.extend(hooks.validate_game_server_spec_at(
        &game_server_spec,
        "spec.template.spec",
    ));
    errors.extend(hooks.validate_scheduling(
        game_server_set.spec.scheduling.as_ref(),
        "spec.scheduling",
    ));

    Ok(errors)
}

pub fn validate_game_server_set_creation_object<H>(
    object: &serde_json::Value,
    hooks: &H,
) -> Result<AdmissionResponse, AdapterError>
where
    H: ApiHooks,
{
    validate_game_server_set_creation_object_with_context(
        &agones_core::GameServerDefaultingContext::default(),
        object,
        hooks,
    )
}

pub fn validate_game_server_set_creation_object_with_context<C, H>(
    context: &C,
    object: &serde_json::Value,
    hooks: &H,
) -> Result<AdmissionResponse, AdapterError>
where
    C: agones_core::CanValidateGameServerFeatures + agones_core::CanValidateResourceQuantities + agones_core::CanValidateObjectMetadata,
    H: ApiHooks,
{
    let game_server_set: GameServerSet =
        serde_json::from_value(object.clone()).map_err(|error| {
            AdapterError::Hook(format!(
                "error unmarshalling GameServerSet json after schema validation: {object}: {error}"
            ))
        })?;

    let errors = validate_game_server_set_creation_with_context(context, &game_server_set, hooks)?;
    if errors.is_empty() {
        Ok(AdmissionResponse::Allow)
    } else {
        Ok(AdmissionResponse::Deny(format_validation_errors(
            &errors,
        )))
    }
}

/// Upstream GameServerSet UPDATE validation.
///
/// Only the name constraint and immutable template rule apply on update;
/// replica/scheduling/allocation-overflow changes remain allowed, matching
/// `GameServerSet.ValidateUpdate`.
pub fn validate_game_server_set_update(
    old_game_server_set: &GameServerSet,
    new_game_server_set: &GameServerSet,
) -> Vec<ValidationError> {
    old_game_server_set.validate_update(new_game_server_set)
}

pub fn validate_game_server_set_update_objects(
    old_object: &serde_json::Value,
    new_object: &serde_json::Value,
) -> Result<AdmissionResponse, AdapterError> {
    let new_game_server_set: GameServerSet =
        serde_json::from_value(new_object.clone()).map_err(|error| {
            AdapterError::Hook(format!(
                "error unmarshalling new GameServerSet json: {new_object}: {error}"
            ))
        })?;
    let old_game_server_set: GameServerSet =
        serde_json::from_value(old_object.clone()).map_err(|error| {
            AdapterError::Hook(format!(
                "error unmarshalling old GameServerSet json: {old_object}: {error}"
            ))
        })?;

    let errors = validate_game_server_set_update(
        &old_game_server_set,
        &new_game_server_set,
    );
    if errors.is_empty() {
        Ok(AdmissionResponse::Allow)
    } else {
        Ok(AdmissionResponse::Deny(format_validation_errors(
            &errors,
        )))
    }
}

#[derive(Clone, Debug)]
pub struct GameServerSetCreationValidationAdmission<H, C = agones_core::GameServerDefaultingContext> {
    hooks: H,
    validation_context: C,
}

impl<H> GameServerSetCreationValidationAdmission<H> {
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

impl<H, C> GameServerSetCreationValidationAdmission<H, C> {
    pub fn from_context(validation_context: C, hooks: H) -> Self {
        Self { hooks, validation_context }
    }
}

#[async_trait]
impl<H, C> AdmissionController for GameServerSetCreationValidationAdmission<H, C>
where
    C: agones_core::CanValidateGameServerFeatures + agones_core::CanValidateResourceQuantities + agones_core::CanValidateObjectMetadata + Send + Sync,
    H: ApiHooks + Send + Sync,
{
    fn name(&self) -> &str {
        "AgonesGameServerSetCreationValidation"
    }

    fn supports_operation(&self, operation: &Operation) -> bool {
        matches!(operation, Operation::Create)
    }

    async fn admit(&self, request: &AdmissionRequest) -> AdmissionResponse {
        if request.kind != "GameServerSet" {
            return AdmissionResponse::Allow;
        }

        validate_game_server_set_creation_object_with_context(
            &self.validation_context,
            &request.object,
            &self.hooks,
        )
        .unwrap_or_else(|error| AdmissionResponse::Deny(error.to_string()))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GameServerSetUpdateValidationAdmission;

#[async_trait]
impl AdmissionController for GameServerSetUpdateValidationAdmission {
    fn name(&self) -> &str {
        "AgonesGameServerSetUpdateValidation"
    }

    fn supports_operation(&self, operation: &Operation) -> bool {
        matches!(operation, Operation::Update)
    }

    async fn admit(&self, request: &AdmissionRequest) -> AdmissionResponse {
        if request.kind != "GameServerSet" {
            return AdmissionResponse::Allow;
        }

        let Some(old_object) = request.old_object.as_ref() else {
            return AdmissionResponse::Deny(
                "GameServerSet UPDATE admission requires old_object".into(),
            );
        };

        validate_game_server_set_update_objects(old_object, &request.object)
            .unwrap_or_else(|error| AdmissionResponse::Deny(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use agones_core::{
        Container, GameServerPort, GameServerSetSpec, GameServerTemplateSpec,
        ObjectMeta, PodSpec, PodTemplateSpec, PortPolicy, Protocol,
        SchedulingStrategy, ValidationErrorKind,
    };

    use crate::NoopApiHooks;

    use super::*;

    fn fixture() -> GameServerSet {
        GameServerSet {
            metadata: ObjectMeta {
                name: "set".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                replicas: 3,
                scheduling: Some(SchedulingStrategy::Packed),
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

    #[test]
    fn creation_validation_accepts_valid_gameserverset() {
        let response = validate_game_server_set_creation_object(
            &serde_json::to_value(fixture()).unwrap(),
            &NoopApiHooks,
        )
        .unwrap();

        assert!(matches!(response, AdmissionResponse::Allow));
    }

    #[test]
    fn creation_validation_rebases_nested_gameserver_errors() {
        let mut set = fixture();
        set.spec.template.spec.template.spec.containers.push(Container {
            name: "sidecar".into(),
            image: "sidecar:v1".into(),
            ..Default::default()
        });
        set.spec.template.spec.container = "missing".into();

        let response = validate_game_server_set_creation_object(
            &serde_json::to_value(set).unwrap(),
            &NoopApiHooks,
        )
        .unwrap();

        let AdmissionResponse::Deny(reason) = response else {
            panic!("invalid GameServerSet should be denied");
        };
        assert!(reason.contains("spec.template.spec.container"));
    }

    #[test]
    fn creation_validation_reports_nested_resource_paths() {
        let mut set = fixture();
        set.spec.template.spec.template.spec.containers[0]
            .resources.requests = Some(serde_json::from_value(serde_json::json!({"cpu": "-1"})).unwrap());

        let response = validate_game_server_set_creation_object(
            &serde_json::to_value(set).unwrap(),
            &NoopApiHooks,
        )
        .unwrap();

        let AdmissionResponse::Deny(reason) = response else {
            panic!("negative resource request should be denied");
        };
        assert!(reason.contains(
            "spec.template.spec.template.spec.containers[0].resources.requests[cpu]"
        ));
    }

    #[derive(Clone, Copy)]
    struct SchedulingHook;

    impl ApiHooks for SchedulingHook {
        fn validate_scheduling(
            &self,
            scheduling: Option<&SchedulingStrategy>,
            path: &str,
        ) -> Vec<ValidationError> {
            if matches!(scheduling, Some(SchedulingStrategy::Distributed)) {
                vec![ValidationError {
                    origin: String::new(),
                    covered_by_declarative: false,
                    kind: ValidationErrorKind::Invalid,
                    path: path.into(),
                    bad_value: Some("Distributed".into()),
                    detail: "test hook rejects distributed scheduling".into(),
                }]
            } else {
                Vec::new()
            }
        }
    }

    #[test]
    fn creation_validation_invokes_scheduling_hook_with_nested_path() {
        let mut set = fixture();
        set.spec.scheduling = Some(SchedulingStrategy::Distributed);

        let errors =
            validate_game_server_set_creation(&set, &SchedulingHook).unwrap();

        assert!(errors.iter().any(|error| {
            error.path == "spec.scheduling"
                && error.detail == "test hook rejects distributed scheduling"
        }));
    }

    #[test]
    fn update_allows_replica_change_but_rejects_template_change() {
        let old = fixture();

        let mut replicas = old.clone();
        replicas.spec.replicas = 8;
        assert!(matches!(
            validate_game_server_set_update_objects(
                &serde_json::to_value(&old).unwrap(),
                &serde_json::to_value(&replicas).unwrap(),
            )
            .unwrap(),
            AdmissionResponse::Allow
        ));

        let mut template = old.clone();
        template.spec.template.spec.ports[0].container_port = 9000;
        let response = validate_game_server_set_update_objects(
            &serde_json::to_value(&old).unwrap(),
            &serde_json::to_value(&template).unwrap(),
        )
        .unwrap();

        let AdmissionResponse::Deny(reason) = response else {
            panic!("template mutation should be denied");
        };
        assert!(reason.contains("spec.template"));
    }

    #[test]
    fn creation_and_update_adapters_are_operation_specific() {
        use rusternetes_common::admission::AdmissionController;

        let create = GameServerSetCreationValidationAdmission::new(NoopApiHooks);
        let update = GameServerSetUpdateValidationAdmission;

        assert!(create.supports_operation(&Operation::Create));
        assert!(!create.supports_operation(&Operation::Update));
        assert!(update.supports_operation(&Operation::Update));
        assert!(!update.supports_operation(&Operation::Create));
    }

    #[test]
    fn malformed_creation_object_is_denied_after_schema_validation_boundary() {
        let error = validate_game_server_set_creation_object(
            &serde_json::json!("WRONG DATA"),
            &NoopApiHooks,
        )
        .unwrap_err();

        assert!(error.to_string().contains("unmarshalling GameServerSet"));
    }

    #[test]
    fn update_validation_does_not_revalidate_mutable_scheduling() {
        let old = fixture();
        let mut new = old.clone();
        new.spec.scheduling = Some(SchedulingStrategy::Distributed);

        let errors = validate_game_server_set_update(&old, &new);
        assert!(errors.is_empty());
    }

    #[test]
    fn hook_can_return_multiple_errors_without_losing_order() {
        struct Hook;
        impl ApiHooks for Hook {
            fn validate_scheduling(
                &self,
                _scheduling: Option<&SchedulingStrategy>,
                path: &str,
            ) -> Vec<ValidationError> {
                vec![
                    ValidationError {
                        origin: String::new(),
                        covered_by_declarative: false,
                        kind: ValidationErrorKind::Invalid,
                        path: path.into(),
                        bad_value: None,
                        detail: "first".into(),
                    },
                    ValidationError {
                        origin: String::new(),
                        covered_by_declarative: false,
                        kind: ValidationErrorKind::Invalid,
                        path: format!("{path}.secondary"),
                        bad_value: None,
                        detail: "second".into(),
                    },
                ]
            }
        }

        let errors = validate_game_server_set_creation(&fixture(), &Hook).unwrap();
        let hook_errors: Vec<_> = errors
            .iter()
            .filter(|error| error.detail == "first" || error.detail == "second")
            .collect();
        assert_eq!(hook_errors.len(), 2);
        assert_eq!(hook_errors[0].detail, "first");
        assert_eq!(hook_errors[1].detail, "second");
    }

    #[test]
    fn template_metadata_validation_remains_core_owned() {
        let mut set = fixture();
        set.spec.template.metadata.labels = BTreeMap::from([
            ("bad key".into(), "value".into()),
        ]);

        let errors =
            validate_game_server_set_creation(&set, &NoopApiHooks).unwrap();
        assert!(errors.iter().any(|error| {
            error.path.starts_with("spec.template.metadata")
        }));
    }
}
