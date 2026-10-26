//! Native consumers for the pinned sbt/Bazel build declarations.
//! Original build files remain intact in cpg-rs; these types own Rust support.
use std::{ffi::OsString, path::{Path, PathBuf}, process::Command};
use cgp::prelude::*;
use comprende::c;
use better_comprehension::vector;
use crate::{FileUtils, Projects, Utils::GitLFSUtils, Versions, ProjectVersions, Error, Result};
use crate::source_io::HasBuildIo;
use crate::lifecycle::HasBuildProjectRoot;

pub const SOURCE_REVISION: &str = "301c4010d28ecdac05bc2ed44310f56446459465";
pub const SOURCE_SCALA_VERSION: &str = "3.9.0";
pub const SOURCE_JAVA_RELEASE: &str = "17";
pub const SOURCE_SBT_VERSION: &str = "1.12.5";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeTargetKind { Library, Binary, Harness, ImportedArtifact }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetAction { Build, Run }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleExport {
    pub source_name: &'static str,
    pub native_kind: NativeTargetKind,
}

#[derive(Clone, Debug)]
pub struct RuleFactory {
    pub source_language: &'static str,
    pub source_version: &'static str,
    pub source_compile_options: &'static [&'static str],
    pub mixed_java_options: &'static [&'static str],
    pub binary_runtime_dependencies: Vec<String>,
    pub test_runtime_dependencies: Vec<String>,
    pub exports: Vec<RuleExport>,
    pub native_manifest: PathBuf,
    pub native_debug_information: bool,
}

