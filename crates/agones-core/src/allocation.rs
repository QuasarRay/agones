use std::collections::BTreeMap;

use agones_macros::AgonesStringEnum;
use serde::{Deserialize, Serialize};

use crate::{
    CounterStatus, GameServerState, GameServerStatusPort, ListStatus, ObjectMeta, Priority,
    SchedulingStrategy, GAME_SERVER_PRIORITY_ASCENDING,
};

#[derive(Clone, Debug, PartialEq, Eq, AgonesStringEnum)]
pub enum GameServerAllocationState {
    #[agones(value = "Allocated")]
    Allocated,
    #[agones(value = "UnAllocated")]
    Unallocated,
    #[agones(value = "Contention")]
    Contention,
    #[agones(unknown)]
    Unknown(String),
}

impl Default for GameServerAllocationState {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelectorRequirement {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelector {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub match_labels: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<LabelSelectorRequirement>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterSelector {
    pub min_count: i64,
    pub max_count: i64,
    pub min_available: i64,
    pub max_available: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSelector {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub contains_value: String,
    pub min_available: i64,
    pub max_available: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerSelector {
    #[serde(flatten)]
    pub label_selector: LabelSelector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_server_state: Option<GameServerState>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, CounterSelector>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub lists: BTreeMap<String, ListSelector>,
}

impl GameServerSelector {
    pub fn apply_defaults(&mut self) {
        if self.game_server_state.is_none() {
            self.game_server_state = Some(GameServerState::Ready);
        }
    }

    pub fn matches(&self, game_server: &crate::GameServer) -> bool {
        if !self.label_selector.matches(&game_server.metadata.labels) {
            return false;
        }

        if let Some(state) = &self.game_server_state {
            if game_server.status.state.as_ref() != Some(state) {
                return false;
            }
        }

        if !self.counters.is_empty() && !self.match_counters(game_server) {
            return false;
        }
        if !self.lists.is_empty() && !self.match_lists(game_server) {
            return false;
        }
        true
    }

    fn match_counters(&self, game_server: &crate::GameServer) -> bool {
        let Some(counters) = game_server.status.counters.as_ref() else {
            return false;
        };

        self.counters.iter().all(|(name, selector)| {
            let Some(status) = counters.get(name) else {
                return false;
            };
            let available = status.capacity.wrapping_sub(status.count);
            if available < selector.min_available
                || (selector.max_available != 0 && available > selector.max_available)
            {
                return false;
            }
            if status.count < selector.min_count
                || (selector.max_count != 0 && status.count > selector.max_count)
            {
                return false;
            }
            true
        })
    }

    fn match_lists(&self, game_server: &crate::GameServer) -> bool {
        let Some(lists) = game_server.status.lists.as_ref() else {
            return false;
        };

        self.lists.iter().all(|(name, selector)| {
            let Some(status) = lists.get(name) else {
                return false;
            };
            let available = status
                .capacity
                .wrapping_sub(status.values.len() as i64);
            if available < selector.min_available
                || (selector.max_available != 0 && available > selector.max_available)
            {
                return false;
            }
            selector.contains_value.is_empty()
                || status
                    .values
                    .iter()
                    .any(|value| value == &selector.contains_value)
        })
    }
}

impl LabelSelector {
    pub fn matches(&self, labels: &BTreeMap<String, String>) -> bool {
        if !self
            .match_labels
            .iter()
            .all(|(key, value)| labels.get(key) == Some(value))
        {
            return false;
        }

        self.match_expressions.iter().all(|requirement| {
            let current = labels.get(&requirement.key);
            match requirement.operator.as_str() {
                "In" => current.is_some_and(|value| requirement.values.contains(value)),
                "NotIn" => current.is_none_or(|value| !requirement.values.contains(value)),
                "Exists" => current.is_some(),
                "DoesNotExist" => current.is_none(),
                invalid => panic!(
                    "GameServerSelector validation required before matching: unsupported label selector operator {invalid}"
                ),
            }
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAction {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_values: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delete_values: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiClusterSetting {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub policy_selector: LabelSelector,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaPatch {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerAllocationSpec {
    #[serde(default)]
    pub multi_cluster_setting: MultiClusterSetting,
    #[serde(default)]
    pub required: GameServerSelector,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred: Vec<GameServerSelector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priorities: Vec<Priority>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<GameServerSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<SchedulingStrategy>,
    #[serde(default)]
    pub metadata: MetaPatch,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, CounterAction>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub lists: BTreeMap<String, ListAction>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddress {
    #[serde(rename = "type")]
    pub address_type: String,
    pub address: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerMetadata {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerAllocationStatus {
    #[serde(default)]
    pub state: GameServerAllocationState,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub game_server_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<GameServerStatusPort>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub address: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<NodeAddress>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GameServerMetadata>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, CounterStatus>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub lists: BTreeMap<String, ListStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerAllocation {
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: GameServerAllocationSpec,
    #[serde(default)]
    pub status: GameServerAllocationStatus,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerAllocationList {
    #[serde(default)]
    pub items: Vec<GameServerAllocation>,
}

impl GameServerAllocation {
    pub fn apply_defaults(&mut self) {
        if self.spec.scheduling.is_none() {
            self.spec.scheduling = Some(SchedulingStrategy::Packed);
        }

        for priority in &mut self.spec.priorities {
            if priority.order.is_empty() {
                priority.order = GAME_SERVER_PRIORITY_ASCENDING.to_owned();
            }
        }

        if self.spec.selectors.is_empty() {
            self.spec.required.apply_defaults();
            for selector in &mut self.spec.preferred {
                selector.apply_defaults();
            }
        } else {
            for selector in &mut self.spec.selectors {
                selector.apply_defaults();
            }
        }
    }

    pub fn converter(&mut self) {
        if self.spec.selectors.is_empty() {
            let mut selectors = self.spec.preferred.clone();
            selectors.push(self.spec.required.clone());
            self.spec.selectors = selectors;
        }
    }

    pub fn sort_key(&self) -> Result<u64, serde_json::Error> {
        let bytes = serde_json::to_vec(&(
            self.spec.scheduling.as_ref().map(AsRef::<str>::as_ref),
            &self.spec.priorities,
        ))?;

        // Stable FNV-1a. The Go implementation uses hashstructure, but the
        // behavioral contract is equivalence over Scheduling + Priorities,
        // not a particular numeric hash value.
        let mut hash = 0xcbf29ce484222325u64;
        for byte in bytes {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        Ok(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_server_with_labels(labels: &[(&str, &str)]) -> crate::GameServer {
        crate::GameServer {
            metadata: ObjectMeta {
                labels: labels
                    .iter()
                    .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
                    .collect(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn selector_matching_matches_original_label_and_state_cases() {
        let blue = GameServerSelector {
            label_selector: LabelSelector {
                match_labels: BTreeMap::from([("colour".into(), "blue".into())]),
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(GameServerSelector::default().matches(&crate::GameServer::default()));
        assert!(!blue.matches(&crate::GameServer::default()));
        assert!(blue.matches(&game_server_with_labels(&[("colour", "blue")])));
        assert!(!blue.matches(&game_server_with_labels(&[("colour", "purple")])));

        let two_labels = GameServerSelector {
            label_selector: LabelSelector {
                match_labels: BTreeMap::from([
                    ("colour".into(), "blue".into()),
                    ("animal".into(), "frog".into()),
                ]),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(two_labels.matches(&game_server_with_labels(&[
            ("colour", "blue"),
            ("animal", "frog"),
        ])));
        assert!(!two_labels.matches(&game_server_with_labels(&[
            ("colour", "blue"),
            ("animal", "cat"),
        ])));

        let state_selector = GameServerSelector {
            game_server_state: Some(GameServerState::Allocated),
            ..Default::default()
        };
        let allocated = crate::GameServer {
            status: crate::GameServerStatus {
                state: Some(GameServerState::Allocated),
                ..Default::default()
            },
            ..Default::default()
        };
        let ready = crate::GameServer {
            status: crate::GameServerStatus {
                state: Some(GameServerState::Ready),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(state_selector.matches(&allocated));
        assert!(!state_selector.matches(&ready));
    }

    #[test]
    fn selector_matching_preserves_counter_range_semantics() {
        let make = |count, capacity| crate::GameServer {
            status: crate::GameServerStatus {
                counters: Some(BTreeMap::from([(
                    "value".into(),
                    CounterStatus { count, capacity },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };

        let available = GameServerSelector {
            counters: BTreeMap::from([(
                "value".into(),
                CounterSelector {
                    min_available: 1,
                    max_available: 1000,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(available.matches(&make(10, 1000)));

        let minimum = GameServerSelector {
            counters: BTreeMap::from([(
                "value".into(),
                CounterSelector {
                    min_available: 100,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(!minimum.matches(&make(999, 1000)));

        let maximum = GameServerSelector {
            counters: BTreeMap::from([(
                "value".into(),
                CounterSelector {
                    min_available: 1,
                    max_available: 100,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(!maximum.matches(&make(0, 1000)));

        let unlimited_max_count = GameServerSelector {
            counters: BTreeMap::from([(
                "value".into(),
                CounterSelector {
                    min_count: 1,
                    max_count: 0,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(unlimited_max_count.matches(&make(10, 1000)));

        let bounded_count = GameServerSelector {
            counters: BTreeMap::from([(
                "value".into(),
                CounterSelector {
                    max_count: 10,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(!bounded_count.matches(&make(11, 100)));

        let missing = GameServerSelector {
            counters: BTreeMap::from([("missing".into(), CounterSelector::default())]),
            ..Default::default()
        };
        assert!(!missing.matches(&make(1, 100)));
        assert!(!missing.matches(&crate::GameServer::default()));
    }

    #[test]
    fn selector_matching_preserves_list_capacity_and_contains_semantics() {
        let make = |capacity, values: &[&str]| crate::GameServer {
            status: crate::GameServerStatus {
                lists: Some(BTreeMap::from([(
                    "value".into(),
                    ListStatus {
                        capacity,
                        values: values.iter().map(|value| (*value).to_owned()).collect(),
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };

        let capacity = GameServerSelector {
            lists: BTreeMap::from([(
                "value".into(),
                ListSelector {
                    min_available: 1,
                    max_available: 3,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(capacity.matches(&make(3, &["one", "two"])));

        let below_min = GameServerSelector {
            lists: BTreeMap::from([(
                "value".into(),
                ListSelector {
                    min_available: 1,
                    max_available: 1000,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(!below_min.matches(&make(3, &["one", "two", "three"])));

        let above_max = GameServerSelector {
            lists: BTreeMap::from([(
                "value".into(),
                ListSelector {
                    min_available: 1,
                    max_available: 10,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(!above_max.matches(&make(1000, &["one", "two", "three"])));

        let contains = GameServerSelector {
            lists: BTreeMap::from([(
                "value".into(),
                ListSelector {
                    contains_value: "baz1".into(),
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(contains.matches(&make(3, &["baz1", "baz2", "baz3"])));

        let case_sensitive = GameServerSelector {
            lists: BTreeMap::from([(
                "value".into(),
                ListSelector {
                    contains_value: "BAZ1".into(),
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        assert!(!case_sensitive.matches(&make(3, &["baz1", "baz2", "baz3"])));
        assert!(!case_sensitive.matches(&crate::GameServer::default()));
    }

    #[test]
    fn label_selector_expressions_follow_kubernetes_operator_semantics() {
        let labels = BTreeMap::from([
            ("region".into(), "us".into()),
            ("tier".into(), "prod".into()),
        ]);
        let selector = LabelSelector {
            match_expressions: vec![
                LabelSelectorRequirement {
                    key: "region".into(),
                    operator: "In".into(),
                    values: vec!["us".into(), "eu".into()],
                },
                LabelSelectorRequirement {
                    key: "tier".into(),
                    operator: "NotIn".into(),
                    values: vec!["dev".into()],
                },
                LabelSelectorRequirement {
                    key: "region".into(),
                    operator: "Exists".into(),
                    values: Vec::new(),
                },
                LabelSelectorRequirement {
                    key: "debug".into(),
                    operator: "DoesNotExist".into(),
                    values: Vec::new(),
                },
            ],
            ..Default::default()
        };
        assert!(selector.matches(&labels));
    }

    #[test]
    fn allocation_defaults_match_original_go_behavior() {
        let mut allocation = GameServerAllocation::default();
        allocation.apply_defaults();
        assert_eq!(
            allocation.spec.scheduling,
            Some(SchedulingStrategy::Packed)
        );
        assert_eq!(
            allocation.spec.required.game_server_state,
            Some(GameServerState::Ready)
        );

        let mut allocation = GameServerAllocation {
            spec: GameServerAllocationSpec {
                scheduling: Some(SchedulingStrategy::Distributed),
                priorities: vec![
                    Priority {
                        priority_type: "List".into(),
                        ..Default::default()
                    },
                    Priority {
                        priority_type: "Counter".into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        };
        allocation.apply_defaults();
        assert_eq!(
            allocation.spec.scheduling,
            Some(SchedulingStrategy::Distributed)
        );
        assert!(allocation
            .spec
            .priorities
            .iter()
            .all(|priority| priority.order == GAME_SERVER_PRIORITY_ASCENDING));
    }

    #[test]
    fn selector_defaults_are_idempotent_and_preserve_existing_values() {
        let mut selector = GameServerSelector::default();
        selector.apply_defaults();
        assert_eq!(selector.game_server_state, Some(GameServerState::Ready));
        let once = selector.clone();
        selector.apply_defaults();
        assert_eq!(selector, once);

        let mut selector = GameServerSelector {
            game_server_state: Some(GameServerState::Allocated),
            counters: BTreeMap::from([(
                "foo".into(),
                CounterSelector {
                    min_available: 1,
                    max_available: 10,
                    ..Default::default()
                },
            )]),
            lists: BTreeMap::from([(
                "bar".into(),
                ListSelector {
                    min_available: 2,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        let expected = selector.clone();
        selector.apply_defaults();
        assert_eq!(selector, expected);
        selector.apply_defaults();
        assert_eq!(selector, expected);
    }

    #[test]
    fn new_selectors_take_precedence_for_defaulting() {
        let mut allocation = GameServerAllocation {
            spec: GameServerAllocationSpec {
                required: GameServerSelector {
                    game_server_state: Some(GameServerState::Allocated),
                    ..Default::default()
                },
                preferred: vec![GameServerSelector {
                    game_server_state: Some(GameServerState::Allocated),
                    ..Default::default()
                }],
                selectors: vec![GameServerSelector::default()],
                ..Default::default()
            },
            ..Default::default()
        };
        allocation.apply_defaults();

        assert_eq!(
            allocation.spec.selectors[0].game_server_state,
            Some(GameServerState::Ready)
        );
        assert_eq!(
            allocation.spec.required.game_server_state,
            Some(GameServerState::Allocated)
        );
        assert_eq!(
            allocation.spec.preferred[0].game_server_state,
            Some(GameServerState::Allocated)
        );
    }

    #[test]
    fn converter_orders_preferred_before_required_and_is_idempotent() {
        let preferred = GameServerSelector {
            label_selector: LabelSelector {
                match_labels: BTreeMap::from([("check".into(), "blue".into())]),
                ..Default::default()
            },
            ..Default::default()
        };
        let required = GameServerSelector {
            label_selector: LabelSelector {
                match_labels: BTreeMap::from([("required".into(), "yes".into())]),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut allocation = GameServerAllocation {
            spec: GameServerAllocationSpec {
                preferred: vec![preferred.clone()],
                required: required.clone(),
                ..Default::default()
            },
            ..Default::default()
        };

        allocation.converter();
        assert_eq!(allocation.spec.selectors, vec![preferred, required]);
        let once = allocation.spec.selectors.clone();
        allocation.converter();
        assert_eq!(allocation.spec.selectors, once);
    }

    #[test]
    fn sort_key_ignores_selectors_but_tracks_scheduling_and_priorities() {
        let base = GameServerAllocation {
            spec: GameServerAllocationSpec {
                scheduling: Some(SchedulingStrategy::Packed),
                priorities: vec![Priority {
                    priority_type: "List".into(),
                    key: "foo".into(),
                    order: "Descending".into(),
                }],
                ..Default::default()
            },
            ..Default::default()
        };
        let mut only_selector_differs = base.clone();
        only_selector_differs.spec.selectors = vec![GameServerSelector {
            label_selector: LabelSelector {
                match_labels: BTreeMap::from([("foo".into(), "bar".into())]),
                ..Default::default()
            },
            ..Default::default()
        }];
        assert_eq!(
            base.sort_key().unwrap(),
            only_selector_differs.sort_key().unwrap()
        );

        let mut scheduling_differs = base.clone();
        scheduling_differs.spec.scheduling = Some(SchedulingStrategy::Distributed);
        assert_ne!(
            base.sort_key().unwrap(),
            scheduling_differs.sort_key().unwrap()
        );

        let mut priority_differs = base.clone();
        priority_differs.spec.priorities[0].priority_type = "Counter".into();
        assert_ne!(base.sort_key().unwrap(), priority_differs.sort_key().unwrap());

        assert_eq!(
            GameServerAllocation::default().sort_key().unwrap(),
            GameServerAllocation {
                spec: GameServerAllocationSpec {
                    priorities: Vec::new(),
                    ..Default::default()
                },
                ..Default::default()
            }
            .sort_key()
            .unwrap()
        );
    }

    #[test]
    fn allocation_state_preserves_unknown_wire_values() {
        let state = GameServerAllocationState::from("FutureState");
        assert_eq!(state.as_str(), "FutureState");
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, r#""FutureState""#);
    }
}
