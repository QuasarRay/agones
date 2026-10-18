use std::sync::Arc;

use agones_core::GameServer;
use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::{
    AdapterError, AgonesRepository, ApiHooks, ReconcileCreatingGameServer,
    ReconcileDeletingGameServer, ReconcileDevelopmentGameServer,
    ReconcilePortAllocationGameServer, ReconcileRequestReadyGameServer,
    ReconcileShutdownGameServer, ReconcileStableGameServerPodIps,
    ReconcileStartingGameServer,
};

/// Result of one pass through the main upstream GameServer reconciliation
/// pipeline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameServerSyncOutcome {
    /// The requested GameServer no longer exists in Rusternetes storage.
    NotFound,
    /// The GameServer existed, but none of the ordered reconciliation stages
    /// needed to persist or delete anything.
    Unchanged(GameServer),
    /// At least one reconciliation stage persisted state or requested
    /// deletion. The contained value is the last in-memory GameServer carried
    /// through the ordered pipeline.
    Reconciled(GameServer),
}

impl GameServerSyncOutcome {
    pub fn game_server(&self) -> Option<&GameServer> {
        match self {
            Self::NotFound => None,
            Self::Unchanged(game_server) | Self::Reconciled(game_server) => {
                Some(game_server)
            }
        }
    }

    pub const fn changed(&self) -> bool {
        matches!(self, Self::Reconciled(_))
    }
}

/// CGP composition of the ordered state machine from upstream
/// `Controller.syncGameServer`.
///
/// Each existing state-specific capability remains independently testable, but
/// the production controller can now execute them in the exact upstream order:
/// deletion timestamp, port allocation, creating, starting, request-ready,
/// stable Pod IPs, development GameServer handling, then shutdown.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_game_server_pipeline<S, H>(
    &self,
    game_server: GameServer,
) -> Result<GameServerSyncOutcome, AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
    Self: ReconcileDeletingGameServer<S>
        + ReconcilePortAllocationGameServer<S>
        + ReconcileCreatingGameServer<S, H>
        + ReconcileStartingGameServer<S, H>
        + ReconcileRequestReadyGameServer<S, H>
        + ReconcileStableGameServerPodIps<S>
        + ReconcileDevelopmentGameServer<S>
        + ReconcileShutdownGameServer<S>
        + Sync,
{
    let mut game_server = game_server;
    let mut changed = false;

    if let Some(updated) = self
        .reconcile_deleting_game_server(game_server.clone())
        .await?
    {
        game_server = updated;
        changed = true;
    }

    if let Some(updated) = self
        .reconcile_port_allocation_game_server(game_server.clone())
        .await?
    {
        game_server = updated;
        changed = true;
    }

    if let Some(updated) = self
        .reconcile_creating_game_server(game_server.clone())
        .await?
    {
        game_server = updated;
        changed = true;
    }

    if let Some(updated) = self
        .reconcile_starting_game_server(game_server.clone())
        .await?
    {
        game_server = updated;
        changed = true;
    }

    if let Some(updated) = self
        .reconcile_request_ready_game_server(game_server.clone())
        .await?
    {
        game_server = updated;
        changed = true;
    }

    if let Some(updated) = self
        .reconcile_stable_game_server_pod_ips(game_server.clone())
        .await?
    {
        game_server = updated;
        changed = true;
    }

    if let Some(updated) = self
        .reconcile_development_game_server(game_server.clone())
        .await?
    {
        game_server = updated;
        changed = true;
    }

    if self
        .reconcile_shutdown_game_server(game_server.clone())
        .await?
    {
        changed = true;
    }

    Ok(if changed {
        GameServerSyncOutcome::Reconciled(game_server)
    } else {
        GameServerSyncOutcome::Unchanged(game_server)
    })
}

