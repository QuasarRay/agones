//! Source schema objects retain reference identity and mutable aliases.
//!
//! Relationships use object ids in a shared arena. Joining independently created
//! builders joins their arenas, so an imported module argument keeps its identity
//! without reference cycles between nodes. `Schema` remains the name-resolved,
//! immutable model consumed by graph/domain generation.
use super::*;
use std::{cell::RefCell, rc::Rc, sync::atomic::{AtomicU64, Ordering}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaObjectId(u64);

#[derive(Clone, Default)]
struct Arena(Rc<RefCell<ArenaState>>);
#[derive(Default)]
struct ArenaState { parent: Option<Arena>, objects: BTreeMap<SchemaObjectId, Object> }
enum Object { Property(Property), Node(NodeObject), Edge(EdgeObject), Constant(Constant) }
struct NodeObject {
    value: NodeType, properties: Vec<SchemaObjectId>, bases: Vec<SchemaObjectId>,
    out: Vec<Adjacency>, incoming: Vec<Adjacency>, contained: Vec<Contained>, primary_key: Option<SchemaObjectId>,
}
struct EdgeObject { value: EdgeType, property: Option<SchemaObjectId> }
#[derive(Clone, PartialEq, Eq)]
struct Adjacency { edge: SchemaObjectId, neighbor: SchemaObjectId, cardinality: EdgeCardinality, step: Option<String>, doc: Option<String> }
#[derive(Clone, PartialEq, Eq)]
struct Contained { node: SchemaObjectId, name: String, cardinality: Cardinality, comment: Option<String> }

impl Arena {
    fn root(&self) -> Self {
        let parent = self.0.borrow().parent.clone();
        if let Some(parent) = parent {
            let root = parent.root(); self.0.borrow_mut().parent = Some(root.clone()); root
        } else { self.clone() }
    }
    fn join(&self, other: &Self) {
        let target = self.root(); let source = other.root();
        if !Rc::ptr_eq(&target.0, &source.0) {
            let objects = std::mem::take(&mut source.0.borrow_mut().objects);
            target.0.borrow_mut().objects.extend(objects);
            source.0.borrow_mut().parent = Some(target);
        }
    }
    fn insert(&self, object: Object) -> SchemaObjectId {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = SchemaObjectId(NEXT_ID.fetch_add(1, Ordering::Relaxed));
        self.root().0.borrow_mut().objects.insert(id, object); id
    }
    fn read<T>(&self, id: SchemaObjectId, f: impl FnOnce(&Object) -> T) -> T {
        let root = self.root(); let state = root.0.borrow(); f(&state.objects[&id])
    }
    fn write<T>(&self, id: SchemaObjectId, f: impl FnOnce(&mut Object) -> T) -> T {
        let root = self.root(); let mut state = root.0.borrow_mut(); f(state.objects.get_mut(&id).expect("live schema object"))
    }
    fn property(&self, id: SchemaObjectId) -> PropertyRef { PropertyRef { arena: self.clone(), id } }
    fn node(&self, id: SchemaObjectId) -> NodeRef { NodeRef { arena: self.clone(), id } }
    fn edge(&self, id: SchemaObjectId) -> EdgeRef { EdgeRef { arena: self.clone(), id } }
    fn constant(&self, id: SchemaObjectId) -> ConstantRef { ConstantRef { arena: self.clone(), id } }
}

macro_rules! object_handle {
    ($name:ident, $kind:ident, $value:ty) => {
        #[derive(Clone)]
        pub struct $name { arena: Arena, id: SchemaObjectId }
        impl $name {
            pub fn id(&self) -> SchemaObjectId { self.id }
            pub fn same_object(&self, other: &Self) -> bool { self.id == other.id }
            pub fn value(&self) -> $value {
                self.arena.read(self.id, |object| match object {
                    Object::$kind(value) => value.clone(), _ => unreachable!("typed schema handle"),
                })
            }
            pub fn proto_id(&self, value: i32) -> &Self {
                self.arena.write(self.id, |object| match object {
                    Object::$kind(object) => object.proto_id = Some(value), _ => unreachable!("typed schema handle"),
                }); self
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple(stringify!($name)).field(&self.id).finish() }
        }
    };
}
object_handle!(PropertyRef, Property, Property);
object_handle!(ConstantRef, Constant, Constant);

// Scala Property is a case class. Its constructor fields define equality;
// cardinality and proto id are mutable members outside the case-class product.
impl PartialEq for PropertyRef {
    fn eq(&self, other: &Self) -> bool {
        let a = self.value(); let b = other.value();
        (a.name, a.value_type, a.comment, a.schema_info) == (b.name, b.value_type, b.comment, b.schema_info)
    }
}
impl Eq for PropertyRef {}
impl PartialEq for ConstantRef { fn eq(&self, other: &Self) -> bool { self.same_object(other) } }
impl Eq for ConstantRef {}
impl PropertyRef {
    pub fn mandatory(&self, default: Scalar) -> Result<&Self> {
        self.arena.write(self.id, |object| match object { Object::Property(value) => value.mandatory(default).map(|_| ()), _ => unreachable!() })?; Ok(self)
    }
    pub fn as_list(&self) -> &Self {
        self.arena.write(self.id, |object| match object { Object::Property(value) => { value.as_list(); }, _ => unreachable!() }); self
    }
}

#[derive(Clone)]
pub struct NodeRef { arena: Arena, id: SchemaObjectId }
#[derive(Clone)]
pub struct EdgeRef { arena: Arena, id: SchemaObjectId }
macro_rules! identity_handle {
    ($name:ident) => {
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.id == other.id } }
        impl Eq for $name {}
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple(stringify!($name)).field(&self.id).finish() }
        }
        impl $name { pub fn id(&self) -> SchemaObjectId { self.id } pub fn same_object(&self, other: &Self) -> bool { self == other } }
    };
}
identity_handle!(NodeRef); identity_handle!(EdgeRef);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdjacentNodeRef { pub edge: EdgeRef, pub neighbor: NodeRef, pub cardinality: EdgeCardinality, pub custom_step_name: Option<String>, pub custom_step_doc: Option<String> }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainedNodeRef { pub node_type: NodeRef, pub local_name: String, pub cardinality: Cardinality, pub comment: Option<String> }

