use cgp::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::{
    AgonesBuildVersion, AgonesBuildVersionGetterComponent,
    AgonesVersionAnnotationGetterComponent, CanGetAgonesVersionAnnotation,
    EncodeAgonesVersionAnnotation, GameServerSet, GameServerSetSpec,
    Fleet, DeploymentStrategyType, IntOrPercent, RollingUpdateDeployment,
    SchedulingStrategy, ValidationError, ValidationErrorKind,
    FLEET_NAME_LABEL, VERSION_ANNOTATION,
};

/// Context-generic Fleet -> GameServerSet constructor.
#[cgp_component(FleetGameServerSetBuilder)]
pub trait CanBuildFleetGameServerSet {
    fn build_fleet_game_server_set(&self, fleet: &Fleet) -> GameServerSet;
}

#[cgp_new_provider]
impl<Context> FleetGameServerSetBuilder<Context> for BuildAgonesFleetGameServerSet {
    fn build_fleet_game_server_set(_context: &Context, fleet: &Fleet) -> GameServerSet {
        let mut set = GameServerSet {
            metadata: fleet.spec.template.metadata.clone(),
            spec: GameServerSetSpec {
                template: fleet.spec.template.clone(),
                scheduling: fleet.spec.scheduling.clone(),
                allocation_overflow: fleet.spec.allocation_overflow.clone(),
                priorities: fleet.spec.priorities.clone(),
                ..Default::default()
            },
            ..Default::default()
        };
        let metadata = &mut set.metadata;
        metadata.generate_name = format!("{}-", fleet.metadata.name);
        metadata.name.clear();
        metadata.namespace.clone_from(&fleet.metadata.namespace);
        metadata.resource_version = None;
        metadata.uid.clear();

        let owner = serde_json::json!({
            "apiVersion": "agones.dev/v1",
            "kind": "Fleet",
            "name": fleet.metadata.name,
            "uid": fleet.metadata.uid,
            "controller": true,
            "blockOwnerDeletion": true,
        });
        match metadata.extensions.get_mut("ownerReferences") {
            Some(serde_json::Value::Array(owners)) => owners.push(owner),
            _ => {
                metadata.extensions.insert(
                    "ownerReferences".to_owned(),
                    serde_json::Value::Array(vec![owner]),
                );
            }
        }
        metadata.labels.insert(FLEET_NAME_LABEL.to_owned(), fleet.metadata.name.clone());
        set
    }
}

/// Independently replaceable defaulting policy, parameterized by version source.
#[cgp_component(FleetDefaulter)]
pub trait CanApplyFleetDefaults {
    fn default_fleet(&self, fleet: &mut Fleet);
}

#[cgp_new_provider]
impl<Context> FleetDefaulter<Context> for ApplyAgonesFleetDefaults
where
    Context: CanGetAgonesVersionAnnotation,
{
    fn default_fleet(context: &Context, fleet: &mut Fleet) {
        if fleet.spec.strategy.strategy_type.is_none() {
            fleet.spec.strategy.strategy_type = Some(DeploymentStrategyType::RollingUpdate);
        }
        if fleet.spec.scheduling.is_none() {
            fleet.spec.scheduling = Some(SchedulingStrategy::Packed);
        }
        if fleet.spec.strategy.strategy_type == Some(DeploymentStrategyType::RollingUpdate) {
            fleet.spec.strategy.rolling_update
                .get_or_insert_with(RollingUpdateDeployment::default)
                .apply_threshold_defaults();
        }
        fleet.metadata.annotations.insert(
            VERSION_ANNOTATION.to_owned(),
            context.agones_version_annotation(),
        );
    }
}

/// Validate only deployment strategy. Template and API hook checks remain
/// separate, so their source-defined order is preserved in Fleet::validate.
#[cgp_component(FleetStrategyValidator)]
pub trait CanValidateFleetStrategy {
    fn validate_fleet_strategy(&self, fleet: &Fleet) -> Vec<ValidationError>;
}

#[cgp_new_provider]
impl<Context> FleetStrategyValidator<Context> for ValidateAgonesFleetStrategy {
    fn validate_fleet_strategy(_context: &Context, fleet: &Fleet) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        match fleet.spec.strategy.strategy_type.as_ref() {
            Some(DeploymentStrategyType::RollingUpdate) => {
                if let Some(rolling) = &fleet.spec.strategy.rolling_update {
                    for (value, path) in [
                        (&rolling.max_unavailable, "spec.strategy.rollingUpdate.maxUnavailable"),
                        (&rolling.max_surge, "spec.strategy.rollingUpdate.maxSurge"),
                    ] {
                        if let Some(value) = value {
                            if let Some(error) = super::fleet::validate_rolling_update(value, path) {
                                errors.push(error);
                            }
                        }
                    }
                }
            }
            Some(DeploymentStrategyType::Recreate) => {}
            other => {
                errors.push(ValidationError {
                    origin: String::new(),
                    covered_by_declarative: false,
                    kind: ValidationErrorKind::NotSupported,
                    path: "spec.strategy.type".to_owned(),
                    bad_value: Some(other.map_or("", DeploymentStrategyType::as_str).to_owned()),
                    detail: "supported values: RollingUpdate, Recreate".to_owned(),
                });
            }
        }
        errors
    }
}

/// A single type-level wiring table for the Fleet domain operations.
#[derive(Clone, Debug, Default, HasField)]
pub struct FleetDomainContext {
    pub agones_build_version: AgonesBuildVersion,
}

delegate_and_check_components! {
    FleetDomainContext {
        AgonesBuildVersionGetterComponent: UseField<Symbol!("agones_build_version")>,
        AgonesVersionAnnotationGetterComponent: EncodeAgonesVersionAnnotation,
        FleetGameServerSetBuilderComponent: BuildAgonesFleetGameServerSet,
        FleetDefaulterComponent: ApplyAgonesFleetDefaults,
        FleetStrategyValidatorComponent: ValidateAgonesFleetStrategy,
    }
}

/// The Go FleetList runtime object, including ListMeta and nil-vs-empty items.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetList {
    #[serde(flatten)]
    pub type_meta: crate::TypeMeta,
    #[serde(default)]
    pub metadata: FleetListMeta,
    #[serde(default)]
    pub items: crate::GoSlice<Fleet>,
}

/// Metadata carried by a list response. Unknown extensions are retained.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetListMeta {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub self_link: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    #[serde(rename = "continue", default, skip_serializing_if = "String::is_empty")]
    pub continue_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_item_count: Option<i64>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}