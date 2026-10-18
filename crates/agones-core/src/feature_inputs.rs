//! Source-compatible feature configuration input policy.
//!
//! Command option parsing supplies only an explicitly changed `--feature-gates`
//! value. CGP then owns precedence, the environment effect, and initialization
//! through the ordinary production query parser. Other command options remain
//! the responsibility of their command's parser.

use std::ffi::OsString;

use cgp::prelude::*;

use crate::{
    AGONES_FEATURE_DEFAULTS, CanParseFeatureGates, FEATURE_GATE_ENV, FeatureGateError, FeatureGates,
};

/// The two explicit overrides supported by the source's Viper lookup.
/// `None` means unchanged/unset; `Some(Vec::new())` is an explicit empty value.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FeatureGateInputOptions {
    pub override_value: Option<Vec<u8>>,
    pub flag_value: Option<Vec<u8>>,
}

/// Viper's source order for this key: Set override, changed flag, nonempty
/// environment, then the empty default installed by FeaturesBindFlags.
pub fn select_feature_gate_input<'a>(
    override_value: Option<&'a [u8]>,
    flag_value: Option<&'a [u8]>,
    environment: Option<&'a [u8]>,
) -> &'a [u8] {
    override_value
        .or(flag_value)
        .or_else(|| environment.filter(|value| !value.is_empty()))
        .unwrap_or_default()
}

#[cgp_getter]
pub trait HasFeatureGateInputOptions {
    fn feature_gate_input_options(&self) -> &FeatureGateInputOptions;
}

#[cgp_component(FeatureGateEnvironmentReader)]
pub trait CanReadFeatureGateEnvironment {
    fn read_feature_gate_environment(&self) -> Option<Vec<u8>>;
}

#[cgp_new_provider]
impl<Context> FeatureGateEnvironmentReader<Context> for ReadProcessFeatureGateEnvironment {
    fn read_feature_gate_environment(_context: &Context) -> Option<Vec<u8>> {
        // On the pinned Linux target this is the original OS byte string,
        // including invalid UTF-8. No shared process environment is mutated.
        std::env::var_os(FEATURE_GATE_ENV).map(OsString::into_encoded_bytes)
    }
}

#[cgp_component(FeatureGateConfigurationLoader)]
pub trait CanLoadFeatureGates {
    fn load_feature_gates(&self) -> Result<FeatureGates, FeatureGateError>;
}

#[cgp_new_provider]
impl<Context> FeatureGateConfigurationLoader<Context> for LoadAgonesFeatureGates
where
    Context: HasFeatureGateInputOptions + CanReadFeatureGateEnvironment + CanParseFeatureGates,
{
    fn load_feature_gates(context: &Context) -> Result<FeatureGates, FeatureGateError> {
        let options = context.feature_gate_input_options();
        // Preserve the source effect order: an explicit override never reads
        // the lower-priority environment, even when its value is empty.
        let environment = if options.override_value.is_none() && options.flag_value.is_none() {
            context.read_feature_gate_environment()
        } else {
            None
        };
        let query = select_feature_gate_input(
            options.override_value.as_deref(),
            options.flag_value.as_deref(),
            environment.as_deref(),
        );
        context.parse_feature_gates_bytes(query)
    }
}

/// Production input context; all input and parsing policy is selected by CGP.
#[derive(Clone, Debug, HasField)]
pub struct FeatureGateInputContext {
    pub feature_gate_defaults: &'static [(&'static str, bool)],
    pub feature_gate_input_options: FeatureGateInputOptions,
}

impl Default for FeatureGateInputContext {
    fn default() -> Self {
        Self {
            feature_gate_defaults: AGONES_FEATURE_DEFAULTS,
            feature_gate_input_options: FeatureGateInputOptions::default(),
        }
    }
}

impl FeatureGateInputContext {
    /// Pass a value only when the command parser observed an explicit flag.
    /// This also preserves `--feature-gates=` as a changed, empty string.
    pub fn from_parsed_flag(flag: Option<OsString>) -> Self {
        Self {
            feature_gate_input_options: FeatureGateInputOptions {
                flag_value: flag.map(OsString::into_encoded_bytes),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

delegate_and_check_components! {
    FeatureGateInputContext {
        crate::FeatureGateDefaultsGetterComponent: UseField<Symbol!("feature_gate_defaults")>,
        FeatureGateInputOptionsGetterComponent: UseField<Symbol!("feature_gate_input_options")>,
        FeatureGateEnvironmentReaderComponent: ReadProcessFeatureGateEnvironment,
        crate::FeatureGateParserComponent: crate::ParseAgonesFeatureGates,
        FeatureGateConfigurationLoaderComponent: LoadAgonesFeatureGates,
    }
}
