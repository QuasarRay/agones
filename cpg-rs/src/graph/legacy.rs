//! Native OverflowDB v2 import through its pinned H2 maps and MessagePack records.
//! The source Flatgraph converter's value flattening, column homogenization and
//! independent adjacency sides are retained before installing Grafeo records.
use super::*;
use rmpv::Value as Message;
use std::{io::Cursor,path::Path};

#[derive(Clone,Debug)]
enum LegacyValue {Scalar(Value),Ref(i64)}
struct LegacyEdge {label:String,neighbor:i64,property:Option<LegacyValue>}
struct LegacyNode {id:i64,label:String,properties:BTreeMap<String,Vec<LegacyValue>>,out:Vec<LegacyEdge>,incoming:Vec<LegacyEdge>}
fn type_error(expected:&str)->Error {Error::MessageTypeCast(format!("expected MessagePack {expected}"))}
fn integer(value:&Message)->Result<i64>{
    if !matches!(value,Message::Integer(_)) {return Err(type_error("integer"));}
    value.as_i64().ok_or_else(||Error::MessageIntegerOverflow("MessagePack integer exceeds signed long range".into()))
}
fn narrow<T:TryFrom<i64>>(value:&Message)->Result<T> {T::try_from(integer(value)?).map_err(|_|Error::MessageIntegerOverflow("MessagePack integer exceeds target range".into()))}
fn array(value:&Message)->Result<&[Message]>{value.as_array().map(Vec::as_slice).ok_or_else(||type_error("array"))}
fn read(input:&mut Cursor<&[u8]>)->Result<Message> {rmpv::decode::read_value(input).map_err(|error|Error::Conversion(format!("invalid OverflowDB MessagePack: {error}")))}
fn count(input:&mut Cursor<&[u8]>)->Result<usize> {let value=read(input)?;let count:i32=narrow(&value)?;
    // Scala Range(0, negative) is empty.
    Ok(count.max(0) as usize)}
fn text(dictionary:&BTreeMap<i32,String>,value:&Message)->Result<String> {
    let id:i32=narrow(value)?;dictionary.get(&id).cloned().ok_or_else(||Error::Conversion(format!("missing OverflowDB string mapping {id}")))
}
fn values(value:&Message,typ:Option<i8>,result:&mut Vec<LegacyValue>)->Result<()> {
    let (typ,payload)=match typ {Some(typ)=>(typ,value),None=>{
        let pair=array(value)?;let (Some(tag),Some(payload))=(pair.first(),pair.get(1)) else {return Err(type_error("type/value pair"));};
        (narrow::<i8>(tag)?,payload)
    }};
    let scalar=match typ {
        0=>Value::Boolean(payload.as_bool().ok_or_else(||type_error("boolean"))?),
        1=>Value::string(payload.as_str().ok_or_else(||type_error("UTF-8 string"))?),
        2=>Value::Byte(narrow(payload)?),3=>Value::Short(narrow(payload)?),4=>Value::Int(narrow(payload)?),5=>Value::Long(integer(payload)?),
        6=>Value::Float(match payload {Message::F32(v)=>v.to_bits(),Message::F64(v)=>(*v as f32).to_bits(),_=>return Err(type_error("float"))}),
        7=>Value::Double(match payload {Message::F32(v)=>f64::from(*v).to_bits(),Message::F64(v)=>v.to_bits(),_=>return Err(type_error("float"))}),
        9=>{result.push(LegacyValue::Ref(integer(payload)?));return Ok(());},
        10=>Value::Null,
        8|20=>{for value in array(payload)? {array(value)?;values(value,None,result)?;}return Ok(());},
        12..=17|19=>{
            let element=match typ {12=>2,13=>3,14=>4,15=>5,16=>6,17=>7,19=>0,_=>unreachable!()};
            for value in array(payload)? {
                // The pinned converter calls asArrayValue even for primitive
                // elements, then attempts a scalar cast. Preserve that failure.
                array(value)?;values(value,Some(element),result)?;
            }
            return Ok(());
        },
        11|18=>return Err(Error::SourceNotImplemented("Convert.unpackValue character/character array")),
        _=>return Err(Error::IllegalArgument(format!("unknown id type {typ}"))),
    };
    result.push(LegacyValue::Scalar(scalar));Ok(())
}
fn read_node(id:i64,bytes:&[u8],dictionary:&BTreeMap<i32,String>)->Result<LegacyNode> {
    let mut input=Cursor::new(bytes);let stored=integer(&read(&mut input)?)?;
    if stored!=id {return Err(Error::SourceAssertion("OverflowDB key and encoded node id differ".into()));}
    let label=text(dictionary,&read(&mut input)?)?;let properties=read(&mut input)?;
    let properties=properties.as_map().ok_or_else(||type_error("property map"))?;
    let mut result=LegacyNode {id,label,properties:BTreeMap::new(),out:Vec::new(),incoming:Vec::new()};
    for (key,value) in properties {values(value,None,result.properties.entry(text(dictionary,key)?).or_default())?;}
    for outgoing in [true,false] {
        let kinds=count(&mut input)?;
        for _ in 0..kinds {
            let label=text(dictionary,&read(&mut input)?)?;let edges=count(&mut input)?;
            for _ in 0..edges {
                let neighbor=integer(&read(&mut input)?)?;let properties=read(&mut input)?;
                let properties=properties.as_map().ok_or_else(||type_error("edge property map"))?;
                let property=match properties.as_slice() {
                    []=>None,[(key,value)]=>{text(dictionary,key)?;let mut out=Vec::new();values(value,None,&mut out)?;
                        match out.as_slice() {[]=>None,[v]=>Some(v.clone()),_=>return Err(Error::SourceNotImplemented("multiple values in legacy edge property"))}},
                    _=>return Err(Error::SourceNotImplemented("multiple legacy edge properties")),
                };
                let edge=LegacyEdge {label:label.clone(),neighbor,property};
                if outgoing {result.out.push(edge);}else{result.incoming.push(edge);}
            }
        }
    }
    Ok(result)
}
fn storage_type(value:&LegacyValue)->Option<storage::StorageType>{
    use storage::StorageType as T;
    match value {LegacyValue::Ref(_)=>Some(T::Ref),LegacyValue::Scalar(value)=>Some(match value {
        Value::Null=>return None,Value::Boolean(_)=>T::Bool,Value::Byte(_)=>T::Byte,Value::Short(_)=>T::Short,Value::Int(_)=>T::Int,
        Value::Long(_)=>T::Long,Value::Float(_)=>T::Float,Value::Double(_)=>T::Double,Value::String(_)=>T::String,
        _=>return None,
    })}
}
fn homogenize(values:&[LegacyValue],references:&BTreeMap<i64,Option<NodeHandle>>)->Result<Option<(storage::StorageType,Vec<Value>)>> {
    use storage::StorageType as T;
    let Some(typ)=values.iter().find_map(storage_type) else {return Ok(None);};
    let mut converted=Vec::with_capacity(values.len());
    for value in values {
        let value=match value {
            LegacyValue::Ref(id) if typ==T::Ref=>references.get(id).copied().flatten().map(Value::Node).unwrap_or(Value::Null),
            LegacyValue::Scalar(Value::Null)=>match typ {T::Bool=>Value::Boolean(false),T::Byte=>Value::Byte(0),T::Short=>Value::Short(0),
                T::Int=>Value::Int(0),T::Long=>Value::Long(0),T::Float=>Value::Float(0),T::Double=>Value::Double(0),_=>Value::Null},
            value if storage_type(value)!=Some(typ)=>return Err(Error::ClassCast("legacy column contains incompatible runtime types".into())),
            LegacyValue::Scalar(value)=>value.clone(),
            _=>return Err(Error::ClassCast("legacy reference column contains scalar values".into())),
        };
        converted.push(value);
    }
    Ok(Some((typ,converted)))
}

