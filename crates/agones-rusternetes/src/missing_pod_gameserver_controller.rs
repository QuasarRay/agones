use std::sync::Arc;

use agones_core::{GameServer, GameServerState};
use cgp::prelude::*;
use rusternetes_common::resources::Pod;
use rusternetes_common::types::Phase;
use rusternetes_storage::{build_key, Storage};

use crate::{
    AdapterError, AgonesRepository, GAME_SERVER_LABEL_ROLE, ROLE_LABEL,
};

/// Mirrors upstream gameservers.isBeforePodCreated.
pub fn is_before_pod_created(game_server: &GameServer) -> bool {
    matches!(
        game_server.status.state.as_ref(),
        Some(
            GameServerState::PortAllocation
                | GameServerState::Creating
                | GameServerState::Starting
        )
    )
}

/// Mirrors upstream gameservers.isGameServerPod.
///
/// This is intentionally weaker than game_server_pod(): upstream MissingPod
/// checks only the Agones role label plus the first controller owner's kind.
/// It does not compare the controller UID to the GameServer UID.
pub fn is_game_server_pod(pod: &Pod) -> bool {
    let has_role = pod
        .metadata
        .labels
        .as_ref()
        .and_then(|labels| labels.get(ROLE_LABEL))
        .is_some_and(|role| role == GAME_SERVER_LABEL_ROLE);
    if !has_role {
        return false;
    }

    pod.metadata
        .owner_references
        .as_ref()
        .and_then(|owners| {
            owners
                .iter()
                .find(|owner| owner.controller == Some(true))
        })
        .is_some_and(|owner| owner.kind == "GameServer")
}

fn pod_failed(pod: &Pod) -> bool {
    pod.status
        .as_ref()
        .and_then(|status| status.phase.as_ref())
        == Some(&Phase::Failed)
}

/// Mirrors the MissingPodController informer-side enqueue predicate.
///
/// Storage lookup failures are treated as enqueue-worthy, matching upstream:
/// the sync pass will retry the control-plane access and surface the error.
pub async fn should_enqueue_missing_pod_game_server<S: Storage>(
    storage: &S,
    game_server: &GameServer,
) -> bool {
    if game_server.get_dev_address().is_some()
        || is_before_pod_created(game_server)
        || game_server.is_being_deleted()
        || matches!(
            game_server.status.state.as_ref(),
            Some(GameServerState::Unhealthy | GameServerState::Error)
        )
    {
        return false;
    }

    if game_server.metadata.name.is_empty()
        || game_server.metadata.namespace.is_empty()
    {
        return true;
    }

    let pod_key = build_key(
        "pods",
        Some(game_server.metadata.namespace.as_str()),
        game_server.metadata.name.as_str(),
    );

    match storage.get::<Pod>(&pod_key).await {
        Ok(pod) => !is_game_server_pod(&pod) || pod_failed(&pod),
        Err(_) => true,
    }
}

