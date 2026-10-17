use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gameserver::{
    safe_add, validate_object_meta, CounterStatus, GameServer, GameServerSpec, GameServerState,
    ListStatus, ObjectMeta, Priority, SchedulingStrategy, ValidationError, ValidationErrorKind,
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
    kind = "GameServerSet",
    plural = "gameserversets"
)]
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

/// Creates the zero-valued aggregate counter map that upstream GameServerSet
/// reconciliation publishes even before any GameServers exist.
///
/// Keys come from the GameServer template so configured counters remain visible
/// in status with zero counts/capacity.
pub fn initial_game_server_set_counter_status(
    game_server_set: &GameServerSet,
) -> BTreeMap<String, AggregatedCounterStatus> {
    game_server_set
        .spec
        .template
        .spec
        .counters
        .as_ref()
        .into_iter()
        .flat_map(|counters| counters.keys())
        .cloned()
        .map(|name| (name, AggregatedCounterStatus::default()))
        .collect()
}

/// Creates the zero-valued aggregate list map that upstream GameServerSet
/// reconciliation publishes even before any GameServers exist.
pub fn initial_game_server_set_list_status(
    game_server_set: &GameServerSet,
) -> BTreeMap<String, AggregatedListStatus> {
    game_server_set
        .spec
        .template
        .spec
        .lists
        .as_ref()
        .into_iter()
        .flat_map(|lists| lists.keys())
        .cloned()
        .map(|name| (name, AggregatedListStatus::default()))
        .collect()
}

/// Aggregates one GameServer's counters into GameServerSet status.
///
/// Counter addition intentionally uses Agones `SafeAdd` semantics so positive
/// overflow saturates at `i64::MAX`, matching the Go controller.
pub fn aggregate_game_server_set_counters(
    aggregate: &mut BTreeMap<String, AggregatedCounterStatus>,
    counters: Option<&BTreeMap<String, CounterStatus>>,
    state: Option<&GameServerState>,
) {
    let Some(counters) = counters else {
        return;
    };

    let allocated = matches!(state, Some(GameServerState::Allocated));
    for (name, value) in counters {
        let entry = aggregate.entry(name.clone()).or_default();
        entry.count = safe_add(entry.count, value.count);
        entry.capacity = safe_add(entry.capacity, value.capacity);
        if allocated {
            entry.allocated_count = safe_add(entry.allocated_count, value.count);
            entry.allocated_capacity =
                safe_add(entry.allocated_capacity, value.capacity);
        }
    }
}

/// Aggregates one GameServer's lists into GameServerSet status.
///
/// Agones counts list values, including duplicates. Go's integer addition wraps
/// on signed overflow, so wrapping arithmetic is used for exact parity.
pub fn aggregate_game_server_set_lists(
    aggregate: &mut BTreeMap<String, AggregatedListStatus>,
    lists: Option<&BTreeMap<String, ListStatus>>,
    state: Option<&GameServerState>,
) {
    let Some(lists) = lists else {
        return;
    };

    let allocated = matches!(state, Some(GameServerState::Allocated));
    for (name, value) in lists {
        let count = value.values.len() as i64;
        let entry = aggregate.entry(name.clone()).or_default();
        entry.count = entry.count.wrapping_add(count);
        entry.capacity = entry.capacity.wrapping_add(value.capacity);
        if allocated {
            entry.allocated_count = entry.allocated_count.wrapping_add(count);
            entry.allocated_capacity =
                entry.allocated_capacity.wrapping_add(value.capacity);
        }
    }
}

