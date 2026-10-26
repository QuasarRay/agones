// Source: codepropertygraph 301c4010d28ecdac05bc2ed44310f56446459465,
// project/Projects.scala. Upstream Apache-2.0 terms are retained in LICENSE.
#![allow(non_upper_case_globals)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Project {
    pub source_name: &'static str,
    pub directory: &'static str,
}

pub const codepropertygraph: Project = Project { source_name: "codepropertygraph", directory: "codepropertygraph" };
pub const schema: Project = Project { source_name: "schema", directory: "schema" };
pub const domainClasses: Project = Project { source_name: "domainClasses", directory: "domainClasses" };
pub const protoBindings: Project = Project { source_name: "protoBindings", directory: "proto-bindings" };
pub const schema2json: Project = Project { source_name: "schema2json", directory: "schema2json" };

pub const ALL: &[Project] = &[codepropertygraph, schema, domainClasses, protoBindings, schema2json];
