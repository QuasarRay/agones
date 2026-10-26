// Source: Agones 6eb4a02588d8e7b581de187f3bc849733a78dcb2,
// pkg/util/runtime/features.go. Copyright Contributors to Agones a Series of
// LF Projects, LLC. Apache-2.0.
#![allow(non_snake_case, non_upper_case_globals)]
use std::{convert::Infallible, ffi::OsString, sync::{Mutex, RwLock}};
use cgp::prelude::*;
use agones_core::{CanParseFeatureGates, FeatureGateError, FeatureGateInputOptions, FeatureGates};

pub type Feature = str;
pub const FeatureGateFlag: &str = agones_core::FEATURE_GATE_FLAG;
pub const FeatureDisableResyncOnSDKServer: &Feature = "DisableResyncOnSDKServer";
pub const FeatureAutopilotPassthroughPort: &Feature = "AutopilotPassthroughPort";
pub const FeaturePortPolicyNone: &Feature = "PortPolicyNone";
pub const FeaturePortRanges: &Feature = "PortRanges";
pub const FeatureRollingUpdateFix: &Feature = "RollingUpdateFix";
pub const FeatureCountsAndLists: &Feature = "CountsAndLists";
pub const FeatureGKEAutopilotExtendedDurationPods: &Feature = "GKEAutopilotExtendedDurationPods";
pub const FeatureScheduledAutoscaler: &Feature = "ScheduledAutoscaler";
pub const FeatureSidecarContainers: &Feature = "SidecarContainers";
pub const FeatureFleetAutoscaleRequestMetaData: &Feature = "FleetAutoscaleRequestMetaData";
pub const FeaturePlayerAllocationFilter: &Feature = "PlayerAllocationFilter";
pub const FeaturePlayerTracking: &Feature = "PlayerTracking";
pub const FeatureWasmAutoscaler: &Feature = "WasmAutoscaler";
pub const FeatureProcessorAllocator: &Feature = "ProcessorAllocator";
pub const FeatureExample: &Feature = "Example";
pub const featureDefaults: &[(&str, bool)] = agones_core::AGONES_FEATURE_DEFAULTS;

