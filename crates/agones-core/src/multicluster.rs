use std::collections::{BTreeMap, BTreeSet};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ObjectMeta;

mod base64_bytes {
    use super::*;

    pub fn serialize<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&BASE64.encode(value))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        BASE64.decode(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterConnectionInfo {
    #[serde(default)]
    pub cluster_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allocation_endpoints: Vec<String>,
    #[serde(default)]
    pub secret_name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "base64_bytes::serialize",
        deserialize_with = "base64_bytes::deserialize"
    )]
    pub server_ca: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerAllocationPolicySpec {
    pub priority: i32,
    pub weight: i64,
    #[serde(default)]
    pub connection_info: ClusterConnectionInfo,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerAllocationPolicy {
    #[serde(default)]
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: GameServerAllocationPolicySpec,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerAllocationPolicyList {
    #[serde(default)]
    pub items: Vec<GameServerAllocationPolicy>,
}

type ClusterPolicies = BTreeMap<String, Vec<GameServerAllocationPolicy>>;

#[derive(Clone, Debug)]
pub struct ConnectionInfoIterator {
    curr_priority: usize,
    ordered_priorities: Vec<i32>,
    priority_to_cluster: BTreeMap<i32, ClusterPolicies>,
    cluster_blacklist: BTreeSet<String>,
}

impl ConnectionInfoIterator {
    pub fn new(policies: &[GameServerAllocationPolicy]) -> Self {
        let mut priority_to_cluster: BTreeMap<i32, ClusterPolicies> = BTreeMap::new();

        for policy in policies {
            priority_to_cluster
                .entry(policy.spec.priority)
                .or_default()
                .entry(policy.spec.connection_info.cluster_name.clone())
                .or_default()
                .push(policy.clone());
        }

        let ordered_priorities = priority_to_cluster.keys().copied().collect();
        Self {
            curr_priority: 0,
            ordered_priorities,
            priority_to_cluster,
            cluster_blacklist: BTreeSet::new(),
        }
    }

    pub fn next_connection(&mut self) -> Option<ClusterConnectionInfo> {
        while self.curr_priority < self.ordered_priorities.len() {
            let priority = self.ordered_priorities[self.curr_priority];
            let result = self
                .priority_to_cluster
                .get(&priority)
                .and_then(|policies| self.select_cluster_connection_info(policies));

            match result {
                Some(result) => {
                    self.cluster_blacklist.insert(result.cluster_name.clone());
                    return Some(result);
                }
                None => self.curr_priority += 1,
            }
        }
        None
    }

    fn select_cluster_connection_info(
        &self,
        cluster_policies: &ClusterPolicies,
    ) -> Option<ClusterConnectionInfo> {
        let mut connections = Vec::new();
        let mut weights = Vec::new();

        for (cluster, policies) in cluster_policies {
            if self.cluster_blacklist.contains(cluster) {
                continue;
            }
            connections.push(policies.first()?.spec.connection_info.clone());
            weights.push(avg_weight(policies));
        }

        select_random_weighted(&connections, &weights)
    }
}

impl Iterator for ConnectionInfoIterator {
    type Item = ClusterConnectionInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_connection()
    }
}

pub fn avg_weight(policies: &[GameServerAllocationPolicy]) -> i64 {
    if policies.is_empty() {
        return 0;
    }
    policies
        .iter()
        .map(|policy| policy.spec.weight)
        .sum::<i64>()
        / policies.len() as i64
}

pub fn select_random_weighted(
    connections: &[ClusterConnectionInfo],
    weights: &[i64],
) -> Option<ClusterConnectionInfo> {
    if connections.len() != weights.len() {
        return None;
    }

    let sum = weights
        .iter()
        .try_fold(0i64, |sum, weight| sum.checked_add(*weight))?;
    if sum <= 0 {
        return None;
    }

    let mut random = rand::thread_rng().gen_range(0..sum);
    for (connection, weight) in connections.iter().zip(weights) {
        if *weight <= 0 {
            continue;
        }
        if random < *weight {
            return Some(connection.clone());
        }
        random -= *weight;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy(priority: i32, weight: i64, cluster: &str) -> GameServerAllocationPolicy {
        GameServerAllocationPolicy {
            spec: GameServerAllocationPolicySpec {
                priority,
                weight,
                connection_info: ClusterConnectionInfo {
                    cluster_name: cluster.into(),
                    ..Default::default()
                },
            },
            ..Default::default()
        }
    }

    #[test]
    fn simple_iterator_preserves_connection_information() {
        let policies = vec![GameServerAllocationPolicy {
            spec: GameServerAllocationPolicySpec {
                priority: 1,
                weight: 100,
                connection_info: ClusterConnectionInfo {
                    cluster_name: "cluster1".into(),
                    secret_name: "secret-name".into(),
                    allocation_endpoints: vec!["allocation-endpoint".into()],
                    namespace: "ns1".into(),
                    server_ca: b"c2VydmVyQ0E=".to_vec(),
                },
            },
            ..Default::default()
        }];

        let result: Vec<_> = ConnectionInfoIterator::new(&policies).collect();
        assert_eq!(result, vec![policies[0].spec.connection_info.clone()]);
    }

    #[test]
    fn priority_order_and_global_cluster_blacklist_match_go_iterator() {
        let policies = vec![
            policy(1, 300, "cluster1"),
            policy(2, 300, "cluster1"),
            policy(2, 100, "cluster2"),
        ];
        let result: Vec<_> = ConnectionInfoIterator::new(&policies).collect();
        assert_eq!(result[0].cluster_name, "cluster1");
        assert_eq!(result[1].cluster_name, "cluster2");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn same_priority_clusters_are_each_returned_once() {
        let policies = vec![policy(1, 300, "cluster1"), policy(1, 100, "cluster2")];
        let result: BTreeSet<_> = ConnectionInfoIterator::new(&policies)
            .map(|connection| connection.cluster_name)
            .collect();
        assert_eq!(
            result,
            BTreeSet::from(["cluster1".to_owned(), "cluster2".to_owned()])
        );
    }

    #[test]
    fn zero_weight_cluster_is_never_chosen_when_positive_weight_exists() {
        let policies = vec![policy(1, 0, "cluster1"), policy(1, 100, "cluster2")];
        let result: Vec<_> = ConnectionInfoIterator::new(&policies).collect();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].cluster_name, "cluster2");
    }

    #[test]
    fn duplicate_cluster_same_priority_uses_average_weight_and_returns_once() {
        let policies = vec![
            policy(444, 100, "cluster-name"),
            policy(444, 300, "cluster-name"),
        ];
        assert_eq!(avg_weight(&policies), 200);
        let result: Vec<_> = ConnectionInfoIterator::new(&policies).collect();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].cluster_name, "cluster-name");
    }

    #[test]
    fn empty_policy_list_is_empty() {
        let policies = Vec::new();
        assert!(ConnectionInfoIterator::new(&policies).next().is_none());
    }

    #[test]
    fn multiple_allocation_endpoints_are_preserved() {
        let policies = vec![GameServerAllocationPolicy {
            spec: GameServerAllocationPolicySpec {
                priority: 1,
                weight: 100,
                connection_info: ClusterConnectionInfo {
                    cluster_name: "cluster1".into(),
                    allocation_endpoints: vec!["alloc1".into(), "alloc2".into()],
                    ..Default::default()
                },
            },
            ..Default::default()
        }];
        let result = ConnectionInfoIterator::new(&policies).next().unwrap();
        assert_eq!(result.allocation_endpoints, vec!["alloc1", "alloc2"]);
    }

    #[test]
    fn server_ca_uses_go_compatible_base64_json_wire_format() {
        let info = ClusterConnectionInfo {
            server_ca: b"serverCA".to_vec(),
            ..Default::default()
        };
        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(json["serverCa"], "c2VydmVyQ0E=");
        let decoded: ClusterConnectionInfo = serde_json::from_value(json).unwrap();
        assert_eq!(decoded.server_ca, b"serverCA");
    }

    #[test]
    fn weighted_selection_rejects_nonpositive_total_weight() {
        let connections = vec![
            ClusterConnectionInfo {
                cluster_name: "one".into(),
                ..Default::default()
            },
            ClusterConnectionInfo {
                cluster_name: "two".into(),
                ..Default::default()
            },
        ];
        assert!(select_random_weighted(&connections, &[0, 0]).is_none());
        assert!(select_random_weighted(&connections, &[-1, 0]).is_none());
    }

    #[test]
    fn bolero_average_of_nonnegative_weights_stays_within_input_range() {
        bolero::check!().for_each(|bytes: &[u8]| {
            if bytes.is_empty() {
                return;
            }
            let policies: Vec<_> = bytes
                .iter()
                .take(16)
                .map(|weight| policy(1, i64::from(*weight), "cluster"))
                .collect();
            let average = avg_weight(&policies);
            let min = policies.iter().map(|p| p.spec.weight).min().unwrap();
            let max = policies.iter().map(|p| p.spec.weight).max().unwrap();
            assert!(average >= min);
            assert!(average <= max);
        });
    }
}
