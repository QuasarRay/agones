use std::collections::HashMap;

use cgp::prelude::async_trait;
use agones_core::{
    GameServer, GameServerSpec, ValidationError,
};
use rusternetes_common::admission::{
    AdmissionController, AdmissionRequest, AdmissionResponse, Operation,
    PatchOp, PatchOperation,
};
use rusternetes_common::resources::Pod;

use crate::{
    AdapterError, ApiHooks, PASSTHROUGH_PORT_ASSIGNMENT_ANNOTATION,
    validate_game_server_resources,
};

fn patch_add(path: String, value: serde_json::Value) -> PatchOperation {
    PatchOperation {
        op: PatchOp::Add,
        path,
        value: Some(value),
        from: None,
    }
}

fn patch_remove(path: String) -> PatchOperation {
    PatchOperation {
        op: PatchOp::Remove,
        path,
        value: None,
        from: None,
    }
}

fn patch_replace(path: String, value: serde_json::Value) -> PatchOperation {
    PatchOperation {
        op: PatchOp::Replace,
        path,
        value: Some(value),
        from: None,
    }
}

fn escape_json_pointer(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

fn child_path(path: &str, segment: &str) -> String {
    let segment = escape_json_pointer(segment);
    if path.is_empty() {
        format!("/{segment}")
    } else {
        format!("{path}/{segment}")
    }
}

/// Deterministic RFC6902 diff sufficient for Agones admission mutation.
///
/// Objects are traversed by map order and equal-length arrays are traversed
/// element-by-element so passthrough-port mutation produces the same leaf
/// replacement paths as the upstream jsonpatch library.
pub fn admission_json_patch(
    old: &serde_json::Value,
    new: &serde_json::Value,
) -> Vec<PatchOperation> {
    fn diff(
        path: &str,
        old: &serde_json::Value,
        new: &serde_json::Value,
        out: &mut Vec<PatchOperation>,
    ) {
        if old == new {
            return;
        }

        match (old, new) {
            (
                serde_json::Value::Object(old_map),
                serde_json::Value::Object(new_map),
            ) => {
                let mut removed: Vec<_> = old_map
                    .keys()
                    .filter(|key| !new_map.contains_key(*key))
                    .cloned()
                    .collect();
                removed.sort();
                for key in removed {
                    out.push(patch_remove(child_path(path, &key)));
                }

                let mut keys: Vec<_> = new_map.keys().cloned().collect();
                keys.sort();
                for key in keys {
                    let next = child_path(path, &key);
                    match old_map.get(&key) {
                        Some(old_value) => {
                            diff(&next, old_value, &new_map[&key], out)
                        }
                        None => out.push(patch_add(
                            next,
                            new_map[&key].clone(),
                        )),
                    }
                }
            }
            (
                serde_json::Value::Array(old_items),
                serde_json::Value::Array(new_items),
            ) if old_items.len() == new_items.len() => {
                for (index, (old_item, new_item)) in
                    old_items.iter().zip(new_items).enumerate()
                {
                    diff(
                        &child_path(path, &index.to_string()),
                        old_item,
                        new_item,
                        out,
                    );
                }
            }
            _ => out.push(patch_replace(path.to_owned(), new.clone())),
        }
    }

    let mut patches = Vec::new();
    diff("", old, new, &mut patches);
    patches
}

/// Upstream creationMutationHandler domain behavior.
///
/// A JSON value that cannot deserialize as a GameServer is deliberately
/// accepted without mutation so schema validation can produce the user-facing
/// error, matching the Go webhook's nil-error fallthrough behavior.
pub fn mutate_game_server_creation(
    object: &serde_json::Value,
) -> Result<AdmissionResponse, AdapterError> {
    let Ok(mut game_server) =
        serde_json::from_value::<GameServer>(object.clone())
    else {
        return Ok(AdmissionResponse::Allow);
    };

    game_server.apply_defaults();
    let mutated = serde_json::to_value(game_server)?;
    let patches = admission_json_patch(object, &mutated);

    if patches.is_empty() {
        Ok(AdmissionResponse::Allow)
    } else {
        Ok(AdmissionResponse::AllowWithPatch(patches))
    }
}

/// Upstream GameServer validation including runtime-independent checks,
/// Kubernetes resource quantities, and API hook validation.
///
/// The Go implementation returns early for development GameServers, before
/// resource/API-hook validation. Preserve that detail here.
pub fn validate_game_server_creation<H>(
    game_server: &GameServer,
    hooks: &H,
) -> Result<Vec<ValidationError>, AdapterError>
where
    H: ApiHooks,
{
    let mut errors = game_server.validate();

    if game_server
        .get_dev_address()
        .is_none_or(str::is_empty)
    {
        errors.extend(validate_game_server_resources(
            &game_server.spec,
        )?);
        errors.extend(hooks.validate_game_server_spec(
            &game_server.spec,
        ));
    }

    Ok(errors)
}

fn format_validation_errors(errors: &[ValidationError]) -> String {
    errors
        .iter()
        .map(|error| {
            let value = error
                .bad_value
                .as_deref()
                .map(|value| format!(": {value}"))
                .unwrap_or_default();
            format!(
                "{}{:?}{}: {}",
                error.path, error.kind, value, error.detail
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// Upstream creationValidationHandler domain behavior.
///
/// Unlike mutation, malformed GameServer JSON after schema validation is an
/// error, not an allow-without-change response.
pub fn validate_game_server_creation_object<H>(
    object: &serde_json::Value,
    hooks: &H,
) -> Result<AdmissionResponse, AdapterError>
where
    H: ApiHooks,
{
    let game_server: GameServer =
        serde_json::from_value(object.clone()).map_err(|error| {
            AdapterError::Hook(format!(
                "error unmarshalling GameServer json after schema validation: {object}: {error}"
            ))
        })?;

    let errors = validate_game_server_creation(&game_server, hooks)?;
    if errors.is_empty() {
        Ok(AdmissionResponse::Allow)
    } else {
        Ok(AdmissionResponse::Deny(format_validation_errors(
            &errors,
        )))
    }
}

fn mutate_passthrough_ports(
    pod: &mut Pod,
) -> Result<bool, AdapterError> {
    let Some(annotation) = pod
        .metadata
        .annotations
        .as_ref()
        .and_then(|annotations| {
            annotations.get(PASSTHROUGH_PORT_ASSIGNMENT_ANNOTATION)
        })
    else {
        return Ok(false);
    };

    let assignments: HashMap<String, Vec<usize>> =
        serde_json::from_str(annotation).map_err(|error| {
            AdapterError::Hook(format!(
                "could not unmarshal annotation {annotation:?}: {error}"
            ))
        })?;

    let mut changed = false;
    if let Some(spec) = pod.spec.as_mut() {
        if let Some(init_containers) = spec.init_containers.as_mut() {
            for container in init_containers {
                if container.restart_policy.as_deref() != Some("Always") {
                    continue;
                }
                if let Some(indices) = assignments.get(&container.name) {
                    for &index in indices {
                        let port = container
                            .ports
                            .as_mut()
                            .and_then(|ports| ports.get_mut(index))
                            .ok_or_else(|| {
                                AdapterError::Hook(format!(
                                    "passthrough port index {index} is invalid for container {}",
                                    container.name
                                ))
                            })?;
                        if let Some(host_port) = port.host_port {
                            if port.container_port != host_port {
                                port.container_port = host_port;
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        for container in &mut spec.containers {
            if let Some(indices) = assignments.get(&container.name) {
                for &index in indices {
                    let port = container
                        .ports
                        .as_mut()
                        .and_then(|ports| ports.get_mut(index))
                        .ok_or_else(|| {
                            AdapterError::Hook(format!(
                                "passthrough port index {index} is invalid for container {}",
                                container.name
                            ))
                        })?;
                    if let Some(host_port) = port.host_port {
                        if port.container_port != host_port {
                            port.container_port = host_port;
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    Ok(changed)
}

/// Upstream creationMutationHandlerPod domain behavior.
pub fn mutate_game_server_pod_creation(
    object: &serde_json::Value,
) -> Result<AdmissionResponse, AdapterError> {
    let Ok(mut pod) = serde_json::from_value::<Pod>(object.clone()) else {
        return Ok(AdmissionResponse::Allow);
    };

    if !mutate_passthrough_ports(&mut pod)? {
        return Ok(AdmissionResponse::Allow);
    }

    let mutated = serde_json::to_value(pod)?;
    let patches = admission_json_patch(object, &mutated);
    if patches.is_empty() {
        Ok(AdmissionResponse::Allow)
    } else {
        Ok(AdmissionResponse::AllowWithPatch(patches))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GameServerCreationMutationAdmission;

#[async_trait]
impl AdmissionController for GameServerCreationMutationAdmission {
    fn name(&self) -> &str {
        "AgonesGameServerCreationMutation"
    }

    fn supports_operation(&self, operation: &Operation) -> bool {
        matches!(operation, Operation::Create)
    }

    async fn admit(&self, request: &AdmissionRequest) -> AdmissionResponse {
        if request.kind != "GameServer" {
            return AdmissionResponse::Allow;
        }
        mutate_game_server_creation(&request.object).unwrap_or_else(
            |error| AdmissionResponse::Deny(error.to_string()),
        )
    }
}

#[derive(Clone, Debug)]
pub struct GameServerCreationValidationAdmission<H> {
    hooks: H,
}

impl<H> GameServerCreationValidationAdmission<H> {
    pub fn new(hooks: H) -> Self {
        Self { hooks }
    }
}

#[async_trait]
impl<H> AdmissionController for GameServerCreationValidationAdmission<H>
where
    H: ApiHooks + Send + Sync,
{
    fn name(&self) -> &str {
        "AgonesGameServerCreationValidation"
    }

    fn supports_operation(&self, operation: &Operation) -> bool {
        matches!(operation, Operation::Create)
    }

    async fn admit(&self, request: &AdmissionRequest) -> AdmissionResponse {
        if request.kind != "GameServer" {
            return AdmissionResponse::Allow;
        }
        validate_game_server_creation_object(
            &request.object,
            &self.hooks,
        )
        .unwrap_or_else(|error| {
            AdmissionResponse::Deny(error.to_string())
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GameServerPodCreationMutationAdmission;

#[async_trait]
impl AdmissionController for GameServerPodCreationMutationAdmission {
    fn name(&self) -> &str {
        "AgonesGameServerPodCreationMutation"
    }

    fn supports_operation(&self, operation: &Operation) -> bool {
        matches!(operation, Operation::Create)
    }

    async fn admit(&self, request: &AdmissionRequest) -> AdmissionResponse {
        if request.kind != "Pod" {
            return AdmissionResponse::Allow;
        }
        mutate_game_server_pod_creation(&request.object).unwrap_or_else(
            |error| AdmissionResponse::Deny(error.to_string()),
        )
    }
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerPort, GameServerSpec,
        ObjectMeta, PodSpec as CorePodSpec, PodTemplateSpec,
    };
    use rusternetes_common::admission::PatchOp;

    use crate::{
        NoopApiHooks, PodBuildFeatures, build_pod,
    };

    use super::*;

    fn game_server_fixture() -> GameServer {
        GameServer {
            metadata: ObjectMeta {
                name: "test".into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSpec {
                ports: vec![GameServerPort {
                    container_port: 7777,
                    ..Default::default()
                }],
                template: PodTemplateSpec {
                    spec: CorePodSpec {
                        containers: vec![CoreContainer {
                            name: "container".into(),
                            image: "container/image".into(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn has_patch(
        response: &AdmissionResponse,
        op: PatchOp,
        path: &str,
    ) -> bool {
        let AdmissionResponse::AllowWithPatch(patches) = response else {
            return false;
        };
        patches.iter().any(|patch| {
            patch.op == op && patch.path == path
        })
    }

    #[test]
    fn creation_mutation_matches_upstream_default_patch_behavior() {
        let fixture = serde_json::to_value(game_server_fixture()).unwrap();
        let response = mutate_game_server_creation(&fixture).unwrap();

        assert!(response.is_allowed());
        assert!(has_patch(
            &response,
            PatchOp::Add,
            "/metadata/finalizers",
        ));
        assert!(has_patch(
            &response,
            PatchOp::Add,
            "/spec/ports/0/protocol",
        ));
    }

    #[test]
    fn creation_mutation_falls_through_on_wrong_object_shape() {
        let response =
            mutate_game_server_creation(&serde_json::json!("WRONG DATA"))
                .unwrap();
        assert!(matches!(response, AdmissionResponse::Allow));
    }

    #[test]
    fn creation_validation_accepts_valid_and_denies_invalid_game_server() {
        let mut valid = game_server_fixture();
        valid.apply_defaults();
        assert!(matches!(
            validate_game_server_creation_object(
                &serde_json::to_value(valid).unwrap(),
                &NoopApiHooks,
            )
            .unwrap(),
            AdmissionResponse::Allow
        ));

        let mut invalid = game_server_fixture();
        invalid.spec.container = "NOPE!".into();
        invalid.spec.template.spec.containers.push(
            CoreContainer {
                name: "container2".into(),
                image: "container/image".into(),
                ..Default::default()
            },
        );
        invalid.spec.apply_defaults();

        let response = validate_game_server_creation_object(
            &serde_json::to_value(invalid).unwrap(),
            &NoopApiHooks,
        )
        .unwrap();
        let AdmissionResponse::Deny(reason) = response else {
            panic!("invalid GameServer should be denied");
        };
        assert!(reason.contains("spec.container"));
    }

    #[test]
    fn creation_validation_reports_wrong_object_shape_as_error() {
        let error = validate_game_server_creation_object(
            &serde_json::json!("WRONG DATA"),
            &NoopApiHooks,
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("error unmarshalling GameServer json after schema validation")
        );
    }

    #[test]
    fn pod_mutation_replaces_passthrough_container_ports_at_leaf_paths() {
        let mut game_server = game_server_fixture();
        game_server.spec.apply_defaults();
        game_server.spec.ports = vec![
            GameServerPort {
                container: Some("container".into()),
                container_port: 0,
                host_port: 7000,
                port_policy: Some(agones_core::PortPolicy::Passthrough),
                ..Default::default()
            },
            GameServerPort {
                container: Some("container".into()),
                container_port: 0,
                host_port: 7001,
                port_policy: Some(agones_core::PortPolicy::Passthrough),
                ..Default::default()
            },
        ];

        let pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        let object = serde_json::to_value(pod).unwrap();
        let response =
            mutate_game_server_pod_creation(&object).unwrap();

        assert!(has_patch(
            &response,
            PatchOp::Replace,
            "/spec/containers/0/ports/0/containerPort",
        ));
        assert!(has_patch(
            &response,
            PatchOp::Replace,
            "/spec/containers/0/ports/1/containerPort",
        ));
    }

    #[test]
    fn pod_mutation_ignores_pods_without_passthrough_annotation() {
        let mut game_server = game_server_fixture();
        game_server.apply_defaults();
        let mut object = serde_json::to_value(
            build_pod(
                &game_server,
                Vec::new(),
                PodBuildFeatures::default(),
                &NoopApiHooks,
            )
            .unwrap(),
        )
        .unwrap();
        object["metadata"]["annotations"]
            .as_object_mut()
            .unwrap()
            .remove(PASSTHROUGH_PORT_ASSIGNMENT_ANNOTATION);

        assert!(matches!(
            mutate_game_server_pod_creation(&object).unwrap(),
            AdmissionResponse::Allow
        ));
    }
}
