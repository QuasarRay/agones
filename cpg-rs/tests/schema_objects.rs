#![cfg(feature="native-schema")]
use cpg_rs::{cpg_schema, schema::*};
use serde_json::{Value, json};

fn builder(name: &str) -> ObjectSchemaBuilder { ObjectSchemaBuilder::new(name.into(), format!("fixture.{name}"), vec![]) }
fn string(value: &str) -> Scalar { Scalar::String { value: value.into() } }

#[test]
fn reference_schema_matches_original_duplicate_equality_aliasing_and_cross_builder_behavior() {
    let expected: Value = serde_json::from_slice(include_bytes!("../../verification/agones/first-generation-schema-objects-reference.json")).unwrap();
    let mut builder = builder("Objects");
    let first = builder.add_property("CODE".into(), ValueType::String, "same", None); first.mandatory(string("first")).unwrap().proto_id(1);
    let equal = builder.add_property("CODE".into(), ValueType::String, "same", None); equal.mandatory(string("equal")).unwrap().proto_id(2);
    let different = builder.add_property("CODE".into(), ValueType::Long, "different", None); different.mandatory(Scalar::Long { decimal: "3".into() }).unwrap().proto_id(3);
    let unused = builder.add_property("CODE".into(), ValueType::Int, "unused", None); unused.proto_id(1);
    let base = builder.add_node_base_type("BASE".into(), "", None); base.add_property(&first);
    let other_base = builder.add_node_base_type("BASE".into(), "", None); other_base.add_property(&different);
    let left = builder.add_node_type("NODE".into(), "", None); left.proto_id(10).extend(&base).unwrap().add_properties([&first, &equal]).primary_key(Some(&first));
    let right = builder.add_node_type("NODE".into(), "", None); right.proto_id(11).extend(&other_base).unwrap();
    let diamond = builder.add_node_type("DIAMOND".into(), "", None); diamond.proto_id(12).extend(&other_base).unwrap().extend(&base).unwrap().extend(&base).unwrap();
    let edge = builder.add_edge_type("LINK".into(), "", None); edge.proto_id(20).with_property(Some(&first));
    let other_edge = builder.add_edge_type("LINK".into(), "", None); other_edge.proto_id(21).with_property(Some(&different));
    left.add_out_edge(&edge, &right, EdgeConnection::default()).add_out_edge(&edge, &right, EdgeConnection::default()).add_out_edge(&other_edge, &right, EdgeConnection::default());
    left.add_contained_node(&right, "child".into(), Cardinality::List, "").add_contained_node(&right, "child".into(), Cardinality::List, "");
    builder.dont_warn_for_duplicate_property(&left, &first).dont_warn_for_duplicate_property(&left, &equal);
    let constant = builder.constant(Constant { name: "C".into(), value: "value".into(), value_type: ValueType::String, proto_id: None, comment: None, schema_info: None });
    builder.add_constants("CATEGORY".into(), [constant.clone()]);
    let schema = builder.build().unwrap();
    let mut external = ObjectSchemaBuilder::new("External".into(), "fixture.external".into(), vec![]);
    let external_property = external.add_property("EXTERNAL".into(), ValueType::Boolean, "", None); external_property.mandatory(Scalar::Boolean { value: true }).unwrap();
    let external_node = external.add_node_type("EXTERNAL_NODE".into(), "", None);
    let external_edge = external.add_edge_type("EXTERNAL_EDGE".into(), "", None);
    let properties = [(&first, "first"), (&equal, "equal"), (&different, "different"), (&unused, "unused"), (&external_property, "external")];
    let prop_ref = |p: &PropertyRef| properties.iter().find(|(property, _)| property.same_object(p)).unwrap().1;
    let nodes = [(&base, "base"), (&other_base, "otherBase"), (&left, "left"), (&right, "right"), (&diamond, "diamond"), (&external_node, "external")];
    let node_ref = |n: &NodeRef| nodes.iter().find(|(node, _)| node.same_object(n)).unwrap().1;
    let edges = [(&edge, "edge"), (&other_edge, "otherEdge"), (&external_edge, "external")];
    let edge_ref = |e: &EdgeRef| edges.iter().find(|(edge, _)| edge.same_object(e)).unwrap().1;
    let snapshot = || {
        let mut diamond_properties: Vec<_> = diamond.properties().unwrap().iter().map(prop_ref).collect(); diamond_properties.sort();
        let mut ancestors: Vec<_> = diamond.ancestors().unwrap().iter().map(node_ref).collect(); ancestors.sort();
        let mut subtypes: Vec<_> = base.subtypes(&schema.all_node_types()).unwrap().iter().map(node_ref).collect(); subtypes.sort();
        let adjacency = |node: &NodeRef, outgoing| { let mut edges: Vec<_> = node.edges(outgoing).iter().map(|e| json!({"edge":edge_ref(&e.edge),"node":node_ref(&e.neighbor)})).collect(); edges.sort_by_key(|e| e["edge"].as_str().unwrap().to_owned()); edges };
        json!({"properties":schema.properties.iter().map(|p| { let value = p.value(); json!({"ref":prop_ref(p),"protoId":value.proto_id,"mandatory":value.mandatory,"default":value.default}) }).collect::<Vec<_>>(),
            "nodeProperties":schema.node_properties().unwrap().iter().map(prop_ref).collect::<Vec<_>>(),
            "edgeProperties":schema.edge_properties().iter().map(prop_ref).collect::<Vec<_>>(),
            "leftProperties":left.properties().unwrap().iter().map(prop_ref).collect::<Vec<_>>(),
            "leftDeclared":left.properties_without_inheritance().iter().map(prop_ref).collect::<Vec<_>>(),
            "rightProperties":right.properties().unwrap().iter().map(prop_ref).collect::<Vec<_>>(),
            "diamondProperties":diamond_properties,"diamondAncestors":ancestors,
            "leftEdges":adjacency(&left,true),"rightEdges":adjacency(&right,false),
            "contained":left.contained_nodes().iter().map(|n| node_ref(&n.node_type)).collect::<Vec<_>>(),
            "baseSubtypes":subtypes,"anySubtypes":schema.any_node.subtypes(&schema.all_node_types()).unwrap().len(),
            "noWarn":schema.no_warn.len(),"constantProtoId":schema.constants_by_category["CATEGORY"][0].value().proto_id,
            "accessor":edge.metadata().default_accessor_name})
    };
    let before = snapshot();
    first.mandatory(string("changed")).unwrap().proto_id(41); right.add_property(&first); constant.proto_id(99); edge.with_default_accessor_name(Some("changedAccessor".into()));
    left.add_property(&external_property).add_out_edge(&external_edge, &external_node, EdgeConnection::default());
    builder.add_property("LATE".into(), ValueType::String, "", None);
    let after = snapshot();
    let mut duplicate = ObjectSchemaBuilder::new("Duplicate".into(), "fixture.duplicate".into(), vec![]);
    let p = duplicate.add_property("P".into(), ValueType::String, "equal", None); p.proto_id(7);
    duplicate.add_property("P".into(), ValueType::String, "equal", None).proto_id(7);
    duplicate.add_node_type("N".into(), "", None).add_property(&p);
    let duplicate_error = duplicate.build().err().unwrap();
    let mut custom = ObjectSchemaBuilder::new("Custom".into(), "custom.package".into(), vec!["custom.traversals".into()]);
    let custom_name = custom.add_property("NAME".into(), ValueType::String, "caller property", None); custom_name.mandatory(string("caller")).unwrap().proto_id(999);
    cpg_schema::populate(&mut custom).unwrap();
    let full = custom.build().unwrap();
    let actual = json!({"before":before,"after":after,"propertyEquals":first==equal,"samePropertyObject":first.same_object(&equal),"sameNodeObject":left.same_object(&right),
        "duplicateEqualProto":duplicate_error.source_exception_class().unwrap(),
        "customCpg":{"allNameProperties":full.properties.iter().filter(|p| p.value().name=="NAME").count(),
            "usedCaller":full.node_properties().unwrap().iter().any(|p| p.same_object(&custom_name)),"callerDefault":custom_name.value().default,
            "domain":full.domain_short_name,"package":full.base_package,"nodeTypes":full.node_types.len()}});
    assert_eq!(actual, expected);
    // The name-resolved export must not silently collapse distinct references.
    assert!(schema.to_model().err().unwrap().to_string().contains("ambiguous"));
    assert!(full.to_model().err().unwrap().to_string().contains("ambiguous"));
}

