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