impl NodeRef {
    fn read<T>(&self, f: impl FnOnce(&NodeObject) -> T) -> T { self.arena.read(self.id, |object| match object { Object::Node(value) => f(value), _ => unreachable!() }) }
    fn write<T>(&self, f: impl FnOnce(&mut NodeObject) -> T) -> T { self.arena.write(self.id, |object| match object { Object::Node(value) => f(value), _ => unreachable!() }) }
    pub fn metadata(&self) -> NodeType { self.read(|node| node.value.clone()) }
    pub fn proto_id(&self, value: i32) -> &Self { self.write(|node| node.value.proto_id = Some(value)); self }
    pub fn starter_name(&self, value: Option<String>) -> &Self { self.write(|node| node.value.starter_name = value); self }
    pub fn without_starter(&self) -> &Self { self.starter_name(None) }
    pub fn add_property(&self, property: &PropertyRef) -> &Self {
        self.arena.join(&property.arena);
        if !self.properties_without_inheritance().contains(property) { self.write(|node| node.properties.push(property.id)); } self
    }
    pub fn add_properties<'a>(&self, properties: impl IntoIterator<Item=&'a PropertyRef>) -> &Self {
        for property in properties { self.add_property(property); } self
    }
    pub fn properties_without_inheritance(&self) -> Vec<PropertyRef> {
        let mut values: Vec<_> = self.read(|node| node.properties.clone()).into_iter().map(|id| self.arena.property(id)).collect();
        values.sort_by_key(|value| value.value().name.to_lowercase()); values
    }
    pub fn extend(&self, base: &NodeRef) -> Result<&Self> {
        if base.metadata().kind == NodeKind::Concrete { return Err(Error::Invalid("concrete schema node used as base".into())); }
        self.arena.join(&base.arena);
        self.write(|node| if !node.bases.contains(&base.id) { node.bases.push(base.id); }); Ok(self)
    }
    pub fn bases(&self) -> Vec<NodeRef> {
        let mut values: Vec<_> = self.read(|node| node.bases.clone()).into_iter().map(|id| self.arena.node(id)).collect();
        values.sort_by_key(|node| node.metadata().name); values
    }
    pub fn ancestors(&self) -> Result<Vec<NodeRef>> {
        fn visit(node: &NodeRef, visiting: &mut BTreeSet<SchemaObjectId>) -> Result<Vec<NodeRef>> {
            if !visiting.insert(node.id) { return Err(Error::Invalid("cyclic schema inheritance".into())); }
            let bases = node.bases(); let mut values = bases.clone();
            for base in bases { for ancestor in visit(&base, visiting)? { if !values.contains(&ancestor) { values.push(ancestor); } } }
            visiting.remove(&node.id); Ok(values)
        }
        visit(self, &mut BTreeSet::new())
    }
    pub fn properties(&self) -> Result<Vec<PropertyRef>> {
        let mut values = self.properties_without_inheritance();
        for base in self.ancestors()? { for property in base.properties_without_inheritance() { if !values.contains(&property) { values.push(property); } } }
        values.sort_by_key(|value| value.value().name.to_lowercase()); Ok(values)
    }
    pub fn add_marker_trait(&self, name: String) -> &Self { self.write(|node| if !node.value.marker_traits.contains(&name) { node.value.marker_traits.push(name); }); self }
    pub fn primary_key(&self, property: Option<&PropertyRef>) -> &Self {
        if let Some(property) = property { self.arena.join(&property.arena); }
        self.write(|node| node.primary_key = property.map(|property| property.id)); self
    }
    pub fn get_primary_key(&self) -> Option<PropertyRef> { self.read(|node| node.primary_key).map(|id| self.arena.property(id)) }
    pub fn add_contained_node(&self, node: &NodeRef, name: String, cardinality: Cardinality, comment: &str) -> &Self {
        self.arena.join(&node.arena);
        let value = Contained { node: node.id, name, cardinality, comment: string_to_option(comment) };
        self.write(|node| if !node.contained.contains(&value) { node.contained.push(value); }); self
    }
    pub fn contained_nodes(&self) -> Vec<ContainedNodeRef> {
        let mut values = self.read(|node| node.contained.clone()); values.sort_by_key(|value| value.name.to_lowercase());
        values.into_iter().map(|value| ContainedNodeRef { node_type: self.arena.node(value.node), local_name: value.name, cardinality: value.cardinality, comment: value.comment }).collect()
    }
    pub fn add_out_edge(&self, edge: &EdgeRef, target: &NodeRef, options: EdgeConnection) -> &Self {
        self.arena.join(&edge.arena); self.arena.join(&target.arena);
        let out = Adjacency { edge: edge.id, neighbor: target.id, cardinality: options.out_cardinality, step: string_to_option(&options.out_step), doc: string_to_option(&options.out_doc) };
        let incoming = Adjacency { edge: edge.id, neighbor: self.id, cardinality: options.in_cardinality, step: string_to_option(&options.in_step), doc: string_to_option(&options.in_doc) };
        self.write(|node| if !node.out.contains(&out) { node.out.push(out); });
        target.write(|node| if !node.incoming.contains(&incoming) { node.incoming.push(incoming); }); self
    }
    pub fn add_in_edge(&self, edge: &EdgeRef, source: &NodeRef, options: EdgeConnection) -> &Self { source.add_out_edge(edge, self, options); self }
    pub fn edges(&self, outgoing: bool) -> Vec<AdjacentNodeRef> {
        self.read(|node| if outgoing { node.out.clone() } else { node.incoming.clone() }).into_iter().map(|value|
            AdjacentNodeRef { edge: self.arena.edge(value.edge), neighbor: self.arena.node(value.neighbor), cardinality: value.cardinality, custom_step_name: value.step, custom_step_doc: value.doc }).collect()
    }
    pub fn subtypes(&self, all_nodes: &[NodeRef]) -> Result<Vec<NodeRef>> {
        if self.metadata().kind == NodeKind::Any { return Ok(all_nodes.to_vec()); }
        if self.metadata().kind == NodeKind::Concrete { return Ok(vec![]); }
        let mut values = vec![]; for node in all_nodes { if node.ancestors()?.contains(self) { values.push(node.clone()); } } Ok(values)
    }
    fn model(&self) -> Result<NodeType> {
        let mut value = self.metadata();
        value.bases = self.bases().iter().map(|node| node.metadata().name).collect();
        value.ancestors = self.ancestors()?.iter().map(|node| node.metadata().name).collect();
        value.declared_properties = self.properties_without_inheritance().iter().map(|property| property.value().name).collect();
        value.all_properties = self.properties()?.iter().map(|property| property.value().name).collect();
        value.primary_key = self.get_primary_key().map(|property| property.value().name);
        value.contained_nodes = self.contained_nodes().into_iter().map(|node| ContainedNode { node_type: node.node_type.metadata().name, local_name: node.local_name, cardinality: node.cardinality, comment: node.comment }).collect();
        for (out, destination) in [(true, &mut value.out_edges), (false, &mut value.in_edges)] {
            *destination = self.edges(out).into_iter().map(|edge| AdjacentNode { edge: edge.edge.metadata().name, neighbor: edge.neighbor.metadata().name, cardinality: edge.cardinality, custom_step_name: edge.custom_step_name, custom_step_doc: edge.custom_step_doc }).collect();
            order_adjacencies(destination)?;
        }
        value.marker_traits.sort(); Ok(value)
    }
}
impl EdgeRef {
    fn read<T>(&self, f: impl FnOnce(&EdgeObject) -> T) -> T { self.arena.read(self.id, |object| match object { Object::Edge(value) => f(value), _ => unreachable!() }) }
    fn write<T>(&self, f: impl FnOnce(&mut EdgeObject) -> T) -> T { self.arena.write(self.id, |object| match object { Object::Edge(value) => f(value), _ => unreachable!() }) }
    pub fn metadata(&self) -> EdgeType { self.read(|edge| edge.value.clone()) }
    pub fn proto_id(&self, value: i32) -> &Self { self.write(|edge| edge.value.proto_id = Some(value)); self }
    pub fn with_property(&self, property: Option<&PropertyRef>) -> &Self {
        if let Some(property) = property { self.arena.join(&property.arena); }
        self.write(|edge| edge.property = property.map(|property| property.id)); self
    }
    pub fn property(&self) -> Option<PropertyRef> { self.read(|edge| edge.property).map(|id| self.arena.property(id)) }
    pub fn with_default_accessor_name(&self, value: Option<String>) -> &Self { self.write(|edge| edge.value.default_accessor_name = value); self }
    fn model(&self) -> EdgeType { let mut value = self.metadata(); value.property = self.property().map(|property| property.value().name); value }
}

