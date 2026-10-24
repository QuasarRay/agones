#![cfg(feature="native-schema")]
use cpg_rs::{cpg_schema,construction::ConstructionTrace,schema::*};

fn reference()->Bootstrap {
    Bootstrap::load(include_bytes!("../../verification/agones/first-generation-complete-schema-bootstrap.json").as_slice()).unwrap()
}

#[test]
fn native_factories_reproduce_both_uninstrumented_complete_schemas() {
    let expected=reference();
    assert_eq!(cpg_schema::domain_schema().unwrap(),expected.domain_schema);
    assert_eq!(cpg_schema::protobuf_schema().unwrap(),expected.protobuf_schema);
}
#[test]
fn constructor_population_preserves_callers_builder_configuration_and_extra_data() {
    let mut builder=ObjectSchemaBuilder::new("Custom".into(),"custom.domain".into(),vec!["custom.traversals".into()]);
    builder.add_property("CUSTOM_PROPERTY".into(),ValueType::Long,"extra",None)
        .mandatory(Scalar::Long{decimal:"9223372036854775807".into()}).unwrap();
    cpg_schema::populate(&mut builder).unwrap();
    let schema=builder.build().unwrap().to_model().unwrap();
    assert_eq!(schema.domain_short_name,"Custom");
    assert_eq!(schema.base_package,"custom.domain");
    assert_eq!(schema.additional_traversals_packages,["custom.traversals"]);
    assert!(schema.properties.iter().any(|property|property.name=="CUSTOM_PROPERTY"));
    assert_eq!(schema.node_types.len(),43);
}
#[test]
fn typed_trace_rejects_missing_provenance_and_cross_variant_mutation_loss() {
    let original:serde_json::Value=serde_json::from_slice(include_bytes!(
        "../../verification/agones/first-generation-schema-construction-trace.json")).unwrap();
    let mut missing=original.clone();
    missing["domain"]["events"][1]["sourceFrames"]=serde_json::json!([]);
    assert!(ConstructionTrace::load(serde_json::to_vec(&missing).unwrap().as_slice()).is_err());
    let mut different=original;
    different["protobuf"]["events"].as_array_mut().unwrap().remove(2);
    for (index,event) in different["protobuf"]["events"].as_array_mut().unwrap().iter_mut().enumerate() {
        event["index"]=serde_json::json!(index);
    }
    assert!(ConstructionTrace::load(serde_json::to_vec(&different).unwrap().as_slice()).is_err());
}
#[test]
fn constructor_generated_metadata_includes_hidden_and_constant_only_schema_parts() {
    let parts=cpg_schema::schema_parts();
    assert!(parts.iter().any(|part|part.schema_name=="Hidden"));
    assert!(parts.iter().any(|part|part.schema_name=="Operators"));
    assert!(parts.iter().any(|part|part.schema_name=="ProtoSerialize"));
}
#[test]
fn deprecated_and_current_proto_id_aliases_are_translated_from_the_full_handwritten_source() {
    assert_eq!(cpg_schema::proto_ids::METHOD,1);
    assert_eq!(cpg_schema::proto_ids::METHOD_SHORT_NAME,102);
    assert_eq!(cpg_schema::proto_ids::PACKAGE_NAME,103);
    assert_eq!(cpg_schema::proto_ids::CONTAINED_REF,2007161);
    assert_eq!(cpg_schema::proto_ids::EVIDENCE_DESCRIPTION,2007162);
}
#[test]
fn proto_id_mapping_resolves_the_complete_symbol_instead_of_a_shared_prefix() {
    let report:serde_json::Value=serde_json::from_slice(include_bytes!("../generated/construction.json")).unwrap();
    let row=report["mappings"].as_array().unwrap().iter().find(|row|row["rustSymbol"]=="proto_ids::C").unwrap();
    let source=include_bytes!("../generated/schema_constructors.rs");
    let span=&source[row["startByte"].as_u64().unwrap() as usize..row["endByteExclusive"].as_u64().unwrap() as usize];
    let compact:String=std::str::from_utf8(span).unwrap().chars().filter(|c|!c.is_whitespace()).collect();
    assert!(compact.contains("pubconstC:i32=5;"));
    assert!(!compact.contains("CONTINUE"));
}
