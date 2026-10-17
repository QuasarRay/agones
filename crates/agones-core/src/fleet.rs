use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

use agones_macros::{agones_invariant, AgonesStringEnum};
use serde::{Deserialize, Serialize};

use crate::gameserver::{
    safe_add, validate_object_meta, GameServerSpec, ObjectMeta, Priority, SchedulingStrategy,
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

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    agones_macros::AgonesApiResource,
)]
#[agones_resource(
    group = "agones.dev",
    version = "v1",
    kind = "Fleet",
    plural = "fleets"
)]
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

fn fleet_game_server_set_creation_timestamp(
    game_server_set: &GameServerSet,
) -> Option<DateTime<Utc>> {
    game_server_set
        .metadata
        .extensions
        .get("creationTimestamp")
        .and_then(serde_json::Value::as_str)
        .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
        .map(|value| value.with_timezone(&Utc))
}

fn game_server_set_is_older(
    candidate: &GameServerSet,
    current: &GameServerSet,
) -> bool {
    match (
        fleet_game_server_set_creation_timestamp(candidate),
        fleet_game_server_set_creation_timestamp(current),
    ) {
        (None, Some(_)) => true,
        (Some(_), None) => false,
        (Some(candidate), Some(current)) => candidate < current,
        (None, None) => false,
    }
}

/// Pure port of upstream Fleet filterGameServerSetByActive.
///
/// The caller supplies the GameServerSets already known to be controlled by the
/// Fleet. Template equality determines the active set. If informer lag exposes
/// multiple sets with the active template, the oldest creation timestamp wins
/// and the duplicate matching sets are returned in `rest` so normal rollout
/// logic can scale them down and remove them.
pub fn filter_game_server_sets_by_active(
    fleet: &Fleet,
    game_server_sets: &[GameServerSet],
) -> (Option<GameServerSet>, Vec<GameServerSet>) {
    let mut active: Option<GameServerSet> = None;
    let mut rest = Vec::new();

    for game_server_set in game_server_sets {
        if game_server_set.spec.template == fleet.spec.template {
            match active.take() {
                None => active = Some(game_server_set.clone()),
                Some(current) => {
                    if game_server_set_is_older(game_server_set, &current) {
                        rest.push(current);
                        active = Some(game_server_set.clone());
                    } else {
                        rest.push(game_server_set.clone());
                        active = Some(current);
                    }
                }
            }
        } else {
            rest.push(game_server_set.clone());
        }
    }

    (active, rest)
}

/// Adds one GameServerSet counter aggregate into a Fleet aggregate.
///
/// Counter fields use Agones SafeAdd semantics, saturating positive overflow at
/// i64::MAX exactly like the upstream Fleet controller.
pub fn merge_fleet_counters(
    into: &mut BTreeMap<String, AggregatedCounterStatus>,
    from: &BTreeMap<String, AggregatedCounterStatus>,
) {
    for (name, value) in from {
        let entry = into.entry(name.clone()).or_default();
        entry.allocated_capacity =
            safe_add(entry.allocated_capacity, value.allocated_capacity);
        entry.allocated_count =
            safe_add(entry.allocated_count, value.allocated_count);
        entry.capacity = safe_add(entry.capacity, value.capacity);
        entry.count = safe_add(entry.count, value.count);
    }
}

/// Adds one GameServerSet list aggregate into a Fleet aggregate.
///
/// Upstream uses ordinary signed integer addition. Runtime overflow therefore
/// follows two's-complement wrapping; use wrapping_add for release/debug parity.
pub fn merge_fleet_lists(
    into: &mut BTreeMap<String, AggregatedListStatus>,
    from: &BTreeMap<String, AggregatedListStatus>,
) {
    for (name, value) in from {
        let entry = into.entry(name.clone()).or_default();
        entry.allocated_capacity =
            entry.allocated_capacity.wrapping_add(value.allocated_capacity);
        entry.allocated_count =
            entry.allocated_count.wrapping_add(value.allocated_count);
        entry.capacity = entry.capacity.wrapping_add(value.capacity);
        entry.count = entry.count.wrapping_add(value.count);
    }
}

