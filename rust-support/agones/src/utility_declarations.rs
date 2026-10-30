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
    SourcePrintfInterfaces:
    ${impl crate::utility_format_methods::SourcePrintfMethodViews} {
        fn source_methods(&self) -> crate::utility_format_methods::SourcePrintfInterfaces<'_> {
            let methods = crate::utility_format_methods::SourcePrintfInterfaces::default();
            ${if tmeta(source_formatter) {
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { formatter: Some(self), ..methods };
            }}
            ${if tmeta(source_go_stringer) {
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { go_stringer: Some(self), ..methods };
            }}
            ${if tmeta(source_error_method) {
                let error_methods = crate::utility_error_query::SourceErrorMethods {
                    declared: true, ..methods.error_methods
                };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { error: Some(self), error_methods, ..methods };
            }}
            ${if tmeta(source_error_is) {
                let error_methods = crate::utility_error_query::SourceErrorMethods {
                    declared: true, is: Some(self), ..methods.error_methods
                };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { error_methods, ..methods };
            }}
            ${if tmeta(source_error_as) {
                let error_methods = crate::utility_error_query::SourceErrorMethods {
                    declared: true, as_target: Some(self), ..methods.error_methods
                };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { error_methods, ..methods };
            }}
            ${if tmeta(source_error_unwrap_one) {
                let error_methods = crate::utility_error_query::SourceErrorMethods {
                    declared: true, unwrap_one: Some(self), ..methods.error_methods
                };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { error_methods, ..methods };
            }}
            ${if tmeta(source_error_unwrap_many) {
                let error_methods = crate::utility_error_query::SourceErrorMethods {
                    declared: true, unwrap_many: Some(self), ..methods.error_methods
                };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { error_methods, ..methods };
            }}
            ${if tmeta(source_error_timeout) {
                let error_methods = crate::utility_error_query::SourceErrorMethods {
                    declared: true, timeout: Some(self), ..methods.error_methods
                };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { error_methods, ..methods };
            }}
            ${if tmeta(source_error_temporary) {
                let error_methods = crate::utility_error_query::SourceErrorMethods {
                    declared: true, temporary: Some(self), ..methods.error_methods
                };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { error_methods, ..methods };
            }}
            ${if tmeta(source_api_status) {
                let error_methods = crate::utility_error_query::SourceErrorMethods {
                    api_status: Some(self), ..methods.error_methods
                };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { error_methods, ..methods };
            }}
            ${if tmeta(source_nil_error_pointer) {
                let error_methods = crate::utility_error_query::SourceErrorMethods {
                    nil_pointer: Some(self), ..methods.error_methods
                };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { error_methods, ..methods };
            }}
            ${if tmeta(source_stringer) {
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { stringer: Some(self), ..methods };
            }}
            ${if tmeta(source_klog_text_writer) {
                let klog = crate::utility_klog_dispatch::SourceKlogInterfaces { text_writer: Some(self), ..methods.klog };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { klog, ..methods };
            }}
            ${if tmeta(source_klog_marshaler) {
                let klog = crate::utility_klog_dispatch::SourceKlogInterfaces { marshaler: Some(self), ..methods.klog };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { klog, ..methods };
            }}
            ${if tmeta(source_klog_json_encoder) {
                let klog = crate::utility_klog_dispatch::SourceKlogInterfaces { json_encoder: Some(self), ..methods.klog };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { klog, ..methods };
            }}
            ${if tmeta(source_slog_value) {
                let klog = crate::utility_klog_dispatch::SourceKlogInterfaces { slog_value: Some(self), ..methods.klog };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { klog, ..methods };
            }}
            ${if tmeta(source_slog_log_valuer) {
                let klog = crate::utility_klog_dispatch::SourceKlogInterfaces { log_valuer: Some(self), ..methods.klog };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { klog, ..methods };
            }}
            methods
        }
    }
}
pub(crate) use derive_deftly_template_SourcePrintfInterfaces;

define_derive_deftly! {
    SourceLogSinkInterfaces:
    ${impl crate::utility_logr::SourceLogSinkMethodViews} {
        fn source_sink_methods(&self) -> crate::utility_logr::SourceLogSinkInterfaces<'_> {
            let methods = crate::utility_logr::SourceLogSinkInterfaces::default();
            ${if tmeta(source_logr_call_depth) {
                let methods = crate::utility_logr::SourceLogSinkInterfaces { call_depth: Some(self), ..methods };
            }}
            ${if tmeta(source_logr_call_stack_helper) {
                let methods = crate::utility_logr::SourceLogSinkInterfaces { call_stack_helper: Some(self), ..methods };
            }}
            methods
        }
    }
}
pub(crate) use derive_deftly_template_SourceLogSinkInterfaces;

