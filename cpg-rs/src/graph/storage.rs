//! Native pinned Flatgraph v0 storage: typed Zstandard columns and the original manifest.
use super::*;
use serde::{Serialize,Deserialize};
use std::{fs::{File,OpenOptions},io::{Read,Write,Seek,SeekFrom},path::Path};

pub const MAGIC:&[u8;8]=b"FLT GRPH";
#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Deserialize)]
#[serde(rename_all="lowercase")]
pub enum StorageType {Bool,Byte,Short,Int,Long,Ref,String,Double,Float}
impl StorageType {
    pub fn value_type(self)->ValueType {match self {Self::Bool=>ValueType::Boolean,Self::Byte=>ValueType::Byte,
        Self::Short=>ValueType::Short,Self::Int=>ValueType::Int,Self::Long=>ValueType::Long,Self::Ref=>ValueType::NodeRef,
        Self::String=>ValueType::String,Self::Double=>ValueType::Double,Self::Float=>ValueType::Float}}
    fn from_value_type(value:ValueType)->Result<Self> {Ok(match value {
        ValueType::Boolean=>Self::Bool,ValueType::Byte=>Self::Byte,ValueType::Short=>Self::Short,ValueType::Int=>Self::Int,
        ValueType::Long=>Self::Long,ValueType::NodeRef=>Self::Ref,ValueType::String=>Self::String,
        ValueType::Double=>Self::Double,ValueType::Float=>Self::Float,
        _=>return Err(Error::SourceMatchFailure {operation:"storage encoding of a non-array/unsupported source storage type"}),
    })}
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Outline {#[serde(rename="type")]pub kind:StorageType,pub start_offset:u64,pub compressed_length:usize,pub decompressed_length:usize}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct NodeItem {pub node_label:String,pub nnodes:usize,#[serde(default)]pub deletions:Option<Vec<usize>>}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct EdgeItem {pub node_label:String,pub edge_label:String,pub inout:u8,pub qty:Option<Outline>,pub neighbors:Option<Outline>,pub property:Option<Outline>}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PropertyItem {pub node_label:String,pub property_label:String,pub qty:Option<Outline>,pub property:Option<Outline>}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Manifest {pub version:i32,pub nodes:Vec<NodeItem>,pub edges:Vec<EdgeItem>,pub properties:Vec<PropertyItem>,pub string_pool_length:Outline,pub string_pool_bytes:Outline}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Counts {pub node_kinds:usize,pub edge_kinds:usize,pub property_kinds:usize}

fn invalid(message:impl Into<String>)->Error {Error::Deserialization(message.into())}
pub fn read_manifest(path:&Path)->Result<Manifest> {
    let mut file=File::open(path)?;let size=file.metadata()?.len();
    if size<16 {return Err(invalid(format!("corrupt file: expected at least 16 bytes, but only found {size}")));}
    let mut header=[0u8;16];file.read_exact(&mut header)?;
    if &header[..8]!=MAGIC {return Err(invalid("expected header 'FLT GRPH'"));}
    let offset=i64::from_le_bytes(header[8..].try_into().expect("eight-byte offset"));
    if offset<0 || offset as u64>=size {return Err(invalid(format!("corrupt file: manifest offset ({offset}) is out of bounds (file size: {size})")));}
    if size-offset as u64>i32::MAX as u64 {return Err(invalid("corrupt file: unreasonably large manifest size"));}
    file.seek(SeekFrom::Start(offset as u64))?;
    let mut bytes=Vec::new();file.read_to_end(&mut bytes)?;
    let manifest:Manifest=serde_json::from_slice(&bytes)?;
    if manifest.version!=0 {return Err(Error::Runtime(format!("unsupported Flatgraph manifest version {}",manifest.version)));}
    Ok(manifest)
}
fn read_bytes(file:&mut File,outline:&Outline)->Result<Vec<u8>> {
    let end=outline.start_offset.checked_add(outline.compressed_length as u64).ok_or_else(||invalid("column offset overflow"))?;
    if end>file.metadata()?.len() {return Err(invalid("column extends beyond storage file"));}
    file.seek(SeekFrom::Start(outline.start_offset))?;
    let mut bytes=vec![0u8;outline.compressed_length];file.read_exact(&mut bytes)?;
    let decoded=zstd::bulk::decompress(&bytes,outline.decompressed_length)?;
    if decoded.len()!=outline.decompressed_length {return Err(invalid("column decompressed length differs from manifest"));}
    Ok(decoded)
}
fn read_pool(file:&mut File,manifest:&Manifest)->Result<Vec<Arc<str>>> {
    if manifest.string_pool_length.kind!=StorageType::Int || manifest.string_pool_bytes.kind!=StorageType::Byte {return Err(invalid("invalid string pool storage types"));}
    let lengths=read_bytes(file,&manifest.string_pool_length)?;
    let bytes=read_bytes(file,&manifest.string_pool_bytes)?;let mut offset=0usize;let mut pool=Vec::new();
    for chunk in lengths.chunks_exact(4) {
        let length=i32::from_le_bytes(chunk.try_into().expect("integer chunk"));
        if length<0 {return Err(invalid("negative string pool length"));}
        let end=offset.checked_add(length as usize).filter(|end|*end<=bytes.len()).ok_or_else(||invalid("string pool entry exceeds buffer"))?;
        pool.push(Arc::from(crate::utilities::decode_utf8_replacing(&bytes[offset..end])));offset=end;
    }
    Ok(pool)
}
fn read_array(file:&mut File,outline:Option<&Outline>,nodes:&[Option<Vec<NodeHandle>>],pool:&[Arc<str>])->Result<Vec<Value>> {
    let Some(outline)=outline else {return Ok(Vec::new());};let bytes=read_bytes(file,outline)?;
    let values=match outline.kind {
        StorageType::Bool=>bytes.iter().map(|byte|match byte {0=>Ok(Value::Boolean(false)),1=>Ok(Value::Boolean(true)),_=>Err(Error::SourceMatchFailure {operation:"non-boolean storage byte"})}).collect::<Result<_>>()?,
        StorageType::Byte=>bytes.iter().map(|byte|Value::Byte(*byte as i8)).collect(),
        StorageType::Short=>bytes.chunks_exact(2).map(|chunk|Value::Short(i16::from_le_bytes(chunk.try_into().expect("short chunk")))).collect(),
        StorageType::Int=>bytes.chunks_exact(4).map(|chunk|Value::Int(i32::from_le_bytes(chunk.try_into().expect("int chunk")))).collect(),
        StorageType::Long=>bytes.chunks_exact(8).map(|chunk|Value::Long(i64::from_le_bytes(chunk.try_into().expect("long chunk")))).collect(),
        StorageType::Float=>bytes.chunks_exact(4).map(|chunk|Value::Float(u32::from_le_bytes(chunk.try_into().expect("float chunk")))).collect(),
        StorageType::Double=>bytes.chunks_exact(8).map(|chunk|Value::Double(u64::from_le_bytes(chunk.try_into().expect("double chunk")))).collect(),
        StorageType::String=>bytes.chunks_exact(4).map(|chunk| {
            let index=i32::from_le_bytes(chunk.try_into().expect("string index"));
            if index<0 {Ok(Value::Null)} else {pool.get(index as usize).cloned().map(Value::String).ok_or_else(||invalid("string index out of bounds"))}
        }).collect::<Result<_>>()?,
        StorageType::Ref=>bytes.chunks_exact(8).map(|chunk| {
            let encoded=u64::from_le_bytes(chunk.try_into().expect("node reference"));let kind=(encoded>>32) as i32;let sequence=encoded as u32 as i32;
            if kind<0 || kind as usize>=nodes.len() {return Ok(Value::Null);}
            let remapping=nodes[kind as usize].as_ref().ok_or_else(||invalid("reference to node kind missing from supplied schema"))?;
            if sequence<0 {return Err(invalid("negative stored reference sequence"));}
            remapping.get(sequence as usize).copied().map(Value::Node).ok_or_else(||invalid("stored reference sequence out of bounds"))
        }).collect::<Result<_>>()?,
    };
    Ok(values)
}
fn quantities(file:&mut File,outline:Option<&Outline>,nnodes:usize,pool:&[Arc<str>])->Result<Vec<usize>> {
    let Some(outline)=outline else {return Err(invalid("missing quantity column"));};
    if outline.kind!=StorageType::Int {return Err(invalid("quantity column is not int"));}
    let deltas=read_array(file,Some(outline),&[],pool)?;let mut total=0usize;let mut offsets=Vec::with_capacity(deltas.len());
    for delta in deltas {offsets.push(total);let Value::Int(delta)=delta else {return Err(invalid("non-integer quantity"));};
        if delta<0 {return Err(invalid("negative quantity delta"));}total=total.checked_add(delta as usize).ok_or_else(||invalid("quantity overflow"))?;}
    if offsets.len()<nnodes+1 {return Err(invalid("quantity column is shorter than node count"));}Ok(offsets)
}

pub fn read_graph(schema:Arc<Schema>,path:&Path,persist_on_close:bool)->Result<Graph> {
    let manifest=read_manifest(path)?;let mut file=File::open(path)?;let pool=read_pool(&mut file,&manifest)?;
    let mut graph=Graph::new(schema)?;let mut remapper=Vec::with_capacity(manifest.nodes.len());
    for item in &manifest.nodes {
        let Some(kind)=graph.kinds.get(&item.node_label).copied() else {remapper.push(None);continue;};
        if item.nnodes>i32::MAX as usize {return Err(invalid("node count exceeds source sequence range"));}
        let mut nodes=Vec::with_capacity(item.nnodes);let mut handles=Vec::with_capacity(item.nnodes);
        let deleted:BTreeSet<_>=item.deletions.iter().flatten().copied().collect();
        if deleted.iter().any(|seq|*seq>=item.nnodes) || deleted.len()!=item.deletions.as_ref().map_or(0,Vec::len) {return Err(invalid("invalid or duplicate node deletion"));}
        for sequence in 0..item.nnodes {let handle=NodeHandle {graph:graph.identity,id:NodeId::new(kind,sequence as u32)};
            nodes.push(StoredNode {handle,label:item.node_label.clone(),properties:BTreeMap::new(),deleted:deleted.contains(&sequence)});handles.push(handle);}
        graph.nodes[kind as usize]=nodes;remapper.push(Some(handles));
    }
    for item in &manifest.properties {
        let Some(kind)=graph.kinds.get(&item.node_label).copied() else {continue;};
        if !graph.fields[kind as usize].contains_key(&item.property_label) {continue;}
        let count=graph.nodes[kind as usize].len();let qty=quantities(&mut file,item.qty.as_ref(),count,&pool)?;
        let values=read_array(&mut file,item.property.as_ref(),&remapper,&pool)?;
        graph.property_slots.insert((kind,item.property_label.clone()));
        if let Some(outline)=&item.property {graph.property_storage_types.insert((kind,item.property_label.clone()),outline.kind);}
        for sequence in 0..count {
            let start=qty[sequence];let end=qty[sequence+1];
            if start>end || end>values.len() {return Err(invalid("property offsets exceed values"));}
            if start!=end {graph.nodes[kind as usize][sequence].properties.insert(item.property_label.clone(),values[start..end].to_vec());}
        }
    }
    // Outgoing and incoming columns carry independent encounter order. Pair the nth
    // matching half-edge while retaining both order values, including unsafe halves.
    let mut halves:Vec<(NodeHandle,NodeHandle,u16,u8,u64,Value)>=Vec::new();
    for item in &manifest.edges {
        if item.inout>1 {return Err(Error::IllegalArgument("illegal encoding for direction".into()));}
        let (Some(kind),Some(edge_kind))=(graph.kinds.get(&item.node_label).copied(),graph.edge_kinds.get(&item.edge_label).copied()) else {continue;};
        let count=graph.nodes[kind as usize].len();let qty=quantities(&mut file,item.qty.as_ref(),count,&pool)?;
        if item.neighbors.as_ref().is_some_and(|outline|outline.kind!=StorageType::Ref) {return Err(invalid("neighbor column is not ref"));}
        let neighbors=read_array(&mut file,item.neighbors.as_ref(),&remapper,&pool)?;
        let properties=read_array(&mut file,item.property.as_ref(),&remapper,&pool)?;
        graph.adjacency_slots.insert((kind,edge_kind,item.inout));
        if let Some(outline)=&item.property {graph.edge_property_slots.insert((kind,edge_kind,item.inout));graph.edge_property_storage_types.insert((kind,edge_kind,item.inout),outline.kind);}
        let default=graph.edge_property(edge_kind,EdgeProperty::Default)?;
        for sequence in 0..count {
            let start=qty[sequence];let end=qty[sequence+1];if start>end || end>neighbors.len() {return Err(invalid("neighbor offsets exceed values"));}
            let base=graph.nodes[kind as usize][sequence].handle;
            for position in start..end {
                let Value::Node(neighbor)=neighbors[position] else {return Err(invalid("null/missing graph neighbor"));};
                let property=if item.property.is_none(){default.clone()}else{properties.get(position).cloned().ok_or_else(||invalid("edge property column shorter than neighbors"))?};
                let (source,target)=if item.inout==1 {(base,neighbor)} else {(neighbor,base)};
                halves.push((source,target,edge_kind,item.inout,(position-start) as u64,property));
            }
        }
    }
    // BTreeMap keys use canonical bit/identity encodings instead of floating Eq.
    let mut paired:BTreeMap<Vec<u8>,VecDeque<u64>>=BTreeMap::new();
    for (source,target,kind,direction,order,property) in halves.iter().filter(|half|half.3==1) {
        let id=graph.next_edge;graph.next_edge+=1;
        graph.edges.insert(id,Edge {handle:EdgeHandle {graph:graph.identity,id},source:*source,target:*target,kind:*kind,
            label:graph.schema.edges[*kind as usize].name.clone(),property:property.clone(),outgoing:true,incoming:false,out_order:*order,in_order:0});
        paired.entry(edge_key(*source,*target,*kind,property)?).or_default().push_back(id);
        let _=direction;
    }
    for (source,target,kind,_,order,property) in halves.into_iter().filter(|half|half.3==0) {
        let key=edge_key(source,target,kind,&property)?;
        if let Some(id)=paired.get_mut(&key).and_then(VecDeque::pop_front) {
            let edge=graph.edges.get_mut(&id).expect("paired outgoing edge");edge.incoming=true;edge.in_order=order;
        } else {
            let id=graph.next_edge;graph.next_edge+=1;
            graph.edges.insert(id,Edge {handle:EdgeHandle {graph:graph.identity,id},source,target,kind,
                label:graph.schema.edges[kind as usize].name.clone(),property,outgoing:false,incoming:true,out_order:0,in_order:order});
        }
    }
    graph.storage_path=persist_on_close.then(||path.to_owned());graph.changed=false;Ok(graph)
}
fn edge_key(source:NodeHandle,target:NodeHandle,kind:u16,property:&Value)->Result<Vec<u8>> {
    let mut bytes=Vec::new();bytes.extend(source.id.0.to_le_bytes());bytes.extend(target.id.0.to_le_bytes());bytes.extend(kind.to_le_bytes());
    // Edge properties are schema-typed; include the canonical scalar kind to avoid collisions.
    match property {
        Value::Null=>bytes.push(0),Value::String(value)=>{bytes.push(1);bytes.extend(value.as_bytes());},
        value=>{let kind=value_storage_type(value)?;bytes.push(kind as u8+2);bytes.extend(encode_values(std::slice::from_ref(value),kind,&mut StringPool::default())?);}
    }
    Ok(bytes)
}
fn value_storage_type(value:&Value)->Result<StorageType> {Ok(match value {
    Value::Boolean(_)=>StorageType::Bool,Value::Byte(_)=>StorageType::Byte,Value::Short(_)=>StorageType::Short,Value::Int(_)=>StorageType::Int,
    Value::Long(_)=>StorageType::Long,Value::Float(_)=>StorageType::Float,Value::Double(_)=>StorageType::Double,Value::String(_)=>StorageType::String,
    Value::Node(_)|Value::Null=>StorageType::Ref,_=>return Err(Error::SourceMatchFailure {operation:"unsupported edge property storage array"}),
})}
#[derive(Default)]
struct StringPool {indices:BTreeMap<Arc<str>,i32>,values:Vec<Arc<str>>}
impl StringPool {
    fn insert(&mut self,value:&Arc<str>)->Result<i32> {
        if let Some(index)=self.indices.get(value) {return Ok(*index);}
        let index=i32::try_from(self.values.len()).map_err(|_|Error::Invalid("string pool exceeds signed index range".into()))?;
        self.indices.insert(value.clone(),index);self.values.push(value.clone());Ok(index)
    }
}
fn encode_values(values:&[Value],kind:StorageType,pool:&mut StringPool)->Result<Vec<u8>> {
    let mut bytes=Vec::new();
    for value in values {match (kind,value) {
        (StorageType::Bool,Value::Boolean(value))=>bytes.push(u8::from(*value)),
        (StorageType::Byte,Value::Byte(value))=>bytes.extend(value.to_le_bytes()),
        (StorageType::Short,Value::Short(value))=>bytes.extend(value.to_le_bytes()),
        (StorageType::Int,Value::Int(value))=>bytes.extend(value.to_le_bytes()),
        (StorageType::Long,Value::Long(value))=>bytes.extend(value.to_le_bytes()),
        (StorageType::Float,Value::Float(value))=>bytes.extend(value.to_le_bytes()),
        (StorageType::Double,Value::Double(value))=>bytes.extend(value.to_le_bytes()),
        (StorageType::Ref,Value::Node(value))=>bytes.extend(value.id.0.to_le_bytes()),
        (StorageType::Ref,Value::Null)=>bytes.extend(0x0000ffffffffffffu64.to_le_bytes()),
        (StorageType::String,Value::String(value))=>bytes.extend(pool.insert(value)?.to_le_bytes()),
        (StorageType::String,Value::Null)=>bytes.extend((-1i32).to_le_bytes()),
        _=>return Err(Error::Invalid(format!("unsupported property type for storage {kind:?}"))),
    }}Ok(bytes)
}
struct Writer {file:File,offset:u64,pool:StringPool}
impl Writer {
    fn bytes(&mut self,kind:StorageType,bytes:&[u8])->Result<Outline> {
        let compressed=zstd::bulk::compress(bytes,0)?;
        let outline=Outline {kind,start_offset:self.offset,compressed_length:compressed.len(),decompressed_length:bytes.len()};
        self.file.seek(SeekFrom::Start(self.offset))?;self.file.write_all(&compressed)?;self.offset+=compressed.len() as u64;Ok(outline)
    }
    fn values(&mut self,kind:StorageType,values:&[Value])->Result<Outline> {let bytes=encode_values(values,kind,&mut self.pool)?;self.bytes(kind,&bytes)}
    fn qty(&mut self,counts:&[usize])->Result<Outline> {
        let mut bytes=Vec::with_capacity((counts.len()+1)*4);
        for count in counts {bytes.extend(i32::try_from(*count).map_err(|_|Error::Invalid("quantity exceeds signed int range".into()))?.to_le_bytes());}
        bytes.extend(0i32.to_le_bytes());self.bytes(StorageType::Int,&bytes)
    }
}
pub fn write_graph(graph:&Graph,path:&Path)->Result<Counts> {
    if let Some(parent)=path.parent().filter(|parent|!parent.as_os_str().is_empty()) {std::fs::create_dir_all(parent)?;}
    let file=OpenOptions::new().create(true).read(true).write(true).open(path)?;
    let mut writer=Writer {file,offset:16,pool:StringPool::default()};
    let nodes=graph.schema.node_types.iter().enumerate().map(|(kind,node)| {
        let deleted:Vec<_>=graph.nodes[kind].iter().filter(|node|node.deleted).map(|node|node.handle.id.sequence() as usize).collect();
        NodeItem {node_label:node.name.clone(),nnodes:graph.nodes[kind].len(),deletions:(!deleted.is_empty()).then_some(deleted)}
    }).collect::<Vec<_>>();
    let mut edges=Vec::new();
    for (node_kind,edge_kind,direction) in &graph.adjacency_slots {
        let mut counts=Vec::new();let mut neighbors=Vec::new();let mut properties=Vec::new();
        for node in &graph.nodes[*node_kind as usize] {
            let found=graph.edges(node.handle,if *direction==1 {Direction::Outgoing}else{Direction::Incoming},Some(&graph.schema.edges[*edge_kind as usize].name))?;
            counts.push(found.len());
            for edge in found {neighbors.push(Value::Node(if *direction==1 {edge.target}else{edge.source}));properties.push(edge.property.clone());}
        }
        let qty=Some(writer.qty(&counts)?);let neighbor_column=Some(writer.values(StorageType::Ref,&neighbors)?);
        let property=if graph.edge_property_slots.contains(&(*node_kind,*edge_kind,*direction)) {
            let typ=match graph.edge_property_storage_types.get(&(*node_kind,*edge_kind,*direction)) {
                Some(typ)=>*typ,
                None=>{
                    let name=graph.schema.edges[*edge_kind as usize].property.as_ref().ok_or_else(||Error::Invalid("property column on propertyless edge".into()))?;
                    let definition=graph.schema.properties.iter().find(|property|&property.name==name).ok_or_else(||Error::Invalid("unknown edge property".into()))?;
                    StorageType::from_value_type(definition.value_type)?
                }
            };
            Some(writer.values(typ,&properties)?)
        } else {None};
        edges.push(EdgeItem {node_label:graph.schema.node_types[*node_kind as usize].name.clone(),edge_label:graph.schema.edges[*edge_kind as usize].name.clone(),inout:*direction,qty,neighbors:neighbor_column,property});
    }
    let mut properties=Vec::new();
    for (kind,name) in &graph.property_slots {
        let definition=&graph.fields[*kind as usize][name];let mut counts=Vec::new();let mut values=Vec::new();
        for node in &graph.nodes[*kind as usize] {
            let values_at=node.properties.get(name).map(Vec::as_slice).unwrap_or(&[]);counts.push(values_at.len());values.extend_from_slice(values_at);
        }
        let typ=graph.property_storage_types.get(&(*kind,name.clone())).copied().map(Ok).unwrap_or_else(||StorageType::from_value_type(definition.value_type))?;
        properties.push(PropertyItem {node_label:graph.schema.node_types[*kind as usize].name.clone(),property_label:name.clone(),qty:Some(writer.qty(&counts)?),property:Some(writer.values(typ,&values)?)});
    }
    let mut lengths=Vec::new();let mut pool_bytes=Vec::new();
    for string in &writer.pool.values {lengths.extend(i32::try_from(string.len()).map_err(|_|Error::Invalid("string exceeds source length range".into()))?.to_le_bytes());pool_bytes.extend(string.as_bytes());}
    let string_pool_length=writer.bytes(StorageType::Int,&lengths)?;let string_pool_bytes=writer.bytes(StorageType::Byte,&pool_bytes)?;
    let counts=Counts {node_kinds:nodes.len(),edge_kinds:edges.len(),property_kinds:properties.len()};
    let manifest=Manifest {version:0,nodes,edges,properties,string_pool_length,string_pool_bytes};
    let offset=writer.offset;
    writer.file.seek(SeekFrom::Start(0))?;writer.file.write_all(MAGIC)?;writer.file.write_all(&offset.to_le_bytes())?;
    writer.file.seek(SeekFrom::Start(offset))?;writer.file.write_all(&[b'\n';20])?;
    let json=serde_json::to_vec(&manifest)?;writer.file.write_all(&json)?;writer.file.set_len(offset+20+json.len() as u64)?;writer.file.flush()?;Ok(counts)
}
impl Graph {
    pub fn with_storage(schema:Arc<Schema>,path:&Path,persist_on_close:bool)->Result<Self> {
        if path.exists() && std::fs::metadata(path)?.len()>0 {return read_graph(schema,path,persist_on_close);}
        let mut graph=Self::new(schema)?;graph.storage_path=persist_on_close.then(||path.to_owned());Ok(graph)
    }
}
#[cgp_component(GraphStorageRewriter)]
pub trait CanRewriteGraphStorage {fn rewrite_graph_storage(&self,input:&Path,output:&Path,bootstrap:Option<&Path>)->Result<Counts>;}
#[cgp_new_provider]
impl<Context:CanCreateCpg+crate::codegen::CanLoadBootstrap> GraphStorageRewriter<Context> for RewriteNativeGraphStorage {
    fn rewrite_graph_storage(context:&Context,input:&Path,output:&Path,bootstrap:Option<&Path>)->Result<Counts> {
        let schema=match bootstrap {Some(path)=>Arc::new(context.load_bootstrap(path)?.domain_schema),None=>context.create_cpg()?.graph.schema.clone()};
        let graph=read_graph(schema,input,false)?;write_graph(&graph,output)
    }
}
