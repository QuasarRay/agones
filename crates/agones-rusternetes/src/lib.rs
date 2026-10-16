//! Native Rusternetes integration for the Agones domain model.
//!
//! This crate is the only layer that depends on Rusternetes runtime/API types.
//! `agones-core` remains runtime-independent.

use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt;

use agones_core::{
    Eviction, GameServer, GameServerSpec, PortPolicy, SchedulingStrategy, ValidationError,
    ValidationErrorKind, FINALIZER_NAME, VERSION_ANNOTATION,
};
use rusternetes_common::resources::{
    Affinity, Container, ContainerPort, CustomResourceColumnDefinition,
    CustomResourceDefinition, CustomResourceDefinitionNames, CustomResourceDefinitionSpec,
    CustomResourceDefinitionVersion, CustomResourceSubresourceScale, CustomResourceSubresources,
    CustomResourceValidation,
    Pod, PodAffinity, PodAffinityTerm, PodSpec, ResourceScope, Volume, VolumeMount,
    WeightedPodAffinityTerm,
};
use rusternetes_common::types::{
    LabelSelector, ObjectMeta as RusternetesObjectMeta, OwnerReference, ResourceRequirements,
    TypeMeta,
};

pub const ROLE_LABEL: &str = "agones.dev/role";
pub const GAME_SERVER_LABEL_ROLE: &str = "gameserver";
pub const GAME_SERVER_POD_LABEL: &str = "agones.dev/gameserver";
pub const GAME_SERVER_CONTAINER_ANNOTATION: &str = "agones.dev/container";
pub const PASSTHROUGH_PORT_ASSIGNMENT_ANNOTATION: &str =
    "agones.dev/container-passthrough-port-assignment";

pub const GAME_SERVER_CRD_NAME: &str = "gameservers.agones.dev";
pub const GAME_SERVER_CRD_GROUP: &str = "agones.dev";
pub const GAME_SERVER_CRD_VERSION: &str = "v1";
pub const GAME_SERVER_CRD_KIND: &str = "GameServer";
pub const GAME_SERVER_CRD_PLURAL: &str = "gameservers";
pub const GAME_SERVER_CRD_SINGULAR: &str = "gameserver";
pub const GAME_SERVER_CRD_SHORT_NAME: &str = "gs";

/// Agones enables these selectable fields in its default Helm values.
///
/// The pinned Rusternetes CustomResourceDefinitionVersion model does not yet
/// expose Kubernetes' selectableFields field, so milestone 21 records the
/// source-of-truth values but does not silently drop them from a claimed
/// full-fidelity CRD representation.
pub const GAME_SERVER_CRD_SELECTABLE_FIELDS: &[&str] =
    &[".status.state", ".status.nodeName"];

mod generated_game_server_schema {
    include!(concat!(env!("OUT_DIR"), "/game_server_schema.rs"));
}

pub fn game_server_openapi_validation() -> CustomResourceValidation {
    CustomResourceValidation {
        open_apiv3_schema: serde_json::from_str(
            generated_game_server_schema::GAME_SERVER_OPENAPI_SCHEMA_JSON,
        )
        .expect("build-generated GameServer OpenAPI schema must match Rusternetes schema types"),
    }
}

