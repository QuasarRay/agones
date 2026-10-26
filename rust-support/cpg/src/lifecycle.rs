//! The synchronous build phase owns every walk and reader it creates.
//! No child process is detached: GitLFSUtils waits for its child. Generator
//! effects return before this context is dropped; failures retain their error.
use std::path::{Path, PathBuf};
use cgp::prelude::*;
use derive_deftly::Deftly;
use crate::{declarations::derive_deftly_template_NativeBuildProviders, source_io::BuildIo};

#[cgp_getter]
pub trait HasBuildProjectRoot {
    fn build_project_root(&self) -> &PathBuf;
}

#[derive(HasField, Deftly)]
#[derive_deftly(NativeBuildProviders)]
pub struct NativeBuildContext {
    pub build_project_root: PathBuf,
    pub build_io: BuildIo,
}

impl NativeBuildContext {
    pub fn new(project_root: &Path) -> Self {
        Self { build_project_root: project_root.to_owned(), build_io: BuildIo::default() }
    }
}
