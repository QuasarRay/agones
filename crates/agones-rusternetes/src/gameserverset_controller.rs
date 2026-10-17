use std::sync::{Arc, RwLock};

use agones_core::{
    GameServer, GameServerSet, GameServerSetReconciliationAction,
    GameServerSetStatus, GameServerState, SchedulingStrategy,
    GAME_SERVER_SET_GAME_SERVER_LABEL,
    compute_game_server_set_reconciliation_action,
    compute_game_server_set_status,
};
use cgp::prelude::*;
use chrono::Utc;
use futures::{StreamExt, TryStreamExt};
use rusternetes_storage::Storage;

use crate::{
    AdapterError, AgonesRepository, GameServerStateCache,
    PerNodeCounterState,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameServerSetControllerConfig {
    pub max_creation_parallelism: usize,
    pub max_deletion_parallelism: usize,
    pub max_game_server_creations_per_batch: i32,
    pub max_game_server_deletions_per_batch: i32,
    pub max_pod_pending_count: i32,
    pub player_tracking: bool,
}

impl Default for GameServerSetControllerConfig {
    fn default() -> Self {
        Self {
            max_creation_parallelism: 1,
            max_deletion_parallelism: 1,
            max_game_server_creations_per_batch: i32::MAX,
            max_game_server_deletions_per_batch: i32::MAX,
            max_pod_pending_count: i32::MAX,
            player_tracking: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameServerSetReconcileResult {
    pub game_server_set: GameServerSet,
    pub action: GameServerSetReconciliationAction,
    pub status_changed: bool,
}

impl GameServerSetReconcileResult {
    pub const fn is_partial(&self) -> bool {
        self.action.is_partial
    }

    pub fn changed(&self) -> bool {
        self.status_changed
            || self.action.num_servers_to_add > 0
            || !self.action.to_delete.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameServerSetSyncOutcome {
    NotFound,
    Reconciled(GameServerSetReconcileResult),
}

#[derive(HasField)]
pub struct GameServerSetController<S> {
    pub storage: Arc<S>,
    pub game_server_state_cache: Arc<GameServerStateCache>,
    pub per_node_counter_state: Arc<RwLock<PerNodeCounterState>>,
    pub game_server_set_config: GameServerSetControllerConfig,
}

impl<S> GameServerSetController<S> {
    pub fn new(storage: S, config: GameServerSetControllerConfig) -> Self {
        Self {
            storage: Arc::new(storage),
            game_server_state_cache: Arc::new(GameServerStateCache::default()),
            per_node_counter_state: Arc::new(RwLock::new(
                PerNodeCounterState::default(),
            )),
            game_server_set_config: config,
        }
    }

    pub fn with_game_server_state_cache(
        mut self,
        state_cache: Arc<GameServerStateCache>,
    ) -> Self {
        self.game_server_state_cache = state_cache;
        self
    }

    pub fn with_per_node_counter_state(
        mut self,
        state: Arc<RwLock<PerNodeCounterState>>,
    ) -> Self {
        self.per_node_counter_state = state;
        self
    }
}

fn controller_owner_uid(game_server: &GameServer) -> Option<&str> {
    game_server
        .metadata
        .extensions
        .get("ownerReferences")
        .and_then(serde_json::Value::as_array)
        .and_then(|owners| {
            owners.iter().find(|owner| {
                owner
                    .get("controller")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
            })
        })
        .and_then(|owner| owner.get("uid"))
        .and_then(serde_json::Value::as_str)
}

pub fn game_server_is_controlled_by_set(
    game_server: &GameServer,
    game_server_set: &GameServerSet,
) -> bool {
    game_server
        .metadata
        .labels
        .get(GAME_SERVER_SET_GAME_SERVER_LABEL)
        == Some(&game_server_set.metadata.name)
        && controller_owner_uid(game_server)
            == Some(game_server_set.metadata.uid.as_str())
}

/// Rusternetes equivalent of upstream ListGameServersByGameServerSetOwner.
///
/// The upstream lister first uses the GameServerSet label and then validates
/// Kubernetes controller ownership by UID. Keep both checks: labels alone are
/// not sufficient when an old set is deleted and recreated with the same name.
#[cgp_fn]
#[async_trait]
pub async fn list_owned_game_servers<S>(
    &self,
    #[implicit] storage: Arc<S>,
    game_server_set: &GameServerSet,
) -> Result<Vec<GameServer>, AdapterError>
where
    S: Storage + 'static,
{
    let repository = AgonesRepository::new(storage);
    let game_servers = repository
        .list::<GameServer>(game_server_set.metadata.namespace.as_str())
        .await?;

    Ok(game_servers
        .into_iter()
        .filter(|game_server| {
            game_server_is_controlled_by_set(game_server, game_server_set)
        })
        .collect())
}

/// Persists GameServers derived from one GameServerSet.
///
/// Upstream Create goes through the GameServer mutating webhook. Rusternetes
/// controllers currently persist directly through Storage, so apply the exact
/// same GameServer defaults before persistence, then record each returned
/// server (including its generated name/UID) in the eventual-consistency cache.
#[cgp_fn]
#[async_trait]
pub async fn create_game_servers_for_set<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] game_server_state_cache: Arc<GameServerStateCache>,
    #[implicit] game_server_set_config: GameServerSetControllerConfig,
    game_server_set: &GameServerSet,
    count: i32,
) -> Result<Vec<GameServer>, AdapterError>
where
    S: Storage + 'static,
{
    if count <= 0 {
        return Ok(Vec::new());
    }
    if game_server_set_config.max_creation_parallelism == 0 {
        return Err(AdapterError::Hook(
            "GameServerSet creation parallelism must be at least one".into(),
        ));
    }

    let cache_entry =
        game_server_state_cache.for_game_server_set(game_server_set);

    futures::stream::iter(0..count as usize)
        .map(|_| {
            let storage = storage.clone();
            let cache_entry = cache_entry.clone();
            let mut game_server = game_server_set.game_server();
            game_server.apply_defaults();

            async move {
                let created = AgonesRepository::new(storage)
                    .create(&game_server)
                    .await?;
                cache_entry
                    .lock()
                    .map_err(|_| {
                        AdapterError::Hook(
                            "GameServerSet state-cache entry lock poisoned"
                                .into(),
                        )
                    })?
                    .created(&created);
                Ok::<_, AdapterError>(created)
            }
        })
        .buffer_unordered(game_server_set_config.max_creation_parallelism)
        .try_collect()
        .await
}

/// Performs the upstream scale-down write: selected GameServers transition to
/// Shutdown and are not deleted directly. The main GameServer controller owns
/// final deletion. Successful writes are immediately overlaid in the local
/// state cache to hide informer/storage propagation latency.
#[cgp_fn]
#[async_trait]
pub async fn shutdown_game_servers_for_set<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] game_server_state_cache: Arc<GameServerStateCache>,
    #[implicit] game_server_set_config: GameServerSetControllerConfig,
    game_server_set: &GameServerSet,
    game_servers: Vec<GameServer>,
) -> Result<Vec<GameServer>, AdapterError>
where
    S: Storage + 'static,
{
    if game_servers.is_empty() {
        return Ok(Vec::new());
    }
    if game_server_set_config.max_deletion_parallelism == 0 {
        return Err(AdapterError::Hook(
            "GameServerSet deletion parallelism must be at least one".into(),
        ));
    }

    let cache_entry =
        game_server_state_cache.for_game_server_set(game_server_set);

    futures::stream::iter(game_servers)
        .map(|original| {
            let storage = storage.clone();
            let cache_entry = cache_entry.clone();

            async move {
                let mut shutdown = original.clone();
                shutdown.status.state = Some(GameServerState::Shutdown);
                let updated = AgonesRepository::new(storage)
                    .update(&shutdown)
                    .await?;
                cache_entry
                    .lock()
                    .map_err(|_| {
                        AdapterError::Hook(
                            "GameServerSet state-cache entry lock poisoned"
                                .into(),
                        )
                    })?
                    .deleted(&original);
                Ok::<_, AdapterError>(updated)
            }
        })
        .buffer_unordered(game_server_set_config.max_deletion_parallelism)
        .try_collect()
        .await
}

/// Persists GameServerSet status only when it differs from the status computed
/// from the pre-write informer-equivalent snapshot, matching
/// updateStatusIfChanged.
#[cgp_fn]
#[async_trait]
pub async fn update_game_server_set_status<S>(
    &self,
    #[implicit] storage: Arc<S>,
    game_server_set: GameServerSet,
    status: GameServerSetStatus,
) -> Result<(GameServerSet, bool), AdapterError>
where
    S: Storage + 'static,
{
    if game_server_set.status == status {
        return Ok((game_server_set, false));
    }

    let mut updated = game_server_set;
    updated.status = status;
    let updated = AgonesRepository::new(storage).update(&updated).await?;
    Ok((updated, true))
}

/// Composes the already-ported deterministic GameServerSet decision logic with
/// Rusternetes persistence and the eventual-consistency state cache.
///
/// Ordering intentionally mirrors upstream syncGameServerSet:
/// list -> state-cache overlay -> compute action/status -> create -> Shutdown
/// -> status update. In particular status is computed before child writes.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_game_server_set<S>(
    &self,
    #[implicit] game_server_state_cache: Arc<GameServerStateCache>,
    #[implicit] per_node_counter_state: Arc<RwLock<PerNodeCounterState>>,
    #[implicit] game_server_set_config: GameServerSetControllerConfig,
    game_server_set: GameServerSet,
) -> Result<GameServerSetReconcileResult, AdapterError>
where
    S: Storage + 'static,
    Self: ListOwnedGameServers<S>
        + CreateGameServersForSet<S>
        + ShutdownGameServersForSet<S>
        + UpdateGameServerSetStatus<S>
        + Sync,
{
    let listed = self
        .list_owned_game_servers(&game_server_set)
        .await?;
    let cache_entry =
        game_server_state_cache.for_game_server_set(&game_server_set);
    let observed = cache_entry
        .lock()
        .map_err(|_| {
            AdapterError::Hook(
                "GameServerSet state-cache entry lock poisoned".into(),
            )
        })?
        .reconcile_with_updated_server_list(&listed);

    let counts = per_node_counter_state
        .read()
        .map_err(|_| {
            AdapterError::Hook("PerNodeCounter state lock poisoned".into())
        })?
        .counts();

    let strategy = game_server_set
        .spec
        .scheduling
        .clone()
        .unwrap_or(SchedulingStrategy::Packed);

    let mut action = compute_game_server_set_reconciliation_action(
        &strategy,
        &observed,
        &counts,
        game_server_set.spec.replicas,
        game_server_set_config.max_game_server_creations_per_batch,
        game_server_set_config.max_game_server_deletions_per_batch,
        game_server_set_config.max_pod_pending_count,
        &game_server_set.spec.priorities,
        Utc::now(),
    );

    // Upstream never creates replacements after the GameServerSet itself has
    // entered deletion.
    if game_server_set.metadata.deletion_timestamp.is_some() {
        action.num_servers_to_add = 0;
    }

    let status = compute_game_server_set_status(
        &game_server_set,
        &observed,
        game_server_set_config.player_tracking,
    );

    if action.num_servers_to_add > 0 {
        self.create_game_servers_for_set(
            &game_server_set,
            action.num_servers_to_add,
        )
        .await?;
    }

    if !action.to_delete.is_empty() {
        self.shutdown_game_servers_for_set(
            &game_server_set,
            action.to_delete.clone(),
        )
        .await?;
    }

    let (game_server_set, status_changed) = self
        .update_game_server_set_status(game_server_set, status)
        .await?;

    Ok(GameServerSetReconcileResult {
        game_server_set,
        action,
        status_changed,
    })
}

/// Fetches the current GameServerSet at execution time and runs one persistence
/// pass. Disappearance is a successful no-op, matching the upstream lister.
#[cgp_fn]
#[async_trait]
pub async fn sync_game_server_set<S>(
    &self,
    #[implicit] storage: Arc<S>,
    namespace: &str,
    name: &str,
) -> Result<GameServerSetSyncOutcome, AdapterError>
where
    S: Storage + 'static,
    Self: ReconcileGameServerSet<S> + Sync,
{
    let repository = AgonesRepository::new(storage);
    let game_server_set = match repository
        .get::<GameServerSet>(namespace, name)
        .await
    {
        Ok(game_server_set) => game_server_set,
        Err(AdapterError::Rusternetes(
            rusternetes_common::Error::NotFound(_),
        )) => return Ok(GameServerSetSyncOutcome::NotFound),
        Err(error) => return Err(error),
    };

    self.reconcile_game_server_set(game_server_set)
        .await
        .map(GameServerSetSyncOutcome::Reconciled)
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container, GameServerPort, GameServerSetSpec,
        GameServerTemplateSpec, ObjectMeta, PodSpec, PodTemplateSpec,
        PortPolicy, Protocol,
    };
    use rusternetes_storage::MemoryStorage;

    use super::*;

    fn game_server_set(replicas: i32) -> GameServerSet {
        GameServerSet {
            metadata: ObjectMeta {
                name: "set".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                replicas,
                scheduling: Some(SchedulingStrategy::Distributed),
                template: GameServerTemplateSpec {
                    spec: agones_core::GameServerSpec {
                        container: "game".into(),
                        ports: vec![GameServerPort {
                            name: "game".into(),
                            port_policy: Some(PortPolicy::Static),
                            container: Some("game".into()),
                            container_port: 7777,
                            host_port: 7777,
                            protocol: Some(Protocol::Udp),
                            ..Default::default()
                        }],
                        template: PodTemplateSpec {
                            spec: PodSpec {
                                containers: vec![Container {
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
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn config() -> GameServerSetControllerConfig {
        GameServerSetControllerConfig {
            max_creation_parallelism: 2,
            max_deletion_parallelism: 2,
            max_game_server_creations_per_batch: 100,
            max_game_server_deletions_per_batch: 100,
            max_pod_pending_count: 100,
            player_tracking: false,
        }
    }

    async fn stored_set(
        storage: &MemoryStorage,
        replicas: i32,
    ) -> GameServerSet {
        AgonesRepository::new(storage.clone())
            .create(&game_server_set(replicas))
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn owned_listing_requires_label_and_controller_uid() {
        let storage = MemoryStorage::new();
        let set = stored_set(&storage, 1).await;
        let repository = AgonesRepository::new(storage.clone());

        let mut owned = set.game_server();
        owned.metadata.name = "owned".into();
        owned.metadata.generate_name.clear();
        owned.apply_defaults();
        repository.create(&owned).await.unwrap();

        let mut wrong_uid = set.game_server();
        wrong_uid.metadata.name = "old-owner".into();
        wrong_uid.metadata.generate_name.clear();
        wrong_uid
            .metadata
            .extensions
            .get_mut("ownerReferences")
            .and_then(serde_json::Value::as_array_mut)
            .unwrap()[0]["uid"] = serde_json::json!("different-uid");
        wrong_uid.apply_defaults();
        repository.create(&wrong_uid).await.unwrap();

        let mut wrong_label = set.game_server();
        wrong_label.metadata.name = "wrong-label".into();
        wrong_label.metadata.generate_name.clear();
        wrong_label.metadata.labels.insert(
            GAME_SERVER_SET_GAME_SERVER_LABEL.into(),
            "another-set".into(),
        );
        wrong_label.apply_defaults();
        repository.create(&wrong_label).await.unwrap();

        let controller = GameServerSetController::new(
            storage,
            config(),
        );
        let listed = controller
            .list_owned_game_servers(&set)
            .await
            .unwrap();

        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].metadata.name, "owned");
    }

    #[tokio::test]
    async fn scale_up_uses_generate_name_applies_defaults_and_updates_status_next_pass() {
        let storage = MemoryStorage::new();
        let set = stored_set(&storage, 2).await;
        let repository = AgonesRepository::new(storage.clone());
        let controller = GameServerSetController::new(
            storage.clone(),
            config(),
        );

        let first = controller
            .sync_game_server_set("default", "set")
            .await
            .unwrap();
        let GameServerSetSyncOutcome::Reconciled(first) = first else {
            panic!("stored GameServerSet should reconcile");
        };
        assert_eq!(first.action.num_servers_to_add, 2);
        assert_eq!(first.game_server_set.status.replicas, 0);

        let mut created = repository.list::<GameServer>("default").await.unwrap();
        created.sort_by(|left, right| {
            left.metadata.name.cmp(&right.metadata.name)
        });
        assert_eq!(created.len(), 2);
        for game_server in &created {
            assert!(game_server.metadata.name.starts_with("set-"));
            assert!(!game_server.metadata.uid.is_empty());
            assert_eq!(
                game_server.status.state,
                Some(GameServerState::Creating)
            );
            assert!(
                game_server
                    .metadata
                    .finalizers
                    .iter()
                    .any(|value| value == agones_core::FINALIZER_NAME)
            );
            assert!(game_server_is_controlled_by_set(game_server, &set));
        }

        let second = controller
            .sync_game_server_set("default", "set")
            .await
            .unwrap();
        let GameServerSetSyncOutcome::Reconciled(second) = second else {
            panic!("stored GameServerSet should reconcile");
        };
        assert_eq!(second.action.num_servers_to_add, 0);
        assert!(second.status_changed);
        assert_eq!(second.game_server_set.status.replicas, 2);
    }

    #[tokio::test]
    async fn scale_down_transitions_children_to_shutdown_without_deleting_them() {
        let storage = MemoryStorage::new();
        let mut set = stored_set(&storage, 3).await;
        let repository = AgonesRepository::new(storage.clone());
        let controller = GameServerSetController::new(
            storage.clone(),
            config(),
        );

        controller
            .sync_game_server_set("default", "set")
            .await
            .unwrap();

        let children = repository.list::<GameServer>("default").await.unwrap();
        for mut game_server in children {
            game_server.status.state = Some(GameServerState::Ready);
            repository.update(&game_server).await.unwrap();
        }

        set = repository.get("default", "set").await.unwrap();
        set.spec.replicas = 1;
        repository.update(&set).await.unwrap();

        let outcome = controller
            .sync_game_server_set("default", "set")
            .await
            .unwrap();
        let GameServerSetSyncOutcome::Reconciled(result) = outcome else {
            panic!("stored GameServerSet should reconcile");
        };
        assert_eq!(result.action.to_delete.len(), 2);

        let children = repository.list::<GameServer>("default").await.unwrap();
        assert_eq!(children.len(), 3);
        assert_eq!(
            children
                .iter()
                .filter(|game_server| {
                    game_server.status.state
                        == Some(GameServerState::Shutdown)
                })
                .count(),
            2
        );
        assert_eq!(
            children
                .iter()
                .filter(|game_server| {
                    game_server.status.state
                        == Some(GameServerState::Ready)
                })
                .count(),
            1
        );
    }

    #[tokio::test]
    async fn deleting_gameserverset_never_scales_up() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let mut set = stored_set(&storage, 3).await;
        set.metadata.deletion_timestamp =
            Some("2026-10-18T00:00:00Z".into());
        repository.update(&set).await.unwrap();

        let controller = GameServerSetController::new(
            storage,
            config(),
        );
        let outcome = controller
            .sync_game_server_set("default", "set")
            .await
            .unwrap();
        let GameServerSetSyncOutcome::Reconciled(result) = outcome else {
            panic!("stored GameServerSet should reconcile");
        };

        assert_eq!(result.action.num_servers_to_add, 0);
        assert!(
            repository
                .list::<GameServer>("default")
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn batch_limit_reports_partial_work_for_runtime_requeue() {
        let storage = MemoryStorage::new();
        stored_set(&storage, 5).await;
        let mut limited = config();
        limited.max_game_server_creations_per_batch = 2;

        let controller = GameServerSetController::new(
            storage.clone(),
            limited,
        );
        let outcome = controller
            .sync_game_server_set("default", "set")
            .await
            .unwrap();
        let GameServerSetSyncOutcome::Reconciled(result) = outcome else {
            panic!("stored GameServerSet should reconcile");
        };

        assert_eq!(result.action.num_servers_to_add, 2);
        assert!(result.is_partial());
        assert_eq!(
            AgonesRepository::new(storage)
                .list::<GameServer>("default")
                .await
                .unwrap()
                .len(),
            2
        );
    }

    #[tokio::test]
    async fn missing_gameserverset_is_successful_noop() {
        let controller = GameServerSetController::new(
            MemoryStorage::new(),
            config(),
        );

        assert!(matches!(
            controller
                .sync_game_server_set("default", "missing")
                .await
                .unwrap(),
            GameServerSetSyncOutcome::NotFound
        ));
    }
}