/// Builds the GameServer CRD using native Rusternetes CRD types.
///
/// The OpenAPI schema is generated at build time from Agones' Helm schema
/// fragments and the vendored Kubernetes PodTemplate schema, so the Rust
/// representation follows the machine-readable source instead of duplicating
/// hundreds of schema fields by hand.
pub fn game_server_crd_envelope() -> CustomResourceDefinition {
    CustomResourceDefinition {
        api_version: "apiextensions.k8s.io/v1".to_owned(),
        kind: "CustomResourceDefinition".to_owned(),
        metadata: RusternetesObjectMeta {
            name: GAME_SERVER_CRD_NAME.to_owned(),
            ..Default::default()
        },
        spec: CustomResourceDefinitionSpec {
            group: GAME_SERVER_CRD_GROUP.to_owned(),
            names: CustomResourceDefinitionNames {
                plural: GAME_SERVER_CRD_PLURAL.to_owned(),
                singular: Some(GAME_SERVER_CRD_SINGULAR.to_owned()),
                kind: GAME_SERVER_CRD_KIND.to_owned(),
                short_names: Some(vec![GAME_SERVER_CRD_SHORT_NAME.to_owned()]),
                categories: None,
                list_kind: None,
            },
            scope: ResourceScope::Namespaced,
            versions: vec![CustomResourceDefinitionVersion {
                name: GAME_SERVER_CRD_VERSION.to_owned(),
                served: true,
                storage: true,
                deprecated: None,
                deprecation_warning: None,
                schema: Some(game_server_openapi_validation()),
                subresources: Some(CustomResourceSubresources {
                    status: None,
                    scale: Some(CustomResourceSubresourceScale {
                        spec_replicas_path: ".spec.immutableReplicas".to_owned(),
                        status_replicas_path: ".status.immutableReplicas".to_owned(),
                        label_selector_path: None,
                    }),
                }),
                additional_printer_columns: Some(vec![
                    CustomResourceColumnDefinition {
                        name: "State".to_owned(),
                        type_: "string".to_owned(),
                        format: None,
                        description: None,
                        priority: None,
                        json_path: ".status.state".to_owned(),
                    },
                    CustomResourceColumnDefinition {
                        name: "Address".to_owned(),
                        type_: "string".to_owned(),
                        format: None,
                        description: None,
                        priority: None,
                        json_path: ".status.address".to_owned(),
                    },
                    CustomResourceColumnDefinition {
                        name: "Port".to_owned(),
                        type_: "string".to_owned(),
                        format: None,
                        description: None,
                        priority: None,
                        json_path: ".status.ports[0].port".to_owned(),
                    },
                    CustomResourceColumnDefinition {
                        name: "Node".to_owned(),
                        type_: "string".to_owned(),
                        format: None,
                        description: None,
                        priority: None,
                        json_path: ".status.nodeName".to_owned(),
                    },
                    CustomResourceColumnDefinition {
                        name: "Age".to_owned(),
                        type_: "date".to_owned(),
                        format: None,
                        description: None,
                        priority: None,
                        json_path: ".metadata.creationTimestamp".to_owned(),
                    },
                ]),
            }],
            conversion: None,
            preserve_unknown_fields: None,
        },
        status: None,
    }
}

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

const QUANTITY_SUFFIXES: &[(&str, f64)] = &[
    ("Ki", 1024.0),
    ("Mi", 1_048_576.0),
    ("Gi", 1_073_741_824.0),
    ("Ti", 1_099_511_627_776.0),
    ("Pi", 1_125_899_906_842_624.0),
    ("Ei", 1_152_921_504_606_846_976.0),
    ("m", 0.001),
    ("k", 1_000.0),
    ("M", 1_000_000.0),
    ("G", 1_000_000_000.0),
    ("T", 1_000_000_000_000.0),
    ("P", 1_000_000_000_000_000.0),
    ("E", 1_000_000_000_000_000_000.0),
];

fn parse_quantity_value(quantity: &str) -> Option<f64> {
    let quantity = quantity.trim();
    if quantity.is_empty() {
        return None;
    }

    let value = match quantity.parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            let (digits, multiplier) = QUANTITY_SUFFIXES
                .iter()
                .find_map(|(suffix, multiplier)| {
                    quantity
                        .strip_suffix(suffix)
                        .map(|digits| (digits, multiplier))
                })?;
            digits.trim().parse::<f64>().ok()? * multiplier
        }
    };

    value.is_finite().then_some(value)
}

fn invalid_resource_quantity(path: String, value: &str, detail: String) -> ValidationError {
    ValidationError {
        kind: ValidationErrorKind::Invalid,
        path,
        bad_value: Some(value.to_owned()),
        detail,
    }
}