/// One application-owned replacement for the source process-global map and
/// locks. The core parser remains the only query/default/encoding owner.
#[derive(Debug, Default)]
pub struct FeatureState {
    gates: RwLock<FeatureGates>,
    pub FeatureTestMutex: Mutex<()>,
}
impl FeatureState {
    pub fn from_owned(gates: FeatureGates) -> Self { Self { gates: RwLock::new(gates), FeatureTestMutex: Mutex::new(()) } }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureFlagDeclaration {
    pub name: &'static str,
    pub default_value: Vec<u8>,
    pub help: &'static str,
}

#[derive(Debug)]
struct BindingValues {
    flag: Option<FeatureFlagDeclaration>,
    default_value: Vec<u8>,
    environment_bound: bool,
    environment_key: String,
    options: FeatureGateInputOptions,
}
#[derive(Debug)]
pub struct FeatureBindings { values: Mutex<BindingValues> }
impl Default for FeatureBindings {
    fn default() -> Self {
        // Viper without an env-key replacer uppercases the key literally.
        // Entrypoints configure FEATURE_GATES when they install '-' -> '_'.
        Self::with_options(FeatureGateInputOptions::default(), "FEATURE-GATES".into())
    }
}
impl FeatureBindings {
    pub fn with_options(options: FeatureGateInputOptions, environment_key: String) -> Self {
        Self { values: Mutex::new(BindingValues { flag: None, default_value: Vec::new(),
            environment_bound: false, environment_key, options }) }
    }
    pub fn set_override(&self, value: Option<Vec<u8>>) {
        self.values.lock().unwrap_or_else(|error| error.into_inner()).options.override_value = value;
    }
    /// The command parser supplies a value only for a changed, bound flag.
    /// Some(empty) remains different from an unchanged flag.
    pub fn set_bound_flag(&self, value: Option<Vec<u8>>) {
        self.values.lock().unwrap_or_else(|error| error.into_inner()).options.flag_value = value;
    }
    pub fn declaration(&self) -> Option<FeatureFlagDeclaration> {
        self.values.lock().unwrap_or_else(|error| error.into_inner()).flag.clone()
    }
    pub fn set_environment_key(&self, key: String) {
        self.values.lock().unwrap_or_else(|error| error.into_inner()).environment_key = key;
    }
}

#[cgp_getter]
pub trait HasFeatureState { fn feature_state(&self) -> &FeatureState; }
#[cgp_getter]
pub trait HasFeatureBindings { fn feature_bindings(&self) -> &FeatureBindings; }

#[cgp_component(UtilityFeatureEnvironmentReader)]
pub trait CanReadUtilityFeatureEnvironment {
    fn read_utility_feature_environment(&self, key: &str) -> Option<Vec<u8>>;
}
#[cgp_new_provider]
impl<Context> UtilityFeatureEnvironmentReader<Context> for ReadUtilityFeatureEnvironment {
    fn read_utility_feature_environment(_context: &Context, key: &str) -> Option<Vec<u8>> {
        std::env::var_os(key).map(OsString::into_encoded_bytes)
    }
}

#[cgp_component(UtilityFeatureInputReader)]
pub trait CanReadUtilityFeatureInput { fn read_utility_feature_input(&self) -> Vec<u8>; }
#[cgp_new_provider]
impl<Context> UtilityFeatureInputReader<Context> for ReadBoundFeatureInput
where Context: HasFeatureBindings + CanReadUtilityFeatureEnvironment,
{
    fn read_utility_feature_input(context: &Context) -> Vec<u8> {
        let (options, bound, key, default) = {
            let values = context.feature_bindings().values.lock().unwrap_or_else(|error| error.into_inner());
            (values.options.clone(), values.environment_bound, values.environment_key.clone(), values.default_value.clone())
        };
        // As in the existing input policy, explicit values short-circuit the
        // process environment effect. Binding itself never reads the value.
        if let Some(value) = options.override_value.or(options.flag_value) { return value; }
        if bound {
            if let Some(value) = context.read_utility_feature_environment(&key).filter(|value| !value.is_empty()) { return value; }
        }
        default
    }
}

pub fn FeaturesBindFlags<Context: HasFeatureBindings + CanReadUtilityFeatureInput>(context: &Context) {
    {
        let mut values = context.feature_bindings().values.lock().unwrap_or_else(|error| error.into_inner());
        values.default_value.clear(); // viper.SetDefault runs before pflag.String.
    }
    let default_value = context.read_utility_feature_input();
    let mut values = context.feature_bindings().values.lock().unwrap_or_else(|error| error.into_inner());
    if values.flag.is_some() { panic!("flag redefined: {FeatureGateFlag}"); }
    values.flag = Some(FeatureFlagDeclaration { name: FeatureGateFlag, default_value,
        help: "Flag to pass in the url query list of feature flags to enable or disable" });
}

pub fn FeaturesBindEnv<Context: HasFeatureBindings>(context: &Context) -> Result<(), Infallible> {
    // BindEnv receives one fixed nonempty key, so its argument-error branch is
    // unreachable in this source function; no environment read occurs here.
    context.feature_bindings().values.lock().unwrap_or_else(|error| error.into_inner()).environment_bound = true;
    Ok(())
}

pub fn ParseFeaturesFromEnv<Context>(context: &Context) -> Result<(), FeatureGateError>
where Context: CanReadUtilityFeatureInput + HasFeatureState + CanParseFeatureGates,
{
    ParseFeatures(context, &context.read_utility_feature_input())
}

pub fn ParseFeatures<Context: HasFeatureState + CanParseFeatureGates>(context: &Context, queryString: &[u8]) -> Result<(), FeatureGateError> {
    let mut gates = context.feature_state().gates.write().unwrap_or_else(|error| error.into_inner());
    // Hold the source write lock through parse and publish only on success.
    // A rejected query leaves the previous physical map intact.
    *gates = context.parse_feature_gates_bytes(queryString)?;
    Ok(())
}
pub fn EnableAllFeatures<Context: HasFeatureState + CanParseFeatureGates>(context: &Context) {
    let mut gates = context.feature_state().gates.write().unwrap_or_else(|error| error.into_inner());
    *gates = context.all_features_enabled();
}
pub fn FeatureEnabled<Context: HasFeatureState>(context: &Context, feature: impl AsRef<[u8]>) -> bool {
    let bytes = feature.as_ref();
    let gates = context.feature_state().gates.read().unwrap_or_else(|error| error.into_inner());
    // Go strings may contain arbitrary bytes. Every declared gate is valid
    // ASCII, so an invalid UTF-8 query is necessarily an absent map key.
    let Ok(feature) = std::str::from_utf8(bytes) else { return false; };
    gates.enabled(feature)
}
pub fn EncodeFeatures<Context: HasFeatureState>(context: &Context) -> String {
    context.feature_state().gates.read().unwrap_or_else(|error| error.into_inner()).encode()
}
