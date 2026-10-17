use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use agones_core::{GameServer, GameServerState};
pub use agones_core::NodeCount;
use agones_macros::agones_invariant;
use cgp::prelude::*;
use futures::{StreamExt, stream::BoxStream};
use rusternetes_common::resources::Node;
use rusternetes_storage::{Storage, WatchEvent, build_prefix};

use crate::{AdapterError, AgonesRepository, AgonesWatchEvent};


#[derive(Clone, Debug, PartialEq, Eq)]
struct ProcessedGameServer {
    resource_version: Option<String>,
    state: Option<GameServerState>,
    node_name: String,
}

#[derive(Clone, Debug, Default)]
pub struct PerNodeCounterState {
    counts: HashMap<String, NodeCount>,
    processed: HashMap<String, ProcessedGameServer>,
}

impl PerNodeCounterState {
    pub fn counts(&self) -> HashMap<String, NodeCount> {
        self.counts.clone()
    }

    pub fn processed_len(&self) -> usize {
        self.processed.len()
    }

    pub fn rebuild<I>(&mut self, game_servers: I)
    where
        I: IntoIterator<Item = GameServer>,
    {
        let mut counts = HashMap::<String, NodeCount>::new();
        let mut processed = HashMap::<String, ProcessedGameServer>::new();

        for game_server in game_servers {
            let node_name = game_server.status.node_name.clone();
            let count = counts.entry(node_name).or_default();

            match game_server.status.state.as_ref() {
                Some(GameServerState::Ready) => count.ready += 1,
                Some(GameServerState::Allocated) => count.allocated += 1,
                _ => {}
            }

            update_processed(&mut processed, &game_server);
        }

        self.counts = counts;
        self.processed = processed;
    }

    pub fn add_game_server(&mut self, game_server: &GameServer) {
        let uid = game_server.metadata.uid.clone();

        if let Some(previous) = self.processed.get(&uid).cloned() {
            if previous.resource_version == game_server.metadata.resource_version
                && previous.state == game_server.status.state
            {
                return;
            }

            if previous.state != game_server.status.state {
                let (ready, allocated) = calculate_state_transition(
                    previous.state.as_ref(),
                    game_server.status.state.as_ref(),
                );
                update_processed(&mut self.processed, game_server);
                self.increment(game_server.status.node_name.as_str(), ready, allocated);
            }

            // Upstream intentionally does not refresh processed metadata for an
            // Add event that has only a new ResourceVersion.
            return;
        }

        update_processed(&mut self.processed, game_server);
        match game_server.status.state.as_ref() {
            Some(GameServerState::Ready) => {
                self.increment(game_server.status.node_name.as_str(), 1, 0);
            }
            Some(GameServerState::Allocated) => {
                self.increment(game_server.status.node_name.as_str(), 0, 1);
            }
            _ => {}
        }
    }

    pub fn update_game_server(&mut self, old: &GameServer, new: &GameServer) {
        if self.is_already_processed(
            new.metadata.uid.as_str(),
            new.metadata.resource_version.as_ref(),
        ) {
            return;
        }

        let old_state = self
            .processed
            .get(new.metadata.uid.as_str())
            .and_then(|processed| processed.state.as_ref())
            .or(old.status.state.as_ref());

        let (ready, allocated) =
            calculate_state_transition(old_state, new.status.state.as_ref());

        update_processed(&mut self.processed, new);
        self.increment(new.status.node_name.as_str(), ready, allocated);
    }

    pub fn delete_game_server(&mut self, game_server: &GameServer) {
        let tracked = self.processed.get(game_server.metadata.uid.as_str()).cloned();

        let (state, node_name) = tracked
            .map(|processed| (processed.state, processed.node_name))
            .unwrap_or_else(|| {
                (
                    game_server.status.state.clone(),
                    game_server.status.node_name.clone(),
                )
            });

        match state.as_ref() {
            Some(GameServerState::Ready) => self.increment(node_name.as_str(), -1, 0),
            Some(GameServerState::Allocated) => {
                self.increment(node_name.as_str(), 0, -1);
            }
            _ => {}
        }

        self.processed.remove(game_server.metadata.uid.as_str());
    }

