use std::collections::{BTreeMap, HashMap};

use agones_core::{
    CounterAction, CounterSelector, CounterStatus, GameServerAllocation,
    GameServerAllocationState, GameServerMetadata, GameServerSelector,
    GameServerStatusPort, LabelSelector, LabelSelectorRequirement, ListAction,
    ListSelector, ListStatus, MetaPatch, MultiClusterSetting, NodeAddress, ObjectMeta,
    Priority, SchedulingStrategy, GAME_SERVER_PRIORITY_ASCENDING,
    GAME_SERVER_PRIORITY_COUNTER, GAME_SERVER_PRIORITY_DESCENDING,
    GAME_SERVER_PRIORITY_LIST,
};
use summer_grpc::tonic::{Code, Status};

use crate::allocationpb::{
    allocation_request, allocation_response, game_server_selector,
    label_match_expressions, priority, AllocationRequest, AllocationResponse,
    CounterAction as PbCounterAction, CounterSelector as PbCounterSelector,
    GameServerSelector as PbGameServerSelector, LabelMatchExpressions,
    LabelSelector as PbLabelSelector, ListAction as PbListAction,
    ListSelector as PbListSelector, MetaPatch as PbMetaPatch,
    MultiClusterSetting as PbMultiClusterSetting, Priority as PbPriority,
};

fn btree<K: Ord, V>(map: HashMap<K, V>) -> BTreeMap<K, V> {
    map.into_iter().collect()
}

fn hash<K: std::hash::Hash + Eq, V>(map: BTreeMap<K, V>) -> HashMap<K, V> {
    map.into_iter().collect()
}

pub fn allocation_request_to_gsa(input: &AllocationRequest) -> GameServerAllocation {
    let mut gsa = GameServerAllocation {
        metadata: ObjectMeta {
            namespace: input.namespace.clone(),
            ..Default::default()
        },
        spec: agones_core::GameServerAllocationSpec {
            preferred: selectors_to_gsa(&input.preferred_game_server_selectors),
            selectors: selectors_to_gsa(&input.game_server_selectors),
            scheduling: Some(allocation_scheduling_to_gsa(input.scheduling)),
            ..Default::default()
        },
        ..Default::default()
    };

    if let Some(setting) = &input.multi_cluster_setting {
        gsa.spec.multi_cluster_setting = MultiClusterSetting {
            enabled: setting.enabled,
            policy_selector: setting
                .policy_selector
                .as_ref()
                .map(label_selector_to_gsa)
                .unwrap_or_default(),
        };
    }

    if let Some(metadata) = input.metadata.as_ref().or(input.meta_patch.as_ref()) {
        gsa.spec.metadata = MetaPatch {
            labels: btree(metadata.labels.clone()),
            annotations: btree(metadata.annotations.clone()),
        };
    }

    if let Some(required) = input.required_game_server_selector.as_ref() {
        gsa.spec.required = selector_to_gsa(required);
    }

    gsa.spec.priorities = priorities_to_gsa(&input.priorities);
    gsa.spec.counters = counter_actions_to_gsa(&input.counters);
    gsa.spec.lists = list_actions_to_gsa(&input.lists);
    gsa
}

pub fn gsa_to_allocation_request(input: &GameServerAllocation) -> AllocationRequest {
    let selectors = selectors_to_pb(&input.spec.selectors);
    let required = selectors.last().cloned();
    let preferred = selectors
        .get(..selectors.len().saturating_sub(1))
        .unwrap_or_default()
        .to_vec();

    let metadata = PbMetaPatch {
        labels: hash(input.spec.metadata.labels.clone()),
        annotations: hash(input.spec.metadata.annotations.clone()),
    };

    AllocationRequest {
        namespace: input.metadata.namespace.clone(),
        scheduling: gsa_scheduling_to_allocation(input.spec.scheduling.as_ref()) as i32,
        game_server_selectors: selectors,
        preferred_game_server_selectors: preferred,
        required_game_server_selector: required,
        multi_cluster_setting: Some(PbMultiClusterSetting {
            enabled: input.spec.multi_cluster_setting.enabled,
            policy_selector: input
                .spec
                .multi_cluster_setting
                .enabled
                .then(|| label_selector_to_pb(&input.spec.multi_cluster_setting.policy_selector)),
        }),
        metadata: Some(metadata.clone()),
        // Deprecated, intentionally double-written for cross-version multicluster compatibility.
        meta_patch: Some(metadata),
        priorities: priorities_to_pb(&input.spec.priorities),
        counters: counter_actions_to_pb(&input.spec.counters),
        lists: list_actions_to_pb(&input.spec.lists),
    }
}

fn allocation_scheduling_to_gsa(value: i32) -> SchedulingStrategy {
    match allocation_request::SchedulingStrategy::try_from(value) {
        Ok(allocation_request::SchedulingStrategy::Distributed) => SchedulingStrategy::Distributed,
        _ => SchedulingStrategy::Packed,
    }
}

