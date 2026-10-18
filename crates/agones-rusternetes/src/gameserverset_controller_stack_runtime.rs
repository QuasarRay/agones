use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::{
    AdapterError, RunAllocationOverflowController, RunGameServerSetController,
};

/// Top-level Rusternetes runtime corresponding to upstream
/// gameserversets.Controller.Run.
///
/// The AllocationOverflow controller is detached: upstream starts it in a
/// goroutine and only logs its termination, while the main GameServerSet
/// worker queue remains authoritative for the controller lifetime.
#[cgp_fn]
#[async_trait]
pub async fn run_game_server_set_controller_stack<S>(
    &self,
    workers: usize,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    Self: RunGameServerSetController<S>
        + RunAllocationOverflowController<S>
        + Sync,
{
    if workers == 0 {
        return Err(AdapterError::Hook(
            "GameServerSet controller stack requires at least one worker"
                .into(),
        ));
    }

    let main = self.run_game_server_set_controller(workers);
    let allocation_overflow = self.run_allocation_overflow_controller();

    tokio::pin!(main);
    tokio::pin!(allocation_overflow);

    let mut allocation_overflow_active = true;

    loop {
        tokio::select! {
            result = &mut main => return result,
            _ = &mut allocation_overflow, if allocation_overflow_active => {
                // Upstream logs the detached controller error and leaves the
                // main GameServerSet controller running.
                allocation_overflow_active = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::time::Duration;

    use agones_core::{
        AllocationOverflow, Container, GameServer, GameServerPort,
        GameServerSet, GameServerSetSpec, GameServerSetStatus,
        GameServerState, GameServerTemplateSpec, ObjectMeta, PodSpec,
        PodTemplateSpec, PortPolicy, Protocol, SchedulingStrategy,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::{
        AgonesRepository, GameServerSetController,
        GameServerSetControllerConfig,
    };

    use super::*;

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

    fn template_set(replicas: i32) -> GameServerSet {
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
                                }].into(),
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

    #[tokio::test]
    async fn stack_runs_main_gameserverset_controller() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        repository.create(&template_set(1)).await.unwrap();

        let controller = GameServerSetController::new(
            storage.clone(),
            config(),
        );
        let task = tokio::spawn(async move {
            controller.run_game_server_set_controller_stack(1).await
        });

        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                let children =
                    repository.list::<GameServer>("default").await.unwrap();
                if children.len() == 1 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("GameServerSet stack did not run main controller");

        task.abort();
    }

    #[tokio::test]
    async fn stack_runs_allocation_overflow_alongside_main_controller() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());

        let mut set = template_set(3);
        set.spec.allocation_overflow = Some(AllocationOverflow {
            labels: BTreeMap::from([(
                "colour".into(),
                "green".into(),
            )]),
            ..Default::default()
        });
        set.status = GameServerSetStatus {
            replicas: 5,
            allocated_replicas: 5,
            ..Default::default()
        };
        let set = repository.create(&set).await.unwrap();

        for index in 0..5 {
            let mut game_server = set.game_server();
            game_server.metadata.name = format!("server-{index}");
            game_server.metadata.generate_name.clear();
            game_server.status.state = Some(GameServerState::Allocated);
            game_server.apply_defaults();
            repository.create(&game_server).await.unwrap();
        }

        let controller = GameServerSetController::new(
            storage.clone(),
            config(),
        );
        let task = tokio::spawn(async move {
            controller.run_game_server_set_controller_stack(1).await
        });

        // Both runtimes subscribe before processing future events.
        tokio::time::sleep(Duration::from_millis(25)).await;

        let mut current: GameServerSet =
            repository.get("default", "set").await.unwrap();
        current
            .metadata
            .annotations
            .insert("touch".into(), "1".into());
        repository.update(&current).await.unwrap();

        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                let matches = repository
                    .list::<GameServer>("default")
                    .await
                    .unwrap()
                    .into_iter()
                    .filter(|game_server| {
                        game_server
                            .metadata
                            .labels
                            .get("colour")
                            .map(String::as_str)
                            == Some("green")
                    })
                    .count();
                if matches == 2 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("GameServerSet stack did not run AllocationOverflow");

        task.abort();
    }

    #[tokio::test]
    async fn stack_rejects_zero_workers_before_starting_detached_runtime() {
        let controller = GameServerSetController::new(
            MemoryStorage::new(),
            config(),
        );

        let error = controller
            .run_game_server_set_controller_stack(0)
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("requires at least one worker")
        );
    }
}
