//! The configuration phase of the application lifecycle.
//!
//! Initialization performs the source environment lookup through CGP. A failed
//! parse returns its original error; a successful result owns the parsed map.
//! This phase creates no child tasks or open I/O resources. Controller/server
//! supervision, cancellation and drain belong to the later entrypoint milestone.
use super::AgonesConfigurationContext;
use crate::{CanLoadFeatureGates, FeatureGateError, FeatureGates};

#[derive(Clone, Debug)]
pub struct InitializedAgonesConfiguration {
    context: AgonesConfigurationContext,
    feature_gates: FeatureGates,
}

impl AgonesConfigurationContext {
    pub fn initialize(self) -> Result<InitializedAgonesConfiguration, FeatureGateError> {
        // Parse before transferring ownership. No partial initialized context
        // escapes if environment decoding or source query validation fails.
        let feature_gates = self.load_feature_gates()?;
        Ok(InitializedAgonesConfiguration { context: self, feature_gates })
    }
}

impl InitializedAgonesConfiguration {
    /// Transfer the canonical parsed state into its runtime owner.
    pub fn into_parts(self) -> (AgonesConfigurationContext, FeatureGates) {
        (self.context, self.feature_gates)
    }

    pub fn context(&self) -> &AgonesConfigurationContext {
        &self.context
    }

    pub fn feature_gates(&self) -> &FeatureGates {
        &self.feature_gates
    }

    pub fn replace_feature_gates(&mut self, query: &[u8]) -> Result<(), FeatureGateError> {
        // FeatureGates retains the active map until replacement parsing succeeds.
        self.feature_gates.replace_bytes(&self.context, query)
    }
}
