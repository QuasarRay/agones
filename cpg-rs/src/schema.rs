//! Flatgraph schema semantics used by every handwritten CPG schema module.
//! Source: pinned flatgraph/schema/{Schema,SchemaBuilder,Helpers}.scala and CPG/schema.
use crate::{Error, Result};
use better_comprehension::b_tree_map;
use comprende::c;
use derive_deftly::{Deftly, define_derive_deftly};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SchemaInfo {
    pub definition_class: String,
    pub schema_class: String,
    pub schema_name: String,
    pub doc_index: i32,
    pub description: String,
    pub provided_by_frontend: bool,
}

/// All defaults in handwritten CpgSchema.PropertyDefaults, including types absent from today's CPG fields.
pub mod property_defaults {
    pub const STRING: &str = "<empty>";
    pub const BOOLEAN: bool = false;
    pub const BYTE: i8 = -1;
    pub const SHORT: i16 = -1;
    pub const INT: i32 = -1;
    pub const LONG: i64 = -1;
    pub const FLOAT: f32 = 0.0;
    pub const DOUBLE: f64 = 0.0;
    pub const CHAR: u16 = 0x3f;
}

pub trait SchemaBase {
    fn doc_index(&self) -> i32;
    fn description(&self) -> &str;
    fn provided_by_frontend(&self) -> bool { false }
}
impl SchemaBase for SchemaInfo {
    fn doc_index(&self) -> i32 { self.doc_index }
    fn description(&self) -> &str { &self.description }
    fn provided_by_frontend(&self) -> bool { self.provided_by_frontend }
}

