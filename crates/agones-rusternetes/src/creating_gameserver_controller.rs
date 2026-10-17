use std::sync::Arc;

use agones_core::{
    GAME_SERVER_ERRORED_AT_ANNOTATION, GameServer, GameServerState,
};
use chrono::{SecondsFormat, Utc};
use cgp::prelude::*;
use rusternetes_common::resources::Container;
use rusternetes_storage::{build_key, Storage};

use crate::{
    augment_game_server_pod, build_pod,
    configure_game_server_service_account, game_server_pod, AdapterError,
    AgonesRepository, ApiHooks, NoopApiHooks, SdkSidecarConfig,
    GameServerControllerWatchState, PodBuildFeatures, PortAllocator,
    StateUpdateLimiter, build_sdk_sidecar,
};

/// CGP context for the upstream GameServer `Creating` reconciliation slice.
///
/// Storage, Pod construction inputs, and API hooks are context fields rather
/// than parameters threaded through every controller layer. `#[cgp_fn]`
/// generates the capability implementation for any compatible context.
#[derive(HasField)]
pub struct CreatingGameServerController<S, H = NoopApiHooks> {
    pub storage: Arc<S>,
    pub sidecars: Vec<Container>,
    pub sdk_sidecar: Option<SdkSidecarConfig>,
    pub sdk_service_account: String,
    pub pod_build_features: PodBuildFeatures,
    pub hooks: H,
    pub port_allocator: Arc<PortAllocator>,
    pub unhealthy_limiter: Arc<StateUpdateLimiter>,
    pub shutdown_limiter: Arc<StateUpdateLimiter>,
    pub health_unhealthy_limiter: Arc<StateUpdateLimiter>,
    pub wait_on_free_ports: bool,
    pub game_server_watch_state: Arc<std::sync::RwLock<GameServerControllerWatchState>>,
}

impl<S> CreatingGameServerController<S, NoopApiHooks> {
    pub fn new(storage: S) -> Self {
        Self::with_context(
            storage,
            Vec::new(),
            PodBuildFeatures::default(),
            NoopApiHooks,
        )
    }
}

impl<S, H> CreatingGameServerController<S, H> {
    pub fn with_context(
        storage: S,
        sidecars: Vec<Container>,
        pod_build_features: PodBuildFeatures,
        hooks: H,
    ) -> Self {
        Self {
            storage: Arc::new(storage),
            sidecars,
            sdk_sidecar: None,
            sdk_service_account: "agones-sdk".to_owned(),
            pod_build_features,
            hooks,
            port_allocator: Arc::new(PortAllocator::empty()),
            unhealthy_limiter: Arc::new(StateUpdateLimiter::new()),
            shutdown_limiter: Arc::new(StateUpdateLimiter::new()),
            health_unhealthy_limiter: Arc::new(StateUpdateLimiter::new()),
            wait_on_free_ports: false,
            game_server_watch_state: Arc::new(std::sync::RwLock::new(
                GameServerControllerWatchState::default(),
            )),
        }
    }

    pub fn with_port_allocator(mut self, port_allocator: PortAllocator) -> Self {
        self.port_allocator = Arc::new(port_allocator);
        self
    }

    pub fn with_unhealthy_limiter(mut self, limiter: StateUpdateLimiter) -> Self {
        self.unhealthy_limiter = Arc::new(limiter);
        self
    }

    pub fn with_shutdown_limiter(mut self, limiter: StateUpdateLimiter) -> Self {
        self.shutdown_limiter = Arc::new(limiter);
        self
    }

    pub fn with_health_unhealthy_limiter(
        mut self,
        limiter: StateUpdateLimiter,
    ) -> Self {
        self.health_unhealthy_limiter = Arc::new(limiter);
        self
    }

    pub fn with_wait_on_free_ports(mut self, wait_on_free_ports: bool) -> Self {
        self.wait_on_free_ports = wait_on_free_ports;
        self
    }

    pub fn with_sdk_sidecar_config(mut self, config: SdkSidecarConfig) -> Self {
        self.sdk_sidecar = Some(config);
        self
    }

    pub fn with_sdk_service_account(
        mut self,
        service_account: impl Into<String>,
    ) -> Self {
        self.sdk_service_account = service_account.into();
        self
    }
}

pub fn pod_creation_error_moves_game_server_to_error(
    error: &rusternetes_common::Error,
) -> bool {
    matches!(
        error,
        rusternetes_common::Error::InvalidResource(_)
            | rusternetes_common::Error::Forbidden(_)
            | rusternetes_common::Error::Authorization(_)
    )
}