#[test]
fn copied_handles_survive_builder_ownership_and_keep_repeated_population_separate() {
    let (schema, property, alias) = {
        let mut builder = builder("Aliasing");
        let property = builder.add_property("P".into(), ValueType::Int, "", None);
        builder.add_node_type("N".into(), "", None).add_property(&property);
        (builder.build().unwrap(), property.clone(), property)
    };
    alias.mandatory(Scalar::Int { decimal: "42".into() }).unwrap();
    assert_eq!(schema.node_properties().unwrap()[0].value().default, property.value().default);
    assert_eq!(property.value().default, Some(Scalar::Int { decimal: "42".into() }));
    let mut builder = builder("Repeated");
    cpg_schema::populate(&mut builder).unwrap(); cpg_schema::populate(&mut builder).unwrap();
    let error = builder.build().err().unwrap();
    assert_eq!(error.source_exception_class(), Some("java.lang.AssertionError"));
}

#[test]
fn generated_factories_bind_the_reference_runtime_and_retain_every_object_operand() {
    let report: Value = serde_json::from_slice(include_bytes!("../generated/construction.json")).unwrap();
    assert_eq!(report["schemaObjectsSha256"], cpg_rs::content_digest(include_bytes!("../src/schema_objects.rs")));
    assert_eq!(report["generatorSha256"], cpg_rs::content_digest(include_bytes!("../src/construction.rs")));
    let source = include_str!("../generated/schema_constructors.rs");
    assert!(!source.contains("builder.property_mut(") && !source.contains("builder.node_mut(") && !source.contains("builder.edge_mut("));
    let model = cpg_schema::domain_schema_objects().unwrap();
    assert_eq!(model.node_types.len(), 43);
    assert_eq!(model.properties.len(), 55);
    assert_eq!(model.to_model().unwrap(), cpg_schema::domain_schema().unwrap());
}
