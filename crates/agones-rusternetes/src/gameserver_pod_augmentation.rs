use agones_core::{GameServer, GameServerSpec, SdkServer};
use agones_macros::agones_invariant;
use rusternetes_common::resources::{
    Container, EmptyDirVolumeSource, EnvVar, HTTPGetAction, Pod, Probe,
    Volume, VolumeMount,
};
use rusternetes_common::resources::policy::IntOrString;

use crate::{AdapterError, apply_to_container};

pub const SDK_SERVER_SIDECAR_NAME: &str = "agones-gameserver-sidecar";
pub const GRPC_PORT_ENV_VAR: &str = "AGONES_SDK_GRPC_PORT";
pub const HTTP_PORT_ENV_VAR: &str = "AGONES_SDK_HTTP_PORT";
pub const SERVICE_ACCOUNT_VOLUME_NAME: &str = "empty";
pub const SERVICE_ACCOUNT_MOUNT_PATH: &str =
    "/var/run/secrets/kubernetes.io/serviceaccount";

pub fn reserved_environment_variable_name(name: &str) -> bool {
    matches!(name, GRPC_PORT_ENV_VAR | HTTP_PORT_ENV_VAR)
}

pub fn sdk_environment_variables(game_server: &GameServer) -> Vec<EnvVar> {
    let mut env = Vec::with_capacity(2);

    if game_server.spec.sdk_server.grpc_port != 0 {
        env.push(EnvVar {
            name: GRPC_PORT_ENV_VAR.to_owned(),
            value: Some(game_server.spec.sdk_server.grpc_port.to_string()),
            value_from: None,
        });
    }

    if game_server.spec.sdk_server.http_port != 0 {
        env.push(EnvVar {
            name: HTTP_PORT_ENV_VAR.to_owned(),
            value: Some(game_server.spec.sdk_server.http_port.to_string()),
            value_from: None,
        });
    }

    env
}

