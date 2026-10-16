use std::collections::BTreeMap;

use agones_macros::{agones_invariant, AgonesStringEnum};
use serde::{Deserialize, Serialize};

use crate::gameserver::{
    validate_object_meta, GameServerSpec, ObjectMeta, Priority, SchedulingStrategy,
    ValidationError, ValidationErrorKind, DEFAULT_SDK_VERSION, VERSION_ANNOTATION,
};
use crate::gameserverset::{
    AggregatedCounterStatus, AggregatedListStatus, AggregatedPlayerStatus, AllocationOverflow,
    GameServerSet, GameServerSetSpec, GameServerTemplateSpec, FLEET_NAME_LABEL,
};

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum DeploymentStrategyType {
    #[agones(value = "RollingUpdate")]
    RollingUpdate,
    #[agones(value = "Recreate")]
    Recreate,
    #[agones(unknown)]
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntOrPercent {
    Int(i32),
    String(String),
}

impl IntOrPercent {
    fn value(&self) -> Result<i32, ()> {
        match self {
            Self::Int(value) => Ok(*value),
            Self::String(value) => {
                let Some(percent) = value.strip_suffix('%') else {
                    return Err(());
                };
                percent.parse::<i32>().map_err(|_| ())
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDeployment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrPercent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrPercent>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStrategy {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<DeploymentStrategyType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDeployment>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetSpec {
    pub replicas: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_overflow: Option<AllocationOverflow>,
    #[serde(default)]
    pub strategy: DeploymentStrategy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<SchedulingStrategy>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priorities: Vec<Priority>,
    pub template: GameServerTemplateSpec,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetStatus {
    pub replicas: i32,
    pub ready_replicas: i32,
    pub reserved_replicas: i32,
    pub allocated_replicas: i32,
    pub allocations: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub players: Option<AggregatedPlayerStatus>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, AggregatedCounterStatus>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub lists: BTreeMap<String, AggregatedListStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fleet {
    #[serde(default)]
    pub metadata: ObjectMeta,
    pub spec: FleetSpec,
    #[serde(default)]
    pub status: FleetStatus,
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

fn validate_rolling_update(
    value: &IntOrPercent,
    path: &str,
) -> Option<ValidationError> {
    match value {
        IntOrPercent::String(_) => {
            let Ok(resolved) = value.value() else {
                return Some(ValidationError {
                    kind: ValidationErrorKind::Invalid,
                    path: path.to_owned(),
                    bad_value: None,
                    detail: "must be between 1% and 99%".to_owned(),
                });
            };
            if !(1..=99).contains(&resolved) {
                Some(ValidationError {
                    kind: ValidationErrorKind::Invalid,
                    path: path.to_owned(),
                    bad_value: None,
                    detail: "must be between 1% and 99%".to_owned(),
                })
            } else {
                None
            }
        }
        IntOrPercent::Int(value) if *value < 1 => Some(ValidationError {
            kind: ValidationErrorKind::Invalid,
            path: path.to_owned(),
            bad_value: Some(value.to_string()),
            detail: "must be at least 1".to_owned(),
        }),
        IntOrPercent::Int(_) => None,
    }
}

impl Fleet {
    pub fn game_server_set(&self) -> GameServerSet {
        let mut set = GameServerSet {
            metadata: self.spec.template.metadata.clone(),
            spec: GameServerSetSpec {
                template: self.spec.template.clone(),
                scheduling: self.spec.scheduling.clone(),
                allocation_overflow: self.spec.allocation_overflow.clone(),
                priorities: self.spec.priorities.clone(),
                ..Default::default()
            },
            ..Default::default()
        };

        set.metadata.generate_name = format!("{}-", self.metadata.name);
        set.metadata.name.clear();
        set.metadata.namespace = self.metadata.namespace.clone();
        set.metadata.resource_version = None;
        set.metadata.uid.clear();

        let owner = serde_json::json!({
            "apiVersion": "agones.dev/v1",
            "kind": "Fleet",
            "name": self.metadata.name,
            "uid": self.metadata.uid,
            "controller": true,
            "blockOwnerDeletion": true
        });
        match set.metadata.extensions.get_mut("ownerReferences") {
            Some(serde_json::Value::Array(owners)) => owners.push(owner),
            _ => {
                set.metadata.extensions.insert(
                    "ownerReferences".to_owned(),
                    serde_json::Value::Array(vec![owner]),
                );
            }
        }

        set.metadata
            .labels
            .insert(FLEET_NAME_LABEL.to_owned(), self.metadata.name.clone());
        set
    }

    pub fn apply_defaults(&mut self) {
        if self.spec.strategy.strategy_type.is_none() {
            self.spec.strategy.strategy_type = Some(DeploymentStrategyType::RollingUpdate);
        }
        if self.spec.scheduling.is_none() {
            self.spec.scheduling = Some(SchedulingStrategy::Packed);
        }

        if self.spec.strategy.strategy_type == Some(DeploymentStrategyType::RollingUpdate) {
            let rolling = self
                .spec
                .strategy
                .rolling_update
                .get_or_insert_with(RollingUpdateDeployment::default);
            if rolling.max_surge.is_none() {
                rolling.max_surge = Some(IntOrPercent::String("25%".to_owned()));
            }
            if rolling.max_unavailable.is_none() {
                rolling.max_unavailable = Some(IntOrPercent::String("25%".to_owned()));
            }
        }

        self.metadata.annotations.insert(
            VERSION_ANNOTATION.to_owned(),
            DEFAULT_SDK_VERSION.to_owned(),
        );
    }

    pub fn game_server_spec(&self) -> &GameServerSpec {
        &self.spec.template.spec
    }

    pub fn game_server_spec_mut(&mut self) -> &mut GameServerSpec {
        &mut self.spec.template.spec
    }

    pub fn validate(&mut self) -> Vec<ValidationError> {
        let mut errors = validate_name(&self.metadata);

        match self.spec.strategy.strategy_type.as_ref() {
            Some(DeploymentStrategyType::RollingUpdate) => {
                if let Some(rolling) = &self.spec.strategy.rolling_update {
                    if let Some(value) = &rolling.max_unavailable {
                        if let Some(error) = validate_rolling_update(
                            value,
                            "spec.strategy.rollingUpdate.maxUnavailable",
                        ) {
                            errors.push(error);
                        }
                    }
                    if let Some(value) = &rolling.max_surge {
                        if let Some(error) =
                            validate_rolling_update(value, "spec.strategy.rollingUpdate.maxSurge")
                        {
                            errors.push(error);
                        }
                    }
                }
            }
            Some(DeploymentStrategyType::Recreate) => {}
            Some(strategy) => errors.push(ValidationError {
                kind: ValidationErrorKind::NotSupported,
                path: "spec.strategy.type".to_owned(),
                bad_value: Some(strategy.as_str().to_owned()),
                detail: "supported values: RollingUpdate, Recreate".to_owned(),
            }),
            None => errors.push(ValidationError {
                kind: ValidationErrorKind::NotSupported,
                path: "spec.strategy.type".to_owned(),
                bad_value: Some(String::new()),
                detail: "supported values: RollingUpdate, Recreate".to_owned(),
            }),
        }

        self.spec.template.spec.apply_defaults();
        errors.extend(
            self.spec
                .template
                .spec
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

    pub fn upper_bound_replicas(&self, value: i32) -> i32 {
        value.min(self.spec.replicas)
    }

    pub fn lower_bound_replicas(&self, value: i32) -> i32 {
        value.max(0)
    }
}

pub fn sum_game_server_sets<'a>(
    sets: impl IntoIterator<Item = Option<&'a GameServerSet>>,
    value: impl Fn(&GameServerSet) -> i32,
) -> i32 {
    sets.into_iter().flatten().map(value).sum()
}

#[agones_invariant]
fn fleet_replica_bounds_are_bounded(input: (i32, i32)) -> bool {
    let (replicas, value) = input;
    if replicas < 0 {
        return true;
    }

    let fleet = Fleet {
        spec: FleetSpec {
            replicas,
            ..Default::default()
        },
        ..Default::default()
    };

    fleet.upper_bound_replicas(value) <= replicas
        && fleet.lower_bound_replicas(value) >= 0
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameserver::{Container, GameServerPort, PodSpec, PodTemplateSpec};

    fn default_fleet() -> Fleet {
        Fleet {
            metadata: ObjectMeta {
                generate_name: "simple-fleet-".into(),
                namespace: "defaultNs".into(),
                ..Default::default()
            },
            spec: FleetSpec {
                replicas: 2,
                template: GameServerTemplateSpec {
                    spec: GameServerSpec {
                        template: PodTemplateSpec {
                            spec: PodSpec {
                                containers: vec![Container {
                                    name: "testing".into(),
                                    image: "testing/image".into(),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn derives_gameserverset_from_fleet() {
        let mut fleet = Fleet {
            metadata: ObjectMeta {
                name: "test".into(),
                namespace: "namespace".into(),
                uid: "1234".into(),
                ..Default::default()
            },
            spec: FleetSpec {
                replicas: 10,
                scheduling: Some(SchedulingStrategy::Packed),
                template: GameServerTemplateSpec {
                    spec: GameServerSpec {
                        ports: vec![GameServerPort {
                            container_port: 1234,
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
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        fleet.spec.allocation_overflow = Some(AllocationOverflow {
            labels: BTreeMap::from([("stuff".into(), "things".into())]),
            ..Default::default()
        });
        fleet.spec.priorities = vec![Priority {
            priority_type: "Counter".into(),
            key: "Foo".into(),
            order: "Ascending".into(),
        }];

        let set = fleet.game_server_set();
        assert!(set.metadata.name.is_empty());
        assert_eq!(set.metadata.namespace, "namespace");
        assert_eq!(set.metadata.generate_name, "test-");
        assert_eq!(set.metadata.labels[FLEET_NAME_LABEL], "test");
        assert_eq!(set.spec.replicas, 0);
        assert_eq!(set.spec.scheduling, fleet.spec.scheduling);
        assert_eq!(set.spec.template, fleet.spec.template);
        assert_eq!(
            set.spec.allocation_overflow.unwrap().labels["stuff"],
            "things"
        );
        assert_eq!(set.spec.priorities[0], fleet.spec.priorities[0]);
        assert_eq!(set.metadata.extensions["ownerReferences"][0]["kind"], "Fleet");
    }

    #[test]
    fn fleet_defaults_match_go_and_are_idempotent() {
        let mut fleet = Fleet::default();
        assert!(fleet.spec.strategy.strategy_type.is_none());
        assert!(fleet.spec.scheduling.is_none());

        fleet.apply_defaults();
        let once = fleet.clone();
        assert_eq!(
            fleet.spec.strategy.strategy_type,
            Some(DeploymentStrategyType::RollingUpdate)
        );
        let rolling = fleet.spec.strategy.rolling_update.as_ref().unwrap();
        assert_eq!(
            rolling.max_unavailable,
            Some(IntOrPercent::String("25%".into()))
        );
        assert_eq!(rolling.max_surge, Some(IntOrPercent::String("25%".into())));
        assert_eq!(fleet.spec.scheduling, Some(SchedulingStrategy::Packed));
        assert_eq!(
            fleet.metadata.annotations[VERSION_ANNOTATION],
            DEFAULT_SDK_VERSION
        );

        fleet.apply_defaults();
        assert_eq!(fleet, once);
    }

    #[test]
    fn validates_gameserver_spec_and_rolling_update_parameters() {
        let mut fleet = default_fleet();
        fleet.apply_defaults();
        assert!(fleet.validate().is_empty());

        fleet.spec.template.spec.template = PodTemplateSpec {
            spec: PodSpec {
                containers: vec![
                    Container {
                        name: "container".into(),
                        image: "myimage".into(),
                        ..Default::default()
                    },
                    Container {
                        name: "container2".into(),
                        image: "myimage".into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        };
        let errors = fleet.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.template.spec.container");

        fleet.spec.template.spec.container = "testing".into();
        let errors = fleet.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].detail,
            "Could not find a container named testing"
        );

        fleet.spec.template.spec.container = "container".into();
        assert!(fleet.validate().is_empty());

        let rolling = fleet.spec.strategy.rolling_update.as_mut().unwrap();
        rolling.max_unavailable = Some(IntOrPercent::String("0%".into()));
        rolling.max_surge = Some(IntOrPercent::String("0%".into()));
        assert_eq!(fleet.validate().len(), 2);

        let rolling = fleet.spec.strategy.rolling_update.as_mut().unwrap();
        rolling.max_unavailable = Some(IntOrPercent::Int(0));
        rolling.max_surge = Some(IntOrPercent::Int(0));
        assert_eq!(fleet.validate().len(), 2);

        let rolling = fleet.spec.strategy.rolling_update.as_mut().unwrap();
        rolling.max_unavailable = Some(IntOrPercent::String("2a".into()));
        rolling.max_surge = Some(IntOrPercent::String("2a".into()));
        assert_eq!(fleet.validate().len(), 2);
    }

    #[test]
    fn fleet_name_template_metadata_and_allocation_overflow_validate() {
        let mut fleet = default_fleet();
        fleet.apply_defaults();

        fleet.metadata.name = "f".repeat(64);
        assert_eq!(fleet.validate()[0].path, "metadata.name");

        fleet.metadata.name.clear();
        fleet.metadata.generate_name = "f".repeat(64);
        assert!(fleet.validate().is_empty());

        fleet.spec.template.metadata.labels.insert(
            "label".into(),
            "f".repeat(64),
        );
        assert_eq!(fleet.validate().len(), 1);
        fleet.spec.template.metadata.labels.clear();

        fleet.spec.allocation_overflow = Some(AllocationOverflow {
            labels: BTreeMap::from([("$$$nope".into(), "value".into())]),
            ..Default::default()
        });
        assert_eq!(fleet.validate().len(), 1);
    }

    #[test]
    fn rejects_unsupported_strategy() {
        let mut fleet = default_fleet();
        fleet.apply_defaults();
        fleet.spec.strategy.strategy_type =
            Some(DeploymentStrategyType::from("SomethingElse"));
        let errors = fleet.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.strategy.type");
        assert_eq!(errors[0].kind, ValidationErrorKind::NotSupported);
    }

    #[test]
    fn replica_bounds_match_go() {
        let fleet = Fleet {
            spec: FleetSpec {
                replicas: 10,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(fleet.upper_bound_replicas(12), 10);
        assert_eq!(fleet.upper_bound_replicas(10), 10);
        assert_eq!(fleet.upper_bound_replicas(5), 5);
        assert_eq!(fleet.lower_bound_replicas(5), 5);
        assert_eq!(fleet.lower_bound_replicas(0), 0);
        assert_eq!(fleet.lower_bound_replicas(-5), 0);
    }

    #[test]
    fn sums_gameserverset_values_while_skipping_absent_entries() {
        let one = GameServerSet {
            status: crate::gameserverset::GameServerSetStatus {
                ready_replicas: 1000,
                ..Default::default()
            },
            ..Default::default()
        };
        let two = GameServerSet {
            status: crate::gameserverset::GameServerSetStatus {
                ready_replicas: 15,
                ..Default::default()
            },
            ..Default::default()
        };
        let three = GameServerSet {
            status: crate::gameserverset::GameServerSetStatus {
                ready_replicas: 5,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            sum_game_server_sets(
                [Some(&one), Some(&two), Some(&three), None],
                |set| set.status.ready_replicas,
            ),
            1020
        );
    }

}
