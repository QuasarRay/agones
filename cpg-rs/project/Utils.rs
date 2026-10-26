// Source: codepropertygraph 301c4010d28ecdac05bc2ed44310f56446459465,
// project/Utils.scala. Upstream Apache-2.0 terms are retained in LICENSE.
#![allow(non_snake_case)]
use std::{path::Path, process::{Command, Stdio}};
use crate::Result;

pub struct GitLFSUtils;

impl GitLFSUtils {
    pub fn isGitLFSEnabled(project_root: &Path) -> Result<bool> {
        let status = Command::new("git")
            .args(["lfs", "ls-files"])
            .current_dir(project_root)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        Ok(status.success())
    }
}
