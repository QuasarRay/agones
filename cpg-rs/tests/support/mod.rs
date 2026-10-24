use cpg_rs::graph::*;
use serde_json::{json,Value as Json};
use std::path::{Path,PathBuf};

pub fn evidence(name:&str)->PathBuf {Path::new(env!("CARGO_MANIFEST_DIR")).join("../verification/agones").join(name)}
pub fn scalar(value:&Value)->Json {match value {
    Value::Null=>json!({"kind":"Null"}),Value::Boolean(value)=>json!({"kind":"Boolean","value":value}),
    Value::Byte(value)=>json!({"kind":"Byte","decimal":value.to_string()}),Value::Short(value)=>json!({"kind":"Short","decimal":value.to_string()}),
    Value::Int(value)=>json!({"kind":"Int","decimal":value.to_string()}),Value::Long(value)=>json!({"kind":"Long","decimal":value.to_string()}),
    Value::Float(value)=>json!({"kind":"Float","bits":format!("{value:08x}")}),Value::Double(value)=>json!({"kind":"Double","bits":format!("{value:016x}")}),
    Value::String(value)=>json!({"kind":"String","value":value.as_ref()}),Value::Node(value)=>json!({"kind":"Ref","id":value.id.0.to_string()}),
    other=>panic!("unaccounted storage fixture scalar {other:?}"),
}}
pub fn snapshot(graph:&Graph)->Json {
    fn edge(edge:&Edge)->Json {json!({"src":edge.source.id.0.to_string(),"dst":edge.target.id.0.to_string(),"label":edge.label,"property":scalar(&edge.property)})}
    json!({"nodes":graph.all_nodes().map(|node| {
        let properties:serde_json::Map<_,_>=node.explicit_properties().iter().filter(|(_,values)|!values.is_empty()).map(|(name,values)|(name.clone(),Json::Array(values.iter().map(scalar).collect()))).collect();
        json!({"id":node.handle.id.0.to_string(),"label":node.label,"properties":properties,
            "out":graph.edges(node.handle,Direction::Outgoing,None).unwrap().into_iter().map(edge).collect::<Vec<_>>(),
            "in":graph.edges(node.handle,Direction::Incoming,None).unwrap().into_iter().map(edge).collect::<Vec<_>>()})
    }).collect::<Vec<_>>()})
}
