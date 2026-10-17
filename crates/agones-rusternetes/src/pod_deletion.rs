use chrono::Utc;
use rusternetes_common::resources::Pod;
use rusternetes_storage::{build_key, Storage};

use crate::AdapterError;

/// Reproduce the persistence behavior of the pinned Rusternetes Pod DELETE
/// handler for an ordinary delete request with no explicit DeleteOptions.
///
/// The live Pod is re-read before mutation. The Pod's configured termination
/// grace period is used, defaulting to 30 seconds. A zero grace period deletes
/// immediately; otherwise deletionTimestamp/grace/generation are persisted.
/// One CAS conflict is retried with a fresh read, matching the handler.
pub async fn request_pod_deletion<S: Storage>(
    storage: &S,
    pod: &Pod,
) -> Result<(), AdapterError> {
    if pod.metadata.name.is_empty() {
        return Err(AdapterError::MissingField("pod.metadata.name"));
    }
    let namespace = pod
        .metadata
        .namespace
        .as_deref()
        .filter(|namespace| !namespace.is_empty())
        .ok_or(AdapterError::MissingField("pod.metadata.namespace"))?;
    let key = build_key("pods", Some(namespace), pod.metadata.name.as_str());

    let mut live: Pod = storage.get(&key).await?;
    let grace_period = live
        .spec
        .as_ref()
        .and_then(|spec| spec.termination_grace_period_seconds)
        .unwrap_or(30);

    if live.metadata.deletion_timestamp.is_none() {
        live.metadata.deletion_timestamp = Some(Utc::now());
    }
    live.metadata.deletion_grace_period_seconds = Some(grace_period);

    if grace_period == 0 {
        storage.delete(&key).await?;
        return Ok(());
    }

    let current_generation = live.metadata.generation.unwrap_or(1);
    live.metadata.generation = Some(current_generation + 1);

    match storage.update(&key, &live).await {
        Ok::<Pod, _>(_) => Ok(()),
        Err(rusternetes_common::Error::Conflict(_)) => {
            let mut retry: Pod = storage.get(&key).await?;
            if retry.metadata.deletion_timestamp.is_none() {
                retry.metadata.deletion_timestamp = Some(Utc::now());
            }
            retry.metadata.deletion_grace_period_seconds = Some(grace_period);
            let generation = retry.metadata.generation.unwrap_or(1);
            retry.metadata.generation = Some(generation + 1);
            let _: Pod = storage.update(&key, &retry).await?;
            Ok(())
        }
        Err(error) => Err(error.into()),
    }
}

#[cfg(test)]
mod tests {
    use rusternetes_common::resources::PodSpec;
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn pod(grace_period: Option<i64>) -> Pod {
        let mut pod = Pod::new("server", PodSpec::default());
        pod.metadata.namespace = Some("default".into());
        pod.spec.as_mut().unwrap().termination_grace_period_seconds = grace_period;
        pod
    }

    #[tokio::test]
    async fn default_delete_marks_pod_with_thirty_second_grace_period() {
        let storage = MemoryStorage::new();
        let key = build_key("pods", Some("default"), "server");
        storage.create(&key, &pod(None)).await.unwrap();

        let stored: Pod = storage.get(&key).await.unwrap();
        request_pod_deletion(&storage, &stored).await.unwrap();

        let deleting: Pod = storage.get(&key).await.unwrap();
        assert!(deleting.metadata.deletion_timestamp.is_some());
        assert_eq!(
            deleting.metadata.deletion_grace_period_seconds,
            Some(30)
        );
        assert_eq!(deleting.metadata.generation, Some(2));
    }

    #[tokio::test]
    async fn configured_grace_period_is_preserved() {
        let storage = MemoryStorage::new();
        let key = build_key("pods", Some("default"), "server");
        storage.create(&key, &pod(Some(17))).await.unwrap();

        let stored: Pod = storage.get(&key).await.unwrap();
        request_pod_deletion(&storage, &stored).await.unwrap();

        let deleting: Pod = storage.get(&key).await.unwrap();
        assert_eq!(
            deleting.metadata.deletion_grace_period_seconds,
            Some(17)
        );
    }

    #[tokio::test]
    async fn zero_grace_period_force_deletes_pod() {
        let storage = MemoryStorage::new();
        let key = build_key("pods", Some("default"), "server");
        storage.create(&key, &pod(Some(0))).await.unwrap();

        let stored: Pod = storage.get(&key).await.unwrap();
        request_pod_deletion(&storage, &stored).await.unwrap();

        assert!(matches!(
            storage.get::<Pod>(&key).await,
            Err(rusternetes_common::Error::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn one_update_conflict_is_retried() {
        let storage = MemoryStorage::new();
        let key = build_key("pods", Some("default"), "server");
        storage.create(&key, &pod(None)).await.unwrap();
        storage.inject_conflicts(1);

        let stored: Pod = storage.get(&key).await.unwrap();
        request_pod_deletion(&storage, &stored).await.unwrap();

        let deleting: Pod = storage.get(&key).await.unwrap();
        assert!(deleting.metadata.deletion_timestamp.is_some());
        assert_eq!(
            deleting.metadata.deletion_grace_period_seconds,
            Some(30)
        );
    }

    #[tokio::test]
    async fn second_update_conflict_is_propagated() {
        let storage = MemoryStorage::new();
        let key = build_key("pods", Some("default"), "server");
        storage.create(&key, &pod(None)).await.unwrap();
        storage.inject_conflicts(2);

        let stored: Pod = storage.get(&key).await.unwrap();
        assert!(matches!(
            request_pod_deletion(&storage, &stored).await,
            Err(AdapterError::Rusternetes(
                rusternetes_common::Error::Conflict(_)
            ))
        ));

        let unchanged: Pod = storage.get(&key).await.unwrap();
        assert!(unchanged.metadata.deletion_timestamp.is_none());
    }
}
