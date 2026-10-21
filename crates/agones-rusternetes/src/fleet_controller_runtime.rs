use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use agones_core::{
    FLEET_NAME_LABEL, Fleet, GameServer, GameServerSet, GameServerState,
};
use agones_macros::agones_invariant;
use cgp::prelude::*;
use futures::stream::{BoxStream, FuturesUnordered};
use futures::StreamExt;
use rusternetes_storage::Storage;

use crate::controller_work_queue::{
    ControllerWorkItem, ControllerWorkQueue, FastSlowRetryPolicy,
};
use crate::{
    AdapterError, AgonesRepository, AgonesWatchEvent, CanReconcileFleet,
    FleetController, FleetSyncOutcome, HasFleetAllocationTracker,
    HasFleetStorage,
};

pub const FLEET_FAST_RETRIES: u32 = 5;
pub const FLEET_FAST_RETRY_DELAY_MS: u64 = 200;
pub const FLEET_SLOW_RETRY_DELAY_MS: u64 = 3_000;

pub const fn fleet_retry_delay_ms(retries: u32) -> u64 {
    if retries < FLEET_FAST_RETRIES {
        FLEET_FAST_RETRY_DELAY_MS
    } else {
        FLEET_SLOW_RETRY_DELAY_MS
    }
}

#[agones_invariant]
fn fleet_retry_policy_matches_upstream(retries: u8) -> bool {
    let retries = retries as u32;
    fleet_retry_delay_ms(retries)
        == if retries < FLEET_FAST_RETRIES {
            FLEET_FAST_RETRY_DELAY_MS
        } else {
            FLEET_SLOW_RETRY_DELAY_MS
        }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetWorkItem {
    pub namespace: String,
    pub name: String,
}

impl FleetWorkItem {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    pub fn key(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }
}

impl ControllerWorkItem for FleetWorkItem {
    fn work_key(&self) -> String {
        self.key()
    }
}

type FleetWorkQueue =
    ControllerWorkQueue<FleetWorkItem, FastSlowRetryPolicy>;

fn fleet_retry_policy() -> FastSlowRetryPolicy {
    FastSlowRetryPolicy {
        fast_retries: FLEET_FAST_RETRIES,
        fast_delay: Duration::from_millis(FLEET_FAST_RETRY_DELAY_MS),
        slow_delay: Duration::from_millis(FLEET_SLOW_RETRY_DELAY_MS),
    }
}

#[derive(Clone, Debug)]
pub enum FleetControllerWatchEvent {
    Fleet(AgonesWatchEvent<Fleet>),
    GameServerSet(AgonesWatchEvent<GameServerSet>),
    GameServer(AgonesWatchEvent<GameServer>),
}

pub type FleetControllerWatchStream =
    BoxStream<'static, Result<FleetControllerWatchEvent, AdapterError>>;

/// Minimal informer-equivalent observation state needed by the Fleet
/// controller's GameServer UpdateFunc.
///
/// Rusternetes watch events contain the new value only. Recording the previous
/// GameServer state recovers the upstream old/new transition predicate without
/// counting pre-existing Allocated GameServers at startup.
#[derive(Clone, Debug, Default)]
pub struct FleetControllerWatchState {
    game_server_states: HashMap<String, Option<GameServerState>>,
}

impl FleetControllerWatchState {
    fn key(game_server: &GameServer) -> String {
        format!(
            "{}/{}",
            game_server.metadata.namespace,
            game_server.metadata.name
        )
    }

    fn replace_game_server_snapshot(
        &mut self,
        game_servers: &[GameServer],
    ) {
        self.game_server_states.clear();
        for game_server in game_servers {
            self.game_server_states.insert(
                Self::key(game_server),
                game_server.status.state.clone(),
            );
        }
    }

    fn observe_added(&mut self, game_server: &GameServer) {
        self.game_server_states.insert(
            Self::key(game_server),
            game_server.status.state.clone(),
        );
    }

    fn observe_modified(
        &mut self,
        game_server: &GameServer,
    ) -> Option<Option<GameServerState>> {
        self.game_server_states.insert(
            Self::key(game_server),
            game_server.status.state.clone(),
        )
    }

    fn observe_deleted(&mut self, game_server: &GameServer) {
        self.game_server_states.remove(Self::key(game_server).as_str());
    }
}

fn controller_owner_name(game_server_set: &GameServerSet) -> Option<&str> {
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
        .and_then(|owner| owner.get("name"))
        .and_then(serde_json::Value::as_str)
}

async fn owner_fleet_work_item<S>(
    storage: Arc<S>,
    game_server_set: &GameServerSet,
) -> Result<Option<FleetWorkItem>, AdapterError>
where
    S: Storage + 'static,
{
    let Some(owner_name) = controller_owner_name(game_server_set) else {
        return Ok(None);
    };

    let namespace = game_server_set.metadata.namespace.as_str();
    match AgonesRepository::new(storage)
        .get::<Fleet>(namespace, owner_name)
        .await
    {
        Ok(_) => Ok(Some(FleetWorkItem::new(namespace, owner_name))),
        Err(AdapterError::Rusternetes(
            rusternetes_common::Error::NotFound(_),
        )) => Ok(None),
        Err(error) => Err(error),
    }
}

pub async fn watch_fleet_controller_events<S>(
    storage: Arc<S>,
) -> Result<FleetControllerWatchStream, AdapterError>
where
    S: Storage + 'static,
{
    let repository = AgonesRepository::new(storage);

    let fleets = repository
        .watch_all::<Fleet>()
        .await?
        .map(|event| event.map(FleetControllerWatchEvent::Fleet));
    let game_server_sets = repository
        .watch_all::<GameServerSet>()
        .await?
        .map(|event| {
            event.map(FleetControllerWatchEvent::GameServerSet)
        });
    let game_servers = repository
        .watch_all::<GameServer>()
        .await?
        .map(|event| event.map(FleetControllerWatchEvent::GameServer));

    Ok(Box::pin(futures::stream::select(
        futures::stream::select(fleets, game_server_sets),
        game_servers,
    )))
}

#[async_trait]
#[cgp_component(FleetControllerWatchRebuilder)]
pub trait CanRebuildFleetControllerWatchState {
    async fn rebuild_fleet_controller_watch_state(
        &self,
        watch_state: &Arc<RwLock<FleetControllerWatchState>>,
    ) -> Result<Vec<FleetWorkItem>, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetControllerWatchRebuilder<Context>
    for RebuildFleetControllerWatchStateWithStorage
where
    Context: HasFleetStorage + Sync,
{
    async fn rebuild_fleet_controller_watch_state(
        context: &Context,
        watch_state: &Arc<RwLock<FleetControllerWatchState>>,
    ) -> Result<Vec<FleetWorkItem>, AdapterError> {
        let repository =
            AgonesRepository::new(Arc::clone(context.fleet_storage()));
        let fleets = repository.list_all::<Fleet>().await?;
        let game_servers = repository.list_all::<GameServer>().await?;

        watch_state
            .write()
            .map_err(|_| {
                AdapterError::Hook(
                    "Fleet watch-state lock poisoned".into(),
                )
            })?
            .replace_game_server_snapshot(&game_servers);

        Ok(fleets
            .into_iter()
            .map(|fleet| {
                FleetWorkItem::new(
                    fleet.metadata.namespace,
                    fleet.metadata.name,
                )
            })
            .collect())
    }
}

#[async_trait]
#[cgp_component(FleetControllerEventRouter)]
pub trait CanRouteFleetControllerEvent {
    async fn route_fleet_controller_event(
        &self,
        watch_state: &Arc<RwLock<FleetControllerWatchState>>,
        event: FleetControllerWatchEvent,
    ) -> Result<Option<FleetWorkItem>, AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetControllerEventRouter<Context>
    for RouteFleetControllerEventWithStorage
where
    Context: HasFleetStorage + HasFleetAllocationTracker + Sync,
{
    async fn route_fleet_controller_event(
        context: &Context,
        watch_state: &Arc<RwLock<FleetControllerWatchState>>,
        event: FleetControllerWatchEvent,
    ) -> Result<Option<FleetWorkItem>, AdapterError> {
        match event {
            FleetControllerWatchEvent::Fleet(event) => match event {
                AgonesWatchEvent::Added { resource, .. }
                | AgonesWatchEvent::Modified { resource, .. } => {
                    Ok(Some(FleetWorkItem::new(
                        resource.metadata.namespace,
                        resource.metadata.name,
                    )))
                }
                AgonesWatchEvent::Deleted { resource, .. } => {
                    context.fleet_allocation_tracker().remove(
                        resource.metadata.namespace.as_str(),
                        resource.metadata.name.as_str(),
                    );
                    Ok(None)
                }
            },
            FleetControllerWatchEvent::GameServerSet(event) => match event {
                AgonesWatchEvent::Added { resource, .. } => {
                    owner_fleet_work_item(
                        Arc::clone(context.fleet_storage()),
                        &resource,
                    )
                    .await
                }
                AgonesWatchEvent::Modified { resource, .. } => {
                    if resource.metadata.deletion_timestamp.is_some() {
                        Ok(None)
                    } else {
                        owner_fleet_work_item(
                            Arc::clone(context.fleet_storage()),
                            &resource,
                        )
                        .await
                    }
                }
                AgonesWatchEvent::Deleted { .. } => Ok(None),
            },
            FleetControllerWatchEvent::GameServer(event) => {
                match event {
                    AgonesWatchEvent::Added { resource, .. } => {
                        watch_state
                            .write()
                            .map_err(|_| {
                                AdapterError::Hook(
                                    "Fleet watch-state lock poisoned"
                                        .into(),
                                )
                            })?
                            .observe_added(&resource);
                    }
                    AgonesWatchEvent::Deleted { resource, .. } => {
                        watch_state
                            .write()
                            .map_err(|_| {
                                AdapterError::Hook(
                                    "Fleet watch-state lock poisoned"
                                        .into(),
                                )
                            })?
                            .observe_deleted(&resource);
                    }
                    AgonesWatchEvent::Modified { resource, .. } => {
                        let previous = watch_state
                            .write()
                            .map_err(|_| {
                                AdapterError::Hook(
                                    "Fleet watch-state lock poisoned"
                                        .into(),
                                )
                            })?
                            .observe_modified(&resource);

                        let transitioned_to_allocated =
                            resource.status.state
                                == Some(GameServerState::Allocated)
                                && previous.is_some()
                                && previous.flatten()
                                    != Some(GameServerState::Allocated);

                        if transitioned_to_allocated {
                            if let Some(fleet_name) = resource
                                .metadata
                                .labels
                                .get(FLEET_NAME_LABEL)
                                .filter(|name| !name.is_empty())
                            {
                                context
                                    .fleet_allocation_tracker()
                                    .increment(
                                        resource
                                            .metadata
                                            .namespace
                                            .as_str(),
                                        fleet_name.as_str(),
                                        1,
                                    );
                            }
                        }
                    }
                }

                Ok(None)
            }
        }
    }
}

#[async_trait]
#[cgp_component(FleetAllocationFlusher)]
pub trait CanFlushFleetAllocations {
    async fn flush_fleet_allocations(&self);
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetAllocationFlusher<Context>
    for FlushFleetAllocationsWithStorage
where
    Context: HasFleetStorage + HasFleetAllocationTracker + Sync,
{
    async fn flush_fleet_allocations(context: &Context) {
        let repository =
            AgonesRepository::new(Arc::clone(context.fleet_storage()));

        for (namespace, fleet_name, count) in context
            .fleet_allocation_tracker()
            .nonzero_snapshot()
        {
            let Ok(mut fleet) = repository
                .get::<Fleet>(
                    namespace.as_str(),
                    fleet_name.as_str(),
                )
                .await
            else {
                continue;
            };

            fleet.status.allocations =
                fleet.status.allocations.wrapping_add(count);
            let _ = repository.update(&fleet).await;
        }
    }
}

async fn run_fleet_worker<C>(
    controller: &C,
    queue: FleetWorkQueue,
) -> Result<(), AdapterError>
where
    C: CanReconcileFleet + Sync,
{
    while let Some(item) = queue.next().await {
        match controller
            .reconcile_fleet(
                item.namespace.as_str(),
                item.name.as_str(),
            )
            .await
        {
            Ok(
                FleetSyncOutcome::Reconciled(_)
                | FleetSyncOutcome::NotFound
                | FleetSyncOutcome::Deleting(_),
            ) => queue.complete_success(&item),
            Err(_) => {
                let delay = queue.record_failure(&item);
                tokio::time::sleep(delay).await;
                queue.complete_retry(&item);
            }
        }
    }

    Ok(())
}

#[async_trait]
#[cgp_component(FleetControllerRunner)]
pub trait CanRunFleetController {
    async fn run_fleet_controller(
        &self,
        workers: usize,
    ) -> Result<(), AdapterError>;
}

#[cgp_new_provider]
#[async_trait]
impl<Context> FleetControllerRunner<Context>
    for RunFleetControllerWithRusternetes
where
    Context: HasFleetStorage
        + CanRebuildFleetControllerWatchState
        + CanRouteFleetControllerEvent
        + CanReconcileFleet
        + CanFlushFleetAllocations
        + Sync,
{
    async fn run_fleet_controller(
        context: &Context,
        workers: usize,
    ) -> Result<(), AdapterError> {
        if workers == 0 {
            return Err(AdapterError::Hook(
                "Fleet controller requires at least one worker".into(),
            ));
        }

        // Subscribe before LIST so events racing startup are retained and then
        // collapsed by the keyed work queue / observation state.
        let mut events = watch_fleet_controller_events(
            Arc::clone(context.fleet_storage()),
        )
        .await?;
        let watch_state =
            Arc::new(RwLock::new(FleetControllerWatchState::default()));
        let initial_work = context
            .rebuild_fleet_controller_watch_state(&watch_state)
            .await?;

        let queue = FleetWorkQueue::new(fleet_retry_policy());
        for item in initial_work {
            queue.enqueue(item);
        }

        let mut worker_futures = FuturesUnordered::new();
        for _ in 0..workers {
            worker_futures.push(run_fleet_worker(
                context,
                queue.clone(),
            ));
        }

        loop {
            tokio::select! {
                event = events.next() => {
                    let Some(event) = event else {
                        context.flush_fleet_allocations().await;
                        return Ok(());
                    };
                    if let Some(item) = context
                        .route_fleet_controller_event(
                            &watch_state,
                            event?,
                        )
                        .await?
                    {
                        queue.enqueue(item);
                    }
                }
                worker = worker_futures.next() => {
                    match worker {
                        Some(Ok(())) => {
                            return Err(AdapterError::Hook(
                                "Fleet worker queue terminated unexpectedly".into(),
                            ));
                        }
                        Some(Err(error)) => return Err(error),
                        None => {
                            return Err(AdapterError::Hook(
                                "Fleet controller has no active workers".into(),
                            ));
                        }
                    }
                }
            }
        }
    }
}

// Runtime wiring is deliberately separate from the persistence/policy table in
// fleet_controller.rs. CGP allows capabilities to be stacked onto the same
// context incrementally without centralizing a monolithic implementation.
delegate_and_check_components! {
    <S: Storage + 'static>
    FleetController<S> {
        FleetControllerWatchRebuilderComponent:
            RebuildFleetControllerWatchStateWithStorage,
        FleetControllerEventRouterComponent:
            RouteFleetControllerEventWithStorage,
        FleetAllocationFlusherComponent:
            FlushFleetAllocationsWithStorage,
        FleetControllerRunnerComponent:
            RunFleetControllerWithRusternetes,
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use agones_core::{
        FleetSpec, GameServerSpec, GameServerStatus, ObjectMeta,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::FleetControllerConfig;

    use super::*;

    fn fleet() -> Fleet {
        let mut fleet = Fleet {
            metadata: ObjectMeta {
                name: "fleet".into(),
                namespace: "default".into(),
                uid: "fleet-uid".into(),
                ..Default::default()
            },
            spec: FleetSpec {
                replicas: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        fleet.apply_defaults();
        fleet
    }

    fn game_server(state: GameServerState) -> GameServer {
        GameServer {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: "server".into(),
                namespace: "default".into(),
                labels: std::collections::BTreeMap::from([(
                    FLEET_NAME_LABEL.into(),
                    "fleet".into(),
                )]),
                ..Default::default()
            },
            spec: GameServerSpec::default(),
            status: GameServerStatus {
                state: Some(state),
                ..Default::default()
            },
        }
    }

    #[test]
    fn retry_schedule_matches_upstream_fast_rate_limiter() {
        for retry in 0..FLEET_FAST_RETRIES {
            assert_eq!(
                fleet_retry_delay_ms(retry),
                FLEET_FAST_RETRY_DELAY_MS
            );
        }
        assert_eq!(
            fleet_retry_delay_ms(FLEET_FAST_RETRIES),
            FLEET_SLOW_RETRY_DELAY_MS
        );
    }

    #[tokio::test]
    async fn fleet_events_enqueue_updates_and_delete_tracker_state() {
        let controller =
            FleetController::new(MemoryStorage::new());
        let state =
            Arc::new(RwLock::new(FleetControllerWatchState::default()));
        let fleet = fleet();

        let added = controller
            .route_fleet_controller_event(
                &state,
                FleetControllerWatchEvent::Fleet(
                    AgonesWatchEvent::Added {
                        key: "default/fleet".into(),
                        resource: fleet.clone(),
                    },
                ),
            )
            .await
            .unwrap();
        assert_eq!(
            added,
            Some(FleetWorkItem::new("default", "fleet"))
        );

        controller
            .allocation_tracker
            .increment("default", "fleet", 3);
        let deleted = controller
            .route_fleet_controller_event(
                &state,
                FleetControllerWatchEvent::Fleet(
                    AgonesWatchEvent::Deleted {
                        key: "default/fleet".into(),
                        resource: fleet,
                    },
                ),
            )
            .await
            .unwrap();
        assert!(deleted.is_none());
        assert_eq!(
            controller
                .allocation_tracker
                .get("default", "fleet"),
            0
        );
    }

    #[tokio::test]
    async fn gameserverset_add_enqueues_live_owner_but_deleting_update_is_ignored() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let fleet = repository.create(&fleet()).await.unwrap();
        let controller = FleetController::new(storage);
        let state =
            Arc::new(RwLock::new(FleetControllerWatchState::default()));

        let mut game_server_set = fleet.game_server_set();
        game_server_set.metadata.name = "set".into();
        game_server_set.metadata.generate_name.clear();

        let added = controller
            .route_fleet_controller_event(
                &state,
                FleetControllerWatchEvent::GameServerSet(
                    AgonesWatchEvent::Added {
                        key: "default/set".into(),
                        resource: game_server_set.clone(),
                    },
                ),
            )
            .await
            .unwrap();
        assert_eq!(
            added,
            Some(FleetWorkItem::new("default", "fleet"))
        );

        game_server_set.metadata.deletion_timestamp =
            Some("2026-10-18T00:00:00Z".into());
        let deleting = controller
            .route_fleet_controller_event(
                &state,
                FleetControllerWatchEvent::GameServerSet(
                    AgonesWatchEvent::Modified {
                        key: "default/set".into(),
                        resource: game_server_set,
                    },
                ),
            )
            .await
            .unwrap();
        assert!(deleting.is_none());
    }

    #[tokio::test]
    async fn allocation_counter_tracks_only_observed_transition_into_allocated() {
        let controller =
            FleetController::new(MemoryStorage::new());
        let state =
            Arc::new(RwLock::new(FleetControllerWatchState::default()));

        let ready = game_server(GameServerState::Ready);
        controller
            .route_fleet_controller_event(
                &state,
                FleetControllerWatchEvent::GameServer(
                    AgonesWatchEvent::Added {
                        key: "default/server".into(),
                        resource: ready,
                    },
                ),
            )
            .await
            .unwrap();

        let allocated = game_server(GameServerState::Allocated);
        controller
            .route_fleet_controller_event(
                &state,
                FleetControllerWatchEvent::GameServer(
                    AgonesWatchEvent::Modified {
                        key: "default/server".into(),
                        resource: allocated.clone(),
                    },
                ),
            )
            .await
            .unwrap();
        assert_eq!(
            controller
                .allocation_tracker
                .get("default", "fleet"),
            1
        );

        controller
            .route_fleet_controller_event(
                &state,
                FleetControllerWatchEvent::GameServer(
                    AgonesWatchEvent::Modified {
                        key: "default/server".into(),
                        resource: allocated,
                    },
                ),
            )
            .await
            .unwrap();
        assert_eq!(
            controller
                .allocation_tracker
                .get("default", "fleet"),
            1
        );
    }

    #[tokio::test]
    async fn rebuild_baseline_does_not_count_existing_allocated_servers() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        repository.create(&fleet()).await.unwrap();
        repository
            .create(&game_server(GameServerState::Allocated))
            .await
            .unwrap();

        let controller = FleetController::new(storage);
        let state =
            Arc::new(RwLock::new(FleetControllerWatchState::default()));

        let work = controller
            .rebuild_fleet_controller_watch_state(&state)
            .await
            .unwrap();
        assert_eq!(
            work,
            vec![FleetWorkItem::new("default", "fleet")]
        );

        controller
            .route_fleet_controller_event(
                &state,
                FleetControllerWatchEvent::GameServer(
                    AgonesWatchEvent::Modified {
                        key: "default/server".into(),
                        resource: game_server(
                            GameServerState::Allocated,
                        ),
                    },
                ),
            )
            .await
            .unwrap();
        assert_eq!(
            controller
                .allocation_tracker
                .get("default", "fleet"),
            0
        );
    }

    #[tokio::test]
    async fn shutdown_flush_persists_pending_allocations() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        repository.create(&fleet()).await.unwrap();

        let controller = FleetController::new(storage.clone());
        controller
            .allocation_tracker
            .increment("default", "fleet", 4);
        controller.flush_fleet_allocations().await;

        let stored: Fleet =
            repository.get("default", "fleet").await.unwrap();
        assert_eq!(stored.status.allocations, 4);
    }

    #[tokio::test]
    async fn runtime_reconciles_initial_fleet_snapshot() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        repository.create(&fleet()).await.unwrap();

        let controller = FleetController::new(storage.clone())
            .with_config(FleetControllerConfig {
                player_tracking: false,
            });
        let task = tokio::spawn(async move {
            controller.run_fleet_controller(1).await
        });

        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                let sets = repository
                    .list::<GameServerSet>("default")
                    .await
                    .unwrap();
                if sets.len() == 1 {
                    assert_eq!(sets[0].spec.replicas, 1);
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("Fleet runtime did not reconcile initial informer work");

        task.abort();
    }

    #[tokio::test]
    async fn zero_workers_is_rejected() {
        let controller =
            FleetController::new(MemoryStorage::new());
        let error = controller
            .run_fleet_controller(0)
            .await
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("requires at least one worker")
        );
    }
}
