use std::sync::Arc;

use agones_core::{GameServer, GameServerState};
use cgp::prelude::*;
use rusternetes_common::resources::Pod;
use rusternetes_storage::{build_key, Storage};

use crate::{apply_game_server_pod_ips, AdapterError, AgonesRepository};

/// CGP capability for upstream `syncGameServerPodIPs`.
///
/// Stable GameServers can receive Pod IPs after their state transition. This
/// capability folds only newly observed Pod IPs into status and avoids writes
/// when no status change is required.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_stable_game_server_pod_ips<S>(
    &self,
    #[implicit] storage: Arc<S>,
    game_server: GameServer,
) -> Result<Option<GameServer>, AdapterError>
where
    S: Storage + 'static,
{
    let mut game_server = game_server;

    if !matches!(
        game_server.status.state,
        Some(
            GameServerState::Scheduled
                | GameServerState::Ready
                | GameServerState::Reserved
                | GameServerState::Allocated
        )
    ) || game_server.metadata.deletion_timestamp.is_some()
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
    let pod: Pod = match storage.get(&pod_key).await {
        Ok(pod) => pod,
        Err(rusternetes_common::Error::NotFound(_)) => return Ok(None),
        Err(error) => return Err(error.into()),
    };

    let controlled_by_game_server = pod
        .metadata
        .owner_references
        .as_ref()
        .is_some_and(|owners| {
            owners.iter().any(|owner| {
                owner.controller == Some(true) && owner.uid == game_server.metadata.uid
            })
        });
    if !controlled_by_game_server {
        // Upstream gameServerPod() treats a same-name Pod controlled by a
        // different owner as NotFound.
        return Ok(None);
    }

    if !apply_game_server_pod_ips(&mut game_server, &pod) {
        return Ok(None);
    }

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
    use agones_core::gameserver::NodeAddress as GameServerNodeAddress;
    use rusternetes_common::resources::{PodIP, PodStatus};
    use rusternetes_storage::MemoryStorage;

    use crate::{build_pod, CreatingGameServerController, NoopApiHooks, PodBuildFeatures};

    use super::*;

    fn stable_game_server(state: GameServerState) -> GameServer {
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

    async fn store_pod(storage: &MemoryStorage, game_server: &GameServer, ips: &[&str]) {
        let mut pod = build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.status = Some(PodStatus {
            pod_i_ps: Some(
                ips.iter()
                    .map(|ip| PodIP {
                        ip: (*ip).to_owned(),
                    })
                    .collect(),
            ),
            ..Default::default()
        });
        storage
            .create(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn every_stable_state_folds_late_pod_ip_and_persists_once() {
        for state in [
            GameServerState::Scheduled,
            GameServerState::Ready,
            GameServerState::Reserved,
            GameServerState::Allocated,
        ] {
            let storage = MemoryStorage::new();
            let repository = AgonesRepository::new(storage.clone());
            let created = repository
                .create(&stable_game_server(state.clone()))
                .await
                .unwrap();
            store_pod(&storage, &created, &["10.0.0.9"]).await;

            let controller = CreatingGameServerController::new(storage.clone());
            let reconciled = controller
                .reconcile_stable_game_server_pod_ips(created)
                .await
                .unwrap()
                .expect("late PodIP should update stable GameServer");

            assert_eq!(reconciled.status.state, Some(state));
            assert_eq!(
                reconciled.status.addresses,
                vec![GameServerNodeAddress {
                    address: "10.0.0.9".into(),
                    address_type: "PodIP".into(),
                }]
            );

            let persisted: GameServer = repository.get("default", "server").await.unwrap();
            assert_eq!(persisted, reconciled);
        }
    }

    #[tokio::test]
    async fn already_present_pod_ip_is_a_write_free_noop() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fixture = stable_game_server(GameServerState::Ready);
        fixture.status.addresses = vec![GameServerNodeAddress {
            address: "10.0.0.9".into(),
            address_type: "PodIP".into(),
        }];
        let created = repository.create(&fixture).await.unwrap();
        store_pod(&storage, &created, &["10.0.0.9"]).await;

        let controller = CreatingGameServerController::new(storage);
        assert!(
            controller
                .reconcile_stable_game_server_pod_ips(created)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn same_name_pod_controlled_by_another_owner_is_treated_as_missing() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&stable_game_server(GameServerState::Ready))
            .await
            .unwrap();

        let mut pod = build_pod(
            &created,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.status = Some(PodStatus {
            pod_i_ps: Some(vec![PodIP {
                ip: "10.0.0.9".into(),
            }]),
            ..Default::default()
        });
        pod.metadata.owner_references.as_mut().unwrap()[0].uid = "different-owner".into();
        storage
            .create(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage);
        assert!(
            controller
                .reconcile_stable_game_server_pod_ips(created)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn missing_pod_is_expected_noop_for_stable_state() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&stable_game_server(GameServerState::Ready))
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage);
        assert!(
            controller
                .reconcile_stable_game_server_pod_ips(created)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn deleted_development_and_nonstable_states_are_noops() {
        let storage = MemoryStorage::new();
        let controller = CreatingGameServerController::new(storage);

        let mut deleted = stable_game_server(GameServerState::Ready);
        deleted.metadata.deletion_timestamp = Some("2026-10-17T00:00:00Z".into());
        assert!(
            controller
                .reconcile_stable_game_server_pod_ips(deleted)
                .await
                .unwrap()
                .is_none()
        );

        let mut development = stable_game_server(GameServerState::Ready);
        development
            .metadata
            .annotations
            .insert(DEV_ADDRESS_ANNOTATION.into(), "203.0.113.7".into());
        assert!(
            controller
                .reconcile_stable_game_server_pod_ips(development)
                .await
                .unwrap()
                .is_none()
        );

        let starting = stable_game_server(GameServerState::Starting);
        assert!(
            controller
                .reconcile_stable_game_server_pod_ips(starting)
                .await
                .unwrap()
                .is_none()
        );
    }
}
