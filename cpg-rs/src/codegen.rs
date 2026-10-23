//! Schema-to-Rust domain generation. No Scala domain corpus is translated or maintained here.
use crate::{content_digest, schema::*, Error, Result};
use cgp::prelude::*;
use genco::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
use crate::construction::{SchemaConstructorGeneratorComponent,CompileSchemaConstructors};
#[path="codegen_graph.rs"]
mod graph_domain;

#[cgp_component(BootstrapLoader)]
pub trait CanLoadBootstrap { fn load_bootstrap(&self, path: &Path) -> Result<Bootstrap>; }
#[cgp_new_provider]
impl<Context> BootstrapLoader<Context> for LoadBootstrapFromFile {
    fn load_bootstrap(_context: &Context, path: &Path) -> Result<Bootstrap> {
        let model = Bootstrap::load(std::fs::File::open(path)?)?;
        model.domain_schema.validate()?;
        model.protobuf_schema.validate()?;
        Ok(model)
    }
}
#[cgp_component(DomainGenerator)]
pub trait CanGenerateDomain { fn generate_domain(&self, input: &Path, output: &Path) -> Result<DomainReport>; }
#[cgp_new_provider]
impl<Context: CanLoadBootstrap> DomainGenerator<Context> for EmitDomainWithGenco {
    fn generate_domain(context: &Context, input: &Path, output: &Path) -> Result<DomainReport> {
        let model = context.load_bootstrap(input)?;
        generate(&model.domain_schema, input, output)
    }
}
pub struct CpgToolContext;
delegate_and_check_components! {
    CpgToolContext {
        BootstrapLoaderComponent: LoadBootstrapFromFile,
        DomainGeneratorComponent: EmitDomainWithGenco,
        SchemaConstructorGeneratorComponent: CompileSchemaConstructors,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DomainReport {
    pub format_version: u32,
    pub schema_sha256: String,
    pub generator_sha256: String,
    pub schema_implementation_sha256: String,
    #[serde(default)] pub runtime_module_sha256: String,
    #[serde(default)] pub vendored_source_manifests: BTreeMap<String, String>,
    pub cargo_lock_sha256: String,
    #[serde(default)] pub cargo_manifest_sha256: String,
    pub output_sha256: String,
    pub concrete_types: usize,
    pub abstract_traits: usize,
    pub edge_types: usize,
    pub properties: usize,
    pub mappings: Vec<Value>,
    #[serde(default)] pub graph_output_sha256:String,
    #[serde(default)] pub graph_generator_sha256:String,
    #[serde(default)] pub graph_runtime_sha256:BTreeMap<String,String>,
    #[serde(default)] pub graph_api:graph_domain::GraphApiReport,
}
fn field_name(name: &str) -> String {
    let name = snake_case(name);
    match name.as_str() {
        "self"|"super"|"crate" => format!("{name}_value"),
        "as"|"async"|"await"|"break"|"const"|"continue"|"dyn"|"else"|"enum"|"extern"|"false"|"fn"|
        "for"|"if"|"impl"|"in"|"let"|"loop"|"match"|"mod"|"move"|"mut"|"pub"|"ref"|"return"|
        "static"|"struct"|"trait"|"true"|"type"|"unsafe"|"use"|"where"|"while"|"yield"|"try"|"gen" => format!("r#{name}"),
        _ => name,
    }
}
fn value_type(value: ValueType) -> rust::Tokens {
    match value {
        ValueType::Boolean => quote! { bool }, ValueType::String => quote! { String },
        ValueType::Byte => quote! { i8 }, ValueType::Short => quote! { i16 },
        ValueType::Int => quote! { i32 }, ValueType::Long => quote! { i64 },
        ValueType::Float => quote! { f32 }, ValueType::Double => quote! { f64 },
        ValueType::Char => quote! { u16 }, ValueType::NodeRef => quote! { NodeRef },
        ValueType::List => quote! { Vec<crate::schema::Scalar> }, ValueType::Unknown => quote! { crate::schema::Scalar },
    }
}
fn property_type(property: &Property) -> rust::Tokens {
    let value = value_type(property.value_type);
    match property.cardinality {
        Cardinality::ZeroOrOne => quote! { Option<$value> },
        Cardinality::List => quote! { Vec<$value> },
        Cardinality::One { .. } => value,
    }
}
fn scalar(value: &Scalar) -> Result<rust::Tokens> {
    Ok(match value {
        Scalar::String { value } => { let value = quoted(value.as_str()); quote! { $value.to_owned() } }
        Scalar::Boolean { value } => { let value = value.to_string(); quote! { $value } }
        Scalar::Byte { decimal } | Scalar::Short { decimal } | Scalar::Int { decimal } | Scalar::Long { decimal } => quote! { $decimal },
        Scalar::Float { bits } => { let bits = format!("0x{bits}u32"); quote! { f32::from_bits($bits) } }
        Scalar::Double { bits } => { let bits = format!("0x{bits}u64"); quote! { f64::from_bits($bits) } }
        Scalar::Char { utf16_code_unit } => { let value = utf16_code_unit.to_string(); quote! { $value } }
        Scalar::List { values } => {
            // ValueType.List carries heterogeneous scalar values; serde derives retain each explicit scalar kind.
            let json = quoted(serde_json::to_string(values)?);
            quote! { serde_json::from_str($json).expect("schema-validated scalar list literal") }
        }
        Scalar::Null => return Err(Error::Invalid("null scalar requires a nullable reference, not a primitive property".into())),
    })
}
fn property_default(property: &Property) -> Result<rust::Tokens> {
    match &property.cardinality {
        Cardinality::ZeroOrOne => Ok(quote! { None }),
        Cardinality::List => Ok(quote! { Vec::new() }),
        Cardinality::One { default: Some(value), .. } => scalar(value),
        _ => Err(Error::Invalid(format!("mandatory property {} has no representable default", property.name))),
    }
}
fn contained_type(contained: &ContainedNode, schema: &Schema) -> Result<rust::Tokens> {
    let node = schema.nodes().find(|node| node.name == contained.node_type)
        .ok_or_else(|| Error::Invalid(format!("missing contained type {}", contained.node_type)))?;
    let reference = if node.kind == NodeKind::Any { "NodeRef".into() } else { format!("node_refs::{}", node.class_name) };
    Ok(match contained.cardinality {
        Cardinality::List => quote! { Vec<$reference> },
        _ => quote! { Option<$reference> },
    })
}
fn header(tokens: &mut rust::Tokens, text: String) { tokens.append(text); tokens.push(); }

pub fn generate(schema: &Schema, input: &Path, output: &Path) -> Result<DomainReport> {
    schema.validate()?;
    let schema_digest = content_digest(&std::fs::read(input)?);
    let generator_digest = content_digest(include_bytes!("codegen.rs"));
    let implementation_digest = content_digest(include_bytes!("schema.rs"));
    let runtime_digest = content_digest(include_bytes!("lib.rs"));
    let lock_digest = content_digest(include_bytes!("../Cargo.lock"));
    let manifest_digest = content_digest(include_bytes!("../Cargo.toml"));
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).parent().ok_or_else(||Error::Invalid("missing repository parent".into()))?;
    let mut vendor_bindings = BTreeMap::new();
    for name in ["genco","comprende","better-comprehension"] {
        let manifest_path = repository.join(format!("verification/agones/first-generation-vendor-{name}.json"));
        let manifest_bytes = std::fs::read(manifest_path)?;
        let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
        let files = manifest["upstream_files"].as_object().ok_or_else(||Error::Invalid("vendor input lacks file identities".into()))?;
        for (path, record) in files {
            let source = repository.join("third_party/first-generation").join(name).join(path);
            if content_digest(&std::fs::read(source)?) != record["sha256"].as_str().unwrap_or("") {
                return Err(Error::Invalid(format!("changed {name} source input {path}")));
            }
        }
        vendor_bindings.insert(name.into(),content_digest(&manifest_bytes));
    }
    let domain_path = output.join("domain.rs");
    let receipt_path = output.join("generation.json");
    if receipt_path.exists() {
        let prior: DomainReport = serde_json::from_slice(&std::fs::read(&receipt_path)?)?;
        if content_digest(&std::fs::read(&domain_path)?) != prior.output_sha256 {
            return Err(Error::Invalid("generated CPG domain was edited; preserve the changes before regeneration".into()));
        }
        if prior.schema_sha256 == schema_digest && prior.generator_sha256 == generator_digest &&
           prior.schema_implementation_sha256 == implementation_digest && prior.runtime_module_sha256 == runtime_digest &&
           prior.vendored_source_manifests == vendor_bindings && prior.cargo_lock_sha256 == lock_digest &&
           prior.cargo_manifest_sha256 == manifest_digest { return Ok(prior); }
    } else if domain_path.exists() {
        return Err(Error::Invalid("unowned domain output already exists".into()));
    }
    let properties: BTreeMap<_,_> = schema.properties.iter().map(|property| (property.name.as_str(),property)).collect();
    let bases: BTreeMap<_,_> = schema.node_base_types.iter().map(|node| (node.name.as_str(),node)).collect();
    let mut tokens: rust::Tokens = rust::Tokens::new();
    header(&mut tokens, "// Generated CPG domain from the complete pinned schema. Do not edit.".into());
    header(&mut tokens, format!("// Schema input SHA-256: {schema_digest}"));
    quote_in! { tokens =>
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub struct NodeRef(pub u64);
    };
    let mut references = rust::Tokens::new();
    for node in schema.node_base_types.iter().chain(&schema.node_types) {
        let name = node.class_name.as_str();
        quote_in! { references =>
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
            pub struct $name(pub u64);
        };
    }
    quote_in! { tokens => pub mod node_refs { $references } };
    for base in &schema.node_base_types {
        let trait_name = format!("{}Properties", base.class_name);
        let mut methods = rust::Tokens::new();
        for name in &base.all_properties {
            let property = properties[name.as_str()];
            let name = field_name(&property.name); let ty = property_type(property);
            quote_in! { methods => fn $name(&self) -> &$ty; };
            methods.push();
        }
        quote_in! { tokens =>
            pub trait $trait_name { $methods }
        };
    }
    for (kind,node) in schema.node_types.iter().enumerate() {
        header(&mut tokens, format!("// CPG_SCHEMA_NODE:{}",node.name));
        let name = &node.class_name; let index = kind.to_string();
        let label = quoted(node.name.as_str());
        let mut fields = rust::Tokens::new();
        let mut defaults = rust::Tokens::new();
        for property_name in &node.all_properties {
            let property = properties[property_name.as_str()];
            let field_string = field_name(property_name); let field = field_string.as_str(); let key = quoted(property_name.as_str());
            let ty = property_type(property); let default = property_default(property)?;
            quote_in! { fields => #[serde(rename = $key)] pub $field: $ty, };
            quote_in! { defaults => $field: $default, };
            fields.push(); defaults.push();
        }
        for contained in &node.contained_nodes {
            let field_string = field_name(&contained.local_name); let field = field_string.as_str(); let key = quoted(contained.local_name.as_str());
            let ty = contained_type(contained,schema)?;
            let default: rust::Tokens = if matches!(contained.cardinality,Cardinality::List) { quote! { Vec::new() } } else { quote! { None } };
            quote_in! { fields => #[serde(rename = $key)] pub $field: $ty, };
            quote_in! { defaults => $field: $default, };
            fields.push(); defaults.push();
        }
        quote_in! { tokens =>
            #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
            #[serde(default, deny_unknown_fields)]
            pub struct $name {
                pub node_id: Option<u64>,
                $fields
            }
            impl Default for $name {
                fn default() -> Self { Self { node_id: None, $defaults } }
            }
            impl $name {
                pub const NODE_KIND: u16 = $index;
                pub const LABEL: &'static str = $label;
            }
        };
        for ancestor in &node.ancestors {
            let base = bases[ancestor.as_str()];
            let trait_name = format!("{}Properties",base.class_name);
            let mut getters = rust::Tokens::new();
            for property_name in &base.all_properties {
                let property = properties[property_name.as_str()];
                let field_string = field_name(property_name); let field = field_string.as_str(); let ty = property_type(property);
                quote_in! { getters => fn $field(&self) -> &$ty { &self.$field } };
                getters.push();
            }
            quote_in! { tokens => impl $trait_name for $name { $getters } };
        }
    }
    for (kind,edge) in schema.edges.iter().enumerate() {
        header(&mut tokens,format!("// CPG_SCHEMA_EDGE:{}",edge.name));
        let name_string = format!("{}Edge",edge.class_name); let name = name_string.as_str();
        let index = kind.to_string(); let label = quoted(edge.name.as_str());
        let mut fields = rust::Tokens::new();
        if let Some(property) = edge.property.as_ref().map(|name| properties[name.as_str()]) {
            let field = field_name(&property.name); let ty = property_type(property); let key = quoted(property.name.as_str());
            quote_in! { fields => #[serde(rename = $key)] pub $field: $ty, };
        }
        quote_in! { tokens =>
            #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            pub struct $name {
                pub edge_id: Option<u64>, pub source: NodeRef, pub target: NodeRef, $fields
            }
            impl $name { pub const EDGE_KIND: u16 = $index; pub const LABEL: &'static str = $label; }
        };
    }
    header(&mut tokens,"// CPG_SCHEMA_CONSTANTS".into());
    for (module,values) in [("node_types",schema.node_types.iter().map(|n| &n.name).collect::<Vec<_>>()),
                          ("edge_types",schema.edges.iter().map(|e| &e.name).collect()),
                          ("property_names",schema.properties.iter().map(|p| &p.name).collect())] {
        let mut constants = rust::Tokens::new();
        for value in values { let name = snake_case(value).to_uppercase(); let value = quoted(value.as_str());
            quote_in! { constants => pub const $name: &str = $value; };
            constants.push();
        }
        quote_in! { tokens => pub mod $module { $constants } };
    }
    for (category,values) in &schema.constants_by_category {
        let module = snake_case(category); let mut constants = rust::Tokens::new();
        for value in values {
            let name = snake_case(&value.name).to_uppercase(); let value = quoted(value.value.as_str());
            quote_in! { constants => pub const $name: &str = $value; };
            constants.push();
        }
        quote_in! { tokens => pub mod $module { $constants } };
    }
    let rendered = tokens.to_string().map_err(|error| Error::Invalid(error.to_string()))?;
    let mut mappings = Vec::new();
    for (position,node) in schema.node_types.iter().enumerate() {
        let marker = format!("// CPG_SCHEMA_NODE:{}",node.name);
        let next = schema.node_types.get(position+1).map(|node| format!("// CPG_SCHEMA_NODE:{}",node.name))
            .unwrap_or_else(||schema.edges.first().map(|edge|format!("// CPG_SCHEMA_EDGE:{}",edge.name))
                .unwrap_or_else(||"// CPG_SCHEMA_CONSTANTS".into()));
        let start = rendered.find(&marker).ok_or_else(||Error::Invalid("Genco omitted node marker".into()))?;
        let end = rendered[start+marker.len()..].find(&next).map(|index|start+marker.len()+index)
            .ok_or_else(||Error::Invalid("Genco omitted section boundary".into()))?;
        mappings.push(json!({"schemaNode":node.name,"symbol":node.class_name,"path":"domain.rs",
            "startByte":start,"endByteExclusive":end,"sha256":content_digest(&rendered.as_bytes()[start..end]),
            "properties":node.all_properties,"containedNodes":node.contained_nodes,"schemaInfo":node.schema_info}));
    }
    for (position,edge) in schema.edges.iter().enumerate() {
        let marker = format!("// CPG_SCHEMA_EDGE:{}",edge.name);
        let next = schema.edges.get(position+1).map(|edge|format!("// CPG_SCHEMA_EDGE:{}",edge.name))
            .unwrap_or_else(||"// CPG_SCHEMA_CONSTANTS".into());
        let start = rendered.find(&marker).ok_or_else(||Error::Invalid("Genco omitted edge marker".into()))?;
        let end = rendered[start+marker.len()..].find(&next).map(|index|start+marker.len()+index)
            .ok_or_else(||Error::Invalid("Genco omitted edge boundary".into()))?;
        mappings.push(json!({"schemaEdge":edge.name,"symbol":format!("{}Edge",edge.class_name),"path":"domain.rs",
            "startByte":start,"endByteExclusive":end,"sha256":content_digest(&rendered.as_bytes()[start..end]),
            "property":edge.property,"schemaInfo":edge.schema_info}));
    }
    let report = DomainReport { format_version:2,schema_sha256:schema_digest,generator_sha256:generator_digest,
        schema_implementation_sha256:implementation_digest,cargo_lock_sha256:lock_digest,
        runtime_module_sha256:runtime_digest,vendored_source_manifests:vendor_bindings,
        cargo_manifest_sha256:manifest_digest,
        output_sha256:content_digest(rendered.as_bytes()),concrete_types:schema.node_types.len(),
        abstract_traits:schema.node_base_types.len(),edge_types:schema.edges.len(),properties:schema.properties.len(),mappings };
    std::fs::create_dir_all(output)?;
    let temporary = output.join("domain.rs.tmp");
    if temporary.exists() { return Err(Error::Invalid("recover existing incomplete CPG domain output".into())); }
    std::fs::write(&temporary,rendered)?;
    std::fs::rename(temporary,&domain_path)?;
    std::fs::write(&receipt_path,serde_json::to_vec_pretty(&report)?)?;
    let guide = output.join("AGENTS.MD");
    if !guide.exists() { std::fs::write(guide,"# AGENTS.MD\n\nProject guidance is inherited from [the repository root](../../AGENTS.MD).\nThis directory is generated CPG tooling, not the terminal Agones output.\n")?; }
    Ok(report)
}

pub fn snake_case(value: &str) -> String {
    let chars: Vec<_> = value.chars().collect(); let mut out = String::new(); let mut index = 0;
    while index < chars.len() {
        if index+2 < chars.len() && chars[index].is_uppercase() && chars[index+1].is_uppercase() && chars[index+2].is_lowercase() {
            out.push(chars[index]); out.push('_'); out.push(chars[index+1]); out.push(chars[index+2]); index += 3;
        } else if index+1 < chars.len() && chars[index].is_lowercase() && chars[index+1].is_uppercase() {
            out.push(chars[index]); out.push('_'); out.push(chars[index+1]); index += 2;
        } else { out.push(chars[index]); index += 1; }
    }
    out.to_lowercase()
}
