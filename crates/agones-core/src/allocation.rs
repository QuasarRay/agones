use std::collections::BTreeMap;

use agones_macros::AgonesStringEnum;
use serde::{Deserialize, Serialize};

use crate::{
    CounterStatus, GameServerState, GameServerStatusPort, ListStatus, ObjectMeta, Priority,
    SchedulingStrategy, ValidationError, ValidationErrorKind, GAME_SERVER_PRIORITY_ASCENDING,
    GAME_SERVER_PRIORITY_COUNTER, GAME_SERVER_PRIORITY_DESCENDING, GAME_SERVER_PRIORITY_LIST,
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

    pub fn validate(&self, path: &str) -> Vec<ValidationError> {
        let mut errors = self.label_selector.validate(path);

        if let Some(state) = &self.game_server_state {
            if !matches!(state, GameServerState::Allocated | GameServerState::Ready) {
                errors.push(invalid(
                    format!("{path}.gameServerState"),
                    state.as_str(),
                    "GameServerState must be either Allocated or Ready",
                ));
            }
        }

        errors.extend(validate_counter_selectors(
            &self.counters,
            &format!("{path}.counters"),
        ));
        errors.extend(validate_list_selectors(
            &self.lists,
            &format!("{path}.lists"),
        ));
        errors
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


fn invalid(path: impl Into<String>, bad_value: impl Into<String>, detail: impl Into<String>) -> ValidationError {
    ValidationError {
        origin: String::new(),
        covered_by_declarative: false,
        kind: ValidationErrorKind::Invalid,
        path: path.into(),
        bad_value: Some(bad_value.into()),
        detail: detail.into(),
    }
}

fn not_supported(path: impl Into<String>, bad_value: impl Into<String>, detail: impl Into<String>) -> ValidationError {
    ValidationError {
        origin: String::new(),
        covered_by_declarative: false,
        kind: ValidationErrorKind::NotSupported,
        path: path.into(),
        bad_value: Some(bad_value.into()),
        detail: detail.into(),
    }
}

fn valid_label_component(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }
    value.len() <= 63
        && value.chars().next().is_some_and(|c| c.is_ascii_alphanumeric())
        && value.chars().last().is_some_and(|c| c.is_ascii_alphanumeric())
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

fn valid_qualified_name(value: &str) -> bool {
    let name = value.rsplit('/').next().unwrap_or(value);
    !name.is_empty() && valid_label_component(name)
}

impl LabelSelector {
    fn validate(&self, path: &str) -> Vec<ValidationError> {
        let mut invalid_selector = false;

        for (key, value) in &self.match_labels {
            if !valid_qualified_name(key) || !valid_label_component(value) {
                invalid_selector = true;
            }
        }

        for requirement in &self.match_expressions {
            if !valid_qualified_name(&requirement.key) {
                invalid_selector = true;
            }
            match requirement.operator.as_str() {
                "In" | "NotIn" => {
                    if requirement.values.is_empty()
                        || requirement
                            .values
                            .iter()
                            .any(|value| !valid_label_component(value))
                    {
                        invalid_selector = true;
                    }
                }
                "Exists" | "DoesNotExist" => {
                    if !requirement.values.is_empty() {
                        invalid_selector = true;
                    }
                }
                _ => invalid_selector = true,
            }
        }

        if invalid_selector {
            vec![invalid(
                format!("{path}.labelSelector"),
                serde_json::to_string(self).unwrap_or_default(),
                "Error converting label selector: invalid label selector",
            )]
        } else {
            Vec::new()
        }
    }
}

fn validate_counter_selectors(
    counters: &BTreeMap<String, CounterSelector>,
    path: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for (key, selector) in counters {
        let key_path = format!("{path}[{key}]");
        if selector.min_count < 0 {
            errors.push(invalid(
                format!("{key_path}.minCount"),
                selector.min_count.to_string(),
                "must be greater than or equal to 0",
            ));
        }
        if selector.max_count < 0 {
            errors.push(invalid(
                format!("{key_path}.maxCount"),
                selector.max_count.to_string(),
                "must be greater than or equal to 0",
            ));
        }
        if selector.max_count != 0 && selector.max_count < selector.min_count {
            errors.push(invalid(
                key_path.clone(),
                selector.max_count.to_string(),
                format!(
                    "maxCount must zero or greater than minCount {}",
                    selector.min_count
                ),
            ));
        }
        if selector.min_available < 0 {
            errors.push(invalid(
                format!("{key_path}.minAvailable"),
                selector.min_available.to_string(),
                "must be greater than or equal to 0",
            ));
        }
        if selector.max_available < 0 {
            errors.push(invalid(
                format!("{key_path}.maxAvailable"),
                selector.max_available.to_string(),
                "must be greater than or equal to 0",
            ));
        }
        if selector.max_available != 0 && selector.max_available < selector.min_available {
            errors.push(invalid(
                key_path,
                selector.max_available.to_string(),
                format!(
                    "maxAvailable must zero or greater than minAvailable {}",
                    selector.min_available
                ),
            ));
        }
    }
    errors
}

fn validate_list_selectors(
    lists: &BTreeMap<String, ListSelector>,
    path: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for (key, selector) in lists {
        let key_path = format!("{path}[{key}]");
        if selector.min_available < 0 {
            errors.push(invalid(
                format!("{key_path}.minAvailable"),
                selector.min_available.to_string(),
                "must be greater than or equal to 0",
            ));
        }
        if selector.max_available < 0 {
            errors.push(invalid(
                format!("{key_path}.maxAvailable"),
                selector.max_available.to_string(),
                "must be greater than or equal to 0",
            ));
        }
        if selector.max_available != 0 && selector.max_available < selector.min_available {
            errors.push(invalid(
                key_path,
                selector.max_available.to_string(),
                format!(
                    "maxAvailable must zero or greater than minAvailable {}",
                    selector.min_available
                ),
            ));
        }
    }
    errors
}

pub fn validate_priorities(priorities: &[Priority], path: &str) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for (index, priority) in priorities.iter().enumerate() {
        let key_path = format!("{path}[{index}]");
        if priority.priority_type != GAME_SERVER_PRIORITY_COUNTER
            && priority.priority_type != GAME_SERVER_PRIORITY_LIST
        {
            errors.push(invalid(
                key_path.clone(),
                priority.priority_type.clone(),
                r#"type must be "Counter" or "List""#,
            ));
        }
        if priority.key.is_empty() {
            errors.push(invalid(
                key_path.clone(),
                priority.priority_type.clone(),
                "key must not be nil",
            ));
        }
        if priority.order != GAME_SERVER_PRIORITY_ASCENDING
            && priority.order != GAME_SERVER_PRIORITY_DESCENDING
        {
            errors.push(invalid(
                key_path,
                priority.order.clone(),
                r#"order must be "Ascending" or "Descending""#,
            ));
        }
    }
    errors
}

pub fn validate_counter_actions(
    counters: &BTreeMap<String, CounterAction>,
    path: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for (key, action) in counters {
        let key_path = format!("{path}[{key}]");
        if action.amount.is_some_and(|amount| amount < 0) {
            errors.push(invalid(
                format!("{key_path}.amount"),
                action.amount.unwrap().to_string(),
                "must be greater than or equal to 0",
            ));
        }
        if action.capacity.is_some_and(|capacity| capacity < 0) {
            errors.push(invalid(
                format!("{key_path}.capacity"),
                action.capacity.unwrap().to_string(),
                "must be greater than or equal to 0",
            ));
        }
        if action.amount.is_some() && action.action.is_none() {
            errors.push(invalid(
                key_path.clone(),
                "null",
                r#"action must be "Increment" or "Decrement" if the amount is not nil"#,
            ));
        }
        if action.amount.is_none() && action.action.is_some() {
            errors.push(invalid(
                key_path,
                "null",
                "amount must not be nil if action is not nil",
            ));
        }
    }
    errors
}

pub fn validate_list_actions(
    lists: &BTreeMap<String, ListAction>,
    path: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for (key, action) in lists {
        if action.capacity.is_some_and(|capacity| capacity < 0) {
            errors.push(invalid(
                format!("{path}[{key}].capacity"),
                action.capacity.unwrap().to_string(),
                "must be greater than or equal to 0",
            ));
        }
    }
    errors
}

impl MetaPatch {
    pub fn validate(&self, path: &str) -> Vec<ValidationError> {
        let metadata = ObjectMeta {
            labels: self.labels.clone(),
            annotations: self.annotations.clone(),
            ..Default::default()
        };
        crate::gameserver::validate_object_meta(&metadata, path)
    }
}


fn push_action_error(errors: &mut Vec<String>, result: Result<(), String>) {
    if let Err(error) = result {
        errors.push(error);
    }
}

fn finish_action_errors(errors: Vec<String>) -> Result<(), String> {
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

impl CounterAction {
    /// Applies allocation-time Counter mutations in the same order as Agones:
    /// capacity first, then count. A failure does not suppress later actions.
    pub fn counter_actions(
        &self,
        counter: &str,
        game_server: &mut crate::GameServer,
    ) -> Result<(), String> {
        let mut errors = Vec::new();

        if let Some(capacity) = self.capacity {
            push_action_error(
                &mut errors,
                game_server.update_counter_capacity(counter, capacity),
            );
        }

        if let (Some(action), Some(amount)) = (self.action.as_deref(), self.amount) {
            push_action_error(
                &mut errors,
                game_server.update_count(counter, action, amount),
            );
        }

        finish_action_errors(errors)
    }
}

impl ListAction {
    /// Applies allocation-time List mutations in Agones order:
    /// capacity, append, delete. All requested actions are attempted.
    pub fn list_actions(
        &self,
        list: &str,
        game_server: &mut crate::GameServer,
        max_capacity: i64,
    ) -> Result<(), String> {
        let mut errors = Vec::new();

        if let Some(capacity) = self.capacity {
            push_action_error(
                &mut errors,
                game_server.update_list_capacity(list, capacity, max_capacity),
            );
        }

        if !self.add_values.is_empty() {
            push_action_error(
                &mut errors,
                game_server.append_list_values(list, Some(&self.add_values)),
            );
        }

        if !self.delete_values.is_empty() {
            push_action_error(
                &mut errors,
                game_server.delete_list_values(list, Some(&self.delete_values)),
            );
        }

        finish_action_errors(errors)
    }
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

    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        match self.spec.scheduling.as_ref() {
            Some(SchedulingStrategy::Packed | SchedulingStrategy::Distributed) => {}
            Some(value) => errors.push(not_supported(
                "spec.scheduling",
                value.as_str(),
                "supported values: Packed, Distributed",
            )),
            None => errors.push(not_supported(
                "spec.scheduling",
                "",
                "supported values: Packed, Distributed",
            )),
        }

        errors.extend(self.spec.required.validate("spec.required"));
        for (index, selector) in self.spec.preferred.iter().enumerate() {
            errors.extend(selector.validate(&format!("spec.preferred[{index}]")));
        }
        for (index, selector) in self.spec.selectors.iter().enumerate() {
            errors.extend(selector.validate(&format!("spec.selectors[{index}]")));
        }

        errors.extend(validate_priorities(&self.spec.priorities, "spec.priorities"));
        errors.extend(validate_counter_actions(&self.spec.counters, "spec.counters"));
        errors.extend(validate_list_actions(&self.spec.lists, "spec.lists"));
        errors.extend(self.spec.metadata.validate("spec.metadata"));
        errors
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
    fn selector_validation_matches_state_counter_and_list_invariants() {
        let valid = GameServerSelector {
            game_server_state: Some(GameServerState::Allocated),
            ..Default::default()
        };
        assert!(valid.validate("fieldName").is_empty());

        let invalid_state = GameServerSelector {
            game_server_state: Some(GameServerState::Starting),
            ..Default::default()
        };
        let errors = invalid_state.validate("fieldName");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "fieldName.gameServerState");
        assert_eq!(
            errors[0].detail,
            "GameServerState must be either Allocated or Ready"
        );

        let invalid_counter = GameServerSelector {
            counters: BTreeMap::from([(
                "counter".into(),
                CounterSelector {
                    min_available: -1,
                    max_available: -1,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        let errors = invalid_counter.validate("fieldName");
        assert_eq!(errors.len(), 2);
        assert_eq!(
            errors[0].path,
            "fieldName.counters[counter].minAvailable"
        );
        assert_eq!(
            errors[1].path,
            "fieldName.counters[counter].maxAvailable"
        );

        let inverted_counter = GameServerSelector {
            counters: BTreeMap::from([(
                "foo".into(),
                CounterSelector {
                    min_count: 10,
                    max_count: 1,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        let errors = inverted_counter.validate("fieldName");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "fieldName.counters[foo]");
        assert_eq!(
            errors[0].detail,
            "maxCount must zero or greater than minCount 10"
        );

        let invalid_list = GameServerSelector {
            lists: BTreeMap::from([(
                "list".into(),
                ListSelector {
                    min_available: 11,
                    max_available: 2,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };
        let errors = invalid_list.validate("fieldName");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "fieldName.lists[list]");
        assert_eq!(
            errors[0].detail,
            "maxAvailable must zero or greater than minAvailable 11"
        );
    }

    #[test]
    fn selector_validation_rejects_invalid_label_selector() {
        let selector = GameServerSelector {
            label_selector: LabelSelector {
                match_labels: BTreeMap::from([("$$".into(), "true".into())]),
                ..Default::default()
            },
            ..Default::default()
        };
        let errors = selector.validate("fieldName");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "fieldName.labelSelector");
    }

    #[test]
    fn metadata_patch_validation_matches_agones_paths() {
        let valid = MetaPatch {
            labels: BTreeMap::from([("foo".into(), "bar".into())]),
            annotations: BTreeMap::from([("bar".into(), "anything $$".into())]),
        };
        assert!(valid.validate("spec.metadata").is_empty());

        let mut invalid = valid.clone();
        invalid.labels.insert("$$".into(), "no".into());
        let errors = invalid.validate("spec.metadata");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, "spec.metadata.labels");

        invalid
            .annotations
            .insert("$$".into(), "no".into());
        let errors = invalid.validate("spec.metadata");
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].path, "spec.metadata.labels");
        assert_eq!(errors[1].path, "spec.metadata.annotations");
    }

    #[test]
    fn priority_validation_matches_original_matrix() {
        let valid = vec![
            Priority {
                priority_type: GAME_SERVER_PRIORITY_LIST.into(),
                key: "test".into(),
                order: GAME_SERVER_PRIORITY_ASCENDING.into(),
            },
            Priority {
                priority_type: GAME_SERVER_PRIORITY_COUNTER.into(),
                key: "test".into(),
                order: GAME_SERVER_PRIORITY_DESCENDING.into(),
            },
        ];
        assert!(validate_priorities(&valid, "spec.Priorities").is_empty());

        for invalid_priority in [
            Priority {
                key: "test".into(),
                order: GAME_SERVER_PRIORITY_DESCENDING.into(),
                ..Default::default()
            },
            Priority {
                priority_type: "invalid".into(),
                key: "test".into(),
                order: GAME_SERVER_PRIORITY_DESCENDING.into(),
            },
            Priority {
                priority_type: GAME_SERVER_PRIORITY_COUNTER.into(),
                order: GAME_SERVER_PRIORITY_DESCENDING.into(),
                ..Default::default()
            },
            Priority {
                priority_type: GAME_SERVER_PRIORITY_LIST.into(),
                key: "test".into(),
                ..Default::default()
            },
            Priority {
                priority_type: GAME_SERVER_PRIORITY_LIST.into(),
                key: "test".into(),
                order: "invalid".into(),
            },
        ] {
            assert!(!validate_priorities(&[invalid_priority], "spec.Priorities").is_empty());
        }
    }

    #[test]
    fn counter_and_list_action_validation_matches_original_behavior() {
        let valid_counters = BTreeMap::from([
            (
                "foo".into(),
                CounterAction {
                    action: Some("Increment".into()),
                    amount: Some(10),
                    ..Default::default()
                },
            ),
            (
                "bar".into(),
                CounterAction {
                    capacity: Some(100),
                    ..Default::default()
                },
            ),
            (
                "baz".into(),
                CounterAction {
                    action: Some("Decrement".into()),
                    amount: Some(1000),
                    capacity: Some(0),
                },
            ),
        ]);
        assert!(validate_counter_actions(&valid_counters, "spec.Counters").is_empty());

        for invalid_action in [
            CounterAction {
                action: Some("Increment".into()),
                amount: Some(-1),
                ..Default::default()
            },
            CounterAction {
                capacity: Some(-20),
                ..Default::default()
            },
            CounterAction {
                amount: Some(10),
                ..Default::default()
            },
            CounterAction {
                action: Some("Decrement".into()),
                ..Default::default()
            },
        ] {
            assert!(!validate_counter_actions(
                &BTreeMap::from([("foo".into(), invalid_action)]),
                "spec.Counters",
            )
            .is_empty());
        }

        let arbitrary_action_is_preserved = CounterAction {
            action: Some("NotAnAgonesAction".into()),
            amount: Some(1),
            ..Default::default()
        };
        assert!(validate_counter_actions(
            &BTreeMap::from([("foo".into(), arbitrary_action_is_preserved)]),
            "spec.Counters",
        )
        .is_empty());

        assert!(validate_list_actions(
            &BTreeMap::from([(
                "foo".into(),
                ListAction {
                    add_values: vec!["hello".into(), "world".into()],
                    capacity: Some(10),
                    ..Default::default()
                },
            )]),
            "spec.Lists",
        )
        .is_empty());
        assert!(!validate_list_actions(
            &BTreeMap::from([(
                "foo".into(),
                ListAction {
                    capacity: Some(-20),
                    ..Default::default()
                },
            )]),
            "spec.Lists",
        )
        .is_empty());
    }

    #[test]
    fn allocation_validation_matches_scheduling_and_metadata_behavior() {
        let mut allocation = GameServerAllocation::default();
        allocation.apply_defaults();
        assert!(allocation.validate().is_empty());

        allocation.spec.scheduling = Some(SchedulingStrategy::from("FLERG"));
        let errors = allocation.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, ValidationErrorKind::NotSupported);
        assert_eq!(errors[0].path, "spec.scheduling");

        allocation = GameServerAllocation {
            spec: GameServerAllocationSpec {
                metadata: MetaPatch {
                    labels: BTreeMap::from([("$$".into(), "foo".into())]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        allocation.apply_defaults();
        let errors = allocation.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, ValidationErrorKind::Invalid);
        assert_eq!(errors[0].path, "spec.metadata.labels");
    }

    #[test]
    fn counter_actions_apply_capacity_before_count() {
        let mut game_server = crate::GameServer {
            status: crate::GameServerStatus {
                counters: Some(BTreeMap::from([(
                    "heroes".into(),
                    CounterStatus {
                        count: 11,
                        capacity: 100,
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        let action = CounterAction {
            action: Some("Decrement".into()),
            amount: Some(10),
            capacity: Some(10),
        };

        action
            .counter_actions("heroes", &mut game_server)
            .unwrap();

        assert_eq!(
            game_server.status.counters.as_ref().unwrap()["heroes"],
            CounterStatus {
                count: 0,
                capacity: 10,
            }
        );
    }

    #[test]
    fn counter_actions_continue_after_capacity_error() {
        let mut game_server = crate::GameServer {
            status: crate::GameServerStatus {
                counters: Some(BTreeMap::from([(
                    "sages".into(),
                    CounterStatus {
                        count: 99,
                        capacity: 100,
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        let action = CounterAction {
            action: Some("Increment".into()),
            amount: Some(10),
            capacity: Some(-1),
        };

        assert!(action
            .counter_actions("sages", &mut game_server)
            .is_err());
        assert_eq!(
            game_server.status.counters.as_ref().unwrap()["sages"],
            CounterStatus {
                count: 100,
                capacity: 100,
            }
        );
    }

    #[test]
    fn counter_capacity_action_truncates_existing_count() {
        let mut game_server = crate::GameServer {
            status: crate::GameServerStatus {
                counters: Some(BTreeMap::from([(
                    "mages".into(),
                    CounterStatus {
                        count: 1,
                        capacity: 100,
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        CounterAction {
            capacity: Some(0),
            ..Default::default()
        }
        .counter_actions("mages", &mut game_server)
        .unwrap();

        assert_eq!(
            game_server.status.counters.as_ref().unwrap()["mages"],
            CounterStatus {
                count: 0,
                capacity: 0,
            }
        );
    }

    #[test]
    fn counter_count_action_updates_count() {
        let mut game_server = crate::GameServer {
            status: crate::GameServerStatus {
                counters: Some(BTreeMap::from([(
                    "baddies".into(),
                    CounterStatus {
                        count: 1,
                        capacity: 100,
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        CounterAction {
            action: Some("Increment".into()),
            amount: Some(10),
            ..Default::default()
        }
        .counter_actions("baddies", &mut game_server)
        .unwrap();

        assert_eq!(game_server.status.counters.as_ref().unwrap()["baddies"].count, 11);
    }

    #[test]
    fn list_capacity_action_truncates_values() {
        let mut game_server = crate::GameServer {
            status: crate::GameServerStatus {
                lists: Some(BTreeMap::from([(
                    "pages".into(),
                    ListStatus {
                        values: vec!["page1".into(), "page2".into()].into(),
                        capacity: 100,
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        ListAction {
            capacity: Some(0),
            ..Default::default()
        }
        .list_actions("pages", &mut game_server, 1000)
        .unwrap();

        assert_eq!(
            game_server.status.lists.as_ref().unwrap()["pages"],
            ListStatus {
                values: Vec::new().into(),
                capacity: 0,
            }
        );
    }

    #[test]
    fn list_actions_preserve_dedup_order_and_delete_after_append() {
        let mut game_server = crate::GameServer {
            status: crate::GameServerStatus {
                lists: Some(BTreeMap::from([(
                    "magicians".into(),
                    ListStatus {
                        values: vec![
                            "magician1".into(),
                            "magician2".into(),
                            "magician4".into(),
                            "magician5".into(),
                        ].into(),
                        capacity: 100,
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        let action = ListAction {
            add_values: vec!["magician1".into(), "magician3".into()],
            delete_values: vec![
                "magician2".into(),
                "magician5".into(),
                "magician6".into(),
            ],
            capacity: Some(42),
        };

        action
            .list_actions("magicians", &mut game_server, 1000)
            .unwrap();

        assert_eq!(
            game_server.status.lists.as_ref().unwrap()["magicians"],
            ListStatus {
                values: vec![
                    "magician1".into(),
                    "magician4".into(),
                    "magician3".into(),
                ].into(),
                capacity: 42,
            }
        );
    }

    #[test]
    fn list_append_silently_truncates_to_new_capacity() {
        let mut game_server = crate::GameServer {
            status: crate::GameServerStatus {
                lists: Some(BTreeMap::from([(
                    "fairies".into(),
                    ListStatus {
                        values: vec!["fairy1".into(), "fairy2".into()].into(),
                        capacity: 100,
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };
        ListAction {
            add_values: vec!["fairy1".into(), "fairy3".into()],
            capacity: Some(2),
            ..Default::default()
        }
        .list_actions("fairies", &mut game_server, 1000)
        .unwrap();

        assert_eq!(
            game_server.status.lists.as_ref().unwrap()["fairies"],
            ListStatus {
                values: vec!["fairy1".into(), "fairy2".into()].into(),
                capacity: 2,
            }
        );
    }

    #[test]
    fn list_capacity_above_configured_max_errors_without_capacity_change() {
        let mut game_server = crate::GameServer {
            status: crate::GameServerStatus {
                lists: Some(BTreeMap::from([(
                    "pages".into(),
                    ListStatus {
                        values: vec!["page1".into()].into(),
                        capacity: 5,
                    },
                )])),
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(ListAction {
            capacity: Some(26),
            ..Default::default()
        }
        .list_actions("pages", &mut game_server, 25)
        .is_err());

        assert_eq!(
            game_server.status.lists.as_ref().unwrap()["pages"],
            ListStatus {
                values: vec!["page1".into()].into(),
                capacity: 5,
            }
        );
    }

    #[test]
    fn action_errors_are_joined_and_later_actions_still_run() {
        let mut game_server = crate::GameServer::default();
        let error = CounterAction {
            action: Some("Increment".into()),
            amount: Some(1),
            capacity: Some(1),
        }
        .counter_actions("missing", &mut game_server)
        .unwrap_err();

        let messages: Vec<_> = error.lines().collect();
        assert_eq!(messages.len(), 2);
        assert!(messages[0].contains("Counter not found"));
        assert!(messages[1].contains("Counter not found"));
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
