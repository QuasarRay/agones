use agones_core::GameServer;
use rusternetes_common::resources::{Pod, PodSpec};
use rusternetes_storage::{build_key, Storage};

use crate::AdapterError;

/// Rusternetes equivalent of upstream Controller.gameServerPod.
///
/// Semantics intentionally follow Kubernetes metav1.IsControlledBy:
/// - development GameServers have no managed Pod and yield an empty Pod;
/// - a missing Pod propagates NotFound;
/// - for an existing Pod, only the first owner reference marked controller=true
///   is considered, and its UID must match the GameServer UID;
/// - a same-name Pod controlled by another object is surfaced as NotFound.
pub async fn game_server_pod<S: Storage>(
    storage: &S,
    game_server: &GameServer,
) -> Result<Pod, AdapterError> {
    if game_server.get_dev_address().is_some() {
        return Ok(Pod::new("", PodSpec::default()));
    }

    if game_server.metadata.name.is_empty() {
        return Err(AdapterError::MissingField("metadata.name"));
    }
    if game_server.metadata.namespace.is_empty() {
        return Err(AdapterError::MissingField("metadata.namespace"));
    }

    let key = build_key(
        "pods",
        Some(game_server.metadata.namespace.as_str()),
        game_server.metadata.name.as_str(),
    );
    let pod: Pod = storage.get(&key).await?;

    let controller = pod
        .metadata
        .owner_references
        .as_ref()
        .and_then(|owners| owners.iter().find(|owner| owner.controller == Some(true)));

    if controller.is_none_or(|owner| owner.uid != game_server.metadata.uid) {
        return Err(rusternetes_common::Error::NotFound(format!(
            "Pod {}",
            game_server.metadata.name
        ))
        .into());
    }

    Ok(pod)
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerSpec, ObjectMeta,
        PodSpec as CorePodSpec, PodTemplateSpec, DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_common::types::OwnerReference;
    use rusternetes_storage::{build_key, MemoryStorage};

    use crate::{build_pod, NoopApiHooks, PodBuildFeatures};

    use super::*;

    fn fixture() -> GameServer {
        GameServer {
            metadata: ObjectMeta {
                name: "server".into(),
                namespace: "default".into(),
                uid: "gameserver-uid".into(),
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
            ..Default::default()
        }
    }

    fn owner(uid: &str, controller: bool) -> OwnerReference {
        OwnerReference::new(
            "agones.dev/v1",
            "GameServer",
            "server",
            uid,
        )
        .with_controller(controller)
    }

    #[tokio::test]
    async fn returns_owned_pod() {
        let storage = MemoryStorage::new();
        let game_server = fixture();
        let pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        storage
            .create(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();

        let loaded = game_server_pod(&storage, &game_server).await.unwrap();
        assert_eq!(loaded.metadata.name, "server");
        assert_eq!(
            loaded.metadata.owner_references.as_ref().unwrap()[0].uid,
            "gameserver-uid"
        );
    }

    #[tokio::test]
    async fn missing_and_foreign_pods_are_not_found() {
        let storage = MemoryStorage::new();
        let game_server = fixture();

        assert!(matches!(
            game_server_pod(&storage, &game_server).await,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_)
            ))
        ));

        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.metadata.owner_references = Some(vec![owner("foreign-uid", true)]);
        storage
            .create(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();

        assert!(matches!(
            game_server_pod(&storage, &game_server).await,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_)
            ))
        ));
    }

    #[tokio::test]
    async fn first_controller_reference_matches_kubernetes_is_controlled_by_semantics() {
        let storage = MemoryStorage::new();
        let game_server = fixture();
        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.metadata.owner_references = Some(vec![
            owner("foreign-uid", true),
            owner("gameserver-uid", true),
        ]);
        storage
            .create(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();

        assert!(matches!(
            game_server_pod(&storage, &game_server).await,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_)
            ))
        ));
    }

    #[tokio::test]
    async fn development_gameserver_returns_empty_pod_without_storage_lookup() {
        let storage = MemoryStorage::new();
        let mut game_server = fixture();
        game_server.metadata.annotations.insert(
            DEV_ADDRESS_ANNOTATION.into(),
            "203.0.113.7".into(),
        );

        let pod = game_server_pod(&storage, &game_server).await.unwrap();
        assert!(pod.metadata.name.is_empty());
    }
}
