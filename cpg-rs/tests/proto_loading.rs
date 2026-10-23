use cpg_rs::{Error,graph::*,proto::{self,CpgStruct,PropertyValue},proto_loading::*};
use protobuf::Message;
use serde_json::{Value as Json,json};
use std::{path::{Path,PathBuf},sync::Arc};

fn evidence(name:&str)->PathBuf {Path::new(env!("CARGO_MANIFEST_DIR")).join("../verification/agones").join(name)}
fn fixture(name:&str)->CpgStruct {CpgStruct::parse_from_bytes(&std::fs::read(evidence(&format!("first-generation-proto-reference-{name}.pb"))).unwrap()).unwrap()}
fn snapshot(graph:&Graph)->Json {
    json!({"nodes":graph.all_nodes().map(|node| {
        let properties:serde_json::Map<_,_>=node.explicit_properties().iter().filter(|(_,values)|!values.is_empty()).map(|(name,values)| (name.clone(),
            Json::Array(values.iter().map(|value|match value {
                Value::String(value)=>json!(value.as_ref()),Value::Int(value)=>json!(value),Value::Boolean(value)=>json!(value),Value::Null=>Json::Null,
                other=>panic!("unaccounted reference property {other:?}"),
            }).collect()))).collect();
        json!({"id":node.handle.id.0.to_string(),"label":node.label,"properties":properties})
    }).collect::<Vec<_>>(),"edges":graph.all_edges().iter().map(|edge|json!({"src":edge.source.id.0.to_string(),"dst":edge.target.id.0.to_string(),
        "label":edge.label,"property":match &edge.property {Value::Null=>Json::Null,Value::String(value)=>json!(value.as_ref()),other=>panic!("unexpected reference edge {other:?}")}})).collect::<Vec<_>>()})
}

#[test]
fn two_pass_import_matches_original_graphs_and_source_failures_by_fixture_identity() {
    let reference:Json=serde_json::from_slice(&std::fs::read(evidence("first-generation-proto-reference.json")).unwrap()).unwrap();
    for (name,expected) in reference["cases"].as_object().unwrap() {
        let proto=fixture(name);let mut cpg=Cpg::empty().unwrap();
        let result=CpgContext.load_proto_into_graph(cpg.graph_mut(),||Ok(vec![Ok(proto.clone())]));
        if expected.get("errorClass").is_some() {
            let error=match result {Err(error)=>error,Ok(_)=>panic!("source case {name} must fail")};
            assert_eq!(error.source_exception_class(),expected["errorClass"].as_str(),"{name}");
            let native=error.to_string();let original=expected["errorMessage"].as_str().unwrap();
            let shared=if name=="multiple-edge-properties" {"flatgraph only supports zero or one edge properties"}
                else if name=="unsupported-long" {"Error: unsupported property case: LONG_VALUE"}
                else {"node with proto node id=99 not found in graph"};
            assert!(native.contains(shared),"{name}: {native}");assert!(original.contains(shared),"{name}: {original}");
        } else {result.unwrap();assert_eq!(snapshot(cpg.graph()),*expected,"{name}");}
    }
}

#[test]
fn source_type_deduplication_does_not_invent_duplicate_id_aliases_and_requires_materialization() {
    let fixture=fixture("duplicate-types");let a=&fixture.node[0];let b=&fixture.node[1];
    let detached=DetachedNode::new("TYPE");let mut mappings=ProtoToGraphNodeMappings::default();mappings.add(a,detached.clone());
    assert!(mappings.contains(b));assert!(mappings.find_stored(b).is_err());assert!(mappings.find_stored_id(b.key).unwrap().is_none());
    let mut cpg=Cpg::empty().unwrap();let mut diff=DiffGraphBuilder::new();diff.add_node(detached);
    cpg.graph_mut().apply_diff(diff).unwrap();
    assert_eq!(mappings.find_stored(a).unwrap(),mappings.find_stored(b).unwrap());
    assert!(mappings.find_stored_id(b.key).unwrap().is_none());
    let mut calls=0;
    let mut cpg=Cpg::empty().unwrap();let report=CpgContext.load_proto_into_graph(cpg.graph_mut(),||{calls+=1;Ok(vec![Ok(fixture.clone())])}).unwrap();
    assert_eq!(calls,2);assert_eq!(report.nodes_deduplicated,1);assert_eq!(report.edges_added,1);assert_eq!(report.edges_skipped,1);
    let strings:Vec<_>=cpg.graph().all_nodes().filter_map(|node|match cpg.graph().property(node.handle,"NAME").unwrap(){Some(Value::String(value)) if value.as_ref()=="shared"=>Some(value),_=>None}).collect();
    assert_eq!(strings.len(),2);assert!(Arc::ptr_eq(&strings[0],&strings[1]));
}

