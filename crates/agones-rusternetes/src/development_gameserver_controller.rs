use std::sync::Arc;

use agones_core::GameServer;
use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::{AdapterError, AgonesRepository, AgonesWatchEvent};

/// Rusternetes persistence boundary for the isolated development-GameServer
/// reconciliation path.
///
/// The state transition itself lives in agones-core. This type only connects
/// typed watch events to backend-generic Rusternetes storage.
pub struct DevelopmentGameServerController<S> {
    repository: AgonesRepository<S>,
}

impl<S> DevelopmentGameServerController<S> {
    pub const fn new(storage: S) -> Self {
        Self {
            repository: AgonesRepository::new(storage),
        }
    }

    pub fn repository(&self) -> &AgonesRepository<S> {
        &self.repository
    }
}

/// CGP capability for upstream `syncDevelopmentGameServer`.
///
/// The state transition remains pure in agones-core while persistence is
/// supplied implicitly through the shared GameServer controller context.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_development_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    game_server: GameServer,
) -> Result<Option<GameServer>, AdapterError>
where
    S: Storage + 'static,
{
    let mut game_server = game_server;
    if !game_server.reconcile_development_status() {
        return Ok(None);
    }

    AgonesRepository::new(storage)
        .update(&game_server)
        .await
        .map(Some)
}

impl<S: Storage> DevelopmentGameServerController<S> {
    pub async fn reconcile_game_server(
        &self,
        mut game_server: GameServer,
    ) -> Result<Option<GameServer>, AdapterError> {
        if !game_server.reconcile_development_status() {
            return Ok(None);
        }

        self.repository.update(&game_server).await.map(Some)
    }

    pub async fn reconcile_event(
        &self,
        event: AgonesWatchEvent<GameServer>,
    ) -> Result<Option<GameServer>, AdapterError> {
        match event {
            AgonesWatchEvent::Added { resource, .. }
            | AgonesWatchEvent::Modified { resource, .. } => {
                self.reconcile_game_server(resource).await
            }
            AgonesWatchEvent::Deleted { .. } => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use agones_core::{
        GameServerPort, GameServerState, ObjectMeta, PortPolicy, Protocol,
        DEV_ADDRESS_ANNOTATION,
    };
    use futures::StreamExt;

    use super::*;

    async fn next_event(
        stream: &mut crate::AgonesWatchStream<GameServer>,
    ) -> AgonesWatchEvent<GameServer> {
        tokio::time::timeout(Duration::from_secs(2), stream.next())
            .await
            .expect("development GameServer watch timed out")
            .expect("development GameServer watch ended")
            .expect("development GameServer watch failed")
    }

    #[tokio::test]
    async fn typed_watch_event_reconciles_and_persists_development_gameserver() {
        let storage = rusternetes_storage::MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let controller = DevelopmentGameServerController::new(storage.clone());
        let mut watch = repository.watch::<GameServer>("default").await.unwrap();

        let game_server = GameServer {
            metadata: ObjectMeta {
                name: "dev-server".into(),
                namespace: "default".into(),
                annotations: std::collections::BTreeMap::from([(
                    DEV_ADDRESS_ANNOTATION.to_owned(),
                    "203.0.113.7".to_owned(),
                )]),
                ..Default::default()
            },
            spec: agones_core::GameServerSpec {
                ports: vec![GameServerPort {
                    name: "game".into(),
                    port_policy: Some(PortPolicy::Static),
                    container_port: 7777,
                    host_port: 7777,
                    protocol: Some(Protocol::Udp),
                    ..Default::default()
                }],
                ..Default::default()
            },
            status: agones_core::GameServerStatus {
                state: Some(GameServerState::Creating),
                ..Default::default()
            },
        };

        let created = repository.create(&game_server).await.unwrap();
        let event = next_event(&mut watch).await;
        let reconciled = controller
            .reconcile_event(event)
            .await
            .unwrap()
            .expect("Creating development GameServer should update");

        assert_eq!(reconciled.status.state, Some(GameServerState::Ready));
        assert_eq!(reconciled.status.address, "203.0.113.7");
        assert_eq!(reconciled.status.node_name, "203.0.113.7");
        assert_eq!(reconciled.status.ports[0].port, 7777);
        assert_eq!(reconciled.metadata.uid, created.metadata.uid);

        let persisted: GameServer = repository
            .get("default", "dev-server")
            .await
            .unwrap();
        assert_eq!(persisted, reconciled);

        let modified = next_event(&mut watch).await;
        assert!(matches!(modified, AgonesWatchEvent::Modified { .. }));
        assert!(
            controller
                .reconcile_event(modified)
                .await
                .unwrap()
                .is_none(),
            "Ready development GameServer must not trigger an update loop"
        );
    }

    #[tokio::test]
    async fn deleted_watch_event_never_recreates_development_gameserver() {
        let controller =
            DevelopmentGameServerController::new(rusternetes_storage::MemoryStorage::new());
        let resource = GameServer {
            status: agones_core::GameServerStatus {
                state: Some(GameServerState::Creating),
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(
            controller
                .reconcile_event(AgonesWatchEvent::Deleted {
                    key: "/registry/agones_dev_gameservers/default/dev-server".into(),
                    resource,
                })
                .await
                .unwrap()
                .is_none()
        );
    }
}