// Klog method adapters have one typed `value` and one method closure. Copy
// and lifecycle ownership remain with that value and the existing CGP caller.
define_derive_deftly! {
    SourceKlogAdapter:
    impl<$tgens> crate::utility_format_methods::SourcePrintfMethodViews for $ttype $twheres {
        fn source_methods(&self) -> crate::utility_format_methods::SourcePrintfInterfaces<'_> {
            let methods = self.value.source_methods();
            ${if tmeta(source_klog_text_writer) {
                let klog = crate::utility_klog_dispatch::SourceKlogInterfaces { text_writer: Some(self), ..methods.klog };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { klog, ..methods };
            }}
            ${if tmeta(source_klog_marshaler) {
                let klog = crate::utility_klog_dispatch::SourceKlogInterfaces { marshaler: Some(self), ..methods.klog };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { klog, ..methods };
            }}
            ${if tmeta(source_klog_json_encoder) {
                let klog = crate::utility_klog_dispatch::SourceKlogInterfaces { json_encoder: Some(self), ..methods.klog };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { klog, ..methods };
            }}
            ${if tmeta(source_slog_log_valuer) {
                let klog = crate::utility_klog_dispatch::SourceKlogInterfaces { log_valuer: Some(self), ..methods.klog };
                let methods = crate::utility_format_methods::SourcePrintfInterfaces { klog, ..methods };
            }}
            methods
        }
    }
    impl<$tgens> crate::utility_format_arguments::SourcePrintfValue for $ttype $twheres {
        const SOURCE_KIND_UINT8: bool = Value::SOURCE_KIND_UINT8;
        const SOURCE_DEREFERENCE_AT_ROOT: bool = Value::SOURCE_DEREFERENCE_AT_ROOT;
        fn source_type(&self) -> &str { self.value.source_type() }
        fn source_integer(&self) -> crate::utility_format_scan::SourcePrintfInteger { self.value.source_integer() }
        fn source_is_nil(&self) -> bool { self.value.source_is_nil() }
        fn source_nil_interface_type(&self) -> &str { self.value.source_nil_interface_type() }
        fn source_is_nil_pointer(&self) -> bool { self.value.source_is_nil_pointer() }
        fn source_error_view(&self) -> Option<&(dyn ::std::error::Error + Send + Sync + 'static)> { self.value.source_error_view() }
        fn source_uint8(&self) -> Option<u8> { self.value.source_uint8() }
        fn write_source(&self, verb: char, state: &crate::utility_format_scan::SourcePrintfResolvedCounts,
            profile: crate::utility_http::SourceUnicodeProfile, mode: crate::utility_format_print::SourcePrintfMode,
            recursion: crate::utility_format_methods::SourcePrintfRecursion, output: &mut Vec<u8>)
            -> Result<(), crate::utility_http::SourceSliceAllocationError>
        { self.value.write_source(verb, state, profile, mode, recursion, output) }
        fn into_source_error(self) -> Option<crate::errors::BoxError> { self.value.into_source_error() }
    }
}
pub(crate) use derive_deftly_template_SourceKlogAdapter;

// Concrete nil pointer drivers retain their real source method set. They
// share the nil dereference and pointer formatting bodies, without declaring
// a fictional source Is/As method or a general Go pointer/object interpreter.
define_derive_deftly! {
    SourceNilErrorPointer:
    ${impl crate::utility_error_query::SourceErrorNilPointer} {
        fn source_nil_pointer_target(&self) -> ::std::any::TypeId {
            ::std::any::TypeId::of::<${tmeta(source_error_target) as ty}>()
        }
    }
    ${impl ::std::fmt::Display} {
        fn fmt(&self, _: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            crate::utility_error_nil::nil_error_receiver()
        }
    }
    ${impl ::std::error::Error} {}
    ${impl crate::utility_format_methods::SourceErrorMethod} {
        fn source_error_message(&self) -> ::std::borrow::Cow<'_, [u8]> {
            crate::utility_error_nil::nil_error_receiver()
        }
    }
    ${impl crate::utility_format_arguments::SourcePrintfValue} {
        fn source_type(&self) -> &str { <Self as crate::runtime::SourceLogType>::SOURCE_LOG_TYPE }
        fn source_is_nil_pointer(&self) -> bool { true }
        fn source_error_view(&self) -> Option<&(dyn ::std::error::Error + Send + Sync + 'static)> { Some(self) }
        fn write_source(&self, verb: char, state: &crate::utility_format_scan::SourcePrintfResolvedCounts,
            profile: crate::utility_http::SourceUnicodeProfile, mode: crate::utility_format_print::SourcePrintfMode,
            recursion: crate::utility_format_methods::SourcePrintfRecursion, output: &mut Vec<u8>)
            -> Result<(), crate::utility_http::SourceSliceAllocationError>
        {
            crate::utility_format_pointer::write_source_address(self, 0, verb, state, profile, mode, recursion, output)
        }
        fn into_source_error(self) -> Option<crate::errors::BoxError> {
            Some(crate::utility_error_alias::SourceSharedError::from_original(Box::new(self)).into_error())
        }
    }
}
pub(crate) use derive_deftly_template_SourceNilErrorPointer;

