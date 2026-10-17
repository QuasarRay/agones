use std::sync::{Arc, RwLock};

use agones_core::{
    GameServer, GameServerSet, SchedulingStrategy,
    sort_game_servers_by_strategy,
};
use cgp::prelude::*;
use futures::StreamExt;
use rusternetes_storage::Storage;

use crate::controller_work_queue::{
    ControllerWorkQueue, FastSlowRetryPolicy,
};
use crate::gameserverset_controller_runtime::game_server_set_retry_policy;
use crate::{
    AdapterError, AgonesRepository, AgonesWatchEvent,
    GameServerSetWorkItem, ListOwnedGameServers, PerNodeCounterState,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocationOverflowSyncOutcome {
    NotFound,
    Unchanged,
    Applied(usize),
}

pub fn should_enqueue_allocation_overflow(
    game_server_set: &GameServerSet,
) -> bool {
    let Some(overflow) = game_server_set.spec.allocation_overflow.as_ref()
    else {
        return false;
    };

    (!overflow.labels.is_empty() || !overflow.annotations.is_empty())
        && game_server_set.status.allocated_replicas
            > game_server_set.spec.replicas
}

pub fn route_allocation_overflow_event(
    event: AgonesWatchEvent<GameServerSet>,
) -> Option<GameServerSetWorkItem> {
    let AgonesWatchEvent::Modified { resource, .. } = event else {
        return None;
    };

    should_enqueue_allocation_overflow(&resource).then(|| {
        GameServerSetWorkItem::new(
            resource.metadata.namespace,
            resource.metadata.name,
        )
    })
}

/// Applies AllocationOverflow labels/annotations to the additional Allocated
/// GameServers selected by the upstream scheduling strategy.
///
/// The controller re-fetches the current GameServerSet before every pass and
/// repeats all watch-time guards. Owned children are provided by the same
/// label+controller-UID listing capability used by the main GameServerSet
/// controller.
#[cgp_fn]
#[async_trait]
pub async fn sync_allocation_overflow_game_server_set<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] per_node_counter_state: Arc<RwLock<PerNodeCounterState>>,
    namespace: &str,
    name: &str,
) -> Result<AllocationOverflowSyncOutcome, AdapterError>
where
    S: Storage + 'static,
    Self: ListOwnedGameServers<S> + Sync,
{
    let repository = AgonesRepository::new(storage.clone());
    let game_server_set = match repository
        .get::<GameServerSet>(namespace, name)
        .await
    {
        Ok(game_server_set) => game_server_set,
        Err(AdapterError::Rusternetes(
            rusternetes_common::Error::NotFound(_),
        )) => return Ok(AllocationOverflowSyncOutcome::NotFound),
        Err(error) => return Err(error),
    };

    let Some(overflow_policy) =
        game_server_set.spec.allocation_overflow.as_ref()
    else {
        return Ok(AllocationOverflowSyncOutcome::Unchanged);
    };

    if game_server_set.status.allocated_replicas
        <= game_server_set.spec.replicas
    {
        return Ok(AllocationOverflowSyncOutcome::Unchanged);
    }

    let overflow_count = game_server_set.status.allocated_replicas
        - game_server_set.spec.replicas;
    let game_servers = self
        .list_owned_game_servers(&game_server_set)
        .await?;
    let (matches, rest) = overflow_policy.count_matches(&game_servers);

    if matches >= overflow_count {
        return Ok(AllocationOverflowSyncOutcome::Unchanged);
    }

    let needed = (overflow_count - matches) as usize;
    let mut rest = rest.into_iter().cloned().collect::<Vec<_>>();

    let counts = per_node_counter_state
        .read()
        .map_err(|_| {
            AdapterError::Hook("PerNodeCounter state lock poisoned".into())
        })?
        .counts();

    let strategy = game_server_set
        .spec
        .scheduling
        .clone()
        .unwrap_or(SchedulingStrategy::Packed);
    sort_game_servers_by_strategy(
        &strategy,
        &mut rest,
        &counts,
        &game_server_set.spec.priorities,
    );

    // Upstream slices directly because its informer/status invariants imply
    // enough Allocated children exist. A temporarily inconsistent Rusternetes
    // snapshot should be retried rather than panicking the controller.
    if rest.len() < needed {
        return Err(AdapterError::Hook(format!(
            "AllocationOverflow expected {needed} unmatched Allocated GameServers but observed {}",
            rest.len()
        )));
    }

    for mut game_server in rest.into_iter().take(needed) {
        overflow_policy.apply(&mut game_server);
        repository.update(&game_server).await?;
    }

    Ok(AllocationOverflowSyncOutcome::Applied(needed))
}

type AllocationOverflowWorkQueue =
    ControllerWorkQueue<GameServerSetWorkItem, FastSlowRetryPolicy>;

