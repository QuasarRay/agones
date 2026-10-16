//! Native Rusternetes integration for the Agones domain model.
//!
//! This crate is the only layer that depends on Rusternetes runtime/API types.
//! `agones-core` remains runtime-independent.

use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt;

use agones_core::{
    Eviction, GameServer, GameServerSpec, PortPolicy, SchedulingStrategy, FINALIZER_NAME,
    VERSION_ANNOTATION,
};
use rusternetes_common::resources::{
    Affinity, Container, ContainerPort, Pod, PodAffinity, PodAffinityTerm, PodSpec, Volume,
    VolumeMount, WeightedPodAffinityTerm,
};
use rusternetes_common::types::{LabelSelector, OwnerReference, TypeMeta};

pub const ROLE_LABEL: &str = "agones.dev/role";
pub const GAME_SERVER_LABEL_ROLE: &str = "gameserver";
pub const GAME_SERVER_POD_LABEL: &str = "agones.dev/gameserver";
pub const GAME_SERVER_CONTAINER_ANNOTATION: &str = "agones.dev/container";
pub const PASSTHROUGH_PORT_ASSIGNMENT_ANNOTATION: &str =
    "agones.dev/container-passthrough-port-assignment";

#[derive(Debug)]
pub enum AdapterError {
    Serde(serde_json::Error),
    InvalidPort { field: &'static str, value: i32 },
    ContainerNotFound(String),
    Hook(String),
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serde(error) => write!(f, "Rusternetes conversion failed: {error}"),
            Self::InvalidPort { field, value } => {
                write!(f, "{field} value {value} is outside the u16 port range")
            }
            Self::ContainerNotFound(name) => {
                write!(f, "failed to find container named {name} in pod spec")
            }
            Self::Hook(error) => f.write_str(error),
        }
    }
}

impl Error for AdapterError {}

impl From<serde_json::Error> for AdapterError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

pub trait ApiHooks {
    fn mutate_game_server_pod(
        &self,
        _spec: &GameServerSpec,
        _pod: &mut Pod,
    ) -> Result<(), AdapterError> {
        Ok(())
    }

    fn set_eviction(
        &self,
        _eviction: Option<&Eviction>,
        _pod: &mut Pod,
    ) -> Result<(), AdapterError> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NoopApiHooks;

impl ApiHooks for NoopApiHooks {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PodBuildFeatures {
    pub sidecar_containers: bool,
}

fn to_native<T: serde::de::DeserializeOwned>(
    value: &impl serde::Serialize,
) -> Result<T, AdapterError> {
    Ok(serde_json::from_value(serde_json::to_value(value)?)?)
}

fn port(value: i32, field: &'static str) -> Result<u16, AdapterError> {
    u16::try_from(value).map_err(|_| AdapterError::InvalidPort { field, value })
}

fn apply_to_container(
    spec: &mut PodSpec,
    name: &str,
    mut apply: impl FnMut(&mut Container),
) -> Result<(), AdapterError> {
    if let Some(container) = spec
        .containers
        .iter_mut()
        .find(|container| container.name == name)
    {
        apply(container);
        return Ok(());
    }

    if let Some(container) = spec
        .init_containers
        .as_mut()
        .and_then(|containers| {
            containers.iter_mut().find(|container| {
                container.restart_policy.as_deref() == Some("Always") && container.name == name
            })
        })
    {
        apply(container);
        return Ok(());
    }

    Err(AdapterError::ContainerNotFound(name.to_owned()))
}

fn apply_packed_scheduling(spec: &mut PodSpec) {
    let weighted = WeightedPodAffinityTerm {
        weight: 100,
        pod_affinity_term: PodAffinityTerm {
            label_selector: LabelSelector {
                match_labels: Some(HashMap::from([(
                    ROLE_LABEL.to_owned(),
                    GAME_SERVER_LABEL_ROLE.to_owned(),
                )])),
                match_expressions: None,
            },
            namespaces: None,
            topology_key: "kubernetes.io/hostname".to_owned(),
        },
    };

    let affinity = spec.affinity.get_or_insert_with(|| Affinity {
        node_affinity: None,
        pod_affinity: None,
        pod_anti_affinity: None,
    });
    let pod_affinity = affinity.pod_affinity.get_or_insert_with(|| PodAffinity {
        required_during_scheduling_ignored_during_execution: None,
        preferred_during_scheduling_ignored_during_execution: None,
    });
    pod_affinity
        .preferred_during_scheduling_ignored_during_execution
        .get_or_insert_with(Vec::new)
        .push(weighted);
}

pub fn build_pod(
    game_server: &GameServer,
    sidecars: Vec<Container>,
    features: PodBuildFeatures,
    hooks: &impl ApiHooks,
) -> Result<Pod, AdapterError> {
    let mut spec: PodSpec = to_native(&game_server.spec.template.spec)?;
    let mut metadata: rusternetes_common::types::ObjectMeta =
        to_native(&game_server.spec.template.metadata)?;

    if spec.hostname.as_deref().unwrap_or("").is_empty() {
        spec.hostname = Some(game_server.metadata.name.replace('.', "-"));
    }

    metadata.generate_name = None;
    metadata.name = game_server.metadata.name.clone();
    metadata.namespace = if game_server.metadata.namespace.is_empty() {
        None
    } else {
        Some(game_server.metadata.namespace.clone())
    };
    metadata.resource_version = None;
    metadata.uid.clear();

    let labels = metadata.labels.get_or_insert_with(HashMap::new);
    labels.insert(ROLE_LABEL.to_owned(), GAME_SERVER_LABEL_ROLE.to_owned());
    labels.insert(
        GAME_SERVER_POD_LABEL.to_owned(),
        game_server.metadata.name.clone(),
    );

    let annotations = metadata.annotations.get_or_insert_with(HashMap::new);
    annotations.insert(
        GAME_SERVER_CONTAINER_ANNOTATION.to_owned(),
        game_server.spec.container.clone(),
    );
    annotations.insert(
        VERSION_ANNOTATION.to_owned(),
        game_server
            .metadata
            .annotations
            .get(VERSION_ANNOTATION)
            .cloned()
            .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_owned()),
    );

