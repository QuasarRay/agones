use std::sync::Arc;

use agones_core::{
    GameServer, GameServerState, GAME_SERVER_READY_CONTAINER_ID_ANNOTATION,
};
use cgp::prelude::*;
use rusternetes_common::resources::{ContainerState, EventType, Pod};
use rusternetes_common::types::Phase;
use rusternetes_storage::{build_key, Storage};

use crate::{
    game_server_event::record_game_server_event, is_game_server_pod,
    AdapterError, AgonesRepository, PodBuildFeatures, StateUpdateLimiter,
    GAME_SERVER_CONTAINER_ANNOTATION,
};

const POD_SCHEDULED_CONDITION: &str = "PodScheduled";
const UNSCHEDULABLE_REASON: &str = "Unschedulable";

/// Mirrors upstream HealthController.evictedPod.
pub fn pod_evicted(pod: &Pod) -> bool {
    pod.status
        .as_ref()
        .and_then(|status| status.reason.as_deref())
        == Some("Evicted")
}

/// Mirrors upstream HealthController.unschedulableWithNoFreePorts.
///
/// SidecarContainers mode and waitOnFreePorts both intentionally disable this
/// legacy host-port failure signal.
pub fn unschedulable_with_no_free_ports(
    pod: &Pod,
    sidecar_containers: bool,
    wait_on_free_ports: bool,
) -> bool {
    if sidecar_containers || wait_on_free_ports {
        return false;
    }

    pod.status
        .as_ref()
        .and_then(|status| status.conditions.as_ref())
        .is_some_and(|conditions| {
            conditions.iter().any(|condition| {
                condition.condition_type == POD_SCHEDULED_CONDITION
                    && condition.reason.as_deref() == Some(UNSCHEDULABLE_REASON)
                    && condition
                        .message
                        .as_deref()
                        .is_some_and(|message| message.contains("free ports"))
            })
        })
}

/// Mirrors upstream HealthController.failedPod.
pub fn failed_pod(pod: &Pod, sidecar_containers: bool) -> bool {
    sidecar_containers
        && pod.status.as_ref().and_then(|status| status.phase.as_ref())
            == Some(&Phase::Failed)
}

/// Mirrors upstream HealthController.failedContainer.
///
/// In SidecarContainers mode only a non-zero terminal exit under
/// RestartPolicy=Never is unhealthy. In legacy mode any current or previous
/// termination of the game container is considered a failure.
pub fn failed_container(pod: &Pod, sidecar_containers: bool) -> bool {
    let Some(container_name) = pod
        .metadata
        .annotations
        .as_ref()
        .and_then(|annotations| annotations.get(GAME_SERVER_CONTAINER_ANNOTATION))
    else {
        return false;
    };

    let Some(statuses) = pod
        .status
        .as_ref()
        .and_then(|status| status.container_statuses.as_ref())
    else {
        return false;
    };

    let Some(container_status) = statuses
        .iter()
        .find(|status| status.name == *container_name)
    else {
        return false;
    };

    if sidecar_containers {
        if pod
            .spec
            .as_ref()
            .and_then(|spec| spec.restart_policy.as_deref())
            != Some("Never")
        {
            return false;
        }

        return matches!(
            container_status.state.as_ref(),
            Some(ContainerState::Terminated { exit_code, .. }) if *exit_code != 0
        );
    }

    matches!(
        container_status.state.as_ref(),
        Some(ContainerState::Terminated { .. })
    ) || matches!(
        container_status.last_state.as_ref(),
        Some(ContainerState::Terminated { .. })
    )
}

/// Mirrors upstream HealthController.isUnhealthy.
pub fn is_unhealthy_pod(
    pod: &Pod,
    sidecar_containers: bool,
    wait_on_free_ports: bool,
) -> bool {
    pod_evicted(pod)
        || unschedulable_with_no_free_ports(
            pod,
            sidecar_containers,
            wait_on_free_ports,
        )
        || failed_container(pod, sidecar_containers)
        || failed_pod(pod, sidecar_containers)
}

