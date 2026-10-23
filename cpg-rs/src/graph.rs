//! Schema-driven native graph mutation and query support for the CPG APIs and passes.
//! Node ids, detached-node identity, deletion precedence and diff consumption follow
//! the pinned flatgraph APIs used by handwritten codepropertygraph code.
use crate::{Error, Result, schema::{Cardinality, Scalar, Schema, ValueType}};
use cgp::prelude::*;
use std::{collections::{BTreeMap, BTreeSet, VecDeque}, sync::{Arc, Mutex, atomic::{AtomicU64, Ordering}}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u64);
impl NodeId {
    pub fn new(kind: u16, sequence: u32) -> Self { Self((u64::from(kind) << 32) | u64::from(sequence)) }
    pub fn kind(self) -> u32 { (self.0 >> 32) as u32 }
    pub fn sequence(self) -> u32 { self.0 as u32 }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeHandle { pub graph: u64, pub id: NodeId }
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeHandle { pub graph: u64, pub id: u64 }

/// Floating point payloads retain their bits, including signed zero and NaN payloads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Null, Boolean(bool), Byte(i8), Short(i16), Int(i32), Long(i64), Float(u32), Double(u64),
    Char(u16), String(Arc<str>), Node(NodeHandle), List(Vec<Value>),
}
impl Value {
    pub fn string(value: impl Into<Arc<str>>) -> Self { Self::String(value.into()) }
    pub fn from_scalar(value: &Scalar) -> Result<Self> {
        fn invalid() -> Error { Error::Invalid("invalid scalar numeric payload".into()) }
        Ok(match value {
            Scalar::Null => Self::Null,
            Scalar::String { value } => Self::string(value.as_str()),
            Scalar::Boolean { value } => Self::Boolean(*value),
            Scalar::Byte { decimal } => Self::Byte(decimal.parse().map_err(|_| invalid())?),
            Scalar::Short { decimal } => Self::Short(decimal.parse().map_err(|_| invalid())?),
            Scalar::Int { decimal } => Self::Int(decimal.parse().map_err(|_| invalid())?),
            Scalar::Long { decimal } => Self::Long(decimal.parse().map_err(|_| invalid())?),
            Scalar::Float { bits } => Self::Float(u32::from_str_radix(bits, 16).map_err(|_| invalid())?),
            Scalar::Double { bits } => Self::Double(u64::from_str_radix(bits, 16).map_err(|_| invalid())?),
            Scalar::Char { utf16_code_unit } => Self::Char(*utf16_code_unit),
            Scalar::List { values } => Self::List(values.iter().map(Self::from_scalar).collect::<Result<_>>()?),
        })
    }
    fn matches(&self, kind: ValueType) -> bool {
        matches!((self, kind), (Self::Boolean(_), ValueType::Boolean) | (Self::Byte(_), ValueType::Byte) |
            (Self::Short(_), ValueType::Short) | (Self::Int(_), ValueType::Int) | (Self::Long(_), ValueType::Long) |
            (Self::Float(_), ValueType::Float) | (Self::Double(_), ValueType::Double) | (Self::Char(_), ValueType::Char) |
            (Self::String(_), ValueType::String) | (Self::Node(_), ValueType::NodeRef) | (Self::List(_), ValueType::List)) ||
            matches!(self, Self::Null) && matches!(kind, ValueType::String | ValueType::NodeRef | ValueType::Unknown) || kind == ValueType::Unknown
    }
}

#[derive(Clone, Debug)]
pub enum NodeReference { Stored(NodeHandle), Detached(DetachedNode) }
impl From<NodeHandle> for NodeReference { fn from(value: NodeHandle) -> Self { Self::Stored(value) } }
impl From<DetachedNode> for NodeReference { fn from(value: DetachedNode) -> Self { Self::Detached(value) } }
#[derive(Clone, Debug)]
pub enum PropertyInput { Value(Value), Node(NodeReference), List(Vec<PropertyInput>) }
impl From<Value> for PropertyInput { fn from(value: Value) -> Self { Self::Value(value) } }
impl From<NodeHandle> for PropertyInput { fn from(value: NodeHandle) -> Self { Self::Node(value.into()) } }
impl From<DetachedNode> for PropertyInput { fn from(value: DetachedNode) -> Self { Self::Node(value.into()) } }

