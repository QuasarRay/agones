use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{Arc, Mutex, RwLock};

use agones_core::{GameServer, GameServerSet};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GameServerSetCacheEntry {
    pending_creation: BTreeMap<String, GameServer>,
    pending_deletion: BTreeMap<String, GameServer>,
}

impl GameServerSetCacheEntry {
    /// Records a GameServer created locally but not necessarily visible from
    /// the backing list/watch path yet.
    pub fn created(&mut self, game_server: &GameServer) {
        self.pending_creation.insert(
            game_server.metadata.name.clone(),
            game_server.clone(),
        );
    }

    /// Records a GameServer deleted locally and overlays a deletion timestamp
    /// until the backing list/watch path observes the deletion.
    pub fn deleted_at(
        &mut self,
        game_server: &GameServer,
        deletion_timestamp: DateTime<Utc>,
    ) {
        self.pending_creation
            .remove(game_server.metadata.name.as_str());

        let mut deleted = game_server.clone();
        deleted.metadata.deletion_timestamp =
            Some(deletion_timestamp.to_rfc3339());

        self.pending_deletion
            .insert(game_server.metadata.name.clone(), deleted);
    }

    pub fn deleted(&mut self, game_server: &GameServer) {
        self.deleted_at(game_server, Utc::now());
    }

    pub fn pending_creation_len(&self) -> usize {
        self.pending_creation.len()
    }

    pub fn pending_deletion_len(&self) -> usize {
        self.pending_deletion.len()
    }

    /// Reconciles the latest backing-store list with locally issued creates and
    /// deletes, matching upstream `gameServerSetCacheEntry.reconcileWithUpdatedServerList`.
    pub fn reconcile_with_updated_server_list(
        &mut self,
        list: &[GameServer],
    ) -> Vec<GameServer> {
        let mut result = Vec::with_capacity(
            list.len() + self.pending_creation.len(),
        );
        let mut found = BTreeSet::new();

        for game_server in list {
            let name = game_server.metadata.name.as_str();

            if let Some(deleted) =
                self.pending_deletion.get(name).cloned()
            {
                result.push(deleted);
                if game_server.metadata.deletion_timestamp.is_some() {
                    self.pending_deletion.remove(name);
                }
            } else {
                result.push(game_server.clone());
            }

            // Once a locally created GameServer appears in the backing list,
            // the backing copy becomes authoritative.
            self.pending_creation.remove(name);
            found.insert(name.to_owned());
        }

        // A locally deleted object that no longer appears in the backing list
        // has completed deletion and no longer needs an overlay.
        self.pending_deletion
            .retain(|name, _| found.contains(name));

        // Keep locally created GameServers visible until they are observed.
        result.extend(self.pending_creation.values().cloned());
        result
    }
}

#[derive(Debug, Default)]
pub struct GameServerStateCache {
    cache: RwLock<
        HashMap<String, Arc<Mutex<GameServerSetCacheEntry>>>,
    >,
}

impl GameServerStateCache {
    fn key(game_server_set: &GameServerSet) -> String {
        format!(
            "{}/{}",
            game_server_set.metadata.namespace,
            game_server_set.metadata.name
        )
    }

    pub fn for_game_server_set(
        &self,
        game_server_set: &GameServerSet,
    ) -> Arc<Mutex<GameServerSetCacheEntry>> {
        let key = Self::key(game_server_set);

        if let Some(entry) = self
            .cache
            .read()
            .expect("GameServer state-cache read lock poisoned")
            .get(key.as_str())
            .cloned()
        {
            return entry;
        }

        self.cache
            .write()
            .expect("GameServer state-cache write lock poisoned")
            .entry(key)
            .or_insert_with(|| {
                Arc::new(Mutex::new(
                    GameServerSetCacheEntry::default(),
                ))
            })
            .clone()
    }

    pub fn delete_game_server_set(
        &self,
        game_server_set: &GameServerSet,
    ) {
        self.cache
            .write()
            .expect("GameServer state-cache write lock poisoned")
            .remove(Self::key(game_server_set).as_str());
    }