impl RuleFactory {
    pub fn target(&self, source_name: &str, name: &str, native_target: &str, dependencies: &[&str]) -> Result<NativeTarget> {
        let export = self.exports.iter().find(|export| export.source_name == source_name)
            .ok_or_else(|| Error::IllegalArgument(format!("unknown source rule {source_name}")))?;
        let runtime_dependencies = match export.native_kind {
            NativeTargetKind::Binary => self.binary_runtime_dependencies.clone(),
            NativeTargetKind::Harness => self.test_runtime_dependencies.clone(),
            _ => Vec::new(),
        };
        Ok(NativeTarget {
            name: name.to_owned(),
            native_target: native_target.to_owned(),
            kind: export.native_kind,
            dependencies: vector![dependency.to_string() for dependency in dependencies],
            runtime_dependencies,
            manifest: self.native_manifest.clone(),
            debug_information: self.native_debug_information,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NativeTarget {
    pub name: String,
    pub native_target: String,
    pub kind: NativeTargetKind,
    pub dependencies: Vec<String>,
    pub runtime_dependencies: Vec<String>,
    pub manifest: PathBuf,
    pub debug_information: bool,
}

impl NativeTarget {
    pub fn cargo_arguments(&self, action: TargetAction) -> Result<Vec<OsString>> {
        let verb = match (self.kind, action) {
            (_, TargetAction::Build) => "build",
            (NativeTargetKind::Binary, TargetAction::Run) => "run",
            (NativeTargetKind::Harness, TargetAction::Run) => "test",
            _ => return Err(Error::IllegalArgument("only a binary or harness can run".into())),
        };
        let mut arguments = vec![OsString::from(verb), "--manifest-path".into(), self.manifest.as_os_str().to_owned(), "--locked".into()];
        match self.kind {
            NativeTargetKind::Library | NativeTargetKind::ImportedArtifact => {
                arguments.extend(["--package".into(), self.native_target.clone().into(), "--lib".into()]);
            }
            NativeTargetKind::Binary => arguments.extend(["--bin".into(), self.native_target.clone().into()]),
            NativeTargetKind::Harness => {
                // Cargo's test command owns harness compilation and execution.
                arguments[0] = "test".into();
                arguments.extend(["--test".into(), self.native_target.clone().into()]);
                if action == TargetAction::Build { arguments.push("--no-run".into()); }
            }
        }
        Ok(arguments)
    }

    pub fn execute<Context: CanExecuteNativeTarget>(&self, context: &Context, action: TargetAction) -> Result<()> {
        context.execute_native_target(self, action)
    }
}

#[cgp_component(NativeTargetExecutor)]
pub trait CanExecuteNativeTarget {
    fn execute_native_target(&self, target: &NativeTarget, action: TargetAction) -> Result<()>;
}

#[cgp_new_provider]
impl<Context> NativeTargetExecutor<Context> for ExecuteTargetWithCargo {
    fn execute_native_target(_context: &Context, target: &NativeTarget, action: TargetAction) -> Result<()> {
        let status = Command::new("cargo")
            .args(target.cargo_arguments(action)?)
            .env("CARGO_PROFILE_DEV_DEBUG", if target.debug_information { "2" } else { "0" })
            .status()?;
        if !status.success() {
            return Err(Error::Runtime(format!("native target {} failed: {status}", target.name)));
        }
        Ok(())
    }
}

pub fn java_rules(manifest: &Path) -> RuleFactory {
    RuleFactory {
        source_language: "java",
        source_version: SOURCE_JAVA_RELEASE,
        source_compile_options: &["-g", "--release", "17", "-Xlint:all", "-Xlint:-cast", "-XepDisableAllChecks"],
        mixed_java_options: &[],
        binary_runtime_dependencies: Vec::new(),
        test_runtime_dependencies: Vec::new(),
        exports: vec![
            RuleExport { source_name: "java_library", native_kind: NativeTargetKind::Library },
            RuleExport { source_name: "java_binary", native_kind: NativeTargetKind::Binary },
            RuleExport { source_name: "java_test", native_kind: NativeTargetKind::Harness },
            RuleExport { source_name: "java_import", native_kind: NativeTargetKind::ImportedArtifact },
        ],
        native_manifest: manifest.to_owned(),
        native_debug_information: true,
    }
}

pub fn scala_rules(manifest: &Path) -> RuleFactory {
    RuleFactory {
        source_language: "scala",
        source_version: SOURCE_SCALA_VERSION,
        source_compile_options: &[
            "-java-output-version", "17", "-no-indent", "-old-syntax",
            "-Wshadow:type-parameter-shadow", "-deprecation",
            "-Wconf:msg=Implicit parameters should be provided with a `using` clause:s",
        ],
        mixed_java_options: &["--release", "17", "-XepDisableAllChecks"],
        binary_runtime_dependencies: Vec::new(),
        test_runtime_dependencies: Vec::new(),
        exports: vec![
            RuleExport { source_name: "scala_library", native_kind: NativeTargetKind::Library },
            RuleExport { source_name: "scala_binary", native_kind: NativeTargetKind::Binary },
            RuleExport { source_name: "scala_test", native_kind: NativeTargetKind::Harness },
        ],
        native_manifest: manifest.to_owned(),
        native_debug_information: true,
    }
}

#[derive(Clone, Debug)]
pub struct BuildPlugin {
    pub organization: &'static str,
    pub name: &'static str,
    pub version: String,
    pub native_owner: &'static str,
}

#[derive(Clone, Debug)]
pub struct SourceDependency {
    pub organization: &'static str,
    pub artifact: &'static str,
    pub version: String,
    pub scala_cross_version: bool,
    pub test_only: bool,
    pub native_owner: &'static str,
}

pub fn codepropertygraph_dependencies(flatgraph: &str) -> Vec<SourceDependency> {
    [
        ("io.joern", "flatgraph-formats", flatgraph, true, false, "cpg_rs::graph and proto_loading"),
        ("io.joern", "flatgraph-help", flatgraph, true, false, "native CPG facade/help support"),
        ("io.joern", "flatgraph-odb-convert", flatgraph, true, false, "cpg_rs::graph::legacy"),
        ("com.github.scopt", "scopt", "4.0.1", true, false, "native source-tool argument parsing"),
        ("com.github.pathikrit", "better-files", "3.9.2", true, false, "native filesystem/archive effects"),
        ("org.slf4j", "slf4j-api", "2.0.6", false, false, "native tracing effect boundary"),
        ("org.scalatest", "scalatest", Versions::scalatest, true, true, "M0036 source harnesses after global gate"),
    ].into_iter().map(|(organization, artifact, version, scala_cross_version, test_only, native_owner)| {
        SourceDependency { organization, artifact, version: version.to_owned(), scala_cross_version, test_only, native_owner }
    }).collect()
}

#[derive(Clone, Debug)]
pub struct SchemaGeneratorSettings {
    pub class_with_schema: &'static str,
    pub field_name: &'static str,
    pub output_project: Projects::Project,
    pub output_setting: &'static str,
    pub scala3_generation: bool,
    pub scala3_check: bool,
    pub native_generation_owner: &'static str,
}

pub fn plugins(flatgraph: &str) -> Vec<BuildPlugin> {
    [
        ("com.github.sbt", "sbt-protobuf", "0.8.2", "cpg_rs::protobuf_bindings"),
        ("org.scalameta", "sbt-scalafmt", "2.4.6", "post-gate formatter tooling"),
        ("io.shiftleft", "sbt-ci-release-early", "2.1.2", "authorized delivery and packaging"),
        ("com.github.sbt", "sbt-dynver", "5.1.0", "source version and VCS identity"),
        ("com.github.sbt", "sbt-native-packager", "1.10.4", "native Cargo package/stage artifacts"),
        ("io.joern", "sbt-flatgraph", flatgraph, "cpg_rs::construction and codegen"),
    ].into_iter().map(|(organization, name, version, native_owner)| BuildPlugin {
        organization, name, version: version.to_owned(), native_owner,
    }).collect()
}

#[derive(Clone, Debug)]
pub struct BuildConfiguration {
    pub project_names: Vec<&'static str>,
    pub flatgraph: String,
    pub nested_flatgraph: String,
    pub organization: &'static str,
    pub scala_version: &'static str,
    pub root_publish_skip: bool,
    pub source_and_documentation_artifacts: bool,
    pub reload_on_source_changes: bool,
    pub source_test_fork: bool,
    pub source_test_java_options: Vec<String>,
    pub source_scalac_options: &'static [&'static str],
    pub source_javac_options: &'static [&'static str],
    pub credential_host: &'static str,
    pub sonatype_timeout_millis: u64,
    pub public_repository: &'static str,
    pub public_scm: &'static str,
    pub homepage: &'static str,
    pub license: (&'static str, &'static str),
    pub resolvers: &'static [&'static str],
    pub developers: &'static [(&'static str, &'static str, &'static str, &'static str)],
    pub optional_logging_dependencies: &'static [(&'static str, &'static str, &'static str)],
    pub plugins: Vec<BuildPlugin>,
    pub codepropertygraph_projects: &'static [Projects::Project],
    pub codepropertygraph_dependencies: Vec<SourceDependency>,
    pub schema_dependency: SourceDependency,
    pub schema_generation: SchemaGeneratorSettings,
    pub java: RuleFactory,
    pub scala: RuleFactory,
}

