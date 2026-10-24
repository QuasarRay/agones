use cpg_rs::domain::*;
mod support;

#[test]
fn schema_defaults_and_inherited_accessors_are_generated_concretely() {
    let call = Call::default();
    assert_eq!(Call::LABEL,"CALL");
    assert_eq!(call.argument_index,-1);
    assert_eq!(call.code,"<empty>");
    assert!(call.possible_types.is_empty());
    assert_eq!(AstNodeProperties::code(&call),"<empty>");
    assert_eq!(CfgNodeProperties::order(&call),&-1);
}
#[test]
fn graph_handles_and_reference_nodes_have_distinct_rust_types() {
    let handle = node_refs::Method(17);
    let mut reference_node = MethodRef::default();
    reference_node.node_id = Some(handle.0);
    assert_eq!(reference_node.node_id,Some(17));
    let type_handle = node_refs::Type(33);
    let mut type_reference = TypeRef::default();
    type_reference.node_id = Some(type_handle.0);
    assert_eq!(type_reference.node_id,Some(33));
}
#[test]
fn hidden_nodes_contained_references_and_edge_properties_are_retained() {
    assert_eq!(Import::LABEL,"IMPORT");
    let pair = TagNodePair { tag:Some(node_refs::Tag(5)),node:Some(NodeRef(7)),..Default::default() };
    let serialized = serde_json::to_value(&pair).unwrap();
    assert_eq!(serialized["tag"],5);
    assert_eq!(serialized["node"],7);
    let edge = ReachingDefEdge { edge_id:None,source:NodeRef(1),target:NodeRef(2),variable:"value".into() };
    assert_eq!(serde_json::to_value(edge).unwrap()["VARIABLE"],"value");
}
#[test]
fn emitted_names_preserve_source_constants_and_optional_presence() {
    assert_eq!(operators::ASSIGNMENT_EXPONENTIATION,"<operators>.assignmentExponentiation");
    assert_eq!(node_types::CLOSURE_BINDING,"CLOSURE_BINDING");
    let empty: Annotation = serde_json::from_str("{}").unwrap();
    assert_eq!(empty,Annotation::default());
    assert!(serde_json::from_str::<Annotation>(r#"{"NOT_A_SCHEMA_PROPERTY":1}"#).is_err());
}

fn typed_value_json(value:&cpg_rs::graph::Value,graph:Option<&cpg_rs::graph::Graph>)->serde_json::Value {
    use cpg_rs::graph::Value as V;use serde_json::json;
    match value {
        V::Null=>serde_json::Value::Null,V::String(value)=>json!(value.as_ref()),V::Boolean(value)=>json!(value),V::Int(value)=>json!(value),
        V::List(values)=>serde_json::Value::Array(values.iter().map(|v|typed_value_json(v,graph)).collect()),
        V::Node(node)=>json!({"stored":node.id.0.to_string(),"label":graph.unwrap().get(*node).unwrap().unwrap().label}),
        other=>panic!("unaccounted typed fixture value {other:?}"),
    }
}
fn typed_input_json(value:&cpg_rs::graph::PropertyInput)->serde_json::Value {
    use cpg_rs::graph::{PropertyInput as P,NodeReference as R};use serde_json::json;
    match value {
        P::Value(value)=>typed_value_json(value,None),P::List(values)=>serde_json::Value::Array(values.iter().map(typed_input_json).collect()),
        P::Node(R::Detached(node))=>json!({"new":node.label().unwrap(),"stored":node.stored_ref().unwrap().map(|node|node.id.0.to_string())}),
        P::Node(R::Stored(_))=>panic!("this fixture keeps new-node references in new-node properties"),
    }
}
fn new_properties<C:cpg_rs::graph::typed::ConcreteNodeClass>(node:&cpg_rs::graph::typed::TypedNew<C>)->serde_json::Value {
    serde_json::Value::Object(node.properties_map().unwrap().iter().map(|(name,value)|(name.clone(),typed_input_json(value))).collect())
}
fn stored_properties<C:cpg_rs::graph::typed::NodeClass>(node:cpg_rs::graph::typed::TypedNode<'_,C>)->serde_json::Value {
    serde_json::Value::Object(node.properties_map().unwrap().iter().map(|(name,value)|(name.clone(),typed_value_json(value,Some(node.graph())))).collect())
}

#[test]
fn generated_graph_api_matches_original_typed_construction_accessors_and_queries() {
    use new_nodes::*;use cpg_rs::graph::{Cpg,DiffGraphBuilder,EdgeProperty,Value,Direction};
    use cpg_rs::graph::typed::{TypedReference,TypedNode};use graph::markers;use serde_json::{json,Value as Json};
    let reference:Json=serde_json::from_slice(&std::fs::read(support::evidence("first-generation-typed-domain-reference.json")).unwrap()).unwrap();
    let method=NewMethod::new();method.name("alpha").unwrap().full_name("example.alpha").unwrap().column_number(Some(4)).unwrap();
    let copied=method.copy().unwrap();copied.name("copy").unwrap().column_number(None).unwrap();
    let tag=NewTag::new();tag.name("label").unwrap().value("display").unwrap();
    let pair=NewTagNodePair::new();pair.node(&method).unwrap().tag(&tag).unwrap();
    let key_value=NewKeyValuePair::new();key_value.key("x").unwrap().value("y").unwrap();
    let finding=NewFinding::new();finding.evidence_nullable([Some((&method).into()),Some((&copied).into()),None]).unwrap().key_value_pairs([&key_value]).unwrap();
    let call=NewCall::new();call.name("target").unwrap().possible_types_nullable(vec![Some("A".into()),None,Some("A".into())]).unwrap();
    let first=NewBlock::new();first.code("first").unwrap();let second=NewBlock::new();second.code("second").unwrap();let ret=NewMethodReturn::new();
    let before=json!({"method":new_properties(&method),"copy":new_properties(&copied),"emptyPair":new_properties(&NewTagNodePair::new()),
        "pair":new_properties(&pair),"finding":new_properties(&finding),"call":new_properties(&call),
        "validNeighbors":[method.is_valid_neighbor(&first,"AST",Direction::Outgoing),method.is_valid_neighbor(&tag,"AST",Direction::Outgoing),first.is_valid_neighbor(&method,"AST",Direction::Incoming)]});
    assert_eq!(before,reference["before"]);
    assert_eq!(method.get_name().unwrap(),"alpha");assert_eq!(copied.get_name().unwrap(),"copy");
    assert_eq!(pair.get_tag(None).unwrap().unwrap().name(None).unwrap(),"label");
    let mut cpg=Cpg::empty().unwrap();let mut diff=DiffGraphBuilder::new();
    diff.add_node(method.as_detached()).add_node(copied.as_detached()).add_node(call.as_detached()).add_node(pair.as_detached()).add_node(finding.as_detached());
    diff.add_edge(method.clone(),first.clone(),"AST",EdgeProperty::Default).add_edge(method.clone(),second.clone(),"AST",EdgeProperty::Default)
        .add_edge(method.clone(),ret.clone(),"AST",EdgeProperty::Default).add_edge(method.clone(),copied.clone(),"REACHING_DEF",EdgeProperty::Value(Value::Null));
    cpg.graph_mut().apply_diff(diff).unwrap();
    let stored=stored::Method::new(cpg.graph(),method.stored_ref().unwrap().unwrap()).unwrap();
    let copy=stored::Method::new(cpg.graph(),copied.stored_ref().unwrap().unwrap()).unwrap();
    let names=cpg.method().name().collect::<cpg_rs::Result<Vec<_>>>().unwrap();
    let exact=cpg.method().name_exact(["alpha".to_owned()]).unwrap().map(|node|node.unwrap().handle().id.0.to_string()).collect::<Vec<_>>();
    let columns=cpg.method().column_number().collect::<cpg_rs::Result<Vec<_>>>().unwrap();
    let orders=cpg.method().order_gt(-2).map(|node|node.unwrap().handle().id.0.to_string()).collect::<Vec<_>>();
    let possible=cpg.call().possible_types_nullable().collect::<cpg_rs::Result<Vec<_>>>().unwrap();
    let missing=copy.block().unwrap_err().source_exception_class();
    let after=json!({"raw":support::snapshot(cpg.graph()),"method":stored_properties(stored),
        "pair":stored_properties(stored::TagNodePair::new(cpg.graph(),pair.stored_ref().unwrap().unwrap()).unwrap()),
        "finding":stored_properties(stored::Finding::new(cpg.graph(),finding.stored_ref().unwrap().unwrap()).unwrap()),
        "newPair":new_properties(&pair),"names":names,"namesExact":exact,"columns":columns,"ordersGt":orders,
        "possibleTypes":possible,"block":stored.block().unwrap().handle().id.0.to_string(),"missingBlockError":missing});
    assert_eq!(after,reference["after"]);
    let base=stored.cast::<markers::AstNode>().unwrap();assert_eq!(stored_properties(base),reference["after"]["method"]);
    let reference:TypedReference<markers::StoredNode>=(&method).into();assert_eq!(reference.stored_ref().unwrap(),method.stored_ref().unwrap());
    let evidence=stored::Finding::new(cpg.graph(),finding.stored_ref().unwrap().unwrap()).unwrap().evidence().unwrap();
    assert_eq!(evidence.len(),3);assert!(evidence[2].is_none());
    let _:TypedNode<'_,markers::Block>=stored.block().unwrap();
}

#[test]
fn typed_mutations_retain_identity_and_survive_grafeo_persistence() {
    use new_nodes::*;use cpg_rs::graph::{Cpg,DiffGraphBuilder,EdgeProperty,Value};
    let method=NewMethod::new();method.name("before").unwrap();let other=NewMethod::new();
    let mut cpg=Cpg::from_initial(|diff|{diff.add_edge(method.clone(),other.clone(),"REACHING_DEF",EdgeProperty::Default);Ok(())}).unwrap();
    let handle=method.stored_ref().unwrap().unwrap();let mut diff=DiffGraphBuilder::new();
    let stored=stored::Method::new(cpg.graph(),handle).unwrap();stored.set_name(&mut diff,"after").unwrap().set_column_number(&mut diff,Some(9)).unwrap();
    let edge=cpg.reaching_def_edges().next().unwrap().unwrap();assert_eq!(edge.variable().unwrap(),Some("<empty>".into()));edge.set_variable(&mut diff,None).unwrap();
    cpg.graph_mut().apply_diff(diff).unwrap();assert_eq!(cpg.method().name_exact(["after".into()]).unwrap().count(),1);
    assert!(cpg.reaching_def_edges().next().unwrap().unwrap().variable().unwrap().is_none());
    assert_eq!(method.get_name().unwrap(),"before");let copied=method.copy().unwrap();assert!(copied.stored_ref().unwrap().is_none());
    assert_eq!(method.reference().name(None).unwrap(),"before");
    let view=stored::Method::new(cpg.graph(),handle).unwrap();assert_eq!(view.reference().name(Some(cpg.graph())).unwrap(),"after");
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("typed.grafeo");cpg.graph().save_database(&path).unwrap();
    let reopened=Cpg::with_storage_options(&path,false).unwrap();assert_eq!(reopened.method().name_exact(["after".into()]).unwrap().column_number().collect::<cpg_rs::Result<Vec<_>>>().unwrap(),vec![9]);
    assert_eq!(reopened.reaching_def_edges().next().unwrap().unwrap().variable().unwrap(),None);
    let mut diff=DiffGraphBuilder::new();diff.set_node_property(handle,"NAME",Value::List(vec![Value::string("a"),Value::string("b")]));cpg.graph_mut().apply_diff(diff).unwrap();
    let failure=stored::Method::new(cpg.graph(),handle).unwrap().name().unwrap_err();
    let reference:serde_json::Value=serde_json::from_slice(&std::fs::read(support::evidence("first-generation-typed-domain-reference.json")).unwrap()).unwrap();
    assert_eq!(failure.source_exception_class(),reference["multipleSinglePropertyError"].as_str());
}

#[test]
fn generated_graph_bindings_cover_the_complete_schema_with_current_output_ranges() {
    use std::collections::BTreeSet;
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let schema=cpg_rs::schema::Bootstrap::load(std::fs::File::open(support::evidence("first-generation-complete-schema-bootstrap.json")).unwrap()).unwrap().domain_schema;
    let report:serde_json::Value=serde_json::from_slice(&std::fs::read(root.join("generated/generation.json")).unwrap()).unwrap();
    let output=std::fs::read(root.join("generated/domain.rs")).unwrap();assert_eq!(cpg_rs::content_digest(&output),report["outputSha256"]);
    let classes:Vec<_>=std::iter::once(&schema.any_node).chain(&schema.node_base_types).chain(&schema.node_types).collect();
    let mut expected=BTreeSet::new();
    for node in &classes {for role in ["TYPED_CLASS","STORED_CLASS","REFERENCE_CLASS"] {expected.insert((role.to_owned(),node.name.clone()));}
        if node.kind==cpg_rs::schema::NodeKind::Concrete {expected.insert(("NEW_CLASS".into(),node.name.clone()));}}
    for edge in &schema.edges {expected.insert(("TYPED_EDGE".into(),edge.name.clone()));}
    let mut observed=BTreeSet::new();
    for row in report["mappings"].as_array().unwrap().iter().filter(|row|row.get("graphApiRole").is_some()) {
        let role=row["graphApiRole"].as_str().unwrap();let identity=row["schemaIdentity"].as_str().unwrap();
        assert!(observed.insert((role.to_owned(),identity.to_owned())),"duplicate generated identity");
        let start=row["startByte"].as_u64().unwrap() as usize;let end=row["endByteExclusive"].as_u64().unwrap() as usize;
        assert_eq!(cpg_rs::content_digest(&output[start..end]),row["sha256"]);
        if role!="TYPED_EDGE" {let node=classes.iter().find(|node|node.name==identity).unwrap();
            assert_eq!(row["schemaInfo"]["properties"],serde_json::to_value(&node.all_properties).unwrap());
            assert_eq!(row["schemaInfo"]["containedNodes"],serde_json::to_value(&node.contained_nodes).unwrap());}
    }
    assert_eq!(observed,expected);
    assert_eq!(report["graphApi"]["storedPropertyAccessors"].as_u64().unwrap() as usize,classes.iter().map(|node|node.all_properties.len()).sum::<usize>());
    for (name,hash) in report["graphRuntimeSha256"].as_object().unwrap() {assert_eq!(cpg_rs::content_digest(&std::fs::read(root.join("src").join(name)).unwrap()),hash.as_str().unwrap());}
}

#[test]
fn typed_views_read_the_pinned_original_generated_graph_and_check_runtime_classes() {
    use graph::markers;use cpg_rs::graph::{typed::GraphValue,Value};
    let graph=cpg_rs::graph::storage::read_graph(std::sync::Arc::new(cpg_rs::cpg_schema::domain_schema().unwrap()),&support::evidence("first-generation-typed-domain-reference.fg")).unwrap();
    let reference:serde_json::Value=serde_json::from_slice(&std::fs::read(support::evidence("first-generation-typed-domain-reference.json")).unwrap()).unwrap();
    assert_eq!(support::snapshot(&graph),reference["after"]["raw"]);
    let method=graph.method().name_exact(["alpha".into()]).unwrap().next().unwrap().unwrap();
    assert!(stored::Call::new(&graph,method.handle()).is_err());
    assert!(graph.id(method.handle().id).cast::<markers::Call>().next().unwrap().is_err());
    assert_eq!(graph.id(method.handle().id).of_type::<markers::Call>().count(),0);
    let tag=graph.tag_node_pair().next().unwrap().unwrap().tag().unwrap().unwrap();assert_eq!(tag.name().unwrap(),"label");
    assert_eq!(graph.finding().next().unwrap().unwrap().evidence().unwrap().len(),3);
    assert_eq!(graph.method().name_exact(["alpha".into()]).unwrap().block().next().unwrap().unwrap().code().unwrap(),"first");
    for bits in [0x7fc01234,0x80000000] {let value=Value::Float(bits);let decoded=f32::from_value(&value).unwrap();assert_eq!(decoded.to_bits(),bits);assert_eq!(decoded.into_value().unwrap(),value);}
    for bits in [0x7ff8000000001234,0x8000000000000000] {let value=Value::Double(bits);let decoded=f64::from_value(&value).unwrap();assert_eq!(decoded.to_bits(),bits);assert_eq!(decoded.into_value().unwrap(),value);}
}
