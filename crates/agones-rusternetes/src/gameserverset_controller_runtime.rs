use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use agones_core::{GameServer, GameServerSet};
use agones_macros::agones_invariant;
use cgp::prelude::*;
use futures::stream::{BoxStream, FuturesUnordered};
use futures::StreamExt;
use rusternetes_storage::Storage;

use crate::controller_work_queue::{
    ControllerWorkItem, ControllerWorkQueue, FastSlowRetryPolicy,
};
use crate::{
    AdapterError, AgonesRepository, AgonesWatchEvent,
    GameServerSetSyncOutcome, GameServerStateCache, SyncGameServerSet,
};

pub const GAME_SERVER_SET_FAST_RETRIES: u32 = 5;
pub const GAME_SERVER_SET_FAST_RETRY_DELAY_MS: u64 = 200;
pub const GAME_SERVER_SET_SLOW_RETRY_DELAY_MS: u64 = 3_000;

pub const fn game_server_set_retry_delay_ms(retries: u32) -> u64 {
    if retries < GAME_SERVER_SET_FAST_RETRIES {
        GAME_SERVER_SET_FAST_RETRY_DELAY_MS
    } else {
        GAME_SERVER_SET_SLOW_RETRY_DELAY_MS
    }
}

#[agones_invariant]
fn game_server_set_retry_policy_matches_upstream(retries: u8) -> bool {
    let retries = retries as u32;
    game_server_set_retry_delay_ms(retries)
        == if retries < GAME_SERVER_SET_FAST_RETRIES {
            GAME_SERVER_SET_FAST_RETRY_DELAY_MS
        } else {
            GAME_SERVER_SET_SLOW_RETRY_DELAY_MS
        }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameServerSetWorkItem {
    pub namespace: String,
    pub name: String,
}

impl GameServerSetWorkItem {
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    pub fn key(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }
}

impl ControllerWorkItem for GameServerSetWorkItem {
    fn work_key(&self) -> String {
        self.key()
    }
}

type GameServerSetWorkQueue =
    ControllerWorkQueue<GameServerSetWorkItem, FastSlowRetryPolicy>;

fn game_server_set_retry_policy() -> FastSlowRetryPolicy {
    FastSlowRetryPolicy {
        fast_retries: GAME_SERVER_SET_FAST_RETRIES,
        fast_delay: Duration::from_millis(
            GAME_SERVER_SET_FAST_RETRY_DELAY_MS,
        ),
        slow_delay: Duration::from_millis(
            GAME_SERVER_SET_SLOW_RETRY_DELAY_MS,
        ),
    }
}

#[derive(Clone, Debug)]
pub enum GameServerSetControllerWatchEvent {
    GameServerSet(AgonesWatchEvent<GameServerSet>),
    GameServer(AgonesWatchEvent<GameServer>),
}

pub type GameServerSetControllerWatchStream =
    BoxStream<'static, Result<GameServerSetControllerWatchEvent, AdapterError>>;

#[derive(Clone, Debug, Default)]
pub struct GameServerSetControllerWatchState {
    replicas: HashMap<String, i32>,
}

impl GameServerSetControllerWatchState {
    fn resource_key(namespace: &str, name: &str) -> String {
        format!("{namespace}/{name}")
    }

    fn replace_snapshot(&mut self, game_server_sets: &[GameServerSet]) {
        self.replicas.clear();
        for game_server_set in game_server_sets {
            self.replicas.insert(
                Self::resource_key(
                    game_server_set.metadata.namespace.as_str(),
                    game_server_set.metadata.name.as_str(),
                ),
                game_server_set.spec.replicas,
            );
        }
    }

    fn route_game_server_set(
        &mut self,
        event: AgonesWatchEvent<GameServerSet>,
    ) -> Option<GameServerSetWorkItem> {
        match event {
            AgonesWatchEvent::Added { resource, .. } => {
                let item = GameServerSetWorkItem::new(
                    resource.metadata.namespace.clone(),
                    resource.metadata.name.clone(),
                );
                self.replicas
                    .insert(item.key(), resource.spec.replicas);
                Some(item)
            }
            AgonesWatchEvent::Modified { resource, .. } => {
                let item = GameServerSetWorkItem::new(
                    resource.metadata.namespace.clone(),
                    resource.metadata.name.clone(),
                );
                let previous = self
                    .replicas
                    .insert(item.key(), resource.spec.replicas);
                (previous != Some(resource.spec.replicas)).then_some(item)
            }
            AgonesWatchEvent::Deleted { resource, .. } => {
                let key = Self::resource_key(
                    resource.metadata.namespace.as_str(),
                    resource.metadata.name.as_str(),
                );
                self.replicas.remove(key.as_str());
                None
            }
        }
    }
}

fn controller_owner_name(game_server: &GameServer) -> Option<&str> {
    game_server
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

async fn owner_game_server_set_work_item<S>(
    storage: Arc<S>,
    game_server: &GameServer,
) -> Result<Option<GameServerSetWorkItem>, AdapterError>
where
    S: Storage + 'static,
{
    let Some(owner_name) = controller_owner_name(game_server) else {
        return Ok(None);
    };

    let namespace = game_server.metadata.namespace.as_str();
    match AgonesRepository::new(storage)
        .get::<GameServerSet>(namespace, owner_name)
        .await
    {
        Ok(_) => Ok(Some(GameServerSetWorkItem::new(
            namespace,
            owner_name,
        ))),
        Err(AdapterError::Rusternetes(
            rusternetes_common::Error::NotFound(_),
        )) => Ok(None),
        Err(error) => Err(error),
    }
}

pub async fn watch_game_server_set_controller_events<S>(
    storage: Arc<S>,
) -> Result<GameServerSetControllerWatchStream, AdapterError>
where
    S: Storage + 'static,
{
    let repository = AgonesRepository::new(storage);
    let game_server_sets = repository
        .watch_all::<GameServerSet>()
        .await?
        .map(|event| {
            event.map(GameServerSetControllerWatchEvent::GameServerSet)
        });
    let game_servers = repository
        .watch_all::<GameServer>()
        .await?
        .map(|event| {
            event.map(GameServerSetControllerWatchEvent::GameServer)
        });

    Ok(Box::pin(futures::stream::select(
        game_server_sets,
        game_servers,
    )))
}

/// Rebuilds the informer-equivalent GameServerSet observation snapshot.
///
/// SharedInformer delivers AddFunc for every existing GameServerSet during
/// startup, so every object in the initial LIST becomes work. Child GameServer
/// startup AddFuncs are redundant with those set AddFuncs after cache sync and
/// the queue deduplicates the same owner key.
#[cgp_fn]
#[async_trait]
pub async fn rebuild_game_server_set_controller_watch_state<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit]
    game_server_set_watch_state: Arc<RwLock<GameServerSetControllerWatchState>>,
) -> Result<Vec<GameServerSetWorkItem>, AdapterError>
where
    S: Storage + 'static,
{
    let game_server_sets = AgonesRepository::new(storage)
        .list_all::<GameServerSet>()
        .await?;

    game_server_set_watch_state
        .write()
        .map_err(|_| {
            AdapterError::Hook(
                "GameServerSet watch-state lock poisoned".into(),
            )
        })?
        .replace_snapshot(&game_server_sets);

    Ok(game_server_sets
        .into_iter()
        .map(|game_server_set| {
            GameServerSetWorkItem::new(
                game_server_set.metadata.namespace,
                game_server_set.metadata.name,
            )
        })
        .collect())
}

