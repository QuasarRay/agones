use std::sync::Arc;

use agones_core::{
    BufferPolicy, Fleet, FleetAutoscalerPolicy, FleetAutoscalerPolicyType,
    FleetAutoscalerScaleDecision, apply_buffer_autoscaler_policy,
};
use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::AdapterError;

/// FleetAutoscaler controller context.
///
/// The context starts with storage even though pure policy providers do not
/// require it. Subsequent reconciliation/runtime milestones can add providers
/// to this same context without threading storage through policy functions.
#[derive(HasField)]
pub struct FleetAutoscalerController<S> {
    pub storage: Arc<S>,
}

impl<S> FleetAutoscalerController<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage: Arc::new(storage),
        }
    }
}

#[cgp_type]
pub trait HasFleetAutoscalerStorageType {
    type FleetAutoscalerStorage: Storage + 'static;
}

#[cgp_getter]
pub trait HasFleetAutoscalerStorage: HasFleetAutoscalerStorageType {
    fn fleet_autoscaler_storage(
        &self,
    ) -> &Arc<Self::FleetAutoscalerStorage>;
}

#[cgp_component(BufferFleetAutoscalerPolicyEvaluator)]
pub trait CanApplyBufferFleetAutoscalerPolicy {
    fn apply_buffer_fleet_autoscaler_policy(
        &self,
        policy: &BufferPolicy,
        fleet: &Fleet,
    ) -> Result<FleetAutoscalerScaleDecision, AdapterError>;
}

#[cgp_new_provider]
impl<Context> BufferFleetAutoscalerPolicyEvaluator<Context>
    for UseAgonesBufferFleetAutoscalerPolicy
{
    fn apply_buffer_fleet_autoscaler_policy(
        _context: &Context,
        policy: &BufferPolicy,
        fleet: &Fleet,
    ) -> Result<FleetAutoscalerScaleDecision, AdapterError> {
        apply_buffer_autoscaler_policy(policy, fleet)
            .map_err(AdapterError::Hook)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetAutoscalerPolicyDecision {
    pub replicas: i32,
    pub limited: bool,
    pub applied_policy: FleetAutoscalerPolicyType,
}

#[cgp_component(FleetAutoscalerPolicyEvaluator)]
pub trait CanComputeFleetAutoscalerDesiredSize {
    fn compute_fleet_autoscaler_desired_size(
        &self,
        policy: &FleetAutoscalerPolicy,
        fleet: &Fleet,
    ) -> Result<FleetAutoscalerPolicyDecision, AdapterError>;
}

/// Runtime policy dispatch remains an enum match because policy type is data,
/// while each policy implementation is a separately replaceable CGP provider.
///
/// This milestone wires Buffer. Additional policy providers can be added to the
/// context without coupling their implementation to persistence/runtime code.
#[cgp_new_provider]
impl<Context> FleetAutoscalerPolicyEvaluator<Context>
    for ComputeFleetAutoscalerDesiredSizeWithComponents
where
    Context: CanApplyBufferFleetAutoscalerPolicy,
{
    fn compute_fleet_autoscaler_desired_size(
        context: &Context,
        policy: &FleetAutoscalerPolicy,
        fleet: &Fleet,
    ) -> Result<FleetAutoscalerPolicyDecision, AdapterError> {
        match &policy.policy_type {
            FleetAutoscalerPolicyType::Buffer => {
                let buffer = policy.buffer.as_ref().ok_or_else(|| {
                    AdapterError::Hook(
                        "buffer policy config params are missing".into(),
                    )
                })?;
                let decision = context
                    .apply_buffer_fleet_autoscaler_policy(
                        buffer,
                        fleet,
                    )?;
                Ok(FleetAutoscalerPolicyDecision {
                    replicas: decision.replicas,
                    limited: decision.limited,
                    applied_policy:
                        FleetAutoscalerPolicyType::Buffer,
                })
            }
            _ => Err(AdapterError::Hook(
                "wrong policy type, should be one of: Buffer, Webhook, Counter, List, Schedule, Chain"
                    .into(),
            )),
        }
    }
}

delegate_and_check_components! {
    <S: Storage + 'static>
    FleetAutoscalerController<S> {
        FleetAutoscalerStorageTypeProviderComponent: UseType<S>,
        FleetAutoscalerStorageGetterComponent:
            UseField<Symbol!("storage")>,
        BufferFleetAutoscalerPolicyEvaluatorComponent:
            UseAgonesBufferFleetAutoscalerPolicy,
        FleetAutoscalerPolicyEvaluatorComponent:
            ComputeFleetAutoscalerDesiredSizeWithComponents,
    }
}

#[cfg(test)]
mod tests {
    use agones_core::{
        BufferPolicy, FleetAutoscalerPolicy, IntOrPercent,
    };
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn fleet(allocated: i32) -> Fleet {
        let mut fleet = Fleet::default();
        fleet.status.allocated_replicas = allocated;
        fleet
    }

    #[test]
    fn cgp_buffer_provider_matches_upstream_integer_policy() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let decision = controller
            .apply_buffer_fleet_autoscaler_policy(
                &BufferPolicy {
                    buffer_size: IntOrPercent::Int(20),
                    min_replicas: 0,
                    max_replicas: 100,
                },
                &fleet(40),
            )
            .unwrap();

        assert_eq!(decision.replicas, 60);
        assert!(!decision.limited);
    }

    #[test]
    fn cgp_dispatch_uses_buffer_provider_and_preserves_policy_tag() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let policy = FleetAutoscalerPolicy {
            policy_type: FleetAutoscalerPolicyType::Buffer,
            buffer: Some(BufferPolicy {
                buffer_size: IntOrPercent::String("20%".into()),
                min_replicas: 0,
                max_replicas: 100,
            }),
            ..Default::default()
        };

        let decision = controller
            .compute_fleet_autoscaler_desired_size(
                &policy,
                &fleet(50),
            )
            .unwrap();

        assert_eq!(decision.replicas, 63);
        assert!(!decision.limited);
        assert_eq!(
            decision.applied_policy,
            FleetAutoscalerPolicyType::Buffer
        );
    }

    #[test]
    fn missing_buffer_configuration_is_reported_without_panic() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let error = controller
            .compute_fleet_autoscaler_desired_size(
                &FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Buffer,
                    ..Default::default()
                },
                &fleet(1),
            )
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("buffer policy config params are missing")
        );
    }

    #[test]
    fn unsupported_policy_preserves_upstream_dispatch_error() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        let error = controller
            .compute_fleet_autoscaler_desired_size(
                &FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Webhook,
                    ..Default::default()
                },
                &fleet(1),
            )
            .unwrap_err();

        assert!(error.to_string().contains(
            "wrong policy type, should be one of: Buffer, Webhook, Counter, List, Schedule, Chain"
        ));
    }
}
