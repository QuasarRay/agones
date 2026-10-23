//! Native counterparts of ZipArchive and the deprecated SerializedCpg filesystem.
use crate::{Error,Result};
use protobuf::Message;
use std::{collections::{BTreeMap,BTreeSet},fs::{File,OpenOptions},io::{Read,Write},path::{Path,PathBuf},sync::{Mutex,OnceLock}};

#[derive(Clone)]
struct ArchiveNode { directory:bool, index:Option<usize>, children:Vec<String> }
pub struct ZipArchive {
    archive:Option<zip::ZipArchive<File>>,
    nodes:BTreeMap<String,ArchiveNode>,
    central_names:Vec<String>,
}
fn mount_name(name:&[u8])->Result<(String,bool)> {
    let name=std::str::from_utf8(name).map_err(|_|Error::Invalid("invalid UTF-8 ZIP entry name".into()))?;
    if name.contains('\0') {return Err(Error::Invalid("NUL in ZIP entry path".into()));}
    let directory=name.ends_with('/');
    let segments:Vec<_>=name.split('/').filter(|segment|!segment.is_empty()).collect();
    if segments.iter().any(|segment|matches!(*segment,"."|"..")) {
        return Err(Error::Invalid("ZIP filesystem cannot mount dot or dot-dot entry names".into()));
    }
    Ok((format!("/{}",segments.join("/")),directory))
}
fn parent_name(name:&str)->&str {
    name.rsplit_once('/').map_or("/",|(parent,_)|if parent.is_empty(){"/"}else{parent})
}
/// Resolving a path inside an already-mounted ZIP permits normal dot/dot-dot traversal.
fn resolve_name(name:&str)->Result<String> {
    if name.contains('\0') {return Err(Error::Invalid("NUL in ZIP filesystem path".into()));}
    let mut parts=Vec::new();
    for part in name.split('/') {match part {""|"."=>{},".."=>{parts.pop();},_=>parts.push(part)}}
    Ok(format!("/{}",parts.join("/")))
}
impl ZipArchive {
    pub fn open(path:&Path)->Result<Self> {
        let mut archive=zip::ZipArchive::new(File::open(path)?)?;
        let mut nodes=BTreeMap::new();
        let mut central_names=Vec::new();
        for index in 0..archive.len() {
            let entry=archive.by_index_raw(index)?;
            if entry.encrypted() || !matches!(entry.compression(),zip::CompressionMethod::Stored|zip::CompressionMethod::Deflated) {
                return Err(Error::Invalid("ZIP filesystem cannot mount encrypted entries or unsupported compression methods".into()));
            }
            let (name,directory)=mount_name(entry.name_raw())?;
            if !nodes.contains_key(&name) {central_names.push(name.clone());}
            nodes.insert(name,ArchiveNode {directory,index:Some(index),children:Vec::new()});
        }
        // ZipFileSystem uses a LinkedHashMap populated in central-directory order,
        // then prepends children while creating missing parent directories.
        nodes.entry("/".into()).or_insert(ArchiveNode {directory:true,index:None,children:Vec::new()});
        for name in &central_names {
            if name=="/" {continue;}
            let mut current=name.clone();
            loop {
                let parent=parent_name(&current).to_owned();
                if let Some(node)=nodes.get_mut(&parent) {node.children.insert(0,current);break;}
                nodes.insert(parent.clone(),ArchiveNode {directory:true,index:None,children:vec![current]});
                current=parent;
            }
        }
        Ok(Self {archive:Some(archive),nodes,central_names})
    }
    fn ensure_open(&self)->Result<()> {
        if self.archive.is_none() {Err(Error::ClosedFileSystem)} else {Ok(())}
    }
    pub fn entries(&self)->Result<Vec<String>> {
        self.ensure_open()?;
        fn walk(name:&str,nodes:&BTreeMap<String,ArchiveNode>,result:&mut Vec<String>) {
            let node=&nodes[name];
            if node.directory {for child in &node.children {walk(child,nodes,result);}}
            else {result.push(name.to_owned());}
        }
        let mut result=Vec::new();walk("/",&self.nodes,&mut result);Ok(result)
    }
    pub fn get_file_entries(&self)->Result<Vec<String>> {self.entries()}
    pub fn read(&mut self,path:&str)->Result<Vec<u8>> {
        self.ensure_open()?;
        let name=resolve_name(path)?;
        let node=self.nodes.get(&name).ok_or_else(||Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound,name.clone())))?;
        if node.directory {return Err(Error::Invalid(format!("cannot read ZIP directory {name}")));}
        let index=node.index.ok_or_else(||Error::Invalid(format!("regular ZIP entry lacks data: {name}")))?;
        let mut result=Vec::new();self.archive.as_mut().expect("checked open archive").by_index(index)?.read_to_end(&mut result)?;
        Ok(result)
    }
    pub fn close(&mut self)->Result<()> {self.archive.take();Ok(())}
}

