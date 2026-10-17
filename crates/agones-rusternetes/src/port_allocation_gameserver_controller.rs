use std::sync::Arc;

use agones_core::{GameServer, GameServerState};
use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::{AdapterError, AgonesRepository, PortAllocator};

/// CGP capability for upstream syncGameServerPortAllocationState.
///
/// The allocator is synchronized once from Rusternetes storage, allocates all
/// Dynamic and Passthrough ports, and is rolled back if GameServer persistence
/// fails so a retry can reuse the same capacity.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_port_allocation_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] port_allocator: Arc<PortAllocator>,
    game_server: GameServer,
) -> Result<Option<GameServer>, AdapterError>
where
    S: Storage + 'static,
{
    if game_server.status.state != Some(GameServerState::PortAllocation)
        || game_server.metadata.deletion_timestamp.is_some()
    {
        return Ok(None);
    }

    port_allocator.ensure_synced(storage.as_ref()).await?;

    let mut allocated = port_allocator.allocate(&game_server).await?;
    allocated.status.state = Some(GameServerState::Creating);

    match AgonesRepository::new(storage).update(&allocated).await {
        Ok(updated) => Ok(Some(updated)),
        Err(error) => {
            port_allocator.deallocate(&allocated).await;
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use agones_core::{
        Container, GameServerPort, GameServerSpec, GameServerStatus, ObjectMeta,
        PodSpec, PodTemplateSpec, PortPolicy, Protocol,
    };
    use rusternetes_common::resources::Node;
    use rusternetes_storage::{build_key, MemoryStorage};

    use crate::{CreatingGameServerController, PortRange};

    use super::*;

    fn game_server(name: &str, state: GameServerState) -> GameServer {
        GameServer {
            metadata: ObjectMeta {
                name: name.into(),
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
            status: GameServerStatus {
                state: Some(state),
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

    async fn store_node(storage: &MemoryStorage) {
        let node = Node::new("node1");
        storage
            .create(&build_key("nodes", None, "node1"), &node)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn port_allocation_assigns_port_and_advances_to_creating() {
        let storage = MemoryStorage::new();
        store_node(&storage).await;

        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(
                "server",
                GameServerState::PortAllocation,
            ))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone())
            .with_port_allocator(allocator());
        let reconciled = controller
            .reconcile_port_allocation_game_server(created)
            .await
            .unwrap()
            .expect("PortAllocation GameServer should reconcile");

        assert_eq!(reconciled.status.state, Some(GameServerState::Creating));
        assert!((10..=20).contains(&reconciled.spec.ports[0].host_port));

        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(persisted, reconciled);
    }

    #[tokio::test]
    async fn startup_sync_avoids_ports_used_by_existing_gameservers() {
        let storage = MemoryStorage::new();
        store_node(&storage).await;
        let repository = AgonesRepository::new(storage.clone());

        let mut existing = game_server("existing", GameServerState::Creating);
        existing.spec.ports[0].host_port = 10;
        existing.status.node_name = "node1".into();
        repository.create(&existing).await.unwrap();

        let target = repository
            .create(&game_server(
                "target",
                GameServerState::PortAllocation,
            ))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage)
            .with_port_allocator(allocator());
        let reconciled = controller
            .reconcile_port_allocation_game_server(target)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(reconciled.spec.ports[0].host_port, 11);
    }

    #[tokio::test]
    async fn update_failure_deallocates_port_for_retry() {
        let storage = MemoryStorage::new();
        store_node(&storage).await;
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(
                "server",
                GameServerState::PortAllocation,
            ))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone())
            .with_port_allocator(allocator());

        storage.inject_conflicts(1);
        assert!(
            controller
                .reconcile_port_allocation_game_server(created.clone())
                .await
                .is_err()
        );

        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(
            persisted.status.state,
            Some(GameServerState::PortAllocation)
        );
        assert_eq!(persisted.spec.ports[0].host_port, 0);

        let retried = controller
            .reconcile_port_allocation_game_server(persisted)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retried.spec.ports[0].host_port, 10);
    }

    #[tokio::test]
    async fn non_port_allocation_and_deleting_gameservers_are_noops() {
        let storage = MemoryStorage::new();
        let controller = CreatingGameServerController::new(storage)
            .with_port_allocator(allocator());

        assert!(
            controller
                .reconcile_port_allocation_game_server(game_server(
                    "ready",
                    GameServerState::Ready,
                ))
                .await
                .unwrap()
                .is_none()
        );

        let mut deleting =
            game_server("deleting", GameServerState::PortAllocation);
        deleting.metadata.deletion_timestamp =
            Some("2026-10-17T00:00:00Z".into());
        assert!(
            controller
                .reconcile_port_allocation_game_server(deleting)
                .await
                .unwrap()
                .is_none()
        );
    }
}
