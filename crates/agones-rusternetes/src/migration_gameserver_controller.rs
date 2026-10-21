use std::sync::Arc;

use agones_core::{GameServer, GameServerState};
use cgp::prelude::*;
use rusternetes_common::resources::{EventType, Node, Pod};
use rusternetes_storage::{build_key, Storage};

use crate::{
    apply_game_server_address_and_port, game_server_event::record_game_server_event,
    is_game_server_pod, AdapterError, AgonesRepository, ApiHooks,
};

/// Returns true when the GameServer's primary address is still present on the
/// current Node.
///
/// This is the behavioral core of upstream MigrationController.anyAddressMatch.
pub fn any_address_match(node: &Node, game_server: &GameServer) -> bool {
    node.status
        .as_ref()
        .and_then(|status| status.addresses.as_deref())
        .unwrap_or_default()
        .iter()
        .any(|address| address.address == game_server.status.address)
}

/// Mirrors upstream MigrationController.isMigratingGameServerPod.
///
/// A migration is detected only for a scheduled, non-terminating GameServer
/// Pod whose persisted GameServer already has a Node, is not being deleted or
/// Unhealthy, whose current Node still exists and is not terminating, and whose
/// Node identity/address differs from the GameServer status.
pub async fn migrating_game_server_for_pod<S>(
    storage: Arc<S>,
    pod: &Pod,
) -> Result<Option<(GameServer, Node)>, AdapterError>
where
    S: Storage + 'static,
{
    let pod_node_name = pod
        .spec
        .as_ref()
        .and_then(|spec| spec.node_name.as_deref())
        .unwrap_or_default();

    if pod_node_name.is_empty()
        || pod.metadata.deletion_timestamp.is_some()
        || !is_game_server_pod(pod)
    {
        return Ok(None);
    }

    let Some(namespace) = pod.metadata.namespace.as_deref() else {
        return Ok(None);
    };

    let repository = AgonesRepository::new(storage.clone());
    let game_server = match repository.get::<GameServer>(namespace, &pod.metadata.name).await {
        Ok(game_server) => game_server,
        Err(AdapterError::Rusternetes(rusternetes_common::Error::NotFound(_))) => {
            return Ok(None);
        }
        Err(error) => return Err(error),
    };

    if game_server.status.node_name.is_empty()
        || game_server.is_being_deleted()
        || game_server.status.state == Some(GameServerState::Unhealthy)
    {
        return Ok(None);
    }

    let node_key = build_key("nodes", None, pod_node_name);
    let node: Node = match storage.get(&node_key).await {
        Ok(node) => node,
        Err(rusternetes_common::Error::NotFound(_)) => return Ok(None),
        Err(error) => return Err(error.into()),
    };

    if node.metadata.deletion_timestamp.is_some() {
        return Ok(None);
    }

    if pod_node_name == game_server.status.node_name
        && any_address_match(&node, &game_server)
    {
        return Ok(None);
    }

    Ok(Some((game_server, node)))
}

