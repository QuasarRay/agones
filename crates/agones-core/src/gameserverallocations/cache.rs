use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use agones_core::GameServer;

#[derive(Debug, Default)]
pub struct GameServerCache {
    cache: RwLock<HashMap<String, Arc<GameServer>>>,
}

impl GameServerCache {
    /// Stores a GameServer unless the update is stale or already represented.
    pub fn store(&self, key: String, game_server: &GameServer) {
        let mut cache = self
            .cache
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        if let Some(existing) = cache.get(&key) {
            let incoming_generation = generation(game_server);
            let existing_generation = generation(existing);

            if incoming_generation < existing_generation
                || game_server.metadata.resource_version
                    == existing.metadata.resource_version
            {
                return;
            }
        }

        cache.insert(key, Arc::new(game_server.clone()));
    }

    /// Deletes a GameServer and reports whether it existed.
    pub fn delete(&self, key: &str) -> bool {
        self.cache
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(key)
            .is_some()
    }

    /// Returns the cached GameServer if present.
    pub fn load(&self, key: &str) -> Option<Arc<GameServer>> {
        self.cache
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(key)
            .cloned()
    }

    /// Visits cached GameServers until the callback returns false.
    pub fn range(
        &self,
        mut f: impl FnMut(&str, &GameServer) -> bool,
    ) {
        let cache = self
            .cache
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        for (key, game_server) in cache.iter() {
            if !f(key, game_server) {
                break;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.cache
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// `generation` is currently retained in ObjectMeta's flattened extension map.
fn generation(game_server: &GameServer) -> i64 {
    game_server
        .metadata
        .extensions
        .get("generation")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or_default()
}