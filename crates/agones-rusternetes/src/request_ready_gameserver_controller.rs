use std::sync::Arc;

use agones_core::{
    GameServer, GameServerState, GAME_SERVER_READY_CONTAINER_ID_ANNOTATION,
};
use cgp::prelude::*;
use rusternetes_common::resources::{ContainerState, Node, Pod};
use rusternetes_storage::{build_key, Storage};

use crate::{
    apply_game_server_address_and_port, apply_game_server_pod_ips, game_server_pod,
    AdapterError, AgonesRepository, ApiHooks, PodBuildFeatures,
};

async fn apply_ready_container_annotation<S: Storage>(
    storage: &S,
    features: PodBuildFeatures,
    game_server: &mut GameServer,
    pod: &Pod,
) -> Result<(), AdapterError> {
    if features.sidecar_containers {
        return Ok(());
    }

    if !game_server
        .metadata
        .annotations
        .contains_key(GAME_SERVER_READY_CONTAINER_ID_ANNOTATION)
    {
        let status = pod
            .status
            .as_ref()
            .and_then(|status| status.container_statuses.as_deref())
            .and_then(|statuses| {
                statuses
                    .iter()
                    .find(|status| status.name == game_server.spec.container)
            })
            .ok_or_else(|| {
                AdapterError::Hook(format!(
                    "game server container for GameServer {} in namespace {} not present in pod status, try again",
                    game_server.metadata.name, game_server.metadata.namespace
                ))
            })?;

        if !matches!(status.state, Some(ContainerState::Running { .. })) {
            return Err(AdapterError::Hook(format!(
                "game server container for GameServer {} in namespace {} is not currently running, try again",
                game_server.metadata.name, game_server.metadata.namespace
            )));
        }

        let container_id = status.container_id.clone().unwrap_or_default();
        game_server.metadata.annotations.insert(
            GAME_SERVER_READY_CONTAINER_ID_ANNOTATION.to_owned(),
            container_id,
        );
    }

    let ready_container_id = game_server
        .metadata
        .annotations
        .get(GAME_SERVER_READY_CONTAINER_ID_ANNOTATION)
        .cloned()
        .ok_or_else(|| {
            AdapterError::Hook(format!(
                "game server container for GameServer {} in namespace {} not present in pod status, try again",
                game_server.metadata.name, game_server.metadata.namespace
            ))
        })?;

    let pod_has_annotation = pod
        .metadata
        .annotations
        .as_ref()
        .is_some_and(|annotations| {
            annotations.contains_key(GAME_SERVER_READY_CONTAINER_ID_ANNOTATION)
        });

    if !pod_has_annotation {
        let mut pod = pod.clone();
        pod.metadata
            .annotations
            .get_or_insert_with(Default::default)
            .insert(
                GAME_SERVER_READY_CONTAINER_ID_ANNOTATION.to_owned(),
                ready_container_id,
            );

        let pod_key = build_key(
            "pods",
            pod.metadata.namespace.as_deref(),
            pod.metadata.name.as_str(),
        );
        storage.update(&pod_key, &pod).await?;
    }

    Ok(())
}