pub trait SchemaElement {
    fn name(&self) -> &str;
    fn proto_id(&self) -> Option<i32>;
    fn schema_info(&self) -> Option<&SchemaInfo>;
    fn comment(&self) -> Option<&str>;
    fn is_hidden(&self) -> bool {
        self.schema_info().is_some_and(|info| info.schema_name == "Hidden")
    }
}
define_derive_deftly! {
    SchemaElement:
    impl SchemaElement for $ttype {
        fn name(&self) -> &str { &self.name }
        fn proto_id(&self) -> Option<i32> { self.proto_id }
        fn schema_info(&self) -> Option<&SchemaInfo> { self.schema_info.as_ref() }
        fn comment(&self) -> Option<&str> { self.comment.as_deref() }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType { Boolean, String, Byte, Short, Int, Long, Float, Double, List, Char, NodeRef, Unknown }

/// Decimal strings and bit patterns retain precision independently of a JSON number implementation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Scalar {
    Null,
    String { value: String },
    Boolean { value: bool },
    Byte { decimal: String },
    Short { decimal: String },
    Int { decimal: String },
    Long { decimal: String },
    Float { bits: String },
    Double { bits: String },
    Char { #[serde(rename = "utf16CodeUnit")] utf16_code_unit: u16 },
    List { values: Vec<Scalar> },
}
impl Scalar {
    pub fn matches(&self, value_type: ValueType) -> bool {
        match (self, value_type) {
            (Self::Null, ValueType::NodeRef | ValueType::Unknown) => true,
            (Self::String { .. }, ValueType::String) | (Self::Boolean { .. }, ValueType::Boolean) |
            (Self::Char { .. }, ValueType::Char) | (Self::List { .. }, ValueType::List) => true,
            (Self::Byte { decimal }, ValueType::Byte) => decimal.parse::<i8>().is_ok(),
            (Self::Short { decimal }, ValueType::Short) => decimal.parse::<i16>().is_ok(),
            (Self::Int { decimal }, ValueType::Int) => decimal.parse::<i32>().is_ok(),
            (Self::Long { decimal }, ValueType::Long) => decimal.parse::<i64>().is_ok(),
            (Self::Float { bits }, ValueType::Float) => u32::from_str_radix(bits, 16).is_ok(),
            (Self::Double { bits }, ValueType::Double) => u64::from_str_radix(bits, 16).is_ok(),
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Cardinality {
    ZeroOrOne, List,
    One {
        #[serde(rename = "defaultWrapperIsNull")]
        default_wrapper_is_null: bool,
        default: Option<Scalar>,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeCardinality { One, ZeroOrOne, List }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind { Any, Base, Concrete }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Deftly)]
#[derive_deftly(SchemaElement)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Property {
    pub name: String,
    pub class_name: String,
    pub value_type: ValueType,
    pub comment: Option<String>,
    pub schema_info: Option<SchemaInfo>,
    pub proto_id: Option<i32>,
    pub cardinality: Cardinality,
    pub mandatory: bool,
    pub has_default: bool,
    pub default: Option<Scalar>,
}
impl Property {
    pub fn new(name: String, value_type: ValueType, comment: &str, schema_info: Option<SchemaInfo>) -> Self {
        Self { class_name: camel_case_caps(&name), name, value_type, comment: string_to_option(comment),
               schema_info, proto_id: None, cardinality: Cardinality::ZeroOrOne,
               mandatory: false, has_default: false, default: None }
    }
    pub fn mandatory(&mut self, default: Scalar) -> Result<&mut Self> {
        if !default.matches(self.value_type) { return Err(Error::Invalid(format!("invalid default for {}", self.name))); }
        self.cardinality = Cardinality::One { default_wrapper_is_null: false, default: Some(default.clone()) };
        self.mandatory = true;
        self.has_default = true;
        self.default = Some(default);
        Ok(self)
    }
    pub fn as_list(&mut self) -> &mut Self {
        self.cardinality = Cardinality::List;
        self.mandatory = false;
        self.has_default = false;
        self.default = None;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdjacentNode {
    pub edge: String,
    pub neighbor: String,
    pub cardinality: EdgeCardinality,
    pub custom_step_name: Option<String>,
    pub custom_step_doc: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainedNode {
    #[serde(rename = "type")] pub node_type: String,
    pub local_name: String,
    pub cardinality: Cardinality,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Deftly)]
#[derive_deftly(SchemaElement)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeType {
    pub name: String,
    pub class_name: String,
    pub kind: NodeKind,
    pub comment: Option<String>,
    pub schema_info: Option<SchemaInfo>,
    pub proto_id: Option<i32>,
    pub starter_name: Option<String>,
    #[serde(rename = "extends")] pub bases: Vec<String>,
    pub ancestors: Vec<String>,
    pub declared_properties: Vec<String>,
    pub all_properties: Vec<String>,
    pub out_edges: Vec<AdjacentNode>,
    pub in_edges: Vec<AdjacentNode>,
    pub marker_traits: Vec<String>,
    pub primary_key: Option<String>,
    pub contained_nodes: Vec<ContainedNode>,
}
impl NodeType {
    pub fn new(name: String, kind: NodeKind, comment: &str, schema_info: Option<SchemaInfo>) -> Self {
        Self { class_name: camel_case_caps(&name), starter_name: Some(camel_case(&name)), name,
               kind, comment: string_to_option(comment), schema_info, proto_id: None, bases: vec![],
               ancestors: vec![], declared_properties: vec![], all_properties: vec![],
               out_edges: vec![], in_edges: vec![], marker_traits: vec![], primary_key: None, contained_nodes: vec![] }
    }
    pub fn add_property(&mut self, property: String) -> &mut Self {
        if !self.declared_properties.contains(&property) { self.declared_properties.push(property); }
        self
    }
    pub fn extend(&mut self, base: String) -> &mut Self {
        if !self.bases.contains(&base) { self.bases.push(base); }
        self
    }
    pub fn without_starter(&mut self) -> &mut Self { self.starter_name = None; self }
    pub fn class_name_db(&self) -> String { format!("{}Db", self.class_name) }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Deftly)]
#[derive_deftly(SchemaElement)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EdgeType {
    pub name: String,
    pub class_name: String,
    pub proto_id: Option<i32>,
    pub comment: Option<String>,
    pub schema_info: Option<SchemaInfo>,
    pub property: Option<String>,
    pub default_accessor_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Deftly)]
#[derive_deftly(SchemaElement)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Constant {
    pub name: String,
    pub value: String,
    pub value_type: ValueType,
    pub proto_id: Option<i32>,
    pub comment: Option<String>,
    pub schema_info: Option<SchemaInfo>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtoOptions {
    pub package: String, pub java_outer_classname: String, pub java_package: String,
    pub go_package: String, pub csharp_namespace: String,
    pub uncommon_proto_enum_name_mappings: BTreeMap<String, String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropertyKind { pub name: String, pub index: usize }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropertyWarningException { pub node: String, pub property: String }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Schema {
    pub domain_short_name: String,
    pub base_package: String,
    pub additional_traversals_packages: Vec<String>,
    pub properties: Vec<Property>,
    pub node_property_kinds: Vec<PropertyKind>,
    pub edge_property_names: Vec<String>,
    pub any_node: NodeType,
    pub node_base_types: Vec<NodeType>,
    pub node_types: Vec<NodeType>,
    pub edges: Vec<EdgeType>,
    pub constants_by_category: BTreeMap<String, Vec<Constant>>,
    pub proto_options: Option<ProtoOptions>,
    pub duplicate_property_warning_exceptions: Vec<PropertyWarningException>,
}
impl Schema {
    pub fn nodes(&self) -> impl Iterator<Item=&NodeType> {
        std::iter::once(&self.any_node).chain(&self.node_base_types).chain(&self.node_types)
    }
    pub fn named_nodes(&self) -> BTreeMap<&str, &NodeType> {
        b_tree_map! { node.name.as_str() => node
            for node in self.node_base_types.iter().chain(&self.node_types).chain(std::iter::once(&self.any_node)) }
    }
    pub fn node_properties(&self) -> Vec<&Property> {
        let names: BTreeSet<_> = self.node_base_types.iter().chain(&self.node_types)
            .flat_map(|node| node.all_properties.iter().map(String::as_str)).collect();
        c![property for property in &self.properties if names.contains(property.name.as_str())]
    }
    pub fn edge_properties(&self) -> Vec<&Property> {
        let names: BTreeSet<_> = self.edges.iter().filter_map(|edge| edge.property.as_deref()).collect();
        self.properties.iter().filter(|property| names.contains(property.name.as_str())).collect()
    }
    pub fn subtypes(&self, name: &str) -> Vec<&NodeType> {
        if self.any_node.name == name { return self.node_types.iter().chain(&self.node_base_types).collect(); }
        self.node_types.iter().chain(&self.node_base_types).filter(|node| node.ancestors.iter().any(|base| base == name)).collect()
    }
    pub fn verify_proto_ids_unique(&self) -> Result<()> {
        fn verify<'a>(category: &str, values: impl IntoIterator<Item=&'a dyn SchemaElement>) -> Result<()> {
            let mut ids: BTreeMap<i32, Vec<&str>> = BTreeMap::new();
            for value in values { if let Some(id) = value.proto_id() { ids.entry(id).or_default().push(value.name()); } }
            let duplicates: Vec<_> = ids.into_iter().filter(|(_, names)| names.len() > 1).collect();
            if !duplicates.is_empty() { return Err(Error::Invalid(format!("duplicate proto ids in {category}: {duplicates:?}"))); }
            Ok(())
        }
        verify("node properties", self.node_properties().into_iter().map(|p| p as &dyn SchemaElement))?;
        verify("edge properties", self.edge_properties().into_iter().map(|p| p as &dyn SchemaElement))?;
        verify("node types", self.node_types.iter().map(|p| p as &dyn SchemaElement))?;
        verify("edge types", self.edges.iter().map(|p| p as &dyn SchemaElement))?;
        for (category, constants) in &self.constants_by_category {
            verify(category, constants.iter().map(|p| p as &dyn SchemaElement))?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Bootstrap { pub format_version: u32, pub domain_schema: Schema, pub protobuf_schema: Schema }
impl Bootstrap {
    pub fn load(reader: impl std::io::Read) -> Result<Self> {
        let model: Self = serde_json::from_reader(reader)?;
        if model.format_version != 1 { return Err(Error::Invalid(format!("unknown schema format {}", model.format_version))); }
        model.domain_schema.verify_proto_ids_unique()?;
        model.protobuf_schema.verify_proto_ids_unique()?;
        Ok(model)
    }
}

pub fn string_to_option(value: &str) -> Option<String> {
    // java.lang.String.trim removes only code units <= U+0020.
    let trimmed = value.trim_matches(|c: char| c <= '\u{20}');
    if trimmed.is_empty() { None } else { Some(trimmed.to_owned()) }
}
pub fn singularize(value: &str) -> String {
    if let Some(prefix) = value.strip_suffix("ies") { format!("{prefix}y") }
    else { let mut chars = value.chars(); chars.next_back(); chars.as_str().to_owned() }
}
pub fn camel_case(value: &str) -> String {
    let value = value.strip_prefix('_').unwrap_or(value);
    value.split('_').enumerate().map(|(index, part)| {
        let part = part.to_lowercase();
        if index == 0 { part } else { capitalize(&part) }
    }).collect()
}
pub fn camel_case_caps(value: &str) -> String { capitalize(&camel_case(value)) }
fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    chars.next().map(|c| c.to_uppercase().collect::<String>() + chars.as_str()).unwrap_or_default()
}

/// Name-resolved model construction and normalization. Source schema composition
/// uses ObjectSchemaBuilder, whose typed handles preserve aliases and duplicate names.
pub struct SchemaBuilder { schema: Schema }

#[path="schema_objects.rs"]
pub mod objects;
pub use objects::{ObjectSchemaBuilder, ObjectSchema, SchemaObjectId, PropertyRef, NodeRef, EdgeRef, ConstantRef, AdjacentNodeRef, ContainedNodeRef};

impl SchemaBuilder {
    pub fn new(domain_short_name: String, base_package: String, additional_traversals_packages: Vec<String>) -> Self {
        let mut any_node = NodeType::new("AnyNode".into(), NodeKind::Any,
            "generic node base trait - use if you want to be explicitly unspecific", None);
        any_node.class_name = "StoredNode".into();
        Self { schema: Schema { domain_short_name, base_package, additional_traversals_packages,
            properties: vec![], node_property_kinds: vec![], edge_property_names: vec![], any_node,
            node_base_types: vec![], node_types: vec![], edges: vec![], constants_by_category: BTreeMap::new(),
            proto_options: None, duplicate_property_warning_exceptions: vec![] } }
    }
    pub fn from_schema(schema: Schema) -> Self { Self { schema } }
    pub fn add_property(&mut self, name: String, value_type: ValueType, comment: &str, info: Option<SchemaInfo>) -> &mut Property {
        self.schema.properties.push(Property::new(name, value_type, comment, info));
        self.schema.properties.last_mut().expect("just appended property")
    }
    pub fn add_node_base_type(&mut self, name: String, comment: &str, info: Option<SchemaInfo>) -> &mut NodeType {
        self.schema.node_base_types.push(NodeType::new(name, NodeKind::Base, comment, info));
        self.schema.node_base_types.last_mut().expect("just appended base")
    }
    pub fn add_node_type(&mut self, name: String, comment: &str, info: Option<SchemaInfo>) -> &mut NodeType {
        self.schema.node_types.push(NodeType::new(name, NodeKind::Concrete, comment, info));
        self.schema.node_types.last_mut().expect("just appended node")
    }
    pub fn add_edge_type(&mut self, name: String, comment: &str, info: Option<SchemaInfo>) -> &mut EdgeType {
        self.schema.edges.push(EdgeType { class_name: camel_case_caps(&name), name, comment: string_to_option(comment),
            schema_info: info, proto_id: None, property: None, default_accessor_name: None });
        self.schema.edges.last_mut().expect("just appended edge")
    }
    pub fn add_constants(&mut self, category: String, values: impl IntoIterator<Item=Constant>) -> &mut Self {
        self.schema.constants_by_category.entry(category).or_default().extend(values); self
    }
    pub fn proto_options(&mut self, options: ProtoOptions) -> &mut Self { self.schema.proto_options = Some(options); self }
    pub fn dont_warn_for_duplicate_property(&mut self, node: String, property: String) -> &mut Self {
        let value = PropertyWarningException { node, property };
        if !self.schema.duplicate_property_warning_exceptions.contains(&value) {
            self.schema.duplicate_property_warning_exceptions.push(value);
        }
        self
    }
    pub fn property_mut(&mut self, name: &str) -> Result<&mut Property> {
        self.schema.properties.iter_mut().find(|property| property.name == name)
            .ok_or_else(|| Error::Invalid(format!("unknown property {name}")))
    }
    pub fn node_mut(&mut self, name: &str) -> Result<&mut NodeType> {
        std::iter::once(&mut self.schema.any_node).chain(&mut self.schema.node_base_types).chain(&mut self.schema.node_types)
            .find(|node| node.name == name).ok_or_else(|| Error::Invalid(format!("unknown node {name}")))
    }
    pub fn edge_mut(&mut self, name: &str) -> Result<&mut EdgeType> {
        self.schema.edges.iter_mut().find(|edge| edge.name == name)
            .ok_or_else(|| Error::Invalid(format!("unknown edge {name}")))
    }
    pub fn connect(&mut self, edge: &str, out: &str, incoming: &str, options: EdgeConnection) -> Result<&mut Self> {
        if !self.schema.edges.iter().any(|candidate| candidate.name == edge) {
            return Err(Error::Invalid(format!("unknown edge {edge}")));
        }
        for name in [out, incoming] {
            if !self.schema.nodes().any(|node| node.name == name) {
                return Err(Error::Invalid(format!("unknown node {name}")));
            }
        }
        let forward = AdjacentNode { edge: edge.into(), neighbor: incoming.into(), cardinality: options.out_cardinality,
            custom_step_name: string_to_option(&options.out_step), custom_step_doc: string_to_option(&options.out_doc) };
        let backward = AdjacentNode { edge: edge.into(), neighbor: out.into(), cardinality: options.in_cardinality,
            custom_step_name: string_to_option(&options.in_step), custom_step_doc: string_to_option(&options.in_doc) };
        let source = self.node_mut(out)?;
        if !source.out_edges.contains(&forward) { source.out_edges.push(forward); }
        let target = self.node_mut(incoming)?;
        if !target.in_edges.contains(&backward) { target.in_edges.push(backward); }
        Ok(self)
    }
    pub fn build(mut self) -> Result<Schema> {
        self.schema.properties.sort_by_key(|value| value.name.to_lowercase());
        self.schema.node_base_types.sort_by_key(|value| value.name.to_lowercase());
        self.schema.node_types.sort_by_key(|value| value.name.to_lowercase());
        self.schema.edges.sort_by_key(|value| value.name.to_lowercase());
        // A name is the stable handle in this Rust API; reject ambiguous handles before resolving references.
        unique_names("properties", self.schema.properties.iter().map(|p| p.name.as_str()))?;
        unique_names("nodes", self.schema.nodes().map(|n| n.name.as_str()))?;
        unique_names("edges", self.schema.edges.iter().map(|e| e.name.as_str()))?;
        for property in &mut self.schema.properties {
            let default = match &property.cardinality {
                Cardinality::One { default_wrapper_is_null: false, default } => default.clone(),
                _ => None,
            };
            if default.as_ref().is_some_and(|value| !value.matches(property.value_type)) {
                return Err(Error::Invalid(format!("invalid default for {}", property.name)));
            }
            property.mandatory = matches!(property.cardinality, Cardinality::One { .. });
            property.has_default = default.is_some();
            property.default = default;
        }
        for node in std::iter::once(&mut self.schema.any_node).chain(&mut self.schema.node_base_types).chain(&mut self.schema.node_types) {
            node.bases.sort(); node.bases.dedup();
            node.declared_properties.sort_by_key(|value| value.to_lowercase()); node.declared_properties.dedup();
            node.contained_nodes.sort_by_key(|value| value.local_name.to_lowercase()); node.contained_nodes.dedup();
            node.marker_traits.sort(); node.marker_traits.dedup();
            order_adjacencies(&mut node.out_edges)?; order_adjacencies(&mut node.in_edges)?;
        }
        let nodes = self.schema.named_nodes();
        let property_names: BTreeSet<_> = self.schema.properties.iter().map(|p| p.name.as_str()).collect();
        let edge_names: BTreeSet<_> = self.schema.edges.iter().map(|p| p.name.as_str()).collect();
        let mut derived = Vec::new();
        for node in self.schema.nodes() {
            let ancestors = ancestors(&nodes, &node.name, &mut BTreeSet::new())?;
            let mut properties = Vec::new();
            for name in std::iter::once(&node.name).chain(&ancestors) {
                let parent = nodes[name.as_str()];
                for property in &parent.declared_properties {
                    if !property_names.contains(property.as_str()) { return Err(Error::Invalid(format!("unknown property {property} on {name}"))); }
                    if !properties.contains(property) { properties.push(property.clone()); }
                }
            }
            properties.sort_by_key(|value| value.to_lowercase());
            for edge in node.out_edges.iter().chain(&node.in_edges) {
                if !edge_names.contains(edge.edge.as_str()) || !nodes.contains_key(edge.neighbor.as_str()) {
                    return Err(Error::Invalid(format!("unresolved adjacency on {}", node.name)));
                }
            }
            for contained in &node.contained_nodes {
                if !nodes.contains_key(contained.node_type.as_str()) { return Err(Error::Invalid(format!("unknown contained node {}", contained.node_type))); }
            }
            if node.primary_key.as_ref().is_some_and(|key| !properties.contains(key)) {
                return Err(Error::Invalid(format!("unknown primary key on {}", node.name)));
            }
            derived.push((node.name.clone(), ancestors, properties));
        }
        for edge in &self.schema.edges {
            if edge.property.as_ref().is_some_and(|name| !property_names.contains(name.as_str())) {
                return Err(Error::Invalid(format!("unknown property on edge {}", edge.name)));
            }
        }
        drop(nodes);
        for (name, ancestors, properties) in derived {
            let node = self.node_mut(&name)?; node.ancestors = ancestors; node.all_properties = properties;
        }
        self.schema.node_property_kinds = self.schema.node_properties().iter().enumerate()
            .map(|(index, property)| PropertyKind { name: property.name.clone(), index }).collect();
        self.schema.edge_property_names = self.schema.edge_properties().iter().map(|property| property.name.clone()).collect();
        self.schema.duplicate_property_warning_exceptions.sort_by(|a, b| (&a.node, &a.property).cmp(&(&b.node, &b.property)));
        self.schema.duplicate_property_warning_exceptions.dedup();
        self.schema.verify_proto_ids_unique()?;
        Ok(self.schema)
    }
}

pub struct EdgeConnection {
    pub out_cardinality: EdgeCardinality, pub in_cardinality: EdgeCardinality,
    pub out_step: String, pub in_step: String, pub out_doc: String, pub in_doc: String,
}
impl Default for EdgeConnection {
    fn default() -> Self {
        Self { out_cardinality: EdgeCardinality::List, in_cardinality: EdgeCardinality::List,
               out_step: String::new(), in_step: String::new(), out_doc: String::new(), in_doc: String::new() }
    }
}
fn unique_names<'a>(category: &str, names: impl Iterator<Item=&'a str>) -> Result<()> {
    let mut seen = BTreeSet::new();
    for name in names { if !seen.insert(name) { return Err(Error::Invalid(format!("ambiguous {category} name {name}"))); } }
    Ok(())
}
fn order_adjacencies(values: &mut Vec<AdjacentNode>) -> Result<()> {
    let mut indexed: Vec<_> = values.drain(..).map(|value| Ok((serde_json::to_string(&value)?, value))).collect::<Result<_>>()?;
    indexed.sort_by(|a,b| a.0.cmp(&b.0));
    *values = indexed.into_iter().map(|(_, value)| value).collect(); values.dedup(); Ok(())
}
fn ancestors(nodes: &BTreeMap<&str, &NodeType>, name: &str, visiting: &mut BTreeSet<String>) -> Result<Vec<String>> {
    if !visiting.insert(name.into()) { return Err(Error::Invalid(format!("inheritance cycle at {name}"))); }
    let node = nodes.get(name).ok_or_else(|| Error::Invalid(format!("unknown base {name}")))?;
    let mut result = node.bases.clone();
    for base in &node.bases {
        let parent = nodes.get(base.as_str()).ok_or_else(|| Error::Invalid(format!("unknown base {base}")))?;
        if parent.kind == NodeKind::Concrete { return Err(Error::Invalid(format!("concrete node {base} used as base"))); }
        for ancestor in ancestors(nodes, base, visiting)? {
            if !result.contains(&ancestor) { result.push(ancestor); }
        }
    }
    visiting.remove(name); Ok(result)
}

impl Schema {
    /// Reconcile observations instead of silently accepting inconsistent derived fields.
    pub fn validate(&self) -> Result<()> {
        let rebuilt = SchemaBuilder::from_schema(self.clone()).build()?;
        if &rebuilt != self { return Err(Error::Invalid("schema contains inconsistent ordering, inheritance, property kinds or defaults".into())); }
        Ok(())
    }
    /// Source-compatible public documentation export; the internal model retains the hidden metadata this omits.
    pub fn standard_json(&self) -> serde_json::Value {
        use serde_json::{Value, json};
        fn index(value: &impl SchemaElement) -> i32 { value.schema_info().map_or(i32::MAX, |info| info.doc_index) }
        fn schema_name(value: &impl SchemaElement) -> &str { value.schema_info().map_or("unknown", |info| info.schema_name.as_str()) }
        fn card(value: &Cardinality) -> &'static str {
            match value { Cardinality::ZeroOrOne => "zeroOrOne", Cardinality::List => "list", Cardinality::One { .. } => "one" }
        }
        let properties: BTreeMap<_, _> = self.properties.iter().map(|p| (p.name.as_str(), p)).collect();
        let nodes = self.named_nodes();
        let mut node_values: Vec<_> = self.node_types.iter().chain(&self.node_base_types).filter(|node| !node.is_hidden()).collect();
        node_values.sort_by(|a,b| (index(*a), &a.name).cmp(&(index(*b), &b.name)));
        let exported_nodes: Vec<Value> = node_values.into_iter().map(|node| {
            let visible: Vec<_> = node.all_properties.iter().filter(|name| !properties[name.as_str()].is_hidden()).collect();
            let mut inherited = BTreeMap::new();
            for base in &node.bases { for name in &nodes[base.as_str()].all_properties { inherited.insert(name, base); } }
            let mut value = json!({"name":node.name, "extends":node.bases, "allProperties":visible,
                "cardinalities":visible.iter().map(|name| card(&properties[name.as_str()].cardinality)).collect::<Vec<_>>(),
                "inheritedProperties":inherited.iter().map(|(name, base)| json!({"baseType":base,"name":name})).collect::<Vec<_>>(),
                "properties":visible.iter().filter(|name| !inherited.contains_key(*name)).collect::<Vec<_>>(),
                "schema":schema_name(node), "schemaIndex":index(node), "isAbstract":node.kind != NodeKind::Concrete,
                "containedNodes":if node.kind == NodeKind::Concrete { node.contained_nodes.iter().map(|child|
                    json!({"name":child.local_name,"type":child.node_type,"cardinality":card(&child.cardinality)})).collect::<Vec<_>>() } else { vec![] }});
            if let Some(comment) = &node.comment { value["comment"] = json!(comment); }
            value
        }).collect();
        let mut edge_values: Vec<_> = self.edges.iter().filter(|edge| !edge.is_hidden()).collect();
        edge_values.sort_by(|a,b| a.name.cmp(&b.name));
        let exported_edges: Vec<_> = edge_values.into_iter().map(|edge| {
            let mut value = json!({"name":edge.name,"schema":schema_name(edge)});
            if let Some(comment) = &edge.comment { value["comment"] = json!(comment); } value
        }).collect();
        let mut property_values: Vec<_> = self.properties.iter().filter(|property| !property.is_hidden()).collect();
        property_values.sort_by(|a,b| a.name.cmp(&b.name));
        let exported_properties: Vec<_> = property_values.into_iter().map(|property| {
            let mut value = json!({"name":property.name,"schema":schema_name(property)});
            if let Some(comment) = &property.comment { value["comment"] = json!(comment); } value
        }).collect();
        let mut infos: Vec<_> = self.node_types.iter().filter_map(|n| n.schema_info.as_ref())
            .chain(self.edges.iter().filter_map(|e| e.schema_info.as_ref())).filter(|i| i.schema_name != "Hidden").collect();
        infos.sort_by_key(|info| info.doc_index);
        let mut seen = BTreeSet::new();
        let exported_schemas: Vec<_> = infos.into_iter().filter(|info| seen.insert(info.schema_class.clone())).map(|info|
            json!({"name":info.schema_name,"description":info.description,"providedByFrontend":info.provided_by_frontend})).collect();
        json!({"schemas":exported_schemas,"nodes":exported_nodes,"edges":exported_edges,"properties":exported_properties})
    }
}
