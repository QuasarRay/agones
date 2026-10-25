use std::{
    cmp::Ordering,
    collections::HashMap,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering as AtomicOrdering},
    },
};

use agones_core::{
    GameServer, GameServerAllocation, GameServerState, NodeCount,
};
use futures::StreamExt;
use rusternetes_storage::Storage;

use crate::{
    AdapterError, AgonesRepository, AgonesWatchEvent, PerNodeCounterState,
    cache::GameServerCache,
    controller_work_queue::{
        ControllerWorkItem, ControllerWorkQueue, DefaultControllerRetryPolicy,
    },
};

type Matcher = fn(&GameServer) -> bool;

pub fn ready_or_allocated(gs: &GameServer) -> bool {
    matches!(
        gs.status.state,
        Some(GameServerState::Ready | GameServerState::Allocated)
    )
}

fn key(gs: &GameServer) -> String {
    format!("{}/{}", gs.metadata.namespace, gs.metadata.name)
}

fn priority_cmp(
    a: &GameServer,
    b: &GameServer,
    gsa: Option<&GameServerAllocation>,
) -> Option<Ordering> {
    gsa.and_then(|gsa| {
        a.compare_count_and_list_priorities(&gsa.spec.priorities, b)
            .map(|before| if before { Ordering::Less } else { Ordering::Greater })
    })
}

fn packed_cmp(
    a: &GameServer,
    b: &GameServer,
    gsa: Option<&GameServerAllocation>,
    counts: &HashMap<String, NodeCount>,
) -> Ordering {
    if a.status.state != b.status.state {
        return match (
            a.status.state == Some(GameServerState::Allocated),
            b.status.state == Some(GameServerState::Allocated),
        ) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => Ordering::Equal,
        };
    }

    match (
        counts.get(&a.status.node_name),
        counts.get(&b.status.node_name),
    ) {
        (None, Some(_)) => return Ordering::Greater,
        (Some(_), None) => return Ordering::Less,
        (None, None) => return Ordering::Equal,
        (Some(ca), Some(cb)) => {
            let order = cb
                .allocated
                .cmp(&ca.allocated)
                .then_with(|| cb.ready.cmp(&ca.ready));
            if order != Ordering::Equal {
                return order;
            }
        }
    }

    priority_cmp(a, b, gsa)
        .unwrap_or_else(|| a.metadata.name.cmp(&b.metadata.name))
}

#[derive(Clone)]
struct Resync;

impl ControllerWorkItem for Resync {
    fn work_key(&self) -> String {
        String::new()
    }
}

type ResyncQueue =
    ControllerWorkQueue<Resync, DefaultControllerRetryPolicy>;

struct Healthy<'a>(&'a AtomicBool);

impl Drop for Healthy<'_> {
    fn drop(&mut self) {
        self.0.store(false, AtomicOrdering::Release);
    }
}

pub struct AllocationCache<S> {
    storage: Arc<S>,
    cache: GameServerCache,
    counter: Arc<RwLock<PerNodeCounterState>>,
    matcher: Matcher,
    resync: ResyncQueue,
    healthy: AtomicBool,
}

impl<S> AllocationCache<S> {
    pub fn new(
        storage: S,
        counter: Arc<RwLock<PerNodeCounterState>>,
    ) -> Self {
        Self {
            storage: Arc::new(storage),
            cache: GameServerCache::default(),
            counter,
            matcher: ready_or_allocated,
            resync: ResyncQueue::new(DefaultControllerRetryPolicy::default()),
            healthy: AtomicBool::new(false),
        }
    }

    pub fn healthy(&self) -> bool {
        self.healthy.load(AtomicOrdering::Acquire)
    }

    pub fn remove_game_server(&self, gs: &GameServer) -> Result<(), AdapterError> {
        self.cache
            .delete(&key(gs))
            .then_some(())
            .ok_or_else(|| {
                AdapterError::Hook(
                    "The Gameserver was already allocated".into(),
                )
            })
    }