/// Mutable source-compatible declarations. `build` freezes membership/order,
/// while returned object handles and built schemas continue sharing mutations.
pub struct ObjectSchemaBuilder {
    arena: Arena, domain: String, package: String, traversals: Vec<String>,
    properties: Vec<PropertyRef>, nodes: Vec<NodeRef>, bases: Vec<NodeRef>, edges: Vec<EdgeRef>,
    any: Option<NodeRef>, constants: BTreeMap<String, Vec<ConstantRef>>, options: Option<ProtoOptions>, no_warn: Vec<(NodeRef, PropertyRef)>,
}
impl ObjectSchemaBuilder {
    pub fn new(domain: String, package: String, traversals: Vec<String>) -> Self {
        Self { arena: Arena::default(), domain, package, traversals, properties: vec![], nodes: vec![], bases: vec![], edges: vec![], any: None, constants: BTreeMap::new(), options: None, no_warn: vec![] }
    }
    pub fn add_property(&mut self, name: String, value_type: ValueType, comment: &str, info: Option<SchemaInfo>) -> PropertyRef {
        let id = self.arena.insert(Object::Property(Property::new(name, value_type, comment, info))); let value = self.arena.property(id); self.properties.push(value.clone()); value
    }
    fn add_node(&mut self, name: String, kind: NodeKind, comment: &str, info: Option<SchemaInfo>) -> NodeRef {
        let id = self.arena.insert(Object::Node(NodeObject { value: NodeType::new(name, kind, comment, info), properties: vec![], bases: vec![], out: vec![], incoming: vec![], contained: vec![], primary_key: None })); self.arena.node(id)
    }
    pub fn any_node(&mut self) -> NodeRef {
        if let Some(node) = &self.any { return node.clone(); }
        let node = self.add_node("AnyNode".into(), NodeKind::Any, "generic node base trait - use if you want to be explicitly unspecific", None);
        node.write(|node| node.value.class_name = "StoredNode".into()); self.any = Some(node.clone()); node
    }
    pub fn add_node_base_type(&mut self, name: String, comment: &str, info: Option<SchemaInfo>) -> NodeRef {
        let node = self.add_node(name, NodeKind::Base, comment, info); self.bases.push(node.clone()); node
    }
    pub fn add_node_type(&mut self, name: String, comment: &str, info: Option<SchemaInfo>) -> NodeRef {
        let node = self.add_node(name, NodeKind::Concrete, comment, info); self.nodes.push(node.clone()); node
    }
    pub fn add_edge_type(&mut self, name: String, comment: &str, info: Option<SchemaInfo>) -> EdgeRef {
        let id = self.arena.insert(Object::Edge(EdgeObject { value: EdgeType { class_name: camel_case_caps(&name), name, proto_id: None, comment: string_to_option(comment), schema_info: info, property: None, default_accessor_name: None }, property: None }));
        let value = self.arena.edge(id); self.edges.push(value.clone()); value
    }
    pub fn constant(&self, value: Constant) -> ConstantRef { self.arena.constant(self.arena.insert(Object::Constant(value))) }
    pub fn add_constants(&mut self, category: String, values: impl IntoIterator<Item=ConstantRef>) -> &mut Self {
        for value in values { self.arena.join(&value.arena); self.constants.entry(category.clone()).or_default().push(value); } self
    }
    pub fn proto_options(&mut self, value: Option<ProtoOptions>) -> &mut Self { self.options = value; self }
    pub fn dont_warn_for_duplicate_property(&mut self, node: &NodeRef, property: &PropertyRef) -> &mut Self {
        self.arena.join(&node.arena); self.arena.join(&property.arena);
        let pair = (node.clone(), property.clone()); if !self.no_warn.contains(&pair) { self.no_warn.push(pair); } self
    }
    pub fn build(&mut self) -> Result<ObjectSchema> {
        let any_node = self.any_node();
        let mut properties = self.properties.clone(); properties.sort_by_key(|value| value.value().name.to_lowercase());
        let mut bases = self.bases.clone(); bases.sort_by_key(|value| value.metadata().name.to_lowercase());
        let mut nodes = self.nodes.clone(); nodes.sort_by_key(|value| value.metadata().name.to_lowercase());
        let mut edges = self.edges.clone(); edges.sort_by_key(|value| value.metadata().name.to_lowercase());
        let schema = ObjectSchema { domain_short_name: self.domain.clone(), base_package: self.package.clone(), additional_traversals_packages: self.traversals.clone(), properties, any_node, node_base_types: bases, node_types: nodes, edges, constants_by_category: self.constants.clone(), proto_options: self.options.clone(), no_warn: self.no_warn.clone() };
        schema.verify_proto_ids_unique()?; Ok(schema)
    }
}

