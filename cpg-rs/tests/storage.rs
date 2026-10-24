use cpg_rs::{Error,graph::*,schema::{Schema,SchemaBuilder,ValueType}};
use serde_json::Value as Json;
use std::{path::Path,sync::Arc};

mod support;
use support::{evidence,snapshot};

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
fn import_cpg(path:impl AsRef<Path>)->Cpg {Cpg::from_graph(storage::read_graph(Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),path.as_ref()).unwrap())}
fn reference()->Json {serde_json::from_slice(&std::fs::read(evidence("first-generation-flatgraph-reference.json")).expect("pinned storage observation must exist")).unwrap()}

#[test]
fn original_storage_reopens_with_independent_edge_orders_deletions_refs_and_defaults() {
    let expected=reference();let mut cpg=import_cpg(evidence("first-generation-flatgraph-reference.fg"));
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
    let mut graph=storage::read_graph(schema,evidence("first-generation-flatgraph-primitives.fg").as_path()).unwrap();
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
    let cpg=import_cpg(evidence("first-generation-flatgraph-reference.fg"));
    let counts=storage::write_graph(cpg.graph(),&path).unwrap();
    assert_eq!(counts.node_kinds,expected["counts"]["nodeKinds"].as_u64().unwrap() as usize);
    assert_eq!(counts.edge_kinds,expected["counts"]["edgeKinds"].as_u64().unwrap() as usize);
    assert_eq!(counts.property_kinds,expected["counts"]["propertyKinds"].as_u64().unwrap() as usize);
    let cpg=import_cpg(&path);assert_eq!(snapshot(cpg.graph()),expected["cpg"]);
    let primitive=storage::read_graph(Arc::new(primitive_schema()),&evidence("first-generation-flatgraph-primitives.fg")).unwrap();
    let primitive_path=directory.path().join("primitive.fg");storage::write_graph(&primitive,&primitive_path).unwrap();
    let primitive=storage::read_graph(Arc::new(primitive_schema()),&primitive_path).unwrap();assert_eq!(snapshot(&primitive),expected["primitive"]);
}

#[test]
fn storage_lifecycle_defers_creation_persists_changes_and_obeys_read_only_configuration() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("new/graph.grafeo");
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
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("import.grafeo");
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
        let failure=match storage::read_graph(Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),&path){Err(error)=>error,Ok(_)=>panic!("corrupt storage must fail")};
        assert_eq!(failure.source_exception_class(),Some("flatgraph.storage.Deserialization$DeserializationException"));
    }
}

#[test]
fn grafeo_queries_and_its_independent_reader_observe_native_records_and_deny_mutations() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("native.grafeo");
    let a=DetachedNode::new("METHOD");a.set("NAME",Value::string("native")).unwrap();
    let b=DetachedNode::new("METHOD");
    let mut cpg=Cpg::from_initial(|diff| {diff.add_edge(a.clone(),b.clone(),"AST",EdgeProperty::Default);Ok(())}).unwrap();
    let result=CpgContext.query_cpg_database(cpg.graph(),"MATCH (n:METHOD) RETURN count(n)").unwrap();
    assert_eq!(result.scalar::<i64>().unwrap(),2);
    let result=CpgContext.query_cpg_database(cpg.graph(),"MATCH (n:METHOD) WHERE n.`cpg:NAME`[0][1] = 'native' RETURN n.__cpg_id").unwrap();
    assert_eq!(result.scalar::<i64>().unwrap(),a.stored_ref().unwrap().unwrap().id.0 as i64);
    assert!(CpgContext.query_cpg_database(cpg.graph(),"INSERT (:METHOD)").is_err());
    assert_eq!(cpg.graph().node_count(),2);
    CpgContext.save_cpg_database(cpg.graph(),&path).unwrap();
    let raw=::grafeo::GrafeoDB::open_read_only(&path).unwrap();
    assert_eq!(raw.session().execute("MATCH (:METHOD)-[e:AST]->(:METHOD) RETURN count(e)").unwrap().scalar::<i64>().unwrap(),1);
    assert_eq!(raw.iter_nodes().filter(|node|node.get_property("__cpg_id").is_some()).count(),2);
    assert!(raw.iter_nodes().any(|node|node.get_property("cpg:NAME").is_some()));raw.close().unwrap();
    // Invalidate an already-populated borrowed view through a real Grafeo write.
    let mut diff=DiffGraphBuilder::new();diff.set_node_property(a.stored_ref().unwrap().unwrap(),"NAME",Value::string("changed"));
    cpg.graph_mut().apply_diff(diff).unwrap();
    assert_eq!(cpg.graph().nodes_with_property(None,"NAME","native").count(),0);
    assert_eq!(cpg.graph().nodes_with_property(None,"NAME","changed").count(),1);
}

