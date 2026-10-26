// Source: codepropertygraph 301c4010d28ecdac05bc2ed44310f56446459465,
// project/Versions.scala. Upstream Apache-2.0 terms are retained in LICENSE.
#![allow(non_snake_case, non_upper_case_globals)]
use std::path::Path;
use crate::{source_io::HasBuildIo, Error, Result};

pub const scalatest: &str = "3.2.10";
pub const ujson: &str = "4.4.3";

pub fn flatgraph<Context: HasBuildIo>(context: &Context, project_root: &Path) -> Result<String> {
    parseVersion(context, project_root, "flatgraphVersion")
}

pub fn parseVersion<Context: HasBuildIo>(
    context: &Context,
    project_root: &Path,
    key: &str,
) -> Result<String> {
    let text = context.build_io().read_source_text(&project_root.join("build.sbt"))?;
    // Scala regex extractors match the whole line. Do not accept a trailing
    // comment or arbitrary whitespace that the source pattern rejects.
    let pattern = format!(r#"\A.*val {key}[ ]+=[ ]?"(.*?)"\z"#);
    let expression = regex::Regex::new(&pattern)
        .map_err(|error| Error::Invalid(error.to_string()))?;
    let marker = format!("val {key}");
    let versions: Vec<_> = text.split(['\r', '\n'])
        .filter(|line| line.contains(&marker))
        .filter_map(|line| expression.captures(line))
        .map(|capture| capture[1].to_owned())
        .collect();
    if versions.len() != 1 {
        return Err(Error::SourceAssertion(format!(
            "assertion failed: unable to extract {key} from build.sbt, expected exactly one line like `val {key}= \"0.0.0-SNAPSHOT\"`."
        )));
    }
    Ok(versions.into_iter().next().expect("one source version"))
}
