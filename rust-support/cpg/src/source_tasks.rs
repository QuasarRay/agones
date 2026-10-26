//! Fixed source-task adapters. The retained sbt/Bazel graph remains an input;
//! native Cargo manifests own Rust dependencies and the sole Grafeo runtime.
//! These matches select commands, never runtime providers or extensions.
use std::path::PathBuf;
use cgp::prelude::*;
use crate::{Error, Result, build_contracts::{CanRunProtobufBuildTask, SchemaBuildPaths},
    lifecycle::HasBuildProjectRoot, stages::{BuildArtifact, BuildRequest, CanRunNativeBuildStage, NativeBuildOperation}};

#[derive(Clone, Copy, Debug)]
pub enum SourceBuildTask {
    GenerateDomain, CachedProtocol, Protocol, Bindings, Compile, SchemaJson,
    Documentation, PackageSources, StageSchemaTool, Format, FormatCheck, Version,
}

impl SourceBuildTask {
    pub fn parse(name: &str) -> Result<Self> {
        match name {
            "schema/generateDomainClasses" | "schema/Compile/generateDomainClasses" | "//schema:codegen" => Ok(Self::GenerateDomain),
            "schema/generateProtobuf" => Ok(Self::CachedProtocol),
            "//schema:protogen-bin" | "//schema:run_protogen" | "//schema:proto" => Ok(Self::Protocol),
            "//schema:java_proto" => Ok(Self::Bindings),
            "codepropertygraph/compile" | "schema/compile" | "domainClasses/compile"
            | "//schema" | "//schema:schema" | "//codepropertygraph" | "//codepropertygraph:codepropertygraph"
            | "//schema:domainClassesDontUseDirectly" | "//schema:domainClassesNeverLinked" | "//schema:domainClassesLinked" => Ok(Self::Compile),
            "//schema2json" | "//schema2json:schema2json" => Ok(Self::SchemaJson),
            "packageDoc" => Ok(Self::Documentation),
            "packageSrc" | "sonatypePublishToBundle" => Ok(Self::PackageSources),
            "schema2json/stage" => Ok(Self::StageSchemaTool),
            "scalafmtAll" | "//:format" | "//bazel/format" | "//bazel/format:format" | "//bazel/format:scalafmt" => Ok(Self::Format),
            "scalafmtCheckAll" | "//:formatCheck" | "//bazel/format:format.check" => Ok(Self::FormatCheck),
            "version" => Ok(Self::Version),
            _ => Err(Error::IllegalArgument(format!("source task {name} has no native production adapter; consult its exact milestone owner"))),
        }
    }

    pub fn uses_sbt_load(name: &str) -> bool { !name.starts_with("//") }
}

#[cgp_component(SourceBuildTaskRunner)]
pub trait CanRunSourceBuildTask {
    /// The lifecycle entrypoint owns sbt onLoad and its previous-hook state.
    fn run_source_build_task(&self, name: &str, request: &BuildRequest) -> Result<BuildArtifact>;
}

#[cgp_new_provider]
impl<Context> SourceBuildTaskRunner<Context> for RunSourceBuildTasks
where Context: HasBuildProjectRoot + CanRunProtobufBuildTask + CanRunNativeBuildStage,
{
    fn run_source_build_task(context: &Context, name: &str, request: &BuildRequest) -> Result<BuildArtifact> {
        let task = SourceBuildTask::parse(name)?;
        if matches!(task, SourceBuildTask::CachedProtocol) {
            let mut paths = SchemaBuildPaths::at(context.build_project_root());
            if let Some(source) = &request.schema_source_directory {
                paths.source_directory = context.build_project_root().join(source);
            }
            if let Some(target) = &request.schema_target {
                paths.target = context.build_project_root().join(target);
            }
            let output = context.run_protobuf_build_task(&paths)?;
            return Ok(BuildArtifact { source_operation: name.to_owned(), paths: vec![output], inputs: Default::default(),
                compilation: "UNVERIFIED".into(), behavior: "UNVERIFIED".into() });
        }
        let operation = match task {
            SourceBuildTask::GenerateDomain => NativeBuildOperation::Domain,
            SourceBuildTask::Protocol => NativeBuildOperation::Protocol,
            SourceBuildTask::Bindings => NativeBuildOperation::Bindings,
            SourceBuildTask::Compile => NativeBuildOperation::Compile,
            SourceBuildTask::SchemaJson => NativeBuildOperation::SchemaJson,
            SourceBuildTask::Documentation => NativeBuildOperation::Documentation,
            SourceBuildTask::PackageSources => NativeBuildOperation::Package,
            SourceBuildTask::StageSchemaTool => NativeBuildOperation::Stage,
            SourceBuildTask::Format => NativeBuildOperation::FormatSnapshot,
            SourceBuildTask::FormatCheck => NativeBuildOperation::FormatCheck,
            SourceBuildTask::Version => NativeBuildOperation::SourceVersion,
            SourceBuildTask::CachedProtocol => unreachable!("cached protocol returns above"),
        };
        let mut artifact = context.run_native_build_stage(operation, request)?;
        artifact.source_operation = name.to_owned();
        // The Scala/JVM domain libraries are merged into one statically owned
        // Rust crate. Both imported-artifact views refer to that same owner;
        // neither constructs another graph or copies a runtime implementation.
        if name.ends_with("domainClassesNeverLinked") {
            artifact.inputs.insert("sourceImportPolicy".into(), "neverlink; canonical native crate provides referenced types".into());
        }
        if name.ends_with("domainClassesLinked") {
            artifact.inputs.insert("sourceImportPolicy".into(), "linked; canonical native crate provides runtime implementation".into());
        }
        artifact.inputs.insert("nativeDependencyManifest".into(),
            request.native_workspace.join(PathBuf::from("cpg-rs/Cargo.toml")).display().to_string());
        Ok(artifact)
    }
}