    pub fn len(&self) -> usize {
        self.cache
            .read()
            .expect("GameServer state-cache read lock poisoned")
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agones_core::{GameServerState, GameServerStatus};

    fn game_server(name: &str) -> GameServer {
        GameServer {
            metadata: agones_core::ObjectMeta {
                name: name.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn deleted(game_server: &GameServer) -> GameServer {
        let mut deleted = game_server.clone();
        deleted.metadata.deletion_timestamp =
            Some("2026-10-18T00:00:00+00:00".into());
        deleted
    }

    fn status_info(
        list: &[GameServer],
    ) -> Vec<(String, Option<GameServerState>, bool)> {
        let mut result = list
            .iter()
            .map(|game_server| {
                (
                    game_server.metadata.name.clone(),
                    game_server.status.state.clone(),
                    game_server.metadata.deletion_timestamp.is_some(),
                )
            })
            .collect::<Vec<_>>();
        result.sort_by(|left, right| left.0.cmp(&right.0));
        result
    }

    #[test]
    fn state_cache_keys_entries_by_namespace_and_name() {
        let cache = GameServerStateCache::default();

        let set1 = GameServerSet {
            metadata: agones_core::ObjectMeta {
                name: "set-1".into(),
                namespace: "ns1".into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let set1_other_namespace = GameServerSet {
            metadata: agones_core::ObjectMeta {
                name: "set-1".into(),
                namespace: "ns2".into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let set2 = GameServerSet {
            metadata: agones_core::ObjectMeta {
                name: "set-2".into(),
                namespace: "ns1".into(),
                ..Default::default()
            },
            ..Default::default()
        };

        let entry1 = cache.for_game_server_set(&set1);
        assert!(Arc::ptr_eq(
            &entry1,
            &cache.for_game_server_set(&set1)
        ));
        assert!(!Arc::ptr_eq(
            &entry1,
            &cache.for_game_server_set(
                &set1_other_namespace
            )
        ));
        assert!(!Arc::ptr_eq(
            &entry1,
            &cache.for_game_server_set(&set2)
        ));
        assert_eq!(cache.len(), 3);

        cache.delete_game_server_set(&set1);
        let replacement = cache.for_game_server_set(&set1);
        assert!(!Arc::ptr_eq(&entry1, &replacement));
    }

    #[test]
    fn locally_created_server_remains_visible_until_observed() {
        let gs1 = game_server("gs-1");
        let gs2 = game_server("gs-2");
        let mut entry = GameServerSetCacheEntry::default();

        entry.created(&gs1);
        let first =
            entry.reconcile_with_updated_server_list(&[gs2.clone()]);

        assert_eq!(
            status_info(&first),
            status_info(&[gs1.clone(), gs2])
        );
        assert_eq!(entry.pending_creation_len(), 1);

        let second =
            entry.reconcile_with_updated_server_list(&first);
        assert_eq!(status_info(&second), status_info(&first));
        assert_eq!(entry.pending_creation_len(), 0);
        assert_eq!(entry.pending_deletion_len(), 0);
    }

    #[test]
    fn locally_deleted_server_stays_deleted_until_store_catches_up() {
        let gs1 = game_server("gs-1");
        let mut entry = GameServerSetCacheEntry::default();
        let now = DateTime::parse_from_rfc3339(
            "2026-10-18T00:00:00Z",
        )
        .unwrap()
        .with_timezone(&Utc);

        entry.deleted_at(&gs1, now);
        let first =
            entry.reconcile_with_updated_server_list(&[gs1.clone()]);

        assert_eq!(status_info(&first), status_info(&[deleted(&gs1)]));
        assert_eq!(entry.pending_deletion_len(), 1);

        let second =
            entry.reconcile_with_updated_server_list(&first);
        assert_eq!(status_info(&second), status_info(&first));
        assert_eq!(entry.pending_deletion_len(), 0);
    }

    #[test]
    fn locally_deleted_server_is_retired_when_absent_from_store() {
        let gs1 = game_server("gs-1");
        let mut entry = GameServerSetCacheEntry::default();
        entry.deleted(&gs1);

        let result = entry.reconcile_with_updated_server_list(&[]);
        assert!(result.is_empty());
        assert_eq!(entry.pending_deletion_len(), 0);
    }

    #[test]
    fn create_then_delete_cancels_pending_creation() {
        let gs1 = game_server("gs-1");
        let mut entry = GameServerSetCacheEntry::default();

        entry.created(&gs1);
        entry.deleted(&gs1);

        assert_eq!(entry.pending_creation_len(), 0);
        assert_eq!(entry.pending_deletion_len(), 1);

        let observed = vec![gs1.clone()];
        let first =
            entry.reconcile_with_updated_server_list(&observed);
        assert_eq!(status_info(&first), status_info(&[deleted(&gs1)]));
        assert_eq!(entry.pending_deletion_len(), 1);

        let second =
            entry.reconcile_with_updated_server_list(&first);
        assert_eq!(status_info(&second), status_info(&first));
        assert_eq!(entry.pending_deletion_len(), 0);
    }

    #[test]
    fn status_is_preserved_through_local_overlay() {
        let mut gs1 = game_server("gs-1");
        gs1.status = GameServerStatus {
            state: Some(GameServerState::Ready),
            ..Default::default()
        };

        let mut entry = GameServerSetCacheEntry::default();
        entry.created(&gs1);
        let result = entry.reconcile_with_updated_server_list(&[]);

        assert_eq!(
            result[0].status.state,
            Some(GameServerState::Ready)
        );
    }
}