/// Rusternetes-backed equivalent of upstream `Controller.syncGameServer`.
///
/// The current object is fetched at execution time rather than trusting a
/// potentially stale watch payload. A disappeared GameServer is a successful
/// no-op, matching the upstream informer/lister behavior.
#[cgp_fn]
#[async_trait]
pub async fn sync_game_server<S, H>(
    &self,
    #[implicit] storage: Arc<S>,
    namespace: &str,
    name: &str,
) -> Result<GameServerSyncOutcome, AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
    Self: ReconcileGameServerPipeline<S, H> + Sync,
{
    let repository = AgonesRepository::new(storage);
    let game_server = match repository.get::<GameServer>(namespace, name).await {
        Ok(game_server) => game_server,
        Err(AdapterError::Rusternetes(
            rusternetes_common::Error::NotFound(_),
        )) => return Ok(GameServerSyncOutcome::NotFound),
        Err(error) => return Err(error),
    };

    self.reconcile_game_server_pipeline(game_server).await
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use agones_core::{
        Container as CoreContainer, GameServerPort, GameServerSpec,
        GameServerState, GameServerStatus, ObjectMeta, PodSpec as CorePodSpec,
        PodTemplateSpec, PortPolicy, Protocol, DEV_ADDRESS_ANNOTATION,
        FINALIZER_NAME,
    };
    use rusternetes_common::resources::{
        Container, Node, NodeAddress, NodeStatus, PodIP, PodStatus,
    };
    use rusternetes_storage::{build_key, MemoryStorage, Storage};

    use crate::{
        build_pod, CreatingGameServerController, NoopApiHooks,
        PodBuildFeatures,
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
                ports: vec![GameServerPort {
                    name: "game".into(),
                    port_policy: Some(PortPolicy::Static),
                    container: Some("game".into()),
                    container_port: 7777,
                    host_port: 7777,
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

    async fn store_node(storage: &MemoryStorage) {
        let mut node = Node::new("node1");
        node.status = Some(NodeStatus {
            addresses: Some(vec![NodeAddress {
                address_type: "ExternalIP".into(),
                address: "203.0.113.10".into(),
            }]),
            ..Default::default()
        });
        storage
            .create(&build_key("nodes", None, "node1"), &node)
            .await
            .unwrap();
    }

    async fn store_scheduled_pod(
        storage: &MemoryStorage,
        game_server: &GameServer,
    ) {
        let mut pod = build_pod(
            game_server,
            Vec::<Container>::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.spec.as_mut().unwrap().node_name = Some("node1".into());
        pod.status = Some(PodStatus {
            pod_i_ps: Some(vec![PodIP {
                ip: "10.0.0.8".into(),
            }]),
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
    async fn creating_pipeline_can_advance_through_starting_to_scheduled() {
        let storage = MemoryStorage::new();
        store_node(&storage).await;
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Creating))
            .await
            .unwrap();
        store_scheduled_pod(&storage, &created).await;

        let controller = CreatingGameServerController::new(storage.clone());
        let outcome = controller
            .sync_game_server("default", "server")
            .await
            .unwrap();

        let reconciled = match outcome {
            GameServerSyncOutcome::Reconciled(game_server) => game_server,
            outcome => panic!("expected reconciled outcome, got {outcome:?}"),
        };
        assert_eq!(
            reconciled.status.state,
            Some(GameServerState::Scheduled)
        );
        assert_eq!(reconciled.status.node_name, "node1");
        assert_eq!(reconciled.status.address, "203.0.113.10");
        assert!(reconciled
            .status
            .addresses
            .iter()
            .any(|address| address.address == "10.0.0.8"));

        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(persisted, reconciled);
    }

    #[tokio::test]
    async fn development_creating_path_skips_pod_states_and_becomes_ready() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fixture = game_server(GameServerState::Creating);
        fixture.metadata.annotations = BTreeMap::from([(
            DEV_ADDRESS_ANNOTATION.to_owned(),
            "198.51.100.7".to_owned(),
        )]);
        repository.create(&fixture).await.unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let outcome = controller
            .sync_game_server("default", "server")
            .await
            .unwrap();

        let reconciled = match outcome {
            GameServerSyncOutcome::Reconciled(game_server) => game_server,
            outcome => panic!("expected reconciled outcome, got {outcome:?}"),
        };
        assert_eq!(reconciled.status.state, Some(GameServerState::Ready));
        assert_eq!(reconciled.status.address, "198.51.100.7");
        assert_eq!(reconciled.status.node_name, "198.51.100.7");

        let pod_result: rusternetes_common::Result<
            rusternetes_common::resources::Pod,
        > = storage
            .get(&build_key("pods", Some("default"), "server"))
            .await;
        assert!(matches!(
            pod_result,
            Err(rusternetes_common::Error::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn ready_without_late_pod_ip_changes_is_unchanged() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut ready = game_server(GameServerState::Ready);
        ready.status.node_name = "node1".into();
        repository.create(&ready).await.unwrap();

        let controller = CreatingGameServerController::new(storage);
        let outcome = controller
            .sync_game_server("default", "server")
            .await
            .unwrap();

        assert!(matches!(
            outcome,
            GameServerSyncOutcome::Unchanged(game_server)
                if game_server.status.state == Some(GameServerState::Ready)
        ));
    }

    #[tokio::test]
    async fn shutdown_pipeline_requests_background_deletion() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut shutdown = game_server(GameServerState::Shutdown);
        shutdown.metadata.finalizers = vec![FINALIZER_NAME.into()];
        repository.create(&shutdown).await.unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let outcome = controller
            .sync_game_server("default", "server")
            .await
            .unwrap();
        assert!(outcome.changed());

        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert!(persisted.metadata.deletion_timestamp.is_some());
        assert_eq!(
            persisted.status.state,
            Some(GameServerState::Shutdown)
        );
    }

    #[tokio::test]
    async fn missing_gameserver_is_successful_not_found_noop() {
        let controller = CreatingGameServerController::new(
            MemoryStorage::new(),
        );
        assert_eq!(
            controller
                .sync_game_server("default", "missing")
                .await
                .unwrap(),
            GameServerSyncOutcome::NotFound
        );
    }
}