struct ArchiveEditor {
    path:PathBuf, original:Option<ZipArchive>, updates:BTreeMap<String,Vec<u8>>, insertion_order:Vec<String>, closed:bool,
    lease:Option<SerializedArchiveLease>,
}
fn serialized_mounts()->&'static Mutex<BTreeSet<PathBuf>> {
    static MOUNTS:OnceLock<Mutex<BTreeSet<PathBuf>>>=OnceLock::new();MOUNTS.get_or_init(||Mutex::new(BTreeSet::new()))
}
/// Models the original URI-based filesystem's exclusive per-process mount ownership.
/// Independent path-based ZipArchive readers remain permitted while a writer is open.
struct SerializedArchiveLease(PathBuf);
impl Drop for SerializedArchiveLease {
    fn drop(&mut self) {if let Ok(mut mounts)=serialized_mounts().lock() {mounts.remove(&self.0);}}
}
/// The no-argument source constructor intentionally creates an empty/dummy writer.
/// That behavior remains distinct from an opened, possibly empty, ZIP filesystem.
pub struct SerializedCpg { filesystem:Option<ArchiveEditor>, counter:i32 }
impl Default for SerializedCpg {fn default()->Self {Self::empty()}}
impl SerializedCpg {
    pub fn empty()->Self {Self {filesystem:None,counter:0}}
    pub fn open(path:&Path)->Result<Self> {
        let mut mounts=serialized_mounts().lock().map_err(|_|Error::Invalid("poisoned serialized archive ownership".into()))?;
        if !path.exists() {
            let file=OpenOptions::new().create_new(true).write(true).read(true).open(path)?;
            zip::ZipWriter::new(file).finish()?.sync_all()?;
        }
        let canonical=path.canonicalize()?;
        if mounts.contains(&canonical) {return Err(Error::FileSystemAlreadyExists);}
        let original=ZipArchive::open(path)?;
        mounts.insert(canonical.clone());
        Ok(Self {filesystem:Some(ArchiveEditor {path:path.to_owned(),original:Some(original),updates:BTreeMap::new(),
            insertion_order:Vec::new(),closed:false,lease:Some(SerializedArchiveLease(canonical))}),counter:0})
    }
    pub fn is_empty(&self)->bool {self.filesystem.is_none()}
    pub fn add_overlay<M:Message>(&mut self,overlay:&M,name:&str)->Result<()> {
        if self.filesystem.is_none() {return Ok(());} // Explicit original dummy-writer behavior.
        let path=format!("{}_{name}",self.counter);
        self.counter=self.counter.wrapping_add(1);
        let filesystem=self.filesystem.as_mut().expect("checked configured filesystem");
        if filesystem.closed {return Err(Error::ClosedFileSystem);}
        let path=resolve_name(&path)?;
        let original=filesystem.original.as_ref().ok_or_else(||Error::Invalid("serialized CPG input was already closed".into()))?;
        let parent=parent_name(&path);
        if !original.nodes.get(parent).is_some_and(|node|node.directory) {
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound,parent.to_owned())));
        }
        if original.nodes.get(&path).is_some_and(|node|node.directory) {return Err(Error::Invalid(format!("overlay path is a directory: {path}")));}
        // Encode only after checking the path, as in Files.newOutputStream/writeTo.
        let bytes=overlay.write_to_bytes()?;
        if !filesystem.updates.contains_key(&path) {filesystem.insertion_order.push(path.clone());}
        filesystem.updates.insert(path,bytes);Ok(())
    }
    pub fn add_overlays<M:Message>(&mut self,overlays:impl IntoIterator<Item=M>,name:&str)->Result<()> {
        let mut index=0i32;
        for overlay in overlays {self.add_overlay(&overlay,&format!("{name}_{index}"))?;index=index.wrapping_add(1);}
        Ok(())
    }
    pub fn close(&mut self)->Result<()> {
        let Some(filesystem)=&mut self.filesystem else {return Ok(());};
        if filesystem.closed {return Ok(());}
        filesystem.closed=true;
        let _lease=filesystem.lease.take();
        if filesystem.updates.is_empty() {
            if let Some(original)=&mut filesystem.original {original.close()?;}
            return Ok(());
        }
        let parent=filesystem.path.parent().filter(|path|!path.as_os_str().is_empty()).unwrap_or_else(||Path::new("."));
        let permissions=std::fs::metadata(&filesystem.path)?.permissions();
        let temporary=tempfile::NamedTempFile::new_in(parent)?;
        let mut writer=zip::ZipWriter::new(temporary.reopen()?);
        let mut original=filesystem.original.take().ok_or_else(||Error::Invalid("missing original ZIP filesystem".into()))?;
        let mut written=BTreeSet::new();
        let options=zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        for name in &original.central_names {
            if let Some(bytes)=filesystem.updates.get(name) {
                writer.start_file(name.trim_start_matches('/'),options)?;writer.write_all(bytes)?;
            } else if let Some(index)=original.nodes[name].index {
                writer.raw_copy_file(original.archive.as_mut().expect("original archive open").by_index_raw(index)?)?;
            }
            written.insert(name.clone());
        }
        for name in &filesystem.insertion_order {
            if written.insert(name.clone()) {
                writer.start_file(name.trim_start_matches('/'),options)?;
                writer.write_all(&filesystem.updates[name])?;
            }
        }
        let file=writer.finish()?;file.set_permissions(permissions)?;file.sync_all()?;drop(file);drop(original);
        temporary.persist(&filesystem.path).map_err(|error|Error::Io(error.error))?;
        filesystem.updates.clear();Ok(())
    }
}
impl Drop for SerializedCpg {
    fn drop(&mut self) {if let Err(error)=self.close() {tracing::error!(%error,"failed to close serialized CPG archive");}}
}
