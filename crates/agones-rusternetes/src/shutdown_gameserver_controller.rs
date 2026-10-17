use std::sync::Arc;

use agones_core::{GameServer, GameServerState};
use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::{
    request_agones_resource_deletion, AdapterError, DeletionRequestOutcome,
};

/// CGP capability for upstream syncGameServerShutdownState.
///
/// The Go controller issues a background DELETE request. At the direct
/// Rusternetes-storage boundary used by this reimplementation, that request is
/// represented by request_agones_resource_deletion so Kubernetes finalizers are
/// respected rather than bypassed by a raw storage delete.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_shutdown_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    game_server: GameServer,
) -> Result<bool, AdapterError>
where
    S: Storage + 'static,
{
    if game_server.status.state != Some(GameServerState::Shutdown)
        || game_server.metadata.deletion_timestamp.is_some()
    {
        return Ok(false);
    }

    let outcome =
        request_agones_resource_deletion(storage.as_ref(), &game_server).await?;
    debug_assert!(matches!(
        outcome,
        DeletionRequestOutcome::Deleted
            | DeletionRequestOutcome::MarkedForDeletion
    ));
    Ok(true)
}

#[cfg(test)]
mod tests {
    use agones_core::{
        GameServerSpec, GameServerStatus, ObjectMeta, FINALIZER_NAME,
        DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::{AgonesRepository, CreatingGameServerController};

    use super::*;

    fn shutdown_game_server(finalizers: Vec<String>) -> GameServer {
        GameServer {
            metadata: ObjectMeta {
                name: "server".into(),
                namespace: "default".into(),
                finalizers,
                ..Default::default()
            },
            spec: GameServerSpec::default(),
            status: GameServerStatus {
                state: Some(GameServerState::Shutdown),
                ..Default::default()
            },
        }
    }

    #[tokio::test]
    async fn shutdown_with_agones_finalizer_starts_deletion() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&shutdown_game_server(vec![FINALIZER_NAME.into()]))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_shutdown_game_server(created)
                .await
                .unwrap()
        );

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert!(persisted.metadata.deletion_timestamp.is_some());
        assert_eq!(
            persisted.metadata.finalizers,
            vec![FINALIZER_NAME.to_owned()]
        );
        assert_eq!(persisted.status.state, Some(GameServerState::Shutdown));
    }

    #[tokio::test]
    async fn shutdown_without_finalizers_is_deleted_immediately() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&shutdown_game_server(Vec::new()))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage);
        assert!(
            controller
                .reconcile_shutdown_game_server(created)
                .await
                .unwrap()
        );

        assert!(matches!(
            repository.get::<GameServer>("default", "server").await,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_)
            ))
        ));
    }

    #[tokio::test]
    async fn development_shutdown_still_requests_deletion() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fixture = shutdown_game_server(vec![FINALIZER_NAME.into()]);
        fixture.metadata.annotations.insert(
            DEV_ADDRESS_ANNOTATION.into(),
            "203.0.113.7".into(),
        );
        let created = repository.create(&fixture).await.unwrap();

        let controller = CreatingGameServerController::new(storage);
        assert!(
            controller
                .reconcile_shutdown_game_server(created)
                .await
                .unwrap()
        );

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert!(persisted.metadata.deletion_timestamp.is_some());
    }

    #[tokio::test]
    async fn non_shutdown_and_already_deleting_gameservers_are_noops() {
        let storage = MemoryStorage::new();
        let controller = CreatingGameServerController::new(storage);

        let mut ready = shutdown_game_server(vec![FINALIZER_NAME.into()]);
        ready.status.state = Some(GameServerState::Ready);
        assert!(
            !controller
                .reconcile_shutdown_game_server(ready)
                .await
                .unwrap()
        );

        let mut deleting = shutdown_game_server(vec![FINALIZER_NAME.into()]);
        deleting.metadata.deletion_timestamp =
            Some("2026-10-17T00:00:00Z".into());
        assert!(
            !controller
                .reconcile_shutdown_game_server(deleting)
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn persistence_failure_does_not_report_deletion_started() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&shutdown_game_server(vec![FINALIZER_NAME.into()]))
            .await
            .unwrap();
        storage.inject_conflicts(1);

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_shutdown_game_server(created)
                .await
                .is_err()
        );

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert!(persisted.metadata.deletion_timestamp.is_none());
    }
}
