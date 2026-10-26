//! The two Versions objects have separate defining classloaders and state.
//! A failed initializer is remembered. The first access propagates its source
//! failure; later accesses fail without reopening files or rerunning parsing.
use std::sync::{Arc, OnceLock};
use cgp::prelude::*;
use crate::{Error, Result, Versions, ProjectVersions};

pub(crate) struct SourceSingleton<T> {
    result: OnceLock<std::result::Result<Arc<T>, InitializationFailure>>,
}
struct InitializationFailure { first: Error, later_cause: Error }
impl<T> Default for SourceSingleton<T> {
    fn default() -> Self { Self { result: OnceLock::new() } }
}
impl<T> SourceSingleton<T> {
    pub fn get_or_initialize(&self, class_name: &str, initialize: impl FnOnce() -> Result<T>) -> Result<Arc<T>> {
        let mut initialized_here = false;
        let result = self.result.get_or_init(|| {
            initialized_here = true;
            initialize().map(Arc::new).map_err(|failure| {
                let thread = std::thread::current();
                let later_cause = Error::initialization_snapshot(&failure, thread.name().unwrap_or("main"));
                InitializationFailure { first: Error::initializer_failure(failure), later_cause }
            })
        });
        match result {
            Ok(value) => Ok(Arc::clone(value)),
            Err(error) if initialized_here => Err(error.first.clone()),
            Err(error) => Err(Error::failed_class(class_name, &error.later_cause)),
        }
    }
}

#[derive(Default)]
pub struct BuildSingletons {
    pub(crate) project_versions: Arc<SourceSingleton<Versions::Values>>,
    pub(crate) build_definition_versions: Arc<SourceSingleton<ProjectVersions::Values>>,
}

#[derive(Clone, Copy, Debug)]
pub enum DefinitionReload { ReuseDefiningClasses, ProjectDefinition, MetaBuildAndProject }

impl BuildSingletons {
    /// Replacing a project definition need not replace its parent meta-build
    /// classloader. The native host supplies the actual defining-scope event;
    /// existing readers and values keep their old owners until phase teardown.
    pub fn for_reload(&self, scope: DefinitionReload) -> Self {
        match scope {
            DefinitionReload::ReuseDefiningClasses => Self {
                project_versions: Arc::clone(&self.project_versions),
                build_definition_versions: Arc::clone(&self.build_definition_versions),
            },
            DefinitionReload::ProjectDefinition => Self {
                project_versions: Arc::default(),
                build_definition_versions: Arc::clone(&self.build_definition_versions),
            },
            DefinitionReload::MetaBuildAndProject => Self::default(),
        }
    }
}

#[cgp_getter]
pub trait HasBuildSingletons {
    fn build_singletons(&self) -> &BuildSingletons;
}
