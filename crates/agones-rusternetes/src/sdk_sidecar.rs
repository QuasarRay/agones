use std::collections::HashMap;

use agones_core::{FeatureGates, GameServer};
use agones_macros::agones_invariant;
use rusternetes_common::resources::{
    Capabilities, Container, EnvVar, EnvVarSource, HTTPGetAction,
    ObjectFieldSelector, Probe, SeccompProfile, SecurityContext,
};
use rusternetes_common::types::ResourceRequirements;

use crate::AdapterError;
use crate::gameserver_pod_augmentation::SDK_SERVER_SIDECAR_NAME;

#[derive(Clone, Debug)]
pub struct SdkSidecarConfig {
    pub image: String,
    pub feature_gates: String,
    pub requests_rate_limit: String,
    pub list_max_capacity: i64,
    pub cpu_request: Option<String>,
    pub cpu_limit: Option<String>,
    pub memory_request: Option<String>,
    pub memory_limit: Option<String>,
    pub always_pull_image: bool,
    pub security_context: SecurityContext,
}

impl SdkSidecarConfig {
    pub fn from_feature_inputs<C: agones_core::CanLoadFeatureGates>(
        image: impl Into<String>,
        run_as_user: i64,
        context: &C,
    ) -> Result<Self, agones_core::FeatureGateError> {
        let gates = context.load_feature_gates()?;
        Ok(Self::with_feature_gates(image, run_as_user, &gates))
    }

    pub fn new(image: impl Into<String>, run_as_user: i64) -> Self {
        Self::from_feature_inputs(image, run_as_user, &agones_core::FeatureGateInputContext::default())
            .expect("invalid Agones process feature configuration")
    }

    fn with_feature_gates(image: impl Into<String>, run_as_user: i64, gates: &FeatureGates) -> Self {
        Self {
            image: image.into(),
            feature_gates: gates.encode(),
            requests_rate_limit: "500ms".to_owned(),
            list_max_capacity: 1000,
            cpu_request: None,
            cpu_limit: None,
            memory_request: None,
            memory_limit: None,
            always_pull_image: false,
            security_context: default_sidecar_security_context(run_as_user),
        }
    }
}

pub fn default_sidecar_security_context(run_as_user: i64) -> SecurityContext {
    SecurityContext {
        privileged: None,
        run_as_user: Some(run_as_user),
        run_as_group: None,
        run_as_non_root: Some(true),
        read_only_root_filesystem: None,
        allow_privilege_escalation: Some(false),
        proc_mount: None,
        capabilities: Some(Capabilities {
            add: None,
            drop: Some(vec!["ALL".to_owned()]),
        }),
        seccomp_profile: Some(SeccompProfile {
            r#type: "RuntimeDefault".to_owned(),
            localhost_profile: None,
        }),
        se_linux_options: None,
        app_armor_profile: None,
        windows_options: None,
    }
}

