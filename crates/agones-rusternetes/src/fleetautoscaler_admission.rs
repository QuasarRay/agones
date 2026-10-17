use agones_core::{
    AutoscalerFeatures, FleetAutoscaler, ValidationError,
};
use async_trait::async_trait;
use cgp::prelude::*;
use rusternetes_common::admission::{
    AdmissionController, AdmissionRequest, AdmissionResponse, Operation,
};

use crate::{AdapterError, admission_json_patch};
use crate::gameserver_admission::format_validation_errors;

/// Admission-time FleetAutoscaler dependencies.
///
/// Feature-gate state is a context field rather than a constructor argument
/// threaded through every helper. CGP lets mutation and validation providers
/// select their dependencies from the context at compile time.
#[derive(Clone, Copy, Debug, Default, HasField)]
pub struct FleetAutoscalerAdmissionContext {
    pub autoscaler_features: AutoscalerFeatures,
}

impl FleetAutoscalerAdmissionContext {
    pub const fn new(autoscaler_features: AutoscalerFeatures) -> Self {
        Self {
            autoscaler_features,
        }
    }
}

#[cgp_getter]
pub trait HasFleetAutoscalerFeatures {
    fn autoscaler_features(&self) -> &AutoscalerFeatures;
}

#[cgp_component(FleetAutoscalerDefaulter)]
pub trait CanDefaultFleetAutoscaler {
    fn default_fleet_autoscaler(
        &self,
        fleet_autoscaler: &mut FleetAutoscaler,
    );
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerDefaulter<Context>
    for DefaultFleetAutoscalerWithCore
{
    fn default_fleet_autoscaler(
        _context: &Context,
        fleet_autoscaler: &mut FleetAutoscaler,
    ) {
        fleet_autoscaler.apply_defaults();
    }
}

#[cgp_component(FleetAutoscalerValidator)]
pub trait CanValidateFleetAutoscaler {
    fn validate_fleet_autoscaler(
        &self,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Vec<ValidationError>;
}

#[cgp_new_provider]
impl<Context> FleetAutoscalerValidator<Context>
    for ValidateFleetAutoscalerWithCore
where
    Context: HasFleetAutoscalerFeatures,
{
    fn validate_fleet_autoscaler(
        context: &Context,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Vec<ValidationError> {
        fleet_autoscaler.validate(*context.autoscaler_features())
    }
}

delegate_and_check_components! {
    FleetAutoscalerAdmissionContext {
        FleetAutoscalerFeaturesGetterComponent:
            UseField<Symbol!("autoscaler_features")>,
        FleetAutoscalerDefaulterComponent:
            DefaultFleetAutoscalerWithCore,
        FleetAutoscalerValidatorComponent:
            ValidateFleetAutoscalerWithCore,
    }
}

/// Upstream FleetAutoscaler CREATE/UPDATE mutation.
///
/// Invalid JSON deliberately falls through without a patch so the schema layer
/// can produce the user-facing validation error, matching the Go webhook.
pub fn mutate_fleet_autoscaler_object<C>(
    context: &C,
    object: &serde_json::Value,
) -> Result<AdmissionResponse, AdapterError>
where
    C: CanDefaultFleetAutoscaler,
{
    let Ok(mut fleet_autoscaler) =
        serde_json::from_value::<FleetAutoscaler>(object.clone())
    else {
        return Ok(AdmissionResponse::Allow);
    };

    context.default_fleet_autoscaler(&mut fleet_autoscaler);
    let mutated = serde_json::to_value(fleet_autoscaler)?;
    let patches = admission_json_patch(object, &mutated);

    if patches.is_empty() {
        Ok(AdmissionResponse::Allow)
    } else {
        Ok(AdmissionResponse::AllowWithPatch(patches))
    }
}

/// Upstream FleetAutoscaler CREATE/UPDATE validation.
///
/// Validation applies defaults first, exactly as validationHandler does after
/// schema validation, then evaluates the selected policy under the configured
/// Agones feature gates.
pub fn validate_fleet_autoscaler_object<C>(
    context: &C,
    object: &serde_json::Value,
) -> Result<AdmissionResponse, AdapterError>
where
    C: CanDefaultFleetAutoscaler + CanValidateFleetAutoscaler,
{
    let mut fleet_autoscaler: FleetAutoscaler =
        serde_json::from_value(object.clone()).map_err(|error| {
            AdapterError::Hook(format!(
                "error unmarshalling FleetAutoscaler json after schema validation: {object}: {error}"
            ))
        })?;

    context.default_fleet_autoscaler(&mut fleet_autoscaler);
    let errors =
        context.validate_fleet_autoscaler(&fleet_autoscaler);

    if errors.is_empty() {
        Ok(AdmissionResponse::Allow)
    } else {
        Ok(AdmissionResponse::Deny(format_validation_errors(
            &errors,
        )))
    }
}

#[derive(Clone, Debug)]
pub struct FleetAutoscalerMutationAdmission<C> {
    context: C,
}

impl<C> FleetAutoscalerMutationAdmission<C> {
    pub const fn new(context: C) -> Self {
        Self { context }
    }

    pub fn context(&self) -> &C {
        &self.context
    }
}

#[async_trait]
impl<C> AdmissionController for FleetAutoscalerMutationAdmission<C>
where
    C: CanDefaultFleetAutoscaler + Send + Sync,
{
    fn name(&self) -> &str {
        "AgonesFleetAutoscalerMutation"
    }

    fn supports_operation(&self, operation: &Operation) -> bool {
        matches!(operation, Operation::Create | Operation::Update)
    }

    async fn admit(
        &self,
        request: &AdmissionRequest,
    ) -> AdmissionResponse {
        if request.kind != "FleetAutoscaler" {
            return AdmissionResponse::Allow;
        }

        mutate_fleet_autoscaler_object(
            &self.context,
            &request.object,
        )
        .unwrap_or_else(|error| {
            AdmissionResponse::Deny(error.to_string())
        })
    }
}

#[derive(Clone, Debug)]
pub struct FleetAutoscalerValidationAdmission<C> {
    context: C,
}

impl<C> FleetAutoscalerValidationAdmission<C> {
    pub const fn new(context: C) -> Self {
        Self { context }
    }

    pub fn context(&self) -> &C {
        &self.context
    }
}

#[async_trait]
impl<C> AdmissionController for FleetAutoscalerValidationAdmission<C>
where
    C: CanDefaultFleetAutoscaler
        + CanValidateFleetAutoscaler
        + Send
        + Sync,
{
    fn name(&self) -> &str {
        "AgonesFleetAutoscalerValidation"
    }

    fn supports_operation(&self, operation: &Operation) -> bool {
        matches!(operation, Operation::Create | Operation::Update)
    }

    async fn admit(
        &self,
        request: &AdmissionRequest,
    ) -> AdmissionResponse {
        if request.kind != "FleetAutoscaler" {
            return AdmissionResponse::Allow;
        }

        validate_fleet_autoscaler_object(
            &self.context,
            &request.object,
        )
        .unwrap_or_else(|error| {
            AdmissionResponse::Deny(error.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use agones_core::{
        ActivePeriod, BufferPolicy, FleetAutoscalerPolicy,
        FleetAutoscalerPolicyType, FleetAutoscalerSpec, IntOrPercent,
        ObjectMeta, SchedulePolicy,
    };
    use rusternetes_common::admission::PatchOp;

    use super::*;

    fn buffer_fixture() -> FleetAutoscaler {
        FleetAutoscaler {
            metadata: ObjectMeta {
                name: "autoscaler".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: FleetAutoscalerSpec {
                fleet_name: "fleet".into(),
                policy: FleetAutoscalerPolicy {
                    policy_type: FleetAutoscalerPolicyType::Buffer,
                    buffer: Some(BufferPolicy {
                        buffer_size: IntOrPercent::Int(2),
                        min_replicas: 2,
                        max_replicas: 20,
                    }),
                    ..Default::default()
                },
                sync: None,
            },
            ..Default::default()
        }
    }

    fn context() -> FleetAutoscalerAdmissionContext {
        FleetAutoscalerAdmissionContext::default()
    }

    fn has_patch(
        response: &AdmissionResponse,
        op: PatchOp,
        path: &str,
    ) -> bool {
        let AdmissionResponse::AllowWithPatch(patches) = response
        else {
            return false;
        };
        patches
            .iter()
            .any(|patch| patch.op == op && patch.path == path)
    }

    #[test]
    fn mutation_applies_fixed_interval_defaults_on_create_and_update() {
        let object =
            serde_json::to_value(buffer_fixture()).unwrap();
        let response =
            mutate_fleet_autoscaler_object(&context(), &object)
                .unwrap();

        assert!(has_patch(
            &response,
            PatchOp::Add,
            "/spec/sync"
        ));
    }

    #[test]
    fn mutation_is_idempotent_after_defaults_exist() {
        let context = context();
        let mut fleet_autoscaler = buffer_fixture();
        context.default_fleet_autoscaler(
            &mut fleet_autoscaler,
        );
        let object =
            serde_json::to_value(fleet_autoscaler).unwrap();

        assert!(matches!(
            mutate_fleet_autoscaler_object(&context, &object)
                .unwrap(),
            AdmissionResponse::Allow
        ));
    }

    #[test]
    fn validation_applies_defaults_before_fixed_interval_check() {
        let response =
            validate_fleet_autoscaler_object(
                &context(),
                &serde_json::to_value(buffer_fixture())
                    .unwrap(),
            )
            .unwrap();

        assert!(matches!(response, AdmissionResponse::Allow));
    }

    #[test]
    fn validation_preserves_buffer_policy_errors() {
        let mut fleet_autoscaler = buffer_fixture();
        fleet_autoscaler
            .spec
            .policy
            .buffer
            .as_mut()
            .unwrap()
            .buffer_size = IntOrPercent::String("100%".into());

        let response =
            validate_fleet_autoscaler_object(
                &context(),
                &serde_json::to_value(fleet_autoscaler)
                    .unwrap(),
            )
            .unwrap();

        let AdmissionResponse::Deny(reason) = response
        else {
            panic!("invalid FleetAutoscaler should be denied");
        };
        assert!(reason.contains(
            "spec.policy.buffer.bufferSize"
        ));
    }

    #[test]
    fn schedule_policy_uses_context_feature_gate() {
        let mut fleet_autoscaler = buffer_fixture();
        fleet_autoscaler.spec.policy =
            FleetAutoscalerPolicy {
                policy_type:
                    FleetAutoscalerPolicyType::Schedule,
                schedule: Some(SchedulePolicy {
                    active_period: ActivePeriod {
                        start_cron: "0 * * * *".into(),
                        duration: "30m".into(),
                        ..Default::default()
                    },
                    policy: Box::new(
                        FleetAutoscalerPolicy {
                            policy_type:
                                FleetAutoscalerPolicyType::Buffer,
                            buffer: Some(BufferPolicy {
                                buffer_size:
                                    IntOrPercent::Int(2),
                                min_replicas: 2,
                                max_replicas: 20,
                            }),
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            };

        let object =
            serde_json::to_value(&fleet_autoscaler)
                .unwrap();

        let disabled =
            validate_fleet_autoscaler_object(
                &context(),
                &object,
            )
            .unwrap();
        let AdmissionResponse::Deny(reason) = disabled
        else {
            panic!("disabled schedule feature should deny");
        };
        assert!(reason.contains(
            "feature ScheduledAutoscaler must be enabled"
        ));

        let enabled_context =
            FleetAutoscalerAdmissionContext::new(
                AutoscalerFeatures {
                    scheduled_autoscaler: true,
                    wasm_autoscaler: false,
                },
            );
        let enabled =
            validate_fleet_autoscaler_object(
                &enabled_context,
                &object,
            )
            .unwrap();
        assert!(matches!(enabled, AdmissionResponse::Allow));
    }

    #[test]
    fn malformed_mutation_falls_through_but_validation_errors() {
        let object = serde_json::json!("WRONG DATA");
        assert!(matches!(
            mutate_fleet_autoscaler_object(
                &context(),
                &object,
            )
            .unwrap(),
            AdmissionResponse::Allow
        ));

        let error =
            validate_fleet_autoscaler_object(
                &context(),
                &object,
            )
            .unwrap_err();
        assert!(error
            .to_string()
            .contains("unmarshalling FleetAutoscaler"));
    }

    #[test]
    fn admission_adapters_cover_create_and_update_only() {
        let mutation =
            FleetAutoscalerMutationAdmission::new(context());
        let validation =
            FleetAutoscalerValidationAdmission::new(context());

        for operation in [Operation::Create, Operation::Update] {
            assert!(mutation.supports_operation(&operation));
            assert!(validation.supports_operation(&operation));
        }
        assert!(!mutation.supports_operation(&Operation::Delete));
        assert!(!validation.supports_operation(&Operation::Delete));
    }

    #[test]
    fn cgp_context_wiring_exposes_default_and_validation_capabilities() {
        let context = context();
        let mut fleet_autoscaler = buffer_fixture();

        context.default_fleet_autoscaler(
            &mut fleet_autoscaler,
        );
        assert!(
            fleet_autoscaler.spec.sync.is_some()
        );
        assert!(
            context
                .validate_fleet_autoscaler(
                    &fleet_autoscaler,
                )
                .is_empty()
        );
    }
}