    pub fn add_game_server(&self, gs: &GameServer) {
        self.cache.store(key(gs), gs);
    }

    fn game_servers(&self) -> Vec<Arc<GameServer>> {
        self.cache.snapshot()
    }

    pub fn list_sorted_game_servers(
        &self,
        gsa: Option<&GameServerAllocation>,
    ) -> Result<Vec<Arc<GameServer>>, AdapterError> {
        let counts = self
            .counter
            .read()
            .map_err(|_| AdapterError::Hook("PerNodeCounter lock poisoned".into()))?
            .counts();
        let mut list = self.game_servers();
        list.sort_unstable_by(|a, b| packed_cmp(a, b, gsa, &counts));
        Ok(list)
    }

    pub fn list_sorted_game_servers_priorities(
        &self,
        gsa: Option<&GameServerAllocation>,
    ) -> Vec<Arc<GameServer>> {
        let mut list = self.game_servers();
        list.sort_unstable_by(|a, b| {
            priority_cmp(a, b, gsa)
                .unwrap_or_else(|| a.metadata.name.cmp(&b.metadata.name))
        });
        list
    }

    pub fn resync(&self) {
        self.resync.enqueue(Resync);
    }

    fn apply_event(&self, event: AgonesWatchEvent<GameServer>) {
        match event {
            AgonesWatchEvent::Added { .. } => {}
            AgonesWatchEvent::Modified { resource: gs, .. } => {
                let k = key(&gs);
                if gs.is_being_deleted() || !(self.matcher)(&gs) {
                    self.cache.delete(&k);
                } else {
                    self.cache.store(k, &gs);
                }
            }
            AgonesWatchEvent::Deleted { resource: gs, .. } => {
                self.cache.delete(&key(&gs));
            }
        }
    }
}

impl<S: Storage + 'static> AllocationCache<S> {
    async fn sync_cache(&self) -> Result<(), AdapterError> {
        let current = AgonesRepository::new(self.storage.clone())
            .list_all::<GameServer>()
            .await?
            .into_iter()
            .map(|gs| (key(&gs), gs))
            .collect::<HashMap<_, _>>();

        let mut stale = Vec::new();
        self.cache.range(|k, _| {
            if !current.contains_key(k) {
                stale.push(k.to_owned());
            }
            true
        });
        for k in stale {
            self.cache.delete(&k);
        }

        for (k, gs) in current {
            if gs.is_being_deleted() || !(self.matcher)(&gs) {
                self.cache.delete(&k);
            } else {
                self.cache.store(k, &gs);
            }
        }
        Ok(())
    }

    pub async fn sync(&self) -> Result<(), AdapterError> {
        self.sync_cache().await
    }

    pub async fn sync_game_servers(
        &self,
        _key: &str,
    ) -> Result<(), AdapterError> {
        self.sync_cache().await
    }

    async fn run_resync_worker(&self) {
        while let Some(item) = self.resync.next().await {
            match self.sync_game_servers("").await {
                Ok(()) => self.resync.complete_success(&item),
                Err(_) => {
                    let delay = self.resync.record_failure(&item);
                    tokio::time::sleep(delay).await;
                    self.resync.complete_retry(&item);
                }
            }
        }
    }

    pub async fn run(&self) -> Result<(), AdapterError> {
        let mut events = AgonesRepository::new(self.storage.clone())
            .watch_all::<GameServer>()
            .await?;

        self.sync().await?;
        self.healthy.store(true, AtomicOrdering::Release);
        let _healthy = Healthy(&self.healthy);
        let worker = self.run_resync_worker();
        tokio::pin!(worker);

        loop {
            tokio::select! {
                event = events.next() => match event {
                    Some(Ok(event)) => self.apply_event(event),
                    Some(Err(error)) => return Err(error),
                    None => return Ok(()),
                },
                _ = &mut worker => {
                    return Err(AdapterError::Hook(
                        "AllocationCache worker terminated unexpectedly".into(),
                    ));
                }
            }
        }
    }
}
