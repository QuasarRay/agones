use std::sync::Arc;

use agones_core::{
    Fleet, FleetDeploymentPlan, GameServerSet, fleet_deployment_plan,
};
use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::{AdapterError, AgonesRepository, FleetAllocationTracker};

/// Fleet controller context.
///
/// Storage is exposed to components as an abstract CGP type and getter rather
/// than threaded through every capability. This keeps controller policies
/// independently replaceable while retaining static dispatch.
#[derive(HasField)]
pub struct FleetController<S> {
    pub storage: Arc<S>,
    pub allocation_tracker: Arc<FleetAllocationTracker>,
}

impl<S> FleetController<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage: Arc::new(storage),
            allocation_tracker: Arc::new(FleetAllocationTracker::default()),
        }
    }

    pub fn with_allocation_tracker(
        mut self,
        allocation_tracker: Arc<FleetAllocationTracker>,
    ) -> Self {
        self.allocation_tracker = allocation_tracker;
        self
    }
}

#[cgp_type]
pub trait HasFleetStorageType {
    type FleetStorage: Storage + 'static;
}

#[cgp_getter]
pub trait HasFleetStorage: HasFleetStorageType {
    fn fleet_storage(&self) -> &Arc<Self::FleetStorage>;
}

/// Pluggable pure policy boundary for Fleet deployment planning.
///
/// The default provider delegates to agones-core. Keeping this as a component
/// lets tests or future policy experiments replace rollout planning without
/// changing persistence or controller orchestration.
#[cgp_component(FleetDeploymentPlanner)]
pub trait CanPlanFleetDeployment {
    fn plan_fleet_deployment(
        &self,
        fleet: &Fleet,
        active: &GameServerSet,
        rest: &[GameServerSet],
    ) -> Result<FleetDeploymentPlan, AdapterError>;
}

#[cgp_new_provider]
impl<Context> FleetDeploymentPlanner<Context>
    for UseAgonesFleetDeploymentPlanner
{
    fn plan_fleet_deployment(
        _context: &Context,
        fleet: &Fleet,
        active: &GameServerSet,
        rest: &[GameServerSet],
    ) -> Result<FleetDeploymentPlan, AdapterError> {
        fleet_deployment_plan(fleet, active, rest).map_err(AdapterError::Hook)
    }
}

