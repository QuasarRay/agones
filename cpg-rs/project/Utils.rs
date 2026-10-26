// Source: codepropertygraph 301c4010d28ecdac05bc2ed44310f56446459465,
// project/Utils.scala. Upstream Apache-2.0 terms are retained in LICENSE.
#![allow(non_snake_case)]
use std::{path::Path, process::Command};
use crate::{process, Result};

pub struct GitLFSUtils;

impl GitLFSUtils {
    pub fn isGitLFSEnabled(project_root: &Path) -> Result<bool> {
        let mut command = Command::new("git");
        command
            .args(["lfs", "ls-files"])
            .current_dir(project_root);
        let status = process::discard_logged_output(&mut command, "git")?;
        Ok(status.success())
    }
}