// The concrete filesystem pointer errors share their method/format adapters.
// Each type retains its own exported fields in a typed mutable owner.
define_derive_deftly! {
    SourceFsPointerError:
    ${impl ::std::fmt::Display} {
        fn fmt(&self, output: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::utility_format_methods::SourceErrorMethod;
            output.write_str(&String::from_utf8_lossy(&self.source_error_message()))
        }
    }
    ${impl ::std::fmt::Debug} {
        fn fmt(&self, output: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { ::std::fmt::Display::fmt(self, output) }
    }
    ${impl ::std::error::Error} {}
    ${impl crate::utility_format_methods::SourceErrorMethod} {
        fn source_error_message(&self) -> ::std::borrow::Cow<'_, [u8]> { crate::utility_fs_error::pointer_message(self) }
    }
    ${impl crate::utility_error_query::SourceErrorUnwrapOne} {
        fn source_unwrap_one(&self) -> Option<crate::utility_error_slice::SourceRetainedError> {
            crate::utility_fs_error::SourceFsErrorFields::error_field(self)
        }
    }
    ${if tmeta(source_error_timeout) {
        ${impl crate::utility_error_query::SourceErrorTimeout} {
            fn source_error_timeout(&self) -> bool { crate::utility_fs_error::pointer_timeout(self) }
        }
    }}
    ${impl crate::utility_format_arguments::SourcePrintfValue} {
        fn source_type(&self) -> &str { <Self as crate::runtime::SourceLogType>::SOURCE_LOG_TYPE }
        fn source_error_view(&self) -> Option<&(dyn ::std::error::Error + Send + Sync + 'static)> { Some(self) }
        fn write_source(&self, verb: char, state: &crate::utility_format_scan::SourcePrintfResolvedCounts,
            profile: crate::utility_http::SourceUnicodeProfile, mode: crate::utility_format_print::SourcePrintfMode,
            recursion: crate::utility_format_methods::SourcePrintfRecursion, output: &mut Vec<u8>)
            -> Result<(), crate::utility_http::SourceSliceAllocationError>
        { crate::utility_fs_error::write_pointer(self, verb, state, profile, mode, recursion, output) }
    }
}
pub(crate) use derive_deftly_template_SourceFsPointerError;

// Source value copies are declaration-driven. Each field supplies its own
// scalar/header copy; Rust Clone is not an implicit source-copy operation.
define_derive_deftly! {
    SourceFmtValueCopy:
    impl<$tgens> crate::utility_format_copy::SourceFmtValueCopy for $ttype
    where $( $ftype: crate::utility_format_copy::SourceFmtValueCopy, )
          $twheres
    {
        fn source_fmt_copy(&self) -> Self {
            match self { $(
                $vpat => $vtype { $(
                    $fname: crate::utility_format_copy::SourceFmtValueCopy::source_fmt_copy($fpatname),
                ) },
            ) }
        }
    }
}
pub(crate) use derive_deftly_template_SourceFmtValueCopy;

// Both real ErrorHandlers and keysAndValues drivers fix their source element
// width here. Copies retain backing/window metadata; they never clone bodies.
define_derive_deftly! {
    SourceCallbackSlice:
    impl<$tgens> $ttype $twheres {
        pub fn nil() -> Self {
            let profile = if usize::BITS == 32 { crate::utility_http::HeaderAllocationProfile::GO126_32 }
                else { crate::utility_http::HeaderAllocationProfile::GO126_64 };
            Self::nil_with_profile(profile)
        }
        pub fn nil_with_profile(profile: crate::utility_http::HeaderAllocationProfile) -> Self {
            Self { slice: crate::utility_callback_slice::SourceCallbackSlice::nil(profile, ${tmeta(source_slice_words) as expr}) }
        }
        pub fn make(length: i64, capacity: i64, profile: crate::utility_http::HeaderAllocationProfile) -> Self {
            Self { slice: crate::utility_callback_slice::SourceCallbackSlice::make(length, capacity, profile, ${tmeta(source_slice_words) as expr}) }
        }
        pub fn is_nil(&self) -> bool { self.slice.is_nil() }
        pub fn len(&self) -> usize { self.slice.len() }
        pub fn capacity(&self) -> usize { self.slice.capacity() }
        pub fn allocation_profile(&self) -> crate::utility_http::HeaderAllocationProfile { self.slice.allocation_profile() }
        pub fn get(&self, index: usize) -> ${tmeta(source_slice_value) as ty} {
            self.get_source(crate::utility_error_slice::SourceErrorSliceIndex::unsigned(index as u64))
        }
        pub fn get_source(&self, index: crate::utility_error_slice::SourceErrorSliceIndex) -> ${tmeta(source_slice_value) as ty} {
            self.slice.get(index)
        }
        pub fn set(&self, index: usize, value: ${tmeta(source_slice_value) as ty}) -> ${tmeta(source_slice_value) as ty} {
            self.set_source(crate::utility_error_slice::SourceErrorSliceIndex::unsigned(index as u64), value)
        }
        pub fn set_source(&self, index: crate::utility_error_slice::SourceErrorSliceIndex, value: ${tmeta(source_slice_value) as ty})
            -> ${tmeta(source_slice_value) as ty} { self.slice.set(index, value) }
        pub fn slice2(&self, low: crate::utility_error_slice::SourceErrorSliceIndex, high: crate::utility_error_slice::SourceErrorSliceIndex) -> Self {
            Self { slice: self.slice.slice2(low, high) }
        }
        pub fn slice3(&self, low: crate::utility_error_slice::SourceErrorSliceIndex, high: crate::utility_error_slice::SourceErrorSliceIndex,
            maximum: crate::utility_error_slice::SourceErrorSliceIndex) -> Self { Self { slice: self.slice.slice3(low, high, maximum) } }
        pub fn append(&mut self, value: ${tmeta(source_slice_value) as ty}) { self.slice.append(value); }
        pub fn try_append(&mut self, value: ${tmeta(source_slice_value) as ty}) -> Result<(), crate::utility_http::SourceSliceAllocationError> {
            self.slice.try_append(value)
        }
    }
    ${impl crate::utility_format_copy::SourceFmtValueCopy} {
        fn source_fmt_copy(&self) -> Self { Self { slice: crate::utility_format_copy::SourceFmtValueCopy::source_fmt_copy(&self.slice) } }
    }
}
pub(crate) use derive_deftly_template_SourceCallbackSlice;

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
            crate::utility_klog::SourceKlogRecordEmitterComponent: crate::utility_klog::EmitNativeSourceKlogRecord,
            crate::utility_klog_daemon::SourceKlogFlushDaemonControllerComponent:
                crate::utility_klog_daemon::ControlNativeSourceKlogFlushDaemon,
            crate::utility_klog_file_env::SourceKlogFileInitializerComponent:
                crate::utility_klog_file_env::InitializeOwnedSourceKlogFiles,
            crate::utility_logging::SourceErrorBackoffGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_error_backoff")>,
            crate::utility_error_handlers::SourceErrorHandlerStateGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("source_error_handler_state")>,
            crate::utility_error_handlers::SourceRuntimeErrorDispatcherComponent: crate::utility_error_handlers::DispatchSourceRuntimeError,
            crate::utility_error_handlers::SourceRuntimeErrorClockComponent: crate::utility_error_handlers::ReadNativeSourceErrorClock,
            crate::utility_error_handlers::SourceRuntimeErrorDelayComponent: crate::utility_error_handlers::WaitNativeSourceErrorDelay,
            crate::utility_logging::SourceLogClockComponent: crate::utility_logging::ReadLocalSourceLogClock,
            crate::utility_logging::SourceLogEmitterComponent: crate::utility_logging::EmitSourceJsonLog,
            crate::utility_logging::SourceErrorLogEmitterComponent: crate::utility_logging::EmitSourceJsonErrorLog,
            crate::utility_logging::SourceRuntimeErrorHandlerComponent: crate::utility_logging::HandleNativeRuntimeError,
            crate::utility_scope::UtilityTaskScopeGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("utility_task_scope")>,
            crate::utility_scope::UtilityTaskSpawnerComponent: crate::utility_scope::SpawnOwnedUtilityTask,
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
