//! Shared production configuration, registered by agones-core.
//!
//! agones-source owns build identity, and agones-core owns feature policy.
//! Keeping this composition in registration support avoids a dependency cycle.
use agones_source::derive_deftly_template_AgonesConfigurationProviders;
use cgp::prelude::*;
use derive_deftly::Deftly;

use crate::{AgonesBuildVersion, FeatureGateContext, FeatureGateInputOptions};

#[derive(Clone, Debug, HasField, Deftly)]
#[derive_deftly(AgonesConfigurationProviders)]
#[deftly(feature_configuration)]
pub struct AgonesConfigurationContext {
    pub agones_build_version: AgonesBuildVersion,
    pub feature_gate_defaults: &'static [(&'static str, bool)],
    pub feature_gate_input_options: FeatureGateInputOptions,
}

impl Default for AgonesConfigurationContext {
    fn default() -> Self {
        Self {
            agones_build_version: AgonesBuildVersion::default(),
            feature_gate_defaults: FeatureGateContext::default().feature_gate_defaults,
            feature_gate_input_options: FeatureGateInputOptions::default(),
        }
    }
}

#[path = "lifecycle.rs"]
mod lifecycle;
pub use lifecycle::InitializedAgonesConfiguration;
