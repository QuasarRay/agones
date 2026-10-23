#![cfg(feature="native-schema")]
use cpg_rs::{cpg_schema,protobuf_schema,schema_tools::*};

#[test]
fn native_protocol_matches_original_pinned_generator_bytes() {
    let schema=cpg_schema::protobuf_schema().unwrap();
    let native=protobuf_schema::render(&schema).unwrap();
    let reference=include_str!("../../verification/agones/first-generation-cpg-reference.proto");
    let first=native.bytes().zip(reference.bytes()).position(|(a,b)|a!=b).unwrap_or(native.len().min(reference.len()));
    assert!(native==reference,"native protobuf text differs at byte {first}; native length {}, original length {}",native.len(),reference.len());
}

#[test]
fn native_schema_context_owns_protocol_and_non_overwriting_json_export_effects() {
    let root=std::env::temp_dir().join(format!("cpg-schema-tools-{}-{}",std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir(&root).unwrap();
    let protocol=NativeCpgSchemaContext.generate_cpg_proto(&root).unwrap();
    let generated=std::fs::read(protocol).unwrap();
    assert!(generated==include_bytes!("../../verification/agones/first-generation-cpg-reference.proto"),
        "protocol written by CGP context differs from pinned reference");
    let json_path=root.join("schema.json");
    NativeCpgSchemaContext.export_cpg_schema_json(&json_path).unwrap();
    let original=std::fs::read(&json_path).unwrap();
    assert!(NativeCpgSchemaContext.export_cpg_schema_json(&json_path).is_err());
    assert_eq!(std::fs::read(&json_path).unwrap(),original);
    let model:serde_json::Value=serde_json::from_slice(&original).unwrap();
    assert!(model["nodes"].as_array().unwrap().iter().any(|node|node["name"]=="METHOD"));
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn protocol_generation_rejects_missing_options_and_includes_additional_category_entries() {
    let domain=cpg_schema::domain_schema().unwrap();
    assert!(protobuf_schema::render(&domain).is_err());
    let mut schema=cpg_schema::protobuf_schema().unwrap();
    schema.constants_by_category.insert("ExtraCategories".into(),vec![cpg_rs::schema::Constant {
        name:"EXTRA_CATEGORY".into(),value:"extra".into(),value_type:cpg_rs::schema::ValueType::String,
        proto_id:Some(123),comment:Some("extra category".into()),schema_info:None }]);
    let rendered=protobuf_schema::render(&schema).unwrap();
    assert!(rendered.contains("enum ExtraCategories {"));
    assert!(rendered.contains("EXTRA_CATEGORY = 123;"));
}
