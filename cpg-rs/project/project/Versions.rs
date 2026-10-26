// Source: codepropertygraph 301c4010d28ecdac05bc2ed44310f56446459465,
// project/project/Versions.scala. Upstream Apache-2.0 terms are retained in LICENSE.
#![allow(non_snake_case)]
use std::path::Path;
use crate::{source_io::HasBuildIo, Result};

pub fn flatgraph<Context: HasBuildIo>(context: &Context, project_root: &Path) -> Result<String> {
    parseVersion(context, project_root, "flatgraphVersion")
}

pub fn parseVersion<Context: HasBuildIo>(
    context: &Context,
    project_root: &Path,
    key: &str,
) -> Result<String> {
    // Both upstream objects contain the same parser; one implementation owns it.
    crate::Versions::parseVersion(context, project_root, key)
}
