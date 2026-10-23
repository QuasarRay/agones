use cpg_rs::domain::*;

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