/// Persistence component for one Fleet-owned GameServerSet replica change.
#[async_trait]
#[cgp_component(FleetGameServerSetScaler)]
pub trait CanScaleFleetGameServerSet {
    async fn scale_fleet_game_server_set(
        &self,
        game_server_set: &GameServerSet,
        replicas: i32,
    ) -> Result<GameServerSet, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetGameServerSetScaler<Context>
    for ScaleFleetGameServerSetWithStorage
where
    Context: HasFleetStorage + Sync,
{
    async fn scale_fleet_game_server_set(
        context: &Context,
        game_server_set: &GameServerSet,
        replicas: i32,
    ) -> Result<GameServerSet, AdapterError> {
        if game_server_set.spec.replicas == replicas {
            return Ok(game_server_set.clone());
        }

        let mut updated = game_server_set.clone();
        updated.spec.replicas = replicas;
        AgonesRepository::new(Arc::clone(context.fleet_storage()))
            .update(&updated)
            .await
    }
}

/// High-level deployment component that composes the pluggable planner with
/// the pluggable persistence capability.
#[async_trait]
#[cgp_component(FleetDeploymentExecutor)]
pub trait CanExecuteFleetDeployment {
    async fn execute_fleet_deployment(
        &self,
        fleet: &Fleet,
        active: &GameServerSet,
        rest: &[GameServerSet],
    ) -> Result<FleetDeploymentPlan, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetDeploymentExecutor<Context>
    for ExecuteFleetDeploymentWithStorage
where
    Context:
        CanPlanFleetDeployment + CanScaleFleetGameServerSet + Sync,
{
    async fn execute_fleet_deployment(
        context: &Context,
        fleet: &Fleet,
        active: &GameServerSet,
        rest: &[GameServerSet],
    ) -> Result<FleetDeploymentPlan, AdapterError> {
        let plan = context.plan_fleet_deployment(fleet, active, rest)?;

        for scale in &plan.inactive_scales {
            let game_server_set = rest
                .iter()
                .find(|game_server_set| {
                    game_server_set.metadata.namespace == scale.namespace
                        && game_server_set.metadata.name == scale.name
                })
                .ok_or_else(|| {
                    AdapterError::Hook(format!(
                        "Fleet deployment plan references missing GameServerSet {}/{}",
                        scale.namespace, scale.name
                    ))
                })?;

            context
                .scale_fleet_game_server_set(
                    game_server_set,
                    scale.to_replicas,
                )
                .await?;
        }

        Ok(plan)
    }
}

// The concrete Fleet controller is only wiring. Policy, persistence, storage
// access and their compile-time dependency checks are generated by CGP.
delegate_and_check_components! {
    <S: Storage + 'static>
    FleetController<S> {
        FleetStorageTypeProviderComponent: UseType<S>,
        FleetStorageGetterComponent: UseField<Symbol!("storage")>,
        FleetDeploymentPlannerComponent: UseAgonesFleetDeploymentPlanner,
        FleetGameServerSetScalerComponent: ScaleFleetGameServerSetWithStorage,
        FleetDeploymentExecutorComponent: ExecuteFleetDeploymentWithStorage,
    }
}

#[cfg(test)]
mod tests {
    use agones_core::{
        DeploymentStrategyType, FleetSpec, GameServerSetSpec,
        GameServerSetStatus, IntOrPercent, ObjectMeta,
        RollingUpdateDeployment,
    };
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn fleet(replicas: i32, ready_replicas: i32) -> Fleet {
        Fleet {
            metadata: ObjectMeta {
                name: "fleet".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: FleetSpec {
                replicas,
                strategy: agones_core::DeploymentStrategy {
                    strategy_type: Some(
                        DeploymentStrategyType::RollingUpdate,
                    ),
                    rolling_update: Some(RollingUpdateDeployment {
                        max_unavailable: Some(IntOrPercent::String(
                            "30%".into(),
                        )),
                        max_surge: Some(IntOrPercent::String(
                            "25%".into(),
                        )),
                    }),
                },
                ..Default::default()
            },
            status: agones_core::FleetStatus {
                ready_replicas,
                ..Default::default()
            },
        }
    }

    fn game_server_set(
        name: &str,
        spec_replicas: i32,
        status_replicas: i32,
        ready_replicas: i32,
        allocated_replicas: i32,
    ) -> GameServerSet {
        GameServerSet {
            metadata: ObjectMeta {
                name: name.into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                replicas: spec_replicas,
                ..Default::default()
            },
            status: GameServerSetStatus {
                replicas: status_replicas,
                ready_replicas,
                allocated_replicas,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn cgp_planner_uses_agones_core_policy() {
        let controller = FleetController::new(MemoryStorage::new());
        let fleet = fleet(100, 100);
        let active = game_server_set("active", 0, 0, 0, 0);
        let inactive =
            game_server_set("inactive", 100, 100, 100, 0);

        let plan = controller
            .plan_fleet_deployment(&fleet, &active, &[inactive])
            .unwrap();

        assert_eq!(plan.active_replicas, 25);
        assert_eq!(plan.inactive_scales.len(), 1);
        assert_eq!(plan.inactive_scales[0].to_replicas, 70);
    }

    #[tokio::test]
    async fn cgp_executor_persists_inactive_scale_plan() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let controller = FleetController::new(storage.clone());
        let fleet = fleet(100, 100);
        let active = game_server_set("active", 0, 0, 0, 0);
        let inactive = repository
            .create(&game_server_set(
                "inactive",
                100,
                100,
                100,
                0,
            ))
            .await
            .unwrap();

        let plan = controller
            .execute_fleet_deployment(
                &fleet,
                &active,
                &[inactive.clone()],
            )
            .await
            .unwrap();

        assert_eq!(plan.active_replicas, 25);
        assert_eq!(plan.inactive_scales[0].to_replicas, 70);

        let stored = repository
            .get::<GameServerSet>("default", "inactive")
            .await
            .unwrap();
        assert_eq!(stored.spec.replicas, 70);
    }

    #[tokio::test]
    async fn cgp_executor_recreate_scales_every_inactive_set_to_zero() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let controller = FleetController::new(storage);
        let mut fleet = fleet(10, 10);
        fleet.spec.strategy.strategy_type =
            Some(DeploymentStrategyType::Recreate);

        let active = game_server_set("active", 0, 0, 0, 0);
        let inactive = repository
            .create(&game_server_set("inactive", 10, 10, 9, 1))
            .await
            .unwrap();

        let plan = controller
            .execute_fleet_deployment(
                &fleet,
                &active,
                &[inactive],
            )
            .await
            .unwrap();

        assert_eq!(plan.active_replicas, 9);

        let stored = repository
            .get::<GameServerSet>("default", "inactive")
            .await
            .unwrap();
        assert_eq!(stored.spec.replicas, 0);
    }
}