/// Pure Rust port of upstream `gameserversets.computeStatus`.
///
/// Deleting/Shutdown GameServers contribute only to `shutdown_replicas`.
/// Active GameServers contribute to replica counts plus counter/list
/// aggregation. Player aggregation remains feature-gated explicitly so the
/// caller can mirror the upstream runtime feature gate without coupling
/// `agones-core` to a runtime.
pub fn compute_game_server_set_status(
    game_server_set: &GameServerSet,
    game_servers: &[GameServer],
    player_tracking: bool,
) -> GameServerSetStatus {
    let mut status = GameServerSetStatus {
        counters: initial_game_server_set_counter_status(game_server_set),
        lists: initial_game_server_set_list_status(game_server_set),
        ..Default::default()
    };

    for game_server in game_servers {
        if game_server.is_being_deleted() {
            status.shutdown_replicas += 1;
            continue;
        }

        status.replicas += 1;
        match game_server.status.state.as_ref() {
            Some(GameServerState::Ready) => status.ready_replicas += 1,
            Some(GameServerState::Allocated) => status.allocated_replicas += 1,
            Some(GameServerState::Reserved) => status.reserved_replicas += 1,
            _ => {}
        }

        aggregate_game_server_set_counters(
            &mut status.counters,
            game_server.status.counters.as_ref(),
            game_server.status.state.as_ref(),
        );
        aggregate_game_server_set_lists(
            &mut status.lists,
            game_server.status.lists.as_ref(),
            game_server.status.state.as_ref(),
        );
    }

    if player_tracking {
        let mut players = AggregatedPlayerStatus::default();
        for game_server in game_servers {
            if game_server.is_being_deleted()
                || !matches!(
                    game_server.status.state.as_ref(),
                    Some(
                        GameServerState::Ready
                            | GameServerState::Reserved
                            | GameServerState::Allocated
                    )
                )
            {
                continue;
            }

            if let Some(player_status) = game_server.status.players.as_ref() {
                players.capacity =
                    players.capacity.wrapping_add(player_status.capacity);
                players.count = players.count.wrapping_add(player_status.count);
            }
        }
        status.players = Some(players);
    }

    status
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

    fn game_server_with_state(state: GameServerState) -> GameServer {
        GameServer {
            status: crate::gameserver::GameServerStatus {
                state: Some(state),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn computes_gameserverset_replica_status_like_upstream() {
        let set = GameServerSet::default();
        let servers = vec![
            game_server_with_state(GameServerState::Creating),
            game_server_with_state(GameServerState::Ready),
            game_server_with_state(GameServerState::Allocated),
            game_server_with_state(GameServerState::Allocated),
            game_server_with_state(GameServerState::Reserved),
        ];

        let status = compute_game_server_set_status(&set, &servers, false);
        assert_eq!(status.replicas, 5);
        assert_eq!(status.ready_replicas, 1);
        assert_eq!(status.allocated_replicas, 2);
        assert_eq!(status.reserved_replicas, 1);
        assert_eq!(status.shutdown_replicas, 0);
        assert!(status.counters.is_empty());
        assert!(status.lists.is_empty());
        assert!(status.players.is_none());
    }

    #[test]
    fn gameserverset_status_aggregates_counters_with_safe_add() {
        let mut allocated = game_server_with_state(GameServerState::Allocated);
        allocated.status.counters = Some(BTreeMap::from([
            (
                "firstCounter".into(),
                CounterStatus {
                    count: 5,
                    capacity: 10,
                },
            ),
            (
                "secondCounter".into(),
                CounterStatus {
                    count: 100,
                    capacity: 1000,
                },
            ),
            (
                "fullCounter".into(),
                CounterStatus {
                    count: i64::MAX,
                    capacity: i64::MAX,
                },
            ),
        ]));

        let mut reserved = game_server_with_state(GameServerState::Reserved);
        reserved.status.counters = Some(BTreeMap::from([
            (
                "firstCounter".into(),
                CounterStatus {
                    count: 10,
                    capacity: 15,
                },
            ),
            (
                "fullCounter".into(),
                CounterStatus {
                    count: i64::MAX,
                    capacity: i64::MAX,
                },
            ),
        ]));

        let mut creating = game_server_with_state(GameServerState::Creating);
        creating.status.counters = Some(BTreeMap::from([
            (
                "firstCounter".into(),
                CounterStatus {
                    count: 20,
                    capacity: 30,
                },
            ),
            (
                "secondCounter".into(),
                CounterStatus {
                    count: 100,
                    capacity: 1000,
                },
            ),
        ]));

        let mut ready = game_server_with_state(GameServerState::Ready);
        ready.status.counters = Some(BTreeMap::from([
            (
                "firstCounter".into(),
                CounterStatus {
                    count: 15,
                    capacity: 30,
                },
            ),
            (
                "secondCounter".into(),
                CounterStatus {
                    count: 20,
                    capacity: 200,
                },
            ),
        ]));

        let status = compute_game_server_set_status(
            &GameServerSet::default(),
            &[allocated, reserved, creating, ready],
            false,
        );

        assert_eq!(
            status.counters["firstCounter"],
            AggregatedCounterStatus {
                allocated_count: 5,
                allocated_capacity: 10,
                count: 50,
                capacity: 85,
            }
        );
        assert_eq!(
            status.counters["secondCounter"],
            AggregatedCounterStatus {
                allocated_count: 100,
                allocated_capacity: 1000,
                count: 220,
                capacity: 2200,
            }
        );
        assert_eq!(
            status.counters["fullCounter"],
            AggregatedCounterStatus {
                allocated_count: i64::MAX,
                allocated_capacity: i64::MAX,
                count: i64::MAX,
                capacity: i64::MAX,
            }
        );
    }

    #[test]
    fn gameserverset_status_aggregates_lists_and_counts_duplicates() {
        let mut allocated = game_server_with_state(GameServerState::Allocated);
        allocated.status.lists = Some(BTreeMap::from([
            (
                "firstList".into(),
                ListStatus {
                    capacity: 10,
                    values: vec!["a".into(), "a".into()],
                },
            ),
            (
                "secondList".into(),
                ListStatus {
                    capacity: 1000,
                    values: vec!["1".into(), "2".into()],
                },
            ),
        ]));

        let mut reserved = game_server_with_state(GameServerState::Reserved);
        reserved.status.lists = Some(BTreeMap::from([(
            "firstList".into(),
            ListStatus {
                capacity: 15,
                values: vec!["c".into()],
            },
        )]));

        let mut creating = game_server_with_state(GameServerState::Creating);
        creating.status.lists = Some(BTreeMap::from([
            (
                "firstList".into(),
                ListStatus {
                    capacity: 30,
                    values: vec!["d".into()],
                },
            ),
            (
                "secondList".into(),
                ListStatus {
                    capacity: 1000,
                    values: vec!["3".into()],
                },
            ),
        ]));

        let mut ready = game_server_with_state(GameServerState::Ready);
        ready.status.lists = Some(BTreeMap::from([
            (
                "firstList".into(),
                ListStatus {
                    capacity: 30,
                    values: Vec::new(),
                },
            ),
            (
                "secondList".into(),
                ListStatus {
                    capacity: 100,
                    values: vec!["4".into(), "5".into(), "6".into()],
                },
            ),
        ]));

        let status = compute_game_server_set_status(
            &GameServerSet::default(),
            &[allocated, reserved, creating, ready],
            false,
        );

        assert_eq!(
            status.lists["firstList"],
            AggregatedListStatus {
                allocated_count: 2,
                allocated_capacity: 10,
                count: 4,
                capacity: 85,
            }
        );
        assert_eq!(
            status.lists["secondList"],
            AggregatedListStatus {
                allocated_count: 2,
                allocated_capacity: 1000,
                count: 6,
                capacity: 2100,
            }
        );
    }

    #[test]
    fn configured_counters_and_lists_are_zero_initialized_without_servers() {
        let mut set = GameServerSet::default();
        set.spec.template.spec.counters = Some(BTreeMap::from([
            ("firstCounter".into(), CounterStatus::default()),
            ("secondCounter".into(), CounterStatus::default()),
        ]));
        set.spec.template.spec.lists = Some(BTreeMap::from([
            ("firstList".into(), ListStatus::default()),
            ("secondList".into(), ListStatus::default()),
        ]));

        let status = compute_game_server_set_status(&set, &[], false);
        assert_eq!(status.counters.len(), 2);
        assert_eq!(status.lists.len(), 2);
        assert!(
            status
                .counters
                .values()
                .all(|value| value == &AggregatedCounterStatus::default())
        );
        assert!(
            status
                .lists
                .values()
                .all(|value| value == &AggregatedListStatus::default())
        );
    }

    #[test]
    fn deleting_and_shutdown_servers_count_only_as_shutdown_replicas() {
        let mut deleting = game_server_with_state(GameServerState::Ready);
        deleting.metadata.deletion_timestamp = Some("2026-10-18T00:00:00Z".into());
        deleting.status.counters = Some(BTreeMap::from([(
            "ignored".into(),
            CounterStatus {
                count: 5,
                capacity: 10,
            },
        )]));

        let shutdown = game_server_with_state(GameServerState::Shutdown);
        let status = compute_game_server_set_status(
            &GameServerSet::default(),
            &[deleting, shutdown],
            false,
        );

        assert_eq!(status.shutdown_replicas, 2);
        assert_eq!(status.replicas, 0);
        assert_eq!(status.ready_replicas, 0);
        assert!(status.counters.is_empty());
    }

    #[test]
    fn player_tracking_aggregates_only_ready_reserved_and_allocated_servers() {
        let mut ready = game_server_with_state(GameServerState::Ready);
        ready.status.players = Some(crate::gameserver::PlayerStatus {
            count: 2,
            capacity: 10,
            ..Default::default()
        });
        let mut allocated = game_server_with_state(GameServerState::Allocated);
        allocated.status.players = Some(crate::gameserver::PlayerStatus {
            count: 3,
            capacity: 20,
            ..Default::default()
        });
        let mut creating = game_server_with_state(GameServerState::Creating);
        creating.status.players = Some(crate::gameserver::PlayerStatus {
            count: 99,
            capacity: 99,
            ..Default::default()
        });

        let status = compute_game_server_set_status(
            &GameServerSet::default(),
            &[ready, allocated, creating],
            true,
        );
        assert_eq!(
            status.players,
            Some(AggregatedPlayerStatus {
                count: 5,
                capacity: 30,
            })
        );

        let disabled = compute_game_server_set_status(
            &GameServerSet::default(),
            &[],
            false,
        );
        assert!(disabled.players.is_none());
    }

}
