//! Typed source identities and static utility provider composition.
use derive_deftly::define_derive_deftly;

define_derive_deftly! {
    SourceErrorOrigin:
    ${impl crate::errors::SourceErrorOrigin} {
        const SOURCE_PACKAGE: &'static str = ${tmeta(source_package) as str};
        const SOURCE_STRUCT: &'static str = ${tmeta(source_struct) as str};
    }
    ${if tmeta(source_log_type) {
        ${impl crate::runtime::SourceLogType} {
            const SOURCE_LOG_TYPE: &'static str = ${tmeta(source_log_type) as str};
        }
    }}
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
            crate::utility_crd::CrdRuntimeGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("crd_runtime")>,
            crate::crd::CrdReaderComponent: crate::utility_crd::ReadCrdFromRusternetes,
            crate::crd::CrdPollClockComponent: crate::crd::UseTokioCrdPollClock,
            crate::crd::CrdEstablishmentLoggerComponent: crate::utility_crd::LogEstablishedCrd,
            crate::crd::CrdEstablishmentWaiterComponent: crate::crd::WaitForSourceCrdEstablishment,
        }
    }
}
pub(crate) use derive_deftly_template_UtilityCrdProviders;

define_derive_deftly! {
    UtilityRuntimeProviders:
    ::cgp::prelude::delegate_and_check_components! {
        $tname {
            crate::utility_logging::SourceLoggerGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_logger")>,
            crate::utility_logging::SourceErrorBackoffGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_error_backoff")>,
            crate::utility_logging::SourceLogClockComponent: crate::utility_logging::ReadLocalSourceLogClock,
            crate::utility_logging::SourceLogEmitterComponent: crate::utility_logging::EmitSourceJsonLog,
            crate::utility_logging::SourceRuntimeErrorHandlerComponent: crate::utility_logging::HandleSourceErrorAndBackoff,
            crate::utility_scope::UtilityTaskScopeGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("utility_task_scope")>,
            crate::utility_scope::UtilityTaskSpawnerComponent: crate::utility_scope::SpawnOwnedUtilityTask,
            crate::utility_http::HttpServerRunnerComponent: crate::utility_http::RunNativeHttpServer,
            crate::utility_fswatch::FileWatcherOpenerComponent: crate::utility_fswatch::OpenNativeFileWatcher,
            crate::utility_tls::SourceCertificateLoaderComponent: crate::utility_tls::LoadRustlsCertificate,
            crate::utility_tls::HttpsServerRunnerComponent: crate::utility_tls::RunNativeHttpsServer,
            crate::utility_tls::SourceProcessExitComponent: crate::utility_tls::ExitNativeSourceProcess,
            crate::utility_admission::AdmissionHandlerCallerComponent: crate::utility_admission::CallSourceAdmissionHandler,
            crate::utility_discovery::ApiObjectEncoderComponent: crate::utility_discovery::EncodeSourceMetaObject,
        }
    }
}
pub(crate) use derive_deftly_template_UtilityRuntimeProviders;