/// Routes the exact upstream informer handlers onto one GameServerSet key.
///
/// GameServerSet Add always enqueues; Update enqueues only on replica changes;
/// Delete only drops its local state-cache entry. GameServer Add/Delete enqueue
/// the current controller owner immediately, while a Modified GameServer is
/// ignored once deletionTimestamp is present.
#[cgp_fn]
#[async_trait]
pub async fn route_game_server_set_controller_event<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit]
    game_server_set_watch_state: Arc<RwLock<GameServerSetControllerWatchState>>,
    #[implicit] game_server_state_cache: Arc<GameServerStateCache>,
    event: GameServerSetControllerWatchEvent,
) -> Result<Option<GameServerSetWorkItem>, AdapterError>
where
    S: Storage + 'static,
{
    match event {
        GameServerSetControllerWatchEvent::GameServerSet(event) => {
            if let AgonesWatchEvent::Deleted { resource, .. } = &event {
                game_server_state_cache.delete_game_server_set(resource);
            }

            game_server_set_watch_state
                .write()
                .map_err(|_| {
                    AdapterError::Hook(
                        "GameServerSet watch-state lock poisoned".into(),
                    )
                })?
                .route_game_server_set(event)
                .pipe(Ok)
        }
        GameServerSetControllerWatchEvent::GameServer(event) => {
            match event {
                AgonesWatchEvent::Added { resource, .. }
                | AgonesWatchEvent::Deleted { resource, .. } => {
                    owner_game_server_set_work_item(storage, &resource).await
                }
                AgonesWatchEvent::Modified { resource, .. } => {
                    if resource.metadata.deletion_timestamp.is_some() {
                        Ok(None)
                    } else {
                        owner_game_server_set_work_item(
                            storage,
                            &resource,
                        )
                        .await
                    }
                }
            }
        }
    }
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}
impl<T> Pipe for T {}