/// CGP capability for the core upstream MissingPodController sync pass.
///
/// A missing, non-GameServer, or Failed Pod moves the current GameServer to
/// Unhealthy. A valid non-Failed GameServer Pod is a no-op. The GameServer is
/// re-read after the Pod check so deletion/unhealthy transitions that happened
/// after enqueue are respected.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_missing_pod_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    game_server: GameServer,
) -> Result<Option<GameServer>, AdapterError>
where
    S: Storage + 'static,
{
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

    match storage.get::<Pod>(&pod_key).await {
        Ok(pod) if is_game_server_pod(&pod) && !pod_failed(&pod) => {
            return Ok(None);
        }
        Ok(_) | Err(rusternetes_common::Error::NotFound(_)) => {}
        Err(error) => return Err(error.into()),
    }

    let repository = AgonesRepository::new(storage.clone());
    let mut current = match repository
        .get::<GameServer>(
            game_server.metadata.namespace.as_str(),
            game_server.metadata.name.as_str(),
        )
        .await
    {
        Ok(current) => current,
        Err(AdapterError::Rusternetes(
            rusternetes_common::Error::NotFound(_),
        )) => return Ok(None),
        Err(error) => return Err(error),
    };

    if current.is_being_deleted()
        || current.status.state == Some(GameServerState::Unhealthy)
    {
        return Ok(None);
    }

    current.status.state = Some(GameServerState::Unhealthy);
    repository.update(&current).await.map(Some)
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerSpec, GameServerStatus,
        ObjectMeta, PodSpec as CorePodSpec, PodTemplateSpec,
        DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_common::resources::PodStatus;
    use rusternetes_storage::{MemoryStorage, Storage};

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
                state: Some(state),
                ..Default::default()
            },
        }
    }

    async fn store_pod(
        storage: &MemoryStorage,
        game_server: &GameServer,
        phase: Option<Phase>,
    ) -> Pod {
        let mut pod = build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.status = Some(PodStatus {
            phase,
            ..Default::default()
        });

        storage
            .create(
                &build_key("pods", Some("default"), "server"),
                &pod,
            )
            .await
            .unwrap()
    }

    #[test]
    fn gameserver_pod_detection_uses_role_and_first_controller_kind() {
        let game_server = game_server(GameServerState::Ready);
        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        assert!(is_game_server_pod(&pod));

        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            "different-uid".into();
        assert!(
            is_game_server_pod(&pod),
            "MissingPod upstream does not compare controller UID"
        );

        pod.metadata.owner_references.as_mut().unwrap()[0].kind =
            "ReplicaSet".into();
        assert!(!is_game_server_pod(&pod));

        pod.metadata.owner_references.as_mut().unwrap()[0].kind =
            "GameServer".into();
        pod.metadata
            .labels
            .as_mut()
            .unwrap()
            .remove(ROLE_LABEL);
        assert!(!is_game_server_pod(&pod));
    }

    #[tokio::test]
    async fn enqueue_filter_matches_upstream_preconditions() {
        let storage = MemoryStorage::new();

        for state in [
            GameServerState::PortAllocation,
            GameServerState::Creating,
            GameServerState::Starting,
            GameServerState::Unhealthy,
            GameServerState::Error,
            GameServerState::Shutdown,
        ] {
            assert!(
                !should_enqueue_missing_pod_game_server(
                    &storage,
                    &game_server(state),
                )
                .await,
                "state should not enqueue"
            );
        }

        let mut deleting = game_server(GameServerState::Ready);
        deleting.metadata.deletion_timestamp =
            Some("2026-10-17T00:00:00Z".into());
        assert!(
            !should_enqueue_missing_pod_game_server(&storage, &deleting)
                .await
        );

        let mut development = game_server(GameServerState::Ready);
        development.metadata.annotations.insert(
            DEV_ADDRESS_ANNOTATION.into(),
            "203.0.113.7".into(),
        );
        assert!(
            !should_enqueue_missing_pod_game_server(
                &storage,
                &development,
            )
            .await
        );

        assert!(
            should_enqueue_missing_pod_game_server(
                &storage,
                &game_server(GameServerState::Ready),
            )
            .await
        );
    }

    #[tokio::test]
    async fn healthy_gameserver_pod_is_noop() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();
        store_pod(&storage, &created, Some(Phase::Running)).await;

        assert!(
            !should_enqueue_missing_pod_game_server(&storage, &created).await
        );

        let controller = CreatingGameServerController::new(storage);
        assert!(
            controller
                .reconcile_missing_pod_game_server(created)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn missing_pod_moves_gameserver_to_unhealthy() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let updated = controller
            .reconcile_missing_pod_game_server(created)
            .await
            .unwrap()
            .expect("missing Pod should move GameServer to Unhealthy");

        assert_eq!(
            updated.status.state,
            Some(GameServerState::Unhealthy)
        );
        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(persisted, updated);
    }

    #[tokio::test]
    async fn failed_pod_moves_gameserver_to_unhealthy() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();
        store_pod(&storage, &created, Some(Phase::Failed)).await;

        assert!(
            should_enqueue_missing_pod_game_server(&storage, &created).await
        );

        let controller = CreatingGameServerController::new(storage);
        let updated = controller
            .reconcile_missing_pod_game_server(created)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            updated.status.state,
            Some(GameServerState::Unhealthy)
        );
    }

    #[tokio::test]
    async fn non_gameserver_pod_is_treated_as_missing() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Allocated))
            .await
            .unwrap();
        let mut pod =
            store_pod(&storage, &created, Some(Phase::Running)).await;
        pod.metadata
            .labels
            .as_mut()
            .unwrap()
            .remove(ROLE_LABEL);
        storage
            .update(
                &build_key("pods", Some("default"), "server"),
                &pod,
            )
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage);
        let updated = controller
            .reconcile_missing_pod_game_server(created)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            updated.status.state,
            Some(GameServerState::Unhealthy)
        );
    }

    #[tokio::test]
    async fn missing_gameserver_after_pod_check_is_noop() {
        let storage = MemoryStorage::new();
        let snapshot = game_server(GameServerState::Ready);
        let controller = CreatingGameServerController::new(storage);

        assert!(
            controller
                .reconcile_missing_pod_game_server(snapshot)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn current_deleting_or_unhealthy_state_is_noop() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let queued = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();

        let mut deleting = queued.clone();
        deleting.metadata.deletion_timestamp =
            Some("2026-10-17T00:00:00Z".into());
        repository.update(&deleting).await.unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_missing_pod_game_server(queued.clone())
                .await
                .unwrap()
                .is_none()
        );

        let mut unhealthy = deleting;
        unhealthy.metadata.deletion_timestamp = None;
        unhealthy.status.state = Some(GameServerState::Unhealthy);
        repository.update(&unhealthy).await.unwrap();

        assert!(
            controller
                .reconcile_missing_pod_game_server(queued)
                .await
                .unwrap()
                .is_none()
        );
    }
}