#[derive(Clone)]
pub struct ObjectSchema {
    pub domain_short_name: String, pub base_package: String, pub additional_traversals_packages: Vec<String>,
    pub properties: Vec<PropertyRef>, pub any_node: NodeRef, pub node_base_types: Vec<NodeRef>, pub node_types: Vec<NodeRef>,
    pub edges: Vec<EdgeRef>, pub constants_by_category: BTreeMap<String, Vec<ConstantRef>>, pub proto_options: Option<ProtoOptions>,
    pub no_warn: Vec<(NodeRef, PropertyRef)>,
}
impl ObjectSchema {
    pub fn all_node_types(&self) -> Vec<NodeRef> { self.node_types.iter().chain(&self.node_base_types).cloned().collect() }
    pub fn node_properties(&self) -> Result<Vec<PropertyRef>> {
        let mut used = vec![]; for node in self.all_node_types() { used.extend(node.properties()?); }
        Ok(self.properties.iter().filter(|property| used.contains(property)).cloned().collect())
    }
    pub fn edge_properties(&self) -> Vec<PropertyRef> {
        let used: Vec<_> = self.edges.iter().filter_map(EdgeRef::property).collect(); self.properties.iter().filter(|property| used.contains(property)).cloned().collect()
    }
    pub fn verify_proto_ids_unique(&self) -> Result<()> {
        fn verify(category: &str, values: impl IntoIterator<Item=(String, Option<i32>)>) -> Result<()> {
            let mut ids = BTreeMap::<i32, Vec<String>>::new(); for (name, id) in values { if let Some(id) = id { ids.entry(id).or_default().push(name); } }
            let duplicates: Vec<_> = ids.into_iter().filter(|(_, values)| values.len() > 1).collect();
            if duplicates.is_empty() { Ok(()) } else { Err(Error::SourceAssertion(format!("duplicate proto ids in {category}: {duplicates:?}"))) }
        }
        verify("node properties", self.node_properties()?.iter().map(|property| { let value = property.value(); (value.name, value.proto_id) }))?;
        verify("edge properties", self.edge_properties().iter().map(|property| { let value = property.value(); (value.name, value.proto_id) }))?;
        verify("node types", self.node_types.iter().map(|node| { let value = node.metadata(); (value.name, value.proto_id) }))?;
        verify("edge types", self.edges.iter().map(|edge| { let value = edge.metadata(); (value.name, value.proto_id) }))?;
        for (category, values) in &self.constants_by_category { verify(category, values.iter().map(|constant| { let value = constant.value(); (value.name, value.proto_id) }))?; } Ok(())
    }
    /// Resolve references for a name-addressed graph schema. Keep ambiguous
    /// declarations in ObjectSchema; rejecting a lossy projection is distinct
    /// from rejecting their valid construction.
    pub fn to_model(&self) -> Result<Schema> {
        let properties: Vec<_> = self.properties.iter().map(PropertyRef::value).collect();
        let node_types = self.node_types.iter().map(NodeRef::model).collect::<Result<Vec<_>>>()?;
        let bases = self.node_base_types.iter().map(NodeRef::model).collect::<Result<Vec<_>>>()?;
        let edges: Vec<_> = self.edges.iter().map(EdgeRef::model).collect();
        unique_names("name-addressed properties", properties.iter().map(|value| value.name.as_str()))?;
        unique_names("name-addressed nodes", std::iter::once(&self.any_node).chain(&self.node_base_types).chain(&self.node_types).map(|node| node.metadata().name).collect::<Vec<_>>().iter().map(String::as_str))?;
        unique_names("name-addressed edges", edges.iter().map(|value| value.name.as_str()))?;
        let schema = Schema { domain_short_name: self.domain_short_name.clone(), base_package: self.base_package.clone(), additional_traversals_packages: self.additional_traversals_packages.clone(), properties,
            node_property_kinds: self.node_properties()?.iter().enumerate().map(|(index, property)| PropertyKind { name: property.value().name, index }).collect(),
            edge_property_names: self.edge_properties().iter().map(|property| property.value().name).collect(), any_node: self.any_node.model()?, node_base_types: bases, node_types, edges,
            constants_by_category: self.constants_by_category.iter().map(|(name, values)| (name.clone(), values.iter().map(ConstantRef::value).collect())).collect(), proto_options: self.proto_options.clone(),
            duplicate_property_warning_exceptions: self.no_warn.iter().map(|(node, property)| PropertyWarningException { node: node.metadata().name, property: property.value().name }).collect() };
        SchemaBuilder::from_schema(schema).build()
    }
}