fn gsa_scheduling_to_allocation(
    value: Option<&SchedulingStrategy>,
) -> allocation_request::SchedulingStrategy {
    match value {
        Some(SchedulingStrategy::Distributed) => allocation_request::SchedulingStrategy::Distributed,
        _ => allocation_request::SchedulingStrategy::Packed,
    }
}

fn label_selector_to_gsa(input: &PbLabelSelector) -> LabelSelector {
    LabelSelector {
        match_labels: btree(input.match_labels.clone()),
        ..Default::default()
    }
}

fn label_selector_to_pb(input: &LabelSelector) -> PbLabelSelector {
    PbLabelSelector {
        match_labels: hash(input.match_labels.clone()),
    }
}

fn match_expressions_to_gsa(
    input: &[LabelMatchExpressions],
) -> Vec<LabelSelectorRequirement> {
    input
        .iter()
        .map(|expr| LabelSelectorRequirement {
            key: expr.key.clone(),
            values: expr.values.clone(),
            operator: match label_match_expressions::Operator::try_from(expr.operator) {
                Ok(label_match_expressions::Operator::NotIn) => "NotIn",
                Ok(label_match_expressions::Operator::Exists) => "Exists",
                Ok(label_match_expressions::Operator::DoesNotExist) => "DoesNotExist",
                _ => "In",
            }
            .into(),
        })
        .collect()
}

fn match_expressions_to_pb(
    input: &[LabelSelectorRequirement],
) -> Vec<LabelMatchExpressions> {
    input
        .iter()
        .map(|req| LabelMatchExpressions {
            key: req.key.clone(),
            values: req.values.clone(),
            operator: match req.operator.as_str() {
                "NotIn" => label_match_expressions::Operator::NotIn,
                "Exists" => label_match_expressions::Operator::Exists,
                "DoesNotExist" => label_match_expressions::Operator::DoesNotExist,
                _ => label_match_expressions::Operator::In,
            } as i32,
        })
        .collect()
}

fn selector_to_gsa(input: &PbGameServerSelector) -> GameServerSelector {
    GameServerSelector {
        label_selector: LabelSelector {
            match_labels: btree(input.match_labels.clone()),
            match_expressions: match_expressions_to_gsa(&input.match_expressions),
        },
        game_server_state: Some(
            match game_server_selector::GameServerState::try_from(input.game_server_state) {
                Ok(game_server_selector::GameServerState::Allocated) =>
                    agones_core::GameServerState::Allocated,
                _ => agones_core::GameServerState::Ready,
            },
        ),
        counters: input
            .counters
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    CounterSelector {
                        min_count: v.min_count,
                        max_count: v.max_count,
                        min_available: v.min_available,
                        max_available: v.max_available,
                    },
                )
            })
            .collect(),
        lists: input
            .lists
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    ListSelector {
                        contains_value: v.contains_value.clone(),
                        min_available: v.min_available,
                        max_available: v.max_available,
                    },
                )
            })
            .collect(),
    }
}

fn selector_to_pb(input: &GameServerSelector) -> PbGameServerSelector {
    PbGameServerSelector {
        match_labels: hash(input.label_selector.match_labels.clone()),
        match_expressions: match_expressions_to_pb(&input.label_selector.match_expressions),
        game_server_state: match input.game_server_state.as_ref() {
            Some(agones_core::GameServerState::Allocated) =>
                game_server_selector::GameServerState::Allocated,
            _ => game_server_selector::GameServerState::Ready,
        } as i32,
        counters: input
            .counters
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    PbCounterSelector {
                        min_count: v.min_count,
                        max_count: v.max_count,
                        min_available: v.min_available,
                        max_available: v.max_available,
                    },
                )
            })
            .collect(),
        lists: input
            .lists
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    PbListSelector {
                        contains_value: v.contains_value.clone(),
                        min_available: v.min_available,
                        max_available: v.max_available,
                    },
                )
            })
            .collect(),
    }
}

fn selectors_to_gsa(input: &[PbGameServerSelector]) -> Vec<GameServerSelector> {
    input.iter().map(selector_to_gsa).collect()
}

fn selectors_to_pb(input: &[GameServerSelector]) -> Vec<PbGameServerSelector> {
    input.iter().map(selector_to_pb).collect()
}

pub fn gsa_to_allocation_response(
    input: &GameServerAllocation,
    unallocated_code: Code,
) -> Result<AllocationResponse, Status> {
    state_to_error(&input.status.state, unallocated_code)?;

    Ok(AllocationResponse {
        game_server_name: input.status.game_server_name.clone(),
        address: input.status.address.clone(),
        addresses: input
            .status
            .addresses
            .iter()
            .map(|a| allocation_response::GameServerStatusAddress {
                r#type: a.address_type.clone(),
                address: a.address.clone(),
            })
            .collect(),
        node_name: input.status.node_name.clone(),
        ports: input
            .status
            .ports
            .iter()
            .map(|p| allocation_response::GameServerStatusPort {
                name: p.name.clone(),
                port: p.port,
            })
            .collect(),
        source: input.status.source.clone(),
        metadata: input.status.metadata.as_ref().map(|m| {
            allocation_response::GameServerMetadata {
                labels: hash(m.labels.clone()),
                annotations: hash(m.annotations.clone()),
            }
        }),
        counters: input
            .status
            .counters
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    allocation_response::CounterStatus {
                        count: Some(v.count),
                        capacity: Some(v.capacity),
                    },
                )
            })
            .collect(),
        lists: input
            .status
            .lists
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    allocation_response::ListStatus {
                        values: v.values.iter().cloned().collect(),
                        capacity: Some(v.capacity),
                    },
                )
            })
            .collect(),
    })
}

