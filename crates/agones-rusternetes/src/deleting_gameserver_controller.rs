use std::sync::Arc;

use agones_core::{
    GameServer, FINALIZER_NAME, GROUP_NAME,
};
use cgp::prelude::*;
use rusternetes_storage::Storage;

use crate::{
    game_server_pod, request_agones_resource_deletion, request_pod_deletion,
    AdapterError, AgonesRepository, DeletionRequestOutcome, PortAllocator,
};

/// CGP capability for upstream syncGameServerDeletionTimestamp.
///
/// A deleting GameServer first tears down its owned Pod. Agones finalizers are
/// removed only after that Pod is no longer present. Development GameServers
/// have no managed Pod and therefore proceed directly to finalizer removal.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_deleting_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] port_allocator: Arc<PortAllocator>,
    game_server: GameServer,
) -> Result<Option<GameServer>, AdapterError>
where
    S: Storage + 'static,
{
    if game_server.metadata.deletion_timestamp.is_none() {
        return Ok(None);
    }

    let is_development = game_server.get_dev_address().is_some();
    let pod = match game_server_pod(storage.as_ref(), &game_server).await {
        Ok(pod) => Some(pod),
        Err(AdapterError::Rusternetes(rusternetes_common::Error::NotFound(_))) => None,
        Err(error) => return Err(error),
    };

    if let Some(pod) = pod.filter(|pod| !is_development && !pod.metadata.name.is_empty()) {
        if pod.metadata.deletion_timestamp.is_none() {
            request_pod_deletion(storage.as_ref(), &pod).await?;
        }

        // Upstream never removes the GameServer finalizer in the same pass in
        // which an owned Pod still exists, even when Pod DELETE completes
        // immediately. A later reconciliation observes that it is gone.
        return Ok(None);
    }

    let mut finalized = game_server;
    finalized
        .metadata
        .finalizers
        .retain(|finalizer| finalizer != GROUP_NAME && finalizer != FINALIZER_NAME);

    let repository = AgonesRepository::new(storage.clone());
    let updated = repository.update(&finalized).await?;

    // Kubernetes deletes an object automatically once deletionTimestamp is set
    // and the finalizer list becomes empty. Direct Rusternetes Storage updates
    // do not perform that API-server completion step, so explicitly reuse the
    // delete-request primitive here to finish deletion only when no unrelated
    // finalizers remain.
    if updated.metadata.finalizers.is_empty() {
        let outcome =
            request_agones_resource_deletion(storage.as_ref(), &updated).await?;
        debug_assert_eq!(outcome, DeletionRequestOutcome::Deleted);
        if outcome == DeletionRequestOutcome::Deleted {
            port_allocator.deallocate(&updated).await;
        }
    }

    Ok(Some(updated))
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerSpec, GameServerState,
        GameServerStatus, ObjectMeta, PodSpec as CorePodSpec, PodTemplateSpec,
        DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_common::resources::Pod;
    use rusternetes_storage::{build_key, MemoryStorage, Storage};

    use crate::{
        build_pod, CreatingGameServerController, NoopApiHooks, PodBuildFeatures,
    };

    use super::*;

    const OTHER_FINALIZER: &str = "example.com/other";

    fn deleting_game_server(finalizers: Vec<String>) -> GameServer {
        GameServer {
            metadata: ObjectMeta {
                name: "server".into(),
                namespace: "default".into(),
                finalizers,
                deletion_timestamp: Some("2026-10-17T00:00:00Z".into()),
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
                state: Some(GameServerState::Shutdown),
                ..Default::default()
            },
        }
    }

    async fn store_owned_pod(
        storage: &MemoryStorage,
        game_server: &GameServer,
        grace_period: Option<i64>,
    ) -> Pod {
        let mut pod = build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.spec.as_mut().unwrap().termination_grace_period_seconds = grace_period;
        storage
            .create(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn owned_pod_is_marked_for_deletion_before_finalizer_release() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&deleting_game_server(vec![FINALIZER_NAME.into()]))
            .await
            .unwrap();
        store_owned_pod(&storage, &created, None).await;

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_deleting_game_server(created.clone())
                .await
                .unwrap()
                .is_none()
        );

        let pod: Pod = storage
            .get(&build_key("pods", Some("default"), "server"))
            .await
            .unwrap();
        assert!(pod.metadata.deletion_timestamp.is_some());
        assert_eq!(pod.metadata.deletion_grace_period_seconds, Some(30));

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(
            persisted.metadata.finalizers,
            vec![FINALIZER_NAME.to_owned()]
        );
    }

    #[tokio::test]
    async fn already_deleting_owned_pod_only_waits() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&deleting_game_server(vec![FINALIZER_NAME.into()]))
            .await
            .unwrap();
        let mut pod = store_owned_pod(&storage, &created, None).await;
        pod.metadata.deletion_timestamp = Some(chrono::Utc::now());
        pod.metadata.deletion_grace_period_seconds = Some(30);
        let pod_key = build_key("pods", Some("default"), "server");
        let pod: Pod = storage.update(&pod_key, &pod).await.unwrap();
        let before = serde_json::to_value(&pod).unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_deleting_game_server(created)
                .await
                .unwrap()
                .is_none()
        );

        let after: Pod = storage.get(&pod_key).await.unwrap();
        assert_eq!(serde_json::to_value(after).unwrap(), before);
        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(
            persisted.metadata.finalizers,
            vec![FINALIZER_NAME.to_owned()]
        );
    }

    #[tokio::test]
    async fn finalizer_is_released_only_after_owned_pod_is_gone() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&deleting_game_server(vec![
                GROUP_NAME.into(),
                FINALIZER_NAME.into(),
            ]))
            .await
            .unwrap();
        let pod = store_owned_pod(&storage, &created, Some(0)).await;

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_deleting_game_server(created.clone())
                .await
                .unwrap()
                .is_none()
        );

        assert!(matches!(
            storage
                .get::<Pod>(&build_key("pods", Some("default"), "server"))
                .await,
            Err(rusternetes_common::Error::NotFound(_))
        ));
        let still_finalized: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(still_finalized.metadata.finalizers.len(), 2);

        let finalized = controller
            .reconcile_deleting_game_server(still_finalized)
            .await
            .unwrap()
            .expect("missing Pod should release Agones finalizers");
        assert!(finalized.metadata.finalizers.is_empty());

        assert!(matches!(
            repository.get::<GameServer>("default", "server").await,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_)
            ))
        ));

        // Keep the first Pod value used so the test also proves the immediate
        // delete path was acting on the owned Pod.
        assert_eq!(pod.metadata.owner_references.as_ref().unwrap()[0].uid, created.metadata.uid);
    }

    #[tokio::test]
    async fn unrelated_finalizer_is_preserved_and_blocks_final_deletion() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&deleting_game_server(vec![
                GROUP_NAME.into(),
                FINALIZER_NAME.into(),
                OTHER_FINALIZER.into(),
            ]))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let updated = controller
            .reconcile_deleting_game_server(created)
            .await
            .unwrap()
            .expect("no Pod should release only Agones finalizers");

        assert_eq!(
            updated.metadata.finalizers,
            vec![OTHER_FINALIZER.to_owned()]
        );
        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(
            persisted.metadata.finalizers,
            vec![OTHER_FINALIZER.to_owned()]
        );
    }

    #[tokio::test]
    async fn development_gameserver_releases_finalizer_without_pod() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fixture = deleting_game_server(vec![FINALIZER_NAME.into()]);
        fixture.metadata.annotations.insert(
            DEV_ADDRESS_ANNOTATION.into(),
            "203.0.113.7".into(),
        );
        let created = repository.create(&fixture).await.unwrap();

        let controller = CreatingGameServerController::new(storage);
        let updated = controller
            .reconcile_deleting_game_server(created)
            .await
            .unwrap()
            .expect("development GameServer should release finalizer");
        assert!(updated.metadata.finalizers.is_empty());

        assert!(matches!(
            repository.get::<GameServer>("default", "server").await,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_)
            ))
        ));
    }

    #[tokio::test]
    async fn foreign_same_name_pod_is_not_deleted_or_blocking() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&deleting_game_server(vec![FINALIZER_NAME.into()]))
            .await
            .unwrap();
        let mut foreign = store_owned_pod(&storage, &created, None).await;
        foreign.metadata.owner_references.as_mut().unwrap()[0].uid =
            "different-owner".into();
        let pod_key = build_key("pods", Some("default"), "server");
        let foreign: Pod = storage.update(&pod_key, &foreign).await.unwrap();
        let before = serde_json::to_value(&foreign).unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let updated = controller
            .reconcile_deleting_game_server(created)
            .await
            .unwrap()
            .expect("foreign Pod should be treated as missing");
        assert!(updated.metadata.finalizers.is_empty());

        let after: Pod = storage.get(&pod_key).await.unwrap();
        assert_eq!(serde_json::to_value(after).unwrap(), before);
    }

    #[tokio::test]
    async fn pod_delete_failure_keeps_gameserver_finalizer() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&deleting_game_server(vec![FINALIZER_NAME.into()]))
            .await
            .unwrap();
        store_owned_pod(&storage, &created, None).await;
        storage.inject_conflicts(2);

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_deleting_game_server(created)
                .await
                .is_err()
        );

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(
            persisted.metadata.finalizers,
            vec![FINALIZER_NAME.to_owned()]
        );
    }

    #[tokio::test]
    async fn no_deletion_timestamp_is_noop() {
        let storage = MemoryStorage::new();
        let mut game_server =
            deleting_game_server(vec![FINALIZER_NAME.into()]);
        game_server.metadata.deletion_timestamp = None;

        let controller = CreatingGameServerController::new(storage);
        assert!(
            controller
                .reconcile_deleting_game_server(game_server)
                .await
                .unwrap()
                .is_none()
        );
    }
}