#[derive(Clone)]
pub struct DetachedNode(Arc<Mutex<DetachedState>>);
struct DetachedState { label: String, properties: BTreeMap<String, PropertyInput>, stored: Option<NodeHandle> }
impl std::fmt::Debug for DetachedNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DetachedNode").field(&Arc::as_ptr(&self.0)).finish()
    }
}
impl DetachedNode {
    pub fn new(label: impl Into<String>) -> Self {
        Self(Arc::new(Mutex::new(DetachedState { label: label.into(), properties: BTreeMap::new(), stored: None })))
    }
    pub fn set(&self, name: impl Into<String>, value: impl Into<PropertyInput>) -> Result<&Self> {
        self.0.lock().map_err(|_| Error::Invalid("poisoned detached node".into()))?.properties.insert(name.into(), value.into());
        Ok(self)
    }
    pub fn stored_ref(&self) -> Result<Option<NodeHandle>> {
        Ok(self.0.lock().map_err(|_| Error::Invalid("poisoned detached node".into()))?.stored)
    }
}

#[derive(Clone, Debug)]
pub enum EdgeProperty { Default, Value(Value) }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction { Incoming, Outgoing }
#[derive(Clone, Debug)]
enum Update {
    AddNode(DetachedNode),
    AddEdge { source: NodeReference, target: NodeReference, label: String, property: EdgeProperty, half: Option<Direction> },
    SetNodeProperty { node: NodeHandle, name: String, value: PropertyInput },
    SetEdgeProperty { edge: EdgeHandle, value: Value },
    RemoveNode(NodeHandle), RemoveEdge(EdgeHandle),
}
#[derive(Default)]
pub struct DiffGraphBuilder { updates: Vec<Update> }
impl DiffGraphBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn size(&self) -> usize { self.updates.len() }
    pub fn add_node(&mut self, node: DetachedNode) -> &mut Self { self.updates.push(Update::AddNode(node)); self }
    pub fn add_edge(&mut self, source: impl Into<NodeReference>, target: impl Into<NodeReference>, label: impl Into<String>, property: EdgeProperty) -> &mut Self {
        self.updates.push(Update::AddEdge { source: source.into(), target: target.into(), label: label.into(), property, half: None }); self
    }
    pub fn add_half_edge(&mut self, source: impl Into<NodeReference>, target: impl Into<NodeReference>, label: impl Into<String>, property: EdgeProperty, direction: Direction) -> &mut Self {
        self.updates.push(Update::AddEdge { source: source.into(), target: target.into(), label: label.into(), property, half: Some(direction) }); self
    }
    pub fn set_node_property(&mut self, node: NodeHandle, name: impl Into<String>, value: impl Into<PropertyInput>) -> &mut Self {
        self.updates.push(Update::SetNodeProperty { node, name: name.into(), value: value.into() }); self
    }
    pub fn set_edge_property(&mut self, edge: EdgeHandle, value: Value) -> &mut Self {
        self.updates.push(Update::SetEdgeProperty { edge, value }); self
    }
    pub fn remove_node(&mut self, node: NodeHandle) -> &mut Self { self.updates.push(Update::RemoveNode(node)); self }
    pub fn remove_edge(&mut self, edge: EdgeHandle) -> &mut Self { self.updates.push(Update::RemoveEdge(edge)); self }
    /// Moving the right builder enforces the source API's destructive absorb operation.
    pub fn absorb(&mut self, mut right: Self) { self.updates.append(&mut right.updates); }
}

