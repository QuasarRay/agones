use std::sync::Arc;
use std::time::Duration;

use agones_macros::agones_invariant;
use cgp::prelude::*;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use rusternetes_storage::Storage;

use crate::controller_work_queue::{
    ControllerWorkItem, ControllerWorkQueue, FastSlowRetryPolicy,
};
use crate::{
    AdapterError, ApiHooks, GameServerQueueLane, GameServerWorkItem,
    RebuildGameServerControllerWatchState, RouteGameServerControllerEvent,
    SyncGameServer, watch_game_server_controller_events,
};

pub const GAME_SERVER_FAST_RETRIES: u32 = 5;
pub const GAME_SERVER_FAST_RETRY_DELAY_MS: u64 = 20;
pub const GAME_SERVER_SLOW_RETRY_DELAY_MS: u64 = 500;

/// Mirrors the main GameServer controller's fast/slow workqueue rate limiter.
///
/// The first five failed attempts are retried after 20ms. Subsequent failures
/// retry after 500ms without exponential backoff.
pub const fn game_server_retry_delay_ms(retries: u32) -> u64 {
    if retries < GAME_SERVER_FAST_RETRIES {
        GAME_SERVER_FAST_RETRY_DELAY_MS
    } else {
        GAME_SERVER_SLOW_RETRY_DELAY_MS
    }
}

#[agones_invariant]
fn game_server_retry_policy_matches_upstream(retries: u8) -> bool {
    let retries = retries as u32;
    game_server_retry_delay_ms(retries)
        == if retries < GAME_SERVER_FAST_RETRIES {
            GAME_SERVER_FAST_RETRY_DELAY_MS
        } else {
            GAME_SERVER_SLOW_RETRY_DELAY_MS
        }
}

impl ControllerWorkItem for GameServerWorkItem {
    fn work_key(&self) -> String {
        self.key()
    }
}

type GameServerWorkQueue =
    ControllerWorkQueue<GameServerWorkItem, FastSlowRetryPolicy>;

fn game_server_retry_policy() -> FastSlowRetryPolicy {
    FastSlowRetryPolicy {
        fast_retries: GAME_SERVER_FAST_RETRIES,
        fast_delay: Duration::from_millis(GAME_SERVER_FAST_RETRY_DELAY_MS),
        slow_delay: Duration::from_millis(GAME_SERVER_SLOW_RETRY_DELAY_MS),
    }
}

/// Three independent queues matching the upstream controller's general,
/// creation, and deletion WorkerQueue instances.
///
/// Deduplication is lane-local, as it is upstream: the same GameServer key may
/// legitimately move between queues as its state changes.
#[derive(Clone)]
pub struct GameServerWorkQueues {
    main: GameServerWorkQueue,
    creation: GameServerWorkQueue,
    deletion: GameServerWorkQueue,
}

impl Default for GameServerWorkQueues {
    fn default() -> Self {
        Self {
            main: GameServerWorkQueue::new(game_server_retry_policy()),
            creation: GameServerWorkQueue::new(game_server_retry_policy()),
            deletion: GameServerWorkQueue::new(game_server_retry_policy()),
        }
    }
}

impl GameServerWorkQueues {
    pub fn enqueue(&self, item: GameServerWorkItem) {
        self.queue(item.lane).enqueue(item);
    }

    fn queue(&self, lane: GameServerQueueLane) -> &GameServerWorkQueue {
        match lane {
            GameServerQueueLane::Main => &self.main,
            GameServerQueueLane::Creation => &self.creation,
            GameServerQueueLane::Deletion => &self.deletion,
        }
    }
}