#[cgp_component(BuildConfigurationLoader)]
pub trait CanLoadBuildConfiguration {
    fn load_build_configuration(&self) -> Result<BuildConfiguration>;
}

#[cgp_new_provider]
impl<Context> BuildConfigurationLoader<Context> for LoadNativeBuildConfiguration
where Context: HasBuildIo + HasBuildProjectRoot,
{
    fn load_build_configuration(context: &Context) -> Result<BuildConfiguration> {
        let root = context.build_project_root();
        let flatgraph = Versions::flatgraph(context, root)?;
        let nested_flatgraph = ProjectVersions::flatgraph(context, root)?;
        // This is the root build's actual onLoad predicate. Spawn errors remain
        // errors; a nonzero command result triggers the source assertion.
        if !GitLFSUtils::isGitLFSEnabled(root)? {
            return Err(Error::SourceAssertion(
                "assertion failed: You need to install git-lfs and run 'git lfs pull'".into()
            ));
        }
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../cpg-rs/Cargo.toml");
        let configured = BuildConfiguration {
            project_names: c![project.source_name for project in Projects::ALL],
            plugins: plugins(&flatgraph),
            codepropertygraph_projects: &[Projects::protoBindings, Projects::domainClasses],
            codepropertygraph_dependencies: codepropertygraph_dependencies(&flatgraph),
            schema_dependency: SourceDependency {
                organization: "io.joern", artifact: "flatgraph-domain-classes-generator",
                version: flatgraph.clone(), scala_cross_version: true, test_only: false,
                native_owner: "cpg_rs::codegen and construction",
            },
            schema_generation: SchemaGeneratorSettings {
                class_with_schema: SCHEMA_CLASS, field_name: SCHEMA_FIELD,
                output_project: Projects::domainClasses, output_setting: "generatedSrcDir",
                scala3_generation: true, scala3_check: true,
                native_generation_owner: "cpg_rs::codegen::CpgToolContext (M0010 output mapping)",
            },
            flatgraph, nested_flatgraph,
            organization: "io.shiftleft",
            scala_version: SOURCE_SCALA_VERSION,
            root_publish_skip: true,
            source_and_documentation_artifacts: true,
            reload_on_source_changes: true,
            source_test_fork: true,
            source_test_java_options: vec![
                format!("-Dlog4j2.configurationFile=file:{}/resources/log4j2-test.xml", root.display()),
                format!("-Duser.dir={}", root.display()),
            ],
            source_scalac_options: &[
                "-release", "17", "-deprecation", "-feature",
                "-Wconf:msg=Implicit parameters should be provided with a `using` clause:s",
                "-language:implicitConversions", "-no-indent", "-old-syntax",
            ],
            source_javac_options: &["-g", "--release", "17"],
            credential_host: "central.sonatype.com",
            sonatype_timeout_millis: 7_200_000,
            public_repository: "https://github.com/ShiftLeftSecurity/codepropertygraph",
            public_scm: "scm:git@github.com:ShiftLeftSecurity/codepropertygraph.git",
            homepage: "https://github.com/ShiftLeftSecurity/codepropertygraph/",
            license: ("Apache-2.0", "http://www.apache.org/licenses/LICENSE-2.0"),
            resolvers: &["mavenLocal", "https://oss.sonatype.org/content/repositories/public"],
            developers: &[
                ("fabsx00", "Fabian Yamaguchi", "fabs@shiftleft.io", "https://github.com/fabsx00"),
                ("ml86", "Markus Lottmann", "markus@shiftleft.io", "https://github.com/ml86"),
                ("mpollmeier", "Michael Pollmeier", "michael@michaelpollmeier.com", "http://www.michaelpollmeier.com/"),
            ],
            optional_logging_dependencies: &[
                ("org.apache.logging.log4j", "log4j-slf4j2-impl", "2.19.0"),
                ("org.apache.logging.log4j", "log4j-core", "2.19.0"),
            ],
            java: java_rules(&manifest),
            scala: scala_rules(&manifest),
        };
        Ok(configured)
    }
}

