//! Native, schema-derived wire bindings. No JVM or protoc subprocess supplies runtime behavior.
use crate::{Error, Result, content_digest, protobuf_schema, schema_tools::{CanBuildCpgSchema, SchemaVariant}};
use cgp::prelude::*;
use protobuf::Message;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct BindingReport {
    pub format_version: u32,
    pub generator: String,
    pub inputs: BTreeMap<String,String>,
    pub files: BTreeMap<String,String>,
    pub mappings: Vec<Value>,
    pub descriptor_files: usize,
}
#[cgp_component(CpgProtocolBindingGenerator)]
pub trait CanGenerateCpgBindings { fn generate_cpg_bindings(&self, output:&Path)->Result<BindingReport>; }
#[cgp_new_provider]
impl<Context:CanBuildCpgSchema> CpgProtocolBindingGenerator<Context> for GenerateCpgProtocolBindings {
    fn generate_cpg_bindings(context:&Context,output:&Path)->Result<BindingReport> {
        let schema=context.build_cpg_schema(SchemaVariant::Protobuf)?;
        let protocol=protobuf_schema::render(&schema)?;
        let root=Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut inputs=BTreeMap::new();
        for relative in ["Cargo.toml","Cargo.lock","src/lib.rs","src/protobuf_bindings.rs","src/protobuf_schema.rs",
            "src/schema.rs","src/schema_tools.rs","generated/schema_constructors.rs","templates/cpg.proto.template","templates/pinned-category-order.json"] {
            inputs.insert(relative.to_owned(),content_digest(&std::fs::read(root.join(relative))?));
        }
        inputs.insert("nativeCompleteSchemaModel".into(),content_digest(&serde_json::to_vec(&schema)?));
        inputs.insert("nativeProtocol".into(),content_digest(protocol.as_bytes()));
        let receipt=output.join("protobuf.json");
        if receipt.exists() {
            let previous:BindingReport=serde_json::from_slice(&std::fs::read(&receipt)?)?;
            for (file,digest) in &previous.files {
                if content_digest(&std::fs::read(output.join(file))?)!=*digest {
                    return Err(Error::Invalid(format!("preserve edited generated protocol artifact {file}")));
                }
            }
            if previous.inputs==inputs {return Ok(previous);}
        } else {
            for name in ["cpg.rs","cpg.proto","cpg-descriptor.pb"] {
                if output.join(name).exists() {return Err(Error::Invalid(format!("unowned protocol output already exists: {name}")));}
            }
        }
        let work=tempfile::tempdir()?;
        let proto=work.path().join("cpg.proto");
        std::fs::write(&proto,&protocol)?;
        let parsed=protobuf_parse::Parser::new().pure().include(work.path()).input(&proto).parse_and_typecheck()
            .map_err(|error|Error::Invalid(format!("native protocol parse/typecheck: {error:#}")))?;
        if parsed.file_descriptors.len()!=1 || parsed.file_descriptors[0].name()!="cpg.proto" {
            return Err(Error::Invalid("unexpected protocol import/descriptor closure".into()));
        }
        let descriptor=protobuf::descriptor::FileDescriptorSet {file:parsed.file_descriptors,..Default::default()};
        std::fs::write(work.path().join("cpg-descriptor.pb"),descriptor.write_to_bytes()?)?;
        protobuf_codegen::Codegen::new().pure().include(work.path()).input(&proto).out_dir(work.path())
            .customize(protobuf_codegen::Customize::default().gen_mod_rs(false)).run()
            .map_err(|error|Error::Invalid(format!("native protocol Rust generation: {error:#}")))?;
        let rust=std::fs::read_to_string(work.path().join("cpg.rs"))?;
        let prefix="// @@protoc_insertion_point(";
        let markers:Vec<_>=rust.match_indices(prefix).map(|(start,_)| {
            let name_start=start+prefix.len();
            let end=rust[name_start..].find(')').ok_or_else(||Error::Invalid("unclosed generated protocol marker".into()))?+name_start;
            Ok((start,rust[name_start..end].to_owned()))
        }).collect::<Result<_>>()?;
        if markers.is_empty() {return Err(Error::Invalid("protocol generator omitted source mappings".into()));}
        let mappings=markers.iter().enumerate().map(|(index,(start,symbol))| {
            let end=markers.get(index+1).map_or(rust.len(),|(start,_)|*start);
            json!({"protoSymbol":symbol,"path":"cpg.rs","startByte":start,"endByteExclusive":end,
                "sha256":content_digest(&rust.as_bytes()[*start..end])})
        }).collect();
        let mut files=BTreeMap::new();
        std::fs::create_dir_all(output)?;
        for name in ["cpg.rs","cpg.proto","cpg-descriptor.pb"] {
            let bytes=std::fs::read(work.path().join(name))?;
            files.insert(name.to_owned(),content_digest(&bytes));
            let temporary=output.join(format!("{name}.tmp"));
            let mut target=std::fs::OpenOptions::new().create_new(true).write(true).open(&temporary)?;
            std::io::Write::write_all(&mut target,&bytes)?;
            std::io::Write::flush(&mut target)?;
            drop(target);std::fs::rename(temporary,output.join(name))?;
        }
        let report=BindingReport {format_version:1,generator:"protobuf-codegen/protobuf-parse 3.7.2, pure Rust".into(),
            inputs,files,mappings,descriptor_files:descriptor.file.len()};
        std::fs::write(receipt,serde_json::to_vec_pretty(&report)?)?;
        Ok(report)
    }
}