async fn run_allocation_overflow_worker<C, S>(
    controller: &C,
    queue: AllocationOverflowWorkQueue,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    C: SyncAllocationOverflowGameServerSet<S> + Sync,
{
    while let Some(item) = queue.next().await {
        match controller
            .sync_allocation_overflow_game_server_set(
                item.namespace.as_str(),
                item.name.as_str(),
            )
            .await
        {
            Ok(_) => queue.complete_success(&item),
            Err(_) => {
                let delay = queue.record_failure(&item);
                tokio::time::sleep(delay).await;
                queue.complete_retry(&item);
            }
        }
    }

    Ok(())
}

/// Detached one-worker runtime corresponding to upstream
/// AllocationOverflowController.Run.
///
/// There is deliberately no initial LIST replay: the upstream controller has
/// only a GameServerSet UpdateFunc, so existing objects are not processed until
/// a later GameServerSet update (normally status aggregation) fires.
#[cgp_fn]
#[async_trait]
pub async fn run_allocation_overflow_controller<S>(
    &self,
    #[implicit] storage: Arc<S>,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    Self: SyncAllocationOverflowGameServerSet<S> + Sync,
{
    let mut events = AgonesRepository::new(storage)
        .watch_all::<GameServerSet>()
        .await?;
    let queue =
        AllocationOverflowWorkQueue::new(game_server_set_retry_policy());
    let worker = run_allocation_overflow_worker::<Self, S>(
        self,
        queue.clone(),
    );
    tokio::pin!(worker);

    loop {
        tokio::select! {
            event = events.next() => {
                let Some(event) = event else {
                    return Ok(());
                };
                if let Some(item) =
                    route_allocation_overflow_event(event?)
                {
                    queue.enqueue(item);
                }
            }
            result = &mut worker => {
                match result {
                    Ok(()) => {
                        return Err(AdapterError::Hook(
                            "AllocationOverflow worker queue terminated unexpectedly".into(),
                        ));
                    }
                    Err(error) => return Err(error),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::time::Duration;

    use agones_core::{
        AllocationOverflow, Container, GameServerPort,
        GameServerSetSpec, GameServerSetStatus,
        GameServerTemplateSpec, ObjectMeta, PodSpec,
        PodTemplateSpec, PortPolicy, Protocol,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::{
        GameServerSetController, GameServerSetControllerConfig,
    };

    use super::*;

    fn set() -> GameServerSet {
        GameServerSet {
            metadata: ObjectMeta {
                name: "set".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                replicas: 3,
                allocation_overflow: Some(AllocationOverflow {
                    labels: BTreeMap::from([(
                        "colour".into(),
                        "green".into(),
                    )]),
                    ..Default::default()
                }),
                scheduling: Some(SchedulingStrategy::Distributed),
                template: GameServerTemplateSpec {
                    spec: agones_core::GameServerSpec {
                        container: "game".into(),
                        ports: vec![GameServerPort {
                            name: "game".into(),
                            port_policy: Some(PortPolicy::Static),
                            container: Some("game".into()),
                            container_port: 7777,
                            host_port: 7777,
                            protocol: Some(Protocol::Udp),
                            ..Default::default()
                        }],
                        template: PodTemplateSpec {
                            spec: PodSpec {
                                containers: vec![Container {
                                    name: "game".into(),
                                    image: "game:v1".into(),
                                    ..Default::default()
                                }],
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
            status: GameServerSetStatus {
                replicas: 3,
                allocated_replicas: 5,
                ..Default::default()
            },
        }
    }

    fn config() -> GameServerSetControllerConfig {
        GameServerSetControllerConfig::default()
    }

    async fn setup(
        preset_labels: usize,
    ) -> (
        MemoryStorage,
        AgonesRepository<MemoryStorage>,
        GameServerSet,
    ) {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let game_server_set = repository.create(&set()).await.unwrap();

        for index in 0..5 {
            let mut game_server = game_server_set.game_server();
            game_server.metadata.name = format!("gs-{index}");
            game_server.metadata.generate_name.clear();
            game_server.apply_defaults();
            game_server.status.state = Some(
                agones_core::GameServerState::Allocated,
            );
            if index < preset_labels {
                game_server
                    .metadata
                    .labels
                    .insert("colour".into(), "green".into());
            }
            repository.create(&game_server).await.unwrap();
        }

        (storage, repository, game_server_set)
    }

    #[test]
    fn watch_predicate_matches_upstream_update_guards() {
        let mut game_server_set = set();

        game_server_set.spec.allocation_overflow = None;
        assert!(!should_enqueue_allocation_overflow(&game_server_set));

        game_server_set.spec.allocation_overflow =
            Some(AllocationOverflow::default());
        assert!(!should_enqueue_allocation_overflow(&game_server_set));

        game_server_set
            .spec
            .allocation_overflow
            .as_mut()
            .unwrap()
            .labels
            .insert("colour".into(), "green".into());
        game_server_set.status.allocated_replicas =
            game_server_set.spec.replicas;
        assert!(!should_enqueue_allocation_overflow(&game_server_set));

        game_server_set.status.allocated_replicas += 1;
        assert!(should_enqueue_allocation_overflow(&game_server_set));
    }

    #[test]
    fn only_modified_events_can_enqueue_overflow_work() {
        let current = set();
        assert!(route_allocation_overflow_event(
            AgonesWatchEvent::Added {
                key: "default/set".into(),
                resource: current.clone(),
            }
        )
        .is_none());
        assert!(route_allocation_overflow_event(
            AgonesWatchEvent::Deleted {
                key: "default/set".into(),
                resource: current.clone(),
            }
        )
        .is_none());
        assert_eq!(
            route_allocation_overflow_event(
                AgonesWatchEvent::Modified {
                    key: "default/set".into(),
                    resource: current,
                }
            ),
            Some(GameServerSetWorkItem::new("default", "set"))
        );
    }

    #[tokio::test]
    async fn sync_applies_labels_to_exact_overflow_count() {
        let (storage, repository, _) = setup(0).await;
        let controller = GameServerSetController::new(
            storage,
            config(),
        );

        let outcome = controller
            .sync_allocation_overflow_game_server_set(
                "default",
                "set",
            )
            .await
            .unwrap();
        assert_eq!(
            outcome,
            AllocationOverflowSyncOutcome::Applied(2)
        );

        let game_servers =
            repository.list::<GameServer>("default").await.unwrap();
        assert_eq!(
            game_servers
                .iter()
                .filter(|game_server| {
                    game_server.metadata.labels.get("colour").map(String::as_str)
                        == Some("green")
                })
                .count(),
            2
        );
        assert!(game_servers.iter().all(|game_server| {
            game_server.status.state
                == Some(agones_core::GameServerState::Allocated)
        }));
    }

    #[tokio::test]
    async fn existing_matches_reduce_or_eliminate_updates() {
        let (storage, repository, _) = setup(1).await;
        let controller = GameServerSetController::new(
            storage,
            config(),
        );
        assert_eq!(
            controller
                .sync_allocation_overflow_game_server_set(
                    "default",
                    "set",
                )
                .await
                .unwrap(),
            AllocationOverflowSyncOutcome::Applied(1)
        );
        assert_eq!(
            repository
                .list::<GameServer>("default")
                .await
                .unwrap()
                .iter()
                .filter(|game_server| {
                    game_server.metadata.labels.get("colour").map(String::as_str)
                        == Some("green")
                })
                .count(),
            2
        );

        assert_eq!(
            controller
                .sync_allocation_overflow_game_server_set(
                    "default",
                    "set",
                )
                .await
                .unwrap(),
            AllocationOverflowSyncOutcome::Unchanged
        );
    }

    #[tokio::test]
    async fn runtime_does_not_replay_existing_overflow_before_an_update() {
        let (storage, repository, _) = setup(0).await;
        let controller = GameServerSetController::new(
            storage.clone(),
            config(),
        );

        let task = tokio::spawn(async move {
            controller.run_allocation_overflow_controller().await
        });

        // SharedInformer invokes only registered handlers during initial
        // population. AllocationOverflow registers UpdateFunc only, so an
        // already-overflowing object must not be processed at startup.
        tokio::time::sleep(Duration::from_millis(25)).await;
        let matches = repository
            .list::<GameServer>("default")
            .await
            .unwrap()
            .iter()
            .filter(|game_server| {
                game_server.metadata.labels.get("colour").map(String::as_str)
                    == Some("green")
            })
            .count();
        assert_eq!(matches, 0);

        task.abort();
    }

    #[tokio::test]
    async fn live_gameserverset_update_drives_overflow_runtime() {
        let (storage, repository, mut game_server_set) = setup(0).await;

        // Start below the watch predicate, then create the status update only
        // after the UpdateFunc-equivalent watch is subscribed.
        game_server_set.status.allocated_replicas =
            game_server_set.spec.replicas;
        repository.update(&game_server_set).await.unwrap();

        let controller = GameServerSetController::new(
            storage,
            config(),
        );
        let task = tokio::spawn(async move {
            controller.run_allocation_overflow_controller().await
        });
        tokio::task::yield_now().await;

        game_server_set.status.allocated_replicas = 5;
        repository.update(&game_server_set).await.unwrap();

        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                let game_servers = repository
                    .list::<GameServer>("default")
                    .await
                    .unwrap();
                let matches = game_servers
                    .iter()
                    .filter(|game_server| {
                        game_server.metadata.labels.get("colour")
                            == Some(&"green".to_owned())
                    })
                    .count();
                if matches == 2 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("AllocationOverflow runtime did not apply metadata");

        task.abort();
    }

    #[tokio::test]
    async fn missing_gameserverset_is_successful_noop() {
        let controller = GameServerSetController::new(
            MemoryStorage::new(),
            config(),
        );
        assert_eq!(
            controller
                .sync_allocation_overflow_game_server_set(
                    "default",
                    "missing",
                )
                .await
                .unwrap(),
            AllocationOverflowSyncOutcome::NotFound
        );
    }
}
