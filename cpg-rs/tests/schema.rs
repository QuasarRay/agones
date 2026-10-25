use cpg_rs::schema::*;

fn bootstrap() -> Bootstrap {
    Bootstrap::load(include_bytes!("../../verification/agones/first-generation-complete-schema-bootstrap.json").as_slice()).unwrap()
}

#[test]
fn schema_base_preserves_scala_default_and_validates_dynamic_json() {
    struct MinimalSchema;

    impl SchemaBase for MinimalSchema {
        fn doc_index(&self) -> i32 { 7 }
        fn description(&self) -> &str { "minimal" }
    }

    let minimal = MinimalSchema;
    assert_eq!(minimal.doc_index(), 7);
    assert_eq!(minimal.description(), "minimal");
    assert!(!minimal.provided_by_frontend());

    let value = serde_json::json!({
        "definitionClass": "fixture.Definition",
        "schemaClass": "fixture.Schema",
        "schemaName": "Fixture",
        "docIndex": 3,
        "description": "fixture schema"
    });
    validate_schema_base_json(&value).unwrap();

    let info: SchemaInfo = serde_json::from_value(value).unwrap();
    assert_eq!(info.doc_index(), 3);
    assert_eq!(info.description(), "fixture schema");
    assert!(!info.provided_by_frontend());

    let out_of_range = serde_json::json!({
        "docIndex": i64::from(i32::MAX) + 1,
        "description": "invalid"
    });
    assert!(validate_schema_base_json(&out_of_range).is_err());

    let wrong_type = serde_json::json!({
        "docIndex": 1,
        "description": "invalid",
        "providedByFrontend": "false"
    });
    assert!(validate_schema_base_json(&wrong_type).is_err());
}

#[test]
fn both_upstream_schema_configurations_rebuild_without_losing_metadata() {
    let model = bootstrap();
    for schema in [model.domain_schema, model.protobuf_schema] {
        let rebuilt = SchemaBuilder::from_schema(schema.clone()).build().unwrap();
        assert_eq!(rebuilt, schema);
        rebuilt.validate().unwrap();
    }
}

#[test]
fn construction_resolves_diamond_inheritance_and_updates_both_edge_directions() {
    let mut builder = SchemaBuilder::new("Fixture".into(), "fixture".into(), vec![]);
    builder.add_property("CODE".into(), ValueType::String, " source ", None)
        .mandatory(Scalar::String { value: "<empty>".into() }).unwrap();
    builder.add_node_base_type("ROOT".into(), "", None).add_property("CODE".into());
    builder.add_node_base_type("BASE_B".into(), "", None).extend("ROOT".into());
    builder.add_node_base_type("BASE_A".into(), "", None).extend("ROOT".into());
    let node = builder.add_node_type("CHILD".into(), "", None);
    node.extend("BASE_B".into()).extend("BASE_A".into()).extend("BASE_A".into());
    builder.add_edge_type("AST".into(), "", None);
    builder.connect("AST", "CHILD", "CHILD", EdgeConnection {
        out_cardinality: EdgeCardinality::One, in_cardinality: EdgeCardinality::ZeroOrOne,
        out_step: " child ".into(), in_step: " parent ".into(), ..Default::default()
    }).unwrap();
    let schema = builder.build().unwrap();
    let node = &schema.node_types[0];
    assert_eq!(node.ancestors, ["BASE_A", "BASE_B", "ROOT"]);
    assert_eq!(node.all_properties, ["CODE"]);
    assert_eq!(node.out_edges[0].custom_step_name.as_deref(), Some("child"));
    assert_eq!(node.in_edges[0].custom_step_name.as_deref(), Some("parent"));
    assert_eq!(node.in_edges[0].cardinality, EdgeCardinality::ZeroOrOne);
    assert_eq!(schema.subtypes("ROOT").len(), 3);
}

#[test]
fn duplicate_proto_ids_reject_only_the_same_wire_namespace() {
    let mut schema = bootstrap().domain_schema;
    // Original schema intentionally reuses numeric ids across node/edge/constant namespaces.
    schema.verify_proto_ids_unique().unwrap();
    let properties = schema.node_properties();
    let first_name = properties[0].name.clone();
    let first_id = properties[0].proto_id.unwrap();
    let second_name = properties[1].name.clone();
    let second = schema.properties.iter_mut().find(|p| p.name == second_name).unwrap();
    second.proto_id = Some(first_id);
    let error = schema.verify_proto_ids_unique().unwrap_err().to_string();
    assert!(error.contains("node properties"));
    assert!(error.contains(&first_name) && error.contains(&second_name));
}

#[test]
fn mandatory_absent_wrapper_null_value_and_numeric_bits_remain_distinct() {
    let mut builder = SchemaBuilder::new("Fixture".into(), "fixture".into(), vec![]);
    let absent = builder.add_property("ABSENT".into(), ValueType::NodeRef, "", None);
    absent.cardinality = Cardinality::One { default_wrapper_is_null: true, default: None };
    builder.add_property("NULL".into(), ValueType::NodeRef, "", None).mandatory(Scalar::Null).unwrap();
    builder.add_property("WIDE".into(), ValueType::Long, "", None)
        .mandatory(Scalar::Long { decimal: i64::MIN.to_string() }).unwrap();
    builder.add_property("NAN".into(), ValueType::Double, "", None)
        .mandatory(Scalar::Double { bits: "7ff8000000000042".into() }).unwrap();
    let schema = builder.build().unwrap();
    let roundtrip: Schema = serde_json::from_slice(&serde_json::to_vec(&schema).unwrap()).unwrap();
    assert_eq!(roundtrip, schema);
    let absent = schema.properties.iter().find(|p| p.name == "ABSENT").unwrap();
    let null = schema.properties.iter().find(|p| p.name == "NULL").unwrap();
    assert!(absent.mandatory && !absent.has_default && absent.default.is_none());
    assert!(null.mandatory && null.has_default && null.default == Some(Scalar::Null));
}

#[test]
fn standard_export_filters_hidden_elements_while_the_internal_schema_retains_them() {
    let schema = bootstrap().domain_schema;
    let internal_hidden = schema.node_types.iter().filter(|n| n.is_hidden()).count();
    assert_eq!(internal_hidden, 4);
    let public = schema.standard_json();
    let public_nodes = public["nodes"].as_array().unwrap();
    assert!(!public_nodes.iter().any(|node| node["name"] == "CLOSURE_BINDING"));
    assert!(schema.node_types.iter().any(|node| node.name == "CLOSURE_BINDING"));
    assert!(public["properties"].as_array().unwrap().iter().all(|p| p["name"] != "POSSIBLE_TYPES"));
}

#[test]
fn incorrect_derived_fields_and_unknown_metadata_cannot_silently_pass_loading() {
    let mut schema = bootstrap().domain_schema;
    schema.node_types[0].all_properties.clear();
    assert!(schema.validate().is_err());
    let mut value: serde_json::Value = serde_json::from_slice(
        include_bytes!("../../verification/agones/first-generation-complete-schema-bootstrap.json")).unwrap();
    value["domainSchema"]["properties"][0]["unrepresentedMetadata"] = serde_json::json!(true);
    assert!(Bootstrap::load(serde_json::to_vec(&value).unwrap().as_slice()).is_err());
}