#[test]
fn zip_import_reads_both_passes_and_preserves_cross_entry_node_edge_dependencies() {
    use std::io::Write;
    let data=fixture("duplicate-types");let directory=tempfile::tempdir().unwrap();let path=directory.path().join("cpg.zip");
    let mut nodes=CpgStruct::new();nodes.node=data.node;
    let mut edges=CpgStruct::new();edges.edge=data.edge;
    let file=std::fs::File::create(&path).unwrap();let mut zip=zip::ZipWriter::new(file);
    for (name,proto) in [("1_nodes",nodes),("2_edges",edges)] {
        zip.start_file(name,zip::write::SimpleFileOptions::default()).unwrap();zip.write_all(&proto.write_to_bytes().unwrap()).unwrap();
    }
    zip.finish().unwrap();
    let (cpg,report)=CpgContext.load_proto_zip_in_memory(&path).unwrap();
    assert_eq!(cpg.graph().node_count(),3);assert_eq!(report.edges_added,1);assert_eq!(report.edges_skipped,1);
}

#[test]
fn unsupported_oneofs_follow_source_errors_and_unset_unknown_values_remain_null() {
    let mut interner=cpg_rs::utilities::StrongStringInterner::default();
    let wire=PropertyValue::parse_from_bytes(&[0x98,0x06,0x07]).unwrap();assert_eq!(extract_property_value(&wire,&mut interner).unwrap(),Value::Null);
    let mut value=PropertyValue::new();value.set_long_value(1);
    assert!(extract_property_value(&value,&mut interner).unwrap_err().to_string().contains("LONG_VALUE"));
    value.set_bool_list(proto::BoolList::new());
    assert!(extract_property_value(&value,&mut interner).unwrap_err().to_string().contains("BOOL_LIST"));
    let mut cpg=Cpg::empty().unwrap();let proto=fixture("duplicate-types");let mut calls=0;
    let failure=CpgContext.load_proto_into_graph(cpg.graph_mut(),|| {calls+=1;if calls==1 {Ok(vec![Ok(proto.clone())])} else {Err(Error::Invalid("second source failure".into()))}});
    assert!(failure.is_err());assert_eq!(calls,2);assert_eq!(cpg.graph().node_count(),3);
}

#[test]
fn format_probes_retain_original_prefix_lengths_short_reads_and_io_failure_results() {
    use cpg_rs::cpg_loading::*;
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("input");
    for (bytes,proto,overflow,flat) in [(b"PKrest".as_slice(),true,false,false),(b"H:2rest".as_slice(),false,true,false),
        (b"FLT GRPHrest".as_slice(),false,false,true),(b"FL".as_slice(),false,false,false),(b"".as_slice(),false,false,false)] {
        std::fs::write(&path,bytes).unwrap();
        assert_eq!(is_proto_format(&path),proto);assert_eq!(is_overflow_db_format(&path),overflow);assert_eq!(is_flatgraph_format(&path),flat);
    }
    std::fs::write(&path,"é").unwrap();assert!(!probe_first_bytes(&path,"é"));
    assert!(!is_proto_format(&directory.path().join("absent")));
    assert_eq!(measure_and_report(||Ok(7)).unwrap(),7);
    assert!(measure_and_report(||Err::<(),_>(Error::Runtime("original computation failed".into()))).is_err());
}
