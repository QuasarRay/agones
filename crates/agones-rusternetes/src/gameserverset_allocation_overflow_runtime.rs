use std::sync::{Arc, RwLock};
use std::time::Duration;

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

type AllocationOverflowWorkQueue =
    ControllerWorkQueue<GameServerSetWorkItem, FastSlowRetryPolicy>;

/// Result of one AllocationOverflowController reconciliation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocationOverflowSyncOutcome {
    NotFound,
    Unchanged,
    Updated(Vec<GameServer>),
}

/// Exact upstream UpdateFunc enqueue predicate.
///
/// Allocation overflow reacts only when metadata is configured and the number
/// of Allocated GameServers currently exceeds the desired replica count.
pub fn should_enqueue_allocation_overflow(
    game_server_set: &GameServerSet,
) -> bool {
    let Some(overflow) = game_server_set.spec.allocation_overflow.as_ref()
    else {
        return false;
    };

    if overflow.labels.is_empty() && overflow.annotations.is_empty() {
        return false;
    }

    game_server_set.status.allocated_replicas
        > game_server_set.spec.replicas
}

/// Routes the AllocationOverflowController's UpdateFunc.
///
/// Upstream registers no AddFunc and no DeleteFunc, so Added/Deleted events
/// are intentionally ignored. Modified events inspect only the new object.
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

/// Rusternetes equivalent of AllocationOverflowController.syncGameServerSet.
///
/// The live GameServerSet is re-read at worker execution time, then the same
/// defensive guards as upstream are repeated. Only Allocated GameServers owned
/// by this set are considered. Existing matches satisfy the overflow first;
/// remaining candidates are sorted with the same GameServerSet scheduling
/// strategy and PerNodeCounter snapshot before labels/annotations are applied.
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
    let repository = AgonesRepository::new(storage);
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

    let Some(allocation_overflow) =
        game_server_set.spec.allocation_overflow.as_ref()
    else {
        return Ok(AllocationOverflowSyncOutcome::Unchanged);
    };

    if game_server_set.status.allocated_replicas
        <= game_server_set.spec.replicas
    {
        return Ok(AllocationOverflowSyncOutcome::Unchanged);
    }

    let overflow =
        game_server_set.status.allocated_replicas - game_server_set.spec.replicas;
    let game_servers = self
        .list_owned_game_servers(&game_server_set)
        .await?;
    let (matches, rest) =
        allocation_overflow.count_matches(&game_servers);

    if matches >= overflow {
        return Ok(AllocationOverflowSyncOutcome::Unchanged);
    }

    let required = usize::try_from(overflow - matches)
        .map_err(|_| {
            AdapterError::Hook(
                "allocation overflow deficit cannot be negative".into(),
            )
        })?;

    let mut candidates: Vec<GameServer> =
        rest.into_iter().cloned().collect();

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
        &mut candidates,
        &counts,
        &game_server_set.spec.priorities,
    );

    // Upstream slices rest[:overflow-matches]. A shorter candidate list would
    // panic there and indicates inconsistent status/list observations. Surface
    // the same impossible-state boundary as a retryable controller error
    // rather than silently under-applying overflow metadata.
    if candidates.len() < required {
        return Err(AdapterError::Hook(format!(
            "allocation overflow requires {required} GameServers but only {} eligible allocated candidates were found",
            candidates.len()
        )));
    }

    let mut updated = Vec::with_capacity(required);
    for mut game_server in candidates.into_iter().take(required) {
        allocation_overflow.apply(&mut game_server);
        updated.push(repository.update(&game_server).await?);
    }

    if updated.is_empty() {
        Ok(AllocationOverflowSyncOutcome::Unchanged)
    } else {
        Ok(AllocationOverflowSyncOutcome::Updated(updated))
    }
}

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

