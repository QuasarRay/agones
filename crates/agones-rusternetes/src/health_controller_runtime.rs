use std::sync::Arc;

use agones_core::GameServer;
use cgp::prelude::*;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use rusternetes_common::resources::Pod;
use rusternetes_storage::{Storage, WatchEvent, build_prefix};

use crate::controller_work_queue::{
    ControllerWorkItem, ControllerWorkQueue, DefaultControllerRetryPolicy,
};
use crate::{
    AdapterError, AgonesRepository, PodBuildFeatures,
    ReconcileHealthGameServer, is_game_server_pod,
    should_enqueue_unhealthy_pod,
};

pub type HealthControllerWatchStream =
    futures::stream::BoxStream<'static, Result<WatchEvent, AdapterError>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HealthControllerWorkItem {
    pub namespace: String,
    pub name: String,
}

impl HealthControllerWorkItem {
    pub fn key(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }
}

impl ControllerWorkItem for HealthControllerWorkItem {
    fn work_key(&self) -> String {
        self.key()
    }
}

/// Cluster-wide Pod stream used by upstream HealthController.
///
/// HealthController has no AddFunc: only Pod Update and Delete events can
/// enqueue work. Keeping the raw WatchEvent preserves that distinction.
pub async fn watch_health_controller_events<S>(
    storage: Arc<S>,
) -> Result<HealthControllerWatchStream, AdapterError>
where
    S: Storage + 'static,
{
    let stream = storage.watch(&build_prefix("pods", None)).await?;
    Ok(Box::pin(stream.map(|event| event.map_err(AdapterError::from))))
}

fn work_item_from_pod(pod: &Pod) -> Option<HealthControllerWorkItem> {
    let namespace = pod
        .metadata
        .namespace
        .as_deref()
        .filter(|namespace| !namespace.is_empty())?;
    if pod.metadata.name.is_empty() {
        return None;
    }

    Some(HealthControllerWorkItem {
        namespace: namespace.to_owned(),
        name: pod.metadata.name.clone(),
    })
}

/// Mirrors HealthController's Pod informer handlers.
///
/// Added Pods are ignored. Modified Pods enqueue only when they are
/// non-deleting GameServer Pods that are currently unhealthy. Deleted
/// GameServer Pods always enqueue so a missing Pod can drive the GameServer to
/// Unhealthy.
pub fn route_health_controller_event(
    event: WatchEvent,
    features: PodBuildFeatures,
    wait_on_free_ports: bool,
) -> Result<Option<HealthControllerWorkItem>, AdapterError> {
    match event {
        WatchEvent::Added(_, _) => Ok(None),
        WatchEvent::Modified(_, raw) => {
            let pod: Pod = serde_json::from_str(&raw)?;
            if should_enqueue_unhealthy_pod(
                &pod,
                features.sidecar_containers,
                wait_on_free_ports,
            ) {
                Ok(work_item_from_pod(&pod))
            } else {
                Ok(None)
            }
        }
        WatchEvent::Deleted(_, raw) => {
            let pod: Pod = serde_json::from_str(&raw)?;
            if is_game_server_pod(&pod) {
                Ok(work_item_from_pod(&pod))
            } else {
                Ok(None)
            }
        }
    }
}

/// Key-based equivalent of upstream HealthController.syncGameServer.
#[cgp_fn]
#[async_trait]
pub async fn sync_health_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    namespace: &str,
    name: &str,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    Self: ReconcileHealthGameServer<S> + Sync,
{
    let repository = AgonesRepository::new(storage);
    let game_server = match repository.get::<GameServer>(namespace, name).await {
        Ok(game_server) => game_server,
        Err(AdapterError::Rusternetes(
            rusternetes_common::Error::NotFound(_),
        )) => return Ok(()),
        Err(error) => return Err(error),
    };

    let _ = self.reconcile_health_game_server(game_server).await?;
    Ok(())
}