    pub fn delete_node(&mut self, node_name: &str) {
        self.counts.remove(node_name);
    }

    pub fn apply_event(&mut self, event: PerNodeCounterEvent) {
        match event {
            PerNodeCounterEvent::Added(game_server) => {
                self.add_game_server(&game_server);
            }
            PerNodeCounterEvent::Modified { old, new } => {
                self.update_game_server(&old, &new);
            }
            PerNodeCounterEvent::Deleted(game_server) => {
                self.delete_game_server(&game_server);
            }
            PerNodeCounterEvent::NodeDeleted(node_name) => {
                self.delete_node(&node_name);
            }
        }
    }

    fn is_already_processed(
        &self,
        uid: &str,
        resource_version: Option<&String>,
    ) -> bool {
        self.processed
            .get(uid)
            .is_some_and(|processed| {
                processed.resource_version.as_ref() == resource_version
            })
    }

    fn increment(&mut self, node_name: &str, ready: i64, allocated: i64) {
        let count = self.counts.entry(node_name.to_owned()).or_default();
        count.ready += ready;
        count.allocated += allocated;

        // Preserve upstream's defensive clamp against duplicate/out-of-order
        // events driving either count negative.
        count.ready = count.ready.max(0);
        count.allocated = count.allocated.max(0);
    }
}

fn update_processed(
    processed: &mut HashMap<String, ProcessedGameServer>,
    game_server: &GameServer,
) {
    processed.insert(
        game_server.metadata.uid.clone(),
        ProcessedGameServer {
            resource_version: game_server.metadata.resource_version.clone(),
            state: game_server.status.state.clone(),
            node_name: game_server.status.node_name.clone(),
        },
    );
}

/// Mirrors upstream PerNodeCounter.calculateStateTransition.
pub fn calculate_state_transition(
    old_state: Option<&GameServerState>,
    new_state: Option<&GameServerState>,
) -> (i64, i64) {
    let old_ready = old_state == Some(&GameServerState::Ready);
    let new_ready = new_state == Some(&GameServerState::Ready);
    let old_allocated = old_state == Some(&GameServerState::Allocated);
    let new_allocated = new_state == Some(&GameServerState::Allocated);

    let ready = match (old_ready, new_ready) {
        (true, false) => -1,
        (false, true) => 1,
        _ => 0,
    };
    let allocated = match (old_allocated, new_allocated) {
        (true, false) => -1,
        (false, true) => 1,
        _ => 0,
    };

    (ready, allocated)
}

#[agones_invariant]
fn state_transition_deltas_match_membership(
    input: (bool, bool, bool, bool),
) -> bool {
    let (old_ready, old_allocated, new_ready, new_allocated) = input;

    // A GameServer has one state, so Ready+Allocated simultaneously is outside
    // the domain represented by the upstream enum.
    if (old_ready && old_allocated) || (new_ready && new_allocated) {
        return true;
    }

    fn state(ready: bool, allocated: bool) -> GameServerState {
        if ready {
            GameServerState::Ready
        } else if allocated {
            GameServerState::Allocated
        } else {
            GameServerState::Starting
        }
    }

    let old = state(old_ready, old_allocated);
    let new = state(new_ready, new_allocated);
    let (ready_delta, allocated_delta) =
        calculate_state_transition(Some(&old), Some(&new));

    ready_delta == i64::from(new_ready) - i64::from(old_ready)
        && allocated_delta
            == i64::from(new_allocated) - i64::from(old_allocated)
}

#[derive(Clone, Debug)]
pub enum PerNodeCounterEvent {
    Added(GameServer),
    Modified {
        old: GameServer,
        new: GameServer,
    },
    Deleted(GameServer),
    NodeDeleted(String),
}

#[derive(HasField)]
pub struct PerNodeCounterController<S> {
    pub storage: Arc<S>,
    pub per_node_counter_state: Arc<RwLock<PerNodeCounterState>>,
}

