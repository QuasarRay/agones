use std::sync::Arc;

use agones_core::{GameServer, GameServerState};
use cgp::prelude::*;
use rusternetes_common::resources::{ContainerState, EventType, Pod};
use rusternetes_common::types::Phase;
use rusternetes_storage::{build_key, Storage};

use crate::{
    game_server_event::record_game_server_event, is_before_pod_created,
    is_game_server_pod, AdapterError, AgonesRepository, StateUpdateLimiter,
    GAME_SERVER_CONTAINER_ANNOTATION,
};

/// Returns true if the game-server container has exited successfully and
/// Kubernetes cannot restart it, while another regular container keeps the Pod
/// from reaching Succeeded.
///
/// This mirrors upstream gameservers.gameServerContainerCompleted. Failed Pods
/// are intentionally left to HealthController so Shutdown cannot race over an
/// Unhealthy transition.
pub fn game_server_container_completed(pod: &Pod) -> bool {
    if pod.status.as_ref().and_then(|status| status.phase.as_ref())
        == Some(&Phase::Failed)
    {
        return false;
    }

    let Some(spec) = pod.spec.as_ref() else {
        return false;
    };
    if spec.containers.len() < 2
        || spec.restart_policy.as_deref() == Some("Always")
    {
        return false;
    }

    let Some(container_name) = pod
        .metadata
        .annotations
        .as_ref()
        .and_then(|annotations| annotations.get(GAME_SERVER_CONTAINER_ANNOTATION))
    else {
        return false;
    };

    pod.status
        .as_ref()
        .and_then(|status| status.container_statuses.as_ref())
        .and_then(|statuses| {
            statuses
                .iter()
                .find(|status| status.name == *container_name)
        })
        .is_some_and(|status| {
            matches!(
                status.state.as_ref(),
                Some(ContainerState::Terminated { exit_code: 0, .. })
            )
        })
}

/// Mirrors upstream gameservers.podCompleted.
pub fn pod_completed(pod: &Pod) -> bool {
    pod.status.as_ref().and_then(|status| status.phase.as_ref())
        == Some(&Phase::Succeeded)
        || game_server_container_completed(pod)
}

/// Mirrors the GameServer-informer recovery predicate in SucceededController.
///
/// Pod add/update handlers can use is_game_server_pod + pod_completed directly.
/// This helper represents the resync path that re-checks a completed Pod after a
/// missed event or controller restart.
pub async fn should_enqueue_succeeded_game_server<S: Storage>(
    storage: &S,
    game_server: &GameServer,
) -> bool {
    if game_server.get_dev_address().is_some()
        || game_server.is_being_deleted()
        || game_server
            .status
            .state
            .as_ref()
            .is_some_and(GameServerState::is_terminal)
        || is_before_pod_created(game_server)
        || game_server.metadata.name.is_empty()
        || game_server.metadata.namespace.is_empty()
    {
        return false;
    }

    let key = build_key(
        "pods",
        Some(game_server.metadata.namespace.as_str()),
        game_server.metadata.name.as_str(),
    );

    storage
        .get::<Pod>(&key)
        .await
        .is_ok_and(|pod| is_game_server_pod(&pod) && pod_completed(&pod))
}

