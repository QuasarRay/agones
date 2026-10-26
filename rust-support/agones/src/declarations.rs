//! One declaration supplies build-identity projections and CGP delegation.
//!
//! The same template serves the version context and the production shared
//! configuration context. The latter opts into its existing feature providers;
//! the template does not introduce another parser or environment reader.
use derive_deftly::define_derive_deftly;

define_derive_deftly! {
    export AgonesConfigurationProviders:
    ::cgp::prelude::delegate_and_check_components! {
        $tname {
            $crate::pkg::version::AgonesBuildVersionGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("agones_build_version")>,
            $crate::pkg::version::AgonesVersionAnnotationGetterComponent:
                $crate::pkg::version::EncodeAgonesVersionAnnotation,
            ${if tmeta(feature_configuration) {
                crate::FeatureGateDefaultsGetterComponent:
                    ::cgp::prelude::UseField<::cgp::prelude::Symbol!("feature_gate_defaults")>,
                crate::FeatureGateInputOptionsGetterComponent:
                    ::cgp::prelude::UseField<::cgp::prelude::Symbol!("feature_gate_input_options")>,
                crate::FeatureGateEnvironmentReaderComponent:
                    crate::ReadProcessFeatureGateEnvironment,
                crate::FeatureGateParserComponent: crate::ParseAgonesFeatureGates,
                crate::FeatureGateConfigurationLoaderComponent: crate::LoadAgonesFeatureGates,
            }}
        }
    }
}