/// CGP capability for upstream MigrationController.syncGameServer.
///
/// Before Ready, migration reapplies Node address and Pod port information
/// without changing state. After Ready, migration marks the GameServer
/// Unhealthy so its owning controller can replace it. Both paths emit the
/// exact upstream Warning event payload.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_migration_game_server<S, H>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] hooks: H,
    pod: Pod,
) -> Result<Option<GameServer>, AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
{
    let Some((mut game_server, node)) =
        migrating_game_server_for_pod(storage.clone(), &pod).await?
    else {
        return Ok(None);
    };

    let event_message;
    if game_server.is_before_ready() {
        apply_game_server_address_and_port(
            &mut game_server,
            &node,
            &pod,
            &hooks,
        )?;
        event_message = "Address updated due to Node migration";
    } else {
        game_server.status.state = Some(GameServerState::Unhealthy);
        event_message = "Node migration occurred";
    }

    let updated = AgonesRepository::new(storage.clone())
        .update(&game_server)
        .await?;

    let reason = updated
        .status
        .state
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_default();

    record_game_server_event(
        storage.as_ref(),
        &updated,
        "migration-controller",
        EventType::Warning,
        &reason,
        event_message,
    )
    .await;

    Ok(Some(updated))
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerPort, GameServerSpec,
        GameServerStatus, ObjectMeta, PodSpec as CorePodSpec, PodTemplateSpec,
        PortPolicy, Protocol,
    };
    use rusternetes_common::resources::{
        Event, NodeAddress, NodeStatus, PodStatus,
    };
    use rusternetes_storage::{
        build_key, build_prefix, MemoryStorage,
    };

    use crate::{
        build_pod, CreatingGameServerController, NoopApiHooks,
        PodBuildFeatures,
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
                ports: vec![GameServerPort {
                    name: "default".into(),
                    port_policy: Some(PortPolicy::Static),
                    container: Some("game".into()),
                    container_port: 7777,
                    host_port: 9999,
                    protocol: Some(Protocol::Udp),
                    ..Default::default()
                }].into(),
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
        pod.status = Some(PodStatus::default());
        pod
    }

    async fn store_fixture(
        storage: &MemoryStorage,
        game_server: &GameServer,
        pod: &Pod,
        node: &Node,
    ) -> GameServer {
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(game_server).await.unwrap();

        let mut pod = pod.clone();
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        storage
            .create(
                &build_key(
                    "pods",
                    pod.metadata.namespace.as_deref(),
                    pod.metadata.name.as_str(),
                ),
                &pod,
            )
            .await
            .unwrap();
        storage
            .create(
                &build_key("nodes", None, node.metadata.name.as_str()),
                node,
            )
            .await
            .unwrap();

        created
    }

    async fn events(storage: &MemoryStorage) -> Vec<Event> {
        storage
            .list(&build_prefix("events", Some("default")))
            .await
            .unwrap()
    }

    #[test]
    fn address_match_checks_all_node_addresses() {
        let mut game_server = game_server(GameServerState::Scheduled);
        game_server.status.address = "203.0.113.10".into();

        let mut node = node("node-a", "198.51.100.10");
        node.status.as_mut().unwrap().addresses.as_mut().unwrap().push(
            NodeAddress {
                address_type: "InternalIP".into(),
                address: "203.0.113.10".into(),
            },
        );

        assert!(any_address_match(&node, &game_server));
        game_server.status.address = "192.0.2.99".into();
        assert!(!any_address_match(&node, &game_server));
    }

    #[tokio::test]
    async fn unchanged_node_and_address_are_a_noop() {
        let storage = MemoryStorage::new();
        let mut fixture = game_server(GameServerState::Scheduled);
        fixture.status.node_name = "node-a".into();
        fixture.status.address = "203.0.113.10".into();

        let pod = pod_for(&fixture, "node-a");
        let node = node("node-a", "203.0.113.10");
        let created = store_fixture(&storage, &fixture, &pod, &node).await;

        let mut current_pod = pod;
        current_pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_migration_game_server(current_pod)
                .await
                .unwrap()
                .is_none()
        );
        assert!(events(&storage).await.is_empty());
    }

    #[tokio::test]
    async fn gameserver_without_recorded_node_is_a_noop() {
        let storage = MemoryStorage::new();
        let fixture = game_server(GameServerState::Scheduled);
        let pod = pod_for(&fixture, "node-a");
        let node = node("node-a", "203.0.113.10");
        let created = store_fixture(&storage, &fixture, &pod, &node).await;

        let mut current_pod = pod;
        current_pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        let controller = CreatingGameServerController::new(storage);
        assert!(
            controller
                .reconcile_migration_game_server(current_pod)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn address_change_before_ready_reapplies_address_and_ports() {
        let storage = MemoryStorage::new();
        let mut fixture = game_server(GameServerState::Scheduled);
        fixture.status.node_name = "node-a".into();
        fixture.status.address = "203.0.113.10".into();

        let mut pod = pod_for(&fixture, "node-a");
        pod.spec.as_mut().unwrap().containers[0]
            .ports
            .as_mut()
            .unwrap()[0]
            .host_port = Some(10001);
        let node = node("node-a", "198.51.100.20");
        let created = store_fixture(&storage, &fixture, &pod, &node).await;

        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        let controller = CreatingGameServerController::new(storage.clone());
        let updated = controller
            .reconcile_migration_game_server(pod)
            .await
            .unwrap()
            .expect("address change should reconcile");

        assert_eq!(updated.status.state, Some(GameServerState::Scheduled));
        assert_eq!(updated.status.address, "198.51.100.20");
        assert_eq!(updated.status.node_name, "node-a");
        assert_eq!(updated.status.ports[0].port, 9999);

        let events = events(&storage).await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::Warning);
        assert_eq!(events[0].reason, "Scheduled");
        assert_eq!(
            events[0].message,
            "Address updated due to Node migration"
        );
        assert_eq!(events[0].source.component, "migration-controller");
    }

    #[tokio::test]
    async fn node_change_before_ready_reapplies_node_identity() {
        let storage = MemoryStorage::new();
        let mut fixture = game_server(GameServerState::Scheduled);
        fixture.status.node_name = "node-a".into();
        fixture.status.address = "203.0.113.10".into();

        let mut pod = pod_for(&fixture, "node-b");
        let node = node("node-b", "198.51.100.20");
        let created = store_fixture(&storage, &fixture, &pod, &node).await;
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        let controller = CreatingGameServerController::new(storage);
        let updated = controller
            .reconcile_migration_game_server(pod)
            .await
            .unwrap()
            .expect("node migration should reconcile");

        assert_eq!(updated.status.state, Some(GameServerState::Scheduled));
        assert_eq!(updated.status.node_name, "node-b");
        assert_eq!(updated.status.address, "198.51.100.20");
    }

    #[tokio::test]
    async fn migration_after_ready_marks_gameserver_unhealthy() {
        let storage = MemoryStorage::new();
        let mut fixture = game_server(GameServerState::Allocated);
        fixture.status.node_name = "node-a".into();
        fixture.status.address = "203.0.113.10".into();

        let mut pod = pod_for(&fixture, "node-b");
        let node = node("node-b", "198.51.100.20");
        let created = store_fixture(&storage, &fixture, &pod, &node).await;
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        let controller = CreatingGameServerController::new(storage.clone());
        let updated = controller
            .reconcile_migration_game_server(pod)
            .await
            .unwrap()
            .expect("post-ready migration should reconcile");

        assert_eq!(updated.status.state, Some(GameServerState::Unhealthy));

        let events = events(&storage).await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::Warning);
        assert_eq!(events[0].reason, "Unhealthy");
        assert_eq!(events[0].message, "Node migration occurred");
        assert_eq!(events[0].source.component, "migration-controller");
    }

    #[tokio::test]
    async fn terminating_pod_or_node_is_not_migrated() {
        let storage = MemoryStorage::new();
        let mut fixture = game_server(GameServerState::Scheduled);
        fixture.status.node_name = "node-a".into();
        fixture.status.address = "203.0.113.10".into();

        let mut pod = pod_for(&fixture, "node-b");
        pod.metadata.deletion_timestamp = Some(chrono::Utc::now());
        let node = node("node-b", "198.51.100.20");
        let created = store_fixture(&storage, &fixture, &pod, &node).await;
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_migration_game_server(pod)
                .await
                .unwrap()
                .is_none()
        );

        let mut live_pod = pod_for(&created, "node-b");
        live_pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        let mut terminating_node = node;
        terminating_node.metadata.deletion_timestamp =
            Some(chrono::Utc::now());
        storage
            .update(
                &build_key("nodes", None, "node-b"),
                &terminating_node,
            )
            .await
            .unwrap();

        assert!(
            controller
                .reconcile_migration_game_server(live_pod)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn missing_gameserver_or_node_is_an_expected_noop() {
        let storage = MemoryStorage::new();

        let mut fixture = game_server(GameServerState::Scheduled);
        fixture.status.node_name = "node-a".into();
        fixture.status.address = "203.0.113.10".into();
        let pod = pod_for(&fixture, "node-b");

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_migration_game_server(pod.clone())
                .await
                .unwrap()
                .is_none()
        );

        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&fixture).await.unwrap();
        let mut pod = pod;
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();

        assert!(
            controller
                .reconcile_migration_game_server(pod)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn non_gameserver_pod_is_not_considered_for_migration() {
        let storage = MemoryStorage::new();
        let mut fixture = game_server(GameServerState::Scheduled);
        fixture.status.node_name = "node-a".into();
        fixture.status.address = "203.0.113.10".into();

        let mut pod = pod_for(&fixture, "node-b");
        pod.metadata
            .labels
            .as_mut()
            .unwrap()
            .remove(crate::ROLE_LABEL);
        let node = node("node-b", "198.51.100.20");

        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&fixture).await.unwrap();
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();
        storage
            .create(&build_key("nodes", None, "node-b"), &node)
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage);
        assert!(
            controller
                .reconcile_migration_game_server(pod)
                .await
                .unwrap()
                .is_none()
        );
    }
}
