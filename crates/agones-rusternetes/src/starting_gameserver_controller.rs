use std::collections::HashSet;
use std::net::IpAddr;
use std::sync::Arc;

use agones_core::{GameServer, GameServerState};
use agones_core::gameserver::NodeAddress as CoreNodeAddress;
use cgp::prelude::*;
use rusternetes_common::resources::{Node, Pod};
use rusternetes_storage::{build_key, Storage};

use crate::{game_server_pod, AdapterError, AgonesRepository, ApiHooks};

const POD_IP_ADDRESS_TYPE: &str = "PodIP";

/// Selects the primary Agones node address using the same precedence as upstream:
/// ExternalDNS -> valid ExternalIP -> InternalDNS -> valid InternalIP.
///
/// The returned address list preserves the Node's original order and values.
pub fn node_address(node: &Node) -> Result<(String, Vec<CoreNodeAddress>), AdapterError> {
    let addresses = node
        .status
        .as_ref()
        .and_then(|status| status.addresses.as_deref())
        .unwrap_or_default();

    let primary = addresses
        .iter()
        .find(|address| address.address_type == "ExternalDNS")
        .or_else(|| {
            addresses.iter().find(|address| {
                address.address_type == "ExternalIP"
                    && address.address.parse::<IpAddr>().is_ok()
            })
        })
        .or_else(|| {
            addresses
                .iter()
                .find(|address| address.address_type == "InternalDNS")
        })
        .or_else(|| {
            addresses.iter().find(|address| {
                address.address_type == "InternalIP"
                    && address.address.parse::<IpAddr>().is_ok()
            })
        })
        .ok_or_else(|| {
            AdapterError::Hook(format!(
                "Could not find an address for Node: {}",
                node.metadata.name
            ))
        })?;

    Ok((
        primary.address.clone(),
        addresses
            .iter()
            .map(|address| CoreNodeAddress {
                address: address.address.clone(),
                address_type: address.address_type.clone(),
            })
            .collect(),
    ))
}

/// Mirrors upstream `applyGameServerPodIPs`.
///
/// Only addresses already tagged as PodIP participate in duplicate detection,
/// and the initial address set is intentionally not extended while iterating,
/// matching the Go implementation exactly.
pub fn apply_game_server_pod_ips(game_server: &mut GameServer, pod: &Pod) -> bool {
    let pod_ips = pod
        .status
        .as_ref()
        .and_then(|status| status.pod_i_ps.as_deref())
        .unwrap_or_default();

    if pod_ips.is_empty() {
        return false;
    }

    let existing: HashSet<_> = game_server
        .status
        .addresses
        .iter()
        .filter(|address| address.address_type == POD_IP_ADDRESS_TYPE)
        .map(|address| address.address.clone())
        .collect();

    let mut updated = false;
    for pod_ip in pod_ips {
        if !existing.contains(&pod_ip.ip) {
            game_server.status.addresses.push(CoreNodeAddress {
                address: pod_ip.ip.clone(),
                address_type: POD_IP_ADDRESS_TYPE.to_owned(),
            });
            updated = true;
        }
    }

    updated
}

/// Mirrors upstream `applyGameServerAddressAndPort` against Rusternetes Node
/// and Pod types.
pub fn apply_game_server_address_and_port(
    game_server: &mut GameServer,
    node: &Node,
    pod: &Pod,
    hooks: &impl ApiHooks,
) -> Result<(), AdapterError> {
    let (address, addresses) = node_address(node)?;

    game_server.status.address = address;
    game_server.status.addresses = addresses.into();
    game_server.status.node_name = pod
        .spec
        .as_ref()
        .and_then(|spec| spec.node_name.as_deref())
        .unwrap_or_default()
        .to_owned();

    if let Some(pod_ips) = pod
        .status
        .as_ref()
        .and_then(|status| status.pod_i_ps.as_deref())
    {
        game_server
            .status
            .addresses
            .extend(pod_ips.iter().map(|pod_ip| CoreNodeAddress {
                address: pod_ip.ip.clone(),
                address_type: POD_IP_ADDRESS_TYPE.to_owned(),
            }));
    }

    hooks.sync_pod_ports_to_game_server(game_server, pod)?;

    game_server.status.ports = game_server
        .spec
        .ports
        .iter()
        .map(|port| port.status())
        .collect();

    Ok(())
}

