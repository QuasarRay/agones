use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    CanValidateGameServerFeatures,
    CanValidateObjectMetadata,
    CanValidateResourceQuantities,
    Fleet,
    GameServer,
    GameServerSet,
    GameServerSpec,
    GameServerState,
    ObjectMeta,
    ValidationError,
};

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

pub const ERR_CONTAINER_REQUIRED: &str =
    "Container is required when using multiple containers in the pod template";

pub const ERR_HOST_PORT: &str =
    "HostPort cannot be specified with a Dynamic or Passthrough PortPolicy";

pub const ERR_PORT_POLICY_STATIC: &str =
    "PortPolicy must be Static";

pub const ERR_CONTAINER_PORT_REQUIRED: &str =
    "ContainerPort must be defined for Dynamic and Static PortPolicies";

pub const ERR_CONTAINER_PORT_PASSTHROUGH: &str =
    "ContainerPort cannot be specified with Passthrough PortPolicy";

pub const ERR_CONTAINER_NAME_INVALID: &str =
    "Container must be empty or the name of a container in the pod template";

pub const GAME_SERVER_PRIORITY_INCREMENT: &str = "Increment";
pub const GAME_SERVER_PRIORITY_DECREMENT: &str = "Decrement";

pub const GAME_SERVER_PRIORITY_COUNTER: &str = "Counter";
pub const GAME_SERVER_PRIORITY_LIST: &str = "List";

pub const GAME_SERVER_PRIORITY_ASCENDING: &str = "Ascending";
pub const GAME_SERVER_PRIORITY_DESCENDING: &str = "Descending";

// -----------------------------------------------------------------------------
// Aggregated status
// -----------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------
// Common CRD validation
// -----------------------------------------------------------------------------

/// Equivalent to Go's validateName.
///
/// The Go `crd` interface also exposes GetObjectKind(), but validateName never
/// uses it. Rust does not need to encode that unused requirement.
pub fn validate_name(
    metadata: &ObjectMeta,
    path: &str,
) -> Vec<ValidationError> {
    if metadata.name.len() <= 63 {
        return Vec::new();
    }

    vec![ValidationError::too_long(
        child_path(path, "name"),
        metadata.name.as_str(),
        63,
    )]
}

fn child_path(parent: &str, child: &str) -> String {
    if parent.is_empty() {
        child.to_owned()
    } else {
        format!("{parent}.{child}")
    }
}

// -----------------------------------------------------------------------------
// GameServerSpec abstraction
// -----------------------------------------------------------------------------

/// Rust equivalent of the Go gsSpec interface.
///
/// Mutable access is intentional: the Go implementation calls ApplyDefaults()
/// on the actual GameServerSpec before validation.
pub trait HasGameServerSpec {
    fn game_server_spec_mut(&mut self) -> &mut GameServerSpec;
}

impl HasGameServerSpec for GameServerSet {
    fn game_server_spec_mut(&mut self) -> &mut GameServerSpec {
        GameServerSet::game_server_spec_mut(self)
    }
}

impl HasGameServerSpec for Fleet {
    fn game_server_spec_mut(&mut self) -> &mut GameServerSpec {
        Fleet::game_server_spec_mut(self)
    }
}

/// Equivalent to validateGSSpec.
///
/// Unlike a clone-and-validate implementation, this deliberately applies
/// defaults to the actual embedded GameServerSpec, matching the Go function.
pub fn validate_gs_spec<C>(
    context: &C,
    resource: &mut impl HasGameServerSpec,
    path: &str,
) -> Vec<ValidationError>
where
    C: CanValidateGameServerFeatures
        + CanValidateResourceQuantities
        + CanValidateObjectMetadata,
{
    let spec = resource.game_server_spec_mut();

    spec.apply_defaults();

    spec.validate_with_context(context, "")
        .into_iter()
        .map(|error| rebase_spec_error(error, path))
        .collect()
}

fn rebase_spec_error(
    mut error: ValidationError,
    path: &str,
) -> ValidationError {
    error.path = match error.path.strip_prefix("spec") {
        Some("") => path.to_owned(),
        Some(rest) => format!("{path}{rest}"),
        None if path.is_empty() => error.path,
        None => format!("{path}.{}", error.path),
    };

    error
}

// -----------------------------------------------------------------------------
// ObjectMeta validation
// -----------------------------------------------------------------------------

/// Equivalent to validateObjectMeta.
///
/// Kubernetes' ValidateLabels/ValidateAnnotations are replaced by the shared
/// CGP validation capability already used by agones-core.
pub fn validate_object_meta<C>(
    context: &C,
    metadata: &ObjectMeta,
    path: &str,
) -> Vec<ValidationError>
where
    C: CanValidateObjectMetadata,
{
    context.validate_object_metadata(
        &metadata.labels,
        &metadata.annotations,
        path,
    )
}

// -----------------------------------------------------------------------------
// AllocationOverflow
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationOverflow {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
}

impl AllocationOverflow {
    pub fn validate<C>(
        &self,
        context: &C,
        path: &str,
    ) -> Vec<ValidationError>
    where
        C: CanValidateObjectMetadata,
    {
        context.validate_object_metadata(
            &self.labels,
            &self.annotations,
            path,
        )
    }

    /// Returns:
    ///
    /// * the number of Allocated GameServers matching every requested
    ///   label and annotation;
    /// * allocated GameServers that failed either match.
    ///
    /// Non-Allocated GameServers are intentionally absent from `rest`,
    /// matching the Go implementation.
    pub fn count_matches<'a>(
        &self,
        game_servers: &'a [GameServer],
    ) -> (i32, Vec<&'a GameServer>) {
        let mut count = 0;
        let mut rest = Vec::new();

        for game_server in game_servers {
            if game_server.status.state != Some(GameServerState::Allocated) {
                continue;
            }

            if contains_all(&self.labels, &game_server.metadata.labels)
                && contains_all(
                    &self.annotations,
                    &game_server.metadata.annotations,
                )
            {
                count += 1;
            } else {
                rest.push(game_server);
            }
        }

        (count, rest)
    }

    /// Applies AllocationOverflow metadata to a GameServer.
    ///
    /// Existing unrelated metadata survives. Matching keys are overwritten
    /// by AllocationOverflow, exactly like maps.Copy(dst, src).
    pub fn apply(&self, game_server: &mut GameServer) {
        game_server
            .metadata
            .annotations
            .extend(self.annotations.clone());

        game_server
            .metadata
            .labels
            .extend(self.labels.clone());
    }
}

fn contains_all(
    expected: &BTreeMap<String, String>,
    actual: &BTreeMap<String, String>,
) -> bool {
    expected
        .iter()
        .all(|(key, value)| actual.get(key) == Some(value))
}

// -----------------------------------------------------------------------------
// GameServer allocation priority
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    #[serde(rename = "type")]
    pub priority_type: String,

    pub key: String,

    pub order: String,
}

// -----------------------------------------------------------------------------
// Overflow-safe arithmetic
// -----------------------------------------------------------------------------

/// Exact behavioral port of Agones SafeAdd.
///
/// `wrapping_*` is deliberate. The Go implementation performs machine-width
/// signed arithmetic while computing the threshold, so ordinary Rust arithmetic
/// would panic in debug builds for cases where Go wraps.
pub fn safe_add(x: i64, y: i64) -> i64 {
    let threshold = i64::MAX.wrapping_sub(y);

    if x > threshold {
        i64::MAX
    } else {
        x.wrapping_add(y)
    }
}