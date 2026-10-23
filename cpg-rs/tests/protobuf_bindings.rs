use cpg_rs::{proto,protobuf_bindings::CanGenerateCpgBindings,schema_tools::NativeCpgSchemaContext};
use protobuf::Message;

#[test]
fn schema_derived_descriptor_matches_the_original_pinned_protocol_compiler() {
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let native=protobuf::descriptor::FileDescriptorSet::parse_from_bytes(&std::fs::read(root.join("generated/cpg-descriptor.pb")).unwrap()).unwrap();
    let reference=protobuf::descriptor::FileDescriptorSet::parse_from_bytes(&std::fs::read(root.join("../verification/agones/first-generation-protoc-reference-descriptor.pb")).expect("pinned protoc comparison must exist")).unwrap();
    assert_eq!(native,reference);
}

#[test]
fn generated_wire_bindings_preserve_unknown_fields_and_full_numeric_payloads() {
    let wire=[0x0a,0x01,b'x',0x98,0x06,0x07]; // string_value plus unknown field 99.
    let decoded=proto::PropertyValue::parse_from_bytes(&wire).unwrap();
    assert_eq!(decoded.string_value(),"x");
    assert_eq!(decoded.special_fields.unknown_fields().iter().count(),1);
    assert_eq!(decoded.write_to_bytes().unwrap(),wire);
    let mut long=proto::PropertyValue::new();long.set_long_value(i64::MIN);
    assert_eq!(proto::PropertyValue::parse_from_bytes(&long.write_to_bytes().unwrap()).unwrap().long_value(),i64::MIN);
    let mut float=proto::PropertyValue::new();float.set_float_value(f32::from_bits(0x7fc01234));
    assert_eq!(proto::PropertyValue::parse_from_bytes(&float.write_to_bytes().unwrap()).unwrap().float_value().to_bits(),0x7fc01234);
    let mut double=proto::PropertyValue::new();double.set_double_value(-0.0);
    assert_eq!(proto::PropertyValue::parse_from_bytes(&double.write_to_bytes().unwrap()).unwrap().double_value().to_bits(),(-0.0f64).to_bits());
}

#[test]
fn binding_generation_is_reproducible_and_refuses_edited_owned_outputs() {
    let first=tempfile::tempdir().unwrap();let second=tempfile::tempdir().unwrap();
    let a=NativeCpgSchemaContext.generate_cpg_bindings(first.path()).unwrap();
    let b=NativeCpgSchemaContext.generate_cpg_bindings(second.path()).unwrap();
    assert_eq!(a.files,b.files);assert_eq!(a.inputs,b.inputs);assert_eq!(a.mappings,b.mappings);
    for file in a.files.keys() {assert_eq!(std::fs::read(first.path().join(file)).unwrap(),std::fs::read(second.path().join(file)).unwrap());}
    let cached=NativeCpgSchemaContext.generate_cpg_bindings(first.path()).unwrap();assert_eq!(cached.files,a.files);
    std::fs::write(first.path().join("cpg.rs"),"user-edited binding").unwrap();
    assert!(NativeCpgSchemaContext.generate_cpg_bindings(first.path()).is_err());
}

#[test]
fn every_binding_marker_has_a_current_exact_output_range() {
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let receipt:serde_json::Value=serde_json::from_slice(&std::fs::read(root.join("generated/protobuf.json")).unwrap()).unwrap();
    let bytes=std::fs::read(root.join("generated/cpg.rs")).unwrap();
    let mappings=receipt["mappings"].as_array().unwrap();
    assert_eq!(mappings.len(),String::from_utf8_lossy(&bytes).matches("// @@protoc_insertion_point(").count());
    assert!(mappings.len()>300);
    for mapping in mappings {
        let start=mapping["startByte"].as_u64().unwrap() as usize;let end=mapping["endByteExclusive"].as_u64().unwrap() as usize;
        assert_eq!(cpg_rs::content_digest(&bytes[start..end]),mapping["sha256"]);
        let marker=format!("// @@protoc_insertion_point({})",mapping["protoSymbol"].as_str().unwrap());
        assert!(bytes[start..end].starts_with(marker.as_bytes()));
    }
}
