use std::sync::Arc;

use cgp::prelude::*;
use futures::stream::{BoxStream, FuturesUnordered};
use futures::StreamExt;
use rusternetes_common::resources::Pod;
use rusternetes_storage::{Storage, WatchEvent, build_key, build_prefix};

use crate::controller_work_queue::{
    ControllerWorkItem, ControllerWorkQueue, DefaultControllerRetryPolicy,
};
use crate::{
    AdapterError, ApiHooks, ReconcileMigrationGameServer,
    migrating_game_server_for_pod,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MigrationControllerWorkItem {
    pub namespace: String,
    pub name: String,
}

impl MigrationControllerWorkItem {
    pub fn key(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }
}

impl ControllerWorkItem for MigrationControllerWorkItem {
    fn work_key(&self) -> String {
        self.key()
    }
}

#[derive(Clone, Debug)]
pub enum MigrationControllerWatchEvent {
    PodAdded(Pod),
    PodModified(Pod),
}

pub type MigrationControllerWatchStream =
    BoxStream<'static, Result<MigrationControllerWatchEvent, AdapterError>>;

fn work_item_from_pod(pod: &Pod) -> Option<MigrationControllerWorkItem> {
    let namespace = pod
        .metadata
        .namespace
        .as_deref()
        .filter(|namespace| !namespace.is_empty())?;
    if pod.metadata.name.is_empty() {
        return None;
    }

    Some(MigrationControllerWorkItem {
        namespace: namespace.to_owned(),
        name: pod.metadata.name.clone(),
    })
}

/// Subscribes to the Pod informer source used by upstream MigrationController.
///
/// Only Pod Add and Update have upstream handlers. Deletes are intentionally
/// discarded.
pub async fn watch_migration_controller_events<S>(
    storage: Arc<S>,
) -> Result<MigrationControllerWatchStream, AdapterError>
where
    S: Storage + 'static,
{
    let stream = storage
        .watch(&build_prefix("pods", None))
        .await?
        .filter_map(|event| async move {
            match event {
                Ok(WatchEvent::Added(_, raw)) => Some(
                    serde_json::from_str::<Pod>(&raw)
                        .map(MigrationControllerWatchEvent::PodAdded)
                        .map_err(AdapterError::from),
                ),
                Ok(WatchEvent::Modified(_, raw)) => Some(
                    serde_json::from_str::<Pod>(&raw)
                        .map(MigrationControllerWatchEvent::PodModified)
                        .map_err(AdapterError::from),
                ),
                Ok(WatchEvent::Deleted(_, _)) => None,
                Err(error) => Some(Err(AdapterError::from(error))),
            }
        });

    Ok(Box::pin(stream))
}

/// Applies the upstream MigrationController AddFunc/UpdateFunc predicate.
///
/// Both handlers call isMigratingGameServerPod and enqueue when migration is
/// detected or when the lookup predicate itself errors. On predicate errors we
/// deliberately enqueue the Pod identity and let sync_migration_game_server
/// retry the same control-plane reads through the controller work queue.
pub async fn route_migration_controller_event<S>(
    storage: Arc<S>,
    event: MigrationControllerWatchEvent,
) -> Result<Option<MigrationControllerWorkItem>, AdapterError>
where
    S: Storage + 'static,
{
    let pod = match event {
        MigrationControllerWatchEvent::PodAdded(pod)
        | MigrationControllerWatchEvent::PodModified(pod) => pod,
    };

    match migrating_game_server_for_pod(storage, &pod).await {
        Ok(Some(_)) => Ok(work_item_from_pod(&pod)),
        Ok(None) => Ok(None),
        Err(_) => Ok(work_item_from_pod(&pod)),
    }
}

/// Key-based equivalent of upstream MigrationController.syncGameServer.
#[cgp_fn]
#[async_trait]
pub async fn sync_migration_game_server<S, H>(
    &self,
    #[implicit] storage: Arc<S>,
    namespace: &str,
    name: &str,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
    Self: ReconcileMigrationGameServer<S, H> + Sync,
{
    let pod_key = build_key("pods", Some(namespace), name);
    let pod: Pod = match storage.get(&pod_key).await {
        Ok(pod) => pod,
        Err(rusternetes_common::Error::NotFound(_)) => return Ok(()),
        Err(error) => return Err(error.into()),
    };

    let _ = self.reconcile_migration_game_server(pod).await?;
    Ok(())
}

async fn run_migration_worker<C, S, H>(
    controller: &C,
    queue: ControllerWorkQueue<
        MigrationControllerWorkItem,
        DefaultControllerRetryPolicy,
    >,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
    C: SyncMigrationGameServer<S, H> + Sync,
{
    while let Some(item) = queue.next().await {
        match controller
            .sync_migration_game_server(
                item.namespace.as_str(),
                item.name.as_str(),
            )
            .await
        {
            Ok(()) => queue.complete_success(&item),
            Err(_) => {
                let delay = queue.record_failure(&item);
                tokio::time::sleep(delay).await;
                queue.complete_retry(&item);
            }
        }
    }

    Ok(())
}

/// Long-running Rusternetes equivalent of upstream MigrationController.Run.
///
/// Watches are subscribed before the initial Pod LIST. Existing Pods are replayed
/// through the AddFunc predicate, matching SharedInformer startup semantics.
/// Live Pod Add/Update events then feed the same queue. Nodes and GameServers
/// need no event handlers here; their current state is consulted by the
/// migration predicate when each Pod event is evaluated.
#[cgp_fn]
#[async_trait]
pub async fn run_migration_game_server_controller<S, H>(
    &self,
    #[implicit] storage: Arc<S>,
    workers: usize,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
    Self: SyncMigrationGameServer<S, H> + Sync,
{
    if workers == 0 {
        return Err(AdapterError::Hook(
            "MigrationController requires at least one worker".into(),
        ));
    }

    let mut events =
        watch_migration_controller_events(storage.clone()).await?;
    let queue = ControllerWorkQueue::new(
        DefaultControllerRetryPolicy::default(),
    );

    let pods: Vec<Pod> = storage.list(&build_prefix("pods", None)).await?;
    for pod in pods {
        if let Some(item) = route_migration_controller_event(
            storage.clone(),
            MigrationControllerWatchEvent::PodAdded(pod),
        )
        .await?
        {
            queue.enqueue(item);
        }
    }

    let mut worker_futures = FuturesUnordered::new();
    for _ in 0..workers {
        worker_futures.push(run_migration_worker::<Self, S, H>(
            self,
            queue.clone(),
        ));
    }

    loop {
        tokio::select! {
            event = events.next() => {
                let Some(event) = event else {
                    return Ok(());
                };

                if let Some(item) = route_migration_controller_event(
                    storage.clone(),
                    event?,
                ).await? {
                    queue.enqueue(item);
                }
            }
            worker = worker_futures.next() => {
                match worker {
                    Some(Ok(())) => {
                        return Err(AdapterError::Hook(
                            "MigrationController worker queue terminated unexpectedly".into(),
                        ));
                    }
                    Some(Err(error)) => return Err(error),
                    None => {
                        return Err(AdapterError::Hook(
                            "MigrationController has no active workers".into(),
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServer, GameServerSpec,
        GameServerState, GameServerStatus, ObjectMeta,
        PodSpec as CorePodSpec, PodTemplateSpec,
    };
    use rusternetes_common::resources::{
        Node, NodeAddress, NodeStatus,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::{
        AgonesRepository, CreatingGameServerController, NoopApiHooks,
        PodBuildFeatures, build_pod,
    };

    use super::*;

    fn game_server(state: GameServerState) -> GameServer {
        GameServer {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: "server".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSpec {
                container: "game".into(),
                template: PodTemplateSpec {
                    spec: CorePodSpec {
                        containers: vec![CoreContainer {
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
            status: GameServerStatus {
                state: Some(state),
                ..Default::default()
            },
        }
    }

    fn node(name: &str, address: &str) -> Node {
        let mut node = Node::new(name);
        node.status = Some(NodeStatus {
            addresses: Some(vec![NodeAddress {
                address_type: "ExternalIP".into(),
                address: address.into(),
            }]),
            ..Default::default()
        });
        node
    }

    fn pod_for(game_server: &GameServer, node_name: &str) -> Pod {
        let mut pod = build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.spec.as_mut().unwrap().node_name = Some(node_name.into());
        pod
    }

    #[tokio::test]
    async fn routing_matches_upstream_add_and_update_predicate() {
        let storage = Arc::new(MemoryStorage::new());
        let mut gs = game_server(GameServerState::Allocated);
        gs.status.node_name = "node-a".into();
        gs.status.address = "203.0.113.10".into();

        let mut pod = pod_for(&gs, "node-a");

        assert!(
            route_migration_controller_event(
                storage.clone(),
                MigrationControllerWatchEvent::PodAdded(pod.clone()),
            )
            .await
            .unwrap()
            .is_none(),
            "Pod without GameServer/Node should not enqueue"
        );

        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&gs).await.unwrap();
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        assert!(
            route_migration_controller_event(
                storage.clone(),
                MigrationControllerWatchEvent::PodModified(pod.clone()),
            )
            .await
            .unwrap()
            .is_none(),
            "Pod with GameServer but missing Node should not enqueue"
        );

        storage
            .create(&build_key("nodes", None, "node-a"), &node("node-a", "203.0.113.10"))
            .await
            .unwrap();
        storage
            .create(&build_key("nodes", None, "node-b"), &node("node-b", "198.51.100.20"))
            .await
            .unwrap();

        assert!(
            route_migration_controller_event(
                storage.clone(),
                MigrationControllerWatchEvent::PodModified(pod.clone()),
            )
            .await
            .unwrap()
            .is_none(),
            "matching Node and address should not enqueue"
        );

        pod.spec.as_mut().unwrap().node_name = Some("node-b".into());
        let item = route_migration_controller_event(
            storage.clone(),
            MigrationControllerWatchEvent::PodModified(pod.clone()),
        )
        .await
        .unwrap()
        .expect("migrated Pod should enqueue");
        assert_eq!(item.key(), "default/server");

        let mut deleting = pod.clone();
        deleting.metadata.deletion_timestamp = Some(chrono::Utc::now());
        assert!(
            route_migration_controller_event(
                storage.clone(),
                MigrationControllerWatchEvent::PodModified(deleting),
            )
            .await
            .unwrap()
            .is_none()
        );

        let mut foreign = pod_for(&created, "node-b");
        foreign.metadata.name = "foreign".into();
        foreign
            .metadata
            .labels
            .as_mut()
            .unwrap()
            .remove(crate::ROLE_LABEL);
        assert!(
            route_migration_controller_event(
                storage,
                MigrationControllerWatchEvent::PodAdded(foreign),
            )
            .await
            .unwrap()
            .is_none()
        );
    }

    #[tokio::test]
    async fn runtime_replays_initial_migrated_pod_and_marks_allocated_unhealthy() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());

        let mut gs = game_server(GameServerState::Allocated);
        gs.status.node_name = "node-a".into();
        gs.status.address = "203.0.113.10".into();
        let created = repository.create(&gs).await.unwrap();

        let mut pod = pod_for(&created, "node-b");
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();
        storage
            .create(
                &build_key("pods", Some("default"), "server"),
                &pod,
            )
            .await
            .unwrap();
        storage
            .create(
                &build_key("nodes", None, "node-b"),
                &node("node-b", "198.51.100.20"),
            )
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let task = tokio::spawn(async move {
            controller
                .run_migration_game_server_controller(1)
                .await
        });

        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            loop {
                let current: GameServer =
                    repository.get("default", "server").await.unwrap();
                if current.status.state == Some(GameServerState::Unhealthy) {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("MigrationController did not process initial Pod Add");

        task.abort();
    }

    #[tokio::test]
    async fn live_pod_node_change_marks_allocated_gameserver_unhealthy() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());

        let mut gs = game_server(GameServerState::Allocated);
        gs.status.node_name = "node-a".into();
        gs.status.address = "203.0.113.10".into();
        let created = repository.create(&gs).await.unwrap();

        let mut pod = pod_for(&created, "node-a");
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();
        let pod_key = build_key("pods", Some("default"), "server");
        storage.create(&pod_key, &pod).await.unwrap();
        storage
            .create(
                &build_key("nodes", None, "node-a"),
                &node("node-a", "203.0.113.10"),
            )
            .await
            .unwrap();
        storage
            .create(
                &build_key("nodes", None, "node-b"),
                &node("node-b", "198.51.100.20"),
            )
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let task = tokio::spawn(async move {
            controller
                .run_migration_game_server_controller(1)
                .await
        });

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        pod.spec.as_mut().unwrap().node_name = Some("node-b".into());
        storage.update(&pod_key, &pod).await.unwrap();

        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            loop {
                let current: GameServer =
                    repository.get("default", "server").await.unwrap();
                if current.status.state == Some(GameServerState::Unhealthy) {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("MigrationController did not process live Pod migration");

        task.abort();
    }

    #[tokio::test]
    async fn zero_workers_is_rejected() {
        let controller =
            CreatingGameServerController::new(MemoryStorage::new());
        let error = controller
            .run_migration_game_server_controller(0)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("at least one worker"));
    }
}
