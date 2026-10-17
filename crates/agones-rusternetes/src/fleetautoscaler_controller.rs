use std::sync::Arc;

use agones_core::{
    BufferPolicy, Fleet, FleetAutoscaler, FleetAutoscalerPolicy,
    FleetAutoscalerPolicyType,
    FleetAutoscalerScaleDecision, apply_buffer_autoscaler_policy,
};
use cgp::prelude::*;
use chrono::Utc;
use rusternetes_storage::Storage;

use crate::{AdapterError, AgonesRepository};

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


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetAutoscalerReconcileResult {
    pub fleet_autoscaler: FleetAutoscaler,
    pub fleet: Fleet,
    pub decision: FleetAutoscalerPolicyDecision,
    pub scaled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FleetAutoscalerSyncOutcome {
    NotFound,
    FleetNotFound(FleetAutoscaler),
    FleetDeleting {
        fleet_autoscaler: FleetAutoscaler,
        fleet: Fleet,
    },
    Reconciled(FleetAutoscalerReconcileResult),
}

#[async_trait]
#[cgp_component(FleetAutoscalerFleetScaler)]
pub trait CanScaleFleetForAutoscaler {
    async fn scale_fleet_for_autoscaler(
        &self,
        fleet: &Fleet,
        replicas: i32,
    ) -> Result<Fleet, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerFleetScaler<Context>
    for ScaleFleetForAutoscalerWithStorage
where
    Context: HasFleetAutoscalerStorage + Sync,
{
    async fn scale_fleet_for_autoscaler(
        context: &Context,
        fleet: &Fleet,
        replicas: i32,
    ) -> Result<Fleet, AdapterError> {
        if replicas == fleet.spec.replicas {
            return Ok(fleet.clone());
        }

        let mut updated = fleet.clone();
        updated.spec.replicas = replicas;
        AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ))
        .update(&updated)
        .await
        .map_err(|error| {
            AdapterError::Hook(format!(
                "error updating replicas for fleet {}: {error}",
                fleet.metadata.name
            ))
        })
    }
}

#[async_trait]
#[cgp_component(FleetAutoscalerStatusUpdater)]
pub trait CanUpdateFleetAutoscalerStatus {
    async fn update_fleet_autoscaler_status(
        &self,
        fleet_autoscaler: &FleetAutoscaler,
        current_replicas: i32,
        desired_replicas: i32,
        scaled: bool,
        scaling_limited: bool,
        applied_policy: FleetAutoscalerPolicyType,
    ) -> Result<FleetAutoscaler, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerStatusUpdater<Context>
    for UpdateFleetAutoscalerStatusWithStorage
where
    Context: HasFleetAutoscalerStorage + Sync,
{
    async fn update_fleet_autoscaler_status(
        context: &Context,
        fleet_autoscaler: &FleetAutoscaler,
        current_replicas: i32,
        desired_replicas: i32,
        scaled: bool,
        scaling_limited: bool,
        applied_policy: FleetAutoscalerPolicyType,
    ) -> Result<FleetAutoscaler, AdapterError> {
        let mut updated = fleet_autoscaler.clone();
        updated.status.able_to_scale = true;
        updated.status.scaling_limited = scaling_limited;
        updated.status.current_replicas = current_replicas;
        updated.status.desired_replicas = desired_replicas;
        updated.status.last_applied_policy = applied_policy;
        if scaled {
            updated.status.last_scale_time =
                Some(Utc::now().to_rfc3339());
        }

        if updated.status == fleet_autoscaler.status {
            return Ok(updated);
        }

        AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ))
        .update(&updated)
        .await
        .map_err(|error| {
            AdapterError::Hook(format!(
                "error updating status for fleetautoscaler {}: {error}",
                fleet_autoscaler.metadata.name
            ))
        })
    }
}