/// Pod informer UpdateFunc predicate from upstream HealthController.
pub fn should_enqueue_unhealthy_pod(
    pod: &Pod,
    sidecar_containers: bool,
    wait_on_free_ports: bool,
) -> bool {
    pod.metadata.deletion_timestamp.is_none()
        && is_game_server_pod(pod)
        && is_unhealthy_pod(pod, sidecar_containers, wait_on_free_ports)
}

fn is_controlled_by_game_server(pod: &Pod, game_server: &GameServer) -> bool {
    pod.metadata
        .owner_references
        .as_ref()
        .and_then(|owners| {
            owners.iter().find(|owner| owner.controller == Some(true))
        })
        .is_some_and(|owner| owner.uid == game_server.metadata.uid)
}

/// Mirrors upstream HealthController.skipUnhealthyGameContainer.
///
/// The returned bool means "skip the Unhealthy transition". A ready-container
/// annotation mismatch is surfaced as an error so the work queue retries after
/// the Pod/GameServer informer views converge.
pub fn skip_unhealthy_game_container(
    game_server: &GameServer,
    pod: &Pod,
    sidecar_containers: bool,
) -> Result<bool, AdapterError> {
    if !is_controlled_by_game_server(pod, game_server) {
        return Ok(false);
    }

    if sidecar_containers {
        return Ok(false);
    }

    let game_server_ready_container_id = game_server
        .metadata
        .annotations
        .get(GAME_SERVER_READY_CONTAINER_ID_ANNOTATION)
        .map(String::as_str)
        .unwrap_or("");
    let pod_ready_container_id = pod
        .metadata
        .annotations
        .as_ref()
        .and_then(|annotations| {
            annotations.get(GAME_SERVER_READY_CONTAINER_ID_ANNOTATION)
        })
        .map(String::as_str)
        .unwrap_or("");

    if pod_ready_container_id != game_server_ready_container_id {
        return Err(AdapterError::PodGameServerOutOfSync(
            game_server.metadata.name.clone(),
        ));
    }

    if game_server.is_before_ready() {
        return Ok(failed_container(pod, false));
    }

    let Some(container_status) = pod
        .status
        .as_ref()
        .and_then(|status| status.container_statuses.as_ref())
        .and_then(|statuses| {
            statuses
                .iter()
                .find(|status| status.name == game_server.spec.container)
        })
    else {
        return Ok(false);
    };

    if matches!(
        container_status.state.as_ref(),
        Some(ContainerState::Terminated { .. })
    ) {
        return Ok(false);
    }

    if matches!(
        container_status.last_state.as_ref(),
        Some(ContainerState::Terminated { .. })
    ) {
        return Ok(
            container_status.container_id.as_deref().unwrap_or("")
                == game_server_ready_container_id,
        );
    }

    Ok(false)
}