pub fn allocation_response_to_gsa(
    input: &AllocationResponse,
    source: &str,
) -> GameServerAllocation {
    GameServerAllocation {
        status: agones_core::GameServerAllocationStatus {
            state: GameServerAllocationState::Allocated,
            game_server_name: input.game_server_name.clone(),
            address: input.address.clone(),
            addresses: input
                .addresses
                .iter()
                .map(|a| NodeAddress {
                    address_type: a.r#type.clone(),
                    address: a.address.clone(),
                })
                .collect(),
            node_name: input.node_name.clone(),
            ports: input
                .ports
                .iter()
                .map(|p| GameServerStatusPort {
                    name: p.name.clone(),
                    port: p.port,
                })
                .collect(),
            source: source.into(),
            metadata: input.metadata.as_ref().map(|m| GameServerMetadata {
                labels: btree(m.labels.clone()),
                annotations: btree(m.annotations.clone()),
            }),
            counters: input
                .counters
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        CounterStatus {
                            count: v.count.unwrap_or_default(),
                            capacity: v.capacity.unwrap_or_default(),
                        },
                    )
                })
                .collect(),
            lists: input
                .lists
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ListStatus {
                            values: v.values.clone().into_iter().collect(),
                            capacity: v.capacity.unwrap_or_default(),
                        },
                    )
                })
                .collect(),
        },
        ..Default::default()
    }
}

fn state_to_error(state: &GameServerAllocationState, unallocated_code: Code) -> Result<(), Status> {
    match state {
        GameServerAllocationState::Allocated => Ok(()),
        GameServerAllocationState::Unallocated => Err(Status::new(
            unallocated_code,
            "there is no available GameServer to allocate",
        )),
        GameServerAllocationState::Contention => Err(Status::new(
            Code::Aborted,
            "too many concurrent requests have overwhelmed the system",
        )),
        GameServerAllocationState::Unknown(_) => Err(Status::unknown("unknown issue")),
    }
}

fn priorities_to_gsa(input: &[PbPriority]) -> Vec<Priority> {
    input
        .iter()
        .map(|p| Priority {
            priority_type: match priority::Type::try_from(p.r#type) {
                Ok(priority::Type::List) => GAME_SERVER_PRIORITY_LIST,
                _ => GAME_SERVER_PRIORITY_COUNTER,
            }
            .into(),
            key: p.key.clone(),
            order: match priority::Order::try_from(p.order) {
                Ok(priority::Order::Descending) => GAME_SERVER_PRIORITY_DESCENDING,
                _ => GAME_SERVER_PRIORITY_ASCENDING,
            }
            .into(),
        })
        .collect()
}

fn priorities_to_pb(input: &[Priority]) -> Vec<PbPriority> {
    input
        .iter()
        .map(|p| PbPriority {
            r#type: if p.priority_type == GAME_SERVER_PRIORITY_LIST {
                priority::Type::List
            } else {
                priority::Type::Counter
            } as i32,
            key: p.key.clone(),
            order: if p.order == GAME_SERVER_PRIORITY_DESCENDING {
                priority::Order::Descending
            } else {
                priority::Order::Ascending
            } as i32,
        })
        .collect()
}

fn counter_actions_to_gsa(
    input: &HashMap<String, PbCounterAction>,
) -> BTreeMap<String, CounterAction> {
    input
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                CounterAction {
                    action: v.action.clone(),
                    amount: v.amount,
                    capacity: v.capacity,
                },
            )
        })
        .collect()
}

fn counter_actions_to_pb(
    input: &BTreeMap<String, CounterAction>,
) -> HashMap<String, PbCounterAction> {
    input
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                PbCounterAction {
                    action: v.action.clone(),
                    amount: v.amount,
                    capacity: v.capacity,
                },
            )
        })
        .collect()
}

fn list_actions_to_gsa(
    input: &HashMap<String, PbListAction>,
) -> BTreeMap<String, ListAction> {
    input
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                ListAction {
                    add_values: v.add_values.clone(),
                    delete_values: v.delete_values.clone(),
                    capacity: v.capacity,
                },
            )
        })
        .collect()
}

fn list_actions_to_pb(
    input: &BTreeMap<String, ListAction>,
) -> HashMap<String, PbListAction> {
    input
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                PbListAction {
                    add_values: v.add_values.clone(),
                    capacity: v.capacity,
                    delete_values: v.delete_values.clone(),
                },
            )
        })
        .collect()
}