    let owner = OwnerReference::new(
        "agones.dev/v1",
        "GameServer",
        game_server.metadata.name.clone(),
        game_server.metadata.uid.clone(),
    )
    .with_controller(true);
    metadata
        .owner_references
        .get_or_insert_with(Vec::new)
        .push(owner);

    let mut passthrough: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for game_port in &game_server.spec.ports {
        let container_name = game_port
            .container
            .as_deref()
            .unwrap_or(game_server.spec.container.as_str());

        let host_port = if game_port.port_policy == Some(PortPolicy::None) || game_port.host_port == 0
        {
            None
        } else {
            Some(port(game_port.host_port, "hostPort")?)
        };
        let native_port = ContainerPort {
            container_port: port(game_port.container_port, "containerPort")?,
            name: None,
            protocol: game_port
                .protocol
                .as_ref()
                .map(|protocol| protocol.as_str().to_owned()),
            host_port,
            host_ip: None,
        };

        let mut index = 0usize;
        apply_to_container(&mut spec, container_name, |container| {
            let ports = container.ports.get_or_insert_with(Vec::new);
            index = ports.len();
            ports.push(native_port.clone());
        })?;

        if game_port.port_policy == Some(PortPolicy::Passthrough) {
            passthrough
                .entry(container_name.to_owned())
                .or_default()
                .push(index);
        }
    }

    if !passthrough.is_empty() {
        metadata.annotations.get_or_insert_with(HashMap::new).insert(
            PASSTHROUGH_PORT_ASSIGNMENT_ANNOTATION.to_owned(),
            serde_json::to_string(&passthrough)?,
        );
    }

    if features.sidecar_containers {
        let mut sidecars = sidecars;
        for sidecar in &mut sidecars {
            sidecar.restart_policy = Some("Always".to_owned());
        }
        let init = spec.init_containers.get_or_insert_with(Vec::new);
        sidecars.append(init);
        *init = sidecars;
        if spec.restart_policy.as_deref().unwrap_or("").is_empty() {
            spec.restart_policy = Some("Never".to_owned());
        }
    } else if !sidecars.is_empty() {
        let mut containers = sidecars;
        containers.append(&mut spec.containers);
        spec.containers = containers;
    }

    if game_server.spec.scheduling == Some(SchedulingStrategy::Packed) {
        apply_packed_scheduling(&mut spec);
    }

