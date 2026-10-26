// Source: codepropertygraph 301c4010d28ecdac05bc2ed44310f56446459465,
// project/project/Versions.scala. Upstream Apache-2.0 terms are retained in LICENSE.
#![allow(non_snake_case)]
use std::path::Path;
use std::sync::Arc;
use crate::{source_io::HasBuildIo, singletons::HasBuildSingletons, Result};

#[derive(Clone, Debug)]
pub struct Values { pub flatgraph: String }

pub fn values<Context: HasBuildIo + HasBuildSingletons>(context: &Context, project_root: &Path) -> Result<Arc<Values>> {
    context.build_singletons().build_definition_versions.get_or_initialize("Versions$", || {
        Ok(Values { flatgraph: parseVersion(context, project_root, "flatgraphVersion")? })
    })
}

pub fn flatgraph<Context: HasBuildIo + HasBuildSingletons>(context: &Context, project_root: &Path) -> Result<String> {
    Ok(values(context, project_root)?.flatgraph.clone())
}

pub(crate) fn parseVersion<Context: HasBuildIo>(
    context: &Context,
    project_root: &Path,
    key: &str,
) -> Result<String> {
    // Both upstream objects contain the same parser; one implementation owns it.
    crate::Versions::parseVersion(context, project_root, key)
}
