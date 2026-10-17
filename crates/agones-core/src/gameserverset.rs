use std::{cmp::Ordering, collections::{BTreeMap, HashMap}};

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use crate::gameserver::{
    safe_add, validate_object_meta, CounterStatus, GameServer, GameServerSpec, GameServerState,
    ListStatus, ObjectMeta, Priority, SchedulingStrategy, ValidationError, ValidationErrorKind,
};

pub const GAME_SERVER_SET_GAME_SERVER_LABEL: &str = "agones.dev/gameserverset";
pub const FLEET_NAME_LABEL: &str = "agones.dev/fleet";
pub const GAME_SERVER_ERRORED_AT_ANNOTATION: &str = "agones.dev/errored-at";
pub const GAME_SERVER_ERROR_DELETION_DELAY_SECONDS: i64 = 30;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NodeCount {
    pub ready: i64,
    pub allocated: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GameServerSetReconciliationAction {
    pub num_servers_to_add: i32,
    pub to_delete: Vec<GameServer>,
    pub is_partial: bool,
}

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


fn priority_order(
    left: &GameServer,
    right: &GameServer,
    priorities: &[Priority],
) -> Option<Ordering> {
    left.compare_count_and_list_priorities(priorities, right)
        .map(|left_before_right| {
            if left_before_right {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
}

fn game_server_creation_timestamp(game_server: &GameServer) -> Option<DateTime<Utc>> {
    game_server
        .metadata
        .extensions
        .get("creationTimestamp")
        .and_then(serde_json::Value::as_str)
        .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
        .map(|value| value.with_timezone(&Utc))
}

/// Sorts scale-down candidates with the same strategy boundary as upstream
/// `gameserversets.SortGameServersByStrategy`.
///
/// Packed scheduling empties the least-full node first. Distributed scheduling
/// uses counter/list priorities first and then creation timestamp. Kubernetes
/// metadata fields that are not explicitly modeled by `ObjectMeta`, including
/// `creationTimestamp`, remain available through its flattened extension map.
pub fn sort_game_servers_by_strategy(
    strategy: &SchedulingStrategy,
    game_servers: &mut [GameServer],
    counts: &HashMap<String, NodeCount>,
    priorities: &[Priority],
) {
    if matches!(strategy, SchedulingStrategy::Packed) {
        game_servers.sort_by(|left, right| {
            let left_count = counts.get(left.status.node_name.as_str());
            let right_count = counts.get(right.status.node_name.as_str());

            match (left_count, right_count) {
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (None, None) => {
                    return left.metadata.name.cmp(&right.metadata.name);
                }
                (Some(left_count), Some(right_count)) => {
                    if left.status.node_name == right.status.node_name {
                        if left.is_before_ready()
                            && right.status.state == Some(GameServerState::Ready)
                        {
                            return Ordering::Less;
                        }
                        if right.is_before_ready()
                            && left.status.state == Some(GameServerState::Ready)
                        {
                            return Ordering::Greater;
                        }
                    }

                    let left_total = left_count.allocated + left_count.ready;
                    let right_total = right_count.allocated + right_count.ready;
                    match left_total.cmp(&right_total) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }

                    if left.status.node_name == right.status.node_name {
                        if let Some(ordering) = priority_order(left, right, priorities) {
                            return ordering;
                        }
                        return left.metadata.name.cmp(&right.metadata.name);
                    }

                    left.status.node_name.cmp(&right.status.node_name)
                }
            }
        });
        return;
    }

    game_servers.sort_by(|left, right| {
        if let Some(ordering) = priority_order(left, right, priorities) {
            return ordering;
        }

        match (
            game_server_creation_timestamp(left),
            game_server_creation_timestamp(right),
        ) {
            (Some(left), Some(right)) => left
                .cmp(&right)
                .then_with(|| left.to_rfc3339().cmp(&right.to_rfc3339())),
            // Kubernetes' zero-value creation timestamp sorts before real
            // timestamps in the upstream comparator.
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (None, None) => left.metadata.name.cmp(&right.metadata.name),
        }
    });
}

/// Deterministic form of upstream `shouldDeleteErroredGameServer`.
///
/// The Go controller uses `time.Now()` implicitly. Passing `now` explicitly
/// makes the same rule directly testable and suitable for Kani/Bolero-style
/// pure-domain verification.
pub fn should_delete_errored_game_server_at(
    game_server: &GameServer,
    now: DateTime<Utc>,
) -> bool {
    let Some(errored_at) = game_server
        .metadata
        .annotations
        .get(GAME_SERVER_ERRORED_AT_ANNOTATION)
    else {
        return true;
    };

    let Ok(errored_at) = DateTime::parse_from_rfc3339(errored_at) else {
        return true;
    };
    let errored_at = errored_at.with_timezone(&Utc);

    now.signed_duration_since(errored_at).num_seconds()
        >= GAME_SERVER_ERROR_DELETION_DELAY_SECONDS
}

/// Pure port of upstream `gameserversets.computeReconciliationAction`.
///
/// It computes only intent: how many GameServers to create, which existing
/// GameServers should transition to Shutdown, and whether another immediate
/// reconciliation is required. API writes remain a Rusternetes responsibility.
pub fn compute_game_server_set_reconciliation_action(
    strategy: &SchedulingStrategy,
    game_servers: &[GameServer],
    counts: &HashMap<String, NodeCount>,
    target_replica_count: i32,
    max_creations: i32,
    max_deletions: i32,
    max_pending: i32,
    priorities: &[Priority],
    now: DateTime<Utc>,
) -> GameServerSetReconciliationAction {
    let mut up_count = 0i32;
    let mut delete_count = 0i32;
    let mut pod_pending_count = 0i32;
    let mut potential_deletions = Vec::new();
    let mut to_delete = Vec::new();

    // Pass 1: allocated/reserved servers are not normally deletable and always
    // count toward the target.
    for game_server in game_servers {
        if !game_server.is_deletable() {
            up_count += 1;
        }
    }

    // Pass 2: classify all remaining states.
    for game_server in game_servers {
        if !game_server.is_deletable() || game_server.is_being_deleted() {
            continue;
        }

        let mut handle_up = false;
        match game_server.status.state.as_ref() {
            Some(
                GameServerState::PortAllocation
                | GameServerState::Creating
                | GameServerState::Starting
                | GameServerState::Scheduled,
            ) => {
                pod_pending_count += 1;
                handle_up = true;
            }
            Some(
                GameServerState::RequestReady
                | GameServerState::Ready
                | GameServerState::Reserved,
            ) => {
                handle_up = true;
            }
            Some(GameServerState::Error) => {
                if should_delete_errored_game_server_at(game_server, now) {
                    to_delete.push(game_server.clone());
                    delete_count -= 1;
                } else {
                    pod_pending_count += 1;
                    handle_up = true;
                }
            }
            Some(GameServerState::Unhealthy) => {
                to_delete.push(game_server.clone());
                delete_count -= 1;
            }
            _ => {
                // Unknown/unrecognized states are conservatively considered up,
                // matching the Go controller.
                handle_up = true;
            }
        }

        if handle_up {
            if up_count >= target_replica_count {
                delete_count += 1;
            } else {
                up_count += 1;
            }
            potential_deletions.push(game_server.clone());
        }
    }

    let mut is_partial = false;
    let mut num_servers_to_add = 0i32;

    if up_count < target_replica_count {
        num_servers_to_add = target_replica_count - up_count;
        let original = num_servers_to_add;

        num_servers_to_add = num_servers_to_add.min(max_creations.max(0));

        if num_servers_to_add + pod_pending_count > max_pending {
            num_servers_to_add = (max_pending - pod_pending_count).max(0);
        }

        if original != num_servers_to_add {
            is_partial = true;
        }
    }

    if delete_count > 0 {
        sort_game_servers_by_strategy(
            strategy,
            &mut potential_deletions,
            counts,
            priorities,
        );
        to_delete.extend(
            potential_deletions
                .into_iter()
                .take(delete_count as usize),
        );
    }

    let max_deletions = max_deletions.max(0) as usize;
    if to_delete.len() > max_deletions {
        to_delete.truncate(max_deletions);
        is_partial = true;
    }

    GameServerSetReconciliationAction {
        num_servers_to_add,
        to_delete,
        is_partial,
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


    fn named_game_server_with_state(name: &str, state: GameServerState) -> GameServer {
        let mut game_server = game_server_with_state(state);
        game_server.metadata.name = name.into();
        game_server
    }

    #[test]
    fn reconciliation_action_matches_upstream_batch_and_state_rules() {
        let now = Utc::now();
        let distributed = SchedulingStrategy::Distributed;
        let counts = HashMap::new();

        let cases = vec![
            (
                "empty",
                Vec::<GameServer>::new(),
                0,
                0,
                0,
                false,
            ),
            (
                "add",
                vec![game_server_with_state(GameServerState::Ready)],
                3,
                2,
                0,
                false,
            ),
            (
                "creation batch limit",
                vec![game_server_with_state(GameServerState::Ready)],
                30,
                3,
                0,
                true,
            ),
            (
                "pending limit",
                vec![
                    game_server_with_state(GameServerState::Creating),
                    game_server_with_state(GameServerState::PortAllocation),
                ],
                30,
                1,
                0,
                true,
            ),
            (
                "delete",
                vec![
                    game_server_with_state(GameServerState::Ready),
                    game_server_with_state(GameServerState::Reserved),
                    game_server_with_state(GameServerState::Ready),
                ],
                1,
                0,
                2,
                false,
            ),
            (
                "ignore allocated scale down",
                vec![
                    game_server_with_state(GameServerState::Ready),
                    game_server_with_state(GameServerState::Allocated),
                    game_server_with_state(GameServerState::Allocated),
                ],
                0,
                0,
                1,
                false,
            ),
            (
                "replace unhealthy",
                vec![
                    game_server_with_state(GameServerState::Ready),
                    game_server_with_state(GameServerState::Unhealthy),
                    game_server_with_state(GameServerState::Unhealthy),
                ],
                3,
                2,
                2,
                false,
            ),
            (
                "replace errors without timestamp",
                vec![
                    game_server_with_state(GameServerState::Ready),
                    game_server_with_state(GameServerState::Error),
                    game_server_with_state(GameServerState::Error),
                ],
                3,
                2,
                2,
                false,
            ),
        ];

        for (description, servers, target, want_add, want_delete, want_partial) in cases {
            let action = compute_game_server_set_reconciliation_action(
                &distributed,
                &servers,
                &counts,
                target,
                3,
                3,
                3,
                &[],
                now,
            );
            assert_eq!(
                action.num_servers_to_add, want_add,
                "{description}: additions"
            );
            assert_eq!(
                action.to_delete.len(), want_delete,
                "{description}: deletions"
            );
            assert_eq!(
                action.is_partial, want_partial,
                "{description}: partial"
            );
        }
    }

    #[test]
    fn reconciliation_action_caps_deletion_batches() {
        let servers = (0..6)
            .map(|index| {
                named_game_server_with_state(
                    format!("gs{index}").as_str(),
                    GameServerState::Ready,
                )
            })
            .collect::<Vec<_>>();

        let action = compute_game_server_set_reconciliation_action(
            &SchedulingStrategy::Distributed,
            &servers,
            &HashMap::new(),
            1,
            1000,
            3,
            1000,
            &[],
            Utc::now(),
        );

        assert_eq!(action.to_delete.len(), 3);
        assert!(action.is_partial);
    }

    #[test]
    fn reconciliation_action_ignores_already_deleting_servers_when_replacing() {
        let mut deleting_one = game_server_with_state(GameServerState::Unhealthy);
        deleting_one.metadata.deletion_timestamp =
            Some("2026-10-18T00:00:00Z".into());
        let mut deleting_two = deleting_one.clone();
        deleting_two.metadata.name = "second".into();

        let servers = vec![
            deleting_one,
            deleting_two,
            game_server_with_state(GameServerState::Ready),
        ];

        let action = compute_game_server_set_reconciliation_action(
            &SchedulingStrategy::Distributed,
            &servers,
            &HashMap::new(),
            4,
            3,
            3,
            3,
            &[],
            Utc::now(),
        );

        assert_eq!(action.num_servers_to_add, 3);
        assert!(action.to_delete.is_empty());
        assert!(!action.is_partial);
    }

    #[test]
    fn packed_scale_down_empties_least_full_nodes_first() {
        let mut gs1 = named_game_server_with_state("gs1", GameServerState::Ready);
        gs1.status.node_name = "node3".into();
        let mut gs2 = named_game_server_with_state("gs2", GameServerState::Ready);
        gs2.status.node_name = "node1".into();
        let mut gs3 = named_game_server_with_state("gs3", GameServerState::Ready);
        gs3.status.node_name = "node3".into();
        let gs4 = named_game_server_with_state("gs4", GameServerState::Ready);

        let counts = HashMap::from([
            (
                "node1".into(),
                NodeCount {
                    ready: 1,
                    allocated: 0,
                },
            ),
            (
                "node3".into(),
                NodeCount {
                    ready: 2,
                    allocated: 0,
                },
            ),
        ]);

        let action = compute_game_server_set_reconciliation_action(
            &SchedulingStrategy::Packed,
            &[gs1, gs2, gs3, gs4],
            &counts,
            2,
            1000,
            1000,
            1000,
            &[],
            Utc::now(),
        );

        let names = action
            .to_delete
            .iter()
            .map(|game_server| game_server.metadata.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["gs4", "gs2"]);
    }

    #[test]
    fn distributed_scale_down_uses_oldest_creation_timestamp_first() {
        fn with_created(name: &str, created: &str) -> GameServer {
            let mut game_server =
                named_game_server_with_state(name, GameServerState::Ready);
            game_server.metadata.extensions.insert(
                "creationTimestamp".into(),
                serde_json::Value::String(created.into()),
            );
            game_server
        }

        let servers = vec![
            with_created("gs1", "2026-10-18T00:00:10Z"),
            with_created("gs2", "2026-10-18T00:00:00Z"),
            with_created("gs3", "2026-10-18T00:00:40Z"),
            with_created("gs4", "2026-10-18T00:00:30Z"),
        ];

        let action = compute_game_server_set_reconciliation_action(
            &SchedulingStrategy::Distributed,
            &servers,
            &HashMap::new(),
            2,
            1000,
            1000,
            1000,
            &[],
            Utc::now(),
        );

        let names = action
            .to_delete
            .iter()
            .map(|game_server| game_server.metadata.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["gs2", "gs1"]);
    }

    #[test]
    fn distributed_sort_applies_counter_priority_before_timestamp() {
        let mut low_capacity =
            named_game_server_with_state("low", GameServerState::Ready);
        low_capacity.status.counters = Some(BTreeMap::from([(
            "bar".into(),
            CounterStatus {
                count: 0,
                capacity: 100,
            },
        )]));
        low_capacity.metadata.extensions.insert(
            "creationTimestamp".into(),
            serde_json::Value::String("2026-10-18T00:00:30Z".into()),
        );

        let mut high_capacity =
            named_game_server_with_state("high", GameServerState::Ready);
        high_capacity.status.counters = Some(BTreeMap::from([(
            "bar".into(),
            CounterStatus {
                count: 0,
                capacity: 1000,
            },
        )]));
        high_capacity.metadata.extensions.insert(
            "creationTimestamp".into(),
            serde_json::Value::String("2026-10-18T00:00:00Z".into()),
        );

        let priorities = [Priority {
            priority_type: crate::gameserver::GAME_SERVER_PRIORITY_COUNTER.into(),
            key: "bar".into(),
            order: crate::gameserver::GAME_SERVER_PRIORITY_DESCENDING.into(),
        }];
        let mut servers = vec![low_capacity, high_capacity];
        sort_game_servers_by_strategy(
            &SchedulingStrategy::Distributed,
            &mut servers,
            &HashMap::new(),
            &priorities,
        );

        assert_eq!(servers[0].metadata.name, "high");
        assert_eq!(servers[1].metadata.name, "low");
    }

    #[test]
    fn errored_gameserver_deletion_delay_matches_upstream() {
        let now = DateTime::parse_from_rfc3339("2026-10-18T00:01:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let mut game_server = game_server_with_state(GameServerState::Error);

        assert!(should_delete_errored_game_server_at(&game_server, now));

        game_server.metadata.annotations.insert(
            GAME_SERVER_ERRORED_AT_ANNOTATION.into(),
            "not-a-timestamp".into(),
        );
        assert!(should_delete_errored_game_server_at(&game_server, now));

        game_server.metadata.annotations.insert(
            GAME_SERVER_ERRORED_AT_ANNOTATION.into(),
            "2026-10-18T00:00:31Z".into(),
        );
        assert!(!should_delete_errored_game_server_at(&game_server, now));

        game_server.metadata.annotations.insert(
            GAME_SERVER_ERRORED_AT_ANNOTATION.into(),
            "2026-10-18T00:00:30Z".into(),
        );
        assert!(should_delete_errored_game_server_at(&game_server, now));
    }

}