/// CGP capability for upstream `syncGameServerStartingState`.
///
/// The storage and cloud/API hook dependencies are supplied implicitly by the
/// same context introduced for Creating-state reconciliation in milestone 40.
#[cgp_fn]
#[async_trait]
pub async fn reconcile_starting_game_server<S, H>(
    &self,
    #[implicit] storage: Arc<S>,
    #[implicit] hooks: H,
    game_server: GameServer,
) -> Result<Option<GameServer>, AdapterError>
where
    S: Storage + 'static,
    H: ApiHooks + Clone + Send + Sync + 'static,
{
    let mut game_server = game_server;

    if game_server.status.state != Some(GameServerState::Starting)
        || game_server.metadata.deletion_timestamp.is_some()
        || game_server.get_dev_address().is_some()
    {
        return Ok(None);
    }

    if game_server.metadata.name.is_empty() {
        return Err(AdapterError::MissingField("metadata.name"));
    }
    if game_server.metadata.namespace.is_empty() {
        return Err(AdapterError::MissingField("metadata.namespace"));
    }

    let pod = game_server_pod(storage.as_ref(), &game_server).await?;

    let node_name = pod
        .spec
        .as_ref()
        .and_then(|spec| spec.node_name.as_deref())
        .filter(|name| !name.is_empty())
        .ok_or_else(|| {
            AdapterError::Hook(format!(
                "node not yet populated for Pod {}",
                pod.metadata.name
            ))
        })?;

    let node_key = build_key("nodes", None, node_name);
    let node: Node = storage.get(&node_key).await?;

    apply_game_server_address_and_port(&mut game_server, &node, &pod, &hooks)?;
    // Upstream invokes this after address/port population as well. Since the
    // previous helper already appended current PodIPs, this is idempotent here.
    apply_game_server_pod_ips(&mut game_server, &pod);

    game_server.status.state = Some(GameServerState::Scheduled);
    AgonesRepository::new(storage)
        .update(&game_server)
        .await
        .map(Some)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use agones_core::{
        Container as CoreContainer, GameServerPort, GameServerSpec, GameServerStatus, ObjectMeta,
        PodSpec as CorePodSpec, PodTemplateSpec, PortPolicy, Protocol, DEV_ADDRESS_ANNOTATION,
    };
    use rusternetes_common::resources::{
        NodeAddress as NativeNodeAddress, NodeStatus, PodIP, PodStatus,
    };
    use rusternetes_storage::MemoryStorage;

    use crate::{
        build_pod, CreatingGameServerController, NoopApiHooks, PodBuildFeatures,
    };

    use super::*;

    fn starting_game_server() -> GameServer {
        GameServer {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: "server".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSpec {
                container: "game".into(),
                ports: vec![GameServerPort {
                    name: "default".into(),
                    port_policy: Some(PortPolicy::Static),
                    container: Some("game".into()),
                    container_port: 7777,
                    host_port: 9999,
                    protocol: Some(Protocol::Udp),
                    ..Default::default()
                }].into(),
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
                state: Some(GameServerState::Starting),
                ..Default::default()
            },
        }
    }

    fn node(name: &str, addresses: Vec<NativeNodeAddress>) -> Node {
        let mut node = Node::new(name);
        node.status = Some(NodeStatus {
            addresses: Some(addresses),
            ..Default::default()
        });
        node
    }

    #[test]
    fn address_uses_upstream_precedence_and_preserves_all_addresses() {
        let cases = [
            (
                vec![NativeNodeAddress {
                    address_type: "ExternalIP".into(),
                    address: "12.12.12.12".into(),
                }],
                "12.12.12.12",
            ),
            (
                vec![NativeNodeAddress {
                    address_type: "InternalIP".into(),
                    address: "11.11.11.11".into(),
                }],
                "11.11.11.11",
            ),
            (
                vec![
                    NativeNodeAddress {
                        address_type: "InternalIP".into(),
                        address: "12.12.12.12".into(),
                    },
                    NativeNodeAddress {
                        address_type: "ExternalIP".into(),
                        address: "9.9.9.8".into(),
                    },
                ],
                "9.9.9.8",
            ),
            (
                vec![
                    NativeNodeAddress {
                        address_type: "InternalDNS".into(),
                        address: "internal.example.com".into(),
                    },
                    NativeNodeAddress {
                        address_type: "ExternalDNS".into(),
                        address: "external.example.com".into(),
                    },
                ],
                "external.example.com",
            ),
            (
                vec![
                    NativeNodeAddress {
                        address_type: "ExternalIP".into(),
                        address: "not-an-ip".into(),
                    },
                    NativeNodeAddress {
                        address_type: "InternalDNS".into(),
                        address: "internal.example.com".into(),
                    },
                ],
                "internal.example.com",
            ),
        ];

        for (addresses, expected) in cases {
            let node = node("node1", addresses.clone());
            let (primary, copied) = node_address(&node).unwrap();
            assert_eq!(primary, expected);
            assert_eq!(copied.len(), addresses.len());
            for (actual, source) in copied.iter().zip(addresses.iter()) {
                assert_eq!(actual.address, source.address);
                assert_eq!(actual.address_type, source.address_type);
            }
        }
    }

    #[test]
    fn address_rejects_node_without_usable_address() {
        let node = node("node1", Vec::new());
        let error = node_address(&node).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("Could not find an address for Node: node1")
        );
    }

    fn pod_with_ips(ips: &[&str]) -> Pod {
        let mut pod = Pod::new("server", Default::default());
        pod.status = Some(PodStatus {
            pod_i_ps: Some(
                ips.iter()
                    .map(|ip| PodIP {
                        ip: (*ip).to_owned(),
                    })
                    .collect(),
            ),
            ..Default::default()
        });
        pod
    }

    #[test]
    fn pod_ips_add_only_addresses_missing_from_existing_pod_ip_status() {
        let mut game_server = starting_game_server();
        game_server.status.addresses = vec![CoreNodeAddress {
            address: "10.0.0.1".into(),
            address_type: POD_IP_ADDRESS_TYPE.into(),
        }].into();
        let pod = pod_with_ips(&["10.0.0.1", "10.0.0.2"]);

        assert!(apply_game_server_pod_ips(&mut game_server, &pod));
        assert_eq!(game_server.status.addresses.len(), 2);
        assert_eq!(game_server.status.addresses[0].address, "10.0.0.1");
        assert_eq!(game_server.status.addresses[1].address, "10.0.0.2");
        assert_eq!(
            game_server.status.addresses[1].address_type,
            POD_IP_ADDRESS_TYPE
        );
    }

    #[test]
    fn pod_ips_are_noop_when_absent_or_already_present() {
        let mut game_server = starting_game_server();
        assert!(!apply_game_server_pod_ips(
            &mut game_server,
            &Pod::new("server", Default::default())
        ));

        game_server.status.addresses = vec![CoreNodeAddress {
            address: "10.0.0.1".into(),
            address_type: POD_IP_ADDRESS_TYPE.into(),
        }].into();
        assert!(!apply_game_server_pod_ips(
            &mut game_server,
            &pod_with_ips(&["10.0.0.1"])
        ));
        assert_eq!(game_server.status.addresses.len(), 1);
    }

    #[derive(Clone, Copy)]
    struct PodPortSyncHook;

    impl ApiHooks for PodPortSyncHook {
        fn sync_pod_ports_to_game_server(
            &self,
            game_server: &mut GameServer,
            pod: &Pod,
        ) -> Result<(), AdapterError> {
            let host_port = pod
                .spec
                .as_ref()
                .and_then(|spec| spec.containers.first())
                .and_then(|container| container.ports.as_ref())
                .and_then(|ports| ports.first())
                .and_then(|port| port.host_port)
                .ok_or_else(|| AdapterError::Hook("missing Pod host port".into()))?;

            game_server.spec.ports[0].host_port = i32::from(host_port);
            Ok(())
        }
    }

    #[tokio::test]
    async fn scheduled_pod_populates_status_and_advances_to_scheduled() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&starting_game_server()).await.unwrap();

        let mut pod = build_pod(
            &created,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.spec.as_mut().unwrap().node_name = Some("node1".into());
        pod.spec.as_mut().unwrap().containers[0]
            .ports
            .as_mut()
            .unwrap()[0]
            .host_port = Some(9876);
        pod.status = Some(PodStatus {
            pod_i_ps: Some(vec![PodIP {
                ip: "2001:db8::1".into(),
            }]),
            ..Default::default()
        });
        storage
            .create(&build_key("pods", Some("default"), "server"), &pod)
            .await
            .unwrap();

        let node = node(
            "node1",
            vec![NativeNodeAddress {
                address_type: "ExternalIP".into(),
                address: "12.12.12.12".into(),
            }],
        );
        storage
            .create(&build_key("nodes", None, "node1"), &node)
            .await
            .unwrap();

        let controller = CreatingGameServerController::with_context(
            storage.clone(),
            Vec::new(),
            PodBuildFeatures::default(),
            PodPortSyncHook,
        );
        let reconciled = controller
            .reconcile_starting_game_server(created)
            .await
            .unwrap()
            .expect("Starting GameServer should reconcile");

        assert_eq!(reconciled.status.state, Some(GameServerState::Scheduled));
        assert_eq!(reconciled.status.address, "12.12.12.12");
        assert_eq!(reconciled.status.node_name, "node1");
        assert_eq!(reconciled.status.addresses.len(), 2);
        assert_eq!(reconciled.status.addresses[0].address_type, "ExternalIP");
        assert_eq!(reconciled.status.addresses[1].address_type, POD_IP_ADDRESS_TYPE);
        assert_eq!(reconciled.status.addresses[1].address, "2001:db8::1");
        assert_eq!(reconciled.spec.ports[0].host_port, 9876);
        assert_eq!(reconciled.status.ports[0].port, 9876);

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(persisted, reconciled);
    }

    #[tokio::test]
    async fn missing_or_unscheduled_pod_does_not_advance_state() {
        let storage = MemoryStorage::new();
        let repository = AgonesRepository::new(storage.clone());
        let created = repository.create(&starting_game_server()).await.unwrap();
        let controller = CreatingGameServerController::new(storage.clone());

        let missing_error = controller
            .reconcile_starting_game_server(created.clone())
            .await
            .unwrap_err();
        assert!(matches!(
            missing_error,
            AdapterError::Rusternetes(rusternetes_common::Error::NotFound(_))
        ));

        let mut pod = build_pod(
            &created,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            "different-owner".into();
        let pod_key = build_key("pods", Some("default"), "server");
        storage.create(&pod_key, &pod).await.unwrap();

        let foreign_error = controller
            .reconcile_starting_game_server(created.clone())
            .await
            .unwrap_err();
        assert!(matches!(
            foreign_error,
            AdapterError::Rusternetes(rusternetes_common::Error::NotFound(_))
        ));

        pod.metadata.owner_references.as_mut().unwrap()[0].uid =
            created.metadata.uid.clone();
        storage.update(&pod_key, &pod).await.unwrap();

        let unscheduled_error = controller
            .reconcile_starting_game_server(created.clone())
            .await
            .unwrap_err();
        assert!(
            unscheduled_error
                .to_string()
                .contains("node not yet populated for Pod server")
        );

        let persisted: GameServer = repository.get("default", "server").await.unwrap();
        assert_eq!(persisted.status.state, Some(GameServerState::Starting));
    }

    #[tokio::test]
    async fn deleted_development_and_nonstarting_gameservers_are_noops() {
        let storage = MemoryStorage::new();
        let controller = CreatingGameServerController::new(storage);

        let mut deleted = starting_game_server();
        deleted.metadata.deletion_timestamp = Some("2026-10-17T00:00:00Z".into());
        assert!(
            controller
                .reconcile_starting_game_server(deleted)
                .await
                .unwrap()
                .is_none()
        );

        let mut development = starting_game_server();
        development.metadata.annotations = BTreeMap::from([(
            DEV_ADDRESS_ANNOTATION.to_owned(),
            "203.0.113.7".to_owned(),
        )]);
        assert!(
            controller
                .reconcile_starting_game_server(development)
                .await
                .unwrap()
                .is_none()
        );

        let mut ready = starting_game_server();
        ready.status.state = Some(GameServerState::Ready);
        assert!(
            controller
                .reconcile_starting_game_server(ready)
                .await
                .unwrap()
                .is_none()
        );
    }
}
