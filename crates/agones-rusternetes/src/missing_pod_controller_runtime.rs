use std::sync::Arc;

use agones_core::GameServer;
use cgp::prelude::*;
use futures::stream::{BoxStream, FuturesUnordered};
use futures::StreamExt;
use rusternetes_storage::Storage;

use crate::controller_work_queue::{
    ControllerWorkItem, ControllerWorkQueue, DefaultControllerRetryPolicy,
};
use crate::{
    AdapterError, AgonesRepository, AgonesWatchEvent,
    ReconcileMissingPodGameServer, should_enqueue_missing_pod_game_server,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MissingPodControllerWorkItem {
    pub namespace: String,
    pub name: String,
}

impl MissingPodControllerWorkItem {
    pub fn key(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }
}

impl ControllerWorkItem for MissingPodControllerWorkItem {
    fn work_key(&self) -> String {
        self.key()
    }
}

pub type MissingPodControllerWatchStream =
    BoxStream<'static, Result<GameServer, AdapterError>>;

fn work_item_from_game_server(
    game_server: &GameServer,
) -> Option<MissingPodControllerWorkItem> {
    if game_server.metadata.namespace.is_empty()
        || game_server.metadata.name.is_empty()
    {
        return None;
    }

    Some(MissingPodControllerWorkItem {
        namespace: game_server.metadata.namespace.clone(),
        name: game_server.metadata.name.clone(),
    })
}

/// Subscribes to the GameServer informer source used by upstream
/// MissingPodController.
///
/// Upstream registers only UpdateFunc. Initial Add events and deletes therefore
/// do not enqueue this controller and are filtered here as well.
pub async fn watch_missing_pod_controller_events<S>(
    storage: Arc<S>,
) -> Result<MissingPodControllerWatchStream, AdapterError>
where
    S: Storage + 'static,
{
    let stream = AgonesRepository::new(storage)
        .watch_all::<GameServer>()
        .await?
        .filter_map(|event| async move {
            match event {
                Ok(AgonesWatchEvent::Modified { resource, .. }) => {
                    Some(Ok(resource))
                }
                Ok(
                    AgonesWatchEvent::Added { .. }
                    | AgonesWatchEvent::Deleted { .. },
                ) => None,
                Err(error) => Some(Err(error)),
            }
        });

    Ok(Box::pin(stream))
}

/// Applies the upstream MissingPodController UpdateFunc enqueue predicate.
///
/// The live GameServer object is checked against Rusternetes storage for the
/// matching Pod. Missing, foreign, failed, or storage-error Pod lookups enqueue
/// work; dev, pre-Pod, deleting, Unhealthy, and Error GameServers do not.
pub async fn route_missing_pod_controller_event<S>(
    storage: &S,
    game_server: GameServer,
) -> Option<MissingPodControllerWorkItem>
where
    S: Storage,
{
    should_enqueue_missing_pod_game_server(storage, &game_server)
        .await
        .then(|| work_item_from_game_server(&game_server))
        .flatten()
}

/// Key-based equivalent of upstream MissingPodController.syncGameServer.
#[cgp_fn]
#[async_trait]
pub async fn sync_missing_pod_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    namespace: &str,
    name: &str,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    Self: ReconcileMissingPodGameServer<S> + Sync,
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
        .reconcile_missing_pod_game_server(game_server)
        .await?;
    Ok(())
}

