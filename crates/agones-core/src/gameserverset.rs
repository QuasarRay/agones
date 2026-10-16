use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gameserver::{
    validate_object_meta, GameServer, GameServerSpec, ObjectMeta, Priority, SchedulingStrategy,
    ValidationError, ValidationErrorKind,
};

pub const GAME_SERVER_SET_GAME_SERVER_LABEL: &str = "agones.dev/gameserverset";
pub const FLEET_NAME_LABEL: &str = "agones.dev/fleet";

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationOverflow {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedPlayerStatus {
    pub count: i64,
    pub capacity: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedCounterStatus {
    pub allocated_count: i64,
    pub allocated_capacity: i64,
    pub count: i64,
    pub capacity: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedListStatus {
    pub allocated_count: i64,
    pub allocated_capacity: i64,
    pub count: i64,
    pub capacity: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerTemplateSpec {
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: GameServerSpec,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerSetSpec {
    pub replicas: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_overflow: Option<AllocationOverflow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<SchedulingStrategy>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priorities: Vec<Priority>,
    pub template: GameServerTemplateSpec,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerSetStatus {
    pub replicas: i32,
    pub ready_replicas: i32,
    pub reserved_replicas: i32,
    pub allocated_replicas: i32,
    pub shutdown_replicas: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub players: Option<AggregatedPlayerStatus>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, AggregatedCounterStatus>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub lists: BTreeMap<String, AggregatedListStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerSet {
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: GameServerSetSpec,
    #[serde(default)]
    pub status: GameServerSetStatus,
}

fn validate_name(metadata: &ObjectMeta) -> Vec<ValidationError> {
    if metadata.name.len() <= 63 {
        return Vec::new();
    }
    vec![ValidationError {
        kind: ValidationErrorKind::TooLong,
        path: "metadata.name".to_owned(),
        bad_value: Some(metadata.name.clone()),
        detail: "must be no more than 63 bytes".to_owned(),
    }]
}

fn rebase_spec_error(mut error: ValidationError) -> ValidationError {
    error.path = if let Some(rest) = error.path.strip_prefix("spec.") {
        format!("spec.template.spec.{rest}")
    } else {
        format!("spec.template.spec.{}", error.path)
    };
    error
}

impl GameServerSet {
    pub fn validate_update(&self, new: &GameServerSet) -> Vec<ValidationError> {
        let mut errors = validate_name(&new.metadata);
        if self.spec.template != new.spec.template {
            errors.push(ValidationError {
                kind: ValidationErrorKind::Invalid,
                path: "spec.template".to_owned(),
                bad_value: None,
                detail: "GameServerSet template cannot be updated".to_owned(),
            });
        }
        errors
    }

    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = validate_name(&self.metadata);

        // Agones validateGSSpec applies GameServer defaults before validation.
        let mut game_server_spec = self.spec.template.spec.clone();
        game_server_spec.apply_defaults();
        errors.extend(
            game_server_spec
                .validate("")
                .into_iter()
                .map(rebase_spec_error),
        );
        errors.extend(validate_object_meta(
            &self.spec.template.metadata,
            "spec.template.metadata",
        ));
        errors
    }

    pub fn game_server_spec(&self) -> &GameServerSpec {
        &self.spec.template.spec
    }

    pub fn game_server_spec_mut(&mut self) -> &mut GameServerSpec {
        &mut self.spec.template.spec
    }

    /// Derives one GameServer from this set exactly as the Go GameServerSet.GameServer
    /// domain logic does. Rusternetes-specific owner-reference representation is
    /// retained in ObjectMeta's opaque extension map.
    pub fn game_server(&self) -> GameServer {
        let mut game_server = GameServer {
            metadata: self.spec.template.metadata.clone(),
            spec: self.spec.template.spec.clone(),
            ..Default::default()
        };

        game_server.spec.scheduling = self.spec.scheduling.clone();
        game_server.metadata.generate_name = format!("{}-", self.metadata.name);
        game_server.metadata.name.clear();
        game_server.metadata.namespace = self.metadata.namespace.clone();
        game_server.metadata.resource_version = None;
        game_server.metadata.uid.clear();

        let owner = serde_json::json!({
            "apiVersion": "agones.dev/v1",
            "kind": "GameServerSet",
            "name": self.metadata.name,
            "uid": self.metadata.uid,
            "controller": true,
            "blockOwnerDeletion": true
        });
        match game_server.metadata.extensions.get_mut("ownerReferences") {
            Some(serde_json::Value::Array(owners)) => owners.push(owner),
            _ => {
                game_server.metadata.extensions.insert(
                    "ownerReferences".to_owned(),
                    serde_json::Value::Array(vec![owner]),
                );
            }
        }

        game_server.metadata.labels.insert(
            GAME_SERVER_SET_GAME_SERVER_LABEL.to_owned(),
            self.metadata.name.clone(),
        );
        game_server.metadata.labels.insert(
            FLEET_NAME_LABEL.to_owned(),
            self.metadata
                .labels
                .get(FLEET_NAME_LABEL)
                .cloned()
                .unwrap_or_default(),
        );

        game_server
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameserver::{
        Container, GameServerPort, PodSpec, PodTemplateSpec, PortPolicy, Protocol,
    };

    fn game_server_spec() -> GameServerSpec {
        GameServerSpec {
            ports: vec![GameServerPort {
                container_port: 1234,
                port_policy: Some(PortPolicy::Static),
                protocol: Some(Protocol::Udp),
                ..Default::default()
            }],
            template: PodTemplateSpec {
                spec: PodSpec {
                    containers: vec![Container {
                        name: "container".into(),
                        image: "myimage".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn derives_gameserver_from_template_like_go() {
        let set = GameServerSet {
            metadata: ObjectMeta {
                name: "test".into(),
                namespace: "namespace".into(),
                uid: "1234".into(),
                labels: BTreeMap::from([(FLEET_NAME_LABEL.into(), "fleetname".into())]),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                replicas: 10,
                scheduling: Some(SchedulingStrategy::Packed),
                template: GameServerTemplateSpec {
                    spec: game_server_spec(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let game_server = set.game_server();
        assert!(game_server.metadata.name.is_empty());
        assert_eq!(game_server.metadata.namespace, "namespace");
        assert_eq!(game_server.metadata.generate_name, "test-");
        assert_eq!(
            game_server.metadata.labels[GAME_SERVER_SET_GAME_SERVER_LABEL],
            "test"
        );
        assert_eq!(game_server.metadata.labels[FLEET_NAME_LABEL], "fleetname");
        assert_eq!(game_server.spec, set.spec.template.spec);
        assert_eq!(
            game_server.metadata.extensions["ownerReferences"][0]["kind"],
            "GameServerSet"
        );
        assert_eq!(
            game_server.metadata.extensions["ownerReferences"][0]["name"],
            "test"
        );
    }

    #[test]
    fn update_allows_replica_change_but_rejects_template_change() {
        let original = GameServerSet {
            metadata: ObjectMeta {
                name: "test".into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                replicas: 10,
                template: GameServerTemplateSpec {
                    spec: game_server_spec(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let mut replicas = original.clone();
        replicas.spec.replicas = 5;
        assert!(original.validate_update(&replicas).is_empty());

        replicas.spec.template.spec.ports[0].container_port = 321;
        let errors = original.validate_update(&replicas);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.template");
    }

    #[test]
    fn validates_names_and_rebases_nested_gameserver_errors() {
        let mut set = GameServerSet {
            metadata: ObjectMeta {
                name: "test".into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                template: GameServerTemplateSpec {
                    spec: game_server_spec(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        set.metadata.name = "f".repeat(64);
        let errors = set.validate();
        assert_eq!(errors[0].path, "metadata.name");

        set.metadata.name.clear();
        set.metadata.generate_name = "f".repeat(64);
        assert!(set.validate().is_empty());

        set.spec.template.spec.container.clear();
        set.spec.template.spec.template.spec.containers.push(Container {
            name: "container2".into(),
            image: "myimage".into(),
            ..Default::default()
        });
        let errors = set.validate();
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].path, "spec.template.spec.container");
    }

    #[test]
    fn validates_template_metadata_and_nested_pod_template_metadata() {
        let mut set = GameServerSet {
            spec: GameServerSetSpec {
                template: GameServerTemplateSpec {
                    spec: game_server_spec(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let long = "f".repeat(64);

        set.spec
            .template
            .metadata
            .labels
            .insert(long.clone(), String::new());
        let errors = set.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.template.metadata.labels");

        set.spec.template.metadata.labels.clear();
        set.spec
            .template
            .spec
            .template
            .metadata
            .annotations
            .insert(long, String::new());
        let errors = set.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].path,
            "spec.template.spec.template.metadata.annotations"
        );
    }
}