pub fn validate_resource_requirements(
    requirements: &ResourceRequirements,
    path: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let limits = requirements.limits.as_ref();
    let requests = requirements.requests.as_ref();

    if let Some(limits) = limits {
        let mut names: Vec<_> = limits.keys().collect();
        names.sort();
        for name in names {
            let quantity = &limits[name];
            if let Some(value) = parse_quantity_value(quantity) {
                if value < 0.0 {
                    errors.push(invalid_resource_quantity(
                        format!("{path}.limits[{name}]"),
                        quantity,
                        "must be greater than or equal to 0".to_owned(),
                    ));
                }
            }
        }
    }

    if let Some(requests) = requests {
        let mut names: Vec<_> = requests.keys().collect();
        names.sort();
        for name in names {
            let quantity = &requests[name];
            let request = parse_quantity_value(quantity);

            if request.is_some_and(|value| value < 0.0) {
                errors.push(invalid_resource_quantity(
                    format!("{path}.requests[{name}]"),
                    quantity,
                    "must be greater than or equal to 0".to_owned(),
                ));
            }

            let Some(limit_quantity) = limits.and_then(|limits| limits.get(name)) else {
                continue;
            };
            let Some(request) = request else {
                continue;
            };
            let Some(limit) = parse_quantity_value(limit_quantity) else {
                continue;
            };
            if request > limit {
                errors.push(invalid_resource_quantity(
                    format!("{path}.requests"),
                    quantity,
                    format!(
                        "must be less than or equal to {name} limit of {limit_quantity}"
                    ),
                ));
            }
        }
    }

    errors
}

pub fn validate_game_server_resources(
    spec: &GameServerSpec,
) -> Result<Vec<ValidationError>, AdapterError> {
    let pod_spec: PodSpec = to_native(&spec.template.spec)?;
    let mut errors = Vec::new();

    if let Some(init_containers) = &pod_spec.init_containers {
        for (index, container) in init_containers.iter().enumerate() {
            if let Some(resources) = &container.resources {
                errors.extend(validate_resource_requirements(
                    resources,
                    &format!("spec.template.spec.initContainers[{index}].resources"),
                ));
            }
        }
    }

    for (index, container) in pod_spec.containers.iter().enumerate() {
        if let Some(resources) = &container.resources {
            errors.extend(validate_resource_requirements(
                resources,
                &format!("spec.template.spec.containers[{index}].resources"),
            ));
        }
    }

    Ok(errors)
}