/// CGP capability for upstream `syncGameServerRequestReadyState`.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_request_ready_game_server<S, H>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] pod_build_features: PodBuildFeatures,
    #[implicit] hooks: H,
    game_server: GameServer,
) -> Result<Option<GameServer>, AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
{
    let mut game_server = game_server;

    if game_server.status.state != Some(GameServerState::RequestReady)
        || game_server.metadata.deletion_timestamp.is_some()
        || game_server.get_dev_address().is_some()
    {
        return Ok(None);
    }

    if game_server.metadata.name.is_empty() {
        return Err(AdapterError::MissingField("metadata.name"));
    }
    if game_server.metadata.namespace.is_empty() {
        return Err(AdapterError::MissingField("metadata.namespace"));
    }

    let pod = game_server_pod(storage.as_ref(), &game_server).await?;

    if game_server.status.node_name.is_empty() {
        let node_name = pod
            .spec
            .as_ref()
            .and_then(|spec| spec.node_name.as_deref())
            .filter(|name| !name.is_empty())
            .ok_or_else(|| {
                AdapterError::Hook(format!(
                    "node not yet populated for Pod {}",
                    pod.metadata.name
                ))
            })?;

        let node: Node = storage
            .get(&build_key("nodes", None, node_name))
            .await?;
        apply_game_server_address_and_port(
            &mut game_server,
            &node,
            &pod,
            &hooks,
        )?;
    }

    apply_game_server_pod_ips(&mut game_server, &pod);
    apply_ready_container_annotation(
        storage.as_ref(),
        pod_build_features,
        &mut game_server,
        &pod,
    )
    .await?;

    game_server.status.state = Some(GameServerState::Ready);
    AgonesRepository::new(storage)
        .update(&game_server)
        .await
        .map(Some)
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerSpec, GameServerStatus, ObjectMeta,
        PodSpec as CorePodSpec, PodTemplateSpec, DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_common::resources::{
        ContainerState, ContainerStatus, NodeAddress as NativeNodeAddress,
        NodeStatus, PodIP, PodStatus,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::{
        build_pod, CreatingGameServerController, NoopApiHooks, PodBuildFeatures,
    };

    use super::*;

    fn regular_sidecar_controller(storage: MemoryStorage) -> CreatingGameServerController<MemoryStorage> {
        // These fixtures exercise the source's SidecarContainers=false branch.
        CreatingGameServerController::from_feature_configuration(
            storage, &agones_core::FeatureGateContext::default(), "SidecarContainers=false",
        ).unwrap()
    }

    fn request_ready_game_server() -> GameServer {
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
                state: Some(GameServerState::RequestReady),
                node_name: "node1".into(),
                ..Default::default()
            },
        }
    }

    fn running_status(container_name: &str, id: &str) -> ContainerStatus {
        ContainerStatus {
            name: container_name.into(),
            ready: true,
            restart_count: 0,
            state: Some(ContainerState::Running { started_at: None }),
            last_state: None,
            image: None,
            image_id: None,
            container_id: Some(id.into()),
            started: Some(true),
            allocated_resources: None,
            allocated_resources_status: None,
            resources: None,
            user: None,
            volume_mounts: None,
            stop_signal: None,
        }
    }

    async fn store_pod(
        storage: &MemoryStorage,
        game_server: &GameServer,
        container_statuses: Option<Vec<ContainerStatus>>,
        pod_ips: Option<Vec<PodIP>>,
    ) -> Pod {
        let mut pod = build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.status = Some(PodStatus {
            container_statuses,
            pod_i_ps: pod_ips,
            ..Default::default()
        });
        storage
            .create(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn ready_request_pins_container_id_to_gameserver_and_pod() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&request_ready_game_server())
            .await
            .unwrap();
        store_pod(
            &storage,
            &created,
            Some(vec![running_status("game", "containerd://1234")]),
            None,
        )
        .await;

        let controller = regular_sidecar_controller(storage.clone());
        let reconciled = controller
            .reconcile_request_ready_game_server(created)
            .await
            .unwrap()
            .expect("RequestReady GameServer should reconcile");

        assert_eq!(reconciled.status.state, Some(GameServerState::Ready));
        assert_eq!(
            reconciled
                .metadata
                .annotations
                .get(GAME_SERVER_READY_CONTAINER_ID_ANNOTATION)
                .map(String::as_str),
            Some("containerd://1234")
        );

        let pod: Pod = storage
            .get(&build_key("pods", Some("default"), "server"))
            .await
            .unwrap();
        assert_eq!(
            pod.metadata
                .annotations
                .as_ref()
                .and_then(|annotations| {
                    annotations.get(GAME_SERVER_READY_CONTAINER_ID_ANNOTATION)
                })
                .map(String::as_str),
            Some("containerd://1234")
        );
    }

    #[tokio::test]
    async fn ready_request_folds_pod_ips_into_same_state_update() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&request_ready_game_server())
            .await
            .unwrap();
        store_pod(
            &storage,
            &created,
            Some(vec![running_status("game", "containerd://1234")]),
            Some(vec![PodIP {
                ip: "10.0.0.7".into(),
            }]),
        )
        .await;

        let controller = CreatingGameServerController::new(storage.clone());
        let reconciled = controller
            .reconcile_request_ready_game_server(created)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(reconciled.status.state, Some(GameServerState::Ready));
        assert!(reconciled.status.addresses.iter().any(|address| {
            address.address_type == "PodIP" && address.address == "10.0.0.7"
        }));
    }

    #[tokio::test]
    async fn missing_address_is_populated_before_ready() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fixture = request_ready_game_server();
        fixture.status.node_name.clear();
        let created = repository.create(&fixture).await.unwrap();

        let mut pod = store_pod(
            &storage,
            &created,
            Some(vec![running_status("game", "containerd://1234")]),
            None,
        )
        .await;
        pod.spec.as_mut().unwrap().node_name = Some("node1".into());
        storage
            .update(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();

        let mut node = Node::new("node1");
        node.status = Some(NodeStatus {
            addresses: Some(vec![NativeNodeAddress {
                address_type: "ExternalIP".into(),
                address: "203.0.113.8".into(),
            }]),
            ..Default::default()
        });
        storage
            .create(&build_key("nodes", None, "node1"), &node)
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let reconciled = controller
            .reconcile_request_ready_game_server(created)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(reconciled.status.state, Some(GameServerState::Ready));
        assert_eq!(reconciled.status.node_name, "node1");
        assert_eq!(reconciled.status.address, "203.0.113.8");
    }

    #[tokio::test]
    async fn existing_gameserver_annotation_is_preserved_and_copied_to_pod() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fixture = request_ready_game_server();
        fixture.metadata.annotations.insert(
            GAME_SERVER_READY_CONTAINER_ID_ANNOTATION.into(),
            "containerd://old".into(),
        );
        let created = repository.create(&fixture).await.unwrap();
        store_pod(&storage, &created, None, None).await;

        let controller = regular_sidecar_controller(storage.clone());
        let reconciled = controller
            .reconcile_request_ready_game_server(created)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            reconciled.metadata.annotations[GAME_SERVER_READY_CONTAINER_ID_ANNOTATION],
            "containerd://old"
        );
        let pod: Pod = storage
            .get(&build_key("pods", Some("default"), "server"))
            .await
            .unwrap();
        assert_eq!(
            pod.metadata.annotations.as_ref().unwrap()
                [GAME_SERVER_READY_CONTAINER_ID_ANNOTATION],
            "containerd://old"
        );
    }

    #[tokio::test]
    async fn existing_pod_annotation_avoids_rewrite() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&request_ready_game_server())
            .await
            .unwrap();
        let mut pod = store_pod(
            &storage,
            &created,
            Some(vec![running_status("game", "containerd://1234")]),
            None,
        )
        .await;
        pod.metadata
            .annotations
            .get_or_insert_with(Default::default)
            .insert(
                GAME_SERVER_READY_CONTAINER_ID_ANNOTATION.into(),
                "already-set".into(),
            );
        let pod: Pod = storage
            .update(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();
        let before = serde_json::to_value(&pod).unwrap();

        let controller = regular_sidecar_controller(storage.clone());
        controller
            .reconcile_request_ready_game_server(created)
            .await
            .unwrap()
            .unwrap();

        let after: Pod = storage
            .get(&build_key("pods", Some("default"), "server"))
            .await
            .unwrap();
        assert_eq!(serde_json::to_value(after).unwrap(), before);
    }

    #[tokio::test]
    async fn nonrunning_or_missing_container_status_does_not_advance() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&request_ready_game_server())
            .await
            .unwrap();

        let mut not_running = running_status("game", "containerd://1234");
        not_running.state = None;
        store_pod(&storage, &created, Some(vec![not_running]), None).await;

        let controller = regular_sidecar_controller(storage.clone());
        let error = controller
            .reconcile_request_ready_game_server(created.clone())
            .await
            .unwrap_err();
        assert!(error.to_string().contains("is not currently running"));

        let mut pod: Pod = storage
            .get(&build_key("pods", Some("default"), "server"))
            .await
            .unwrap();
        pod.status.as_mut().unwrap().container_statuses = None;
        storage
            .update(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();

        let error = controller
            .reconcile_request_ready_game_server(created)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("not present in pod status"));

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(persisted.status.state, Some(GameServerState::RequestReady));
    }

    #[tokio::test]
    async fn sidecar_containers_mode_skips_ready_container_annotation() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&request_ready_game_server())
            .await
            .unwrap();
        let pod = store_pod(&storage, &created, None, None).await;
        let before = serde_json::to_value(&pod).unwrap();

        let controller = CreatingGameServerController::with_context(
            storage.clone(),
            Vec::new(),
            PodBuildFeatures {
                sidecar_containers: true,
            },
            NoopApiHooks,
        );
        let reconciled = controller
            .reconcile_request_ready_game_server(created)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(reconciled.status.state, Some(GameServerState::Ready));
        assert!(!reconciled
            .metadata
            .annotations
            .contains_key(GAME_SERVER_READY_CONTAINER_ID_ANNOTATION));

        let after: Pod = storage
            .get(&build_key("pods", Some("default"), "server"))
            .await
            .unwrap();
        assert_eq!(serde_json::to_value(after).unwrap(), before);
    }

    #[tokio::test]
    async fn foreign_same_name_pod_does_not_advance_request_ready() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&request_ready_game_server())
            .await
            .unwrap();
        let mut pod = store_pod(
            &storage,
            &created,
            Some(vec![running_status("game", "containerd://1234")]),
            None,
        )
        .await;
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            "different-owner".into();
        storage
            .update(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage);
        let error = controller
            .reconcile_request_ready_game_server(created)
            .await
            .unwrap_err();
        assert!(matches!(
            error,
            AdapterError::Rusternetes(rusternetes_common::Error::NotFound(_))
        ));

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(
            persisted.status.state,
            Some(GameServerState::RequestReady)
        );
    }

    #[tokio::test]
    async fn deleted_development_and_non_request_ready_are_noops() {
        let storage = MemoryStorage::new();
        let controller = CreatingGameServerController::new(storage);

        let mut deleted = request_ready_game_server();
        deleted.metadata.deletion_timestamp = Some("2026-10-17T00:00:00Z".into());
        assert!(controller
            .reconcile_request_ready_game_server(deleted)
            .await
            .unwrap()
            .is_none());

        let mut development = request_ready_game_server();
        development
            .metadata
            .annotations
            .insert(DEV_ADDRESS_ANNOTATION.into(), "203.0.113.7".into());
        assert!(controller
            .reconcile_request_ready_game_server(development)
            .await
            .unwrap()
            .is_none());

        let mut ready = request_ready_game_server();
        ready.status.state = Some(GameServerState::Ready);
        assert!(controller
            .reconcile_request_ready_game_server(ready)
            .await
            .unwrap()
            .is_none());
    }
}
