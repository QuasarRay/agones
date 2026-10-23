use cpg_rs::{Error,Result,graph::*,schema::{Bootstrap,Schema,SchemaBuilder,ValueType}};
use serde_json::{json,Value as Json};
use std::{path::{Path,PathBuf},sync::Arc};

fn evidence(name:&str)->PathBuf {Path::new(env!("CARGO_MANIFEST_DIR")).join("../verification/agones").join(name)}
fn scalar(value:&Value)->Json {match value {
    Value::Null=>json!({"kind":"Null"}),Value::Boolean(value)=>json!({"kind":"Boolean","value":value}),
    Value::Byte(value)=>json!({"kind":"Byte","decimal":value.to_string()}),Value::Short(value)=>json!({"kind":"Short","decimal":value.to_string()}),
    Value::Int(value)=>json!({"kind":"Int","decimal":value.to_string()}),Value::Long(value)=>json!({"kind":"Long","decimal":value.to_string()}),
    Value::Float(value)=>json!({"kind":"Float","bits":format!("{value:08x}")}),Value::Double(value)=>json!({"kind":"Double","bits":format!("{value:016x}")}),
    Value::String(value)=>json!({"kind":"String","value":value.as_ref()}),Value::Node(value)=>json!({"kind":"Ref","id":value.id.0.to_string()}),
    other=>panic!("unaccounted storage fixture scalar {other:?}"),
}}
fn snapshot(graph:&Graph)->Json {
    fn edge(edge:&Edge)->Json {json!({"src":edge.source.id.0.to_string(),"dst":edge.target.id.0.to_string(),"label":edge.label,"property":scalar(&edge.property)})}
    json!({"nodes":graph.all_nodes().map(|node| {
        let properties:serde_json::Map<_,_>=node.explicit_properties().iter().filter(|(_,values)|!values.is_empty()).map(|(name,values)|(name.clone(),Json::Array(values.iter().map(scalar).collect()))).collect();
        json!({"id":node.handle.id.0.to_string(),"label":node.label,"properties":properties,
            "out":graph.edges(node.handle,Direction::Outgoing,None).unwrap().into_iter().map(edge).collect::<Vec<_>>(),
            "in":graph.edges(node.handle,Direction::Incoming,None).unwrap().into_iter().map(edge).collect::<Vec<_>>()})
    }).collect::<Vec<_>>()})
}
fn primitive_schema()->Schema {
    let mut builder=SchemaBuilder::new("Primitive".into(),"primitive".into(),vec![]);
    for (name,kind) in [("BOOL",ValueType::Boolean),("BYTE",ValueType::Byte),("SHORT",ValueType::Short),("INT",ValueType::Int),
        ("LONG",ValueType::Long),("FLOAT",ValueType::Float),("DOUBLE",ValueType::Double),("STR",ValueType::String),("REF",ValueType::NodeRef)] {
        builder.add_property(name.into(),kind,"",None).as_list();
    }
    let node=builder.add_node_type("N".into(),"",None);
    for name in ["BOOL","BYTE","SHORT","INT","LONG","FLOAT","DOUBLE","STR","REF"] {node.add_property(name.into());}
    builder.add_edge_type("X".into(),"",None).property=Some("DOUBLE".into());
    builder.build().unwrap()
}
fn reference()->Json {serde_json::from_slice(&std::fs::read(evidence("first-generation-flatgraph-reference.json")).expect("pinned storage observation must exist")).unwrap()}

#[test]
fn original_storage_reopens_with_independent_edge_orders_deletions_refs_and_defaults() {
    let expected=reference();let mut cpg=Cpg::with_storage_options(evidence("first-generation-flatgraph-reference.fg"),false).unwrap();
    assert_eq!(snapshot(cpg.graph()),expected["cpg"]);assert_eq!(expected["cpg"],expected["cpgReopened"]);
    let method=cpg.graph().nodes("METHOD").next().unwrap().handle;
    assert_eq!(cpg.graph().property(method,"CODE").unwrap(),Some(Value::string("<empty>")));
    assert_eq!(cpg.graph().nodes("LOCAL").count(),0);
    let first=match cpg.graph().property(method,"NAME").unwrap().unwrap(){Value::String(value)=>value,_=>panic!()};
    let second=cpg.graph().nodes("METHOD").nth(1).unwrap().handle;
    let second=match cpg.graph().property(second,"NAME").unwrap().unwrap(){Value::String(value)=>value,_=>panic!()};
    assert!(Arc::ptr_eq(&first,&second));
    cpg.close().unwrap();
}

#[test]
fn primitive_storage_retains_all_scalar_bits_nulls_and_cross_node_references() {
    let expected=reference();let schema=Arc::new(primitive_schema());
    let mut graph=Graph::with_storage(schema,evidence("first-generation-flatgraph-primitives.fg").as_path(),false).unwrap();
    assert_eq!(snapshot(&graph),expected["primitive"]);assert_eq!(expected["primitive"],expected["primitiveReopened"]);
    let node=graph.all_nodes().next().unwrap().handle;
    assert_eq!(graph.property_values(node,"LONG").unwrap(),[Value::Long(i64::MIN),Value::Long(i64::MAX)]);
    assert_eq!(graph.property_values(node,"FLOAT").unwrap(),[Value::Float(0x7fc01234),Value::Float((-0.0f32).to_bits())]);
    assert_eq!(graph.property_values(node,"DOUBLE").unwrap(),[Value::Double(0x7ff8000000001234),Value::Double((-0.0f64).to_bits())]);
    graph.close().unwrap();
}

