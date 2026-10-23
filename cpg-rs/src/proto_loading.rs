//! Two-pass native implementation of the handwritten ProtoCpgLoader and node mappings.
use crate::{Error,Result,archive::ZipArchive,graph::{Graph,Cpg,CanOpenCpgStorage,CanApplyCpgDiff,DetachedNode,NodeHandle,DiffGraphBuilder,EdgeProperty,Value},
    proto::{self,CpgStruct,CpgOverlay,PropertyValue},utilities::{StringInterner,StrongStringInterner}};
use cgp::prelude::*;
use protobuf::{Message,EnumFull,EnumOrUnknown};
use std::{cmp::Ordering,collections::BTreeMap,path::Path,sync::Arc};

pub fn enum_name<E:EnumFull>(value:EnumOrUnknown<E>)->String {
    match value.enum_value() {Ok(value)=>value.descriptor().name().to_owned(),Err(_)=>"UNRECOGNIZED".into()}
}
pub fn type_full_name_maybe(node:&proto::cpg_struct::Node)->Option<&str> {
    if node.type_.value()!=proto::cpg_struct::node::NodeType::TYPE as i32 {return None;}
    node.property.iter().find(|property|property.name.value()==proto::NodePropertyName::FULL_NAME as i32)
        .map(|property|property.value.as_ref().unwrap_or_else(||PropertyValue::default_instance()).string_value())
}
#[derive(Default)]
pub struct ProtoToGraphNodeMappings {
    proto_node_id_to_node:BTreeMap<i64,DetachedNode>,
    type_full_name_to_node:BTreeMap<String,DetachedNode>,
}
impl ProtoToGraphNodeMappings {
    pub fn add(&mut self,node:&proto::cpg_struct::Node,detached:DetachedNode) {
        self.proto_node_id_to_node.insert(node.key,detached.clone());
        if let Some(name)=type_full_name_maybe(node) {self.type_full_name_to_node.insert(name.into(),detached);}
    }
    pub fn find(&self,node:&proto::cpg_struct::Node)->Option<&DetachedNode> {
        self.proto_node_id_to_node.get(&node.key).or_else(||type_full_name_maybe(node).and_then(|name|self.type_full_name_to_node.get(name)))
    }
    pub fn contains(&self,node:&proto::cpg_struct::Node)->bool {self.find(node).is_some()}
    pub fn find_stored(&self,node:&proto::cpg_struct::Node)->Result<Option<NodeHandle>> {
        self.find(node).map(stored_reference).transpose()
    }
    pub fn find_stored_id(&self,key:i64)->Result<Option<NodeHandle>> {
        self.proto_node_id_to_node.get(&key).map(stored_reference).transpose()
    }
}
fn stored_reference(node:&DetachedNode)->Result<NodeHandle> {
    node.stored_ref()?.ok_or(Error::NoSuchElement("proto node has not been materialized by diff application"))
}
pub fn extract_property_value(value:&PropertyValue,interner:&mut impl StringInterner)->Result<Value> {
    use proto::property_value::Value as Wire;
    Ok(match &value.value {
        Some(Wire::IntValue(value))=>Value::Int(*value),
        Some(Wire::BoolValue(value))=>Value::Boolean(*value),
        Some(Wire::StringValue(value))=>Value::String(interner.intern(Arc::from(value.as_str()))),
        Some(Wire::StringList(value))=>Value::List(value.values.iter().map(|value|Value::String(interner.intern(Arc::from(value.as_str())))).collect()),
        None=>Value::Null,
        Some(value)=>{
            let case=match value {
                Wire::LongValue(_)=>"LONG_VALUE",Wire::FloatValue(_)=>"FLOAT_VALUE",Wire::DoubleValue(_)=>"DOUBLE_VALUE",
                Wire::BoolList(_)=>"BOOL_LIST",Wire::IntList(_)=>"INT_LIST",Wire::LongList(_)=>"LONG_LIST",
                Wire::FloatList(_)=>"FLOAT_LIST",Wire::DoubleList(_)=>"DOUBLE_LIST",Wire::ContainedRefs(_)=>"CONTAINED_REFS",
                _=>return Err(Error::Invalid("unexpected supported property dispatch".into())),
            };
            return Err(Error::Runtime(format!("Error: unsupported property case: {case}")));
        }
    })
}
pub fn extract_edge_property_value(edge:&proto::cpg_struct::Edge,interner:&mut impl StringInterner)->Result<Value> {
    match edge.property.as_slice() {
        []=>Ok(Value::Null),
        [property]=>extract_property_value(property.value.as_ref().unwrap_or_else(||PropertyValue::default_instance()),interner),
        properties=>Err(Error::IllegalArgument(format!("flatgraph only supports zero or one edge properties, but the given edge has {} properties",properties.len()))),
    }
}