#[derive(Debug)]
pub struct StoredNode { pub handle: NodeHandle, pub label: String, properties: BTreeMap<String, Vec<Value>>, deleted: bool }
impl StoredNode { pub fn explicit_properties(&self) -> &BTreeMap<String, Vec<Value>> { &self.properties } }
#[derive(Clone, Debug)]
pub struct Edge {
    pub handle: EdgeHandle, pub source: NodeHandle, pub target: NodeHandle, pub kind: u16,
    pub label: String, pub property: Value, outgoing: bool, incoming: bool,
}
#[derive(Clone, Debug)]
struct Field { value_type: ValueType, cardinality: Cardinality }
pub struct Graph {
    identity: u64, schema: Arc<Schema>, kinds: BTreeMap<String, u16>,
    fields: Vec<BTreeMap<String, Field>>, edge_kinds: BTreeMap<String, u16>,
    nodes: Vec<Vec<StoredNode>>, edges: BTreeMap<u64, Edge>, next_edge: u64,
    closed: bool, changed: bool,
}
impl Graph {
    pub fn new(schema: Arc<Schema>) -> Result<Self> {
        schema.validate()?;
        if schema.node_types.len() > i16::MAX as usize || schema.edges.len() > i16::MAX as usize {
            return Err(Error::Invalid("graph kind count exceeds signed short range".into()));
        }
        static NEXT_GRAPH: AtomicU64 = AtomicU64::new(1);
        let identity = NEXT_GRAPH.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| value.checked_add(1))
            .map_err(|_| Error::Invalid("graph identity space exhausted".into()))?;
        let properties: BTreeMap<_, _> = schema.properties.iter().map(|property| (property.name.as_str(), property)).collect();
        let fields = schema.node_types.iter().map(|node| {
            let mut fields: BTreeMap<_, _> = node.all_properties.iter().map(|name| {
                let property = properties[name.as_str()];
                (name.clone(), Field { value_type: property.value_type, cardinality: property.cardinality.clone() })
            }).collect();
            for contained in &node.contained_nodes {
                fields.insert(contained.local_name.clone(), Field { value_type: ValueType::NodeRef, cardinality: contained.cardinality.clone() });
            }
            fields
        }).collect();
        Ok(Self { identity, kinds: schema.node_types.iter().enumerate().map(|(kind, node)| (node.name.clone(), kind as u16)).collect(),
            edge_kinds: schema.edges.iter().enumerate().map(|(kind, edge)| (edge.name.clone(), kind as u16)).collect(),
            nodes: schema.node_types.iter().map(|_| Vec::new()).collect(), fields, schema, edges: BTreeMap::new(), next_edge: 0, closed: false, changed: false })
    }
    pub fn schema(&self) -> &Schema { &self.schema }
    pub fn identity(&self) -> u64 { self.identity }
    pub fn is_closed(&self) -> bool { self.closed }
    pub fn has_changed(&self) -> bool { self.changed }
    pub fn close(&mut self) { self.closed = true; }
    pub fn node(&self, id: NodeId) -> Option<&StoredNode> {
        self.nodes.get(id.kind() as usize)?.get(id.sequence() as usize).filter(|node| !node.deleted)
    }
    fn check_handle(&self, handle: NodeHandle) -> Result<()> {
        if handle.graph != self.identity { return Err(Error::Invalid("node belongs to a different graph".into())); }
        if self.nodes.get(handle.id.kind() as usize).and_then(|nodes| nodes.get(handle.id.sequence() as usize)).is_none() {
            return Err(Error::Invalid("node does not exist in this graph".into()));
        }
        Ok(())
    }
    pub fn get(&self, handle: NodeHandle) -> Result<Option<&StoredNode>> { self.check_handle(handle)?; Ok(self.node(handle.id)) }
    pub fn all_nodes(&self) -> impl Iterator<Item = &StoredNode> { self.nodes.iter().flatten().filter(|node| !node.deleted) }
    pub fn nodes(&self, label: &str) -> impl Iterator<Item = &StoredNode> {
        self.kinds.get(label).and_then(|kind| self.nodes.get(*kind as usize)).into_iter().flatten().filter(|node| !node.deleted)
    }
    pub fn nodes_of_base(&self, base: &str) -> impl Iterator<Item = &StoredNode> {
        let labels: BTreeSet<_> = self.schema.node_types.iter().filter(|node| node.name == base || node.ancestors.iter().any(|name| name == base))
            .map(|node| node.name.as_str()).collect();
        self.all_nodes().filter(move |node| labels.contains(node.label.as_str()))
    }
    pub fn node_count(&self) -> usize { self.all_nodes().count() }
    pub fn node_count_by_label(&self) -> BTreeMap<&str, usize> {
        self.kinds.keys().filter_map(|label| { let count = self.nodes(label).count(); (count > 0).then_some((label.as_str(), count)) }).collect()
    }
    pub fn property_values(&self, node: NodeHandle, name: &str) -> Result<&[Value]> {
        let node = self.get(node)?.ok_or_else(|| Error::Invalid("property read on deleted node".into()))?;
        Ok(node.properties.get(name).map(Vec::as_slice).unwrap_or(&[]))
    }
    pub fn property(&self, node: NodeHandle, name: &str) -> Result<Option<Value>> {
        let field = self.fields.get(node.id.kind() as usize).and_then(|fields| fields.get(name));
        let values = self.property_values(node, name)?;
        let Some(field) = field else { return Ok(None); };
        if matches!(field.cardinality, Cardinality::List) { return Ok(Some(Value::List(values.to_vec()))); }
        if values.len() > 1 { return Err(Error::Invalid(format!("multiple values in single property {name}"))); }
        if let Some(value) = values.first() { return Ok(Some(value.clone())); }
        match &field.cardinality {
            Cardinality::One { default: Some(default), default_wrapper_is_null: false } => Ok(Some(Value::from_scalar(default)?)),
            _ => Ok(None),
        }
    }
    /// The source's inverse string index indexes explicit values, including repetitions in list fields.
    pub fn nodes_with_property<'a>(&'a self, label: Option<&'a str>, name: &'a str, value: &'a str) -> impl Iterator<Item = &'a StoredNode> {
        self.all_nodes().filter(move |node| label.is_none_or(|label| node.label == label)).flat_map(move |node| {
            node.properties.get(name).into_iter().flatten().filter(move |property| matches!(property, Value::String(text) if text.as_ref() == value)).map(move |_| node)
        })
    }
    pub fn edge(&self, edge: EdgeHandle) -> Result<Option<&Edge>> {
        if edge.graph != self.identity { return Err(Error::Invalid("edge belongs to a different graph".into())); }
        Ok(self.edges.get(&edge.id))
    }
    pub fn edges(&self, node: NodeHandle, direction: Direction, label: Option<&str>) -> Result<Vec<&Edge>> {
        self.check_handle(node)?;
        let mut edges: Vec<_> = self.edges.values().filter(|edge| label.is_none_or(|label| edge.label == label) && match direction {
            Direction::Outgoing => edge.outgoing && edge.source == node,
            Direction::Incoming => edge.incoming && edge.target == node,
        }).collect();
        edges.sort_by_key(|edge| (edge.kind, edge.handle.id));
        Ok(edges)
    }
    pub fn neighbors(&self, node: NodeHandle, direction: Direction, label: Option<&str>) -> Result<Vec<NodeHandle>> {
        Ok(self.edges(node, direction, label)?.into_iter().map(|edge| match direction { Direction::Outgoing => edge.target, Direction::Incoming => edge.source }).collect())
    }
    pub fn all_edges(&self) -> Vec<&Edge> {
        let mut edges: Vec<_> = self.edges.values().filter(|edge| edge.outgoing).collect();
        edges.sort_by_key(|edge| (edge.source.id, edge.kind, edge.handle.id)); edges
    }
    pub fn edge_count(&self) -> usize { self.edges.values().filter(|edge| edge.outgoing).count() }

    fn materialize(&mut self, node: NodeReference, deferred: &mut VecDeque<DetachedNode>, created: &mut usize) -> Result<NodeHandle> {
        match node {
            NodeReference::Stored(node) => { self.check_handle(node)?; Ok(node) }
            NodeReference::Detached(node) => {
                let mut state = node.0.lock().map_err(|_| Error::Invalid("poisoned detached node".into()))?;
                if let Some(stored) = state.stored { self.check_handle(stored)?; return Ok(stored); }
                let kind = *self.kinds.get(&state.label).ok_or_else(|| Error::Invalid(format!("unknown node label {}", state.label)))?;
                let nodes = &mut self.nodes[kind as usize];
                let sequence = u32::try_from(nodes.len()).map_err(|_| Error::Invalid("node sequence space exhausted".into()))?;
                let handle = NodeHandle { graph: self.identity, id: NodeId::new(kind, sequence) };
                nodes.push(StoredNode { handle, label: state.label.clone(), properties: BTreeMap::new(), deleted: false });
                state.stored = Some(handle); *created += 1;
                drop(state); deferred.push_back(node); Ok(handle)
            }
        }
    }
    fn resolve_input(&mut self, input: PropertyInput, deferred: &mut VecDeque<DetachedNode>, created: &mut usize) -> Result<Vec<Value>> {
        Ok(match input {
            PropertyInput::Value(Value::Null) => Vec::new(),
            PropertyInput::Value(Value::List(values)) => values,
            PropertyInput::Value(value) => vec![value],
            PropertyInput::Node(node) => vec![Value::Node(self.materialize(node, deferred, created)?)],
            PropertyInput::List(values) => {
                let mut result = Vec::new();
                for value in values { result.push(self.resolve_element(value, deferred, created)?); }
                result
            }
        })
    }
    fn resolve_element(&mut self, input: PropertyInput, deferred: &mut VecDeque<DetachedNode>, created: &mut usize) -> Result<Value> {
        match input {
            PropertyInput::Value(value) => Ok(value),
            PropertyInput::Node(node) => Ok(Value::Node(self.materialize(node, deferred, created)?)),
            PropertyInput::List(values) => {
                let mut result = Vec::new();
                for value in values { result.push(self.resolve_element(value, deferred, created)?); }
                Ok(Value::List(result))
            }
        }
    }
    fn put_property(&mut self, node: NodeHandle, name: String, values: Vec<Value>) -> Result<()> {
        let Some(field) = self.fields[node.id.kind() as usize].get(&name) else { return Ok(()); };
        for value in &values {
            if !value.matches(field.value_type) { return Err(Error::Invalid(format!("unsupported property type on {}.{name}", self.schema.node_types[node.id.kind() as usize].name))); }
            if let Value::Node(reference) = value { self.check_handle(*reference)?; }
        }
        self.nodes[node.id.kind() as usize][node.id.sequence() as usize].properties.insert(name, values);
        Ok(())
    }
    fn drain_deferred(&mut self, deferred: &mut VecDeque<DetachedNode>, created: &mut usize) -> Result<()> {
        while let Some(node) = deferred.pop_front() {
            let (handle, properties) = {
                let state = node.0.lock().map_err(|_| Error::Invalid("poisoned detached node".into()))?;
                (state.stored.ok_or_else(|| Error::Invalid("deferred node has no stored reference".into()))?, state.properties.clone())
            };
            for (name, value) in properties {
                let value = self.resolve_input(value, deferred, created)?;
                self.put_property(handle, name, value)?;
            }
        }
        Ok(())
    }
    fn edge_property(&self, kind: u16, property: EdgeProperty) -> Result<Value> {
        let definition = &self.schema.edges[kind as usize];
        let Some(name) = &definition.property else { return Ok(Value::Null); };
        let property_definition = self.schema.properties.iter().find(|property| &property.name == name)
            .ok_or_else(|| Error::Invalid(format!("unresolved edge property {name}")))?;
        match property {
            EdgeProperty::Default => match &property_definition.default {
                Some(value) => Value::from_scalar(value), None => Ok(Value::Null),
            },
            EdgeProperty::Value(value) if value.matches(property_definition.value_type) => Ok(value),
            _ => Err(Error::Invalid(format!("unsupported property type for edge {}", definition.name))),
        }
    }
    pub fn apply_diff(&mut self, diff: DiffGraphBuilder) -> Result<ApplyReport> {
        if self.closed { return Err(Error::Invalid("graph cannot be modified any longer since it is closed".into())); }
        self.changed = true;
        let submitted = diff.size();
        let mut changes = 0;
        let mut created = 0;
        // Deletions take precedence over every edge/property update in the entire batch.
        for update in &diff.updates {
            if let Update::RemoveNode(node) = update {
                self.check_handle(*node)?;
                let node = &mut self.nodes[node.id.kind() as usize][node.id.sequence() as usize];
                if !node.deleted { node.deleted = true; node.properties.clear(); changes += 1; }
            }
        }
        self.edges.retain(|_, edge| self.nodes[edge.source.id.kind() as usize][edge.source.id.sequence() as usize].deleted == false &&
            self.nodes[edge.target.id.kind() as usize][edge.target.id.sequence() as usize].deleted == false);
        let mut deferred = VecDeque::new();
        let mut edge_additions = Vec::new();
        let mut property_sets = Vec::new();
        let mut edge_sets = Vec::new();
        let mut edge_deletions = Vec::new();
        for update in diff.updates {
            match update {
                Update::AddNode(node) => { self.materialize(node.into(), &mut deferred, &mut created)?; }
                Update::AddEdge { source, target, label, property, half } => {
                    let source = self.materialize(source, &mut deferred, &mut created)?;
                    let target = self.materialize(target, &mut deferred, &mut created)?;
                    let kind = *self.edge_kinds.get(&label).ok_or_else(|| Error::Invalid(format!("unknown edge label {label}")))?;
                    if self.node(source.id).is_some() && self.node(target.id).is_some() {
                        edge_additions.push((source, target, kind, label, property, half)); changes += 1;
                    }
                }
                Update::SetNodeProperty { node, name, value } => {
                    self.check_handle(node)?;
                    if self.node(node.id).is_some() {
                        let values = self.resolve_input(value, &mut deferred, &mut created)?;
                        property_sets.push((node, name, values)); changes += 1;
                    }
                }
                Update::SetEdgeProperty { edge, value } => {
                    if self.edge(edge)?.is_some() { edge_sets.push((edge, value)); changes += 1; }
                }
                Update::RemoveEdge(edge) => {
                    if self.edge(edge)?.is_some() { edge_deletions.push(edge); changes += 1; }
                }
                Update::RemoveNode(_) => {}
            }
            self.drain_deferred(&mut deferred, &mut created)?;
        }
        for (handle, value) in edge_sets {
            let kind = self.edges[&handle.id].kind;
            let value = self.edge_property(kind, EdgeProperty::Value(value))?;
            self.edges.get_mut(&handle.id).expect("edge resolved above").property = value;
        }
        for edge in edge_deletions { self.edges.remove(&edge.id); }
        for (source, target, kind, label, property, half) in edge_additions {
            let property = self.edge_property(kind, property)?;
            let id = self.next_edge;
            self.next_edge = self.next_edge.checked_add(1).ok_or_else(|| Error::Invalid("edge identity space exhausted".into()))?;
            self.edges.insert(id, Edge { handle: EdgeHandle { graph: self.identity, id }, source, target, kind, label, property,
                outgoing: half != Some(Direction::Incoming), incoming: half != Some(Direction::Outgoing) });
        }
        for (node, name, values) in property_sets { self.put_property(node, name, values)?; }
        Ok(ApplyReport { submitted, created_nodes: created, applied_changes: changes + created })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApplyReport { pub submitted: usize, pub created_nodes: usize, pub applied_changes: usize }

pub struct Cpg { graph: Graph }
impl Cpg {
    pub fn from_graph(graph: Graph) -> Self { Self { graph } }
    pub fn graph(&self) -> &Graph { &self.graph }
    pub fn graph_mut(&mut self) -> &mut Graph { &mut self.graph }
    pub fn close(&mut self) { self.graph.close(); }
    pub fn new_diff_graph_builder() -> DiffGraphBuilder { DiffGraphBuilder::new() }
}
impl std::fmt::Display for Cpg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Cpg[Graph[{} nodes]]", self.graph.node_count()) }
}
#[cgp_component(GraphMutator)]
pub trait CanApplyCpgDiff { fn apply_cpg_diff(&self, graph: &mut Graph, diff: DiffGraphBuilder) -> Result<ApplyReport>; }
#[cgp_new_provider]
impl<Context> GraphMutator<Context> for ApplyNativeCpgDiff {
    fn apply_cpg_diff(_context: &Context, graph: &mut Graph, diff: DiffGraphBuilder) -> Result<ApplyReport> { graph.apply_diff(diff) }
}
#[cgp_component(GraphFactory)]
pub trait CanCreateCpg { fn create_cpg(&self) -> Result<Cpg>; }
#[cfg(feature="native-schema")]
#[cgp_new_provider]
impl<Context> GraphFactory<Context> for CreateCpgFromNativeSchema {
    fn create_cpg(_context: &Context) -> Result<Cpg> { Ok(Cpg::from_graph(Graph::new(Arc::new(crate::cpg_schema::domain_schema()?))?)) }
}
#[cfg(feature="native-schema")]
pub struct CpgContext;
#[cfg(feature="native-schema")]
delegate_and_check_components! {
    CpgContext {
        GraphFactoryComponent: CreateCpgFromNativeSchema,
        GraphMutatorComponent: ApplyNativeCpgDiff,
        crate::execution::ExecutionContextRunnerComponent: crate::execution::UseCurrentOrGlobalRayonPool,
        crate::passes::CpgPassRunnerComponent: crate::passes::RunNativeCpgPass,
    }
}
#[cfg(feature="native-schema")]
impl Cpg {
    pub fn empty() -> Result<Self> { CpgContext.create_cpg() }
    pub fn from_initial(initial: impl FnOnce(&mut DiffGraphBuilder) -> Result<()>) -> Result<Self> {
        let mut cpg = Self::empty()?;
        let mut diff = Self::new_diff_graph_builder(); initial(&mut diff)?;
        CpgContext.apply_cpg_diff(&mut cpg.graph, diff)?; Ok(cpg)
    }
}