#[async_trait]
#[cgp_component(FleetAutoscalerUnableStatusUpdater)]
pub trait CanMarkFleetAutoscalerUnableToScale {
    async fn mark_fleet_autoscaler_unable_to_scale(
        &self,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Result<FleetAutoscaler, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerUnableStatusUpdater<Context>
    for MarkFleetAutoscalerUnableToScaleWithStorage
where
    Context: HasFleetAutoscalerStorage + Sync,
{
    async fn mark_fleet_autoscaler_unable_to_scale(
        context: &Context,
        fleet_autoscaler: &FleetAutoscaler,
    ) -> Result<FleetAutoscaler, AdapterError> {
        let mut updated = fleet_autoscaler.clone();
        updated.status.able_to_scale = false;
        updated.status.scaling_limited = false;
        updated.status.current_replicas = 0;
        updated.status.desired_replicas = 0;
        updated.status.last_applied_policy =
            FleetAutoscalerPolicyType::Unknown(String::new());

        if updated.status == fleet_autoscaler.status {
            return Ok(updated);
        }

        AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ))
        .update(&updated)
        .await
        .map_err(|error| {
            AdapterError::Hook(format!(
                "error updating status for fleetautoscaler {}: {error}",
                fleet_autoscaler.metadata.name
            ))
        })
    }
}

#[async_trait]
#[cgp_component(FleetAutoscalerReconciler)]
pub trait CanReconcileFleetAutoscaler {
    async fn reconcile_fleet_autoscaler(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<FleetAutoscalerSyncOutcome, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAutoscalerReconciler<Context>
    for ReconcileFleetAutoscalerWithComponents
where
    Context: HasFleetAutoscalerStorage
        + CanComputeFleetAutoscalerDesiredSize
        + CanScaleFleetForAutoscaler
        + CanUpdateFleetAutoscalerStatus
        + CanMarkFleetAutoscalerUnableToScale
        + Sync,
{
    async fn reconcile_fleet_autoscaler(
        context: &Context,
        namespace: &str,
        name: &str,
    ) -> Result<FleetAutoscalerSyncOutcome, AdapterError> {
        let repository = AgonesRepository::new(Arc::clone(
            context.fleet_autoscaler_storage(),
        ));

        let fleet_autoscaler = match repository
            .get::<FleetAutoscaler>(namespace, name)
            .await
        {
            Ok(fleet_autoscaler) => fleet_autoscaler,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_),
            )) => return Ok(FleetAutoscalerSyncOutcome::NotFound),
            Err(error) => return Err(error),
        };

        let fleet = match repository
            .get::<Fleet>(
                namespace,
                fleet_autoscaler.spec.fleet_name.as_str(),
            )
            .await
        {
            Ok(fleet) => fleet,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_),
            )) => {
                let fleet_autoscaler = context
                    .mark_fleet_autoscaler_unable_to_scale(
                        &fleet_autoscaler,
                    )
                    .await?;
                return Ok(
                    FleetAutoscalerSyncOutcome::FleetNotFound(
                        fleet_autoscaler,
                    ),
                );
            }
            Err(error) => return Err(error),
        };

        if fleet.metadata.deletion_timestamp.is_some() {
            return Ok(
                FleetAutoscalerSyncOutcome::FleetDeleting {
                    fleet_autoscaler,
                    fleet,
                },
            );
        }

        let decision = match context
            .compute_fleet_autoscaler_desired_size(
                &fleet_autoscaler.spec.policy,
                &fleet,
            )
        {
            Ok(decision) => decision,
            Err(error) => {
                context
                    .mark_fleet_autoscaler_unable_to_scale(
                        &fleet_autoscaler,
                    )
                    .await?;
                return Err(AdapterError::Hook(format!(
                    "error calculating autoscaling fleet: {}: {error}",
                    fleet.metadata.name
                )));
            }
        };

        let current_replicas = fleet.status.replicas;
        let scaled = decision.replicas != fleet.spec.replicas;
        let fleet = context
            .scale_fleet_for_autoscaler(
                &fleet,
                decision.replicas,
            )
            .await?;

        let fleet_autoscaler = context
            .update_fleet_autoscaler_status(
                &fleet_autoscaler,
                current_replicas,
                decision.replicas,
                scaled,
                decision.limited,
                decision.applied_policy.clone(),
            )
            .await?;

        Ok(FleetAutoscalerSyncOutcome::Reconciled(
            FleetAutoscalerReconcileResult {
                fleet_autoscaler,
                fleet,
                decision,
                scaled,
            },
        ))
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