/// Pure port of upstream Fleet controller updateFleetStatus aggregation.
///
/// Every reconciliation recomputes replica/counter/list totals from the owned
/// GameServerSets. Allocation events are accumulated onto the already-persisted
/// Fleet allocation total. With PlayerTracking enabled the players aggregate is
/// reset and recomputed; with it disabled upstream leaves any pre-existing
/// Players status untouched, which is intentionally preserved here.
pub fn compute_fleet_status(
    fleet: &Fleet,
    game_server_sets: &[GameServerSet],
    allocation_delta: i64,
    player_tracking: bool,
) -> FleetStatus {
    let mut status = FleetStatus {
        allocations: fleet.status.allocations.wrapping_add(allocation_delta),
        players: fleet.status.players.clone(),
        ..Default::default()
    };

    for game_server_set in game_server_sets {
        status.replicas = status
            .replicas
            .wrapping_add(game_server_set.status.replicas);
        status.ready_replicas = status
            .ready_replicas
            .wrapping_add(game_server_set.status.ready_replicas);
        status.reserved_replicas = status
            .reserved_replicas
            .wrapping_add(game_server_set.status.reserved_replicas);
        status.allocated_replicas = status
            .allocated_replicas
            .wrapping_add(game_server_set.status.allocated_replicas);
        merge_fleet_counters(
            &mut status.counters,
            &game_server_set.status.counters,
        );
        merge_fleet_lists(
            &mut status.lists,
            &game_server_set.status.lists,
        );
    }

    if player_tracking {
        let mut players = AggregatedPlayerStatus::default();
        for game_server_set in game_server_sets {
            if let Some(value) = game_server_set.status.players.as_ref() {
                players.count = players.count.wrapping_add(value.count);
                players.capacity =
                    players.capacity.wrapping_add(value.capacity);
            }
        }
        status.players = Some(players);
    }

    status
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

    fn set_with_template_name(
        name: &str,
        template_name: &str,
        creation_timestamp: Option<&str>,
    ) -> GameServerSet {
        let mut set = GameServerSet {
            metadata: ObjectMeta {
                name: name.into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                template: GameServerTemplateSpec {
                    metadata: ObjectMeta {
                        name: template_name.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        if let Some(timestamp) = creation_timestamp {
            set.metadata.extensions.insert(
                "creationTimestamp".into(),
                serde_json::json!(timestamp),
            );
        }
        set
    }

    #[test]
    fn active_gameserverset_is_the_template_match() {
        let fleet = Fleet {
            spec: FleetSpec {
                template: GameServerTemplateSpec {
                    metadata: ObjectMeta {
                        name: "current-template".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let old = set_with_template_name(
            "old",
            "old-template",
            Some("2026-01-01T00:00:00Z"),
        );
        let active = set_with_template_name(
            "active",
            "current-template",
            Some("2026-02-01T00:00:00Z"),
        );

        let (selected, rest) =
            filter_game_server_sets_by_active(&fleet, &[old.clone(), active.clone()]);
        assert_eq!(selected, Some(active));
        assert_eq!(rest, vec![old]);
    }

    #[test]
    fn duplicate_active_templates_keep_oldest_and_return_newer_as_rest() {
        let fleet = Fleet {
            spec: FleetSpec {
                template: GameServerTemplateSpec {
                    metadata: ObjectMeta {
                        name: "current-template".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let newer = set_with_template_name(
            "newer",
            "current-template",
            Some("2026-03-01T00:00:00Z"),
        );
        let oldest = set_with_template_name(
            "oldest",
            "current-template",
            Some("2026-01-01T00:00:00Z"),
        );
        let middle = set_with_template_name(
            "middle",
            "current-template",
            Some("2026-02-01T00:00:00Z"),
        );

        let (selected, rest) = filter_game_server_sets_by_active(
            &fleet,
            &[newer.clone(), oldest.clone(), middle.clone()],
        );
        assert_eq!(selected, Some(oldest));
        assert_eq!(rest, vec![newer, middle]);
    }

    #[test]
    fn missing_creation_timestamp_behaves_like_kubernetes_zero_time() {
        let fleet = Fleet {
            spec: FleetSpec {
                template: GameServerTemplateSpec {
                    metadata: ObjectMeta {
                        name: "current-template".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let real_time = set_with_template_name(
            "real-time",
            "current-template",
            Some("2026-01-01T00:00:00Z"),
        );
        let zero_time = set_with_template_name(
            "zero-time",
            "current-template",
            None,
        );

        let (selected, rest) = filter_game_server_sets_by_active(
            &fleet,
            &[real_time.clone(), zero_time.clone()],
        );
        assert_eq!(selected, Some(zero_time));
        assert_eq!(rest, vec![real_time]);
    }

    #[test]
    fn no_template_match_returns_no_active_and_all_sets_as_rest() {
        let fleet = Fleet {
            spec: FleetSpec {
                template: GameServerTemplateSpec {
                    metadata: ObjectMeta {
                        name: "current-template".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let one = set_with_template_name("one", "old-a", None);
        let two = set_with_template_name("two", "old-b", None);

        let (selected, rest) =
            filter_game_server_sets_by_active(&fleet, &[one.clone(), two.clone()]);
        assert!(selected.is_none());
        assert_eq!(rest, vec![one, two]);
    }

    #[test]
    fn fleet_status_aggregates_gameserverset_counts_and_allocations() {
        let fleet = Fleet {
            status: FleetStatus {
                allocations: 40,
                ..Default::default()
            },
            ..Default::default()
        };

        let one = GameServerSet {
            status: crate::gameserverset::GameServerSetStatus {
                replicas: 4,
                ready_replicas: 2,
                reserved_replicas: 1,
                allocated_replicas: 1,
                counters: BTreeMap::from([(
                    "rooms".into(),
                    AggregatedCounterStatus {
                        allocated_count: 1,
                        allocated_capacity: 2,
                        count: 3,
                        capacity: 4,
                    },
                )]),
                lists: BTreeMap::from([(
                    "players".into(),
                    AggregatedListStatus {
                        allocated_count: 2,
                        allocated_capacity: 3,
                        count: 4,
                        capacity: 5,
                    },
                )]),
                ..Default::default()
            },
            ..Default::default()
        };
        let two = GameServerSet {
            status: crate::gameserverset::GameServerSetStatus {
                replicas: 6,
                ready_replicas: 3,
                reserved_replicas: 1,
                allocated_replicas: 2,
                counters: BTreeMap::from([(
                    "rooms".into(),
                    AggregatedCounterStatus {
                        allocated_count: 10,
                        allocated_capacity: 20,
                        count: 30,
                        capacity: 40,
                    },
                )]),
                lists: BTreeMap::from([(
                    "players".into(),
                    AggregatedListStatus {
                        allocated_count: 20,
                        allocated_capacity: 30,
                        count: 40,
                        capacity: 50,
                    },
                )]),
                ..Default::default()
            },
            ..Default::default()
        };

        let status = compute_fleet_status(&fleet, &[one, two], 2, false);
        assert_eq!(status.replicas, 10);
        assert_eq!(status.ready_replicas, 5);
        assert_eq!(status.reserved_replicas, 2);
        assert_eq!(status.allocated_replicas, 3);
        assert_eq!(status.allocations, 42);
        assert_eq!(
            status.counters["rooms"],
            AggregatedCounterStatus {
                allocated_count: 11,
                allocated_capacity: 22,
                count: 33,
                capacity: 44,
            }
        );
        assert_eq!(
            status.lists["players"],
            AggregatedListStatus {
                allocated_count: 22,
                allocated_capacity: 33,
                count: 44,
                capacity: 55,
            }
        );
    }

    #[test]
    fn fleet_counter_merge_saturates_while_list_merge_wraps() {
        let mut counters = BTreeMap::from([(
            "key".into(),
            AggregatedCounterStatus {
                count: i64::MAX,
                ..Default::default()
            },
        )]);
        merge_fleet_counters(
            &mut counters,
            &BTreeMap::from([(
                "key".into(),
                AggregatedCounterStatus {
                    count: 1,
                    ..Default::default()
                },
            )]),
        );
        assert_eq!(counters["key"].count, i64::MAX);

        let mut lists = BTreeMap::from([(
            "key".into(),
            AggregatedListStatus {
                count: i64::MAX,
                ..Default::default()
            },
        )]);
        merge_fleet_lists(
            &mut lists,
            &BTreeMap::from([(
                "key".into(),
                AggregatedListStatus {
                    count: 1,
                    ..Default::default()
                },
            )]),
        );
        assert_eq!(lists["key"].count, i64::MIN);
    }

    #[test]
    fn player_tracking_recomputes_when_enabled_and_preserves_when_disabled() {
        let fleet = Fleet {
            status: FleetStatus {
                players: Some(AggregatedPlayerStatus {
                    count: 99,
                    capacity: 100,
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        let sets = vec![
            GameServerSet {
                status: crate::gameserverset::GameServerSetStatus {
                    players: Some(AggregatedPlayerStatus {
                        count: 3,
                        capacity: 10,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            },
            GameServerSet {
                status: crate::gameserverset::GameServerSetStatus {
                    players: Some(AggregatedPlayerStatus {
                        count: 4,
                        capacity: 20,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            },
        ];

        assert_eq!(
            compute_fleet_status(&fleet, &sets, 0, true).players,
            Some(AggregatedPlayerStatus {
                count: 7,
                capacity: 30,
            })
        );
        assert_eq!(
            compute_fleet_status(&fleet, &sets, 0, false).players,
            fleet.status.players
        );
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
