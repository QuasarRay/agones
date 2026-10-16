use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gameserver::{
    validate_object_meta, GameServer, GameServerSpec, GameServerState, ObjectMeta, Priority,
    SchedulingStrategy, ValidationError, ValidationErrorKind,
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

impl AllocationOverflow {
    pub fn validate(&self, path: &str) -> Vec<ValidationError> {
        validate_object_meta(
            &ObjectMeta {
                labels: self.labels.clone(),
                annotations: self.annotations.clone(),
                ..Default::default()
            },
            path,
        )
    }

    pub fn count_matches<'a>(
        &self,
        game_servers: &'a [GameServer],
    ) -> (i32, Vec<&'a GameServer>) {
        fn contains_all(
            expected: &BTreeMap<String, String>,
            actual: &BTreeMap<String, String>,
        ) -> bool {
            expected
                .iter()
                .all(|(key, value)| actual.get(key) == Some(value))
        }

        let mut count = 0i32;
        let mut rest = Vec::new();
        for game_server in game_servers {
            if game_server.status.state != Some(GameServerState::Allocated) {
                continue;
            }
            if contains_all(&self.labels, &game_server.metadata.labels)
                && contains_all(&self.annotations, &game_server.metadata.annotations)
            {
                count += 1;
            } else {
                rest.push(game_server);
            }
        }
        (count, rest)
    }

    pub fn apply(&self, game_server: &mut GameServer) {
        game_server
            .metadata
            .annotations
            .extend(self.annotations.clone());
        game_server.metadata.labels.extend(self.labels.clone());
    }
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
        if let Some(allocation_overflow) = &self.spec.allocation_overflow {
            errors.extend(allocation_overflow.validate("spec.allocationOverflow"));
        }
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
    fn allocation_overflow_counts_only_allocated_matches() {
        let allocated = |name: &str| GameServer {
            metadata: ObjectMeta {
                name: name.into(),
                ..Default::default()
            },
            status: crate::gameserver::GameServerStatus {
                state: Some(GameServerState::Allocated),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut one = allocated("g1");
        one.metadata.labels.insert("colour".into(), "blue".into());
        one.metadata
            .annotations
            .insert("region".into(), "green".into());
        let two = allocated("g2");
        let ready = GameServer {
            metadata: ObjectMeta {
                name: "g3".into(),
                ..Default::default()
            },
            status: crate::gameserver::GameServerStatus {
                state: Some(GameServerState::Ready),
                ..Default::default()
            },
            ..Default::default()
        };
        let list = vec![one, two, ready];

        let overflow = AllocationOverflow {
            labels: BTreeMap::from([("colour".into(), "blue".into())]),
            annotations: BTreeMap::from([("region".into(), "green".into())]),
        };
        let (count, rest) = overflow.count_matches(&list);
        assert_eq!(count, 1);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].metadata.name, "g2");
    }

    #[test]
    fn allocation_overflow_apply_merges_and_overwrites_metadata() {
        let mut gs = GameServer::default();
        let first = AllocationOverflow {
            labels: BTreeMap::from([("colour".into(), "green".into())]),
            annotations: BTreeMap::from([
                ("colour".into(), "blue".into()),
                ("map".into(), "ice cream".into()),
            ]),
        };
        first.apply(&mut gs);

        AllocationOverflow {
            labels: BTreeMap::from([
                ("colour".into(), "red".into()),
                ("version".into(), "1.0".into()),
            ]),
            annotations: BTreeMap::from([
                ("colour".into(), "green".into()),
                ("version".into(), "1.0".into()),
            ]),
        }
        .apply(&mut gs);

        assert_eq!(gs.metadata.labels["colour"], "red");
        assert_eq!(gs.metadata.labels["version"], "1.0");
        assert_eq!(gs.metadata.annotations["colour"], "green");
        assert_eq!(gs.metadata.annotations["map"], "ice cream");
    }

    #[test]
    fn allocation_overflow_validation_matches_label_annotation_rules() {
        let invalid_label_name = AllocationOverflow {
            labels: BTreeMap::from([("$$foobar".into(), "stuff".into())]),
            ..Default::default()
        };
        assert_eq!(
            invalid_label_name.validate("spec.allocationOverflow")[0].path,
            "spec.allocationOverflow.labels"
        );

        let invalid_label_value = AllocationOverflow {
            labels: BTreeMap::from([("valid".into(), "$$NOPE".into())]),
            ..Default::default()
        };
        assert_eq!(
            invalid_label_value.validate("spec.allocationOverflow")[0].path,
            "spec.allocationOverflow.labels"
        );

        let invalid_annotation = AllocationOverflow {
            annotations: BTreeMap::from([("$$foobar".into(), "stuff".into())]),
            ..Default::default()
        };
        assert_eq!(
            invalid_annotation.validate("spec.allocationOverflow")[0].path,
            "spec.allocationOverflow.annotations"
        );

        let valid = AllocationOverflow {
            labels: BTreeMap::from([
                ("valid".into(), "yes".into()),
                ("still.valid".into(), "check-me-out".into()),
            ]),
            annotations: BTreeMap::from([(
                "icando-this".into(),
                "yes, I can do all kinds of things here $$".into(),
            )]),
        };
        assert!(valid.validate("spec.allocationOverflow").is_empty());
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
