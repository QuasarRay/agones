//! Source-compatible format probes shared by the CPG loading/conversion lifecycle.
use std::{io::Read,path::Path};
use crate::{Error,Result,graph::Cpg};
#[cfg(feature="protobuf-bindings")]
use crate::graph::{CanCreateCpg,CanOpenCpgStorage,legacy::CanImportLegacyCpg};
use cgp::prelude::*;

pub fn probe_first_bytes(path:&Path,probe_for:&str)->bool {
    let read=(||->std::io::Result<bool> {
        let mut file=std::fs::File::open(path)?;
        // The original allocates probeFor.size UTF-16 units, then decodes the bytes.
        let mut buffer=vec![0;probe_for.encode_utf16().count()];
        let _read=file.read(&mut buffer)?;
        Ok(crate::utilities::decode_utf8_replacing(&buffer)==probe_for)
    })();
    read.unwrap_or(false)
}
pub fn is_proto_format(path:impl AsRef<Path>)->bool {probe_first_bytes(path.as_ref(),"PK")}
pub fn is_overflow_db_format(path:impl AsRef<Path>)->bool {probe_first_bytes(path.as_ref(),"H:2")}
pub fn is_flatgraph_format(path:impl AsRef<Path>)->bool {probe_first_bytes(path.as_ref(),"FLT GRPH")}
pub fn is_grafeo_format(path:impl AsRef<Path>)->bool {probe_first_bytes(path.as_ref(),"GRAF")}
#[deprecated(note="use is_proto_format")]
pub fn is_legacy_cpg(path:impl AsRef<Path>)->bool {is_proto_format(path)}

#[cfg(feature="protobuf-bindings")]
fn missing(path:&Path)->Error {Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound,
    format!("given input file {} does not exist",path.display())))}
#[cfg(feature="protobuf-bindings")]
fn remove_target(path:&Path)->Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir()=>std::fs::remove_dir(path)?,
        Ok(_)=>std::fs::remove_file(path)?,
        Err(error) if error.kind()==std::io::ErrorKind::NotFound=>{},Err(error)=>return Err(error.into()),
    }Ok(())
}
#[cfg(feature="protobuf-bindings")]
fn default_storage_path(path:&Path)->Result<std::path::PathBuf> {
    let mut name=path.file_name().ok_or_else(||Error::Invalid("input has no file name".into()))?.to_os_string();name.push(".grafeo");
    Ok(path.with_file_name(name))
}
#[cgp_component(CpgLoader)]
pub trait CanLoadCpg {
    fn load_cpg(&self,input:&Path)->Result<Cpg>;
    fn load_cpg_to(&self,input:&Path,persist_to:&Path)->Result<Cpg>;
}
#[cfg(feature="protobuf-bindings")]
#[cgp_new_provider]
impl<Context> CpgLoader<Context> for LoadCpgIntoGrafeo
where Context:CanCreateCpg+CanOpenCpgStorage+CanImportLegacyCpg+crate::proto_loading::CanLoadCpgProtos {
    fn load_cpg(context:&Context,input:&Path)->Result<Cpg> {
        let absolute=std::path::absolute(input)?;
        if !absolute.exists() {return Err(missing(&absolute));}
        if is_grafeo_format(&absolute) {return context.open_cpg_storage(Some(&absolute),true);}
        if is_proto_format(&absolute)||is_overflow_db_format(&absolute)||is_flatgraph_format(&absolute) {
            return Self::load_cpg_to(context,&absolute,&default_storage_path(&absolute)?);
        }
        // The original one-path overload lets its native storage reader report
        // unknown-format errors, rather than using the two-path assertion branch.
        context.open_cpg_storage(Some(&absolute),true)
    }
    fn load_cpg_to(context:&Context,input:&Path,persist_to:&Path)->Result<Cpg> {
        let absolute=std::path::absolute(input)?;
        if persist_to!=input {remove_target(persist_to)?;}
        if !absolute.exists() {return Err(missing(&absolute));}
        if is_proto_format(&absolute) {
            tracing::debug!(input=%absolute.display(),output=%persist_to.display(),"converting protobuf CPG to Grafeo");
            return Ok(context.load_proto_zip(&absolute,Some(persist_to))?.0);
        }
        if is_overflow_db_format(&absolute) {
            tracing::info!(input=%absolute.display(),output=%persist_to.display(),"converting legacy OverflowDB CPG to Grafeo");
            return context.import_legacy_cpg(&absolute,persist_to);
        }
        if is_grafeo_format(&absolute) {
            if input!=persist_to {std::fs::copy(&absolute,persist_to)?;}
            return context.open_cpg_storage(Some(persist_to),true);
        }
        if is_flatgraph_format(&absolute) {
            let schema=std::sync::Arc::new(context.create_cpg()?.graph().schema().clone());
            let mut graph=crate::graph::storage::read_graph(schema,&absolute)?;
            graph.save_database(persist_to)?;graph.configure_storage(persist_to);return Ok(Cpg::from_graph(graph));
        }
        Err(Error::SourceAssertion("unknown file format - we probed the first bytes but it didn't look like one of our known formats (proto.zip, flatgraph, overflowdb, Grafeo)".into()))
    }
}
#[cfg(all(feature="native-schema",feature="protobuf-bindings"))]
impl Cpg {
    pub fn load(path:impl AsRef<Path>)->Result<Self>{crate::graph::CpgContext.load_cpg(path.as_ref())}
    pub fn load_to(input:impl AsRef<Path>,persist_to:impl AsRef<Path>)->Result<Self>{crate::graph::CpgContext.load_cpg_to(input.as_ref(),persist_to.as_ref())}
}