#[derive(Default,Debug,Clone,Copy)]
pub struct ProtoLoadReport {
    pub raw_nodes_seen:usize,pub nodes_added:usize,pub nodes_deduplicated:usize,pub unknown_nodes_skipped:usize,
    pub properties_seen:usize,pub edges_added:usize,pub edges_skipped:usize,
}
#[cgp_component(CpgProtoGraphLoader)]
pub trait CanLoadCpgProtos {
    fn load_proto_into_graph<F,I>(&self,graph:&mut Graph,source:F)->Result<ProtoLoadReport>
    where F:FnMut()->Result<I>,I:IntoIterator<Item=Result<CpgStruct>>;
    fn load_proto_zip_in_memory(&self,path:&Path)->Result<(Cpg,ProtoLoadReport)>;
    fn load_proto_source<F,I>(&self,source:F,storage_path:Option<&Path>)->Result<(Cpg,ProtoLoadReport)>
    where F:FnMut()->Result<I>,I:IntoIterator<Item=Result<CpgStruct>>;
    fn load_proto_zip(&self,path:&Path,storage_path:Option<&Path>)->Result<(Cpg,ProtoLoadReport)>;
    fn load_proto_list(&self,protos:&[CpgStruct],storage_path:Option<&Path>)->Result<(Cpg,ProtoLoadReport)>;
}
#[cgp_new_provider]
impl<Context:CanApplyCpgDiff+CanOpenCpgStorage> CpgProtoGraphLoader<Context> for LoadCpgProtosInTwoPasses {
    fn load_proto_into_graph<F,I>(context:&Context,graph:&mut Graph,mut source:F)->Result<ProtoLoadReport>
    where F:FnMut()->Result<I>,I:IntoIterator<Item=Result<CpgStruct>> {
        let mut report=ProtoLoadReport::default();
        let mut interner=StrongStringInterner::default();
        let mut mappings=ProtoToGraphNodeMappings::default();
        let mut raw=DiffGraphBuilder::new();
        for message in source()? {
            for node in message?.node {
                report.raw_nodes_seen+=1;
                if mappings.contains(&node) {report.nodes_deduplicated+=1;continue;}
                let label=enum_name(node.type_);
                if !graph.schema().node_types.iter().any(|node|node.name==label) {
                    tracing::warn!(%label,"nodeKind not found - is this a valid proto cpg?");report.unknown_nodes_skipped+=1;continue;
                }
                let detached=DetachedNode::new(label);
                raw.add_node(detached.clone());mappings.add(&node,detached);report.nodes_added+=1;
            }
        }
        context.apply_cpg_diff(graph,raw)?;
        let mut diff=DiffGraphBuilder::new();
        for message in source()? {
            let message=message?;
            for node in &message.node {
                // Upstream's lazy gNode is never resolved for nodes without properties.
                let mut stored=None;
                for property in &node.property {
                    report.properties_seen+=1;
                    let target=match stored {
                        Some(target)=>target,
                        None=>{
                            let target=mappings.find_stored(node)?.ok_or_else(||Error::Conversion(format!("node with proto node id={} not found in graph",node.key)))?;
                            stored=Some(target);target
                        }
                    };
                    let value=extract_property_value(property.value.as_ref().unwrap_or_else(||PropertyValue::default_instance()),&mut interner)?;
                    diff.set_node_property(target,enum_name(property.name),value);
                }
            }
            for edge in &message.edge {
                match (mappings.find_stored_id(edge.src)?,mappings.find_stored_id(edge.dst)?) {
                    (Some(source),Some(target))=>{
                        report.edges_added+=1;
                        let value=extract_edge_property_value(edge,&mut interner)?;
                        let label=enum_name(edge.type_);
                        if !graph.schema().edges.iter().any(|edge|edge.name==label) {return Err(Error::Invalid(format!("unknown edge label: `{label}`")));}
                        diff.add_edge(source,target,label,EdgeProperty::Value(value));
                    }
                    _=>report.edges_skipped+=1,
                }
            }
        }
        context.apply_cpg_diff(graph,diff)?;
        tracing::debug!(nodes=graph.node_count(),edges=report.edges_added,properties=report.properties_seen,"Loaded proto graph");
        Ok(report)
    }
    fn load_proto_zip_in_memory(context:&Context,path:&Path)->Result<(Cpg,ProtoLoadReport)> {
        <Self as CpgProtoGraphLoader<Context>>::load_proto_zip(context,path,None)
    }
    fn load_proto_source<F,I>(context:&Context,source:F,storage_path:Option<&Path>)->Result<(Cpg,ProtoLoadReport)>
    where F:FnMut()->Result<I>,I:IntoIterator<Item=Result<CpgStruct>> {
        let mut cpg=context.open_cpg_storage(storage_path,true)?;
        let report=<Self as CpgProtoGraphLoader<Context>>::load_proto_into_graph(context,cpg.graph_mut(),source)?;
        Ok((cpg,report))
    }
    fn load_proto_zip(context:&Context,path:&Path,storage_path:Option<&Path>)->Result<(Cpg,ProtoLoadReport)> {
        measure_and_report(|| {
            <Self as CpgProtoGraphLoader<Context>>::load_proto_source(context,||ZipProtoIterator::open(path),storage_path)
        })
    }
    fn load_proto_list(context:&Context,protos:&[CpgStruct],storage_path:Option<&Path>)->Result<(Cpg,ProtoLoadReport)> {
        <Self as CpgProtoGraphLoader<Context>>::load_proto_source(context,||Ok(protos.iter().cloned().map(Ok)),storage_path)
    }
}