pub fn validate_game_server(
    game_server: &GameServer,
) -> Result<Vec<ValidationError>, AdapterError> {
    let mut errors = game_server.validate();
    errors.extend(validate_game_server_resources(&game_server.spec)?);
    Ok(errors)
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
    .with_controller(true)
    .with_block_owner_deletion(true);
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
    #[test]
    fn game_server_crd_envelope_matches_agones_helm_static_fields() {
        let crd = game_server_crd_envelope();
        assert_eq!(crd.api_version, "apiextensions.k8s.io/v1");
        assert_eq!(crd.kind, "CustomResourceDefinition");
        assert_eq!(crd.metadata.name, GAME_SERVER_CRD_NAME);
        assert_eq!(crd.spec.group, GAME_SERVER_CRD_GROUP);
        assert_eq!(crd.spec.scope, ResourceScope::Namespaced);
        assert_eq!(crd.spec.names.kind, GAME_SERVER_CRD_KIND);
        assert_eq!(crd.spec.names.plural, GAME_SERVER_CRD_PLURAL);
        assert_eq!(
            crd.spec.names.singular.as_deref(),
            Some(GAME_SERVER_CRD_SINGULAR)
        );
        assert_eq!(
            crd.spec.names.short_names.as_deref(),
            Some(&[GAME_SERVER_CRD_SHORT_NAME.to_owned()][..])
        );
        assert!(crd.spec.names.list_kind.is_none());

        let version = &crd.spec.versions[0];
        assert_eq!(version.name, GAME_SERVER_CRD_VERSION);
        assert!(version.served);
        assert!(version.storage);
        assert!(version.schema.is_some());

        let scale = version
            .subresources
            .as_ref()
            .and_then(|subresources| subresources.scale.as_ref())
            .unwrap();
        assert_eq!(scale.spec_replicas_path, ".spec.immutableReplicas");
        assert_eq!(scale.status_replicas_path, ".status.immutableReplicas");
    }

    #[test]
    fn game_server_crd_printer_columns_match_agones_helm_order_and_types() {
        let crd = game_server_crd_envelope();
        let columns = crd.spec.versions[0]
            .additional_printer_columns
            .as_ref()
            .unwrap();

        let actual: Vec<_> = columns
            .iter()
            .map(|column| {
                (
                    column.name.as_str(),
                    column.type_.as_str(),
                    column.json_path.as_str(),
                )
            })
            .collect();
        assert_eq!(
            actual,
            vec![
                ("State", "string", ".status.state"),
                ("Address", "string", ".status.address"),
                ("Port", "string", ".status.ports[0].port"),
                ("Node", "string", ".status.nodeName"),
                ("Age", "date", ".metadata.creationTimestamp"),
            ]
        );
    }

    fn normalize_json_numbers(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Array(values) => {
                for value in values {
                    normalize_json_numbers(value);
                }
            }
            serde_json::Value::Object(values) => {
                for value in values.values_mut() {
                    normalize_json_numbers(value);
                }
            }
            serde_json::Value::Number(number) => {
                if let Some(value) = number.as_f64().and_then(serde_json::Number::from_f64) {
                    *number = value;
                }
            }
            _ => {}
        }
    }

    #[test]
    fn game_server_crd_schema_is_generated_from_helm_machine_sources() {
        let crd = game_server_crd_envelope();
        let schema = crd.spec.versions[0].schema.as_ref().unwrap();
        let mut represented = serde_json::to_value(&schema.open_apiv3_schema).unwrap();
        let mut generated: serde_json::Value = serde_json::from_str(
            generated_game_server_schema::GAME_SERVER_OPENAPI_SCHEMA_JSON,
        )
        .unwrap();

        // Rusternetes models OpenAPI minimum/maximum as f64, so YAML integer
        // bounds round-trip as 0.0/1.0. Canonicalize only number encoding; all
        // schema structure and nonnumeric values must remain exactly identical.
        normalize_json_numbers(&mut represented);
        normalize_json_numbers(&mut generated);
        assert_eq!(
            represented, generated,
            "Rusternetes must not silently drop fields from the generated schema"
        );
        assert_eq!(generated["type"], "object");
        assert_eq!(generated["required"], serde_json::json!(["spec"]));
        assert!(generated["properties"]["spec"].is_object());
        assert!(generated["properties"]["status"].is_object());
        assert!(generated["properties"]["spec"]["properties"]["template"]["properties"]["spec"]
            ["properties"]["containers"]
            .is_object());
    }

    #[test]
    fn game_server_crd_schema_uses_default_list_limits_and_kubernetes_list_semantics() {
        let generated: serde_json::Value = serde_json::from_str(
            generated_game_server_schema::GAME_SERVER_OPENAPI_SCHEMA_JSON,
        )
        .unwrap();

        let spec_values =
            &generated["properties"]["spec"]["properties"]["lists"]["additionalProperties"]
                ["properties"]["values"];
        let status_values =
            &generated["properties"]["status"]["properties"]["lists"]["additionalProperties"]
                ["properties"]["values"];

        assert_eq!(spec_values["maxItems"], 1000);
        assert_eq!(status_values["maxItems"], 1000);
        assert_eq!(spec_values["x-kubernetes-list-type"], "set");
        assert_eq!(status_values["x-kubernetes-list-type"], "set");
        assert!(generated["properties"]["spec"]["properties"]["template"]
            .get("x-kubernetes-preserve-unknown-fields")
            .is_none());
    }

    #[test]
    fn game_server_crd_records_unrepresentable_rusternetes_selectable_fields() {
        assert_eq!(
            GAME_SERVER_CRD_SELECTABLE_FIELDS,
            &[".status.state", ".status.nodeName"]
        );

        let json = serde_json::to_value(game_server_crd_envelope()).unwrap();
        assert!(
            json["spec"]["versions"][0].get("selectableFields").is_none(),
            "pinned Rusternetes CRD version type has no selectableFields member"
        );
    }

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

    fn core_container_with_resources(
        name: &str,
        requests: serde_json::Value,
        limits: serde_json::Value,
    ) -> CoreContainer {
        let mut container = CoreContainer {
            name: name.into(),
            image: "testing/image".into(),
            ..Default::default()
        };
        container.extensions.insert(
            "resources".into(),
            serde_json::json!({
                "requests": requests,
                "limits": limits
            }),
        );
        container
    }

    #[test]
    fn resource_validation_matches_cpu_request_limit_invariants() {
        let mut gs = fixture();
        gs.spec.template.spec.containers = vec![core_container_with_resources(
            "game",
            serde_json::json!({"cpu": "50m", "memory": "32Mi"}),
            serde_json::json!({"cpu": "30m", "memory": "32Mi"}),
        )];
        let errors = validate_game_server_resources(&gs.spec).unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].path,
            "spec.template.spec.containers[0].resources.requests"
        );
        assert_eq!(errors[0].bad_value.as_deref(), Some("50m"));
        assert_eq!(
            errors[0].detail,
            "must be less than or equal to cpu limit of 30m"
        );

        gs.spec.template.spec.containers = vec![core_container_with_resources(
            "game",
            serde_json::json!({"cpu": "-30m", "memory": "32Mi"}),
            serde_json::json!({"cpu": "30m", "memory": "32Mi"}),
        )];
        let errors = validate_game_server_resources(&gs.spec).unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].path,
            "spec.template.spec.containers[0].resources.requests[cpu]"
        );
        assert_eq!(errors[0].bad_value.as_deref(), Some("-30m"));
        assert_eq!(errors[0].detail, "must be greater than or equal to 0");
    }

    #[test]
    fn resource_validation_reports_negative_limit_before_request_comparison() {
        let mut gs = fixture();
        gs.spec.template.spec.containers = vec![core_container_with_resources(
            "game",
            serde_json::json!({"cpu": "30m", "memory": "32Mi"}),
            serde_json::json!({"cpu": "-30m", "memory": "32Mi"}),
        )];
        let errors = validate_game_server_resources(&gs.spec).unwrap();
        assert_eq!(errors.len(), 2);
        assert_eq!(
            errors[0].path,
            "spec.template.spec.containers[0].resources.limits[cpu]"
        );
        assert_eq!(errors[0].bad_value.as_deref(), Some("-30m"));
        assert_eq!(errors[0].detail, "must be greater than or equal to 0");
        assert_eq!(
            errors[1].path,
            "spec.template.spec.containers[0].resources.requests"
        );
        assert_eq!(errors[1].bad_value.as_deref(), Some("30m"));
        assert_eq!(
            errors[1].detail,
            "must be less than or equal to cpu limit of -30m"
        );
    }

    #[test]
    fn resource_validation_matches_memory_invariants() {
        let mut gs = fixture();
        gs.spec.template.spec.containers = vec![core_container_with_resources(
            "game",
            serde_json::json!({"cpu": "30m", "memory": "55Mi"}),
            serde_json::json!({"cpu": "30m", "memory": "32Mi"}),
        )];
        let errors = validate_game_server_resources(&gs.spec).unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].path,
            "spec.template.spec.containers[0].resources.requests"
        );
        assert_eq!(errors[0].bad_value.as_deref(), Some("55Mi"));
        assert_eq!(
            errors[0].detail,
            "must be less than or equal to memory limit of 32Mi"
        );

        gs.spec.template.spec.containers = vec![core_container_with_resources(
            "game",
            serde_json::json!({"cpu": "30m", "memory": "-32Mi"}),
            serde_json::json!({"cpu": "30m", "memory": "32Mi"}),
        )];
        let errors = validate_game_server_resources(&gs.spec).unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].path,
            "spec.template.spec.containers[0].resources.requests[memory]"
        );
        assert_eq!(errors[0].bad_value.as_deref(), Some("-32Mi"));
        assert_eq!(errors[0].detail, "must be greater than or equal to 0");
    }

    #[test]
    fn resource_validation_covers_restartable_and_regular_init_containers() {
        let mut gs = fixture();
        gs.spec.template.spec.init_containers = vec![core_container_with_resources(
            "sidecar",
            serde_json::json!({"cpu": "50m", "memory": "32Mi"}),
            serde_json::json!({"cpu": "30m", "memory": "32Mi"}),
        )];
        let errors = validate_game_server_resources(&gs.spec).unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].path,
            "spec.template.spec.initContainers[0].resources.requests"
        );
        assert_eq!(errors[0].bad_value.as_deref(), Some("50m"));
        assert_eq!(
            errors[0].detail,
            "must be less than or equal to cpu limit of 30m"
        );
    }

    #[test]
    fn quantity_parser_covers_kubernetes_binary_decimal_and_exponent_forms() {
        assert_eq!(parse_quantity_value("100m"), Some(0.1));
        assert_eq!(parse_quantity_value("1Ki"), Some(1024.0));
        assert_eq!(parse_quantity_value("0.5Gi"), Some(536_870_912.0));
        assert_eq!(parse_quantity_value("1e3"), Some(1000.0));
        assert_eq!(parse_quantity_value("-32Mi"), Some(-33_554_432.0));
        assert_eq!(parse_quantity_value("NaN"), None);
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