#[test]
fn flatgraph_import_survives_grafeo_persistence_with_exact_bits_and_roundtrip_columns() {
    let directory=tempfile::tempdir().unwrap();
    for (file,schema,expected) in [("first-generation-flatgraph-primitives.fg",Arc::new(primitive_schema()),"primitive"),
        ("first-generation-flatgraph-reference.fg",Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),"cpg")] {
        let graph=storage::read_graph(schema.clone(),&evidence(file)).unwrap();let path=directory.path().join(format!("{expected}.grafeo"));
        graph.save_database(&path).unwrap();let graph=Graph::with_storage(schema.clone(),&path,false).unwrap();
        assert_eq!(snapshot(&graph),reference()[expected]);
        let output=directory.path().join(format!("{expected}.fg"));
        storage::write_graph(&graph,&output).unwrap();let graph=storage::read_graph(schema,&output).unwrap();
        assert_eq!(snapshot(&graph),reference()[expected]);
    }
}

#[test]
fn grafeo_checkpoints_bind_schema_and_persist_deleted_slots_and_next_edge_identity() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("checkpoint-without-extension");
    let a=DetachedNode::new("METHOD");let b=DetachedNode::new("METHOD");
    let mut cpg=Cpg::from_initial(|diff| {diff.add_edge(a.clone(),b.clone(),"AST",EdgeProperty::Default);Ok(())}).unwrap();
    let old_node=a.stored_ref().unwrap().unwrap().id;let old_edge=cpg.graph().all_edges()[0].handle.id;
    let mut deletion=DiffGraphBuilder::new();deletion.remove_node(a.stored_ref().unwrap().unwrap());cpg.graph_mut().apply_diff(deletion).unwrap();
    cpg.graph().save_database(&path).unwrap();
    let mut reopened=Cpg::with_storage(&path).unwrap();assert!(reopened.graph().node(old_node).is_none());assert_eq!(reopened.graph().edge_count(),0);
    let new_node=DetachedNode::new("METHOD");let existing=reopened.graph().nodes("METHOD").next().unwrap().handle;
    let mut diff=DiffGraphBuilder::new();diff.add_edge(existing,new_node.clone(),"AST",EdgeProperty::Default);reopened.graph_mut().apply_diff(diff).unwrap();
    assert!(new_node.stored_ref().unwrap().unwrap().id.sequence()>old_node.sequence());
    assert!(reopened.graph().all_edges()[0].handle.id>old_edge);reopened.close().unwrap();
    assert!(matches!(Graph::with_storage(Arc::new(primitive_schema()),&path,false),Err(Error::Invalid(_))));
    assert_eq!(Cpg::with_storage_options(&path,false).unwrap().graph().node_count(),2);
}

#[test]
fn storage_cli_imports_and_exports_through_the_configured_cgp_providers() {
    let directory=tempfile::tempdir().unwrap();let database=directory.path().join("cli.grafeo");let exported=directory.path().join("cli.fg");
    let binary=env!("CARGO_BIN_EXE_cpg-storage");
    for (operation,input,output) in [("import",evidence("first-generation-flatgraph-reference.fg"),database.clone()),("export",database.clone(),exported.clone())] {
        let result=std::process::Command::new(binary).arg(operation).arg(input).arg(output).output().unwrap();
        assert!(result.status.success(),"{operation}: {}",String::from_utf8_lossy(&result.stderr));
    }
    assert_eq!(snapshot(Cpg::with_storage_options(&database,false).unwrap().graph()),reference()["cpg"]);
    assert_eq!(snapshot(import_cpg(&exported).graph()),reference()["cpg"]);
}

#[test]
fn native_legacy_reader_matches_original_conversion_and_grafeo_reopen() {
    use cpg_rs::graph::legacy::CanImportLegacyCpg;
    let reference:Json=serde_json::from_slice(&std::fs::read(evidence("first-generation-legacy-reference.json")).unwrap()).unwrap();
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("legacy.grafeo");
    let mut cpg=CpgContext.import_legacy_cpg(&evidence("first-generation-legacy-reference.odb"),&path).unwrap();
    assert_eq!(snapshot(cpg.graph()),reference["cpg"]);cpg.close().unwrap();
    assert_eq!(snapshot(Cpg::with_storage_options(&path,false).unwrap().graph()),reference["cpg"]);
}

#[test]
fn legacy_mvstore_follows_committed_roots_across_pages_versions_updates_and_deletions() {
    let reference:Json=serde_json::from_slice(&std::fs::read(evidence("first-generation-legacy-reference.json")).unwrap()).unwrap();
    let graph=legacy::read_graph(Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),&evidence("first-generation-legacy-multipage.odb")).unwrap();
    let actual=snapshot(&graph);assert_eq!(graph.node_count(),1542);
    for (index,(actual,expected)) in actual["nodes"].as_array().unwrap().iter().zip(reference["multipage"]["nodes"].as_array().unwrap()).enumerate() {
        assert_eq!(actual,expected,"legacy row {index}");
    }
}

