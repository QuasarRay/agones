//! CGP-owned native schema construction and source-compatible schema tool effects.
use crate::{cpg_schema,schema::Schema,protobuf_schema,Result};
use cgp::prelude::*;
use std::{io::Write,path::{Path,PathBuf}};

#[derive(Clone,Copy)]
pub enum SchemaVariant { Domain, Protobuf }
#[cgp_component(NativeSchemaFactory)]
pub trait CanBuildCpgSchema { fn build_cpg_schema(&self,variant:SchemaVariant)->Result<Schema>; }
#[cgp_new_provider]
impl<Context> NativeSchemaFactory<Context> for ConstructPinnedCpgSchema {
    fn build_cpg_schema(_context:&Context,variant:SchemaVariant)->Result<Schema> {
        match variant {SchemaVariant::Domain=>cpg_schema::domain_schema(),SchemaVariant::Protobuf=>cpg_schema::protobuf_schema()}
    }
}
#[cgp_component(CpgProtobufGenerator)]
pub trait CanGenerateCpgProto { fn generate_cpg_proto(&self,directory:&Path)->Result<PathBuf>; }
#[cgp_new_provider]
impl<Context:CanBuildCpgSchema> CpgProtobufGenerator<Context> for GenerateCpgProtocol {
    fn generate_cpg_proto(context:&Context,directory:&Path)->Result<PathBuf> {
        protobuf_schema::write(&context.build_cpg_schema(SchemaVariant::Protobuf)?,directory)
    }
}
#[cgp_component(CpgSchemaJsonExporter)]
pub trait CanExportCpgSchemaJson { fn export_cpg_schema_json(&self,output:&Path)->Result<()>; }
#[cgp_new_provider]
impl<Context:CanBuildCpgSchema> CpgSchemaJsonExporter<Context> for ExportCpgSchemaJson {
    fn export_cpg_schema_json(context:&Context,output:&Path)->Result<()> {
        let schema=context.build_cpg_schema(SchemaVariant::Domain)?;
        let mut file=std::fs::OpenOptions::new().write(true).create_new(true).open(output)?;
        file.write_all(serde_json::to_string(&schema.standard_json())?.as_bytes())?;
        Ok(())
    }
}
pub struct NativeCpgSchemaContext;
delegate_and_check_components! {
    NativeCpgSchemaContext {
        NativeSchemaFactoryComponent:ConstructPinnedCpgSchema,
        CpgProtobufGeneratorComponent:GenerateCpgProtocol,
        CpgSchemaJsonExporterComponent:ExportCpgSchemaJson,
    }
}
