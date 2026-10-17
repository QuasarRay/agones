use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, Ordering};

use agones_core::{GameServer, PortPolicy, Protocol};
use futures::lock::Mutex;
use rusternetes_common::resources::{CustomResource, Node};
use rusternetes_storage::{build_prefix, Storage};

use crate::{
    rusternetes_custom_resource_type, AdapterError, RusternetesCustomResource,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PortRange {
    pub min_port: i32,
    pub max_port: i32,
}

#[derive(Debug)]
struct PortRangeAllocator {
    name: String,
    min_port: i32,
    max_port: i32,
    port_allocations: Vec<BTreeMap<i32, bool>>,
    game_server_registry: BTreeSet<String>,
}

impl PortRangeAllocator {
    fn new(name: String, range: PortRange) -> Result<Self, AdapterError> {
        if range.min_port <= 0
            || range.max_port > i32::from(u16::MAX)
            || range.min_port > range.max_port
        {
            return Err(AdapterError::Hook(format!(
                "invalid port range {name}: {}-{}",
                range.min_port, range.max_port
            )));
        }

        Ok(Self {
            name,
            min_port: range.min_port,
            max_port: range.max_port,
            port_allocations: Vec::new(),
            game_server_registry: BTreeSet::new(),
        })
    }

    fn new_port_allocation(&self) -> BTreeMap<i32, bool> {
        (self.min_port..=self.max_port)
            .map(|port| (port, false))
            .collect()
    }

    fn set_port_allocation(
        port: i32,
        allocations: &mut [BTreeMap<i32, bool>],
        taken: bool,
    ) {
        for allocation in allocations {
            if allocation.get(&port).is_some_and(|value| *value != taken) {
                allocation.insert(port, taken);
                break;
            }
        }
    }

    fn sync_all(&mut self, game_servers: &[GameServer], nodes: &[Node]) {
        let mut node_allocations: BTreeMap<String, BTreeMap<i32, bool>> = nodes
            .iter()
            .filter(|node| {
                !node
                    .spec
                    .as_ref()
                    .and_then(|spec| spec.unschedulable)
                    .unwrap_or(false)
            })
            .map(|node| (node.metadata.name.clone(), self.new_port_allocation()))
            .collect();

        let mut node_counts: BTreeMap<String, usize> = node_allocations
            .keys()
            .map(|name| (name.clone(), 0))
            .collect();
        let mut non_ready_ports = Vec::new();
        let mut registry = BTreeSet::new();

        for game_server in game_servers {
            for port in &game_server.spec.ports {
                if !matches!(
                    port.port_policy.as_ref(),
                    Some(PortPolicy::Dynamic | PortPolicy::Passthrough)
                ) {
                    continue;
                }
                if port.host_port < self.min_port || port.host_port > self.max_port {
                    continue;
                }

                registry.insert(game_server.metadata.uid.clone());

                if !game_server.status.node_name.is_empty() {
                    if let Some(allocation) =
                        node_allocations.get_mut(&game_server.status.node_name)
                    {
                        allocation.insert(port.host_port, true);
                        *node_counts
                            .entry(game_server.status.node_name.clone())
                            .or_default() += 1;
                        continue;
                    }
                }

                if port.host_port != 0 {
                    non_ready_ports.push(port.host_port);
                }
            }
        }

        let mut ordered_nodes: Vec<_> = node_allocations.into_iter().collect();
        ordered_nodes.sort_by(|(left_name, _), (right_name, _)| {
            node_counts
                .get(right_name)
                .copied()
                .unwrap_or_default()
                .cmp(&node_counts.get(left_name).copied().unwrap_or_default())
                .then_with(|| left_name.cmp(right_name))
        });

        let mut allocations: Vec<_> =
            ordered_nodes.into_iter().map(|(_, allocation)| allocation).collect();

        for port in non_ready_ports {
            Self::set_port_allocation(port, &mut allocations, true);
        }

        self.port_allocations = allocations;
        self.game_server_registry = registry;
    }

    fn find_open_ports(&self, amount: usize) -> Vec<(usize, i32)> {
        if amount == 0 {
            return Vec::new();
        }

        let mut ports = Vec::with_capacity(amount);
        for (node_index, allocation) in self.port_allocations.iter().enumerate() {
            for (&port, &taken) in allocation {
                if !taken {
                    ports.push((node_index, port));
                    if ports.len() == amount {
                        return ports;
                    }
                }
            }
        }
        ports
    }

    fn allocate(&mut self, game_server: &mut GameServer) {
        let amount = game_server.count_ports_for_range(&self.name, |policy| {
            matches!(policy, PortPolicy::Dynamic | PortPolicy::Passthrough)
        });

        let allocations = loop {
            let allocations = self.find_open_ports(amount);
            if allocations.len() == amount {
                break allocations;
            }
            self.port_allocations.push(self.new_port_allocation());
        };

        self.game_server_registry
            .insert(game_server.metadata.uid.clone());

        let mut allocations = allocations.into_iter();
        let mut extra_ports = Vec::new();

        for port in &mut game_server.spec.ports {
            if !matches!(
                port.port_policy.as_ref(),
                Some(PortPolicy::Dynamic | PortPolicy::Passthrough)
            ) || port.range != self.name
            {
                continue;
            }

            let (node_index, allocated_port) = allocations
                .next()
                .expect("allocation count must match matching port count");
            self.port_allocations[node_index].insert(allocated_port, true);

            let original = port.clone();
            port.host_port = allocated_port;
            if port.port_policy.as_ref() == Some(&PortPolicy::Passthrough) {
                port.container_port = allocated_port;
            }

            if original.protocol.as_ref() == Some(&Protocol::TcpUdp) {
                let mut duplicate = original;
                duplicate.host_port = allocated_port;
                if duplicate.port_policy.as_ref() == Some(&PortPolicy::Passthrough) {
                    duplicate.container_port = allocated_port;
                }

                let base_name = port.name.clone();
                port.name = format!("{base_name}-tcp");
                port.protocol = Some(Protocol::Tcp);

                duplicate.name = format!("{base_name}-udp");
                duplicate.protocol = Some(Protocol::Udp);
                extra_ports.push(duplicate);
            }
        }

        game_server.spec.ports.extend(extra_ports);
    }

    fn deallocate(&mut self, game_server: &GameServer) {
        if !self
            .game_server_registry
            .contains(&game_server.metadata.uid)
        {
            return;
        }

        for port in &game_server.spec.ports {
            if port.host_port < self.min_port || port.host_port > self.max_port {
                continue;
            }
            Self::set_port_allocation(
                port.host_port,
                &mut self.port_allocations,
                false,
            );
        }

        self.game_server_registry
            .remove(&game_server.metadata.uid);
    }
}

/// Rusternetes-backed equivalent of Agones' dynamic PortAllocator.
///
/// ensure_synced reconstructs the allocator once from all schedulable Nodes
/// and all persisted GameServers. Allocation and deallocation are then
/// serialized through a runtime-independent async mutex, matching the upstream
/// allocator's single critical section while avoiding a Tokio dependency.
pub struct PortAllocator {
    ranges: Mutex<Vec<PortRangeAllocator>>,
    sync_lock: Mutex<()>,
    synced: AtomicBool,
}

impl Default for PortAllocator {
    fn default() -> Self {
        Self::empty()
    }
}

impl PortAllocator {
    pub fn empty() -> Self {
        Self {
            ranges: Mutex::new(Vec::new()),
            sync_lock: Mutex::new(()),
            synced: AtomicBool::new(false),
        }
    }

    pub fn new(
        ranges: BTreeMap<String, PortRange>,
    ) -> Result<Self, AdapterError> {
        let ranges = ranges
            .into_iter()
            .map(|(name, range)| PortRangeAllocator::new(name, range))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            ranges: Mutex::new(ranges),
            sync_lock: Mutex::new(()),
            synced: AtomicBool::new(false),
        })
    }

    pub async fn ensure_synced<S: Storage>(
        &self,
        storage: &S,
    ) -> Result<(), AdapterError> {
        if self.synced.load(Ordering::Acquire) {
            return Ok(());
        }

        let _sync_guard = self.sync_lock.lock().await;
        if self.synced.load(Ordering::Acquire) {
            return Ok(());
        }

        self.sync_from_storage_locked(storage).await?;
        self.synced.store(true, Ordering::Release);
        Ok(())
    }

    pub async fn resync<S: Storage>(
        &self,
        storage: &S,
    ) -> Result<(), AdapterError> {
        let _sync_guard = self.sync_lock.lock().await;
        self.sync_from_storage_locked(storage).await?;
        self.synced.store(true, Ordering::Release);
        Ok(())
    }

    async fn sync_from_storage_locked<S: Storage>(
        &self,
        storage: &S,
    ) -> Result<(), AdapterError> {
        let nodes: Vec<Node> = storage
            .list(&build_prefix("nodes", None))
            .await?;
        let resources: Vec<CustomResource> = storage
            .list(&build_prefix(
                &rusternetes_custom_resource_type::<GameServer>(),
                None,
            ))
            .await?;
        let game_servers = resources
            .into_iter()
            .map(GameServer::from_rusternetes_custom_resource)
            .collect::<Result<Vec<_>, _>>()?;

        let mut ranges = self.ranges.lock().await;
        for range in ranges.iter_mut() {
            range.sync_all(&game_servers, &nodes);
        }
        Ok(())
    }

    pub async fn sync_from_snapshot(
        &self,
        game_servers: &[GameServer],
        nodes: &[Node],
    ) {
        let mut ranges = self.ranges.lock().await;
        for range in ranges.iter_mut() {
            range.sync_all(game_servers, nodes);
        }
        self.synced.store(true, Ordering::Release);
    }

    pub async fn allocate(
        &self,
        game_server: &GameServer,
    ) -> Result<GameServer, AdapterError> {
        let mut allocated = game_server.clone();
        let mut ranges = self.ranges.lock().await;

        for range in ranges.iter_mut() {
            range.allocate(&mut allocated);
        }
        Ok(allocated)
    }

    pub async fn deallocate(&self, game_server: &GameServer) {
        let mut ranges = self.ranges.lock().await;
        for range in ranges.iter_mut() {
            range.deallocate(game_server);
        }
    }
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container, GameServerPort, GameServerSpec, GameServerStatus, ObjectMeta,
        PodSpec, PodTemplateSpec,
    };
    use rusternetes_common::resources::{Node, NodeSpec};

    use super::*;

    fn node(name: &str, unschedulable: bool) -> Node {
        let mut node = Node::new(name);
        node.spec = Some(NodeSpec {
            pod_cidr: None,
            pod_cidrs: None,
            provider_id: None,
            unschedulable: Some(unschedulable),
            taints: None,
        });
        node
    }

    fn game_server(
        uid: &str,
        ports: Vec<GameServerPort>,
        node_name: &str,
    ) -> GameServer {
        GameServer {
            metadata: ObjectMeta {
                name: uid.into(),
                namespace: "default".into(),
                uid: uid.into(),
                ..Default::default()
            },
            spec: GameServerSpec {
                container: "game".into(),
                ports,
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
            status: GameServerStatus {
                node_name: node_name.into(),
                ..Default::default()
            },
        }
    }

    fn port(
        name: &str,
        range: &str,
        policy: PortPolicy,
        protocol: Protocol,
    ) -> GameServerPort {
        GameServerPort {
            name: name.into(),
            range: range.into(),
            port_policy: Some(policy),
            container: Some("game".into()),
            container_port: 7777,
            host_port: 0,
            protocol: Some(protocol),
        }
    }

    fn allocator() -> PortAllocator {
        PortAllocator::new(BTreeMap::from([(
            "default".into(),
            PortRange {
                min_port: 10,
                max_port: 12,
            },
        )]))
        .unwrap()
    }

    #[tokio::test]
    async fn allocates_dynamic_and_passthrough_tcpudp_ports() {
        let allocator = allocator();
        allocator
            .sync_from_snapshot(&[], &[node("node-a", false)])
            .await;

        let source = game_server(
            "gs-1",
            vec![
                port("udp", "default", PortPolicy::Dynamic, Protocol::Udp),
                port(
                    "both",
                    "default",
                    PortPolicy::Passthrough,
                    Protocol::TcpUdp,
                ),
            ],
            "",
        );
        let allocated = allocator.allocate(&source).await.unwrap();

        assert_eq!(allocated.spec.ports.len(), 3);
        assert_eq!(allocated.spec.ports[0].host_port, 10);
        assert_eq!(allocated.spec.ports[0].container_port, 7777);

        assert_eq!(allocated.spec.ports[1].name, "both-tcp");
        assert_eq!(allocated.spec.ports[1].host_port, 11);
        assert_eq!(allocated.spec.ports[1].container_port, 11);
        assert_eq!(allocated.spec.ports[1].protocol, Some(Protocol::Tcp));

        assert_eq!(allocated.spec.ports[2].name, "both-udp");
        assert_eq!(allocated.spec.ports[2].host_port, 11);
        assert_eq!(allocated.spec.ports[2].container_port, 11);
        assert_eq!(allocated.spec.ports[2].protocol, Some(Protocol::Udp));
    }

    #[tokio::test]
    async fn startup_sync_respects_existing_ports_and_unschedulable_nodes() {
        let allocator = allocator();
        let mut existing_port =
            port("existing", "default", PortPolicy::Dynamic, Protocol::Udp);
        existing_port.host_port = 10;
        let existing =
            game_server("existing", vec![existing_port], "node-a");

        allocator
            .sync_from_snapshot(
                &[existing],
                &[node("node-a", false), node("node-b", true)],
            )
            .await;

        let allocated = allocator
            .allocate(&game_server(
                "next",
                vec![port(
                    "next",
                    "default",
                    PortPolicy::Dynamic,
                    Protocol::Udp,
                )],
                "",
            ))
            .await
            .unwrap();

        assert_eq!(allocated.spec.ports[0].host_port, 11);
    }

    #[tokio::test]
    async fn synthetic_node_capacity_is_added_when_real_nodes_are_exhausted() {
        let allocator = PortAllocator::new(BTreeMap::from([(
            "default".into(),
            PortRange {
                min_port: 10,
                max_port: 10,
            },
        )]))
        .unwrap();
        allocator
            .sync_from_snapshot(&[], &[node("node-a", false)])
            .await;

        let first = allocator
            .allocate(&game_server(
                "first",
                vec![port(
                    "first",
                    "default",
                    PortPolicy::Dynamic,
                    Protocol::Udp,
                )],
                "",
            ))
            .await
            .unwrap();
        let second = allocator
            .allocate(&game_server(
                "second",
                vec![port(
                    "second",
                    "default",
                    PortPolicy::Dynamic,
                    Protocol::Udp,
                )],
                "",
            ))
            .await
            .unwrap();

        assert_eq!(first.spec.ports[0].host_port, 10);
        assert_eq!(second.spec.ports[0].host_port, 10);
    }

    #[tokio::test]
    async fn deallocation_reuses_a_port_only_for_registered_gameservers() {
        let allocator = allocator();
        allocator
            .sync_from_snapshot(&[], &[node("node-a", false)])
            .await;

        let first = allocator
            .allocate(&game_server(
                "first",
                vec![port(
                    "first",
                    "default",
                    PortPolicy::Dynamic,
                    Protocol::Udp,
                )],
                "",
            ))
            .await
            .unwrap();

        allocator.deallocate(&first).await;

        let second = allocator
            .allocate(&game_server(
                "second",
                vec![port(
                    "second",
                    "default",
                    PortPolicy::Dynamic,
                    Protocol::Udp,
                )],
                "",
            ))
            .await
            .unwrap();
        assert_eq!(first.spec.ports[0].host_port, second.spec.ports[0].host_port);

        let unregistered = game_server(
            "unregistered",
            vec![GameServerPort {
                host_port: second.spec.ports[0].host_port,
                ..port(
                    "unregistered",
                    "default",
                    PortPolicy::Dynamic,
                    Protocol::Udp,
                )
            }],
            "",
        );
        allocator.deallocate(&unregistered).await;

        let third = allocator
            .allocate(&game_server(
                "third",
                vec![port(
                    "third",
                    "default",
                    PortPolicy::Dynamic,
                    Protocol::Udp,
                )],
                "",
            ))
            .await
            .unwrap();
        assert_ne!(third.spec.ports[0].host_port, second.spec.ports[0].host_port);
    }

}
