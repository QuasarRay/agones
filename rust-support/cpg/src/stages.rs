//! Concrete native consumers of the pinned build facilities.
//! CGP selects schema construction, generation and command effects. Outputs
//! retain their source-stage identities; a build artifact is not assurance.
use std::{fs, path::{Path, PathBuf}, process::Command};
use cgp::prelude::*;
use serde::{Deserialize, Serialize};
use cpg_rs::{schema_tools::{CanBuildCpgSchema, CanGenerateCpgProto, SchemaVariant},
    protobuf_bindings::CanGenerateCpgBindings};
use crate::{Error, Result, packaging::{self, PackageManifest}, source_io::HasBuildIo};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeFacility {
    ProtocolBindings, SourceFormatting, PublicationBundle, VersionIdentity,
    ApplicationStage, DomainGeneration,
}
impl NativeFacility {
    /// These are the fixed facilities used by the six retained sbt declarations.
    /// Choosing a build command never installs or replaces a runtime provider.
    pub fn parse(name: &str) -> Result<Self> {
        match name {
            "sbt-protobuf" => Ok(Self::ProtocolBindings),
            "sbt-scalafmt" => Ok(Self::SourceFormatting),
            "sbt-ci-release-early" => Ok(Self::PublicationBundle),
            "sbt-dynver" => Ok(Self::VersionIdentity),
            "sbt-native-packager" => Ok(Self::ApplicationStage),
            "sbt-flatgraph" => Ok(Self::DomainGeneration),
            _ => Err(Error::IllegalArgument(format!("unknown retained build facility {name}"))),
        }
    }