impl<S> PerNodeCounterController<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage: Arc::new(storage),
            per_node_counter_state: Arc::new(RwLock::new(PerNodeCounterState::default())),
        }
    }

    pub fn counts(&self) -> Result<HashMap<String, NodeCount>, AdapterError> {
        self.per_node_counter_state
            .read()
            .map(|state| state.counts())
            .map_err(|_| AdapterError::Hook("PerNodeCounter state lock poisoned".into()))
    }

    pub fn processed_len(&self) -> Result<usize, AdapterError> {
        self.per_node_counter_state
            .read()
            .map(|state| state.processed_len())
            .map_err(|_| AdapterError::Hook("PerNodeCounter state lock poisoned".into()))
    }
}

/// CGP capability for the initial PerNodeCounter informer-cache snapshot.
///
/// The upstream Run method initializes counts for every observed node, including
/// nodes whose GameServers are in non-counted states, and records every
/// GameServer in the duplicate-event tracker.
#[cgp_fn]
#[async_trait]
pub async fn rebuild_per_node_counter<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] per_node_counter_state: Arc<RwLock<PerNodeCounterState>>,
) -> Result<HashMap<String, NodeCount>, AdapterError>
where
    S: Storage + 'static,
{
    let repository = AgonesRepository::new(storage);
    let game_servers = repository.list_all::<GameServer>().await?;
    let mut state = per_node_counter_state
        .write()
        .map_err(|_| AdapterError::Hook("PerNodeCounter state lock poisoned".into()))?;
    state.rebuild(game_servers);
    Ok(state.counts())
}

/// CGP capability for the upstream informer Add/Update/Delete reducers plus
/// Node deletion cleanup.
///
/// This remains the deterministic reducer shared by direct callers and the
/// live Rusternetes watch runner.
#[cgp_fn]
#[async_trait]
pub async fn apply_per_node_counter_event(
    &self,
    #[implicit] per_node_counter_state: Arc<RwLock<PerNodeCounterState>>,
    event: PerNodeCounterEvent,
) -> Result<(), AdapterError> {
    let mut state = per_node_counter_state
        .write()
        .map_err(|_| AdapterError::Hook("PerNodeCounter state lock poisoned".into()))?;
    state.apply_event(event);
    Ok(())
}

pub type PerNodeCounterWatchStream =
    BoxStream<'static, Result<PerNodeCounterEvent, AdapterError>>;

/// Builds the live Rusternetes streams corresponding to the upstream
/// GameServer and Node informer handlers.
///
/// GameServer Modified events do not carry an old object at the Storage layer.
/// The reducer intentionally reconstructs the old state from its processed map,
/// just as upstream does to tolerate duplicate informer events.
pub async fn watch_per_node_counter_events<S>(
    storage: Arc<S>,
) -> Result<PerNodeCounterWatchStream, AdapterError>
where
    S: Storage + 'static,
{
    let repository = AgonesRepository::new(storage.clone());
    let game_server_stream = repository.watch_all::<GameServer>().await?;
    let node_prefix = build_prefix("nodes", None);
    let node_stream = storage.watch(&node_prefix).await?;

    let game_server_events = game_server_stream.map(|event| {
        event.map(|event| match event {
            AgonesWatchEvent::Added { resource, .. } => {
                PerNodeCounterEvent::Added(resource)
            }
            AgonesWatchEvent::Modified { resource, .. } => {
                PerNodeCounterEvent::Modified {
                    old: resource.clone(),
                    new: resource,
                }
            }
            AgonesWatchEvent::Deleted { resource, .. } => {
                PerNodeCounterEvent::Deleted(resource)
            }
        })
    });

    let node_events = node_stream.filter_map(|event| async move {
        match event {
            Ok(WatchEvent::Deleted(_, raw)) => Some(
                serde_json::from_str::<Node>(&raw)
                    .map(|node| PerNodeCounterEvent::NodeDeleted(node.metadata.name))
                    .map_err(AdapterError::from),
            ),
            Ok(WatchEvent::Added(_, _) | WatchEvent::Modified(_, _)) => None,
            Err(error) => Some(Err(AdapterError::from(error))),
        }
    });

    Ok(Box::pin(futures::stream::select(
        game_server_events,
        node_events,
    )))
}