#[agones_invariant]
fn default_sidecar_security_context_is_restricted(run_as_user: i64) -> bool {
    let context = default_sidecar_security_context(run_as_user);
    context.allow_privilege_escalation == Some(false)
        && context.run_as_non_root == Some(true)
        && context.run_as_user == Some(run_as_user)
        && context
            .capabilities
            .as_ref()
            .and_then(|capabilities| capabilities.drop.as_ref())
            .is_some_and(|drop| drop == &["ALL".to_owned()])
        && context
            .seccomp_profile
            .as_ref()
            .is_some_and(|profile| profile.r#type == "RuntimeDefault")
}

fn resources(config: &SdkSidecarConfig) -> ResourceRequirements {
    let mut requests = HashMap::new();
    if let Some(cpu) = &config.cpu_request {
        requests.insert("cpu".to_owned(), cpu.clone());
    }
    if let Some(memory) = &config.memory_request {
        requests.insert("memory".to_owned(), memory.clone());
    }

    let mut limits = HashMap::new();
    if let Some(cpu) = &config.cpu_limit {
        limits.insert("cpu".to_owned(), cpu.clone());
    }
    if let Some(memory) = &config.memory_limit {
        limits.insert("memory".to_owned(), memory.clone());
    }

    ResourceRequirements {
        requests: (!requests.is_empty()).then_some(requests),
        limits: (!limits.is_empty()).then_some(limits),
        claims: None,
    }
}

/// Mirrors upstream Controller.sidecar on Rusternetes-native Pod types.
pub fn build_sdk_sidecar(
    game_server: &GameServer,
    config: &SdkSidecarConfig,
) -> Result<Container, AdapterError> {
    let mut env = vec![
        EnvVar {
            name: "GAMESERVER_NAME".to_owned(),
            value: Some(game_server.metadata.name.clone()),
            value_from: None,
        },
        EnvVar {
            name: "POD_NAMESPACE".to_owned(),
            value: None,
            value_from: Some(EnvVarSource {
                config_map_key_ref: None,
                secret_key_ref: None,
                field_ref: Some(ObjectFieldSelector {
                    field_path: "metadata.namespace".to_owned(),
                    api_version: None,
                }),
                resource_field_ref: None,
                file_key_ref: None,
            }),
        },
        EnvVar {
            name: "FEATURE_GATES".to_owned(),
            value: Some(config.feature_gates.clone()),
            value_from: None,
        },
        EnvVar {
            name: "LOG_LEVEL".to_owned(),
            value: Some(
                game_server
                    .spec
                    .sdk_server
                    .log_level
                    .as_ref()
                    .map(|value| value.as_str().to_owned())
                    .unwrap_or_default(),
            ),
            value_from: None,
        },
        EnvVar {
            name: "REQUESTS_RATE_LIMIT".to_owned(),
            value: Some(config.requests_rate_limit.clone()),
            value_from: None,
        },
        EnvVar {
            name: "MAX_LIST_ITEMS".to_owned(),
            value: Some(config.list_max_capacity.to_string()),
            value_from: None,
        },
    ];

    // Preserve upstream ordering and avoid accidental mutation sharing.
    env.shrink_to_fit();

    let mut args = Vec::new();
    if game_server.spec.sdk_server.grpc_port != 0 {
        args.push(format!(
            "--grpc-port={}",
            game_server.spec.sdk_server.grpc_port
        ));
    }
    if game_server.spec.sdk_server.http_port != 0 {
        args.push(format!(
            "--http-port={}",
            game_server.spec.sdk_server.http_port
        ));
    }

    Ok(Container {
        name: SDK_SERVER_SIDECAR_NAME.to_owned(),
        image: config.image.clone(),
        command: None,
        args: (!args.is_empty()).then_some(args),
        working_dir: None,
        ports: None,
        env: Some(env),
        env_from: None,
        resources: Some(resources(config)),
        volume_mounts: None,
        volume_devices: None,
        image_pull_policy: config.always_pull_image.then(|| "Always".to_owned()),
        liveness_probe: Some(Probe {
            http_get: Some(HTTPGetAction {
                path: Some("/healthz".to_owned()),
                port: rusternetes_common::resources::IntOrString::Int(8080),
                host: None,
                scheme: None,
                http_headers: None,
            }),
            tcp_socket: None,
            exec: None,
            initial_delay_seconds: Some(3),
            timeout_seconds: None,
            period_seconds: Some(3),
            success_threshold: None,
            failure_threshold: None,
            grpc: None,
            termination_grace_period_seconds: None,
        }),
        readiness_probe: None,
        startup_probe: None,
        security_context: Some(config.security_context.clone()),
        restart_policy: None,
        resize_policy: None,
        lifecycle: None,
        termination_message_path: None,
        termination_message_policy: None,
        stdin: None,
        stdin_once: None,
        tty: None,
    })
}

#[cfg(test)]
mod tests {
    use agones_core::{
        Container as CoreContainer, GameServerPort, GameServerSpec,
        ObjectMeta, PodSpec as CorePodSpec, PodTemplateSpec,
    };

    use super::*;

    fn game_server() -> GameServer {
        let mut game_server = GameServer {
            metadata: ObjectMeta {
                name: "gameserver".into(),
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
        };
        game_server.apply_defaults();
        game_server
    }

    fn env_value<'a>(sidecar: &'a Container, name: &str) -> Option<&'a str> {
        sidecar
            .env
            .as_ref()?
            .iter()
            .find(|entry| entry.name == name)?
            .value
            .as_deref()
    }

    #[test]
    fn sidecar_matches_upstream_identity_env_args_and_health_probe() {
        let game_server = game_server();
        let mut config = SdkSidecarConfig::new("sidecar:dev", 1000);
        config.feature_gates = "SidecarContainers=true".into();
        config.requests_rate_limit = "500ms".into();
        config.list_max_capacity = 1000;

        let sidecar = build_sdk_sidecar(&game_server, &config).unwrap();

        assert_eq!(sidecar.name, SDK_SERVER_SIDECAR_NAME);
        assert_eq!(sidecar.image, "sidecar:dev");
        assert_eq!(
            env_value(&sidecar, "GAMESERVER_NAME"),
            Some("gameserver")
        );
        assert_eq!(
            env_value(&sidecar, "FEATURE_GATES"),
            Some("SidecarContainers=true")
        );
        assert_eq!(env_value(&sidecar, "REQUESTS_RATE_LIMIT"), Some("500ms"));
        assert_eq!(env_value(&sidecar, "MAX_LIST_ITEMS"), Some("1000"));

        let pod_namespace = sidecar
            .env
            .as_ref()
            .unwrap()
            .iter()
            .find(|entry| entry.name == "POD_NAMESPACE")
            .unwrap();
        assert_eq!(
            pod_namespace
                .value_from
                .as_ref()
                .and_then(|source| source.field_ref.as_ref())
                .map(|field| field.field_path.as_str()),
            Some("metadata.namespace")
        );

        assert_eq!(
            sidecar.args.as_ref().unwrap(),
            &[
                "--grpc-port=9357".to_owned(),
                "--http-port=9358".to_owned()
            ]
        );
        let probe = sidecar.liveness_probe.as_ref().unwrap();
        assert_eq!(
            probe.http_get.as_ref().unwrap().path.as_deref(),
            Some("/healthz")
        );
        assert!(matches!(
            probe.http_get.as_ref().unwrap().port,
            rusternetes_common::resources::IntOrString::Int(8080)
        ));
        assert_eq!(probe.initial_delay_seconds, Some(3));
        assert_eq!(probe.period_seconds, Some(3));
    }

    #[test]
    fn sidecar_resource_and_pull_configuration_matches_upstream_options() {
        let game_server = game_server();
        let mut config = SdkSidecarConfig::new("sidecar:dev", 1000);
        config.cpu_request = Some("0.05".into());
        config.cpu_limit = Some("0.1".into());
        config.memory_request = Some("50Mi".into());
        config.memory_limit = Some("100Mi".into());
        config.always_pull_image = true;

        let sidecar = build_sdk_sidecar(&game_server, &config).unwrap();
        let resources = sidecar.resources.as_ref().unwrap();

        assert_eq!(
            resources
                .requests
                .as_ref()
                .and_then(|items| items.get("cpu"))
                .map(String::as_str),
            Some("0.05")
        );
        assert_eq!(
            resources
                .requests
                .as_ref()
                .and_then(|items| items.get("memory"))
                .map(String::as_str),
            Some("50Mi")
        );
        assert_eq!(
            resources
                .limits
                .as_ref()
                .and_then(|items| items.get("cpu"))
                .map(String::as_str),
            Some("0.1")
        );
        assert_eq!(
            resources
                .limits
                .as_ref()
                .and_then(|items| items.get("memory"))
                .map(String::as_str),
            Some("100Mi")
        );
        assert_eq!(sidecar.image_pull_policy.as_deref(), Some("Always"));
    }

    #[test]
    fn default_security_context_matches_restricted_pod_security_standard() {
        let context = default_sidecar_security_context(1000);
        assert_eq!(context.allow_privilege_escalation, Some(false));
        assert_eq!(context.run_as_non_root, Some(true));
        assert_eq!(context.run_as_user, Some(1000));
        assert_eq!(
            context
                .capabilities
                .as_ref()
                .and_then(|capabilities| capabilities.drop.as_ref()),
            Some(&vec!["ALL".to_owned()])
        );
        assert_eq!(
            context
                .seccomp_profile
                .as_ref()
                .map(|profile| profile.r#type.as_str()),
            Some("RuntimeDefault")
        );
    }

    #[test]
    fn custom_security_context_is_used_without_adding_defaults() {
        let game_server = game_server();
        let mut config = SdkSidecarConfig::new("sidecar:dev", 1000);
        config.security_context = SecurityContext {
            privileged: None,
            run_as_user: Some(2000),
            run_as_group: Some(3000),
            run_as_non_root: Some(true),
            read_only_root_filesystem: None,
            allow_privilege_escalation: None,
            proc_mount: None,
            capabilities: None,
            seccomp_profile: None,
            se_linux_options: None,
            app_armor_profile: None,
            windows_options: None,
        };

        let sidecar = build_sdk_sidecar(&game_server, &config).unwrap();
        let context = sidecar.security_context.as_ref().unwrap();

        assert_eq!(context.run_as_user, Some(2000));
        assert_eq!(context.run_as_group, Some(3000));
        assert_eq!(context.run_as_non_root, Some(true));
        assert!(context.capabilities.is_none());
        assert!(context.seccomp_profile.is_none());
    }

    #[test]
    fn every_sidecar_receives_an_independent_security_context_clone() {
        let game_server = game_server();
        let config = SdkSidecarConfig::new("sidecar:dev", 1000);

        let mut first = build_sdk_sidecar(&game_server, &config).unwrap();
        let second = build_sdk_sidecar(&game_server, &config).unwrap();

        first.security_context.as_mut().unwrap().run_as_user = Some(2000);

        assert_eq!(
            second.security_context.as_ref().unwrap().run_as_user,
            Some(1000)
        );
        assert_eq!(config.security_context.run_as_user, Some(1000));
    }
}