/// Converts the original OverflowDB node/message representation into the current
/// schema and Grafeo. The input file is opened read-only and never delegated to JVM code.
pub fn read_graph(schema:Arc<Schema>,path:&Path)->Result<Graph> {
    let store=super::mvstore::Store::open(path)?;
    let metadata=store.map("metadata")?.into_iter().map(|(k,v)|Ok((k.string()?,v.string()?))).collect::<Result<BTreeMap<_,_>>>()?;
    let parse_metadata_integer=|value:&str|crate::utilities::parse_java_integer_units(&value.encode_utf16().collect::<Vec<_>>())
        .ok_or_else(||Error::NumberFormat(format!("invalid storage metadata integer {value}")));
    // OdbStorage initializes the current library-version id before checking
    // storage compatibility, and initializes string ids only after that check.
    if let Some(value)=metadata.get("LIBRARY_VERSIONS_MAX_ID") {parse_metadata_integer(value)?;}
    let version=metadata.get("STORAGE_FORMAT_VERSION").ok_or_else(||Error::BackwardsCompatibility("storage metadata does not contain version number, this must be an old format.".into()))?;
    let version=crate::utilities::parse_java_integer_units(&version.encode_utf16().collect::<Vec<_>>()).ok_or_else(||Error::NumberFormat(format!("invalid storage version {version}")))?;
    if version!=2 {return Err(Error::BackwardsCompatibility(format!("attempting to open storage with different version: {version}; this version of overflowdb requires the version to be exactly 2")));}
    if let Some(value)=metadata.get("STRING_TO_INT_MAX_ID") {parse_metadata_integer(value)?;}
    let dictionary=store.map("stringToIntMappings")?.into_iter().map(|(k,v)|{
        let id=i32::try_from(v.number()?).map_err(|_|Error::ClassCast("invalid OverflowDB string id".into()))?;Ok((id,k.string()?))
    }).collect::<Result<BTreeMap<_,_>>>()?;
    let mut rows=store.map("nodes")?.into_iter().map(|(k,v)|Ok((k.number()?,v.bytes()?))).collect::<Result<Vec<_>>>()?;
    rows.sort_by_key(|row|row.0);
    let raw=rows.iter().map(|(id,bytes)|read_node(*id,bytes,&dictionary)).collect::<Result<Vec<_>>>()?;
    let mut graph=Graph::new(schema)?;let mut nodes:Vec<Vec<StoredNode>>=graph.schema.node_types.iter().map(|_|Vec::new()).collect();
    let mut references=BTreeMap::new();
    for node in &raw {
        let handle=if let Some(kind)=graph.kinds.get(&node.label).copied() {
            let sequence=u32::try_from(nodes[kind as usize].len()).map_err(|_|Error::Invalid("legacy node sequence space exhausted".into()))?;
            let handle=NodeHandle {graph:graph.identity,id:NodeId::new(kind,sequence)};
            nodes[kind as usize].push(StoredNode {handle,label:node.label.clone(),properties:BTreeMap::new(),deleted:false});Some(handle)
        }else{None};references.insert(node.id,handle);
    }
    let mut properties:BTreeMap<(String,String),Vec<(usize,&[LegacyValue])>>=BTreeMap::new();
    let mut adjacencies:BTreeMap<(String,String,u8),Vec<(usize,&LegacyEdge)>>=BTreeMap::new();
    for (index,node) in raw.iter().enumerate() {
        for (name,values) in &node.properties {properties.entry((node.label.clone(),name.clone())).or_default().push((index,values));}
        for (direction,edges) in [(1,&node.out),(0,&node.incoming)] {for edge in edges {
            adjacencies.entry((node.label.clone(),edge.label.clone(),direction)).or_default().push((index,edge));
        }}
    }
    for ((label,name),rows) in properties {
        let combined=rows.iter().flat_map(|(_,values)|values.iter().cloned()).collect::<Vec<_>>();
        let Some((typ,values))=homogenize(&combined,&references)? else {continue;};
        let Some(kind)=graph.kinds.get(&label).copied() else {continue;};
        if !graph.fields[kind as usize].contains_key(&name) {continue;}
        graph.property_slots.insert((kind,name.clone()));graph.property_storage_types.insert((kind,name.clone()),typ);
        let mut offset=0;for (index,raw_values) in rows {
            let handle=references[&raw[index].id].ok_or_else(||Error::Conversion("legacy node missing from target schema".into()))?;
            let end=offset+raw_values.len();if end>offset {nodes[kind as usize][handle.id.sequence() as usize].properties.insert(name.clone(),values[offset..end].to_vec());}offset=end;
        }
    }
    let mut halves=Vec::new();
    for ((label,edge_label,direction),rows) in adjacencies {
        let combined=rows.iter().map(|(_,edge)|edge.property.clone().unwrap_or(LegacyValue::Scalar(Value::Null))).collect::<Vec<_>>();
        let property=homogenize(&combined,&references)?;
        let (Some(node_kind),Some(edge_kind))=(graph.kinds.get(&label).copied(),graph.edge_kinds.get(&edge_label).copied()) else {continue;};
        graph.adjacency_slots.insert((node_kind,edge_kind,direction));
        if let Some((typ,_))=&property {graph.edge_property_slots.insert((node_kind,edge_kind,direction));graph.edge_property_storage_types.insert((node_kind,edge_kind,direction),*typ);}
        let mut orders=BTreeMap::new();
        for (position,(index,edge)) in rows.into_iter().enumerate() {
            let base=references[&raw[index].id].ok_or_else(||Error::Conversion("legacy node missing from target schema".into()))?;
            let neighbor=references.get(&edge.neighbor).copied().flatten().ok_or_else(||Error::Deserialization("null/missing graph neighbor".into()))?;
            let order=orders.entry(base).or_insert(0u64);
            let value=match &property {Some((_,values))=>values[position].clone(),None=>graph.edge_property(edge_kind,EdgeProperty::Default)?};
            let (source,target)=if direction==1 {(base,neighbor)}else{(neighbor,base)};
            halves.push((source,target,edge_kind,direction,*order,value));*order+=1;
        }
    }
    storage::install_columns(&mut graph,nodes,halves)?;graph.changed=true;Ok(graph)
}

#[cgp_component(LegacyCpgImporter)]
pub trait CanImportLegacyCpg {fn import_legacy_cpg(&self,input:&Path,persist_to:&Path)->Result<Cpg>;}
#[cgp_new_provider]
impl<Context:CanCreateCpg> LegacyCpgImporter<Context> for ImportLegacyCpgIntoGrafeo {
    fn import_legacy_cpg(context:&Context,input:&Path,persist_to:&Path)->Result<Cpg> {
        let schema=context.create_cpg()?.graph.schema.clone();let mut graph=read_graph(schema,input)?;
        graph.save_database(persist_to)?;graph.storage_path=Some(persist_to.to_owned());graph.changed=false;Ok(Cpg::from_graph(graph))
    }
}
