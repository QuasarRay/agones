use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

use agones_macros::{agones_invariant, AgonesStringEnum};
use serde::{Deserialize, Serialize};

use crate::gameserver::{
    safe_add, validate_object_meta, GameServerSpec, ObjectMeta, Priority, SchedulingStrategy,
    ValidationError, ValidationErrorKind, VERSION_ANNOTATION,
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
    #[serde(default)]
    pub replicas: i32,
    #[serde(default)]
    pub ready_replicas: i32,
    #[serde(default)]
    pub reserved_replicas: i32,
    #[serde(default)]
    pub allocated_replicas: i32,
    #[serde(default)]
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
        self.apply_defaults_with_version_context(&crate::AgonesBuildVersionContext::default());
    }

    pub fn apply_defaults_with_version_context<C: crate::CanGetAgonesVersionAnnotation>(
        &mut self,
        context: &C,
    ) {
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
            context.agones_version_annotation(),
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

/// Resolves the Kubernetes IntOrString representation used by Fleet rollout
/// percentages.
///
/// Percentage values are scaled against `total`. RollingUpdate MaxSurge uses
/// round-up semantics; MaxUnavailable uses round-down semantics.
pub fn resolve_int_or_percent(
    value: &IntOrPercent,
    total: i32,
    round_up: bool,
) -> Result<i32, String> {
    match value {
        IntOrPercent::Int(value) => Ok(*value),
        IntOrPercent::String(value) => {
            let percent = value
                .strip_suffix('%')
                .ok_or_else(|| format!("invalid percentage {value}"))?
                .parse::<i64>()
                .map_err(|_| format!("invalid percentage {value}"))?;
            let total = i64::from(total);
            let product = percent
                .checked_mul(total)
                .ok_or_else(|| format!("percentage overflow for {value}"))?;

            let resolved = if round_up && product > 0 {
                product
                    .checked_add(99)
                    .ok_or_else(|| format!("percentage overflow for {value}"))?
                    / 100
            } else {
                product / 100
            };

            i32::try_from(resolved)
                .map_err(|_| format!("resolved percentage out of range for {value}"))
        }
    }
}

/// Pure port of upstream rollingUpdateActive.
///
/// This computes only the desired replica count for the active GameServerSet.
/// Scaling inactive sets remains a separate persistence milestone.
pub fn rolling_update_active_replicas(
    fleet: &Fleet,
    active: &GameServerSet,
    rest: &[GameServerSet],
) -> Result<i32, String> {
    let mut replicas = active.spec.replicas;
    let sum_allocated = rest.iter().fold(0i32, |sum, set| {
        sum.wrapping_add(set.status.allocated_replicas)
    });

    if active.spec.replicas != active.status.replicas {
        return Ok(replicas);
    }

    if fleet.spec.replicas == 0 {
        return Ok(0);
    }

    let target_after_allocated =
        fleet.spec.replicas.wrapping_sub(sum_allocated);
    if active.spec.replicas >= target_after_allocated {
        return Ok(fleet.lower_bound_replicas(target_after_allocated));
    }

    let rolling = fleet
        .spec
        .strategy
        .rolling_update
        .as_ref()
        .ok_or_else(|| "RollingUpdate strategy is missing rollingUpdate".to_owned())?;
    let max_surge_value = rolling
        .max_surge
        .as_ref()
        .ok_or_else(|| "RollingUpdate strategy is missing maxSurge".to_owned())?;
    let surge =
        resolve_int_or_percent(max_surge_value, fleet.spec.replicas, true)?;

    let max_surge = fleet.spec.replicas.wrapping_add(surge);
    replicas = fleet.upper_bound_replicas(
        replicas.wrapping_add(surge),
    );

    let total = rest.iter().fold(replicas, |sum, set| {
        sum.wrapping_add(set.status.replicas)
    });
    if total > max_surge {
        replicas = fleet.lower_bound_replicas(
            replicas.wrapping_sub(total.wrapping_sub(max_surge)),
        );
    }

    if replicas.wrapping_add(sum_allocated) > fleet.spec.replicas {
        replicas = fleet.lower_bound_replicas(
            fleet.spec.replicas.wrapping_sub(sum_allocated),
        );
    }

    Ok(replicas)
}

/// Computes the active GameServerSet replica target selected by the upstream
/// deployment-strategy dispatcher.
///
/// For Recreate this returns the target that remains after inactive allocated
/// GameServers block capacity; the actual inactive-set scale-to-zero writes are
/// deliberately left to the persistence layer.
pub fn fleet_active_replicas_target(
    fleet: &Fleet,
    active: &GameServerSet,
    rest: &[GameServerSet],
) -> Result<i32, String> {
    if rest.is_empty() {
        return Ok(fleet.spec.replicas);
    }

    let rest_spec_replicas = rest.iter().fold(0i32, |sum, set| {
        sum.wrapping_add(set.spec.replicas)
    });
    if rest_spec_replicas == 0 {
        let blocked = rest.iter().fold(0i32, |sum, set| {
            sum.wrapping_add(
                set.status
                    .reserved_replicas
                    .wrapping_add(set.status.allocated_replicas),
            )
        });
        return Ok(fleet.lower_bound_replicas(
            fleet.spec.replicas.wrapping_sub(blocked),
        ));
    }

    match fleet.spec.strategy.strategy_type.as_ref() {
        Some(DeploymentStrategyType::Recreate) => {
            let allocated = rest.iter().fold(0i32, |sum, set| {
                sum.wrapping_add(set.status.allocated_replicas)
            });
            Ok(fleet.lower_bound_replicas(
                fleet.spec.replicas.wrapping_sub(allocated),
            ))
        }
        Some(DeploymentStrategyType::RollingUpdate) => {
            rolling_update_active_replicas(fleet, active, rest)
        }
        Some(strategy) => Err(format!(
            "unexpected deployment strategy type: {}",
            strategy.as_str()
        )),
        None => Err("unexpected deployment strategy type: ".to_owned()),
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FleetGameServerSetScaleReason {
    Recreate,
    UnhealthyCleanup,
    RollingUpdate,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetGameServerSetScale {
    pub namespace: String,
    pub name: String,
    pub from_replicas: i32,
    pub to_replicas: i32,
    pub reason: FleetGameServerSetScaleReason,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetDeploymentPlan {
    pub active_replicas: i32,
    pub inactive_scales: Vec<FleetGameServerSetScale>,
}

fn game_server_set_scale(
    game_server_set: &GameServerSet,
    to_replicas: i32,
    reason: FleetGameServerSetScaleReason,
) -> FleetGameServerSetScale {
    FleetGameServerSetScale {
        namespace: game_server_set.metadata.namespace.clone(),
        name: game_server_set.metadata.name.clone(),
        from_replicas: game_server_set.spec.replicas,
        to_replicas,
        reason,
    }
}

/// Pure port of the inactive-GameServerSet side of Agones' fixed-on-ready
/// rolling update.
///
/// The returned actions preserve upstream ordering. The unhealthy cleanup pass
/// is applied to an in-memory working copy before the ordinary scale-down pass,
/// matching the Go controller's mutation of its local GameServerSet slice.
pub fn rolling_update_inactive_scale_plan(
    fleet: &Fleet,
    active: &GameServerSet,
    rest: &[GameServerSet],
) -> Result<Vec<FleetGameServerSetScale>, String> {
    if rest.is_empty() {
        return Ok(Vec::new());
    }

    let rolling = fleet
        .spec
        .strategy
        .rolling_update
        .as_ref()
        .ok_or_else(|| "RollingUpdate strategy is missing rollingUpdate".to_owned())?;
    let max_unavailable_value = rolling
        .max_unavailable
        .as_ref()
        .ok_or_else(|| "RollingUpdate strategy is missing maxUnavailable".to_owned())?;

    let mut unavailable =
        resolve_int_or_percent(max_unavailable_value, fleet.status.ready_replicas, false)?;
    if unavailable == 0 {
        unavailable = 1;
    }
    if unavailable > fleet.spec.replicas {
        unavailable = fleet.spec.replicas;
    }

    let ready_replicas_count = rest.iter().fold(
        active.status.ready_replicas,
        |sum, game_server_set| sum.wrapping_add(game_server_set.status.ready_replicas),
    );
    let min_available = fleet
        .status
        .ready_replicas
        .wrapping_sub(unavailable)
        .min(fleet.spec.replicas);

    let new_game_server_set_unavailable = active
        .spec
        .replicas
        .wrapping_sub(active.status.ready_replicas)
        .wrapping_sub(active.status.reserved_replicas)
        .wrapping_sub(active.status.allocated_replicas)
        .wrapping_sub(active.status.shutdown_replicas);
    let max_scaled_down = ready_replicas_count
        .wrapping_sub(min_available)
        .wrapping_sub(new_game_server_set_unavailable);

    if max_scaled_down <= 0 {
        return Ok(Vec::new());
    }

    let mut working = rest.to_vec();
    let mut actions = Vec::new();
    let mut total_cleanup_scaled_down = 0i32;

    for game_server_set in &mut working {
        if total_cleanup_scaled_down >= max_scaled_down {
            break;
        }
        if game_server_set.spec.replicas == 0 {
            continue;
        }

        let healthy = game_server_set
            .status
            .ready_replicas
            .wrapping_add(game_server_set.status.allocated_replicas)
            .wrapping_add(game_server_set.status.reserved_replicas);
        if game_server_set.spec.replicas <= healthy {
            continue;
        }

        let cleanup_room = max_scaled_down.wrapping_sub(total_cleanup_scaled_down);
        let unhealthy = game_server_set.spec.replicas.wrapping_sub(healthy);
        let scaled_down = cleanup_room.min(unhealthy);
        let new_replicas = game_server_set.spec.replicas.wrapping_sub(scaled_down);
        if new_replicas > game_server_set.spec.replicas {
            return Err(format!(
                "when cleaning up unhealthy replicas, got invalid request to scale down {}/{} {} -> {}",
                game_server_set.metadata.namespace,
                game_server_set.metadata.name,
                game_server_set.spec.replicas,
                new_replicas
            ));
        }

        actions.push(game_server_set_scale(
            game_server_set,
            new_replicas,
            FleetGameServerSetScaleReason::UnhealthyCleanup,
        ));
        game_server_set.spec.replicas = new_replicas;
        total_cleanup_scaled_down = total_cleanup_scaled_down.wrapping_add(scaled_down);
    }

    let total_scale_down_count = ready_replicas_count.wrapping_sub(min_available);
    if ready_replicas_count <= min_available {
        return Ok(actions);
    }

    let mut total_already_scaled_down = 0i32;
    for game_server_set in &working {
        if total_already_scaled_down >= total_scale_down_count {
            break;
        }
        if game_server_set.spec.replicas <= 0 {
            continue;
        }

        // Wait for an earlier spec change to be observed in status before
        // continuing through older inactive GameServerSets.
        if game_server_set.spec.replicas != game_server_set.status.replicas {
            break;
        }
        if game_server_set.status.shutdown_replicas != 0 {
            continue;
        }

        let scale_down_count = game_server_set.spec.replicas.min(
            total_scale_down_count.wrapping_sub(total_already_scaled_down),
        );
        let new_replicas = game_server_set
            .spec
            .replicas
            .wrapping_sub(scale_down_count);
        if new_replicas > game_server_set.spec.replicas {
            return Err(format!(
                "when scaling down old GameServerSet, got invalid request to scale down {}/{} {} -> {}",
                game_server_set.metadata.namespace,
                game_server_set.metadata.name,
                game_server_set.spec.replicas,
                new_replicas
            ));
        }

        let target = if game_server_set.status.replicas
            == game_server_set.status.allocated_replicas
        {
            0
        } else if new_replicas == game_server_set.spec.replicas {
            continue;
        } else {
            new_replicas
        };

        actions.push(game_server_set_scale(
            game_server_set,
            target,
            FleetGameServerSetScaleReason::RollingUpdate,
        ));
        total_already_scaled_down =
            total_already_scaled_down.wrapping_add(scale_down_count);
    }

    Ok(actions)
}

/// Computes all inactive GameServerSet scale writes for the selected deployment
/// strategy, without performing persistence.
pub fn fleet_inactive_scale_plan(
    fleet: &Fleet,
    active: &GameServerSet,
    rest: &[GameServerSet],
) -> Result<Vec<FleetGameServerSetScale>, String> {
    if rest.is_empty() {
        return Ok(Vec::new());
    }

    let rest_spec_replicas = rest.iter().fold(0i32, |sum, set| {
        sum.wrapping_add(set.spec.replicas)
    });
    if rest_spec_replicas == 0 {
        return Ok(Vec::new());
    }

    match fleet.spec.strategy.strategy_type.as_ref() {
        Some(DeploymentStrategyType::Recreate) => Ok(rest
            .iter()
            .filter(|set| set.spec.replicas != 0)
            .map(|set| {
                game_server_set_scale(
                    set,
                    0,
                    FleetGameServerSetScaleReason::Recreate,
                )
            })
            .collect()),
        Some(DeploymentStrategyType::RollingUpdate) => {
            rolling_update_inactive_scale_plan(fleet, active, rest)
        }
        Some(strategy) => Err(format!(
            "unexpected deployment strategy type: {}",
            strategy.as_str()
        )),
        None => Err("unexpected deployment strategy type: ".to_owned()),
    }
}

/// Produces the complete pure deployment-strategy result used by the Fleet
/// controller: the desired active replica count plus the ordered writes for
/// inactive GameServerSets.
pub fn fleet_deployment_plan(
    fleet: &Fleet,
    active: &GameServerSet,
    rest: &[GameServerSet],
) -> Result<FleetDeploymentPlan, String> {
    let active_replicas = fleet_active_replicas_target(fleet, active, rest)?;
    let inactive_scales = fleet_inactive_scale_plan(fleet, active, rest)?;
    Ok(FleetDeploymentPlan {
        active_replicas,
        inactive_scales,
    })
}

fn checked_pow_i128(base: i128, exponent: u32) -> Option<i128> {
    let mut value = 1i128;
    for _ in 0..exponent {
        value = value.checked_mul(base)?;
    }
    Some(value)
}

fn gcd_i128(mut left: i128, mut right: i128) -> i128 {
    left = left.abs();
    right = right.abs();
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

/// Parses the practical Kubernetes Quantity forms used in Pod resource maps
/// into an exact reduced rational. This is for semantic equality, not resource
/// admission; unparseable/overflowing values remain structurally compared.
fn quantity_rational(value: &str) -> Option<(i128, i128)> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    let binary_suffixes = [
        ("Ki", 10u32),
        ("Mi", 20),
        ("Gi", 30),
        ("Ti", 40),
        ("Pi", 50),
        ("Ei", 60),
    ];
    let decimal_suffixes = [
        ("m", -3i32),
        ("k", 3),
        ("M", 6),
        ("G", 9),
        ("T", 12),
        ("P", 15),
        ("E", 18),
    ];

    let mut mantissa = value;
    let mut decimal_exponent = 0i32;
    let mut binary_exponent = None;

    if let Some((suffix, exponent)) = binary_suffixes
        .iter()
        .find(|(suffix, _)| value.ends_with(*suffix))
    {
        mantissa = value.strip_suffix(*suffix)?;
        binary_exponent = Some(*exponent);
    } else if let Some((suffix, exponent)) = decimal_suffixes
        .iter()
        .find(|(suffix, _)| value.ends_with(*suffix))
    {
        mantissa = value.strip_suffix(*suffix)?;
        decimal_exponent = *exponent;
    } else if let Some(index) = value
        .char_indices()
        .rev()
        .find_map(|(index, character)| {
            matches!(character, 'e' | 'E').then_some(index)
        })
    {
        let exponent = value[index + 1..].parse::<i32>().ok()?;
        mantissa = &value[..index];
        decimal_exponent = exponent;
    }

    let (negative, unsigned) = match mantissa.as_bytes().first() {
        Some(b'-') => (true, &mantissa[1..]),
        Some(b'+') => (false, &mantissa[1..]),
        _ => (false, mantissa),
    };
    if unsigned.is_empty() {
        return None;
    }

    let mut digits = String::new();
    let mut fractional_digits = 0u32;
    let mut seen_dot = false;
    for character in unsigned.chars() {
        match character {
            '0'..='9' => {
                digits.push(character);
                if seen_dot {
                    fractional_digits = fractional_digits.checked_add(1)?;
                }
            }
            '.' if !seen_dot => seen_dot = true,
            _ => return None,
        }
    }
    if digits.is_empty() {
        return None;
    }

    let mut numerator = digits.parse::<i128>().ok()?;
    if negative {
        numerator = numerator.checked_neg()?;
    }
    let mut denominator = checked_pow_i128(10, fractional_digits)?;

    if decimal_exponent > 0 {
        numerator = numerator.checked_mul(checked_pow_i128(
            10,
            decimal_exponent as u32,
        )?)?;
    } else if decimal_exponent < 0 {
        denominator = denominator.checked_mul(checked_pow_i128(
            10,
            decimal_exponent.unsigned_abs(),
        )?)?;
    }

    if let Some(binary_exponent) = binary_exponent {
        numerator = numerator.checked_mul(checked_pow_i128(
            2,
            binary_exponent,
        )?)?;
    }

    let divisor = gcd_i128(numerator, denominator);
    Some((numerator / divisor, denominator / divisor))
}

fn normalize_quantity_map(value: &mut serde_json::Value) {
    let Some(map) = value.as_object_mut() else {
        return;
    };
    for quantity in map.values_mut() {
        let Some(raw) = quantity.as_str() else {
            continue;
        };
        let Some((numerator, denominator)) = quantity_rational(raw) else {
            continue;
        };
        *quantity = serde_json::json!({
            "$agonesQuantityNumerator": numerator.to_string(),
            "$agonesQuantityDenominator": denominator.to_string(),
        });
    }
}

fn normalize_template_quantities(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map.iter_mut() {
                if matches!(key.as_str(), "limits" | "requests" | "overhead") {
                    normalize_quantity_map(child);
                } else {
                    normalize_template_quantities(child);
                }
            }
        }
        serde_json::Value::Array(values) => {
            for child in values {
                normalize_template_quantities(child);
            }
        }
        _ => {}
    }
}

/// Kubernetes apiequality.Semantic treats resource.Quantity values by numeric
/// value rather than textual representation. The Rust domain model stores
/// unmodeled Pod fields as JSON extensions, so normalize resource-quantity maps
/// before comparing templates to preserve cases such as CPU `1 == 1000m`.
pub fn game_server_templates_semantically_equal(
    left: &GameServerTemplateSpec,
    right: &GameServerTemplateSpec,
) -> bool {
    if left == right {
        return true;
    }

    let Ok(mut left) = serde_json::to_value(left) else {
        return false;
    };
    let Ok(mut right) = serde_json::to_value(right) else {
        return false;
    };
    normalize_template_quantities(&mut left);
    normalize_template_quantities(&mut right);
    left == right
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
        if game_server_templates_semantically_equal(
            &game_server_set.spec.template,
            &fleet.spec.template,
        ) {
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
    #[test]
    fn fleet_status_missing_scalar_fields_use_go_zero_values() {
        let status: super::FleetStatus = serde_json::from_value(
            serde_json::json!({
                "replicas": 10,
                "readyReplicas": 2,
                "allocatedReplicas": 8
            }),
        )
        .unwrap();

        assert_eq!(status.replicas, 10);
        assert_eq!(status.ready_replicas, 2);
        assert_eq!(status.reserved_replicas, 0);
        assert_eq!(status.allocated_replicas, 8);
        assert_eq!(status.allocations, 0);
    }


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
                                }].into(),
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
                                }].into(),
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
            crate::DEFAULT_SDK_VERSION
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
                ].into(),
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
    fn int_or_percent_resolution_matches_kubernetes_rounding() {
        assert_eq!(
            resolve_int_or_percent(
                &IntOrPercent::String("25%".into()),
                10,
                true,
            )
            .unwrap(),
            3
        );
        assert_eq!(
            resolve_int_or_percent(
                &IntOrPercent::String("25%".into()),
                10,
                false,
            )
            .unwrap(),
            2
        );
        assert_eq!(
            resolve_int_or_percent(&IntOrPercent::Int(7), 10, true)
                .unwrap(),
            7
        );
        assert!(resolve_int_or_percent(
            &IntOrPercent::String("wat".into()),
            10,
            true,
        )
        .is_err());
    }

    fn rollout_fleet(replicas: i32) -> Fleet {
        Fleet {
            spec: FleetSpec {
                replicas,
                strategy: DeploymentStrategy {
                    strategy_type: Some(DeploymentStrategyType::RollingUpdate),
                    rolling_update: Some(RollingUpdateDeployment {
                        max_unavailable: Some(IntOrPercent::String("25%".into())),
                        max_surge: Some(IntOrPercent::String("25%".into())),
                    }),
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn rolling_update_active_waits_for_inflight_scaling() {
        let fleet = rollout_fleet(100);
        let active = GameServerSet {
            spec: GameServerSetSpec {
                replicas: 25,
                ..Default::default()
            },
            status: crate::gameserverset::GameServerSetStatus {
                replicas: 10,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            rolling_update_active_replicas(&fleet, &active, &[]).unwrap(),
            25
        );
    }

    #[test]
    fn rolling_update_active_adds_max_surge_and_respects_fleet_bound() {
        let fleet = rollout_fleet(100);
        let active = GameServerSet {
            spec: GameServerSetSpec {
                replicas: 0,
                ..Default::default()
            },
            status: crate::gameserverset::GameServerSetStatus {
                replicas: 0,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            rolling_update_active_replicas(&fleet, &active, &[]).unwrap(),
            25
        );

        let active = GameServerSet {
            spec: GameServerSetSpec {
                replicas: 90,
                ..Default::default()
            },
            status: crate::gameserverset::GameServerSetStatus {
                replicas: 90,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            rolling_update_active_replicas(&fleet, &active, &[]).unwrap(),
            100
        );
    }

    #[test]
    fn rolling_update_active_leaves_room_for_inactive_allocated_servers() {
        let fleet = rollout_fleet(5);
        let active = GameServerSet {
            spec: GameServerSetSpec {
                replicas: 4,
                ..Default::default()
            },
            status: crate::gameserverset::GameServerSetStatus {
                replicas: 4,
                ready_replicas: 4,
                ..Default::default()
            },
            ..Default::default()
        };
        let inactive = GameServerSet {
            spec: GameServerSetSpec {
                replicas: 0,
                ..Default::default()
            },
            status: crate::gameserverset::GameServerSetStatus {
                replicas: 1,
                allocated_replicas: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            rolling_update_active_replicas(
                &fleet,
                &active,
                &[inactive.clone()],
            )
            .unwrap(),
            4
        );
        assert_eq!(
            fleet_active_replicas_target(&fleet, &active, &[inactive])
                .unwrap(),
            4
        );
    }

    #[test]
    fn deployment_strategy_shortcuts_match_upstream() {
        let fleet = rollout_fleet(10);
        let active = GameServerSet::default();
        assert_eq!(
            fleet_active_replicas_target(&fleet, &active, &[]).unwrap(),
            10
        );

        let scaled_down = GameServerSet {
            spec: GameServerSetSpec {
                replicas: 0,
                ..Default::default()
            },
            status: crate::gameserverset::GameServerSetStatus {
                allocated_replicas: 1,
                reserved_replicas: 2,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            fleet_active_replicas_target(
                &fleet,
                &active,
                &[scaled_down],
            )
            .unwrap(),
            7
        );
    }

    #[test]
    fn recreate_target_accounts_for_allocated_inactive_servers() {
        let mut fleet = rollout_fleet(10);
        fleet.spec.strategy.strategy_type =
            Some(DeploymentStrategyType::Recreate);
        let active = GameServerSet::default();
        let inactive = GameServerSet {
            spec: GameServerSetSpec {
                replicas: 10,
                ..Default::default()
            },
            status: crate::gameserverset::GameServerSetStatus {
                allocated_replicas: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            fleet_active_replicas_target(&fleet, &active, &[inactive])
                .unwrap(),
            9
        );
    }


    fn rollout_set(
        name: &str,
        spec_replicas: i32,
        status_replicas: i32,
        ready_replicas: i32,
        allocated_replicas: i32,
    ) -> GameServerSet {
        GameServerSet {
            metadata: ObjectMeta {
                name: name.into(),
                namespace: "default".into(),
                ..Default::default()
            },
            spec: GameServerSetSpec {
                replicas: spec_replicas,
                ..Default::default()
            },
            status: crate::gameserverset::GameServerSetStatus {
                replicas: status_replicas,
                ready_replicas,
                allocated_replicas,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn recreate_deployment_plan_scales_inactive_sets_to_zero() {
        let mut fleet = rollout_fleet(10);
        fleet.spec.strategy.strategy_type =
            Some(DeploymentStrategyType::Recreate);
        let active = rollout_set("active", 0, 0, 0, 0);
        let inactive = rollout_set("inactive", 10, 10, 9, 1);
        let already_scaled = rollout_set("scaled", 0, 1, 0, 1);

        let plan = fleet_deployment_plan(
            &fleet,
            &active,
            &[inactive, already_scaled],
        )
        .unwrap();

        // Recreate reserves room for Allocated GameServers in every inactive
        // set, including sets whose desired replicas are already zero.
        assert_eq!(plan.active_replicas, 8);
        assert_eq!(
            plan.inactive_scales,
            vec![FleetGameServerSetScale {
                namespace: "default".into(),
                name: "inactive".into(),
                from_replicas: 10,
                to_replicas: 0,
                reason: FleetGameServerSetScaleReason::Recreate,
            }]
        );
    }

    #[test]
    fn rolling_update_plan_matches_upstream_full_inactive_case() {
        let mut fleet = rollout_fleet(100);
        fleet.status.ready_replicas = 100;
        fleet
            .spec
            .strategy
            .rolling_update
            .as_mut()
            .unwrap()
            .max_unavailable = Some(IntOrPercent::String("30%".into()));

        let active = rollout_set("active", 0, 0, 0, 0);
        let inactive = rollout_set("inactive", 100, 100, 100, 0);

        let plan =
            fleet_deployment_plan(&fleet, &active, &[inactive]).unwrap();

        assert_eq!(plan.active_replicas, 25);
        assert_eq!(plan.inactive_scales.len(), 1);
        assert_eq!(plan.inactive_scales[0].from_replicas, 100);
        assert_eq!(plan.inactive_scales[0].to_replicas, 70);
        assert_eq!(
            plan.inactive_scales[0].reason,
            FleetGameServerSetScaleReason::RollingUpdate
        );
    }

    #[test]
    fn rolling_update_plan_preserves_allocated_capacity() {
        let mut fleet = rollout_fleet(100);
        fleet.status.ready_replicas = 85;
        fleet
            .spec
            .strategy
            .rolling_update
            .as_mut()
            .unwrap()
            .max_unavailable = Some(IntOrPercent::String("30%".into()));

        let active = rollout_set("active", 75, 75, 75, 0);
        let inactive = rollout_set("inactive", 10, 10, 10, 5);

        let plan =
            fleet_deployment_plan(&fleet, &active, &[inactive]).unwrap();

        assert_eq!(plan.active_replicas, 95);
        assert_eq!(plan.inactive_scales.len(), 1);
        assert_eq!(plan.inactive_scales[0].to_replicas, 0);
    }

    #[test]
    fn rolling_update_plan_uses_ready_replica_fencepost_for_small_fleets() {
        let mut fleet = rollout_fleet(5);
        fleet.status.ready_replicas = 5;
        fleet
            .spec
            .strategy
            .rolling_update
            .as_mut()
            .unwrap()
            .max_unavailable = Some(IntOrPercent::String("30%".into()));

        let active = rollout_set("active", 0, 0, 0, 0);
        let inactive = rollout_set("inactive", 5, 5, 5, 2);

        let plan =
            fleet_deployment_plan(&fleet, &active, &[inactive]).unwrap();

        assert_eq!(plan.active_replicas, 2);
        assert_eq!(plan.inactive_scales[0].to_replicas, 4);
    }

    #[test]
    fn rolling_update_plan_does_not_scale_down_before_active_is_ready() {
        let mut fleet = rollout_fleet(75);
        fleet.status.ready_replicas = 58;

        let active = rollout_set("active", 75, 75, 50, 0);
        let inactive = rollout_set("inactive", 10, 10, 8, 0);

        let plan =
            fleet_deployment_plan(&fleet, &active, &[inactive]).unwrap();

        assert!(plan.inactive_scales.is_empty());
    }

    #[test]
    fn rolling_update_cleanup_removes_unhealthy_replicas_first() {
        let mut fleet = rollout_fleet(100);
        fleet.status.ready_replicas = 80;
        fleet
            .spec
            .strategy
            .rolling_update
            .as_mut()
            .unwrap()
            .max_unavailable = Some(IntOrPercent::String("25%".into()));

        let active = rollout_set("active", 80, 80, 80, 0);
        let mut inactive = rollout_set("inactive", 20, 20, 0, 5);
        inactive.status.reserved_replicas = 0;

        let plan = rolling_update_inactive_scale_plan(
            &fleet,
            &active,
            &[inactive],
        )
        .unwrap();

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan[0].reason,
            FleetGameServerSetScaleReason::UnhealthyCleanup
        );
        assert_eq!(plan[0].from_replicas, 20);
        assert_eq!(plan[0].to_replicas, 5);
    }

    #[test]
    fn rolling_update_missing_max_unavailable_is_an_error() {
        let mut fleet = rollout_fleet(10);
        fleet
            .spec
            .strategy
            .rolling_update
            .as_mut()
            .unwrap()
            .max_unavailable = None;
        fleet.status.ready_replicas = 10;

        let active = rollout_set("active", 0, 0, 0, 0);
        let inactive = rollout_set("inactive", 10, 10, 10, 0);

        assert!(
            fleet_deployment_plan(&fleet, &active, &[inactive])
                .unwrap_err()
                .contains("maxUnavailable")
        );
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
    fn active_template_uses_kubernetes_resource_quantity_semantics() {
        let mut fleet = Fleet {
            spec: FleetSpec {
                template: GameServerTemplateSpec {
                    spec: GameServerSpec {
                        template: PodTemplateSpec {
                            spec: PodSpec {
                                containers: vec![Container {
                                    name: "game".into(),
                                    image: "game:v1".into(),
                                    extensions: BTreeMap::from([(
                                        "resources".into(),
                                        serde_json::json!({
                                            "limits": {
                                                "cpu": "1000m",
                                                "memory": "1Gi"
                                            }
                                        }),
                                    )]),
                                    ..Default::default()
                                }].into(),
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
        fleet.apply_defaults();

        let mut equivalent = fleet.game_server_set();
        equivalent.metadata.name = "equivalent".into();
        equivalent.spec.template.spec.template.spec.containers[0]
            .extensions
            .insert(
                "resources".into(),
                serde_json::json!({
                    "limits": {
                        "cpu": "1",
                        "memory": "1024Mi"
                    }
                }),
            );

        assert!(game_server_templates_semantically_equal(
            &fleet.spec.template,
            &equivalent.spec.template,
        ));

        let (active, rest) =
            filter_game_server_sets_by_active(&fleet, &[equivalent.clone()]);
        assert_eq!(active, Some(equivalent));
        assert!(rest.is_empty());
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