#[test]
fn legacy_reader_uses_the_surviving_header_and_rejects_total_header_corruption() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("header.odb");
    let mut bytes=std::fs::read(evidence("first-generation-legacy-reference.odb")).unwrap();
    bytes[0]=b'!';std::fs::write(&path,&bytes).unwrap();
    assert_eq!(legacy::read_graph(Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),&path).unwrap().node_count(),4);
    bytes[4096]=b'!';std::fs::write(&path,&bytes).unwrap();
    assert!(matches!(legacy::read_graph(Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),&path),Err(Error::LegacyStorage(_))));
}

#[test]
fn cpg_loader_dispatches_all_formats_to_grafeo_without_changing_import_inputs() {
    use std::io::Write;
    let directory=tempfile::tempdir().unwrap();
    let proto_path=directory.path().join("proto.zip");
    let mut zip=zip::ZipWriter::new(std::fs::File::create(&proto_path).unwrap());
    zip.start_file("cpg",zip::write::SimpleFileOptions::default()).unwrap();
    zip.write_all(&std::fs::read(evidence("first-generation-proto-reference-duplicate-types.pb")).unwrap()).unwrap();zip.finish().unwrap();
    let fg=directory.path().join("flatgraph.fg");std::fs::copy(evidence("first-generation-flatgraph-reference.fg"),&fg).unwrap();
    let legacy=directory.path().join("legacy.odb");std::fs::copy(evidence("first-generation-legacy-reference.odb"),&legacy).unwrap();
    for (input,expected) in [(&proto_path,3),(&fg,4),(&legacy,4)] {
        let original=std::fs::read(input).unwrap();let mut cpg=Cpg::load(input).unwrap();
        assert_eq!(cpg.graph().node_count(),expected);let persisted=cpg.graph().storage_path().unwrap().to_owned();
        assert_eq!(persisted.file_name().unwrap(),format!("{}.grafeo",input.file_name().unwrap().to_str().unwrap()).as_str());
        cpg.close().unwrap();assert!(cpg_rs::cpg_loading::is_grafeo_format(&persisted));
        assert_eq!(std::fs::read(input).unwrap(),original);assert_eq!(Cpg::load(&persisted).unwrap().graph().node_count(),expected);
    }
}

#[test]
fn cpg_loader_native_copy_overwrite_and_error_order_follow_the_storage_lifecycle() {
    let directory=tempfile::tempdir().unwrap();let source=directory.path().join("source.grafeo");let target=directory.path().join("target.grafeo");
    let node=DetachedNode::new("METHOD");let cpg=Cpg::from_initial(|diff|{diff.add_node(node);Ok(())}).unwrap();cpg.graph().save_database(&source).unwrap();
    let original=std::fs::read(&source).unwrap();std::fs::write(&target,b"overwritten").unwrap();
    let mut copied=Cpg::load_to(&source,&target).unwrap();assert_eq!(copied.graph().node_count(),1);copied.close().unwrap();
    assert_eq!(std::fs::read(&source).unwrap(),original);assert_eq!(std::fs::read(&target).unwrap(),original);
    let missing=directory.path().join("missing");assert!(matches!(Cpg::load_to(&missing,&target),Err(Error::Io(_))));assert!(!target.exists());
    std::fs::write(&missing,b"not a supported graph").unwrap();
    assert!(matches!(Cpg::load_to(&missing,&target),Err(Error::SourceAssertion(_))));
}

#[test]
fn legacy_converter_retains_original_unsupported_case_failures_and_all_null_column_omission() {
    let reference:Json=serde_json::from_slice(&std::fs::read(evidence("first-generation-legacy-failures.json")).unwrap()).unwrap();
    for name in ["primitive-array","character","version","null-only"] {
        let result=legacy::read_graph(Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),&evidence(&format!("first-generation-legacy-failure-{name}.odb")));
        match result {
            Err(error)=>assert_eq!(error.source_exception_class(),reference[name]["error"].as_str(),"legacy case {name}: {error}"),
            Ok(graph)=>assert_eq!(snapshot(&graph),reference[name]["snapshot"],"legacy case {name}"),
        }
    }
}

#[test]
fn legacy_mvstore_handles_plain_deflated_large_pages_and_empty_graphs() {
    let reference:Json=serde_json::from_slice(&std::fs::read(evidence("first-generation-legacy-modes.json")).unwrap()).unwrap();
    for mode in ["plain","deflate","empty"] {
        let graph=legacy::read_graph(Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),&evidence(&format!("first-generation-legacy-mode-{mode}.odb"))).unwrap();
        assert!(snapshot(&graph)==reference[mode],"source-bound legacy {mode} snapshot differs");
    }
}

#[test]
fn legacy_metadata_counters_preserve_original_number_failures_and_initialization_order() {
    let reference:Json=serde_json::from_slice(&std::fs::read(evidence("first-generation-legacy-metadata.json")).unwrap()).unwrap();
    for name in ["library","strings"] {
        let error=match legacy::read_graph(Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),&evidence(&format!("first-generation-legacy-meta-{name}.odb"))) {
            Err(error)=>error,Ok(_)=>panic!("invalid {name} counter must fail"),
        };
        assert_eq!(error.source_exception_class(),reference[name]["error"].as_str());
    }
}