#[derive(Clone, Debug)]
pub struct SchemaBuildPaths {
    pub project_root: PathBuf,
    pub source_directory: PathBuf,
    pub target: PathBuf,
}

impl SchemaBuildPaths {
    pub fn at(project_root: &Path) -> Self {
        Self {
            project_root: project_root.to_owned(),
            source_directory: project_root.join("schema/src"),
            target: project_root.join("schema/target"),
        }
    }
}

pub const SCHEMA_CLASS: &str = "io.shiftleft.codepropertygraph.schema.CpgSchema$";
pub const SCHEMA_FIELD: &str = "instance";

pub fn last_schema_md5<Context: HasBuildIo>(context: &Context, root: &Path) -> Option<String> {
    // scala.util.Try(IO.read(...)).toOption suppresses every read failure.
    context.build_io().read_source_text(&root.join("target/schema-src.md5")).ok()
}

pub fn write_last_schema_md5<Context: HasBuildIo>(context: &Context, root: &Path, value: &str) -> Result<()> {
    context.build_io().write_source_text(&root.join("target/schema-src.md5"), value)
}

#[cgp_component(ProtobufBuildTask)]
pub trait CanRunProtobufBuildTask {
    fn run_protobuf_build_task(&self, paths: &SchemaBuildPaths) -> Result<PathBuf>;
}

#[cgp_new_provider]
impl<Context: HasBuildIo> ProtobufBuildTask<Context> for RunNativeProtobufBuildTask {
    fn run_protobuf_build_task(context: &Context, paths: &SchemaBuildPaths) -> Result<PathBuf> {
        use cpg_rs::schema_tools::CanGenerateCpgProto;
        let output_root = paths.target.join("protos");
        let output_file = output_root.join("cpg.proto");
        let digest = FileUtils::md5(context, &[
            paths.source_directory.clone(), paths.project_root.join("schema/build.sbt"),
        ])?;
        // The source checks the output directory, not whether cpg.proto exists.
        // Preserve that condition rather than silently repairing its cache rule.
        if output_root.exists() && last_schema_md5(context, &paths.project_root).as_deref() == Some(digest.as_str()) {
            return Ok(output_file);
        }
        FileUtils::deleteRecursively(context, &output_root)?;
        // The runMain argument is hard-coded in the source even if target is
        // overridden. Keep its path distinct from the returned output path.
        cpg_rs::schema_tools::NativeCpgSchemaContext
            .generate_cpg_proto(&paths.project_root.join("schema/target/protos"))?;
        write_last_schema_md5(context, &paths.project_root, &digest)?;
        Ok(output_file)
    }
}
