//! Typed source identities and static utility provider composition.
use derive_deftly::define_derive_deftly;

define_derive_deftly! {
    SourceErrorOrigin:
    ${impl crate::errors::SourceErrorOrigin} {
        const SOURCE_PACKAGE: &'static str = ${tmeta(source_package) as str};
        const SOURCE_STRUCT: &'static str = ${tmeta(source_struct) as str};
    }
}
pub(crate) use derive_deftly_template_SourceErrorOrigin;

define_derive_deftly! {
    UtilityFeatureProviders:
    ::cgp::prelude::delegate_and_check_components! {
        $tname {
            crate::features::FeatureStateGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("feature_state")>,
            crate::features::FeatureBindingsGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("feature_bindings")>,
            crate::features::UtilityFeatureEnvironmentReaderComponent:
                crate::features::ReadUtilityFeatureEnvironment,
            crate::features::UtilityFeatureInputReaderComponent:
                crate::features::ReadBoundFeatureInput,
            agones_core::FeatureGateDefaultsGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("feature_gate_defaults")>,
            agones_core::FeatureGateParserComponent: agones_core::ParseAgonesFeatureGates,
            agones_core::AgonesBuildVersionGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("agones_build_version")>,
            agones_core::AgonesVersionAnnotationGetterComponent:
                agones_core::EncodeAgonesVersionAnnotation,
            crate::logfields::ResourceLogAugmenterComponent:
                crate::logfields::AugmentSourceResourceLog,
        }
    }
}
pub(crate) use derive_deftly_template_UtilityFeatureProviders;

define_derive_deftly! {
    UtilityCrdProviders:
    ::cgp::prelude::delegate_and_check_components! {
        $tname {
            crate::utility_crd::RusternetesCrdConnectionGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("rusternetes_crd_connection")>,
            crate::crd::CrdReaderComponent: crate::utility_crd::ReadCrdFromRusternetes,
            crate::crd::CrdPollClockComponent: crate::crd::UseTokioCrdPollClock,
            crate::crd::CrdEstablishmentLoggerComponent: crate::utility_crd::LogEstablishedCrd,
            crate::crd::CrdEstablishmentWaiterComponent: crate::crd::WaitForSourceCrdEstablishment,
        }
    }
}
pub(crate) use derive_deftly_template_UtilityCrdProviders;
