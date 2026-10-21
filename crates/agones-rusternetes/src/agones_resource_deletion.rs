use chrono::Utc;
use rusternetes_common::resources::CustomResource;
use rusternetes_storage::Storage;

use crate::{
    rusternetes_custom_resource_key, AdapterError, RusternetesCustomResource,
};

/// Result of applying Kubernetes/Rusternetes delete-request semantics to an
/// Agones custom resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeletionRequestOutcome {
    /// The object had no remaining finalizers and was removed from storage.
    Deleted,
    /// The object has finalizers and was marked with deletionTimestamp.
    MarkedForDeletion,
    /// The object was already deleting and still has finalizers to process.
    WaitingForFinalizers,
}

/// Apply the persistence portion of Rusternetes background DELETE semantics to
/// an Agones custom resource.
///
/// This mirrors the Rusternetes finalizer deletion helper with background
/// propagation: background deletion does not add a propagation finalizer;
/// existing finalizers cause deletionTimestamp to be set; no finalizers means
/// immediate deletion.
pub async fn request_agones_resource_deletion<S, T>(
    storage: &S,
    value: &T,
) -> Result<DeletionRequestOutcome, AdapterError>
where
    S: Storage,
    T: RusternetesCustomResource + Send + Sync,
{
    let identity = value.to_rusternetes_custom_resource()?;

    if identity.metadata.name.is_empty() {
        return Err(AdapterError::MissingField("metadata.name"));
    }
    let namespace = identity
        .metadata
        .namespace
        .as_deref()
        .filter(|namespace| !namespace.is_empty())
        .ok_or(AdapterError::MissingField("metadata.namespace"))?;
    let key = rusternetes_custom_resource_key::<T>(
        namespace,
        identity.metadata.name.as_str(),
    );

    // DELETE semantics are evaluated against the live server object. This
    // prevents a stale controller snapshot from bypassing a finalizer that was
    // added after the snapshot was observed.
    let mut resource: CustomResource = storage.get(&key).await?;
    let _ = T::from_rusternetes_custom_resource(resource.clone())?;

    let has_finalizers = resource
        .metadata
        .finalizers
        .as_ref()
        .is_some_and(|finalizers| !finalizers.is_empty());

    if resource.metadata.deletion_timestamp.is_some() {
        if has_finalizers {
            return Ok(DeletionRequestOutcome::WaitingForFinalizers);
        }

        storage.delete(&key).await?;
        return Ok(DeletionRequestOutcome::Deleted);
    }

    if !has_finalizers {
        storage.delete(&key).await?;
        return Ok(DeletionRequestOutcome::Deleted);
    }

    resource.metadata.deletion_timestamp = Some(Utc::now());
    let _: CustomResource = storage.update(&key, &resource).await?;
    Ok(DeletionRequestOutcome::MarkedForDeletion)
}

#[cfg(test)]
mod tests {
    use agones_core::{
        GameServer, GameServerSpec, GameServerState, GameServerStatus, ObjectMeta,
        FINALIZER_NAME,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::AgonesRepository;

    use super::*;

    fn game_server(finalizers: Vec<String>) -> GameServer {
        GameServer {
            type_meta: Default::default(),
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
    async fn finalizers_mark_resource_for_deletion_without_removing_it() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository
            .create(&game_server(vec![FINALIZER_NAME.into()]))
            .await
            .unwrap();

        assert_eq!(
            request_agones_resource_deletion(&storage, &created)
                .await
                .unwrap(),
            DeletionRequestOutcome::MarkedForDeletion
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
    async fn no_finalizers_delete_resource_immediately() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&game_server(Vec::new())).await.unwrap();

        assert_eq!(
            request_agones_resource_deletion(&storage, &created)
                .await
                .unwrap(),
            DeletionRequestOutcome::Deleted
        );
        assert!(matches!(
            repository.get::<GameServer>("default", "server").await,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_)
            ))
        ));
    }

    #[tokio::test]
    async fn live_storage_finalizers_override_a_stale_controller_snapshot() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let stored = repository
            .create(&game_server(vec![FINALIZER_NAME.into()]))
            .await
            .unwrap();

        let mut stale = stored.clone();
        stale.metadata.finalizers.clear();

        assert_eq!(
            request_agones_resource_deletion(&storage, &stale)
                .await
                .unwrap(),
            DeletionRequestOutcome::MarkedForDeletion
        );

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert!(persisted.metadata.deletion_timestamp.is_some());
        assert_eq!(
            persisted.metadata.finalizers,
            vec![FINALIZER_NAME.to_owned()]
        );
    }

    #[tokio::test]
    async fn already_deleting_resource_waits_while_finalizers_remain() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fixture = game_server(vec![FINALIZER_NAME.into()]);
        fixture.metadata.deletion_timestamp = Some("2026-10-17T00:00:00Z".into());
        let created = repository.create(&fixture).await.unwrap();

        assert_eq!(
            request_agones_resource_deletion(&storage, &created)
                .await
                .unwrap(),
            DeletionRequestOutcome::WaitingForFinalizers
        );

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(
            persisted.metadata.deletion_timestamp.as_deref(),
            Some("2026-10-17T00:00:00Z")
        );
    }

    #[tokio::test]
    async fn already_deleting_resource_without_finalizers_is_removed() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut fixture = game_server(Vec::new());
        fixture.metadata.deletion_timestamp = Some("2026-10-17T00:00:00Z".into());
        let created = repository.create(&fixture).await.unwrap();

        assert_eq!(
            request_agones_resource_deletion(&storage, &created)
                .await
                .unwrap(),
            DeletionRequestOutcome::Deleted
        );
        assert!(matches!(
            repository.get::<GameServer>("default", "server").await,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::NotFound(_)
            ))
        ));
    }
}