async fn run_missing_pod_worker<C, S>(
    controller: &C,
    queue: ControllerWorkQueue<
        MissingPodControllerWorkItem,
        DefaultControllerRetryPolicy,
    >,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    C: SyncMissingPodGameServer<S> + Sync,
{
    while let Some(item) = queue.next().await {
        match controller
            .sync_missing_pod_game_server(
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

/// Long-running Rusternetes equivalent of upstream MissingPodController.Run.
///
/// Unlike controllers with AddFunc handlers, this runtime does not replay an
/// initial GameServer snapshot. Only subsequent GameServer Modify events are
/// routed, matching the upstream UpdateFunc-only registration. Reconciliation
/// errors remain in the default controller retry path.
#[cgp_fn]
#[async_trait]
pub async fn run_missing_pod_game_server_controller<S>(
    &self,
    #[implicit] storage: Arc<S>,
    workers: usize,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    Self: SyncMissingPodGameServer<S> + Sync,
{
    if workers == 0 {
        return Err(AdapterError::Hook(
            "MissingPodController requires at least one worker".into(),
        ));
    }

    let mut events =
        watch_missing_pod_controller_events(storage.clone()).await?;
    let queue = ControllerWorkQueue::new(
        DefaultControllerRetryPolicy::default(),
    );

    let mut worker_futures = FuturesUnordered::new();
    for _ in 0..workers {
        worker_futures.push(run_missing_pod_worker::<Self, S>(
            self,
            queue.clone(),
        ));
    }

    loop {
        tokio::select! {
            event = events.next() => {
                let Some(game_server) = event else {
                    return Ok(());
                };
                if let Some(item) = route_missing_pod_controller_event(
                    storage.as_ref(),
                    game_server?,
                ).await {
                    queue.enqueue(item);
                }
            }
            worker = worker_futures.next() => {
                match worker {
                    Some(Ok(())) => {
                        return Err(AdapterError::Hook(
                            "MissingPodController worker queue terminated unexpectedly".into(),
                        ));
                    }
                    Some(Err(error)) => return Err(error),
                    None => {
                        return Err(AdapterError::Hook(
                            "MissingPodController has no active workers".into(),
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
    use rusternetes_common::resources::PodStatus;
    use rusternetes_common::types::Phase;
    use rusternetes_storage::{MemoryStorage, Storage, build_key};

    use crate::{
        CreatingGameServerController, NoopApiHooks, PodBuildFeatures,
        build_pod,
    };

    use super::*;

    fn game_server(state: GameServerState) -> GameServer {
        GameServer {
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
                        }],
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

    async fn store_pod(
        storage: &MemoryStorage,
        game_server: &GameServer,
        phase: Phase,
    ) {
        let mut pod = build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.status = Some(PodStatus {
            phase: Some(phase),
            ..Default::default()
        });
        storage
            .create(
                &build_key("pods", Some("default"), "server"),
                &pod,
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn routing_matches_upstream_gameserver_update_predicate() {
        let storage = MemoryStorage::new();

        for state in [
            GameServerState::PortAllocation,
            GameServerState::Creating,
            GameServerState::Starting,
        ] {
            assert!(
                route_missing_pod_controller_event(
                    &storage,
                    game_server(state),
                )
                .await
                .is_none()
            );
        }

        for state in [
            GameServerState::Ready,
            GameServerState::Allocated,
            GameServerState::Shutdown,
        ] {
            let item = route_missing_pod_controller_event(
                &storage,
                game_server(state),
            )
            .await
            .expect("missing Pod should enqueue eligible GameServer");
            assert_eq!(item.key(), "default/server");
        }

        for state in [
            GameServerState::Unhealthy,
            GameServerState::Error,
        ] {
            assert!(
                route_missing_pod_controller_event(
                    &storage,
                    game_server(state),
                )
                .await
                .is_none()
            );
        }

        let mut dev = game_server(GameServerState::Ready);
        dev.metadata.annotations.insert(
            DEV_ADDRESS_ANNOTATION.into(),
            "198.51.100.10".into(),
        );
        assert!(
            route_missing_pod_controller_event(&storage, dev)
                .await
                .is_none()
        );
    }

    #[tokio::test]
    async fn healthy_pod_suppresses_enqueue_and_failed_pod_enqueues() {
        let storage = MemoryStorage::new();
        let ready = game_server(GameServerState::Ready);
        store_pod(&storage, &ready, Phase::Running).await;

        assert!(
            route_missing_pod_controller_event(
                &storage,
                ready.clone(),
            )
            .await
            .is_none()
        );

        let pod_key = build_key("pods", Some("default"), "server");
        let mut pod: rusternetes_common::resources::Pod =
            storage.get(&pod_key).await.unwrap();
        pod.status.as_mut().unwrap().phase = Some(Phase::Failed);
        storage.update(&pod_key, &pod).await.unwrap();

        assert!(
            route_missing_pod_controller_event(&storage, ready)
                .await
                .is_some()
        );
    }

    #[tokio::test]
    async fn live_missing_pod_update_moves_gameserver_to_unhealthy() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let task = tokio::spawn(async move {
            controller
                .run_missing_pod_game_server_controller(1)
                .await
        });

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // Update the GameServer without changing state. Upstream's informer
        // UpdateFunc still evaluates the missing-Pod predicate.
        repository.update(&created).await.unwrap();

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
        .expect("MissingPodController did not process GameServer update");

        task.abort();
    }

    #[tokio::test]
    async fn initial_gameserver_add_does_not_run_update_only_controller() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());

        let controller = CreatingGameServerController::new(storage.clone());
        let task = tokio::spawn(async move {
            controller
                .run_missing_pod_game_server_controller(1)
                .await
        });

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
        let current: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(current.status.state, Some(GameServerState::Ready));

        task.abort();
    }

    #[tokio::test]
    async fn zero_workers_is_rejected() {
        let controller =
            CreatingGameServerController::new(MemoryStorage::new());
        let error = controller
            .run_missing_pod_game_server_controller(0)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("at least one worker"));
    }
}
