// Source: codepropertygraph 301c4010d28ecdac05bc2ed44310f56446459465,
// project/Versions.scala. Upstream Apache-2.0 terms are retained in LICENSE.
#![allow(non_snake_case, non_upper_case_globals)]
use std::path::Path;
use std::sync::Arc;
use crate::{source_io::HasBuildIo, singletons::HasBuildSingletons, Error, Result};

#[derive(Clone, Debug)]
pub struct Values {
    pub flatgraph: String,
    pub scalatest: &'static str,
    pub ujson: &'static str,
}

pub fn values<Context: HasBuildIo + HasBuildSingletons>(context: &Context, project_root: &Path) -> Result<Arc<Values>> {
    context.build_singletons().project_versions.get_or_initialize("Versions$", || {
        // Scala val initializers execute in declaration order; later constants
        // are unavailable when parsing the first field fails.
        Ok(Values { flatgraph: parseVersion(context, project_root, "flatgraphVersion")?,
            scalatest: "3.2.10", ujson: "4.4.3" })
    })
}

pub fn flatgraph<Context: HasBuildIo + HasBuildSingletons>(context: &Context, project_root: &Path) -> Result<String> {
    Ok(values(context, project_root)?.flatgraph.clone())
}

pub fn scalatest<Context: HasBuildIo + HasBuildSingletons>(context: &Context, project_root: &Path) -> Result<&'static str> {
    Ok(values(context, project_root)?.scalatest)
}

pub fn ujson<Context: HasBuildIo + HasBuildSingletons>(context: &Context, project_root: &Path) -> Result<&'static str> {
    Ok(values(context, project_root)?.ujson)
}

pub(crate) fn parseVersion<Context: HasBuildIo>(
    context: &Context,
    project_root: &Path,
    key: &str,
) -> Result<String> {
    // Scala regex extractors match the whole line. Do not accept a trailing
    // comment or arbitrary whitespace that the source pattern rejects.
    // Java's dot also excludes NEL and Unicode line/paragraph separators.
    let pattern = format!(r#"\A[^\r\n\u{{85}}\u{{2028}}\u{{2029}}]*val {key}[ ]+=[ ]?"([^\r\n\u{{85}}\u{{2028}}\u{{2029}}]*?)"\z"#);
    let expression = regex::Regex::new(&pattern)
        .map_err(|error| Error::exception("java.util.regex.PatternSyntaxException", Some(error.to_string())))?;
    let marker = format!("val {key}");
    let mut versions = Vec::new();
    context.build_io().for_each_source_line_at(project_root, Path::new("build.sbt"), |line| {
        if line.contains(&marker) {
            if let Some(capture) = expression.captures(line) { versions.push(capture[1].to_owned()); }
        }
    })?;
    if versions.len() != 1 {
        return Err(Error::SourceAssertion(format!(
            "assertion failed: unable to extract {key} from build.sbt, expected exactly one line like `val {key}= \"0.0.0-SNAPSHOT\"`."
        )));
    }
    Ok(versions.into_iter().next().expect("one source version"))
}