#[test]
fn native_writer_roundtrip_preserves_physical_columns_and_original_query_results() {
    let expected=reference();let directory=tempfile::tempdir().unwrap();let path=directory.path().join("nested/roundtrip.fg");
    let cpg=Cpg::with_storage_options(evidence("first-generation-flatgraph-reference.fg"),false).unwrap();
    let counts=storage::write_graph(cpg.graph(),&path).unwrap();
    assert_eq!(counts.node_kinds,expected["counts"]["nodeKinds"].as_u64().unwrap() as usize);
    assert_eq!(counts.edge_kinds,expected["counts"]["edgeKinds"].as_u64().unwrap() as usize);
    assert_eq!(counts.property_kinds,expected["counts"]["propertyKinds"].as_u64().unwrap() as usize);
    let cpg=Cpg::with_storage_options(&path,false).unwrap();assert_eq!(snapshot(cpg.graph()),expected["cpg"]);
    let primitive=Graph::with_storage(Arc::new(primitive_schema()),&evidence("first-generation-flatgraph-primitives.fg"),false).unwrap();
    let primitive_path=directory.path().join("primitive.fg");storage::write_graph(&primitive,&primitive_path).unwrap();
    let primitive=Graph::with_storage(Arc::new(primitive_schema()),&primitive_path,false).unwrap();assert_eq!(snapshot(&primitive),expected["primitive"]);
}

#[test]
fn storage_lifecycle_defers_creation_persists_changes_and_obeys_read_only_configuration() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("new/graph.fg");
    let mut cpg=Cpg::with_storage(&path).unwrap();assert!(!path.exists());cpg.close().unwrap();assert!(!path.exists());
    let mut cpg=Cpg::with_storage(&path).unwrap();let node=DetachedNode::new("METHOD");node.set("NAME",Value::string("persisted")).unwrap();
    let mut diff=DiffGraphBuilder::new();diff.add_node(node);CpgContext.apply_cpg_diff(cpg.graph_mut(),diff).unwrap();
    assert!(!path.exists());cpg.close().unwrap();assert!(path.exists());
    let original=std::fs::read(&path).unwrap();
    let mut read_only=Cpg::with_storage_options(&path,false).unwrap();let node=read_only.graph().all_nodes().next().unwrap().handle;
    let mut diff=DiffGraphBuilder::new();diff.set_node_property(node,"NAME",Value::string("memory-only"));read_only.graph_mut().apply_diff(diff).unwrap();read_only.close().unwrap();
    assert_eq!(std::fs::read(&path).unwrap(),original);
    assert!(matches!(read_only.graph_mut().apply_diff(DiffGraphBuilder::new()),Err(Error::GraphClosed)));
    let reopened=Cpg::with_storage_options(&path,false).unwrap();assert_eq!(reopened.graph().nodes_with_property(None,"NAME","persisted").count(),1);
}

#[test]
fn protobuf_import_can_reopen_and_extend_a_real_persistent_graph() {
    use cpg_rs::proto_loading::CanLoadCpgProtos;
    use protobuf::Message;
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("import.fg");
    let fixture=cpg_rs::proto::CpgStruct::parse_from_bytes(&std::fs::read(evidence("first-generation-proto-reference-duplicate-types.pb")).unwrap()).unwrap();
    let (mut cpg,report)=CpgContext.load_proto_list(&[fixture.clone()],Some(&path)).unwrap();assert_eq!(report.nodes_added,3);assert!(!path.exists());
    cpg.close().unwrap();assert!(path.exists());
    let (mut cpg,report)=CpgContext.load_proto_source(||Ok(vec![Ok(fixture.clone())]),Some(&path)).unwrap();
    // Mapping is local to one source import; reopening doesn't silently deduplicate against existing graph state.
    assert_eq!(report.nodes_added,3);assert_eq!(cpg.graph().node_count(),6);cpg.close().unwrap();
    assert_eq!(Cpg::with_storage_options(&path,false).unwrap().graph().node_count(),6);
}

#[test]
fn corrupt_storage_headers_and_offsets_fail_instead_of_returning_empty_graphs() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("bad.fg");
    for bytes in [b"short".to_vec(),[b"BADMAGIC".as_slice(),&0i64.to_le_bytes(),b"{}"].concat(),
        [storage::MAGIC.as_slice(),&i64::MAX.to_le_bytes(),b"{}"].concat()] {
        std::fs::write(&path,bytes).unwrap();
        let failure=match Cpg::with_storage(&path){Err(error)=>error,Ok(_)=>panic!("corrupt storage must fail")};
        assert_eq!(failure.source_exception_class(),Some("flatgraph.storage.Deserialization$DeserializationException"));
    }
}