#[agones_invariant]
fn sdk_environment_variable_shape_matches_ports(input: (i16, i16)) -> bool {
    let (grpc_port, http_port) = input;
    let game_server = GameServer {
        spec: GameServerSpec {
            sdk_server: SdkServer {
                grpc_port: grpc_port as i32,
                http_port: http_port as i32,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };
    let env = sdk_environment_variables(&game_server);

    let grpc: Vec<_> = env
        .iter()
        .filter(|item| item.name == GRPC_PORT_ENV_VAR)
        .collect();
    let http: Vec<_> = env
        .iter()
        .filter(|item| item.name == HTTP_PORT_ENV_VAR)
        .collect();

    let grpc_value = grpc_port.to_string();
    let http_value = http_port.to_string();

    grpc.len() == usize::from(grpc_port != 0)
        && http.len() == usize::from(http_port != 0)
        && grpc.first().is_none_or(|item| {
            item.value.as_deref() == Some(grpc_value.as_str())
        })
        && http.first().is_none_or(|item| {
            item.value.as_deref() == Some(http_value.as_str())
        })
}

fn add_env_vars_to_container(
    container: &mut Container,
    sdk_env_vars: &[EnvVar],
) {
    let env = container.env.get_or_insert_with(Vec::new);
    env.retain(|item| !reserved_environment_variable_name(&item.name));
    env.extend(sdk_env_vars.iter().cloned());
}

/// Mirrors upstream addSDKServerEnvVars.
///
/// Legacy GameServers with both SDK ports unset are left byte-for-byte
/// untouched. Reserved SDK environment variables are replaced, not duplicated,
/// and the SDK sidecar itself is excluded in both regular and restartable-init
/// layouts.
pub fn add_sdk_server_env_vars(
    game_server: &GameServer,
    pod: &mut Pod,
) -> Result<(), AdapterError> {
    let sdk_env_vars = sdk_environment_variables(game_server);
    if sdk_env_vars.is_empty() {
        return Ok(());
    }

    let spec = pod
        .spec
        .as_mut()
        .ok_or(AdapterError::MissingField("pod.spec"))?;

    if let Some(init_containers) = spec.init_containers.as_mut() {
        for container in init_containers {
            if container.name != SDK_SERVER_SIDECAR_NAME {
                add_env_vars_to_container(container, &sdk_env_vars);
            }
        }
    }

    for container in &mut spec.containers {
        if container.name != SDK_SERVER_SIDECAR_NAME {
            add_env_vars_to_container(container, &sdk_env_vars);
        }
    }

    Ok(())
}

/// Mirrors upstream addGameServerHealthCheck.
///
/// Existing user liveness probes are preserved. For an enabled Agones health
/// policy with no user probe, the game server container receives the sidecar's
/// /gshealthz probe using the GameServer health timing and failure threshold.
pub fn add_game_server_health_check(
    game_server: &GameServer,
    pod: &mut Pod,
) -> Result<(), AdapterError> {
    if game_server.spec.health.disabled {
        return Ok(());
    }

    let spec = pod
        .spec
        .as_mut()
        .ok_or(AdapterError::MissingField("pod.spec"))?;
    let health = &game_server.spec.health;

    apply_to_container(
        spec,
        game_server.spec.container.as_str(),
        |container| {
            if container.liveness_probe.is_none() {
                container.liveness_probe = Some(Probe {
                    http_get: Some(HTTPGetAction {
                        path: Some("/gshealthz".to_owned()),
                        port: IntOrString::Int(8080),
                        host: None,
                        scheme: None,
                        http_headers: None,
                    }),
                    tcp_socket: None,
                    exec: None,
                    initial_delay_seconds: Some(health.initial_delay_seconds),
                    timeout_seconds: None,
                    period_seconds: Some(health.period_seconds),
                    success_threshold: None,
                    failure_threshold: Some(health.failure_threshold),
                    grpc: None,
                    termination_grace_period_seconds: None,
                });
            }
        },
    )
}

/// Mirrors GameServer.DisableServiceAccount on Rusternetes-native Pod types.
///
/// Agones gives its SDK sidecar a service account while masking that token from
/// the game-server container itself with a read-only EmptyDir mount.
pub fn disable_game_server_service_account(
    game_server: &GameServer,
    pod: &mut Pod,
) -> Result<(), AdapterError> {
    let spec = pod
        .spec
        .as_mut()
        .ok_or(AdapterError::MissingField("pod.spec"))?;

    let empty_volume = Volume {
        name: SERVICE_ACCOUNT_VOLUME_NAME.to_owned(),
        empty_dir: Some(EmptyDirVolumeSource {
            medium: None,
            size_limit: None,
        }),
        host_path: None,
        config_map: None,
        secret: None,
        persistent_volume_claim: None,
        downward_api: None,
        csi: None,
        ephemeral: None,
        nfs: None,
        iscsi: None,
        projected: None,
        image: None,
    };
    spec.volumes
        .get_or_insert_with(Vec::new)
        .push(empty_volume);

    let mount = VolumeMount {
        name: SERVICE_ACCOUNT_VOLUME_NAME.to_owned(),
        mount_path: SERVICE_ACCOUNT_MOUNT_PATH.to_owned(),
        read_only: Some(true),
        sub_path: None,
        sub_path_expr: None,
        mount_propagation: None,
        recursive_read_only: None,
    };

    apply_to_container(
        spec,
        game_server.spec.container.as_str(),
        |container| {
            container
                .volume_mounts
                .get_or_insert_with(Vec::new)
                .push(mount.clone());
        },
    )
}

/// Applies upstream createGameServerPod service-account policy.
///
/// If the user supplied serviceAccountName, Agones assumes the user owns the
/// access policy and leaves the game-server container unchanged. Otherwise the
/// controller assigns its SDK service account and masks the token from only the
/// game-server container.
pub fn configure_game_server_service_account(
    game_server: &GameServer,
    pod: &mut Pod,
    sdk_service_account: &str,
) -> Result<(), AdapterError> {
    let has_user_service_account = pod
        .spec
        .as_ref()
        .and_then(|spec| spec.service_account_name.as_deref())
        .is_some_and(|name| !name.is_empty());
    if has_user_service_account {
        return Ok(());
    }

    let spec = pod
        .spec
        .as_mut()
        .ok_or(AdapterError::MissingField("pod.spec"))?;
    spec.service_account_name = Some(sdk_service_account.to_owned());

    disable_game_server_service_account(game_server, pod)
}

#[agones_invariant]
fn reserved_service_account_mount_is_constant(_: u8) -> bool {
    SERVICE_ACCOUNT_VOLUME_NAME == "empty"
        && SERVICE_ACCOUNT_MOUNT_PATH
            == "/var/run/secrets/kubernetes.io/serviceaccount"
}

/// Applies the controller-owned Pod mutations performed immediately before the
/// Pod CREATE in upstream createGameServerPod.
pub fn augment_game_server_pod(
    game_server: &GameServer,
    pod: &mut Pod,
) -> Result<(), AdapterError> {
    add_game_server_health_check(game_server, pod)?;
    add_sdk_server_env_vars(game_server, pod)
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerPort, ObjectMeta,
        PodSpec as CorePodSpec, PodTemplateSpec, PortPolicy,
    };

    use crate::{NoopApiHooks, PodBuildFeatures, build_pod};

    use super::*;

    fn fixture() -> GameServer {
        let mut game_server = GameServer {
            metadata: ObjectMeta {
                name: "gameserver".into(),
                namespace: "default".into(),
                uid: "1234".into(),
                ..Default::default()
            },
            spec: GameServerSpec {
                ports: vec![GameServerPort {
                    container_port: 7777,
                    host_port: 9999,
                    port_policy: Some(PortPolicy::Static),
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
        };
        game_server.apply_defaults();
        game_server
    }

    fn env_value<'a>(container: &'a Container, name: &str) -> Option<&'a str> {
        container
            .env
            .as_ref()?
            .iter()
            .find(|item| item.name == name)?
            .value
            .as_deref()
    }

    #[test]
    fn default_service_account_is_assigned_and_masked_from_game_container() {
        let game_server = fixture();
        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();

        configure_game_server_service_account(
            &game_server,
            &mut pod,
            "agones-sdk",
        )
        .unwrap();

        let spec = pod.spec.as_ref().unwrap();
        assert_eq!(
            spec.service_account_name.as_deref(),
            Some("agones-sdk")
        );
        assert!(spec.volumes.as_ref().unwrap().iter().any(|volume| {
            volume.name == SERVICE_ACCOUNT_VOLUME_NAME
                && volume.empty_dir.is_some()
        }));
        let game = spec
            .containers
            .iter()
            .find(|container| container.name == game_server.spec.container)
            .unwrap();
        let mount = game
            .volume_mounts
            .as_ref()
            .unwrap()
            .iter()
            .find(|mount| mount.name == SERVICE_ACCOUNT_VOLUME_NAME)
            .unwrap();
        assert_eq!(mount.mount_path, SERVICE_ACCOUNT_MOUNT_PATH);
        assert_eq!(mount.read_only, Some(true));
    }

    #[test]
    fn user_service_account_disables_opinionated_token_masking() {
        let game_server = fixture();
        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        pod.spec.as_mut().unwrap().service_account_name =
            Some("foobar".into());
        let before = serde_json::to_value(&pod).unwrap();

        configure_game_server_service_account(
            &game_server,
            &mut pod,
            "agones-sdk",
        )
        .unwrap();

        assert_eq!(serde_json::to_value(&pod).unwrap(), before);
    }

    #[test]
    fn health_check_matches_upstream_probe_fields() {
        let game_server = fixture();
        assert!(!game_server.spec.health.disabled);

        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();

        add_game_server_health_check(&game_server, &mut pod).unwrap();

        let probe = pod.spec.as_ref().unwrap().containers[0]
            .liveness_probe
            .as_ref()
            .unwrap();
        let http = probe.http_get.as_ref().unwrap();

        assert_eq!(http.path.as_deref(), Some("/gshealthz"));
        assert!(matches!(http.port, IntOrString::Int(8080)));
        assert_eq!(
            probe.failure_threshold,
            Some(game_server.spec.health.failure_threshold)
        );
        assert_eq!(
            probe.initial_delay_seconds,
            Some(game_server.spec.health.initial_delay_seconds)
        );
        assert_eq!(
            probe.period_seconds,
            Some(game_server.spec.health.period_seconds)
        );
    }

    #[test]
    fn health_check_preserves_existing_user_probe() {
        let game_server = fixture();
        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();

        let existing: Probe = serde_json::from_value(serde_json::json!({
            "httpGet": {"path": "/custom", "port": 9000},
            "periodSeconds": 17
        }))
        .unwrap();
        pod.spec.as_mut().unwrap().containers[0].liveness_probe =
            Some(existing);

        add_game_server_health_check(&game_server, &mut pod).unwrap();

        let probe = pod.spec.as_ref().unwrap().containers[0]
            .liveness_probe
            .as_ref()
            .unwrap();
        assert_eq!(
            probe.http_get.as_ref().unwrap().path.as_deref(),
            Some("/custom")
        );
        assert_eq!(probe.period_seconds, Some(17));
    }

    #[test]
    fn disabled_health_check_is_noop() {
        let mut game_server = fixture();
        game_server.spec.health.disabled = true;
        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();

        add_game_server_health_check(&game_server, &mut pod).unwrap();
        assert!(
            pod.spec.as_ref().unwrap().containers[0]
                .liveness_probe
                .is_none()
        );
    }

    #[test]
    fn legacy_gameserver_without_sdk_ports_is_unchanged() {
        let mut game_server = fixture();
        game_server.spec.sdk_server = SdkServer::default();
        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        let before = serde_json::to_value(&pod).unwrap();

        add_sdk_server_env_vars(&game_server, &mut pod).unwrap();

        assert_eq!(serde_json::to_value(&pod).unwrap(), before);
    }

    #[test]
    fn sdk_env_replaces_conflicts_and_preserves_unreserved_env() {
        let game_server = fixture();
        let mut pod = build_pod(
            &game_server,
            Vec::new(),
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();
        let container = &mut pod.spec.as_mut().unwrap().containers[0];
        container.env = Some(vec![
            EnvVar {
                name: "one".into(),
                value: Some("value".into()),
                value_from: None,
            },
            EnvVar {
                name: GRPC_PORT_ENV_VAR.into(),
                value: Some("wrong".into()),
                value_from: None,
            },
            EnvVar {
                name: HTTP_PORT_ENV_VAR.into(),
                value: Some("wrong".into()),
                value_from: None,
            },
        ]);

        add_sdk_server_env_vars(&game_server, &mut pod).unwrap();

        let container = &pod.spec.as_ref().unwrap().containers[0];
        assert_eq!(env_value(container, "one"), Some("value"));
        assert_eq!(
            env_value(container, GRPC_PORT_ENV_VAR),
            Some(game_server.spec.sdk_server.grpc_port.to_string().as_str())
        );
        assert_eq!(
            env_value(container, HTTP_PORT_ENV_VAR),
            Some(game_server.spec.sdk_server.http_port.to_string().as_str())
        );
        let env = container.env.as_ref().unwrap();
        assert_eq!(
            env.iter()
                .filter(|item| item.name == GRPC_PORT_ENV_VAR)
                .count(),
            1
        );
        assert_eq!(
            env.iter()
                .filter(|item| item.name == HTTP_PORT_ENV_VAR)
                .count(),
            1
        );
    }

    #[test]
    fn sdk_env_applies_to_regular_and_init_containers_but_not_sdk_sidecar() {
        let mut game_server = fixture();
        game_server.spec.template.spec.init_containers.push(
            CoreContainer {
                name: "init".into(),
                image: "init/image".into(),
                ..Default::default()
            },
        );
        game_server.spec.template.spec.containers.push(CoreContainer {
            name: "other".into(),
            image: "other/image".into(),
            ..Default::default()
        });

        let sidecar = Container {
            name: SDK_SERVER_SIDECAR_NAME.into(),
            image: "sidecar/image".into(),
            command: None,
            args: None,
            working_dir: None,
            ports: None,
            env: None,
            env_from: None,
            resources: None,
            volume_mounts: None,
            volume_devices: None,
            image_pull_policy: None,
            liveness_probe: None,
            readiness_probe: None,
            startup_probe: None,
            security_context: None,
            restart_policy: None,
            resize_policy: None,
            lifecycle: None,
            termination_message_path: None,
            termination_message_policy: None,
            stdin: None,
            stdin_once: None,
            tty: None,
        };
        let mut pod = build_pod(
            &game_server,
            vec![sidecar],
            PodBuildFeatures::default(),
            &NoopApiHooks,
        )
        .unwrap();

        add_sdk_server_env_vars(&game_server, &mut pod).unwrap();

        let spec = pod.spec.as_ref().unwrap();
        let sdk = spec
            .containers
            .iter()
            .find(|container| container.name == SDK_SERVER_SIDECAR_NAME)
            .unwrap();
        assert!(env_value(sdk, GRPC_PORT_ENV_VAR).is_none());
        assert!(env_value(sdk, HTTP_PORT_ENV_VAR).is_none());

        for container in spec
            .containers
            .iter()
            .filter(|container| container.name != SDK_SERVER_SIDECAR_NAME)
        {
            assert_eq!(
                env_value(container, GRPC_PORT_ENV_VAR),
                Some(game_server.spec.sdk_server.grpc_port.to_string().as_str())
            );
            assert_eq!(
                env_value(container, HTTP_PORT_ENV_VAR),
                Some(game_server.spec.sdk_server.http_port.to_string().as_str())
            );
        }
        for container in spec.init_containers.as_ref().unwrap() {
            assert_eq!(
                env_value(container, GRPC_PORT_ENV_VAR),
                Some(game_server.spec.sdk_server.grpc_port.to_string().as_str())
            );
            assert_eq!(
                env_value(container, HTTP_PORT_ENV_VAR),
                Some(game_server.spec.sdk_server.http_port.to_string().as_str())
            );
        }
    }
}