/// CGP equivalent of upstream moveToErrorState.
///
/// The timestamp is stored at second precision in RFC3339 form, matching Go's
/// time.Now().Format(time.RFC3339), before persisting the terminal Error state.
#[cgp_fn]
#[async_trait]
pub async fn move_game_server_to_error<S>(
    &self,
    #[implicit] storage: Arc<S>,
    mut game_server: GameServer,
    _message: &str,
) -> Result<GameServer, AdapterError>
where
    S: Storage + 'static,
{
    game_server.metadata.annotations.insert(
        GAME_SERVER_ERRORED_AT_ANNOTATION.to_owned(),
        Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    );
    game_server.status.state = Some(GameServerState::Error);

    AgonesRepository::new(storage)
        .update(&game_server)
        .await
}

/// Mirrors the isolated behavior of upstream `syncGameServerCreatingState`.
///
/// A missing Pod is created from the existing Rusternetes-native Pod builder.
/// An already-existing Pod is treated as the recovery case from an interrupted
/// prior reconciliation and is not recreated. Only a successful Pod lookup or
/// creation advances the GameServer to `Starting`.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_creating_game_server<S, H>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] sidecars: Vec<Container>,
    #[implicit] sdk_sidecar: Option<SdkSidecarConfig>,
    #[implicit] sdk_service_account: String,
    #[implicit] pod_build_features: PodBuildFeatures,
    #[implicit] hooks: H,
    game_server: GameServer,
) -> Result<Option<GameServer>, AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
    Self: MoveGameServerToError<S> + Sync,
{
    let mut game_server = game_server;

    if game_server.status.state != Some(GameServerState::Creating)
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

    let pod_key = build_key(
        "pods",
        Some(game_server.metadata.namespace.as_str()),
        game_server.metadata.name.as_str(),
    );

    match game_server_pod(storage.as_ref(), &game_server).await {
        Ok(_) => {}
        Err(AdapterError::Rusternetes(rusternetes_common::Error::NotFound(_))) => {
            // Upstream performs this normalization only in the Pod-missing
            // branch immediately before Pod construction.
            game_server.split_static_tcp_udp_ports();

            let mut sidecars = sidecars;
            if let Some(config) = sdk_sidecar.as_ref() {
                sidecars.push(build_sdk_sidecar(&game_server, config)?);
            }

            let mut pod = match build_pod(
                &game_server,
                sidecars,
                pod_build_features,
                &hooks,
            ) {
                Ok(pod) => pod,
                Err(error) => {
                    return self
                        .move_game_server_to_error(
                            game_server,
                            error.to_string().as_str(),
                        )
                        .await
                        .map(Some);
                }
            };

            configure_game_server_service_account(
                &game_server,
                &mut pod,
                sdk_service_account.as_str(),
            )?;
            augment_game_server_pod(&game_server, &mut pod)?;

            match storage.create(&pod_key, &pod).await {
                Ok(_created_pod) => {}
                Err(rusternetes_common::Error::AlreadyExists(_)) => {
                    // Upstream treats Pod AlreadyExists as a successful
                    // recovery and continues the Creating -> Starting step.
                }
                Err(error)
                    if pod_creation_error_moves_game_server_to_error(&error) =>
                {
                    return self
                        .move_game_server_to_error(
                            game_server,
                            error.to_string().as_str(),
                        )
                        .await
                        .map(Some);
                }
                Err(error) => return Err(error.into()),
            }
        }
        Err(error) => return Err(error),
    }

    game_server.status.state = Some(GameServerState::Starting);
    AgonesRepository::new(storage)
        .update(&game_server)
        .await
        .map(Some)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use agones_core::{
        Container as CoreContainer, GameServerPort, GameServerSpec, GameServerStatus, ObjectMeta,
        PodSpec as CorePodSpec, PodTemplateSpec, PortPolicy, Protocol, DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_common::resources::Pod;
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn creating_game_server() -> GameServer {
        GameServer {
            metadata: ObjectMeta {
                name: "server".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSpec {
                container: "game".into(),
                ports: vec![GameServerPort {
                    name: "default".into(),
                    range: "ignored-on-udp-copy".into(),
                    port_policy: Some(PortPolicy::Static),
                    container: Some("game".into()),
                    container_port: 7777,
                    host_port: 7000,
                    protocol: Some(Protocol::TcpUdp),
                }],
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
                state: Some(GameServerState::Creating),
                ..Default::default()
            },
        }
    }

    #[tokio::test]
    async fn missing_pod_is_created_then_gameserver_advances_to_starting() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&creating_game_server()).await.unwrap();
        let controller = CreatingGameServerController::new(storage.clone());

        let reconciled = controller
            .reconcile_creating_game_server(created.clone())
            .await
            .unwrap()
            .expect("Creating GameServer should reconcile");

        assert_eq!(reconciled.status.state, Some(GameServerState::Starting));
        assert_eq!(reconciled.spec.ports.len(), 2);
        assert_eq!(reconciled.spec.ports[0].name, "default-tcp");
        assert_eq!(reconciled.spec.ports[0].protocol, Some(Protocol::Tcp));
        assert_eq!(reconciled.spec.ports[0].range, "ignored-on-udp-copy");
        assert_eq!(reconciled.spec.ports[1].name, "default-udp");
        assert_eq!(reconciled.spec.ports[1].protocol, Some(Protocol::Udp));
        assert!(reconciled.spec.ports[1].range.is_empty());

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(persisted, reconciled);

        let pod_key = build_key("pods", Some("default"), "server");
        let pod: Pod = storage.get(&pod_key).await.unwrap();
        assert_eq!(pod.metadata.name, "server");
        assert_eq!(pod.metadata.namespace.as_deref(), Some("default"));
        let owner = &pod.metadata.owner_references.as_ref().unwrap()[0];
        assert_eq!(owner.kind, "GameServer");
        assert_eq!(owner.uid, created.metadata.uid);

        let ports = pod
            .spec
            .as_ref()
            .unwrap()
            .containers
            .first()
            .unwrap()
            .ports
            .as_ref()
            .unwrap();
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].protocol.as_deref(), Some("TCP"));
        assert_eq!(ports[1].protocol.as_deref(), Some("UDP"));
    }

    #[tokio::test]
    async fn existing_pod_is_not_recreated_or_renormalized() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&creating_game_server()).await.unwrap();

        let existing_pod = build_pod(
            &created,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        let pod_key = build_key("pods", Some("default"), "server");
        let existing_pod: Pod = storage.create(&pod_key, &existing_pod).await.unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let reconciled = controller
            .reconcile_creating_game_server(created)
            .await
            .unwrap()
            .expect("existing Pod recovery path should advance state");

        assert_eq!(reconciled.status.state, Some(GameServerState::Starting));
        assert_eq!(reconciled.spec.ports.len(), 1);
        assert_eq!(reconciled.spec.ports[0].name, "default");
        assert_eq!(reconciled.spec.ports[0].protocol, Some(Protocol::TcpUdp));

        let persisted_pod: Pod = storage.get(&pod_key).await.unwrap();
        assert_eq!(
            serde_json::to_value(persisted_pod).unwrap(),
            serde_json::to_value(existing_pod).unwrap()
        );
    }

    #[tokio::test]
    async fn foreign_same_name_pod_already_exists_recovers_to_starting() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&creating_game_server()).await.unwrap();

        let mut foreign_pod = build_pod(
            &created,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        foreign_pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            "different-owner".into();
        let pod_key = build_key("pods", Some("default"), "server");
        storage.create(&pod_key, &foreign_pod).await.unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let reconciled = controller
            .reconcile_creating_game_server(created)
            .await
            .unwrap()
            .expect("AlreadyExists recovery should advance Creating state");
        assert_eq!(reconciled.status.state, Some(GameServerState::Starting));

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(persisted.status.state, Some(GameServerState::Starting));

        let unchanged: Pod = storage.get(&pod_key).await.unwrap();
        assert_eq!(
            serde_json::to_value(unchanged).unwrap(),
            serde_json::to_value(foreign_pod).unwrap()
        );
    }

    #[test]
    fn invalid_and_forbidden_pod_create_errors_are_terminal() {
        assert!(pod_creation_error_moves_game_server_to_error(
            &rusternetes_common::Error::InvalidResource("invalid".into())
        ));
        assert!(pod_creation_error_moves_game_server_to_error(
            &rusternetes_common::Error::Forbidden("forbidden".into())
        ));
        assert!(pod_creation_error_moves_game_server_to_error(
            &rusternetes_common::Error::Authorization("denied".into())
        ));
        assert!(!pod_creation_error_moves_game_server_to_error(
            &rusternetes_common::Error::Storage("temporary".into())
        ));
        assert!(!pod_creation_error_moves_game_server_to_error(
            &rusternetes_common::Error::Conflict("retry".into())
        ));
    }

    #[tokio::test]
    async fn move_to_error_sets_rfc3339_annotation_and_persists_state() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&creating_game_server()).await.unwrap();
        let controller = CreatingGameServerController::new(storage.clone());

        let errored = controller
            .move_game_server_to_error(created, "some-data")
            .await
            .unwrap();

        assert_eq!(errored.status.state, Some(GameServerState::Error));
        let timestamp = errored
            .metadata
            .annotations
            .get(GAME_SERVER_ERRORED_AT_ANNOTATION)
            .expect("errored-at annotation should exist");
        chrono::DateTime::parse_from_rfc3339(timestamp)
            .expect("errored-at must be RFC3339");

        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(persisted, errored);
    }

    #[tokio::test]
    async fn move_to_error_propagates_update_failure_without_mutating_storage() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&creating_game_server()).await.unwrap();
        let controller = CreatingGameServerController::new(storage.clone());

        storage.inject_conflicts(1);
        assert!(
            controller
                .move_game_server_to_error(created, "some-data")
                .await
                .is_err()
        );

        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(persisted.status.state, Some(GameServerState::Creating));
        assert!(
            !persisted
                .metadata
                .annotations
                .contains_key(GAME_SERVER_ERRORED_AT_ANNOTATION)
        );
    }

    #[tokio::test]
    async fn creating_pod_uses_default_sdk_service_account_and_masks_game_token() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&creating_game_server()).await.unwrap();
        let controller = CreatingGameServerController::new(storage.clone())
            .with_sdk_service_account("sdk-service-account");

        controller
            .reconcile_creating_game_server(created)
            .await
            .unwrap();

        let pod: Pod = storage
            .get(&build_key("pods", Some("default"), "server"))
            .await
            .unwrap();
        let spec = pod.spec.as_ref().unwrap();
        assert_eq!(
            spec.service_account_name.as_deref(),
            Some("sdk-service-account")
        );
        let game = spec
            .containers
            .iter()
            .find(|container| container.name == "game")
            .unwrap();
        assert!(game.volume_mounts.as_ref().unwrap().iter().any(|mount| {
            mount.mount_path
                == crate::SERVICE_ACCOUNT_MOUNT_PATH
        }));
    }

    #[tokio::test]
    async fn explicit_service_account_preserves_game_container_access() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut source = creating_game_server();
        source.spec.template.spec.extensions.insert(
            "serviceAccountName".into(),
            serde_json::Value::String("foobar".into()),
        );
        let created = repository.create(&source).await.unwrap();
        let controller = CreatingGameServerController::new(storage.clone())
            .with_sdk_service_account("sdk-service-account");

        controller
            .reconcile_creating_game_server(created)
            .await
            .unwrap();

        let pod: Pod = storage
            .get(&build_key("pods", Some("default"), "server"))
            .await
            .unwrap();
        let spec = pod.spec.as_ref().unwrap();
        assert_eq!(spec.service_account_name.as_deref(), Some("foobar"));
        let game = spec
            .containers
            .iter()
            .find(|container| container.name == "game")
            .unwrap();
        assert!(
            game.volume_mounts
                .as_ref()
                .is_none_or(Vec::is_empty)
        );
    }

    #[tokio::test]
    async fn deleted_development_and_noncreating_gameservers_are_noops() {
        let storage = MemoryStorage::new();
        let controller = CreatingGameServerController::new(storage);

        let mut deleted = creating_game_server();
        deleted.metadata.deletion_timestamp = Some("2026-10-17T00:00:00Z".into());
        assert!(
            controller
                .reconcile_creating_game_server(deleted)
                .await
                .unwrap()
                .is_none()
        );

        let mut development = creating_game_server();
        development.metadata.annotations = BTreeMap::from([(
            DEV_ADDRESS_ANNOTATION.to_owned(),
            "203.0.113.7".to_owned(),
        )]);
        assert!(
            controller
                .reconcile_creating_game_server(development)
                .await
                .unwrap()
                .is_none()
        );

        let mut ready = creating_game_server();
        ready.status.state = Some(GameServerState::Ready);
        assert!(
            controller
                .reconcile_creating_game_server(ready)
                .await
                .unwrap()
                .is_none()
        );
    }
}
