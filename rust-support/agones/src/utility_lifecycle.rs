//! Feature state is consumed from configuration or created empty, then shared
//! by the parent application. This phase owns no worker/server child tasks.
use cgp::prelude::*;
use derive_deftly::Deftly;
use agones_core::{AgonesBuildVersion, AgonesConfigurationContext, FeatureGateError,
    FeatureGates, InitializedAgonesConfiguration};
use crate::{errors::Errors, features::{self, FeatureBindings, FeatureState},
    logfields::{CanAugmentResourceLog, LogEntry, ResourceType},
    utility_declarations::{derive_deftly_template_SourceErrorOrigin, derive_deftly_template_UtilityFeatureProviders}};

#[derive(HasField, Deftly)]
#[derive_deftly(UtilityFeatureProviders, SourceErrorOrigin)]
#[deftly(source_package = "agones.dev/agones/pkg/util/runtime", source_struct = "")]
pub struct UtilityFeatureContext {
    pub agones_build_version: AgonesBuildVersion,
    pub feature_gate_defaults: &'static [(&'static str, bool)],
    pub feature_state: FeatureState,
    pub feature_bindings: FeatureBindings,
}
impl Default for UtilityFeatureContext {
    fn default() -> Self {
        Self::from_parts(AgonesConfigurationContext::default(), FeatureGates::default(), "FEATURE-GATES")
    }
}
impl UtilityFeatureContext {
    fn from_parts(configuration: AgonesConfigurationContext, gates: FeatureGates, environment_key: &str) -> Self {
        let AgonesConfigurationContext { agones_build_version, feature_gate_defaults, feature_gate_input_options } = configuration;
        Self { agones_build_version, feature_gate_defaults,
            feature_state: FeatureState::from_owned(gates),
            feature_bindings: FeatureBindings::with_options(feature_gate_input_options, environment_key.to_owned()) }
    }

    /// Move the M0001 initialized map into its synchronized runtime owner.
    /// No second mutable feature map is cloned or kept by the old phase.
    pub fn from_initialized(configuration: InitializedAgonesConfiguration) -> Self {
        let (configuration, gates) = configuration.into_parts();
        // M0001's concrete input policy uses the '-' -> '_' environment mapping.
        let context = Self::from_parts(configuration, gates, agones_core::FEATURE_GATE_ENV);
        let _ = features::FeaturesBindEnv(&context);
        context
    }

    pub fn bind_source_inputs(&self) {
        features::FeaturesBindFlags(self);
        let _ = features::FeaturesBindEnv(self);
    }
    pub fn parse_source_inputs(&self) -> Result<(), FeatureGateError> { features::ParseFeaturesFromEnv(self) }
    pub fn source_errors(&self) -> Errors { Errors::FromStruct::<Self>(None, None) }
    pub fn resource_log(&self, entry: &LogEntry, resource: &ResourceType, id: &str) -> LogEntry {
        self.augment_resource_log(entry, resource, id)
    }
}
