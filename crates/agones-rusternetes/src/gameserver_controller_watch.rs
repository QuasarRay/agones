use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use agones_core::{GameServer, GameServerState};
use cgp::prelude::*;
use futures::{StreamExt, stream::BoxStream};
use rusternetes_common::resources::Pod;
use rusternetes_storage::{Storage, WatchEvent, build_prefix};

use crate::{
    AdapterError, AgonesRepository, AgonesWatchEvent, is_game_server_pod,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameServerQueueLane {
    Main,
    Creation,
    Deletion,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameServerWorkItem {
    pub namespace: String,
    pub name: String,
    pub lane: GameServerQueueLane,
}

impl GameServerWorkItem {
    pub fn from_game_server(game_server: &GameServer) -> Option<Self> {
        if game_server.metadata.namespace.is_empty()
            || game_server.metadata.name.is_empty()
        {
            return None;
        }

        Some(Self {
            namespace: game_server.metadata.namespace.clone(),
            name: game_server.metadata.name.clone(),
            lane: queue_lane(game_server.status.state.as_ref()),
        })
    }

    pub fn key(&self) -> String {
        format!("{}/{}", self.namespace, self.name)
    }
}

pub fn queue_lane(state: Option<&GameServerState>) -> GameServerQueueLane {
    match state {
        Some(GameServerState::PortAllocation | GameServerState::Creating) => {
            GameServerQueueLane::Creation
        }
        Some(GameServerState::Shutdown) => GameServerQueueLane::Deletion,
        _ => GameServerQueueLane::Main,
    }
}

#[derive(Clone, Debug)]
pub enum GameServerControllerWatchEvent {
    GameServerAdded(GameServer),
    GameServerModified(GameServer),
    GameServerDeleted(GameServer),
    PodAdded(Pod),
    PodModified(Pod),
    PodDeleted(Pod),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ObservedGameServer {
    state: Option<GameServerState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ObservedPod {
    is_game_server_pod: bool,
    node_name: Option<String>,
    pod_ips: Vec<String>,
}

impl ObservedPod {
    fn from_pod(pod: &Pod) -> Self {
        Self {
            is_game_server_pod: is_game_server_pod(pod),
            node_name: pod
                .spec
                .as_ref()
                .and_then(|spec| spec.node_name.clone()),
            pod_ips: pod
                .status
                .as_ref()
                .and_then(|status| status.pod_i_ps.as_ref())
                .map(|ips| ips.iter().map(|ip| ip.ip.clone()).collect())
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct GameServerControllerWatchState {
    game_servers: HashMap<String, ObservedGameServer>,
    pods: HashMap<String, ObservedPod>,
}

impl GameServerControllerWatchState {
    pub fn rebuild<I, P>(
        &mut self,
        game_servers: I,
        pods: P,
    ) -> Vec<GameServerWorkItem>
    where
        I: IntoIterator<Item = GameServer>,
        P: IntoIterator<Item = Pod>,
    {
        self.game_servers.clear();
        self.pods.clear();

        let mut initial = Vec::new();
        for game_server in game_servers {
            self.observe_game_server(&game_server);
            if let Some(item) = GameServerWorkItem::from_game_server(&game_server) {
                // Informers invoke AddFunc for objects delivered by their initial
                // LIST, so every existing GameServer is initially enqueued.
                initial.push(item);
            }
        }

        for pod in pods {
            self.observe_pod(&pod);
        }

        initial
    }

    pub fn route(
        &mut self,
        event: GameServerControllerWatchEvent,
    ) -> Option<GameServerWorkItem> {
        match event {
            GameServerControllerWatchEvent::GameServerAdded(game_server) => {
                self.observe_game_server(&game_server);
                GameServerWorkItem::from_game_server(&game_server)
            }
            GameServerControllerWatchEvent::GameServerModified(game_server) => {
                let identity = game_server_identity(&game_server);
                let previous = self.game_servers.get(&identity).cloned();
                self.observe_game_server(&game_server);

                let state_changed = previous
                    .as_ref()
                    .is_none_or(|old| old.state != game_server.status.state);
                if state_changed || game_server.metadata.deletion_timestamp.is_some() {
                    GameServerWorkItem::from_game_server(&game_server)
                } else {
                    None
                }
            }
            GameServerControllerWatchEvent::GameServerDeleted(game_server) => {
                self.game_servers.remove(&game_server_identity(&game_server));
                // The upstream main GameServer informer has no DeleteFunc.
                None
            }
            GameServerControllerWatchEvent::PodAdded(pod) => {
                self.observe_pod(&pod);
                None
            }
            GameServerControllerWatchEvent::PodModified(pod) => {
                let identity = pod_identity(&pod);
                let current = ObservedPod::from_pod(&pod);
                let previous = self.pods.insert(identity, current.clone());

                let should_enqueue = previous.as_ref().is_some_and(|old| {
                    old.is_game_server_pod
                        && (old.node_name != current.node_name
                            || old.pod_ips != current.pod_ips)
                });

                should_enqueue
                    .then(|| work_item_for_game_server_pod(&pod))
                    .flatten()
            }
            GameServerControllerWatchEvent::PodDeleted(pod) => {
                self.pods.remove(&pod_identity(&pod));
                if is_game_server_pod(&pod) {
                    work_item_for_game_server_pod(&pod)
                } else {
                    None
                }
            }
        }
    }

    fn observe_game_server(&mut self, game_server: &GameServer) {
        self.game_servers.insert(
            game_server_identity(game_server),
            ObservedGameServer {
                state: game_server.status.state.clone(),
            },
        );
    }

    fn observe_pod(&mut self, pod: &Pod) {
        self.pods
            .insert(pod_identity(pod), ObservedPod::from_pod(pod));
    }
}

fn game_server_identity(game_server: &GameServer) -> String {
    if !game_server.metadata.uid.is_empty() {
        return format!("uid:{}", game_server.metadata.uid);
    }
    format!(
        "name:{}/{}",
        game_server.metadata.namespace, game_server.metadata.name
    )
}

fn pod_identity(pod: &Pod) -> String {
    if !pod.metadata.uid.is_empty() {
        return format!("uid:{}", pod.metadata.uid);
    }
    format!(
        "name:{}/{}",
        pod.metadata.namespace.as_deref().unwrap_or_default(),
        pod.metadata.name
    )
}

fn work_item_for_game_server_pod(pod: &Pod) -> Option<GameServerWorkItem> {
    if !is_game_server_pod(pod) {
        return None;
    }

    let namespace = pod
        .metadata
        .namespace
        .as_deref()
        .filter(|namespace| !namespace.is_empty())?;
    let owner = pod
        .metadata
        .owner_references
        .as_ref()?
        .iter()
        .find(|owner| owner.controller == Some(true) && owner.kind == "GameServer")?;

    if owner.name.is_empty() {
        return None;
    }

    Some(GameServerWorkItem {
        namespace: namespace.to_owned(),
        name: owner.name.clone(),
        // Pod scheduling/IP changes and deletions always feed the general
        // worker queue in the upstream controller.
        lane: GameServerQueueLane::Main,
    })
}

pub type GameServerControllerWatchStream =
    BoxStream<'static, Result<GameServerControllerWatchEvent, AdapterError>>;

/// Builds the two live watch sources used by the upstream main GameServer
/// controller: cluster-wide GameServers and Pods.
pub async fn watch_game_server_controller_events<S>(
    storage: Arc<S>,
) -> Result<GameServerControllerWatchStream, AdapterError>
where
    S: Storage + 'static,
{
    let repository = AgonesRepository::new(storage.clone());
    let game_servers = repository.watch_all::<GameServer>().await?;
    let pods = storage.watch(&build_prefix("pods", None)).await?;

    let game_server_events = game_servers.map(|event| {
        event.map(|event| match event {
            AgonesWatchEvent::Added { resource, .. } => {
                GameServerControllerWatchEvent::GameServerAdded(resource)
            }
            AgonesWatchEvent::Modified { resource, .. } => {
                GameServerControllerWatchEvent::GameServerModified(resource)
            }
            AgonesWatchEvent::Deleted { resource, .. } => {
                GameServerControllerWatchEvent::GameServerDeleted(resource)
            }
        })
    });

    let pod_events = pods.map(|event| match event {
        Ok(WatchEvent::Added(_, raw)) => serde_json::from_str::<Pod>(&raw)
            .map(GameServerControllerWatchEvent::PodAdded)
            .map_err(AdapterError::from),
        Ok(WatchEvent::Modified(_, raw)) => serde_json::from_str::<Pod>(&raw)
            .map(GameServerControllerWatchEvent::PodModified)
            .map_err(AdapterError::from),
        Ok(WatchEvent::Deleted(_, raw)) => serde_json::from_str::<Pod>(&raw)
            .map(GameServerControllerWatchEvent::PodDeleted)
            .map_err(AdapterError::from),
        Err(error) => Err(AdapterError::from(error)),
    });

    Ok(Box::pin(futures::stream::select(
        game_server_events,
        pod_events,
    )))
}

/// Seeds the informer-equivalent observation cache from Rusternetes.
///
/// Every GameServer in the initial LIST is returned as work, matching the
/// AddFunc calls generated by a Kubernetes shared informer during startup.
#[cgp_fn]
#[async_trait]
pub async fn rebuild_game_server_controller_watch_state<S>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] game_server_watch_state: Arc<RwLock<GameServerControllerWatchState>>,
) -> Result<Vec<GameServerWorkItem>, AdapterError>
where
    S: Storage + 'static,
{
    let repository = AgonesRepository::new(storage.clone());
    let game_servers = repository.list_all::<GameServer>().await?;
    let pods: Vec<Pod> = storage.list(&build_prefix("pods", None)).await?;

    let mut state = game_server_watch_state
        .write()
        .map_err(|_| AdapterError::Hook("GameServer watch state lock poisoned".into()))?;
    Ok(state.rebuild(game_servers, pods))
}

/// CGP reducer for the upstream GameServer/Pod informer enqueue rules.
#[cgp_fn]
#[async_trait]
pub async fn route_game_server_controller_event(
    &self,
    #[implicit] game_server_watch_state: Arc<RwLock<GameServerControllerWatchState>>,
    event: GameServerControllerWatchEvent,
) -> Result<Option<GameServerWorkItem>, AdapterError> {
    let mut state = game_server_watch_state
        .write()
        .map_err(|_| AdapterError::Hook("GameServer watch state lock poisoned".into()))?;
    Ok(state.route(event))
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerSpec, GameServerStatus, ObjectMeta,
        PodSpec as CorePodSpec, PodTemplateSpec,
    };
    use futures::StreamExt;
    use rusternetes_common::resources::{PodIP, PodStatus};
    use rusternetes_storage::{MemoryStorage, Storage, build_key};

    use crate::{
        AgonesRepository, CreatingGameServerController, NoopApiHooks,
        PodBuildFeatures, build_pod,
    };

    use super::*;

    fn game_server(state: Option<GameServerState>) -> GameServer {
        GameServer {
            metadata: ObjectMeta {
                name: "test".into(),
                namespace: "default".into(),
                uid: "gs-uid".into(),
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
            status: GameServerStatus {
                state,
                ..Default::default()
            },
        }
    }

    fn game_server_pod(game_server: &GameServer) -> Pod {
        build_pod(
            game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap()
    }

    #[test]
    fn queue_lanes_match_upstream_state_routing() {
        assert_eq!(
            queue_lane(Some(&GameServerState::PortAllocation)),
            GameServerQueueLane::Creation
        );
        assert_eq!(
            queue_lane(Some(&GameServerState::Creating)),
            GameServerQueueLane::Creation
        );
        assert_eq!(
            queue_lane(Some(&GameServerState::Shutdown)),
            GameServerQueueLane::Deletion
        );
        assert_eq!(
            queue_lane(Some(&GameServerState::Starting)),
            GameServerQueueLane::Main
        );
        assert_eq!(queue_lane(None), GameServerQueueLane::Main);
    }

    #[test]
    fn routing_matches_upstream_gameserver_watch_behavior() {
        let mut state = GameServerControllerWatchState::default();
        let fixture = game_server(None);

        let first = state
            .route(GameServerControllerWatchEvent::GameServerAdded(
                fixture.clone(),
            ))
            .unwrap();
        assert_eq!(first.key(), "default/test");
        assert_eq!(first.lane, GameServerQueueLane::Main);

        let pod = game_server_pod(&fixture);
        assert!(
            state
                .route(GameServerControllerWatchEvent::PodAdded(pod.clone()))
                .is_none()
        );

        assert!(
            state
                .route(GameServerControllerWatchEvent::GameServerModified(
                    fixture.clone(),
                ))
                .is_none()
        );

        let mut non_game_pod = Pod::new("foo", Default::default());
        non_game_pod.metadata.namespace = Some("default".into());
        assert!(
            state
                .route(GameServerControllerWatchEvent::PodAdded(non_game_pod))
                .is_none()
        );

        let mut starting = fixture.clone();
        starting.status.state = Some(GameServerState::Starting);
        let item = state
            .route(GameServerControllerWatchEvent::GameServerModified(
                starting.clone(),
            ))
            .unwrap();
        assert_eq!(item.key(), "default/test");
        assert_eq!(item.lane, GameServerQueueLane::Main);

        let mut deleting = starting;
        deleting.metadata.deletion_timestamp =
            Some("2026-10-17T00:00:00Z".into());
        assert!(
            state
                .route(GameServerControllerWatchEvent::GameServerModified(
                    deleting,
                ))
                .is_some()
        );

        let item = state
            .route(GameServerControllerWatchEvent::PodDeleted(pod.clone()))
            .unwrap();
        assert_eq!(item.key(), "default/test");
        assert_eq!(item.lane, GameServerQueueLane::Main);

        let unscheduled = game_server_pod(&fixture);
        assert!(
            state
                .route(GameServerControllerWatchEvent::PodAdded(
                    unscheduled.clone(),
                ))
                .is_none()
        );

        let mut scheduled = unscheduled;
        scheduled.spec.as_mut().unwrap().node_name = Some("node1".into());
        let item = state
            .route(GameServerControllerWatchEvent::PodModified(scheduled))
            .unwrap();
        assert_eq!(item.key(), "default/test");
    }

    #[test]
    fn pod_ip_changes_enqueue_and_identical_updates_do_not() {
        let mut state = GameServerControllerWatchState::default();
        let fixture = game_server(Some(GameServerState::Ready));
        let mut pod = game_server_pod(&fixture);
        pod.status = Some(PodStatus {
            pod_i_ps: Some(vec![PodIP {
                ip: "10.0.0.1".into(),
            }]),
            ..Default::default()
        });

        assert!(
            state
                .route(GameServerControllerWatchEvent::PodAdded(pod.clone()))
                .is_none()
        );
        assert!(
            state
                .route(GameServerControllerWatchEvent::PodModified(pod.clone()))
                .is_none()
        );

        pod.status.as_mut().unwrap().pod_i_ps = Some(vec![
            PodIP {
                ip: "10.0.0.1".into(),
            },
            PodIP {
                ip: "2001:db8::1".into(),
            },
        ]);
        assert!(
            state
                .route(GameServerControllerWatchEvent::PodModified(pod))
                .is_some()
        );
    }

    #[tokio::test]
    async fn rebuild_matches_informer_initial_add_semantics() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());

        let mut creating = game_server(Some(GameServerState::Creating));
        creating.metadata.name = "creating".into();
        creating.metadata.uid = "creating-uid".into();
        repository.create(&creating).await.unwrap();

        let mut ready = game_server(Some(GameServerState::Ready));
        ready.metadata.name = "ready".into();
        ready.metadata.uid = "ready-uid".into();
        repository.create(&ready).await.unwrap();

        let ready_pod = game_server_pod(&ready);
        storage
            .create(
                &build_key("pods", Some("default"), "ready"),
                &ready_pod,
            )
            .await
            .unwrap();

        let controller = CreatingGameServerController::new(storage);
        let mut items = controller
            .rebuild_game_server_controller_watch_state()
            .await
            .unwrap();
        items.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "creating");
        assert_eq!(items[0].lane, GameServerQueueLane::Creation);
        assert_eq!(items[1].name, "ready");
        assert_eq!(items[1].lane, GameServerQueueLane::Main);
    }

    async fn next_event(
        stream: &mut GameServerControllerWatchStream,
    ) -> GameServerControllerWatchEvent {
        tokio::time::timeout(std::time::Duration::from_secs(2), stream.next())
            .await
            .expect("GameServer controller watch timed out")
            .expect("GameServer controller watch ended")
            .expect("GameServer controller watch failed")
    }

    #[tokio::test]
    async fn live_watch_stream_decodes_gameserver_and_pod_events() {
        let storage = Arc::new(MemoryStorage::new());
        let repository = AgonesRepository::new(storage.clone());
        let mut stream = watch_game_server_controller_events(storage.clone())
            .await
            .unwrap();

        let fixture = game_server(Some(GameServerState::Creating));
        repository.create(&fixture).await.unwrap();
        assert!(matches!(
            next_event(&mut stream).await,
            GameServerControllerWatchEvent::GameServerAdded(_)
        ));

        let pod = game_server_pod(&fixture);
        storage
            .create(
                &build_key("pods", Some("default"), "test"),
                &pod,
            )
            .await
            .unwrap();
        assert!(matches!(
            next_event(&mut stream).await,
            GameServerControllerWatchEvent::PodAdded(_)
        ));

        let mut updated_pod = pod.clone();
        updated_pod.spec.as_mut().unwrap().node_name = Some("node1".into());
        storage
            .update(
                &build_key("pods", Some("default"), "test"),
                &updated_pod,
            )
            .await
            .unwrap();
        assert!(matches!(
            next_event(&mut stream).await,
            GameServerControllerWatchEvent::PodModified(_)
        ));

        storage
            .delete(&build_key("pods", Some("default"), "test"))
            .await
            .unwrap();
        assert!(matches!(
            next_event(&mut stream).await,
            GameServerControllerWatchEvent::PodDeleted(_)
        ));
    }
}
