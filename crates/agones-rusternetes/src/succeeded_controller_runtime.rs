use std::sync::Arc;

use agones_core::GameServer;
use cgp::prelude::*;
use futures::stream::{BoxStream, FuturesUnordered};
use futures::{StreamExt, stream};
use rusternetes_common::resources::Pod;
use rusternetes_storage::{Storage, WatchEvent, build_prefix};

use crate::controller_work_queue::{
    ControllerWorkItem, ControllerWorkQueue, DefaultControllerRetryPolicy,
};
use crate::{
    AdapterError, AgonesRepository, AgonesWatchEvent,
    ReconcileSucceededGameServer, is_game_server_pod, pod_completed,
    should_enqueue_succeeded_game_server,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SucceededControllerWorkItem {
    pub namespace: String,
    pub name: String,
}

impl SucceededControllerWorkItem {
    pub fn key(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }
}

impl ControllerWorkItem for SucceededControllerWorkItem {
    fn work_key(&self) -> String {
        self.key()
    }
}

#[derive(Clone, Debug)]
pub enum SucceededControllerWatchEvent {
    PodAdded(Pod),
    PodModified(Pod),
    GameServerModified(GameServer),
}

pub type SucceededControllerWatchStream =
    BoxStream<'static, Result<SucceededControllerWatchEvent, AdapterError>>;

fn work_item_from_pod(pod: &Pod) -> Option<SucceededControllerWorkItem> {
    let namespace = pod
        .metadata
        .namespace
        .as_deref()
        .filter(|namespace| !namespace.is_empty())?;
    if pod.metadata.name.is_empty() {
        return None;
    }

    Some(SucceededControllerWorkItem {
        namespace: namespace.to_owned(),
        name: pod.metadata.name.clone(),
    })
}

fn work_item_from_game_server(
    game_server: &GameServer,
) -> Option<SucceededControllerWorkItem> {
    if game_server.metadata.namespace.is_empty()
        || game_server.metadata.name.is_empty()
    {
        return None;
    }

    Some(SucceededControllerWorkItem {
        namespace: game_server.metadata.namespace.clone(),
        name: game_server.metadata.name.clone(),
    })
}

/// Subscribes to the two informer sources used by upstream SucceededController.
///
/// Pod Add/Update and GameServer Update are retained. Pod Delete, GameServer
/// Add, and GameServer Delete have no upstream handler and are filtered out.
pub async fn watch_succeeded_controller_events<S>(
    storage: Arc<S>,
) -> Result<SucceededControllerWatchStream, AdapterError>
where
    S: Storage + 'static,
{
    let pod_stream = storage
        .watch(&build_prefix("pods", None))
        .await?
        .filter_map(|event| async move {
            match event {
                Ok(WatchEvent::Added(_, raw)) => Some(
                    serde_json::from_str::<Pod>(&raw)
                        .map(SucceededControllerWatchEvent::PodAdded)
                        .map_err(AdapterError::from),
                ),
                Ok(WatchEvent::Modified(_, raw)) => Some(
                    serde_json::from_str::<Pod>(&raw)
                        .map(SucceededControllerWatchEvent::PodModified)
                        .map_err(AdapterError::from),
                ),
                Ok(WatchEvent::Deleted(_, _)) => None,
                Err(error) => Some(Err(AdapterError::from(error))),
            }
        });

    let game_server_stream = AgonesRepository::new(storage)
        .watch_all::<GameServer>()
        .await?
        .filter_map(|event| async move {
            match event {
                Ok(AgonesWatchEvent::Modified { resource, .. }) => Some(Ok(
                    SucceededControllerWatchEvent::GameServerModified(resource),
                )),
                Ok(
                    AgonesWatchEvent::Added { .. }
                    | AgonesWatchEvent::Deleted { .. },
                ) => None,
                Err(error) => Some(Err(error)),
            }
        });

    Ok(Box::pin(stream::select(pod_stream, game_server_stream)))
}

/// Applies the informer-side enqueue predicates from upstream succeeded.go.
///
/// Pod Add and Update use the same completion predicate. GameServer Update is
/// the recovery/resync path and re-reads the Pod from Rusternetes before
/// deciding whether to enqueue.
pub async fn route_succeeded_controller_event<S>(
    storage: &S,
    event: SucceededControllerWatchEvent,
) -> Result<Option<SucceededControllerWorkItem>, AdapterError>
where
    S: Storage,
{
    match event {
        SucceededControllerWatchEvent::PodAdded(pod)
        | SucceededControllerWatchEvent::PodModified(pod) => {
            if is_game_server_pod(&pod) && pod_completed(&pod) {
                Ok(work_item_from_pod(&pod))
            } else {
                Ok(None)
            }
        }
        SucceededControllerWatchEvent::GameServerModified(game_server) => {
            if should_enqueue_succeeded_game_server(storage, &game_server).await {
                Ok(work_item_from_game_server(&game_server))
            } else {
                Ok(None)
            }
        }
    }
}

/// Key-based equivalent of upstream SucceededController.syncGameServer.
#[cgp_fn]
#[async_trait]
pub async fn sync_succeeded_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    namespace: &str,
    name: &str,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    Self: ReconcileSucceededGameServer<S> + Sync,
{
    let repository = AgonesRepository::new(storage);
    let game_server = match repository.get::<GameServer>(namespace, name).await {
        Ok(game_server) => game_server,
        Err(AdapterError::Rusternetes(
            rusternetes_common::Error::NotFound(_),
        )) => return Ok(()),
        Err(error) => return Err(error),
    };

    let _ = self
        .reconcile_succeeded_game_server(game_server)
        .await?;
    Ok(())
}

async fn run_succeeded_worker<C, S>(
    controller: &C,
    queue: ControllerWorkQueue<
        SucceededControllerWorkItem,
        DefaultControllerRetryPolicy,
    >,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    C: SyncSucceededGameServer<S> + Sync,
{
    while let Some(item) = queue.next().await {
        match controller
            .sync_succeeded_game_server(
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

/// Long-running Rusternetes equivalent of upstream SucceededController.Run.
///
/// Watches are subscribed before the initial Pod LIST. Existing completed Pods
/// are replayed through the Pod Add predicate, matching SharedInformer startup
/// AddFunc behavior. GameServer startup objects are not enqueued because the
/// upstream GameServer informer registers only UpdateFunc.
#[cgp_fn]
#[async_trait]
pub async fn run_succeeded_game_server_controller<S>(
    &self,
    #[implicit] storage: Arc<S>,
    workers: usize,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    Self: SyncSucceededGameServer<S> + Sync,
{
    if workers == 0 {
        return Err(AdapterError::Hook(
            "SucceededController requires at least one worker".into(),
        ));
    }

    let mut events =
        watch_succeeded_controller_events(storage.clone()).await?;

    let queue = ControllerWorkQueue::new(
        DefaultControllerRetryPolicy::default(),
    );

    // SharedInformer invokes AddFunc for objects from its initial LIST.
    let pods: Vec<Pod> =
        storage.list(&build_prefix("pods", None)).await?;
    for pod in pods {
        if let Some(item) = route_succeeded_controller_event(
            storage.as_ref(),
            SucceededControllerWatchEvent::PodAdded(pod),
        )
        .await?
        {
            queue.enqueue(item);
        }
    }

    let mut worker_futures = FuturesUnordered::new();
    for _ in 0..workers {
        worker_futures.push(run_succeeded_worker::<Self, S>(
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
                if let Some(item) = route_succeeded_controller_event(
                    storage.as_ref(),
                    event?,
                ).await? {
                    queue.enqueue(item);
                }
            }
            worker = worker_futures.next() => {
                match worker {
                    Some(Ok(())) => {
                        return Err(AdapterError::Hook(
                            "SucceededController worker queue terminated unexpectedly".into(),
                        ));
                    }
                    Some(Err(error)) => return Err(error),
                    None => {
                        return Err(AdapterError::Hook(
                            "SucceededController has no active workers".into(),
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
        Container as CoreContainer, GameServerSpec, GameServerState,
        GameServerStatus, ObjectMeta, PodSpec as CorePodSpec,
        PodTemplateSpec, DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_common::resources::{PodStatus, Pod};
    use rusternetes_common::types::Phase;
    use rusternetes_storage::{MemoryStorage, build_key};

    use crate::{
        CreatingGameServerController, NoopApiHooks, PodBuildFeatures,
        build_pod,
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

    fn owned_pod(game_server: &GameServer) -> Pod {
        build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap()
    }

    async fn store_pod(storage: &MemoryStorage, pod: &Pod) {
        storage
            .create(
                &build_key(
                    "pods",
                    pod.metadata.namespace.as_deref(),
                    pod.metadata.name.as_str(),
                ),
                pod,
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn routing_matches_pod_handlers_and_gameserver_recovery_path() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();

        let mut pod = owned_pod(&created);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Running),
            ..Default::default()
        });
        store_pod(&storage, &pod).await;

        assert!(
            route_succeeded_controller_event(
                &storage,
                SucceededControllerWatchEvent::PodAdded(pod.clone()),
            )
            .await
            .unwrap()
            .is_none()
        );

        pod.status.as_mut().unwrap().phase = Some(Phase::Succeeded);
        storage
            .update(
                &build_key("pods", Some("default"), "server"),
                &pod,
            )
            .await
            .unwrap();

        let pod_item = route_succeeded_controller_event(
            &storage,
            SucceededControllerWatchEvent::PodModified(pod.clone()),
        )
        .await
        .unwrap()
        .expect("completed Pod update should enqueue");
        assert_eq!(pod_item.key(), "default/server");

        let gs_item = route_succeeded_controller_event(
            &storage,
            SucceededControllerWatchEvent::GameServerModified(
                created.clone(),
            ),
        )
        .await
        .unwrap()
        .expect("Ready GameServer resync should recover completed Pod");
        assert_eq!(gs_item.key(), "default/server");

        let mut starting = created.clone();
        starting.status.state = Some(GameServerState::Starting);
        assert!(
            route_succeeded_controller_event(
                &storage,
                SucceededControllerWatchEvent::GameServerModified(starting),
            )
            .await
            .unwrap()
            .is_none()
        );

        let mut dev = created;
        dev.metadata.annotations.insert(
            DEV_ADDRESS_ANNOTATION.into(),
            "198.51.100.10".into(),
        );
        assert!(
            route_succeeded_controller_event(
                &storage,
                SucceededControllerWatchEvent::GameServerModified(dev),
            )
            .await
            .unwrap()
            .is_none()
        );
    }

    #[tokio::test]
    async fn runtime_replays_initial_completed_pod_and_moves_to_shutdown() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();

        let mut pod = owned_pod(&created);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Succeeded),
            ..Default::default()
        });
        store_pod(&storage, &pod).await;

        let controller = CreatingGameServerController::new(storage.clone());
        let task = tokio::spawn(async move {
            controller
                .run_succeeded_game_server_controller(1)
                .await
        });

        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            loop {
                let current: GameServer =
                    repository.get("default", "server").await.unwrap();
                if current.status.state == Some(GameServerState::Shutdown) {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("SucceededController did not process initial Pod Add");

        task.abort();
    }

    #[tokio::test]
    async fn live_completed_pod_update_moves_to_shutdown() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Allocated))
            .await
            .unwrap();

        let mut pod = owned_pod(&created);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Running),
            ..Default::default()
        });
        let pod_key = build_key("pods", Some("default"), "server");
        storage.create(&pod_key, &pod).await.unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let task = tokio::spawn(async move {
            controller
                .run_succeeded_game_server_controller(1)
                .await
        });

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        pod.status.as_mut().unwrap().phase = Some(Phase::Succeeded);
        storage.update(&pod_key, &pod).await.unwrap();

        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            loop {
                let current: GameServer =
                    repository.get("default", "server").await.unwrap();
                if current.status.state == Some(GameServerState::Shutdown) {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("SucceededController did not process completed Pod update");

        task.abort();
    }

    #[tokio::test]
    async fn zero_workers_is_rejected() {
        let controller =
            CreatingGameServerController::new(MemoryStorage::new());
        let error = controller
            .run_succeeded_game_server_controller(0)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("at least one worker"));
    }
}