    let mut pod = Pod {
        type_meta: TypeMeta {
            kind: "Pod".to_owned(),
            api_version: "v1".to_owned(),
        },
        metadata,
        spec: Some(spec),
        status: None,
    };

    hooks.mutate_game_server_pod(&game_server.spec, &mut pod)?;
    hooks.set_eviction(game_server.status.eviction.as_ref(), &mut pod)?;
    Ok(pod)
}

pub fn disable_service_account(pod: &mut Pod, container_name: &str) -> Result<(), AdapterError> {
    let spec = pod
        .spec
        .as_mut()
        .ok_or_else(|| AdapterError::Hook("pod spec is missing".to_owned()))?;

    let volume: Volume = serde_json::from_value(serde_json::json!({
        "name": "empty",
        "emptyDir": {}
    }))?;
    spec.volumes.get_or_insert_with(Vec::new).push(volume);

    let mount: VolumeMount = serde_json::from_value(serde_json::json!({
        "name": "empty",
        "mountPath": "/var/run/secrets/kubernetes.io/serviceaccount",
        "readOnly": true
    }))?;
    apply_to_container(spec, container_name, |container| {
        container
            .volume_mounts
            .get_or_insert_with(Vec::new)
            .push(mount.clone());
    })
}

pub fn ensure_game_server_finalizer(game_server: &mut GameServer) {
    game_server.metadata.finalizers.push(FINALIZER_NAME.to_owned());
}

#[cfg(test)]
mod tests {
    use super::*;
    use agones_core::{
        Container as CoreContainer, GameServerPort, GameServerState, PodSpec as CorePodSpec,
        PodTemplateSpec, Protocol,
    };