/// CGP informer-equivalent runner for PerNodeCounter.
///
/// Watches are subscribed before the cluster-wide snapshot is rebuilt. Events
/// that race with the snapshot are therefore queued and then deduplicated by
/// the same UID/resourceVersion state machine as upstream. The returned future
/// stays active until the underlying Rusternetes watch streams finish or fail;
/// callers can cancel it through their task/runtime lifecycle.
#[cgp_fn]
#[async_trait]
pub async fn run_per_node_counter<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] per_node_counter_state: Arc<RwLock<PerNodeCounterState>>,
) -> Result<(), AdapterError>
where
    S: Storage + 'static,
{
    let mut events = watch_per_node_counter_events(storage.clone()).await?;

    let repository = AgonesRepository::new(storage);
    let game_servers = repository.list_all::<GameServer>().await?;
    {
        let mut state = per_node_counter_state
            .write()
            .map_err(|_| AdapterError::Hook("PerNodeCounter state lock poisoned".into()))?;
        state.rebuild(game_servers);
    }

    while let Some(event) = events.next().await {
        let event = event?;
        let mut state = per_node_counter_state
            .write()
            .map_err(|_| AdapterError::Hook("PerNodeCounter state lock poisoned".into()))?;
        state.apply_event(event);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use agones_core::{GameServerStatus, ObjectMeta};
    use rusternetes_common::resources::Node;
    use rusternetes_storage::{MemoryStorage, Storage, build_key};

    use crate::AgonesRepository;

    use super::*;

    fn game_server(
        name: &str,
        uid: &str,
        resource_version: &str,
        state: GameServerState,
        node_name: &str,
    ) -> GameServer {
        GameServer {
            metadata: ObjectMeta {
                name: name.into(),
                namespace: "default".into(),
                uid: uid.into(),
                resource_version: Some(resource_version.into()),
                ..Default::default()
            },
            status: GameServerStatus {
                state: Some(state),
                node_name: node_name.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn state_transition_matches_upstream_ready_and_allocated_deltas() {
        let cases = [
            (
                GameServerState::Starting,
                GameServerState::Ready,
                (1, 0),
            ),
            (
                GameServerState::Ready,
                GameServerState::Allocated,
                (-1, 1),
            ),
            (
                GameServerState::Allocated,
                GameServerState::Shutdown,
                (0, -1),
            ),
            (
                GameServerState::Ready,
                GameServerState::Ready,
                (0, 0),
            ),
            (
                GameServerState::Starting,
                GameServerState::Starting,
                (0, 0),
            ),
        ];

        for (old, new, expected) in cases {
            assert_eq!(
                calculate_state_transition(Some(&old), Some(&new)),
                expected
            );
        }
    }

    #[test]
    fn add_events_track_state_and_deduplicate_exact_versions() {
        let mut state = PerNodeCounterState::default();
        let starting = game_server(
            "server",
            "uid-server",
            "1",
            GameServerState::Starting,
            "node1",
        );

        state.add_game_server(&starting);
        assert!(state.counts().is_empty());
        assert_eq!(state.processed_len(), 1);

        let mut ready = starting.clone();
        ready.status.state = Some(GameServerState::Ready);
        ready.metadata.resource_version = Some("2".into());
        state.add_game_server(&ready);
        state.add_game_server(&ready);

        assert_eq!(
            state.counts()["node1"],
            NodeCount {
                ready: 1,
                allocated: 0,
            }
        );

        let mut same_state_new_version = ready.clone();
        same_state_new_version.metadata.resource_version = Some("3".into());
        state.add_game_server(&same_state_new_version);
        assert_eq!(state.counts()["node1"].ready, 1);
    }

    #[test]
    fn update_events_use_tracked_previous_state_and_ignore_duplicates() {
        let mut state = PerNodeCounterState::default();
        let ready = game_server(
            "server",
            "uid-server",
            "1",
            GameServerState::Ready,
            "node1",
        );
        state.add_game_server(&ready);

        let mut allocated = ready.clone();
        allocated.status.state = Some(GameServerState::Allocated);
        allocated.metadata.resource_version = Some("2".into());

        state.update_game_server(&ready, &allocated);
        state.update_game_server(&ready, &allocated);

        assert_eq!(
            state.counts()["node1"],
            NodeCount {
                ready: 0,
                allocated: 1,
            }
        );
    }

    #[test]
    fn delete_uses_tracked_state_and_removes_processed_entry() {
        let mut state = PerNodeCounterState::default();
        let allocated = game_server(
            "server",
            "uid-server",
            "1",
            GameServerState::Allocated,
            "node1",
        );
        state.add_game_server(&allocated);

        let mut tombstone = allocated.clone();
        tombstone.status.state = Some(GameServerState::Shutdown);
        tombstone.metadata.resource_version = Some("2".into());
        state.delete_game_server(&tombstone);

        assert_eq!(
            state.counts()["node1"],
            NodeCount {
                ready: 0,
                allocated: 0,
            }
        );
        assert_eq!(state.processed_len(), 0);
    }

    #[test]
    fn defensive_decrement_never_produces_negative_counts() {
        let mut state = PerNodeCounterState::default();
        let ready = game_server(
            "server",
            "uid-server",
            "1",
            GameServerState::Ready,
            "node1",
        );

        state.delete_game_server(&ready);
        assert_eq!(state.counts()["node1"].ready, 0);
    }

    #[test]
    fn deleting_node_removes_only_its_counter_record() {
        let mut state = PerNodeCounterState::default();
        state.add_game_server(&game_server(
            "server1",
            "uid-1",
            "1",
            GameServerState::Ready,
            "node1",
        ));
        state.add_game_server(&game_server(
            "server2",
            "uid-2",
            "1",
            GameServerState::Allocated,
            "node2",
        ));

        state.delete_node("node1");
        let counts = state.counts();
        assert!(!counts.contains_key("node1"));
        assert_eq!(counts["node2"].allocated, 1);
    }

    #[tokio::test]
    async fn rebuild_matches_upstream_initial_snapshot_semantics() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());

        for game_server in [
            game_server(
                "ready",
                "uid-ready",
                "1",
                GameServerState::Ready,
                "node1",
            ),
            game_server(
                "allocated1",
                "uid-allocated1",
                "1",
                GameServerState::Allocated,
                "node1",
            ),
            game_server(
                "starting",
                "uid-starting",
                "1",
                GameServerState::Starting,
                "node2",
            ),
            game_server(
                "allocated2",
                "uid-allocated2",
                "1",
                GameServerState::Allocated,
                "node1",
            ),
        ] {
            repository.create(&game_server).await.unwrap();
        }

        let controller = PerNodeCounterController::new(storage);
        let counts = controller.rebuild_per_node_counter().await.unwrap();

        assert_eq!(
            counts["node1"],
            NodeCount {
                ready: 1,
                allocated: 2,
            }
        );
        assert_eq!(
            counts["node2"],
            NodeCount {
                ready: 0,
                allocated: 0,
            }
        );
        assert_eq!(controller.processed_len().unwrap(), 4);
    }

    #[tokio::test]
    async fn cgp_event_capability_preserves_duplicate_event_behavior() {
        let controller = PerNodeCounterController::new(MemoryStorage::new());
        let ready = game_server(
            "server",
            "uid-server",
            "1",
            GameServerState::Ready,
            "node1",
        );

        controller
            .apply_per_node_counter_event(PerNodeCounterEvent::Added(
                ready.clone(),
            ))
            .await
            .unwrap();
        controller
            .apply_per_node_counter_event(PerNodeCounterEvent::Added(ready))
            .await
            .unwrap();

        assert_eq!(
            controller.counts().unwrap()["node1"],
            NodeCount {
                ready: 1,
                allocated: 0,
            }
        );

        controller
            .apply_per_node_counter_event(PerNodeCounterEvent::NodeDeleted(
                "node1".into(),
            ))
            .await
            .unwrap();
        assert!(controller.counts().unwrap().is_empty());
    }

    async fn next_counter_event(
        events: &mut PerNodeCounterWatchStream,
    ) -> PerNodeCounterEvent {
        tokio::time::timeout(std::time::Duration::from_secs(2), events.next())
            .await
            .expect("PerNodeCounter watch event timed out")
            .expect("PerNodeCounter watch stream ended")
            .expect("PerNodeCounter watch event failed")
    }

    #[tokio::test]
    async fn watch_stream_translates_gameserver_and_node_events() {
        let storage = Arc::new(MemoryStorage::new());
        let repository = AgonesRepository::new(storage.clone());
        let mut events = watch_per_node_counter_events(storage.clone())
            .await
            .unwrap();

        let ready = game_server(
            "server",
            "uid-server",
            "1",
            GameServerState::Ready,
            "node1",
        );
        let created = repository.create(&ready).await.unwrap();

        match next_counter_event(&mut events).await {
            PerNodeCounterEvent::Added(game_server) => {
                assert_eq!(game_server.metadata.uid, created.metadata.uid);
                assert_eq!(game_server.status.state, Some(GameServerState::Ready));
            }
            event => panic!("expected GameServer Added event, got {event:?}"),
        }

        let mut allocated = created.clone();
        allocated.metadata.resource_version = Some("2".into());
        allocated.status.state = Some(GameServerState::Allocated);
        let updated = repository.update(&allocated).await.unwrap();

        match next_counter_event(&mut events).await {
            PerNodeCounterEvent::Modified { old, new } => {
                assert_eq!(old, new);
                assert_eq!(new.metadata.uid, updated.metadata.uid);
                assert_eq!(
                    new.status.state,
                    Some(GameServerState::Allocated)
                );
            }
            event => panic!("expected GameServer Modified event, got {event:?}"),
        }

        let node = Node::new("node1");
        let node_key = build_key("nodes", None, "node1");
        storage.create(&node_key, &node).await.unwrap();
        storage.delete(&node_key).await.unwrap();

        match next_counter_event(&mut events).await {
            PerNodeCounterEvent::NodeDeleted(node_name) => {
                assert_eq!(node_name, "node1");
            }
            event => panic!("expected NodeDeleted event, got {event:?}"),
        }

        repository
            .delete::<GameServer>("default", "server")
            .await
            .unwrap();
        match next_counter_event(&mut events).await {
            PerNodeCounterEvent::Deleted(game_server) => {
                assert_eq!(game_server.metadata.uid, updated.metadata.uid);
            }
            event => panic!("expected GameServer Deleted event, got {event:?}"),
        }
    }

    async fn wait_for_node_count(
        controller: &PerNodeCounterController<MemoryStorage>,
        node_name: &str,
        expected: Option<NodeCount>,
    ) {
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            loop {
                let current = controller
                    .counts()
                    .unwrap()
                    .get(node_name)
                    .copied();
                if current == expected {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("PerNodeCounter runner did not converge");
    }

    #[tokio::test]
    async fn cgp_runner_rebuilds_then_tracks_live_rusternetes_events() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());

        let ready = game_server(
            "server",
            "uid-server",
            "1",
            GameServerState::Ready,
            "node1",
        );
        let created = repository.create(&ready).await.unwrap();

        let controller = Arc::new(PerNodeCounterController::new(storage.clone()));
        let runner = controller.clone();
        let task = tokio::spawn(async move {
            runner.run_per_node_counter().await
        });

        wait_for_node_count(
            controller.as_ref(),
            "node1",
            Some(NodeCount {
                ready: 1,
                allocated: 0,
            }),
        )
        .await;

        let mut allocated = created;
        allocated.metadata.resource_version = Some("2".into());
        allocated.status.state = Some(GameServerState::Allocated);
        repository.update(&allocated).await.unwrap();

        wait_for_node_count(
            controller.as_ref(),
            "node1",
            Some(NodeCount {
                ready: 0,
                allocated: 1,
            }),
        )
        .await;

        let node = Node::new("node1");
        let node_key = build_key("nodes", None, "node1");
        storage.create(&node_key, &node).await.unwrap();
        storage.delete(&node_key).await.unwrap();

        wait_for_node_count(controller.as_ref(), "node1", None).await;
        task.abort();
    }
}