async fn run_health_worker<C, S>(
    controller: &C,
    queue: ControllerWorkQueue<
        HealthControllerWorkItem,
        DefaultControllerRetryPolicy,
    >,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    C: SyncHealthGameServer<S> + Sync,
{
    while let Some(item) = queue.next().await {
        match controller
            .sync_health_game_server(
                item.namespace.as_str(),
                item.name.as_str(),
            )
            .await
        {
            Ok(()) => queue.complete_success(&item),
            Err(_) => {
                // Upstream WorkerQueue retries handler failures with
                // DefaultTypedControllerRateLimiter.
                let delay = queue.record_failure(&item);
                tokio::time::sleep(delay).await;
                queue.complete_retry(&item);
            }
        }
    }

    Ok(())
}

/// Long-running Rusternetes equivalent of upstream HealthController.Run.
///
/// Pod informer events are subscribed directly from Rusternetes. Update/Delete
/// predicates are preserved, event enqueue is immediate, and reconciliation
/// failures are retried through the shared controller queue abstraction.
#[cgp_fn]
#[async_trait]
pub async fn run_health_game_server_controller<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] pod_build_features: PodBuildFeatures,
    #[implicit] wait_on_free_ports: bool,
    workers: usize,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    Self: SyncHealthGameServer<S> + Sync,
{
    if workers == 0 {
        return Err(AdapterError::Hook(
            "HealthController requires at least one worker".into(),
        ));
    }

    let mut events = watch_health_controller_events(storage).await?;
    let queue = ControllerWorkQueue::new(
        DefaultControllerRetryPolicy::default(),
    );

    let mut worker_futures = FuturesUnordered::new();
    for _ in 0..workers {
        worker_futures.push(run_health_worker::<Self, S>(
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
                if let Some(item) = route_health_controller_event(
                    event?,
                    pod_build_features,
                    wait_on_free_ports,
                )? {
                    queue.enqueue(item);
                }
            }
            worker = worker_futures.next() => {
                match worker {
                    Some(Ok(())) => {
                        return Err(AdapterError::Hook(
                            "HealthController worker queue terminated unexpectedly".into(),
                        ));
                    }
                    Some(Err(error)) => return Err(error),
                    None => {
                        return Err(AdapterError::Hook(
                            "HealthController has no active workers".into(),
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
        PodTemplateSpec,
    };
    use rusternetes_common::resources::{PodStatus, Pod};
    use rusternetes_storage::{MemoryStorage, build_key};

    use crate::{
        CreatingGameServerController, NoopApiHooks, build_pod,
    };

    use super::*;

    fn game_server() -> GameServer {
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
                state: Some(GameServerState::Ready),
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

    #[test]
    fn routing_preserves_health_informer_update_delete_semantics() {
        let fixture = game_server();
        let mut pod = owned_pod(&fixture);
        pod.metadata.namespace = Some("default".into());

        let raw = serde_json::to_string(&pod).unwrap();
        assert!(
            route_health_controller_event(
                WatchEvent::Added("key".into(), raw),
                PodBuildFeatures::default(),
                false,
            )
            .unwrap()
            .is_none()
        );

        pod.status = Some(PodStatus {
            reason: Some("Evicted".into()),
            ..Default::default()
        });
        let raw = serde_json::to_string(&pod).unwrap();
        let item = route_health_controller_event(
            WatchEvent::Modified("key".into(), raw),
            PodBuildFeatures::default(),
            false,
        )
        .unwrap()
        .expect("unhealthy update should enqueue");
        assert_eq!(item.key(), "default/server");

        let raw = serde_json::to_string(&pod).unwrap();
        assert!(
            route_health_controller_event(
                WatchEvent::Deleted("key".into(), raw),
                PodBuildFeatures::default(),
                false,
            )
            .unwrap()
            .is_some()
        );
    }

    #[tokio::test]
    async fn live_runtime_moves_updated_unhealthy_pod_to_unhealthy() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&game_server()).await.unwrap();
        let pod = owned_pod(&created);
        let pod_key = build_key("pods", Some("default"), "server");
        storage.create(&pod_key, &pod).await.unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let task = tokio::spawn(async move {
            controller
                .run_health_game_server_controller(1)
                .await
        });

        // Allow the controller to subscribe before emitting the update.
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let mut unhealthy = pod;
        unhealthy.status = Some(PodStatus {
            reason: Some("Evicted".into()),
            ..Default::default()
        });
        storage.update(&pod_key, &unhealthy).await.unwrap();

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
        .expect("HealthController did not process unhealthy Pod update");

        task.abort();
    }

    #[tokio::test]
    async fn live_runtime_marks_gameserver_unhealthy_after_pod_delete() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&game_server()).await.unwrap();
        let pod = owned_pod(&created);
        let pod_key = build_key("pods", Some("default"), "server");
        storage.create(&pod_key, &pod).await.unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let task = tokio::spawn(async move {
            controller
                .run_health_game_server_controller(1)
                .await
        });

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        storage.delete(&pod_key).await.unwrap();

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
        .expect("HealthController did not process deleted Pod");

        task.abort();
    }

    #[tokio::test]
    async fn zero_workers_is_rejected() {
        let controller =
            CreatingGameServerController::new(MemoryStorage::new());
        let error = controller
            .run_health_game_server_controller(0)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("at least one worker"));
    }
}
