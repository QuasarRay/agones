use std::sync::Arc;

use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::{
    AdapterError, ApiHooks, PodBuildFeatures, PortAllocator,
    RunGameServerController, RunHealthGameServerController,
    RunMigrationGameServerController, RunMissingPodGameServerController,
    RunSucceededGameServerController,
};

/// Top-level Rusternetes runtime corresponding to upstream
/// gameservers.Controller.Run.
///
/// Startup ordering preserves the control-plane dependency that matters to the
/// Rust port: the port allocator reconstructs its state before any GameServer
/// worker can allocate. Auxiliary controllers then run alongside the main
/// three-lane GameServer controller. As upstream, an auxiliary controller
/// failure terminates only that auxiliary controller; the main controller
/// remains authoritative for the lifetime of the stack.
///
/// CRD establishment/cache-sync barriers from client-go do not have a direct
/// Rusternetes equivalent at this layer: each runtime subscribes its watch
/// before its initial LIST, and the port allocator is explicitly synchronized
/// before workers begin.
#[cgp_fn]
#[async_trait]
pub async fn run_game_server_controller_stack<S, H>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] port_allocator: Arc<PortAllocator>,
    #[implicit] pod_build_features: PodBuildFeatures,
    workers: usize,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
    Self: RunGameServerController<S, H>
        + RunHealthGameServerController<S>
        + RunMigrationGameServerController<S, H>
        + RunMissingPodGameServerController<S>
        + RunSucceededGameServerController<S>
        + Sync,
{
    if workers == 0 {
        return Err(AdapterError::Hook(
            "GameServer controller stack requires at least one worker".into(),
        ));
    }

    // Upstream Controller.Run starts PortAllocator before every worker queue.
    port_allocator.ensure_synced(storage.as_ref()).await?;

    let main = self.run_game_server_controller(workers);
    let health = self.run_health_game_server_controller(workers);
    let migration = self.run_migration_game_server_controller(workers);
    let missing = self.run_missing_pod_game_server_controller(workers);
    let succeeded = async {
        if pod_build_features.sidecar_containers {
            self.run_succeeded_game_server_controller(workers).await
        } else {
            futures::future::pending::<Result<(), AdapterError>>().await
        }
    };

    tokio::pin!(main);
    tokio::pin!(health);
    tokio::pin!(migration);
    tokio::pin!(missing);
    tokio::pin!(succeeded);

    let mut health_active = true;
    let mut migration_active = true;
    let mut missing_active = true;
    let mut succeeded_active = pod_build_features.sidecar_containers;

    loop {
        tokio::select! {
            result = &mut main => return result,
            _ = &mut health, if health_active => {
                // Upstream logs this error in its detached goroutine and
                // leaves the main controller running.
                health_active = false;
            }
            _ = &mut migration, if migration_active => {
                migration_active = false;
            }
            _ = &mut missing, if missing_active => {
                missing_active = false;
            }
            _ = &mut succeeded, if succeeded_active => {
                succeeded_active = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use agones_core::{
        Container, GameServer, GameServerPort, GameServerSpec,
        GameServerState, GameServerStatus, ObjectMeta, PodSpec,
        PodTemplateSpec, PortPolicy, Protocol,
    };
    use rusternetes_common::resources::Node;
    use rusternetes_storage::{build_key, MemoryStorage, Storage};

    use crate::{
        AgonesRepository, CreatingGameServerController, PortRange,
    };

    use super::*;

    fn dynamic_game_server() -> GameServer {
        GameServer {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: "server".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSpec {
                container: "game".into(),
                ports: vec![GameServerPort {
                    name: "default".into(),
                    range: "default".into(),
                    port_policy: Some(PortPolicy::Dynamic),
                    container: Some("game".into()),
                    container_port: 7777,
                    host_port: 0,
                    protocol: Some(Protocol::Udp),
                }].into(),
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
            status: GameServerStatus {
                state: Some(GameServerState::PortAllocation),
                ..Default::default()
            },
        }
    }

    fn allocator() -> PortAllocator {
        PortAllocator::new(BTreeMap::from([(
            "default".into(),
            PortRange {
                min_port: 10,
                max_port: 20,
            },
        )]))
        .unwrap()
    }

    #[tokio::test]
    async fn stack_starts_port_allocator_before_initial_gameserver_work() {
        let storage = MemoryStorage::new();
        storage
            .create(&build_key("nodes", None, "node-a"), &Node::new("node-a"))
            .await
            .unwrap();

        let repository = AgonesRepository::new(storage.clone());
        repository.create(&dynamic_game_server()).await.unwrap();

        let controller = CreatingGameServerController::new(storage.clone())
            .with_port_allocator(allocator());
        let task = tokio::spawn(async move {
            controller.run_game_server_controller_stack(1).await
        });

        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            loop {
                let current: GameServer =
                    repository.get("default", "server").await.unwrap();

                if current.status.state != Some(GameServerState::PortAllocation) {
                    assert!((10..=20).contains(&current.spec.ports[0].host_port));
                    break;
                }

                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("top-level controller stack did not process initial work");

        task.abort();
    }

    #[tokio::test]
    async fn stack_zero_workers_is_rejected_before_startup() {
        let storage = MemoryStorage::new();
        let controller =
            CreatingGameServerController::new(storage)
                .with_port_allocator(allocator());

        let error = controller
            .run_game_server_controller_stack(0)
            .await
            .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("requires at least one worker")
        );
    }
}