pub struct ZipProtoIterator {archive:ZipArchive,names:std::vec::IntoIter<String>}
impl ZipProtoIterator {
    pub fn open(path:&Path)->Result<Self> {
        let archive=ZipArchive::open(path)?;
        let names=archive.entries()?.into_iter();Ok(Self {archive,names})
    }
}
impl Iterator for ZipProtoIterator {
    type Item=Result<CpgStruct>;
    fn next(&mut self)->Option<Self::Item> {
        self.names.next().map(|name|self.archive.read(&name).and_then(|bytes|Ok(CpgStruct::parse_from_bytes(&bytes)?)))
    }
}

fn path_parts(path:&str)->Vec<String> {
    let stripped=path.replace('/',"");
    let mut parts:Vec<_>=stripped.split('_').map(str::to_owned).collect();
    while parts.last().is_some_and(|part|part.is_empty()) {parts.pop();}parts
}
/// Scala StringOps.toInt delegates to Integer.parseInt. Java counts UTF-16 units
/// and accepts BMP decimal digits; accumulation in the negative range retains MIN_VALUE.
pub fn parse_overlay_integer_units(units:&[u16])->Option<i32> {crate::utilities::parse_java_integer_units(units)}

pub fn compare_overlay_path(left:&str,right:&str)->Result<Ordering> {
    let a=path_parts(left);let b=path_parts(right);
    if a.len()<2 || b.len()<2 {return Ok(left.encode_utf16().cmp(right.encode_utf16()));}
    let a=parse_overlay_integer_units(&a[0].encode_utf16().collect::<Vec<_>>()).ok_or_else(||Error::NumberFormat(format!("invalid overlay integer prefix {}",a[0])))?;
    let b=parse_overlay_integer_units(&b[0].encode_utf16().collect::<Vec<_>>()).ok_or_else(||Error::NumberFormat(format!("invalid overlay integer prefix {}",b[0])))?;
    Ok(a.cmp(&b))
}
pub fn load_overlays_with<T>(path:&Path,mut parse:impl FnMut(&[u8])->Result<T>)->Result<std::vec::IntoIter<T>> {
    let mut archive=ZipArchive::open(path)?;
    let mut names=archive.entries()?;
    // A fallible stable merge sort preserves equal numeric-prefix encounter order.
    fn sorted(names:&mut [String])->Result<()> {
        if names.len()<2 {return Ok(());}
        let mid=names.len()/2;sorted(&mut names[..mid])?;sorted(&mut names[mid..])?;
        let mut merged=Vec::with_capacity(names.len());let (mut a,mut b)=(0,mid);
        while a<mid && b<names.len() {
            if compare_overlay_path(&names[a],&names[b])?!=Ordering::Greater {merged.push(names[a].clone());a+=1;}
            else {merged.push(names[b].clone());b+=1;}
        }
        merged.extend_from_slice(&names[a..mid]);merged.extend_from_slice(&names[b..]);names.clone_from_slice(&merged);Ok(())
    }
    sorted(&mut names)?;
    let mut overlays=Vec::with_capacity(names.len());
    for name in names {overlays.push(parse(&archive.read(&name)?)?);}
    archive.close()?;Ok(overlays.into_iter())
}
pub fn load_overlays(path:&Path)->Result<std::vec::IntoIter<CpgOverlay>> {
    load_overlays_with(path,|bytes|Ok(CpgOverlay::parse_from_bytes(bytes)?))
}
pub fn measure_and_report<T>(work:impl FnOnce()->Result<T>)->Result<T> {
    let start=crate::passes::wall_clock_millis();
    let result=work()?;
    tracing::info!(elapsed_ms=%(crate::passes::wall_clock_millis()-start),"CPG construction finished");
    Ok(result)
}
