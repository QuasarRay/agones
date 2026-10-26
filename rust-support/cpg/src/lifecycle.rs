//! The synchronous build phase owns every walk and reader it creates.
//! No child process is detached: GitLFSUtils waits for its child. Generator
//! effects return before this context is dropped; failures retain their error.
use std::path::{Path, PathBuf};
use cgp::prelude::*;
use derive_deftly::Deftly;
use crate::{declarations::derive_deftly_template_NativeBuildProviders, source_io::{BuildIo, HasBuildIo}, singletons::{BuildSingletons, DefinitionReload}};
use crate::{Result, build_contracts::{BuildConfiguration, CanLoadBuildConfiguration}};
use crate::{source_tasks::{CanRunSourceBuildTask, SourceBuildTask}, stages::{BuildRequest, BuildArtifact}};

#[cgp_getter]
pub trait HasBuildProjectRoot {
    fn build_project_root(&self) -> &PathBuf;
}

#[cgp_new_provider]
impl<Context: HasBuildIo> BuildProjectRootGetter<Context> for ReadBuildIoProjectRoot {
    fn build_project_root(context: &Context) -> &PathBuf { context.build_io().project_root() }
}

#[derive(HasField, Deftly)]
#[derive_deftly(NativeBuildProviders)]
pub struct NativeBuildContext {
    pub build_io: BuildIo,
    pub build_singletons: BuildSingletons,
}

impl NativeBuildContext {
    pub fn new(project_root: &Path) -> Result<Self> {
        let root = std::path::absolute(project_root)?;
        Ok(Self { build_io: BuildIo::at(&root), build_singletons: BuildSingletons::default() })
    }

    /// Reload uses fresh defining classes and resource ownership. Existing
    /// contexts retain their results until their current build phase ends.
    pub fn reloaded(&self, scope: DefinitionReload) -> Self {
        Self { build_io: self.build_io.fresh_generation(), build_singletons: self.build_singletons.for_reload(scope) }
    }

    /// The source constructs its onLoad hook only after the Git-LFS predicate
    /// succeeds, then delegates to the previously installed hook. Keep its
    /// state generic and invoke it once, without hiding either failure.
    pub fn load_with_previous<State>(&self, state: State, previous: impl FnOnce(State) -> Result<State>) -> Result<(BuildConfiguration, State)> {
        let configuration = self.load_build_configuration()?;
        let state = previous(state)?;
        Ok((configuration, state))
    }

    pub fn run_source_task_with_previous<State>(&self, name: &str, request: &BuildRequest,
        state: State, previous: impl FnOnce(State) -> Result<State>) -> Result<(BuildArtifact, State)> {
        // Reject an unknown task before starting a build. sbt task consumers
        // then enter the same ordered onLoad lifecycle; Bazel has no sbt hook.
        SourceBuildTask::parse(name)?;
        let state = if SourceBuildTask::uses_sbt_load(name) {
            self.load_with_previous(state, previous)?.1
        } else { state };
        Ok((self.run_source_build_task(name, request)?, state))
    }
}
