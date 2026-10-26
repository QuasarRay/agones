//! The build context declaration owns static CGP provider selection.
use derive_deftly::define_derive_deftly;

define_derive_deftly! {
    NativeBuildProviders:
    ::cgp::prelude::delegate_and_check_components! {
        $tname {
            crate::source_io::BuildIoGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("build_io")>,
            crate::lifecycle::BuildProjectRootGetterComponent:
                crate::lifecycle::ReadBuildIoProjectRoot,
            crate::singletons::BuildSingletonsGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("build_singletons")>,
            crate::build_contracts::BuildConfigurationLoaderComponent:
                crate::build_contracts::LoadNativeBuildConfiguration,
            crate::build_contracts::ProtobufBuildTaskComponent:
                crate::build_contracts::RunNativeProtobufBuildTask,
            crate::build_contracts::NativeTargetExecutorComponent:
                crate::build_contracts::ExecuteTargetWithCargo,
            cpg_rs::schema_tools::NativeSchemaFactoryComponent:
                cpg_rs::schema_tools::ConstructPinnedCpgSchema,
            cpg_rs::schema_tools::CpgProtobufGeneratorComponent:
                cpg_rs::schema_tools::GenerateCpgProtocol,
            cpg_rs::schema_tools::CpgSchemaJsonExporterComponent:
                cpg_rs::schema_tools::ExportCpgSchemaJson,
            cpg_rs::protobuf_bindings::CpgProtocolBindingGeneratorComponent:
                cpg_rs::protobuf_bindings::GenerateCpgProtocolBindings,
            crate::stages::NativeBuildStageRunnerComponent:
                crate::stages::RunNativeBuildStages,
            crate::source_tasks::SourceBuildTaskRunnerComponent:
                crate::source_tasks::RunSourceBuildTasks,
        }
    }
}
pub(crate) use derive_deftly_template_NativeBuildProviders;