async fn run_game_server_worker<C, S, H>(
    controller: &C,
    queue: GameServerWorkQueue,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
    C: SyncGameServer<S, H> + Sync,
{
    while let Some(item) = queue.next().await {
        match controller
            .sync_game_server(item.namespace.as_str(), item.name.as_str())
            .await
        {
            Ok(_) => queue.complete_success(&item),
            Err(_) => {
                // Upstream WorkerQueue retries every handler error. Conflict
                // and trace errors only differ in log level, not scheduling.
                let delay = queue.record_failure(&item);
                tokio::time::sleep(delay).await;
                queue.complete_retry(&item);
            }
        }
    }

    Ok(())
}

/// Long-running Rusternetes equivalent of the main upstream GameServer
/// controller's informer + three WorkerQueue runtime.
///
/// Watches are subscribed before the initial LIST snapshot. Existing
/// GameServers are then enqueued according to state, after which live
/// GameServer and Pod events are reduced into the same three queues. Each lane
/// receives the requested worker count, matching upstream `Run(ctx, workers)`.
#[cgp_fn]
#[async_trait]
pub async fn run_game_server_controller<S, H>(
    &self,
    #[implicit] storage: Arc<S>,
    workers: usize,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
    Self: RebuildGameServerControllerWatchState<S>
        + RouteGameServerControllerEvent
        + SyncGameServer<S, H>
        + Sync,
{
    if workers == 0 {
        return Err(AdapterError::Hook(
            "GameServer controller requires at least one worker".into(),
        ));
    }

    // Subscribe first so events racing with the initial LIST are retained by
    // Rusternetes and subsequently deduplicated by the observation/queue state.
    let mut events = watch_game_server_controller_events(storage).await?;
    let initial_work = self
        .rebuild_game_server_controller_watch_state()
        .await?;

    let queues = GameServerWorkQueues::default();
    for item in initial_work {
        queues.enqueue(item);
    }

    let mut worker_futures = FuturesUnordered::new();
    for _ in 0..workers {
        worker_futures.push(run_game_server_worker::<Self, S, H>(
            self,
            queues.main.clone(),
        ));
        worker_futures.push(run_game_server_worker::<Self, S, H>(
            self,
            queues.creation.clone(),
        ));
        worker_futures.push(run_game_server_worker::<Self, S, H>(
            self,
            queues.deletion.clone(),
        ));
    }

    loop {
        tokio::select! {
            event = events.next() => {
                let Some(event) = event else {
                    return Ok(());
                };
                let event = event?;
                if let Some(item) = self
                    .route_game_server_controller_event(event)
                    .await?
                {
                    queues.enqueue(item);
                }
            }
            worker = worker_futures.next() => {
                match worker {
                    Some(Ok(())) => {
                        return Err(AdapterError::Hook(
                            "GameServer worker queue terminated unexpectedly".into(),
                        ));
                    }
                    Some(Err(error)) => return Err(error),
                    None => {
                        return Err(AdapterError::Hook(
                            "GameServer controller has no active workers".into(),
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use agones_core::{
        GameServer, GameServerPort, GameServerSpec, GameServerState,
        GameServerStatus, ObjectMeta, PortPolicy, Protocol,
        DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::{AgonesRepository, CreatingGameServerController};

    use super::*;

    fn work_item(lane: GameServerQueueLane) -> GameServerWorkItem {
        GameServerWorkItem {
            namespace: "default".into(),
            name: "server".into(),
            lane,
        }
    }

    #[tokio::test]
    async fn queue_deduplicates_waiting_work_and_requeues_dirty_processing_once() {
        let queue = GameServerWorkQueue::new(game_server_retry_policy());
        let item = work_item(GameServerQueueLane::Main);

        queue.enqueue(item.clone());
        queue.enqueue(item.clone());
        assert_eq!(queue.next().await, Some(item.clone()));

        queue.enqueue(item.clone());
        queue.enqueue(item.clone());
        queue.complete_success(&item);

        assert_eq!(queue.next().await, Some(item.clone()));
        queue.complete_success(&item);

        assert!(
            tokio::time::timeout(Duration::from_millis(10), queue.next())
                .await
                .is_err(),
            "duplicate enqueue should not leave extra work"
        );
    }

    #[tokio::test]
    async fn retry_counter_uses_fast_then_slow_policy_and_resets_on_success() {
        let queue = GameServerWorkQueue::new(game_server_retry_policy());
        let item = work_item(GameServerQueueLane::Main);
        queue.enqueue(item.clone());
        assert_eq!(queue.next().await, Some(item.clone()));

        for expected_retry in 1..=GAME_SERVER_FAST_RETRIES {
            assert_eq!(
                queue.record_failure(&item),
                Duration::from_millis(GAME_SERVER_FAST_RETRY_DELAY_MS)
            );
            assert_eq!(queue.retry_count(&item), expected_retry);
        }
        assert_eq!(
            queue.record_failure(&item),
            Duration::from_millis(GAME_SERVER_SLOW_RETRY_DELAY_MS)
        );

        queue.complete_success(&item);
        assert_eq!(queue.retry_count(&item), 0);
    }

    fn development_game_server() -> GameServer {
        GameServer {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: "dev-server".into(),
                namespace: "default".into(),
                annotations: BTreeMap::from([(
                    DEV_ADDRESS_ANNOTATION.to_owned(),
                    "198.51.100.25".to_owned(),
                )]),
                ..Default::default()
            },
            spec: GameServerSpec {
                ports: vec![GameServerPort {
                    name: "game".into(),
                    port_policy: Some(PortPolicy::Static),
                    container_port: 7777,
                    host_port: 7777,
                    protocol: Some(Protocol::Udp),
                    ..Default::default()
                }].into(),
                ..Default::default()
            },
            status: GameServerStatus {
                state: Some(GameServerState::Creating),
                ..Default::default()
            },
        }
    }

    #[tokio::test]
    async fn runtime_processes_initial_informer_work_through_sync_pipeline() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        repository
            .create(&development_game_server())
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let task = tokio::spawn(async move {
            controller.run_game_server_controller(1).await
        });

        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                let current: GameServer = repository
                    .get("default", "dev-server")
                    .await
                    .unwrap();
                if current.status.state == Some(GameServerState::Ready) {
                    assert_eq!(current.status.address, "198.51.100.25");
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("GameServer runtime did not reconcile initial informer work");

        task.abort();
    }

    #[tokio::test]
    async fn zero_workers_is_rejected() {
        let controller =
            CreatingGameServerController::new(MemoryStorage::new());
        let error = controller
            .run_game_server_controller(0)
            .await
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("requires at least one worker")
        );
    }
}