/// CGP capability for upstream SucceededController.syncGameServer.
///
/// A completed, non-terminating GameServer Pod moves a non-terminal GameServer
/// to Shutdown. The GameServer is re-read after the Pod check, the terminal
/// write consumes the controller's dedicated state-update token, and the
/// successful transition records the same Normal event as upstream.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_succeeded_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] shutdown_limiter: Arc<StateUpdateLimiter>,
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
    let pod = match storage.get::<Pod>(&pod_key).await {
        Ok(pod) => pod,
        Err(rusternetes_common::Error::NotFound(_)) => return Ok(None),
        Err(error) => return Err(error.into()),
    };

    if !is_game_server_pod(&pod)
        || !pod_completed(&pod)
        || pod.metadata.deletion_timestamp.is_some()
    {
        return Ok(None);
    }

    let reason = if pod
        .status
        .as_ref()
        .and_then(|status| status.phase.as_ref())
        == Some(&Phase::Succeeded)
    {
        "Pod is in Succeeded state"
    } else {
        "Game server container exited cleanly"
    };

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
        || current
            .status
            .state
            .as_ref()
            .is_some_and(GameServerState::is_terminal)
    {
        return Ok(None);
    }

    shutdown_limiter.wait().await?;

    current.status.state = Some(GameServerState::Shutdown);
    let updated = repository.update(&current).await?;
    record_game_server_event(
        storage.as_ref(),
        &updated,
        "succeeded-controller",
        EventType::Normal,
        "Shutdown",
        reason,
    )
    .await;
    Ok(Some(updated))
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerSpec, GameServerStatus, ObjectMeta,
        PodSpec as CorePodSpec, PodTemplateSpec, DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_common::resources::{
        ContainerStatus, Event, PodStatus,
    };
    use rusternetes_storage::{build_prefix, MemoryStorage};

    use crate::{
        build_pod, CreatingGameServerController, NoopApiHooks, PodBuildFeatures,
        ROLE_LABEL,
    };

    use super::*;

    fn game_server(state: GameServerState, extra_container: bool) -> GameServer {
        let mut containers = vec![CoreContainer {
            name: "game".into(),
            image: "game:v1".into(),
            ..Default::default()
        }];
        if extra_container {
            containers.push(CoreContainer {
                name: "long-lived".into(),
                image: "side:v1".into(),
                ..Default::default()
            });
        }

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
                        containers: containers.into(),
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

    fn terminated_status(name: &str, exit_code: i32) -> ContainerStatus {
        serde_json::from_value(serde_json::json!({
            "name": name,
            "ready": false,
            "restartCount": 0,
            "state": {
                "terminated": {
                    "exitCode": exit_code
                }
            }
        }))
        .unwrap()
    }

    fn running_status(name: &str) -> ContainerStatus {
        serde_json::from_value(serde_json::json!({
            "name": name,
            "ready": true,
            "restartCount": 0,
            "state": {
                "running": {}
            }
        }))
        .unwrap()
    }

    fn build_owned_pod(
        game_server: &GameServer,
        sidecar_semantics: bool,
    ) -> Pod {
        build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures {
                sidecar_containers: sidecar_semantics,
                ..Default::default()
            },
            &NoopApiHooks,
        )
        .unwrap()
    }

    async fn store_pod(storage: &MemoryStorage, pod: &Pod) {
        storage
            .create(
                &build_key(
                    "pods",
                    pod.metadata.namespace.as_deref(),
                    pod.metadata.name.as_str(),
                ),
                pod,
            )
            .await
            .unwrap();
    }

    #[test]
    fn pod_completion_matches_upstream_rules() {
        let single = game_server(GameServerState::Allocated, false);
        let mut pod = build_owned_pod(&single, false);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Running),
            ..Default::default()
        });
        assert!(!pod_completed(&pod));

        pod.status.as_mut().unwrap().phase = Some(Phase::Succeeded);
        assert!(pod_completed(&pod));

        let multi = game_server(GameServerState::Allocated, true);
        let mut pod = build_owned_pod(&multi, true);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Running),
            container_statuses: Some(vec![
                terminated_status("game", 0),
                running_status("long-lived"),
            ]),
            ..Default::default()
        });
        assert!(game_server_container_completed(&pod));
        assert!(pod_completed(&pod));

        if let Some(ContainerState::Terminated { exit_code, .. }) =
            pod.status.as_mut().unwrap().container_statuses.as_mut().unwrap()[0]
                .state
                .as_mut()
        {
            *exit_code = 1;
        }
        assert!(!game_server_container_completed(&pod));

        pod.status.as_mut().unwrap().container_statuses.as_mut().unwrap()[0] =
            terminated_status("game", 0);
        pod.status.as_mut().unwrap().phase = Some(Phase::Failed);
        assert!(!game_server_container_completed(&pod));

        pod.status.as_mut().unwrap().phase = Some(Phase::Running);
        pod.spec.as_mut().unwrap().restart_policy = Some("Always".into());
        assert!(!game_server_container_completed(&pod));
    }

    #[tokio::test]
    async fn succeeded_pod_moves_gameserver_to_shutdown_and_records_event() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Allocated, false))
            .await
            .unwrap();

        let mut pod = build_owned_pod(&created, false);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Succeeded),
            ..Default::default()
        });
        store_pod(&storage, &pod).await;

        let controller = CreatingGameServerController::new(storage.clone());
        let updated = controller
            .reconcile_succeeded_game_server(created)
            .await
            .unwrap()
            .expect("Succeeded Pod should move GameServer to Shutdown");
        assert_eq!(updated.status.state, Some(GameServerState::Shutdown));

        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(persisted, updated);

        let events: Vec<Event> = storage
            .list(&build_prefix("events", Some("default")))
            .await
            .unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::Normal);
        assert_eq!(events[0].reason, "Shutdown");
        assert_eq!(events[0].message, "Pod is in Succeeded state");
        assert_eq!(events[0].source.component, "succeeded-controller");
    }

    #[tokio::test]
    async fn clean_game_container_exit_moves_gameserver_to_shutdown() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Scheduled, true))
            .await
            .unwrap();

        let mut pod = build_owned_pod(&created, true);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Running),
            container_statuses: Some(vec![
                terminated_status("game", 0),
                running_status("long-lived"),
            ]),
            ..Default::default()
        });
        store_pod(&storage, &pod).await;

        let controller = CreatingGameServerController::new(storage.clone());
        let updated = controller
            .reconcile_succeeded_game_server(created)
            .await
            .unwrap()
            .expect("clean game-container exit should move to Shutdown");
        assert_eq!(updated.status.state, Some(GameServerState::Shutdown));

        let events: Vec<Event> = storage
            .list(&build_prefix("events", Some("default")))
            .await
            .unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].message,
            "Game server container exited cleanly"
        );
    }

    #[tokio::test]
    async fn missing_running_foreign_and_terminating_pods_are_noops() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Ready, false))
            .await
            .unwrap();
        let controller = CreatingGameServerController::new(storage.clone());

        assert!(
            controller
                .reconcile_succeeded_game_server(created.clone())
                .await
                .unwrap()
                .is_none()
        );

        let mut running = build_owned_pod(&created, false);
        running.status = Some(PodStatus {
            phase: Some(Phase::Running),
            ..Default::default()
        });
        store_pod(&storage, &running).await;
        assert!(
            controller
                .reconcile_succeeded_game_server(created.clone())
                .await
                .unwrap()
                .is_none()
        );

        let mut foreign = running.clone();
        foreign.status.as_mut().unwrap().phase = Some(Phase::Succeeded);
        foreign.metadata.labels.as_mut().unwrap().remove(ROLE_LABEL);
        storage
            .update(&build_key("pods", Some("default"), "server"), &foreign)
            .await
            .unwrap();
        assert!(
            controller
                .reconcile_succeeded_game_server(created.clone())
                .await
                .unwrap()
                .is_none()
        );

        let mut terminating = build_owned_pod(&created, false);
        terminating.status = Some(PodStatus {
            phase: Some(Phase::Succeeded),
            ..Default::default()
        });
        terminating.metadata.deletion_timestamp = Some(chrono::Utc::now());
        storage
            .update(
                &build_key("pods", Some("default"), "server"),
                &terminating,
            )
            .await
            .unwrap();
        assert!(
            controller
                .reconcile_succeeded_game_server(created)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn current_terminal_or_deleting_gameserver_is_noop() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let queued = repository
            .create(&game_server(GameServerState::Allocated, false))
            .await
            .unwrap();

        let mut pod = build_owned_pod(&queued, false);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Succeeded),
            ..Default::default()
        });
        store_pod(&storage, &pod).await;

        let controller = CreatingGameServerController::new(storage.clone());

        let mut terminal = queued.clone();
        terminal.status.state = Some(GameServerState::Unhealthy);
        repository.update(&terminal).await.unwrap();
        assert!(
            controller
                .reconcile_succeeded_game_server(queued.clone())
                .await
                .unwrap()
                .is_none()
        );

        let mut deleting = terminal;
        deleting.status.state = Some(GameServerState::Ready);
        deleting.metadata.deletion_timestamp =
            Some("2026-10-17T00:00:00Z".into());
        repository.update(&deleting).await.unwrap();
        assert!(
            controller
                .reconcile_succeeded_game_server(queued)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn shutdown_write_waits_on_limiter_but_terminal_noop_does_not() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let queued = repository
            .create(&game_server(GameServerState::Allocated, false))
            .await
            .unwrap();

        let mut pod = build_owned_pod(&queued, false);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Succeeded),
            ..Default::default()
        });
        store_pod(&storage, &pod).await;

        let controller = CreatingGameServerController::new(storage.clone())
            .with_shutdown_limiter(StateUpdateLimiter::with_limits(0.0, 0));

        let error = controller
            .reconcile_succeeded_game_server(queued.clone())
            .await
            .unwrap_err();
        assert!(matches!(error, AdapterError::StateUpdateLimiter(_)));

        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(persisted.status.state, Some(GameServerState::Allocated));

        let mut shutdown = persisted;
        shutdown.status.state = Some(GameServerState::Shutdown);
        repository.update(&shutdown).await.unwrap();

        assert!(
            controller
                .reconcile_succeeded_game_server(queued)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn resync_enqueue_filter_matches_upstream_preconditions() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let ready = repository
            .create(&game_server(GameServerState::Ready, false))
            .await
            .unwrap();

        let mut pod = build_owned_pod(&ready, false);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Succeeded),
            ..Default::default()
        });
        store_pod(&storage, &pod).await;

        assert!(should_enqueue_succeeded_game_server(&storage, &ready).await);

        for state in [
            GameServerState::Starting,
            GameServerState::Unhealthy,
            GameServerState::Error,
            GameServerState::Shutdown,
        ] {
            let mut game_server = ready.clone();
            game_server.status.state = Some(state);
            assert!(
                !should_enqueue_succeeded_game_server(&storage, &game_server)
                    .await
            );
        }

        let mut dev = ready.clone();
        dev.metadata.annotations.insert(
            DEV_ADDRESS_ANNOTATION.into(),
            "203.0.113.7".into(),
        );
        assert!(!should_enqueue_succeeded_game_server(&storage, &dev).await);
    }
}