    fn fixture() -> GameServer {
        let mut gs = GameServer {
            metadata: agones_core::ObjectMeta {
                name: "game.server".into(),
                namespace: "default".into(),
                uid: "1234".into(),
                ..Default::default()
            },
            spec: GameServerSpec {
                container: "game".into(),
                ports: vec![GameServerPort {
                    name: "main".into(),
                    port_policy: Some(PortPolicy::Static),
                    container: Some("game".into()),
                    container_port: 7777,
                    host_port: 9999,
                    protocol: Some(Protocol::Udp),
                    ..Default::default()
                }],
                scheduling: Some(SchedulingStrategy::Packed),
                template: PodTemplateSpec {
                    spec: CorePodSpec {
                        containers: vec![CoreContainer {
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
            status: agones_core::GameServerStatus {
                state: Some(GameServerState::Creating),
                ..Default::default()
            },
        };
        gs.metadata
            .annotations
            .insert(VERSION_ANNOTATION.into(), "test-version".into());
        gs
    }

    #[test]
    fn builds_native_rusternetes_pod_with_agones_metadata_and_port() {
        let gs = fixture();
        let pod = build_pod(
            &gs,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();

        assert_eq!(pod.metadata.name, "game.server");
        assert_eq!(pod.metadata.namespace.as_deref(), Some("default"));
        assert_eq!(
            pod.metadata.labels.as_ref().unwrap()[ROLE_LABEL],
            GAME_SERVER_LABEL_ROLE
        );
        assert_eq!(
            pod.metadata.annotations.as_ref().unwrap()[GAME_SERVER_CONTAINER_ANNOTATION],
            "game"
        );
        let spec = pod.spec.unwrap();
        assert_eq!(spec.hostname.as_deref(), Some("game-server"));
        let port = &spec.containers[0].ports.as_ref().unwrap()[0];
        assert_eq!(port.container_port, 7777);
        assert_eq!(port.host_port, Some(9999));
        assert_eq!(port.protocol.as_deref(), Some("UDP"));
        assert!(spec
            .affinity
            .unwrap()
            .pod_affinity
            .unwrap()
            .preferred_during_scheduling_ignored_during_execution
            .is_some());
    }

    #[test]
    fn hostname_rewrites_dots_but_preserves_explicit_template_hostname() {
        let gs = fixture();
        let pod = build_pod(
            &gs,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        assert_eq!(pod.spec.unwrap().hostname.as_deref(), Some("game-server"));

        let mut explicit = fixture();
        explicit.spec.template.spec.extensions.insert(
            "hostname".into(),
            serde_json::Value::String("ORANGE".into()),
        );
        let pod = build_pod(
            &explicit,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        assert_eq!(pod.spec.unwrap().hostname.as_deref(), Some("ORANGE"));
    }

    #[test]
    fn missing_port_container_returns_original_error_shape() {
        let mut gs = fixture();
        gs.spec.ports[0].container = Some("missing".into());
        let error = build_pod(
            &gs,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "failed to find container named missing in pod spec"
        );
    }

    #[test]
    fn regular_sidecars_are_prepended_and_template_service_account_is_preserved() {
        let mut gs = fixture();
        gs.spec.template.spec.extensions.insert(
            "serviceAccountName".into(),
            serde_json::Value::String("other-agones-sdk".into()),
        );
        let sidecar = Container {
            name: "sidecar".into(),
            image: "container/sidecar".into(),
            ..Default::default()
        };

        let pod = build_pod(
            &gs,
            vec![sidecar],
            PodBuildFeatures {
                sidecar_containers: false,
            },
            &NoopApiHooks,
        )
        .unwrap();
        let spec = pod.spec.unwrap();
        assert_eq!(spec.service_account_name.as_deref(), Some("other-agones-sdk"));
        assert_eq!(spec.containers.len(), 2);
        assert_eq!(spec.containers[0].name, "sidecar");
        assert_eq!(spec.containers[1].name, "game");
    }

    #[test]
    fn init_sidecars_prepend_existing_init_containers_and_force_always() {
        let mut gs = fixture();
        gs.spec.template.spec.init_containers = vec![
            CoreContainer {
                name: "existing-init-1".into(),
                image: "existing/init1".into(),
                ..Default::default()
            },
            CoreContainer {
                name: "existing-init-2".into(),
                image: "existing/init2".into(),
                ..Default::default()
            },
        ];

        let sidecars = vec![
            Container {
                name: "sidecar-1".into(),
                image: "container/sidecar1".into(),
                ..Default::default()
            },
            Container {
                name: "sidecar-2".into(),
                image: "container/sidecar2".into(),
                ..Default::default()
            },
        ];

        let pod = build_pod(
            &gs,
            sidecars,
            PodBuildFeatures {
                sidecar_containers: true,
            },
            &NoopApiHooks,
        )
        .unwrap();
        let spec = pod.spec.unwrap();
        let init = spec.init_containers.unwrap();
        assert_eq!(
            init.iter().map(|container| container.name.as_str()).collect::<Vec<_>>(),
            vec!["sidecar-1", "sidecar-2", "existing-init-1", "existing-init-2"]
        );
        assert_eq!(init[0].restart_policy.as_deref(), Some("Always"));
        assert_eq!(init[1].restart_policy.as_deref(), Some("Always"));
        assert_eq!(spec.restart_policy.as_deref(), Some("Never"));
    }

    #[test]
    fn multiple_port_allocations_target_the_declared_containers() {
        let mut gs = fixture();
        gs.spec.template.spec.containers.push(CoreContainer {
            name: "authContainer".into(),
            image: "auth:v1".into(),
            ..Default::default()
        });
        gs.spec.ports.push(GameServerPort {
            name: "auth".into(),
            port_policy: Some(PortPolicy::Dynamic),
            container: Some("authContainer".into()),
            container_port: 5000,
            host_port: 6000,
            protocol: Some(Protocol::Udp),
            ..Default::default()
        });

        let pod = build_pod(
            &gs,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        let spec = pod.spec.unwrap();
        let game_port = &spec.containers[0].ports.as_ref().unwrap()[0];
        let auth_port = &spec.containers[1].ports.as_ref().unwrap()[0];
        assert_eq!(game_port.container_port, 7777);
        assert_eq!(game_port.host_port, Some(9999));
        assert_eq!(auth_port.container_port, 5000);
        assert_eq!(auth_port.host_port, Some(6000));
    }

    #[test]
    fn distributed_scheduling_does_not_add_packed_affinity() {
        let mut gs = fixture();
        gs.spec.scheduling = Some(SchedulingStrategy::Distributed);
        let pod = build_pod(
            &gs,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        assert!(pod.spec.unwrap().affinity.is_none());
    }

    #[test]
    fn controller_reference_matches_kubernetes_new_controller_ref() {
        let gs = fixture();
        let pod = build_pod(
            &gs,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        let owner = &pod.metadata.owner_references.as_ref().unwrap()[0];
        assert_eq!(owner.kind, "GameServer");
        assert_eq!(owner.name, "game.server");
        assert_eq!(owner.uid, "1234");
        assert_eq!(owner.controller, Some(true));
        assert_eq!(owner.block_owner_deletion, Some(true));
    }

    #[test]
    fn sidecar_feature_uses_restartable_init_containers() {
        let gs = fixture();
        let sidecar = Container {
            name: "sdk".into(),
            image: "sdk:v1".into(),
            ..Default::default()
        };
        let pod = build_pod(
            &gs,
            vec![sidecar],
            PodBuildFeatures {
                sidecar_containers: true,
            },
            &NoopApiHooks,
        )
        .unwrap();
        let spec = pod.spec.unwrap();
        assert_eq!(spec.restart_policy.as_deref(), Some("Never"));
        let init = spec.init_containers.unwrap();
        assert_eq!(init[0].name, "sdk");
        assert_eq!(init[0].restart_policy.as_deref(), Some("Always"));
    }

    #[test]
    fn passthrough_ports_record_native_container_port_indices() {
        let mut gs = fixture();
        gs.spec.ports[0].port_policy = Some(PortPolicy::Passthrough);
        gs.spec.ports[0].host_port = 7777;
        let pod = build_pod(
            &gs,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        let annotation = &pod.metadata.annotations.as_ref().unwrap()
            [PASSTHROUGH_PORT_ASSIGNMENT_ANNOTATION];
        let map: HashMap<String, Vec<usize>> = serde_json::from_str(annotation).unwrap();
        assert_eq!(map["game"], vec![0]);
    }

    #[test]
    fn passthrough_annotation_is_deterministic_for_multiple_containers() {
        let mut gs = fixture();
        gs.spec.template.spec.containers.extend([
            CoreContainer {
                name: "containerTwo".into(),
                image: "two:v1".into(),
                ..Default::default()
            },
            CoreContainer {
                name: "containerThree".into(),
                image: "three:v1".into(),
                ..Default::default()
            },
        ]);
        gs.spec.ports = vec![
            GameServerPort {
                port_policy: Some(PortPolicy::Passthrough),
                container: Some("game".into()),
                container_port: 7001,
                host_port: 7001,
                protocol: Some(Protocol::Udp),
                ..Default::default()
            },
            GameServerPort {
                port_policy: Some(PortPolicy::Static),
                container: Some("game".into()),
                container_port: 7002,
                host_port: 7002,
                protocol: Some(Protocol::Udp),
                ..Default::default()
            },
            GameServerPort {
                port_policy: Some(PortPolicy::Passthrough),
                container: Some("containerTwo".into()),
                container_port: 7003,
                host_port: 7003,
                protocol: Some(Protocol::Udp),
                ..Default::default()
            },
            GameServerPort {
                port_policy: Some(PortPolicy::Passthrough),
                container: Some("game".into()),
                container_port: 7004,
                host_port: 7004,
                protocol: Some(Protocol::Udp),
                ..Default::default()
            },
            GameServerPort {
                port_policy: Some(PortPolicy::Passthrough),
                container: Some("containerThree".into()),
                container_port: 7005,
                host_port: 7005,
                protocol: Some(Protocol::Udp),
                ..Default::default()
            },
        ];

        let pod = build_pod(
            &gs,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        let annotation = &pod.metadata.annotations.as_ref().unwrap()
            [PASSTHROUGH_PORT_ASSIGNMENT_ANNOTATION];
        assert_eq!(
            annotation,
            r#"{"containerThree":[0],"containerTwo":[0],"game":[0,2]}"#
        );
    }

    #[test]
    fn service_account_suppression_targets_game_container() {
        let gs = fixture();
        let mut pod = build_pod(
            &gs,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        disable_service_account(&mut pod, "game").unwrap();
        let spec = pod.spec.unwrap();
        assert_eq!(spec.volumes.unwrap()[0].name, "empty");
        let mount = &spec.containers[0].volume_mounts.as_ref().unwrap()[0];
        assert_eq!(
            mount.mount_path,
            "/var/run/secrets/kubernetes.io/serviceaccount"
        );
        assert_eq!(mount.read_only, Some(true));
    }
}
