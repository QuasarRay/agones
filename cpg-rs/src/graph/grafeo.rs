//! Grafeo owns graph records in memory and on disk. The borrowed CPG API uses a
//! disposable projection, rebuilt from Grafeo after a mutation; it is never a
//! second writable graph. CPG ids and exact scalar bits are explicit properties.
use super::*;
use ::grafeo::{GrafeoDB, Value as DbValue};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::OnceLock};

const META: &str = "__CpgRsDatabase";
const NODE: &str = "__CpgRsNode";
const ID: &str = "__cpg_id";
const DELETED: &str = "__cpg_deleted";
const PROPERTY: &str = "cpg:";
const FORMAT: i64 = 1;

#[derive(Default, Serialize, Deserialize)]
pub(super) struct Metadata {
    pub next_edge: u64,
    pub adjacency: Vec<(u16,u16,u8)>,
    pub edge_properties: Vec<(u16,u16,u8)>,
    pub properties: Vec<(u16,String)>,
    pub property_types: Vec<((u16,String),storage::StorageType)>,
    pub edge_property_types: Vec<((u16,u16,u8),storage::StorageType)>,
}
pub(super) struct Projection {pub nodes: Vec<Vec<StoredNode>>, pub edges: BTreeMap<u64,Edge>}
pub(super) struct Database {
    db: GrafeoDB, schema: Arc<Schema>, identity: u64, meta: ::grafeo::NodeId,
    // These indexes contain identities only, never graph properties or edges.
    ids: BTreeMap<NodeId,::grafeo::NodeId>, edge_ids: BTreeMap<u64,::grafeo::EdgeId>,
    sequences: Vec<u32>, projection: OnceLock<Projection>,
}
fn invalid(message: impl Into<String>)->Error {Error::Invalid(format!("Grafeo CPG: {}",message.into()))}
fn integer(value: Option<&DbValue>, name:&str)->Result<i64> {
    value.and_then(DbValue::as_int64).ok_or_else(||invalid(format!("missing or invalid {name}")))
}
fn boolean(value: Option<&DbValue>, name:&str)->Result<bool> {
    value.and_then(DbValue::as_bool).ok_or_else(||invalid(format!("missing or invalid {name}")))
}
fn encode(value:&Value)->DbValue {
    let (tag,payload)=match value {
        Value::Null=>(0,DbValue::Null),Value::Boolean(v)=>(1,DbValue::Bool(*v)),
        Value::Byte(v)=>(2,DbValue::Int64(i64::from(*v))),Value::Short(v)=>(3,DbValue::Int64(i64::from(*v))),
        Value::Int(v)=>(4,DbValue::Int64(i64::from(*v))),Value::Long(v)=>(5,DbValue::Int64(*v)),
        Value::Float(bits)=>(6,DbValue::Int64(i64::from(*bits))),
        Value::Double(bits)=>(7,DbValue::Int64(i64::from_le_bytes(bits.to_le_bytes()))),
        Value::Char(v)=>(8,DbValue::Int64(i64::from(*v))),Value::String(v)=>(9,DbValue::from(v.as_ref())),
        Value::Node(v)=>(10,DbValue::Int64(v.id.0 as i64)),
        Value::List(v)=>(11,DbValue::List(v.iter().map(encode).collect())),
    };
    DbValue::List(vec![DbValue::Int64(tag),payload].into())
}
fn decode(value:&DbValue,identity:u64,pool:&mut BTreeMap<String,Arc<str>>)->Result<Value> {
    let pair=value.as_list().filter(|v|v.len()==2).ok_or_else(||invalid("invalid scalar envelope"))?;
    let number=||integer(pair.get(1),"scalar integer");
    macro_rules! narrow {($ty:ty)=>{<$ty>::try_from(number()?).map_err(|_|invalid("scalar exceeds declared type"))?};}
    Ok(match pair[0].as_int64() {
        Some(0) if matches!(pair[1],DbValue::Null)=>Value::Null,
        Some(1)=>Value::Boolean(boolean(pair.get(1),"scalar boolean")?),
        Some(2)=>Value::Byte(narrow!(i8)),Some(3)=>Value::Short(narrow!(i16)),Some(4)=>Value::Int(narrow!(i32)),
        Some(5)=>Value::Long(number()?),Some(6)=>Value::Float(narrow!(u32)),
        Some(7)=>Value::Double(u64::from_le_bytes(number()?.to_le_bytes())),Some(8)=>Value::Char(narrow!(u16)),
        Some(9)=>{let text=pair[1].as_str().ok_or_else(||invalid("invalid scalar string"))?;
            Value::String(pool.entry(text.to_owned()).or_insert_with(||Arc::from(text)).clone())},
        Some(10)=>Value::Node(NodeHandle {graph:identity,id:NodeId(narrow!(u64))}),
        Some(11)=>Value::List(pair[1].as_list().ok_or_else(||invalid("invalid scalar list"))?.iter()
            .map(|v|decode(v,identity,pool)).collect::<Result<_>>()?),
        _=>return Err(invalid("unknown scalar envelope")),
    })
}
impl Database {
    pub fn new(schema:Arc<Schema>,identity:u64)->Result<Self> {
        let db=GrafeoDB::new_in_memory();
        let schema_digest=crate::content_digest(&serde_json::to_vec(&schema)?);
        let meta=db.create_node_with_props(&[META],[
            ("format",DbValue::Int64(FORMAT)),("schema",DbValue::from(schema_digest)),
            ("metadata",DbValue::from(serde_json::to_string(&Metadata::default())?)),
        ])?;
        Ok(Self {db,meta,sequences:vec![0;schema.node_types.len()],schema,identity,
            ids:BTreeMap::new(),edge_ids:BTreeMap::new(),projection:OnceLock::new()})
    }
    pub fn open(schema:Arc<Schema>,identity:u64,path:&Path)->Result<(Self,Metadata)> {
        // Read-only opening avoids modifying an imported checkpoint, including
        // when persist_on_close is false. All mutations use Grafeo's memory DB.
        let source=GrafeoDB::open_read_only(path)?;
        let db=source.to_memory()?;source.close()?;
        let meta_nodes:Vec<_>=db.iter_nodes().filter(|n|n.labels.iter().any(|v|v.as_str()==META)).collect();
        let [meta]=meta_nodes.as_slice() else {return Err(invalid("expected exactly one CPG metadata node"));};
        if integer(meta.get_property("format"),"format")?!=FORMAT {return Err(invalid("unsupported CPG database version"));}
        let schema_digest=crate::content_digest(&serde_json::to_vec(&schema)?);
        if meta.get_property("schema").and_then(DbValue::as_str)!=Some(schema_digest.as_str()) {return Err(invalid("schema identity differs from the database"));}
        let metadata:Metadata=serde_json::from_str(meta.get_property("metadata").and_then(DbValue::as_str)
            .ok_or_else(||invalid("missing storage metadata"))?)?;
        let meta_id=meta.id;
        let mut this=Self {db,meta:meta_id,sequences:vec![0;schema.node_types.len()],schema,identity,
            ids:BTreeMap::new(),edge_ids:BTreeMap::new(),projection:OnceLock::new()};
        for node in this.db.iter_nodes().filter(|n|n.id!=meta_id) {
            if !node.labels.iter().any(|v|v.as_str()==NODE) {return Err(invalid("foreign node in CPG database"));}
            let id=NodeId(u64::try_from(integer(node.get_property(ID),ID)?).map_err(|_|invalid("negative CPG id"))?);
            let kind=id.kind() as usize;
            if kind>=this.sequences.len() || this.ids.insert(id,node.id).is_some() {return Err(invalid("duplicate or invalid CPG node id"));}
            this.sequences[kind]=this.sequences[kind].max(id.sequence().checked_add(1).ok_or_else(||invalid("node sequence overflow"))?);
        }
        for edge in this.db.iter_edges() {
            let id=u64::try_from(integer(edge.get_property(ID),ID)?).map_err(|_|invalid("negative CPG edge id"))?;
            if id>=metadata.next_edge || this.edge_ids.insert(id,edge.id).is_some() {return Err(invalid("duplicate or invalid CPG edge id"));}
        }
        for (kind,count) in this.sequences.iter().enumerate() {
            if (0..*count).any(|seq|!this.ids.contains_key(&NodeId::new(kind as u16,seq))) {return Err(invalid("non-contiguous CPG node slots"));}
        }
        this.projection.set(this.build_projection()?).map_err(|_|invalid("projection already initialized"))?;
        Ok((this,metadata))
    }
    fn build_projection(&self)->Result<Projection> {
        let mut nodes:Vec<Vec<StoredNode>>=self.schema.node_types.iter().map(|_|Vec::new()).collect();
        let mut pool=BTreeMap::new();
        for (id,key) in &self.ids {
            let node=self.db.get_node(*key).ok_or_else(||invalid("node index points to absent record"))?;
            let label=&self.schema.node_types[id.kind() as usize].name;
            if !node.labels.iter().any(|v|v.as_str()==label) {return Err(invalid("CPG label and id disagree"));}
            let mut properties=BTreeMap::new();
            for (key,value) in &node.properties {
                if let Some(name)=key.as_str().strip_prefix(PROPERTY) {
                    let values=value.as_list().ok_or_else(||invalid("node property is not a list"))?.iter()
                        .map(|v|decode(v,self.identity,&mut pool)).collect::<Result<Vec<_>>>()?;
                    properties.insert(name.to_owned(),values);
                }
            }
            nodes[id.kind() as usize].push(StoredNode {handle:NodeHandle {graph:self.identity,id:*id},label:label.clone(),
                properties,deleted:boolean(node.get_property(DELETED),DELETED)?});
        }
        let reverse:BTreeMap<_,_>=self.ids.iter().map(|(id,key)|(*key,*id)).collect();
        let mut edges=BTreeMap::new();
        for (id,key) in &self.edge_ids {
            let edge=self.db.get_edge(*key).ok_or_else(||invalid("edge index points to absent record"))?;
            let kind=u16::try_from(integer(edge.get_property("__cpg_kind"),"edge kind")?).map_err(|_|invalid("invalid edge kind"))?;
            let definition=self.schema.edges.get(kind as usize).ok_or_else(||invalid("unknown edge kind"))?;
            if edge.edge_type.as_str()!=definition.name {return Err(invalid("CPG edge type and kind disagree"));}
            let node_handle=|key|reverse.get(&key).copied().map(|id|NodeHandle {graph:self.identity,id}).ok_or_else(||invalid("edge endpoint absent"));
            let order=|key|->Result<u64>{Ok(u64::from_le_bytes(integer(edge.get_property(key),key)?.to_le_bytes()))};
            edges.insert(*id,Edge {handle:EdgeHandle {graph:self.identity,id:*id},source:node_handle(edge.src)?,target:node_handle(edge.dst)?,kind,
                label:definition.name.clone(),property:decode(edge.get_property("cpg:value").ok_or_else(||invalid("edge value absent"))?,self.identity,&mut pool)?,
                outgoing:boolean(edge.get_property("__cpg_out"),"edge outgoing")?,incoming:boolean(edge.get_property("__cpg_in"),"edge incoming")?,
                out_order:order("__cpg_out_order")?,in_order:order("__cpg_in_order")?});
        }
        Ok(Projection {nodes,edges})
    }
    pub fn view(&self)->&Projection {self.projection.get_or_init(||self.build_projection().expect("private Grafeo records retain validated CPG invariants"))}
    pub fn contains_node(&self,id:NodeId)->bool {self.ids.contains_key(&id)}
    pub fn is_live(&self,id:NodeId)->bool {self.ids.get(&id).and_then(|key|self.db.get_node(*key))
        .is_some_and(|n|n.get_property(DELETED).and_then(DbValue::as_bool)==Some(false))}
    pub fn next_sequence(&self,kind:u16)->u32 {self.sequences[kind as usize]}
    pub fn insert_node(&mut self,node:StoredNode)->Result<()> {
        let id=node.handle.id;
        if self.ids.contains_key(&id) || id.sequence()!=self.sequences[id.kind() as usize] {return Err(invalid("node slots must be appended exactly once"));}
        let mut properties=vec![(ID.to_owned(),DbValue::Int64(id.0 as i64)),(DELETED.to_owned(),DbValue::Bool(node.deleted))];
        for (name,values) in node.properties {properties.push((format!("{PROPERTY}{name}"),DbValue::List(values.iter().map(encode).collect())));}
        let key=self.db.create_node_with_props(&[NODE,&node.label],properties)?;
        self.ids.insert(id,key);self.sequences[id.kind() as usize]=id.sequence().checked_add(1).ok_or_else(||invalid("node sequence overflow"))?;
        self.projection.take();Ok(())
    }
    pub fn set_property(&mut self,node:NodeId,name:&str,values:&[Value])->Result<()> {
        self.db.set_node_property(self.ids[&node],&format!("{PROPERTY}{name}"),DbValue::List(values.iter().map(encode).collect()))?;
        self.projection.take();Ok(())
    }
    pub fn remove_node(&mut self,id:NodeId)->Result<bool> {
        if !self.is_live(id) {return Ok(false);}
        let key=self.ids[&id];let node=self.db.get_node(key).ok_or_else(||invalid("missing node"))?;
        self.db.set_node_property(key,DELETED,DbValue::Bool(true))?;
        for (name,_) in &node.properties {if name.as_str().starts_with(PROPERTY) {self.db.remove_node_property(key,name.as_str())?;}}
        self.projection.take();Ok(true)
    }
    pub fn remove_incident_edges(&mut self)->Result<()> {
        let deleted:BTreeSet<_>=self.ids.iter().filter(|(id,_)|!self.is_live(**id)).map(|(_,key)|*key).collect();
        let remove:Vec<_>=self.edge_ids.iter().filter_map(|(id,key)|self.db.get_edge(*key)
            .filter(|e|deleted.contains(&e.src)||deleted.contains(&e.dst)).map(|_|*id)).collect();
        for id in remove {self.remove_edge(id)?;}Ok(())
    }
    pub fn insert_edge(&mut self,edge:Edge)->Result<()> {
        if self.edge_ids.contains_key(&edge.handle.id) {return Err(invalid("duplicate edge identity"));}
        let key=self.db.create_edge_with_props(self.ids[&edge.source.id],self.ids[&edge.target.id],&edge.label,[
            (ID,DbValue::Int64(edge.handle.id as i64)),("__cpg_kind",DbValue::Int64(i64::from(edge.kind))),
            ("__cpg_out",DbValue::Bool(edge.outgoing)),("__cpg_in",DbValue::Bool(edge.incoming)),
            ("__cpg_out_order",DbValue::Int64(edge.out_order as i64)),("__cpg_in_order",DbValue::Int64(edge.in_order as i64)),
            ("cpg:value",encode(&edge.property)),
        ])?;
        self.edge_ids.insert(edge.handle.id,key);self.projection.take();Ok(())
    }
    pub fn contains_edge(&self,id:u64)->bool {self.edge_ids.contains_key(&id)}
    pub fn edge_info(&self,id:u64)->Result<(u16,NodeId,NodeId,bool,bool)> {
        let edge=self.db.get_edge(self.edge_ids[&id]).ok_or_else(||invalid("missing edge"))?;
        let node_id=|key|self.db.get_node(key).ok_or_else(||invalid("missing endpoint")).and_then(|n|
            Ok(NodeId(u64::try_from(integer(n.get_property(ID),ID)?).map_err(|_|invalid("negative endpoint id"))?)));
        Ok((u16::try_from(integer(edge.get_property("__cpg_kind"),"edge kind")?).map_err(|_|invalid("invalid edge kind"))?,
            node_id(edge.src)?,node_id(edge.dst)?,boolean(edge.get_property("__cpg_out"),"outgoing")?,boolean(edge.get_property("__cpg_in"),"incoming")?))
    }
    pub fn set_edge_property(&mut self,id:u64,value:&Value)->Result<()> {
        self.db.set_edge_property(self.edge_ids[&id],"cpg:value",encode(value))?;self.projection.take();Ok(())
    }
    pub fn remove_edge(&mut self,id:u64)->Result<()> {
        if let Some(key)=self.edge_ids.get(&id).copied() {self.db.delete_edge(key)?;self.edge_ids.remove(&id);self.projection.take();}Ok(())
    }
    pub fn save(&self,path:&Path,metadata:&Metadata)->Result<()> {
        self.db.set_node_property(self.meta,"metadata",DbValue::from(serde_json::to_string(metadata)?))?;
        let parent=path.parent().filter(|p|!p.as_os_str().is_empty()).unwrap_or_else(||Path::new("."));
        std::fs::create_dir_all(parent)?;
        let temporary=tempfile::Builder::new().prefix(".cpg-rs-").tempdir_in(parent)?;
        let checkpoint=temporary.path().join("checkpoint.grafeo");
        self.db.save(&checkpoint)?;
        std::fs::File::open(&checkpoint)?.sync_all()?;
        std::fs::rename(&checkpoint,path)?;
        std::fs::File::open(parent)?.sync_all()?;
        Ok(())
    }
    pub fn query(&self,query:&str)->Result<::grafeo::QueryResult> {
        Ok(self.db.session_with_role(::grafeo::Role::ReadOnly).execute(query)?)
    }
}