/// CGP capability for upstream HealthController.syncGameServer.
///
/// A missing Pod or a Pod that remains unhealthy after the race/annotation
/// checks moves the current GameServer to Unhealthy. The write uses a dedicated
/// HealthController token bucket and records the upstream Warning event.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_health_game_server<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] pod_build_features: PodBuildFeatures,
    #[implicit] wait_on_free_ports: bool,
    #[implicit] health_unhealthy_limiter: Arc<StateUpdateLimiter>,
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
        || matches!(
            current.status.state.as_ref(),
            Some(GameServerState::Unhealthy | GameServerState::Error)
        )
    {
        return Ok(None);
    }

    let pod_key = build_key(
        "pods",
        Some(current.metadata.namespace.as_str()),
        current.metadata.name.as_str(),
    );
    let pod = match storage.get::<Pod>(&pod_key).await {
        Ok(pod) => Some(pod),
        Err(rusternetes_common::Error::NotFound(_)) => None,
        Err(error) => return Err(error.into()),
    };

    if let Some(pod) = pod.as_ref() {
        if skip_unhealthy_game_container(
            &current,
            pod,
            pod_build_features.sidecar_containers,
        )? {
            return Ok(None);
        }

        if !is_unhealthy_pod(
            pod,
            pod_build_features.sidecar_containers,
            wait_on_free_ports,
        ) {
            return Ok(None);
        }
    }

    health_unhealthy_limiter.wait().await?;

    current.status.state = Some(GameServerState::Unhealthy);
    let updated = repository.update(&current).await?;
    record_game_server_event(
        storage.as_ref(),
        &updated,
        "health-controller",
        EventType::Warning,
        "Unhealthy",
        "Issue with Gameserver pod",
    )
    .await;
    Ok(Some(updated))
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerSpec, GameServerStatus, ObjectMeta,
        PodSpec as CorePodSpec, PodTemplateSpec,
    };
    use rusternetes_common::resources::{
        ContainerStatus, Event, PodCondition, PodStatus,
    };
    use rusternetes_storage::{build_prefix, MemoryStorage};

    use crate::{
        build_pod, CreatingGameServerController, NoopApiHooks, PodBuildFeatures,
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

    fn container_status(
        name: &str,
        state: Option<serde_json::Value>,
        last_state: Option<serde_json::Value>,
        container_id: Option<&str>,
    ) -> ContainerStatus {
        let mut value = serde_json::json!({
            "name": name,
            "ready": false,
            "restartCount": 0
        });
        let object = value.as_object_mut().unwrap();
        if let Some(state) = state {
            object.insert("state".into(), state);
        }
        if let Some(last_state) = last_state {
            object.insert("lastState".into(), last_state);
        }
        if let Some(container_id) = container_id {
            object.insert(
                "containerID".into(),
                serde_json::Value::String(container_id.into()),
            );
        }
        serde_json::from_value(value).unwrap()
    }

    fn terminated(exit_code: i32) -> serde_json::Value {
        serde_json::json!({
            "terminated": {
                "exitCode": exit_code
            }
        })
    }

    fn running() -> serde_json::Value {
        serde_json::json!({ "running": {} })
    }

    fn owned_pod(game_server: &GameServer) -> Pod {
        build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures::default(),
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
    fn unhealthy_predicates_match_upstream_modes() {
        let game_server = game_server(GameServerState::Ready);
        let mut pod = owned_pod(&game_server);

        pod.status = Some(PodStatus {
            reason: Some("Evicted".into()),
            ..Default::default()
        });
        assert!(pod_evicted(&pod));
        assert!(is_unhealthy_pod(&pod, false, false));

        pod.status = Some(PodStatus {
            conditions: Some(vec![PodCondition {
                condition_type: POD_SCHEDULED_CONDITION.into(),
                status: "False".into(),
                reason: Some(UNSCHEDULABLE_REASON.into()),
                message: Some("0/1 nodes are available: no free ports".into()),
                last_transition_time: None,
                observed_generation: None,
            }]),
            ..Default::default()
        });
        assert!(unschedulable_with_no_free_ports(&pod, false, false));
        assert!(!unschedulable_with_no_free_ports(&pod, true, false));
        assert!(!unschedulable_with_no_free_ports(&pod, false, true));

        pod.status = Some(PodStatus {
            phase: Some(Phase::Failed),
            ..Default::default()
        });
        assert!(!failed_pod(&pod, false));
        assert!(failed_pod(&pod, true));
    }

    #[test]
    fn failed_container_matches_legacy_and_sidecar_semantics() {
        let game_server = game_server(GameServerState::Ready);
        let mut pod = owned_pod(&game_server);
        pod.status = Some(PodStatus {
            phase: Some(Phase::Running),
            container_statuses: Some(vec![container_status(
                "game",
                Some(terminated(0)),
                None,
                Some("container://1"),
            )]),
            ..Default::default()
        });

        assert!(failed_container(&pod, false));
        assert!(!failed_container(&pod, true));

        pod.spec.as_mut().unwrap().restart_policy = Some("Never".into());
        pod.status.as_mut().unwrap().container_statuses.as_mut().unwrap()[0] =
            container_status(
                "game",
                Some(terminated(1)),
                None,
                Some("container://1"),
            );
        assert!(failed_container(&pod, true));

        pod.status.as_mut().unwrap().container_statuses.as_mut().unwrap()[0] =
            container_status(
                "game",
                Some(terminated(0)),
                None,
                Some("container://1"),
            );
        assert!(!failed_container(&pod, true));

        pod.status.as_mut().unwrap().container_statuses.as_mut().unwrap()[0] =
            container_status(
                "game",
                Some(running()),
                Some(terminated(1)),
                Some("container://2"),
            );
        assert!(failed_container(&pod, false));
    }

    #[test]
    fn skip_unhealthy_container_preserves_ready_container_race_rules() {
        let mut game_server = game_server(GameServerState::Starting);
        game_server.metadata.uid = "gs-uid".into();
        let mut pod = owned_pod(&game_server);
        pod.metadata
            .annotations
            .as_mut()
            .unwrap()
            .insert(
                GAME_SERVER_READY_CONTAINER_ID_ANNOTATION.into(),
                "".into(),
            );
        pod.status = Some(PodStatus {
            container_statuses: Some(vec![container_status(
                "game",
                Some(terminated(1)),
                None,
                Some("container://old"),
            )]),
            ..Default::default()
        });
        assert!(
            skip_unhealthy_game_container(&game_server, &pod, false).unwrap()
        );

        game_server.status.state = Some(GameServerState::Ready);
        game_server.metadata.annotations.insert(
            GAME_SERVER_READY_CONTAINER_ID_ANNOTATION.into(),
            "container://ready".into(),
        );
        pod.metadata.annotations.as_mut().unwrap().insert(
            GAME_SERVER_READY_CONTAINER_ID_ANNOTATION.into(),
            "container://ready".into(),
        );
        pod.status.as_mut().unwrap().container_statuses.as_mut().unwrap()[0] =
            container_status(
                "game",
                Some(running()),
                Some(terminated(1)),
                Some("container://ready"),
            );
        assert!(
            skip_unhealthy_game_container(&game_server, &pod, false).unwrap()
        );

        pod.status.as_mut().unwrap().container_statuses.as_mut().unwrap()[0] =
            container_status(
                "game",
                Some(running()),
                Some(terminated(1)),
                Some("container://new"),
            );
        assert!(
            !skip_unhealthy_game_container(&game_server, &pod, false).unwrap()
        );

        pod.metadata.annotations.as_mut().unwrap().insert(
            GAME_SERVER_READY_CONTAINER_ID_ANNOTATION.into(),
            "container://stale".into(),
        );
        assert!(matches!(
            skip_unhealthy_game_container(&game_server, &pod, false),
            Err(AdapterError::PodGameServerOutOfSync(_))
        ));

        assert!(
            !skip_unhealthy_game_container(&game_server, &pod, true).unwrap()
        );
    }

    #[tokio::test]
    async fn missing_pod_moves_gameserver_to_unhealthy_and_records_event() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone());
        let updated = controller
            .reconcile_health_game_server(created)
            .await
            .unwrap()
            .expect("missing Pod should move GameServer to Unhealthy");
        assert_eq!(updated.status.state, Some(GameServerState::Unhealthy));

        let events: Vec<Event> = storage
            .list(&build_prefix("events", Some("default")))
            .await
            .unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::Warning);
        assert_eq!(events[0].reason, "Unhealthy");
        assert_eq!(events[0].message, "Issue with Gameserver pod");
        assert_eq!(events[0].source.component, "health-controller");
    }

    #[tokio::test]
    async fn healthy_pod_and_restartable_preready_crash_are_noops() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let ready = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();
        let mut healthy_pod = owned_pod(&ready);
        healthy_pod.status = Some(PodStatus {
            phase: Some(Phase::Running),
            container_statuses: Some(vec![container_status(
                "game",
                Some(running()),
                None,
                Some("container://live"),
            )]),
            ..Default::default()
        });
        store_pod(&storage, &healthy_pod).await;

        let controller = CreatingGameServerController::new(storage.clone());
        assert!(
            controller
                .reconcile_health_game_server(ready.clone())
                .await
                .unwrap()
                .is_none()
        );

        let mut starting = ready;
        starting.status.state = Some(GameServerState::Starting);
        repository.update(&starting).await.unwrap();

        let mut crash = healthy_pod;
        crash.status.as_mut().unwrap().container_statuses.as_mut().unwrap()[0] =
            container_status(
                "game",
                Some(terminated(1)),
                None,
                Some("container://old"),
            );
        storage
            .update(&build_key("pods", Some("default"), "server"), &crash)
            .await
            .unwrap();

        assert!(
            controller
                .reconcile_health_game_server(starting)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn sidecar_failed_container_marks_unhealthy() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();

        let mut pod = owned_pod(&created);
        pod.spec.as_mut().unwrap().restart_policy = Some("Never".into());
        pod.status = Some(PodStatus {
            phase: Some(Phase::Running),
            container_statuses: Some(vec![container_status(
                "game",
                Some(terminated(2)),
                None,
                Some("container://dead"),
            )]),
            ..Default::default()
        });
        store_pod(&storage, &pod).await;

        let controller = CreatingGameServerController::new(storage.clone());
        let mut features = PodBuildFeatures::default();
        features.sidecar_containers = true;
        let controller = CreatingGameServerController::with_context(
            storage,
            Vec::new(),
            features,
            NoopApiHooks,
        );

        let updated = controller
            .reconcile_health_game_server(created)
            .await
            .unwrap()
            .expect("non-zero terminal sidecar-mode game container is unhealthy");
        assert_eq!(updated.status.state, Some(GameServerState::Unhealthy));
    }

    #[tokio::test]
    async fn health_write_waits_on_dedicated_limiter_after_noop_guards() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let queued = repository
            .create(&game_server(GameServerState::Ready))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage.clone())
            .with_health_unhealthy_limiter(
                StateUpdateLimiter::with_limits(0.0, 0),
            );

        let error = controller
            .reconcile_health_game_server(queued.clone())
            .await
            .unwrap_err();
        assert!(matches!(error, AdapterError::StateUpdateLimiter(_)));

        let persisted: GameServer =
            repository.get("default", "server").await.unwrap();
        assert_eq!(persisted.status.state, Some(GameServerState::Ready));

        let mut unhealthy = persisted;
        unhealthy.status.state = Some(GameServerState::Unhealthy);
        repository.update(&unhealthy).await.unwrap();

        assert!(
            controller
                .reconcile_health_game_server(queued)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn informer_predicate_excludes_terminating_and_non_gameserver_pods() {
        let game_server = game_server(GameServerState::Ready);
        let mut pod = owned_pod(&game_server);
        pod.status = Some(PodStatus {
            reason: Some("Evicted".into()),
            ..Default::default()
        });
        assert!(should_enqueue_unhealthy_pod(&pod, false, false));

        pod.metadata.deletion_timestamp = Some(chrono::Utc::now());
        assert!(!should_enqueue_unhealthy_pod(&pod, false, false));

        pod.metadata.deletion_timestamp = None;
        pod.metadata.labels.as_mut().unwrap().remove(crate::ROLE_LABEL);
        assert!(!should_enqueue_unhealthy_pod(&pod, false, false));
    }
}