async fn run_game_server_set_worker<C, S>(
    controller: &C,
    queue: GameServerSetWorkQueue,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    C: SyncGameServerSet<S> + Sync,
{
    while let Some(item) = queue.next().await {
        match controller
            .sync_game_server_set(
                item.namespace.as_str(),
                item.name.as_str(),
            )
            .await
        {
            Ok(GameServerSetSyncOutcome::Reconciled(result)) => {
                if result.is_partial() {
                    // Enqueue while Processing marks this key dirty. The
                    // success completion below then emits exactly one
                    // immediate follow-up, matching EnqueueImmediately.
                    queue.enqueue(item.clone());
                }
                queue.complete_success(&item);
            }
            Ok(GameServerSetSyncOutcome::NotFound) => {
                queue.complete_success(&item);
            }
            Err(_) => {
                let delay = queue.record_failure(&item);
                tokio::time::sleep(delay).await;
                queue.complete_retry(&item);
            }
        }
    }

    Ok(())
}

/// Long-running Rusternetes equivalent of GameServerSet Controller.Run,
/// excluding AllocationOverflowController which remains its own stackable
/// controller milestone.
///
/// Watches are subscribed before the initial LIST. The worker retry schedule
/// uses Agones FastRateLimiter(3s): five 200ms retries, then constant 3s.
#[cgp_fn]
#[async_trait]
pub async fn run_game_server_set_controller<S>(
    &self,
    #[implicit] storage: Arc<S>,
    workers: usize,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    Self: RebuildGameServerSetControllerWatchState<S>
        + RouteGameServerSetControllerEvent<S>
        + SyncGameServerSet<S>
        + Sync,
{
    if workers == 0 {
        return Err(AdapterError::Hook(
            "GameServerSet controller requires at least one worker".into(),
        ));
    }

    let mut events =
        watch_game_server_set_controller_events(storage).await?;
    let initial_work = self
        .rebuild_game_server_set_controller_watch_state()
        .await?;

    let queue = GameServerSetWorkQueue::new(
        game_server_set_retry_policy(),
    );
    for item in initial_work {
        queue.enqueue(item);
    }

    let mut worker_futures = FuturesUnordered::new();
    for _ in 0..workers {
        worker_futures.push(run_game_server_set_worker::<Self, S>(
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
                if let Some(item) = self
                    .route_game_server_set_controller_event(event?)
                    .await?
                {
                    queue.enqueue(item);
                }
            }
            worker = worker_futures.next() => {
                match worker {
                    Some(Ok(())) => {
                        return Err(AdapterError::Hook(
                            "GameServerSet worker queue terminated unexpectedly".into(),
                        ));
                    }
                    Some(Err(error)) => return Err(error),
                    None => {
                        return Err(AdapterError::Hook(
                            "GameServerSet controller has no active workers".into(),
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
        Container, GameServerPort, GameServerSetSpec,
        GameServerTemplateSpec, ObjectMeta, PodSpec, PodTemplateSpec,
        PortPolicy, Protocol, SchedulingStrategy,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::{
        GameServerSetController, GameServerSetControllerConfig,
    };

    use super::*;

    fn set(replicas: i32) -> GameServerSet {
        GameServerSet {
            metadata: ObjectMeta {
                name: "set".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                replicas,
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
            ..Default::default()
        }
    }

    fn config(batch: i32) -> GameServerSetControllerConfig {
        GameServerSetControllerConfig {
            max_creation_parallelism: 2,
            max_deletion_parallelism: 2,
            max_game_server_creations_per_batch: batch,
            max_game_server_deletions_per_batch: batch,
            max_pod_pending_count: 100,
            player_tracking: false,
        }
    }

    #[test]
    fn retry_schedule_matches_upstream_fast_rate_limiter() {
        for retry in 0..GAME_SERVER_SET_FAST_RETRIES {
            assert_eq!(
                game_server_set_retry_delay_ms(retry),
                GAME_SERVER_SET_FAST_RETRY_DELAY_MS
            );
        }
        assert_eq!(
            game_server_set_retry_delay_ms(
                GAME_SERVER_SET_FAST_RETRIES,
            ),
            GAME_SERVER_SET_SLOW_RETRY_DELAY_MS
        );
    }

    #[tokio::test]
    async fn gameserverset_routing_enqueues_add_and_replica_change_only() {
        let storage = MemoryStorage::new();
        let controller = GameServerSetController::new(
            storage,
            config(100),
        );
        let current = set(2);

        let added = controller
            .route_game_server_set_controller_event(
                GameServerSetControllerWatchEvent::GameServerSet(
                    AgonesWatchEvent::Added {
                        key: "default/set".into(),
                        resource: current.clone(),
                    },
                ),
            )
            .await
            .unwrap();
        assert_eq!(
            added,
            Some(GameServerSetWorkItem::new("default", "set"))
        );

        let same = controller
            .route_game_server_set_controller_event(
                GameServerSetControllerWatchEvent::GameServerSet(
                    AgonesWatchEvent::Modified {
                        key: "default/set".into(),
                        resource: current.clone(),
                    },
                ),
            )
            .await
            .unwrap();
        assert!(same.is_none());

        let mut scaled = current;
        scaled.spec.replicas = 3;
        let changed = controller
            .route_game_server_set_controller_event(
                GameServerSetControllerWatchEvent::GameServerSet(
                    AgonesWatchEvent::Modified {
                        key: "default/set".into(),
                        resource: scaled,
                    },
                ),
            )
            .await
            .unwrap();
        assert_eq!(
            changed,
            Some(GameServerSetWorkItem::new("default", "set"))
        );
    }

    #[tokio::test]
    async fn gameserverset_delete_clears_state_cache_without_enqueue() {
        let storage = MemoryStorage::new();
        let controller = GameServerSetController::new(
            storage,
            config(100),
        );
        let current = set(1);

        controller
            .game_server_state_cache
            .for_game_server_set(&current);
        assert_eq!(controller.game_server_state_cache.len(), 1);

        let routed = controller
            .route_game_server_set_controller_event(
                GameServerSetControllerWatchEvent::GameServerSet(
                    AgonesWatchEvent::Deleted {
                        key: "default/set".into(),
                        resource: current,
                    },
                ),
            )
            .await
            .unwrap();

        assert!(routed.is_none());
        assert!(controller.game_server_state_cache.is_empty());
    }

    #[tokio::test]
    async fn child_events_enqueue_live_owner_but_deleting_update_is_ignored() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let owner = repository.create(&set(1)).await.unwrap();

        let mut game_server = owner.game_server();
        game_server.metadata.name = "child".into();
        game_server.metadata.generate_name.clear();
        game_server.apply_defaults();

        let controller = GameServerSetController::new(
            storage,
            config(100),
        );

        for event in [
            AgonesWatchEvent::Added {
                key: "child".into(),
                resource: game_server.clone(),
            },
            AgonesWatchEvent::Deleted {
                key: "child".into(),
                resource: game_server.clone(),
            },
        ] {
            let routed = controller
                .route_game_server_set_controller_event(
                    GameServerSetControllerWatchEvent::GameServer(event),
                )
                .await
                .unwrap();
            assert_eq!(
                routed,
                Some(GameServerSetWorkItem::new("default", "set"))
            );
        }

        let mut deleting = game_server;
        deleting.metadata.deletion_timestamp =
            Some("2026-10-18T00:00:00Z".into());
        let routed = controller
            .route_game_server_set_controller_event(
                GameServerSetControllerWatchEvent::GameServer(
                    AgonesWatchEvent::Modified {
                        key: "child".into(),
                        resource: deleting,
                    },
                ),
            )
            .await
            .unwrap();
        assert!(routed.is_none());
    }

    #[tokio::test]
    async fn runtime_requeues_partial_batches_until_target_converges() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        repository.create(&set(5)).await.unwrap();

        let controller = GameServerSetController::new(
            storage.clone(),
            config(2),
        );
        let task = tokio::spawn(async move {
            controller.run_game_server_set_controller(1).await
        });

        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                let children =
                    repository.list::<GameServer>("default").await.unwrap();
                let current: GameServerSet =
                    repository.get("default", "set").await.unwrap();
                if children.len() == 5 && current.status.replicas == 5 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("GameServerSet runtime did not converge partial batches");

        task.abort();
    }

    #[tokio::test]
    async fn zero_workers_is_rejected() {
        let controller = GameServerSetController::new(
            MemoryStorage::new(),
            config(100),
        );
        let error = controller
            .run_game_server_set_controller(0)
            .await
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("requires at least one worker")
        );
    }
}
