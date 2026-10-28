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
    NativeSourceCollectionProviders:
    ::cgp::prelude::delegate_and_check_components! {
        $tname {
            crate::utility_http::SourceCollectionFailureRaiserComponent: crate::utility_http::RaiseSourceCollectionFailure,
            crate::utility_tls::SourceProcessExitComponent: crate::utility_tls::ExitNativeSourceProcess,
        }
    }
}
pub(crate) use derive_deftly_template_NativeSourceCollectionProviders;

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
            crate::utility_runtime_panic::SourceRuntimePanicReporterComponent: crate::utility_runtime_panic::ReportNativeSourceRuntimePanic,
            crate::utility_runtime_panic::SourceUtilityPanicHandlerComponent: crate::utility_runtime_panic::ExitAfterSourceUtilityPanic,
            crate::utility_crash::SourceCrashStateGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_crash_state")>,
            crate::utility_crash::SourceCrashHandlerComponent: crate::utility_crash::HandleSourceCrash,
            crate::utility_crash::SourceCrashPanicReporterComponent: crate::utility_crash::ReportNativeSourceCrashPanic,
            crate::utility_scope::UtilityTaskFailureObserverComponent: crate::utility_scope::ObserveOwnedUtilityTaskFailure,
            crate::utility_http::HttpServerRunnerComponent: crate::utility_http::RunNativeHttpServer,
            crate::utility_http::SourceHttpClockComponent: crate::utility_http::ReadUtcSourceHttpClock,
            crate::utility_http::SourceHttpDiagnosticComponent: crate::utility_http::EmitNativeSourceHttpDiagnostic,
            crate::utility_http::SourceHttpPanicReporterComponent: crate::utility_http::ReportNativeSourceHttpPanic,
            crate::utility_http::SourceHttpDelayComponent: crate::utility_http::WaitTokioSourceHttpDelay,
            crate::utility_http::SourceRequestBodyIoComponent: crate::utility_http::AccessSourceRequestBody,
            crate::utility_http::SourceQuerySettingsGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_query_settings")>,
            crate::utility_http::SourceUnicodeProfileGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_unicode_profile")>,
            crate::utility_http::SourceUppercaseMapperComponent: crate::utility_http::MapSourceUppercase,
            crate::utility_gateway::SourcePatternLoggerGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_pattern_logger")>,
            crate::utility_gateway::SourcePatternDiagnosticEmitterComponent: crate::utility_gateway::EmitSourcePatternDiagnostic,
            crate::utility_gateway::SourceGatewayPatternConstructorComponent: crate::utility_gateway::ConstructSourceGatewayPattern,
            crate::utility_gateway::SourcePatternStringCompletionComponent: crate::utility_gateway::CompleteSourcePatternString,
            crate::utility_http::SourceCollectionFailureRaiserComponent: crate::utility_http::RaiseSourceCollectionFailure,
            crate::utility_fswatch::FileWatcherOpenerComponent: crate::utility_fswatch::OpenNativeFileWatcher,
            crate::utility_tls::SourceCertificateLoaderComponent: crate::utility_tls::LoadRustlsCertificate,
            crate::utility_tls::HttpsServerRunnerComponent: crate::utility_tls::RunNativeHttpsServer,
            crate::utility_tls::SourceProcessExitComponent: crate::utility_tls::ExitNativeSourceProcess,
            crate::utility_admission::AdmissionHandlerCallerComponent: crate::utility_admission::CallSourceAdmissionHandler,
            crate::utility_discovery::ApiObjectEncoderComponent: crate::utility_discovery::EncodeSourceMetaObject,
            crate::utility_signals::SourceSignalStateGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_signal_state")>,
            crate::utility_signals::SourceSignalSubscriberComponent: crate::utility_signals::SubscribeNativeSourceSignals,
            crate::utility_client_config::SourceClientConfigurationReaderComponent: crate::utility_client_config::ReadRusternetesSourceConfiguration,
            crate::utility_gateway::SourceServerMuxFactoryComponent: crate::utility_gateway::CreateNativeSourceServerMux,
        }
    }
}
pub(crate) use derive_deftly_template_UtilityRuntimeProviders;

define_derive_deftly! {
    UtilityQueueProviders:
    ::cgp::prelude::delegate_and_check_components! {
        <$tgens> $ttype {
            crate::utility_queue::SourceQueueStateGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_queue_state")>,
            crate::utility_queue::SourceQueueRuntimeGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_queue_runtime")>,
            crate::utility_queue::SourceQueueClockComponent: crate::utility_queue::ReadTokioSourceQueueClock,
            crate::utility_queue::SourceQueueDelayDeciderComponent: crate::utility_queue::UseOwnedSourceQueueLimiter,
            crate::utility_queue::SourceQueueKeyReaderComponent: crate::utility_queue::ReadSourceMetadataKey,
            crate::utility_queue::SourceQueueOperationsComponent: crate::utility_queue::UseOwnedSourceQueue,
            crate::utility_queue::SourceQueueHandlerCallerComponent: crate::utility_queue::CallSourceQueueHandler,
            crate::utility_queue::SourceQueueErrorClassifierComponent: crate::utility_queue::ClassifySourceQueueError,
        }
    }
}
pub(crate) use derive_deftly_template_UtilityQueueProviders;

define_derive_deftly! {
    UtilityGatewayProviders:
    ::cgp::prelude::delegate_and_check_components! {
        $tname {
            crate::utility_gateway::GatewayStateGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("gateway_state")>,
            crate::utility_gateway::GatewayRuntimeGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("gateway_runtime")>,
            crate::utility_gateway::GatewayCodecComponent: crate::utility_gateway::UseSourceGatewayJsonPb,
            crate::utility_gateway::GatewayHandlerCallerComponent: crate::utility_gateway::CallSourceGatewayHandler,
            crate::utility_gateway::GatewayDispatcherComponent: crate::utility_gateway::DispatchSourceGateway,
            crate::utility_gateway::GatewayResponseWriterComponent: crate::utility_gateway::WriteSourceGatewayResponse,
        }
    }
}
pub(crate) use derive_deftly_template_UtilityGatewayProviders;