/// Long-running Rusternetes equivalent of AllocationOverflowController.Run.
///
/// Upstream runs exactly one worker and registers only a GameServerSet
/// UpdateFunc. Consequently this runtime subscribes the watch but deliberately
/// performs no initial LIST replay: a pre-existing overflow is processed only
/// after a GameServerSet update, matching SharedInformer behavior.
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
    let queue = AllocationOverflowWorkQueue::new(
        game_server_set_retry_policy(),
    );
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
                return match result {
                    Ok(()) => Err(AdapterError::Hook(
                        "AllocationOverflow worker queue terminated unexpectedly"
                            .into(),
                    )),
                    Err(error) => Err(error),
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use agones_core::{
        AllocationOverflow, Container, GameServerPort, GameServerSetSpec,
        GameServerState, GameServerTemplateSpec, ObjectMeta, PodSpec,
        PodTemplateSpec, PortPolicy, Protocol,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::{
        GameServerSetController, GameServerSetControllerConfig,
    };

    use super::*;

    fn set(
        replicas: i32,
        allocated_replicas: i32,
        allocation_overflow: Option<AllocationOverflow>,
    ) -> GameServerSet {
        GameServerSet {
            metadata: ObjectMeta {
                name: "set".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                replicas,
                allocation_overflow,
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
            status: agones_core::GameServerSetStatus {
                replicas,
                allocated_replicas,
                ..Default::default()
            },
        }
    }

    fn config() -> GameServerSetControllerConfig {
        GameServerSetControllerConfig {
            max_creation_parallelism: 2,
            max_deletion_parallelism: 2,
            max_game_server_creations_per_batch: 100,
            max_game_server_deletions_per_batch: 100,
            max_pod_pending_count: 100,
            player_tracking: false,
        }
    }

    fn overflow() -> AllocationOverflow {
        AllocationOverflow {
            labels: BTreeMap::from([
                ("colour".into(), "green".into()),
            ]),
            ..Default::default()
        }
    }

    #[test]
    fn watch_routing_matches_upstream_update_only_predicate() {
        let no_overflow = set(3, 20, None);
        assert!(route_allocation_overflow_event(
            AgonesWatchEvent::Added {
                key: "default/set".into(),
                resource: no_overflow.clone(),
            },
        )
        .is_none());
        assert!(route_allocation_overflow_event(
            AgonesWatchEvent::Modified {
                key: "default/set".into(),
                resource: no_overflow,
            },
        )
        .is_none());

        let empty = set(
            3,
            20,
            Some(AllocationOverflow::default()),
        );
        assert!(route_allocation_overflow_event(
            AgonesWatchEvent::Modified {
                key: "default/set".into(),
                resource: empty,
            },
        )
        .is_none());

        let not_overflowing = set(3, 2, Some(overflow()));
        assert!(route_allocation_overflow_event(
            AgonesWatchEvent::Modified {
                key: "default/set".into(),
                resource: not_overflowing,
            },
        )
        .is_none());

        let overflowing = set(3, 20, Some(overflow()));
        assert_eq!(
            route_allocation_overflow_event(
                AgonesWatchEvent::Modified {
                    key: "default/set".into(),
                    resource: overflowing.clone(),
                },
            ),
            Some(GameServerSetWorkItem::new("default", "set"))
        );
        assert!(route_allocation_overflow_event(
            AgonesWatchEvent::Deleted {
                key: "default/set".into(),
                resource: overflowing,
            },
        )
        .is_none());
    }

    async fn fixture(
        prelabelled: usize,
    ) -> (
        MemoryStorage,
        GameServerSetController<MemoryStorage>,
        GameServerSet,
    ) {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let set = repository
            .create(&set(3, 5, Some(overflow())))
            .await
            .unwrap();

        for index in 0..5 {
            let mut game_server = set.game_server();
            game_server.metadata.name = format!("server-{index}");
            game_server.metadata.generate_name.clear();
            game_server.status.state = Some(GameServerState::Allocated);
            if index < prelabelled {
                game_server
                    .metadata
                    .labels
                    .insert("colour".into(), "green".into());
            }
            game_server.apply_defaults();
            repository.create(&game_server).await.unwrap();
        }

        let controller = GameServerSetController::new(
            storage.clone(),
            config(),
        );
        (storage, controller, set)
    }

    async fn matching_count(storage: &MemoryStorage) -> usize {
        AgonesRepository::new(storage.clone())
            .list::<GameServer>("default")
            .await
            .unwrap()
            .into_iter()
            .filter(|game_server| {
                game_server.status.state == Some(GameServerState::Allocated)
                    && game_server.metadata.labels.get("colour")
                        == Some(&"green".to_owned())
            })
            .count()
    }

    #[tokio::test]
    async fn sync_applies_labels_to_exact_overflow_deficit() {
        let (storage, controller, _) = fixture(0).await;

        let outcome = controller
            .sync_allocation_overflow_game_server_set(
                "default",
                "set",
            )
            .await
            .unwrap();

        let AllocationOverflowSyncOutcome::Updated(updated) = outcome else {
            panic!("overflow should update GameServers");
        };
        assert_eq!(updated.len(), 2);
        assert_eq!(matching_count(&storage).await, 2);
        assert!(updated.iter().all(|game_server| {
            game_server.status.state == Some(GameServerState::Allocated)
                && game_server.metadata.labels.get("colour")
                    == Some(&"green".to_owned())
        }));
    }

    #[tokio::test]
    async fn sync_does_not_rewrite_when_all_overflow_is_already_labelled() {
        let (storage, controller, _) = fixture(5).await;

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
        assert_eq!(matching_count(&storage).await, 5);
    }

    #[tokio::test]
    async fn sync_updates_only_remaining_deficit() {
        let (storage, controller, _) = fixture(1).await;

        let outcome = controller
            .sync_allocation_overflow_game_server_set(
                "default",
                "set",
            )
            .await
            .unwrap();

        let AllocationOverflowSyncOutcome::Updated(updated) = outcome else {
            panic!("one remaining overflow GameServer should update");
        };
        assert_eq!(updated.len(), 1);
        assert_eq!(matching_count(&storage).await, 2);
    }

    #[tokio::test]
    async fn runtime_has_no_initial_replay_and_processes_first_update() {
        let (storage, controller, set) = fixture(0).await;
        let repository = AgonesRepository::new(storage.clone());

        let task = tokio::spawn(async move {
            controller.run_allocation_overflow_controller().await
        });

        // Upstream registers only UpdateFunc, so the existing object itself
        // must not trigger work when the controller starts.
        tokio::time::sleep(Duration::from_millis(25)).await;
        assert_eq!(matching_count(&storage).await, 0);

        let current: GameServerSet =
            repository.get("default", "set").await.unwrap();
        let mut modified = current;
        modified
            .metadata
            .annotations
            .insert("touch".into(), "1".into());
        repository.update(&modified).await.unwrap();

        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                if matching_count(&storage).await == 2 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("AllocationOverflow runtime did not process update");

        // Keep the fixture owner live through the test so ownership is clear.
        assert_eq!(set.metadata.name, "set");
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
