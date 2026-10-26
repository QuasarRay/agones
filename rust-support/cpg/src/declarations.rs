//! The build context declaration owns static CGP provider selection.
use derive_deftly::define_derive_deftly;

define_derive_deftly! {
    NativeBuildProviders:
    ::cgp::prelude::delegate_and_check_components! {
        $tname {
            crate::source_io::BuildIoGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("build_io")>,
            crate::lifecycle::BuildProjectRootGetterComponent:
                ::cgp::prelude::UseField<::cgp::prelude::Symbol!("build_project_root")>,
            crate::build_contracts::BuildConfigurationLoaderComponent:
                crate::build_contracts::LoadNativeBuildConfiguration,
            crate::build_contracts::ProtobufBuildTaskComponent:
                crate::build_contracts::RunNativeProtobufBuildTask,
            crate::build_contracts::NativeTargetExecutorComponent:
                crate::build_contracts::ExecuteTargetWithCargo,
        }
    }
}
pub(crate) use derive_deftly_template_NativeBuildProviders;
