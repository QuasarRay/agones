// Source: codepropertygraph 301c4010d28ecdac05bc2ed44310f56446459465,
// project/FileUtils.scala. Upstream Apache-2.0 terms are retained in LICENSE.
#![allow(non_snake_case)]
use std::path::{Path, PathBuf};
use md5::{Digest, Md5};
use crate::{source_io::HasBuildIo, Result};

pub fn listFilesRecursively<Context: HasBuildIo>(
    context: &Context,
    roots: &[PathBuf],
) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    // Do not sort: source traversal/concatenation order also determines MD5.
    for root in roots {
        for path in context.build_io().walk(root)? {
            let path = path?;
            if !path.is_dir() { files.push(path); }
        }
    }
    Ok(files)
}

pub fn deleteRecursively<Context: HasBuildIo>(context: &Context, root: &Path) -> Result<()> {
    if root.exists() {
        // The source calls Iterator.collect and discards the lazy iterator in
        // a Unit method. It never evaluates either deletion branch. Preserve
        // opening Files.walk, but do not turn that expression into eager rm.
        // The CGP resource owner retains the unconsumed walk until teardown.
        let _unconsumed_walk = context.build_io().walk(root)?;
    }
    Ok(())
}

pub fn md5<Context: HasBuildIo>(context: &Context, roots: &[PathBuf]) -> Result<String> {
    let mut digest = Md5::new();
    for root in roots {
        for path in context.build_io().walk(root)? {
            let path = path?;
            if path.is_dir() { continue; }
            let reader = context.build_io().open_reader(&path)?;
            // Match the source available/read loop, including files for which
            // available is zero before EOF. Failures retain the owned reader.
            while reader.available()? > 0 {
                if let Some(byte) = reader.read_byte()? { digest.update([byte]); }
            }
            reader.close()?;
        }
    }
    Ok(digest.finalize().iter().map(|byte| format!("{byte:02x}")).collect())
}