delegate_and_check_components! {
    <S: Storage + 'static>
    FleetAutoscalerController<S> {
        FleetAutoscalerFleetScalerComponent:
            ScaleFleetForAutoscalerWithStorage,
        FleetAutoscalerStatusUpdaterComponent:
            UpdateFleetAutoscalerStatusWithStorage,
        FleetAutoscalerUnableStatusUpdaterComponent:
            MarkFleetAutoscalerUnableToScaleWithStorage,
        FleetAutoscalerReconcilerComponent:
            ReconcileFleetAutoscalerWithComponents,
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


#[cfg(test)]
mod reconciliation_tests {
    use agones_core::{
        BufferPolicy, FleetAutoscalerSpec, FleetSpec, IntOrPercent,
        ObjectMeta,
    };
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn autoscaler(buffer: i32) -> FleetAutoscaler {
        let mut value = FleetAutoscaler {
            metadata: ObjectMeta {
                name: "fas".into(),
                namespace: "default".into(),
                uid: "fas-uid".into(),
                ..Default::default()
            },
            spec: FleetAutoscalerSpec {
                fleet_name: "fleet".into(),
                policy: FleetAutoscalerPolicy {
                    policy_type:
                        FleetAutoscalerPolicyType::Buffer,
                    buffer: Some(BufferPolicy {
                        buffer_size: IntOrPercent::Int(buffer),
                        min_replicas: 0,
                        max_replicas: 100,
                    }),
                    ..Default::default()
                },
                sync: None,
            },
            ..Default::default()
        };
        value.apply_defaults();
        value
    }

    fn fleet(
        spec_replicas: i32,
        status_replicas: i32,
        allocated_replicas: i32,
    ) -> Fleet {
        Fleet {
            metadata: ObjectMeta {
                name: "fleet".into(),
                namespace: "default".into(),
                uid: "fleet-uid".into(),
                ..Default::default()
            },
            spec: FleetSpec {
                replicas: spec_replicas,
                ..Default::default()
            },
            status: agones_core::FleetStatus {
                replicas: status_replicas,
                allocated_replicas,
                ..Default::default()
            },
        }
    }

    async fn store_pair(
        storage: &MemoryStorage,
        fleet_autoscaler: &FleetAutoscaler,
        fleet: &Fleet,
    ) {
        let repository = AgonesRepository::new(storage.clone());
        repository.create(fleet_autoscaler).await.unwrap();
        repository.create(fleet).await.unwrap();
    }

    #[tokio::test]
    async fn reconcile_scales_up_and_persists_status() {
        let storage = MemoryStorage::new();
        store_pair(
            &storage,
            &autoscaler(7),
            &fleet(5, 5, 5),
        )
        .await;

        let controller =
            FleetAutoscalerController::new(storage.clone());
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::Reconciled(result) =
            outcome
        else {
            panic!("stored autoscaler should reconcile");
        };

        assert_eq!(result.decision.replicas, 12);
        assert!(result.scaled);
        assert_eq!(result.fleet.spec.replicas, 12);
        assert!(result.fleet_autoscaler.status.able_to_scale);
        assert!(!result.fleet_autoscaler.status.scaling_limited);
        assert_eq!(
            result.fleet_autoscaler.status.current_replicas,
            5
        );
        assert_eq!(
            result.fleet_autoscaler.status.desired_replicas,
            12
        );
        assert_eq!(
            result.fleet_autoscaler.status.last_applied_policy,
            FleetAutoscalerPolicyType::Buffer
        );
        assert!(
            result
                .fleet_autoscaler
                .status
                .last_scale_time
                .is_some()
        );

        let repository = AgonesRepository::new(storage);
        let stored: Fleet =
            repository.get("default", "fleet").await.unwrap();
        assert_eq!(stored.spec.replicas, 12);
    }

    #[tokio::test]
    async fn reconcile_scales_down_with_buffer_policy() {
        let storage = MemoryStorage::new();
        store_pair(
            &storage,
            &autoscaler(8),
            &fleet(20, 20, 5),
        )
        .await;

        let controller =
            FleetAutoscalerController::new(storage);
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::Reconciled(result) =
            outcome
        else {
            panic!("stored autoscaler should reconcile");
        };

        assert_eq!(result.decision.replicas, 13);
        assert_eq!(result.fleet.spec.replicas, 13);
        assert_eq!(
            result.fleet_autoscaler.status.current_replicas,
            20
        );
        assert_eq!(
            result.fleet_autoscaler.status.desired_replicas,
            13
        );
    }

    #[tokio::test]
    async fn missing_fleet_marks_autoscaler_unable_to_scale() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut value = autoscaler(5);
        value.status.able_to_scale = true;
        value.status.current_replicas = 5;
        value.status.desired_replicas = 10;
        value.status.last_applied_policy =
            FleetAutoscalerPolicyType::Buffer;
        repository.create(&value).await.unwrap();

        let controller =
            FleetAutoscalerController::new(storage.clone());
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::FleetNotFound(updated) =
            outcome
        else {
            panic!("missing Fleet should be a successful sync");
        };

        assert!(!updated.status.able_to_scale);
        assert!(!updated.status.scaling_limited);
        assert_eq!(updated.status.current_replicas, 0);
        assert_eq!(updated.status.desired_replicas, 0);
        assert_eq!(
            updated.status.last_applied_policy.as_str(),
            ""
        );
    }

    #[tokio::test]
    async fn deleting_fleet_is_a_noop() {
        let storage = MemoryStorage::new();
        let fleet_autoscaler = autoscaler(7);
        let mut deleting = fleet(5, 5, 5);
        deleting.metadata.deletion_timestamp =
            Some("2026-10-18T00:00:00Z".into());
        store_pair(&storage, &fleet_autoscaler, &deleting).await;

        let controller =
            FleetAutoscalerController::new(storage.clone());
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        assert!(matches!(
            outcome,
            FleetAutoscalerSyncOutcome::FleetDeleting { .. }
        ));

        let repository = AgonesRepository::new(storage);
        let stored_fleet: Fleet =
            repository.get("default", "fleet").await.unwrap();
        let stored_fas: FleetAutoscaler =
            repository.get("default", "fas").await.unwrap();
        assert_eq!(stored_fleet.spec.replicas, 5);
        assert_eq!(
            stored_fas.status,
            fleet_autoscaler.status
        );
    }

    #[tokio::test]
    async fn wrong_policy_marks_unable_and_returns_calculation_error() {
        let storage = MemoryStorage::new();
        let mut value = autoscaler(5);
        value.spec.policy = FleetAutoscalerPolicy {
            policy_type:
                FleetAutoscalerPolicyType::Unknown(
                    "WRONG TYPE".into(),
                ),
            ..Default::default()
        };
        store_pair(&storage, &value, &fleet(5, 5, 1)).await;

        let controller =
            FleetAutoscalerController::new(storage.clone());
        let error = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap_err();
        assert!(error.to_string().contains(
            "error calculating autoscaling fleet: fleet"
        ));

        let stored: FleetAutoscaler =
            AgonesRepository::new(storage)
                .get("default", "fas")
                .await
                .unwrap();
        assert!(!stored.status.able_to_scale);
        assert_eq!(stored.status.current_replicas, 0);
        assert_eq!(stored.status.desired_replicas, 0);
    }

    #[tokio::test]
    async fn missing_autoscaler_is_a_successful_noop() {
        let controller =
            FleetAutoscalerController::new(MemoryStorage::new());
        assert_eq!(
            controller
                .reconcile_fleet_autoscaler(
                    "default",
                    "missing",
                )
                .await
                .unwrap(),
            FleetAutoscalerSyncOutcome::NotFound
        );
    }

    #[tokio::test]
    async fn no_spec_scale_preserves_last_scale_time() {
        let storage = MemoryStorage::new();
        let mut fleet_autoscaler = autoscaler(5);
        fleet_autoscaler.status.able_to_scale = true;
        fleet_autoscaler.status.current_replicas = 10;
        fleet_autoscaler.status.desired_replicas = 10;
        fleet_autoscaler.status.last_applied_policy =
            FleetAutoscalerPolicyType::Buffer;
        fleet_autoscaler.status.last_scale_time =
            Some("2026-10-17T00:00:00Z".into());

        let target = fleet(10, 10, 5);
        store_pair(&storage, &fleet_autoscaler, &target).await;

        let controller =
            FleetAutoscalerController::new(storage);
        let outcome = controller
            .reconcile_fleet_autoscaler("default", "fas")
            .await
            .unwrap();
        let FleetAutoscalerSyncOutcome::Reconciled(result) =
            outcome
        else {
            panic!("stored autoscaler should reconcile");
        };

        assert!(!result.scaled);
        assert_eq!(
            result
                .fleet_autoscaler
                .status
                .last_scale_time
                .as_deref(),
            Some("2026-10-17T00:00:00Z")
        );
    }
}