    pub fn operation(self) -> NativeBuildOperation {
        match self {
            Self::ProtocolBindings => NativeBuildOperation::Bindings,
            Self::SourceFormatting => NativeBuildOperation::FormatSnapshot,
            Self::PublicationBundle => NativeBuildOperation::Package,
            Self::VersionIdentity => NativeBuildOperation::SourceVersion,
            Self::ApplicationStage => NativeBuildOperation::Stage,
            Self::DomainGeneration => NativeBuildOperation::Domain,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NativeBuildOperation {
    Domain, Protocol, Bindings, Compile, Documentation, Package, Stage, SchemaJson,
    FormatSnapshot, FormatCheck, SourceVersion,
}
impl NativeBuildOperation {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "domain" => Ok(Self::Domain), "protocol" => Ok(Self::Protocol),
            "bindings" => Ok(Self::Bindings), "compile" => Ok(Self::Compile),
            "documentation" => Ok(Self::Documentation), "package" => Ok(Self::Package),
            "stage" => Ok(Self::Stage), "format" => Ok(Self::FormatSnapshot),
            "format-check" => Ok(Self::FormatCheck), "source-version" => Ok(Self::SourceVersion),
            "schema-json" => Ok(Self::SchemaJson),
            _ => Err(Error::IllegalArgument(format!("unknown native build operation {value}"))),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildRequest {
    pub native_workspace: PathBuf,
    pub output_root: PathBuf,
    pub domain_output: PathBuf,
    pub bindings_output: PathBuf,
    pub protocol_output: PathBuf,
    pub package_manifest: Option<PathBuf>,
    pub staged_binary: Option<PathBuf>,
    pub staged_binary_name: Option<String>,
    pub staged_resources: Vec<(PathBuf, PathBuf)>,
    pub source_revision: String,
    pub source_version: String,
    pub native_revision: String,
    #[serde(default)]
    pub schema_source_directory: Option<PathBuf>,
    #[serde(default)]
    pub schema_target: Option<PathBuf>,
    #[serde(default)]
    pub schema_json_output: Option<PathBuf>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildArtifact {
    pub source_operation: String,
    pub paths: Vec<PathBuf>,
    pub inputs: std::collections::BTreeMap<String, String>,
    pub compilation: String,
    pub behavior: String,
}
impl BuildArtifact {
    fn produced(operation: &str, paths: Vec<PathBuf>) -> Self {
        Self { source_operation: operation.to_owned(), paths, inputs: Default::default(),
            compilation: "UNVERIFIED".into(), behavior: "UNVERIFIED".into() }
    }
}

#[cgp_component(NativeBuildStageRunner)]
pub trait CanRunNativeBuildStage {
    fn run_native_build_stage(&self, operation: NativeBuildOperation, request: &BuildRequest) -> Result<BuildArtifact>;
}

fn core_manifest(request: &BuildRequest) -> PathBuf { request.native_workspace.join("cpg-rs/Cargo.toml") }
fn support_manifest(request: &BuildRequest) -> PathBuf { request.native_workspace.join("rust-support/cpg/Cargo.toml") }
fn package_manifest(request: &BuildRequest) -> Result<&Path> {
    request.package_manifest.as_deref().ok_or_else(|| Error::IllegalArgument("this operation needs a complete payload manifest".into()))
}

fn record_input(path: &Path, bytes: &[u8]) -> Result<()> {
    if path.exists() {
        if fs::read(path)? != bytes { return Err(Error::IllegalArgument(format!("preserve changed build input {}", path.display()))); }
    } else {
        if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
        let mut file = fs::OpenOptions::new().create_new(true).write(true).open(path)?;
        std::io::Write::write_all(&mut file, bytes)?;
    }
    Ok(())
}

#[cgp_new_provider]
impl<Context> NativeBuildStageRunner<Context> for RunNativeBuildStages
where Context: HasBuildIo + CanBuildCpgSchema + CanGenerateCpgProto + CanGenerateCpgBindings,
    Context: cpg_rs::schema_tools::CanExportCpgSchemaJson,
{
    fn run_native_build_stage(context: &Context, operation: NativeBuildOperation, request: &BuildRequest) -> Result<BuildArtifact> {
        match operation {
            NativeBuildOperation::Domain => {
                let schema = context.build_cpg_schema(SchemaVariant::Domain)?;
                let input = serde_json::to_vec(&schema)?;
                let identity = cpg_rs::content_digest(&input);
                let input_path = request.output_root.join(format!("schema-input-{identity}.json"));
                record_input(&input_path, &input)?;
                let report = cpg_rs::codegen::generate(&schema, &input_path, &request.domain_output)?;
                let mut artifact = BuildArtifact::produced("generateDomainClasses", vec![
                    request.domain_output.join("domain.rs"), request.domain_output.join("generation.json"),
                ]);
                artifact.inputs.insert("nativeCompleteSchemaModel".into(), identity);
                artifact.inputs.insert("gencoProducer".into(), report.generator_sha256);
                Ok(artifact)
            }
            NativeBuildOperation::Protocol => {
                let output = context.generate_cpg_proto(&request.protocol_output)?;
                Ok(BuildArtifact::produced("Protogen", vec![output]))
            }
            NativeBuildOperation::Bindings => {
                let report = context.generate_cpg_bindings(&request.bindings_output)?;
                let mut artifact = BuildArtifact::produced("protobufGenerate", report.files.keys()
                    .map(|path| request.bindings_output.join(path)).collect());
                artifact.inputs = report.inputs;
                Ok(artifact)
            }
            NativeBuildOperation::Compile => {
                // Produce the owned runtime inputs before compiling their
                // existing consumer crate. No alternate handwritten owner is
                // copied into an allegedly generated file.
                let consumer = output_location(&request.native_workspace.join("cpg-rs/generated"))?;
                if output_location(&request.domain_output)? != consumer || output_location(&request.bindings_output)? != consumer {
                    return Err(Error::IllegalArgument("compile outputs must match the current cpg-rs source registration; stage-only outputs cannot satisfy it".into()));
                }
                Self::run_native_build_stage(context, NativeBuildOperation::Domain, request)?;
                Self::run_native_build_stage(context, NativeBuildOperation::Bindings, request)?;
                let mut command = Command::new("cargo");
                command.args(["build", "--lib", "--locked", "--message-format=json-render-diagnostics", "--manifest-path"])
                    .arg(core_manifest(request));
                let (status, output) = crate::process::stdout(&mut command, "cargo")?;
                let mut files = Vec::new();
                for line in output.split(|byte| *byte == b'\n').filter(|line| !line.is_empty()) {
                    let message: serde_json::Value = serde_json::from_slice(line)?;
                    if message["reason"] == "compiler-message" {
                        if let Some(rendered) = message["message"]["rendered"].as_str() { eprint!("{rendered}"); }
                    }
                    if message["reason"] == "compiler-artifact" && message["target"]["name"] == "cpg_rs" {
                        if let Some(names) = message["filenames"].as_array() {
                            files.extend(names.iter().filter_map(|name| name.as_str().map(PathBuf::from)));
                        }
                    }
                }
                if !status.success() { return Err(Error::Runtime(format!("native CPG library compilation failed: {status}"))); }
                if files.is_empty() { return Err(Error::Runtime("Cargo produced no CPG library artifact".into())); }
                let mut artifact = BuildArtifact::produced("Compile/compile", files);
                artifact.compilation = "SELECTED_CARGO_COMMAND_SUCCEEDED".into();
                Ok(artifact)
            }
            NativeBuildOperation::Documentation => {
                // Rustdoc reads the same registered generated modules as the
                // library compiler. Keep their producer prerequisites here.
                let consumer = output_location(&request.native_workspace.join("cpg-rs/generated"))?;
                if output_location(&request.domain_output)? != consumer || output_location(&request.bindings_output)? != consumer {
                    return Err(Error::IllegalArgument("documentation outputs must match the current cpg-rs source registration".into()));
                }
                Self::run_native_build_stage(context, NativeBuildOperation::Domain, request)?;
                Self::run_native_build_stage(context, NativeBuildOperation::Bindings, request)?;
                let target = request.output_root.join("cargo-target");
                let mut command = Command::new("cargo");
                command.args(["doc", "--locked", "--no-deps", "--manifest-path"]).arg(core_manifest(request))
                    .arg("--target-dir").arg(&target);
                run_command(command)?;
                Ok(BuildArtifact::produced("packageDoc", vec![target.join("doc/cpg_rs")]))
            }
            NativeBuildOperation::Package => {
                let path = request.output_root.join("cpg-native-source.zip");
                let report = packaging::prepare_publication_bundle(context, package_manifest(request)?, &path)?;
                let mut artifact = BuildArtifact::produced("publishTo/sonatypePublishToBundle + packageSrc", vec![path]);
                artifact.inputs.insert("payloadManifest".into(), report.input_manifest_sha256);
                artifact.inputs.insert("packageArchive".into(), report.archive_sha256);
                Ok(artifact)
            }
            NativeBuildOperation::Stage => {
                Self::run_native_build_stage(context, NativeBuildOperation::Domain, request)?;
                let binary = request.staged_binary.as_deref().ok_or_else(|| Error::IllegalArgument("stage requires a compiled native binary".into()))?;
                let name = request.staged_binary_name.as_deref().ok_or_else(|| Error::IllegalArgument("stage requires its source executable name".into()))?;
                let output = packaging::stage_binary(binary, &request.output_root.join("stage"), name, &request.staged_resources)?;
                Ok(BuildArtifact::produced("JavaAppPackaging/stage", vec![output]))
            }
            NativeBuildOperation::FormatSnapshot => {
                let manifest: PackageManifest = serde_json::from_slice(&fs::read(package_manifest(request)?)?)?;
                let snapshot = request.output_root.join("formatted-source");
                packaging::copy_source_snapshot(context, &manifest, &snapshot)?;
                for relative in ["cpg-rs/Cargo.toml", "rust-support/cpg/Cargo.toml"] {
                    let mut command = Command::new("cargo");
                    command.args(["fmt", "--all", "--manifest-path"]).arg(snapshot.join(relative));
                    run_command(command)?;
                }
                Ok(BuildArtifact::produced("scalafmt / format_multirun", vec![snapshot]))
            }
            NativeBuildOperation::FormatCheck => {
                for manifest in [core_manifest(request), support_manifest(request)] {
                    let mut command = Command::new("cargo");
                    command.args(["fmt", "--all", "--manifest-path"]).arg(manifest).args(["--", "--check"]);
                    run_command(command)?;
                }
                Ok(BuildArtifact::produced("scalafmtCheck / format.check", Vec::new()))
            }
            NativeBuildOperation::SourceVersion => {
                // This is the retained source release, not the surrounding
                // Agones checkout's Git version or Cargo's native crate version.
                if request.source_revision != crate::build_contracts::SOURCE_REVISION {
                    return Err(Error::IllegalArgument("build request selects a different CPG source pin".into()));
                }
                if request.source_version != crate::build_contracts::SOURCE_RELEASE_VERSION {
                    return Err(Error::IllegalArgument("build request changes the pinned source release identity".into()));
                }
                let output = request.output_root.join("source-and-native-version.json");
                let data = serde_json::to_vec_pretty(&serde_json::json!({
                    "sourceRevision": request.source_revision, "sourceVersion": request.source_version,
                    "nativeRevision": request.native_revision, "nativeVersion": env!("CARGO_PKG_VERSION"),
                }))?;
                record_input(&output, &data)?;
                Ok(BuildArtifact::produced("sbt-dynver/source release identity", vec![output]))
            }
            NativeBuildOperation::SchemaJson => {
                use cpg_rs::schema_tools::CanExportCpgSchemaJson;
                let output = request.schema_json_output.clone().unwrap_or_else(|| PathBuf::from("/tmp/schema.json"));
                context.export_cpg_schema_json(&output)?;
                Ok(BuildArtifact::produced("Schema2Json", vec![output]))
            }
        }
    }
}

fn output_location(path: &Path) -> Result<PathBuf> {
    if path.exists() { return Ok(path.canonicalize()?); }
    let parent = path.parent().ok_or_else(|| Error::IllegalArgument("output has no parent".into()))?;
    let name = path.file_name().ok_or_else(|| Error::IllegalArgument("output has no filename".into()))?;
    Ok(parent.canonicalize()?.join(name))
}

fn run_command(mut command: Command) -> Result<()> {
    let status = crate::process::status(&mut command, "cargo")?;
    if !status.success() { return Err(Error::Runtime(format!("native build child failed: {status}"))); }
    Ok(())
}
