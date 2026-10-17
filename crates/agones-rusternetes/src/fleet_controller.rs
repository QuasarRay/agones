use std::sync::Arc;

use agones_core::{
    FLEET_NAME_LABEL, Fleet, FleetDeploymentPlan, GameServerSet,
    compute_fleet_status, filter_game_server_sets_by_active,
    fleet_deployment_plan,
};
use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::{AdapterError, AgonesRepository, FleetAllocationTracker};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FleetControllerConfig {
    pub player_tracking: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetReconcileResult {
    pub fleet: Fleet,
    pub active_game_server_set: GameServerSet,
    pub deployment_plan: FleetDeploymentPlan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FleetSyncOutcome {
    NotFound,
    Deleting(Fleet),
    Reconciled(FleetReconcileResult),
}

/// Fleet controller context.
///
/// Runtime dependencies live as fields, while CGP exposes them through
/// generated getter/type components. High-level reconciliation therefore
/// depends on capabilities rather than the concrete controller struct.
#[derive(HasField)]
pub struct FleetController<S> {
    pub storage: Arc<S>,
    pub allocation_tracker: Arc<FleetAllocationTracker>,
    pub fleet_controller_config: FleetControllerConfig,
}

impl<S> FleetController<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage: Arc::new(storage),
            allocation_tracker: Arc::new(FleetAllocationTracker::default()),
            fleet_controller_config: FleetControllerConfig::default(),
        }
    }

    pub fn with_allocation_tracker(
        mut self,
        allocation_tracker: Arc<FleetAllocationTracker>,
    ) -> Self {
        self.allocation_tracker = allocation_tracker;
        self
    }

    pub fn with_config(mut self, config: FleetControllerConfig) -> Self {
        self.fleet_controller_config = config;
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

#[cgp_getter]
pub trait HasFleetAllocationTracker {
    fn fleet_allocation_tracker(&self) -> &Arc<FleetAllocationTracker>;
}

#[cgp_getter]
pub trait HasFleetControllerConfig {
    fn fleet_controller_config(&self) -> &FleetControllerConfig;
}

fn controller_owner_uid(game_server_set: &GameServerSet) -> Option<&str> {
    game_server_set
        .metadata
        .extensions
        .get("ownerReferences")
        .and_then(serde_json::Value::as_array)
        .and_then(|owners| {
            owners.iter().find(|owner| {
                owner
                    .get("controller")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
            })
        })
        .and_then(|owner| owner.get("uid"))
        .and_then(serde_json::Value::as_str)
}

/// Equivalent of ListGameServerSetsByFleetOwner.
///
/// The Fleet label is the initial selector and controller ownership by UID is
/// the authority check. This prevents an old GameServerSet from attaching to a
/// newly-created Fleet with the same name.
pub fn game_server_set_is_controlled_by_fleet(
    game_server_set: &GameServerSet,
    fleet: &Fleet,
) -> bool {
    game_server_set
        .metadata
        .labels
        .get(FLEET_NAME_LABEL)
        == Some(&fleet.metadata.name)
        && controller_owner_uid(game_server_set)
            == Some(fleet.metadata.uid.as_str())
}

/// Pluggable pure policy boundary for Fleet deployment planning.
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

/// Lists the informer-equivalent Fleet-owned GameServerSets.
#[async_trait]
#[cgp_component(FleetGameServerSetLister)]
pub trait CanListFleetGameServerSets {
    async fn list_fleet_game_server_sets(
        &self,
        fleet: &Fleet,
    ) -> Result<Vec<GameServerSet>, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetGameServerSetLister<Context>
    for ListFleetGameServerSetsWithStorage
where
    Context: HasFleetStorage + Sync,
{
    async fn list_fleet_game_server_sets(
        context: &Context,
        fleet: &Fleet,
    ) -> Result<Vec<GameServerSet>, AdapterError> {
        let listed = AgonesRepository::new(Arc::clone(context.fleet_storage()))
            .list::<GameServerSet>(fleet.metadata.namespace.as_str())
            .await?;

        Ok(listed
            .into_iter()
            .filter(|game_server_set| {
                game_server_set_is_controlled_by_fleet(
                    game_server_set,
                    fleet,
                )
            })
            .collect())
    }
}

/// Deletes inactive GameServerSets only after both live and Shutdown counts
/// have drained to zero.
#[async_trait]
#[cgp_component(EmptyFleetGameServerSetDeleter)]
pub trait CanDeleteEmptyFleetGameServerSets {
    async fn delete_empty_fleet_game_server_sets(
        &self,
        fleet: &Fleet,
        game_server_sets: &[GameServerSet],
    ) -> Result<Vec<String>, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> EmptyFleetGameServerSetDeleter<Context>
    for DeleteEmptyFleetGameServerSetsWithStorage
where
    Context: HasFleetStorage + Sync,
{
    async fn delete_empty_fleet_game_server_sets(
        context: &Context,
        _fleet: &Fleet,
        game_server_sets: &[GameServerSet],
    ) -> Result<Vec<String>, AdapterError> {
        let repository =
            AgonesRepository::new(Arc::clone(context.fleet_storage()));
        let mut deleted = Vec::new();

        for game_server_set in game_server_sets {
            if game_server_set.status.replicas == 0
                && game_server_set.status.shutdown_replicas == 0
            {
                repository
                    .delete::<GameServerSet>(
                        game_server_set.metadata.namespace.as_str(),
                        game_server_set.metadata.name.as_str(),
                    )
                    .await?;
                deleted.push(game_server_set.metadata.name.clone());
            }
        }

        Ok(deleted)
    }
}

/// Creates a missing active GameServerSet or updates the existing active set's
/// mutable Fleet-controlled fields.
#[async_trait]
#[cgp_component(FleetActiveGameServerSetUpserter)]
pub trait CanUpsertFleetActiveGameServerSet {
    async fn upsert_fleet_active_game_server_set(
        &self,
        fleet: &Fleet,
        active: GameServerSet,
        replicas: i32,
    ) -> Result<GameServerSet, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetActiveGameServerSetUpserter<Context>
    for UpsertFleetActiveGameServerSetWithStorage
where
    Context: HasFleetStorage + Sync,
{
    async fn upsert_fleet_active_game_server_set(
        context: &Context,
        fleet: &Fleet,
        mut active: GameServerSet,
        replicas: i32,
    ) -> Result<GameServerSet, AdapterError> {
        let repository =
            AgonesRepository::new(Arc::clone(context.fleet_storage()));

        if active.metadata.uid.is_empty() {
            active.spec.replicas = replicas;
            let mut created = repository.create(&active).await?;

            // Upstream performs a status-subresource initialization immediately
            // after creation. Rusternetes stores the whole CRD, but preserve the
            // write and exact initialized fields.
            created.status.ready_replicas = 0;
            created.status.replicas = 0;
            created.status.allocated_replicas = 0;
            return repository.update(&created).await;
        }

        if active.spec.replicas != replicas
            || active.spec.scheduling != fleet.spec.scheduling
        {
            active.spec.replicas = replicas;
            active.spec.scheduling = fleet.spec.scheduling.clone();
            active = repository.update(&active).await?;
        }

        if active.spec.priorities != fleet.spec.priorities {
            active.spec.priorities = fleet.spec.priorities.clone();
            active = repository.update(&active).await?;
        }

        Ok(active)
    }
}

/// Recomputes Fleet status from a fresh owned-GameServerSet snapshot and
/// consumes the allocation delta only after persistence succeeds.
#[async_trait]
#[cgp_component(FleetStatusUpdater)]
pub trait CanUpdateFleetStatus {
    async fn update_fleet_status(
        &self,
        fleet: &Fleet,
    ) -> Result<Fleet, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetStatusUpdater<Context>
    for UpdateFleetStatusWithStorage
where
    Context: HasFleetStorage
        + HasFleetAllocationTracker
        + HasFleetControllerConfig
        + CanListFleetGameServerSets
        + Sync,
{
    async fn update_fleet_status(
        context: &Context,
        fleet: &Fleet,
    ) -> Result<Fleet, AdapterError> {
        let game_server_sets =
            context.list_fleet_game_server_sets(fleet).await?;
        let repository =
            AgonesRepository::new(Arc::clone(context.fleet_storage()));

        // Upstream gets the Fleet again immediately before UpdateStatus rather
        // than updating the informer snapshot.
        let mut fresh = repository
            .get::<Fleet>(
                fleet.metadata.namespace.as_str(),
                fleet.metadata.name.as_str(),
            )
            .await?;

        let allocation_delta = context
            .fleet_allocation_tracker()
            .get(
                fleet.metadata.namespace.as_str(),
                fleet.metadata.name.as_str(),
            );
        fresh.status = compute_fleet_status(
            &fresh,
            &game_server_sets,
            allocation_delta,
            context.fleet_controller_config().player_tracking,
        );

        let updated = repository.update(&fresh).await?;

        // Any allocations that race with the status write remain in the atomic
        // counter because we subtract only the value captured above.
        context
            .fleet_allocation_tracker()
            .decrement(
                fleet.metadata.namespace.as_str(),
                fleet.metadata.name.as_str(),
                allocation_delta,
            );

        Ok(updated)
    }
}

/// One complete persistence pass for the Fleet controller.
///
/// This mirrors syncFleet ordering:
/// get Fleet -> deletion guard -> list owned sets -> choose/create active in
/// memory -> apply deployment -> delete drained inactive sets -> upsert active
/// -> recompute/persist Fleet status.
#[async_trait]
#[cgp_component(FleetReconciler)]
pub trait CanReconcileFleet {
    async fn reconcile_fleet(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<FleetSyncOutcome, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetReconciler<Context>
    for ReconcileFleetWithComponents
where
    Context: HasFleetStorage
        + CanListFleetGameServerSets
        + CanExecuteFleetDeployment
        + CanDeleteEmptyFleetGameServerSets
        + CanUpsertFleetActiveGameServerSet
        + CanUpdateFleetStatus
        + Sync,
{
    async fn reconcile_fleet(
        context: &Context,
        namespace: &str,
        name: &str,
    ) -> Result<FleetSyncOutcome, AdapterError> {
        let repository =
            AgonesRepository::new(Arc::clone(context.fleet_storage()));
        let fleet = match repository.get::<Fleet>(namespace, name).await {
            Ok(fleet) => fleet,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_),
            )) => return Ok(FleetSyncOutcome::NotFound),
            Err(error) => return Err(error),
        };

        if fleet.metadata.deletion_timestamp.is_some() {
            return Ok(FleetSyncOutcome::Deleting(fleet));
        }

        let game_server_sets =
            context.list_fleet_game_server_sets(&fleet).await?;
        let (active, rest) =
            filter_game_server_sets_by_active(&fleet, &game_server_sets);
        let active =
            active.unwrap_or_else(|| fleet.game_server_set());

        let deployment_plan = context
            .execute_fleet_deployment(&fleet, &active, &rest)
            .await?;

        context
            .delete_empty_fleet_game_server_sets(&fleet, &rest)
            .await?;

        let active_game_server_set = context
            .upsert_fleet_active_game_server_set(
                &fleet,
                active,
                deployment_plan.active_replicas,
            )
            .await?;

        let fleet = context.update_fleet_status(&fleet).await?;

        Ok(FleetSyncOutcome::Reconciled(FleetReconcileResult {
            fleet,
            active_game_server_set,
            deployment_plan,
        }))
    }
}

// The concrete Fleet controller is only wiring. Storage access, config,
// allocation tracking, policy, persistence and orchestration are separately
// replaceable providers, and the wiring is compile-time checked.
delegate_and_check_components! {
    <S: Storage + 'static>
    FleetController<S> {
        FleetStorageTypeProviderComponent: UseType<S>,
        FleetStorageGetterComponent: UseField<Symbol!("storage")>,
        FleetAllocationTrackerGetterComponent:
            UseField<Symbol!("allocation_tracker")>,
        FleetControllerConfigGetterComponent:
            UseField<Symbol!("fleet_controller_config")>,
        FleetDeploymentPlannerComponent: UseAgonesFleetDeploymentPlanner,
        FleetGameServerSetScalerComponent: ScaleFleetGameServerSetWithStorage,
        FleetDeploymentExecutorComponent: ExecuteFleetDeploymentWithStorage,
        FleetGameServerSetListerComponent: ListFleetGameServerSetsWithStorage,
        EmptyFleetGameServerSetDeleterComponent:
            DeleteEmptyFleetGameServerSetsWithStorage,
        FleetActiveGameServerSetUpserterComponent:
            UpsertFleetActiveGameServerSetWithStorage,
        FleetStatusUpdaterComponent: UpdateFleetStatusWithStorage,
        FleetReconcilerComponent: ReconcileFleetWithComponents,
    }
}

#[cfg(test)]
mod tests {
    use agones_core::{
        DeploymentStrategyType, FleetSpec, GameServerSetSpec,
        GameServerSetStatus, IntOrPercent, ObjectMeta, Priority,
        RollingUpdateDeployment, SchedulingStrategy,
    };
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn fleet(replicas: i32, ready_replicas: i32) -> Fleet {
        Fleet {
            metadata: ObjectMeta {
                name: "fleet".into(),
                namespace: "default".into(),
                uid: "fleet-uid".into(),
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
                scheduling: Some(SchedulingStrategy::Packed),
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

    async fn stored_fleet(
        storage: &MemoryStorage,
        replicas: i32,
    ) -> Fleet {
        let mut value = fleet(replicas, 0);
        value.apply_defaults();
        AgonesRepository::new(storage.clone())
            .create(&value)
            .await
            .unwrap()
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

    #[tokio::test]
    async fn owned_listing_requires_fleet_label_and_controller_uid() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let fleet = fleet(1, 0);

        let mut owned = fleet.game_server_set();
        owned.metadata.name = "owned".into();
        owned.metadata.generate_name.clear();
        repository.create(&owned).await.unwrap();

        let mut wrong_uid = fleet.game_server_set();
        wrong_uid.metadata.name = "wrong-uid".into();
        wrong_uid.metadata.generate_name.clear();
        wrong_uid
            .metadata
            .extensions
            .get_mut("ownerReferences")
            .and_then(serde_json::Value::as_array_mut)
            .unwrap()[0]["uid"] = serde_json::json!("old-fleet-uid");
        repository.create(&wrong_uid).await.unwrap();

        let mut wrong_label = fleet.game_server_set();
        wrong_label.metadata.name = "wrong-label".into();
        wrong_label.metadata.generate_name.clear();
        wrong_label.metadata.labels.insert(
            FLEET_NAME_LABEL.into(),
            "another-fleet".into(),
        );
        repository.create(&wrong_label).await.unwrap();

        let controller = FleetController::new(storage);
        let listed = controller
            .list_fleet_game_server_sets(&fleet)
            .await
            .unwrap();

        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].metadata.name, "owned");
    }

    #[tokio::test]
    async fn reconcile_creates_active_set_and_updates_status() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let fleet = stored_fleet(&storage, 3).await;
        let controller = FleetController::new(storage.clone());

        let outcome = controller
            .reconcile_fleet("default", "fleet")
            .await
            .unwrap();
        let FleetSyncOutcome::Reconciled(result) = outcome else {
            panic!("stored Fleet should reconcile");
        };

        assert_eq!(result.deployment_plan.active_replicas, 3);
        assert_eq!(result.active_game_server_set.spec.replicas, 3);
        assert_eq!(
            result.active_game_server_set.spec.scheduling,
            fleet.spec.scheduling
        );
        assert_eq!(result.fleet.status.replicas, 0);

        let sets = repository
            .list::<GameServerSet>("default")
            .await
            .unwrap();
        assert_eq!(sets.len(), 1);
        assert!(game_server_set_is_controlled_by_fleet(
            &sets[0],
            &fleet
        ));
    }

    #[tokio::test]
    async fn delete_empty_inactive_set_removes_drained_resource() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let fleet = stored_fleet(&storage, 3).await;
        let controller = FleetController::new(storage.clone());

        let mut inactive = fleet.game_server_set();
        inactive.metadata.name = "old".into();
        inactive.metadata.generate_name.clear();
        inactive.spec.replicas = 0;
        inactive.status.replicas = 0;
        inactive.status.shutdown_replicas = 0;
        let inactive = repository.create(&inactive).await.unwrap();

        let deleted = controller
            .delete_empty_fleet_game_server_sets(
                &fleet,
                &[inactive],
            )
            .await
            .unwrap();
        assert_eq!(deleted, vec!["old".to_owned()]);

        let missing = repository
            .get::<GameServerSet>("default", "old")
            .await;
        assert!(matches!(
            missing,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_)
            ))
        ));
    }

    #[tokio::test]
    async fn status_update_consumes_allocation_delta_exactly_once() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let fleet = stored_fleet(&storage, 3).await;

        let mut set = fleet.game_server_set();
        set.metadata.name = "set".into();
        set.metadata.generate_name.clear();
        set.spec.replicas = 3;
        set.status.replicas = 3;
        set.status.ready_replicas = 2;
        set.status.allocated_replicas = 1;
        repository.create(&set).await.unwrap();

        let tracker = Arc::new(FleetAllocationTracker::default());
        tracker.increment("default", "fleet", 2);
        let controller = FleetController::new(storage)
            .with_allocation_tracker(Arc::clone(&tracker));

        let first = controller
            .update_fleet_status(&fleet)
            .await
            .unwrap();
        assert_eq!(first.status.replicas, 3);
        assert_eq!(first.status.ready_replicas, 2);
        assert_eq!(first.status.allocated_replicas, 1);
        assert_eq!(first.status.allocations, 2);
        assert_eq!(tracker.get("default", "fleet"), 0);

        let second = controller
            .update_fleet_status(&first)
            .await
            .unwrap();
        assert_eq!(second.status.allocations, 2);
    }

    #[tokio::test]
    async fn reconcile_updates_active_mutable_fields() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fleet = stored_fleet(&storage, 4).await;
        fleet.spec.scheduling = Some(SchedulingStrategy::Distributed);
        fleet.spec.priorities = vec![Priority {
            priority_type: "List".into(),
            key: "score".into(),
            order: "Ascending".into(),
        }];
        fleet = repository.update(&fleet).await.unwrap();

        let mut active = fleet.game_server_set();
        active.metadata.name = "active".into();
        active.metadata.generate_name.clear();
        active.spec.replicas = 1;
        active.spec.scheduling = Some(SchedulingStrategy::Packed);
        active.spec.priorities.clear();
        repository.create(&active).await.unwrap();

        let controller = FleetController::new(storage);
        let outcome = controller
            .reconcile_fleet("default", "fleet")
            .await
            .unwrap();
        let FleetSyncOutcome::Reconciled(result) = outcome else {
            panic!("stored Fleet should reconcile");
        };

        assert_eq!(result.active_game_server_set.spec.replicas, 4);
        assert_eq!(
            result.active_game_server_set.spec.scheduling,
            Some(SchedulingStrategy::Distributed)
        );
        assert_eq!(
            result.active_game_server_set.spec.priorities,
            fleet.spec.priorities
        );
    }

    #[tokio::test]
    async fn deleting_and_missing_fleets_are_successful_noops() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fleet = fleet(1, 0);
        fleet.metadata.deletion_timestamp =
            Some("2026-10-18T00:00:00Z".into());
        repository.create(&fleet).await.unwrap();

        let controller = FleetController::new(storage.clone());
        assert!(matches!(
            controller
                .reconcile_fleet("default", "fleet")
                .await
                .unwrap(),
            FleetSyncOutcome::Deleting(_)
        ));
        assert!(
            repository
                .list::<GameServerSet>("default")
                .await
                .unwrap()
                .is_empty()
        );

        assert_eq!(
            controller
                .reconcile_fleet("default", "missing")
                .await
                .unwrap(),
            FleetSyncOutcome::NotFound
        );
    }
}
