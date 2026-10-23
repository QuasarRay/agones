//! Native equivalent of the pinned Flatgraph ProtoGen used by CPG's handwritten Protogen.
use crate::{schema::*,codegen::snake_case,Error,Result};
use std::{collections::BTreeSet,path::{Path,PathBuf}};

fn enum_definitions<'a>(elements:impl IntoIterator<Item=&'a dyn SchemaElement>)->String {
    let mut entries:Vec<_>=elements.into_iter().filter_map(|element|element.proto_id().map(|id|(id,element))).collect();
    entries.sort_by_key(|(id,_)|*id);
    entries.into_iter().map(|(id,element)| {
        let comment=element.comment().map(|comment|format!("/* {comment} */")).unwrap_or_default();
        format!("  {comment}\n  {} = {id};\n",element.name())
    }).collect::<Vec<_>>().join("\n")
}

pub fn render(schema:&Schema)->Result<String> {
    schema.verify_proto_ids_unique()?;
    let options=schema.proto_options.as_ref().ok_or_else(||Error::Invalid("schema doesn't have any proto options configured".into()))?;
    // Preserve the observed iteration order for the selected pinned CPG, then include additional categories deterministically.
    let pinned_order:Vec<String>=serde_json::from_str(include_str!("../templates/pinned-category-order.json"))?;
    let mut emitted=BTreeSet::new();
    let mut categories=Vec::new();
    for category in pinned_order.iter().chain(schema.constants_by_category.keys()) {
        if !emitted.insert(category.clone()) {continue;}
        let Some(constants)=schema.constants_by_category.get(category) else {continue};
        if !constants.iter().any(|constant|constant.proto_id.is_some()) {continue;}
        let name=options.uncommon_proto_enum_name_mappings.get(category).unwrap_or(category);
        let unknown=snake_case(&singularize(category)).to_uppercase();
        let definitions=enum_definitions(constants.iter().map(|constant|constant as &dyn SchemaElement));
        categories.push(format!("enum {name} {{\n  UNKNOWN_{unknown} = 0;\n\n  {definitions}\n}}\n"));
    }
    let properties=enum_definitions(schema.node_properties().into_iter().map(|value|value as &dyn SchemaElement));
    let edge_properties=enum_definitions(schema.edge_properties().into_iter().map(|value|value as &dyn SchemaElement));
    let nodes=enum_definitions(schema.node_types.iter().map(|value|value as &dyn SchemaElement));
    let edges=enum_definitions(schema.edges.iter().map(|value|value as &dyn SchemaElement));
    let mut output=include_str!("../templates/cpg.proto.template").to_owned();
    for (key,value) in [
        (concat!("$","{protoOpts.pkg}"),options.package.as_str()),
        (concat!("$","{protoOpts.goPackage}"),options.go_package.as_str()),
        (concat!("$","{protoOpts.javaPackage}"),options.java_package.as_str()),
        (concat!("$","{protoOpts.javaOuterClassname}"),options.java_outer_classname.as_str()),
        (concat!("$","{protoOpts.csharpNamespace}"),options.csharp_namespace.as_str()),
        (concat!("$","{protoDefs(schema.nodeProperties.map(enumEntryMaybe))}"),properties.as_str()),
        (concat!("$","{protoDefs(schema.edgeProperties.map(enumEntryMaybe))}"),edge_properties.as_str()),
        (concat!("$","{protoDefs(schema.nodeTypes.map(enumEntryMaybe))}"),nodes.as_str()),
        (concat!("$","{protoDefs(schema.edgeTypes.map(enumEntryMaybe))}"),edges.as_str()),
        ("$enumsFromConstants",categories.join("\n").as_str()),
    ] {output=output.replace(key,value);}
    if output.contains(concat!("$","{proto")) || output.contains("$enumsFromConstants") {
        return Err(Error::Invalid("unresolved protocol template input".into()));
    }
    Ok(output)
}
pub fn write(schema:&Schema,directory:&Path)->Result<PathBuf> {
    let text=render(schema)?;
    let options=schema.proto_options.as_ref().ok_or_else(||Error::Invalid("missing protobuf package".into()))?;
    std::fs::create_dir_all(directory)?;
    let output=directory.join(format!("{}.proto",options.package));
    std::fs::write(&output,text)?;
    Ok(output)
}
