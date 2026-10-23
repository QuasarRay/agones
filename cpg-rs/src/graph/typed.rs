//! Shared runtime for schema-generated graph views and detached builders.
//! Grafeo records remain authoritative; typed views borrow the graph and builders
//! share the native detached identity used by the CGP diff application lifecycle.
use super::*;
use std::marker::PhantomData;

pub trait GraphValue:Sized {
    fn from_value(value:&Value)->Result<Self>;
    fn into_value(self)->Result<Value>;
}
fn wrong_value()->Error {Error::ClassCast("stored CPG property has a different schema type".into())}
macro_rules! scalar_values {
    ($($ty:ty=>$variant:ident),* $(,)?)=>{$(impl GraphValue for $ty {
        fn from_value(value:&Value)->Result<Self>{match value {Value::$variant(v)=>Ok(*v),_=>Err(wrong_value())}}
        fn into_value(self)->Result<Value> {Ok(Value::$variant(self))}
    })*};
}
scalar_values!(bool=>Boolean,i8=>Byte,i16=>Short,i32=>Int,i64=>Long,u16=>Char,NodeHandle=>Node);
impl GraphValue for f32 {
    fn from_value(value:&Value)->Result<Self>{match value {Value::Float(v)=>Ok(Self::from_bits(*v)),_=>Err(wrong_value())}}
    fn into_value(self)->Result<Value> {Ok(Value::Float(self.to_bits()))}
}
impl GraphValue for f64 {
    fn from_value(value:&Value)->Result<Self>{match value {Value::Double(v)=>Ok(Self::from_bits(*v)),_=>Err(wrong_value())}}
    fn into_value(self)->Result<Value> {Ok(Value::Double(self.to_bits()))}
}
impl GraphValue for String {
    fn from_value(value:&Value)->Result<Self>{match value {Value::String(v)=>Ok(v.to_string()),_=>Err(wrong_value())}}
    fn into_value(self)->Result<Value> {Ok(Value::string(self))}
}
impl GraphValue for Value {
    fn from_value(value:&Value)->Result<Self>{Ok(value.clone())}
    fn into_value(self)->Result<Value> {Ok(self)}
}
impl GraphValue for Scalar {
    fn from_value(value:&Value)->Result<Self>{Ok(match value {
        Value::Null=>Self::Null,Value::Boolean(value)=>Self::Boolean {value:*value},
        Value::Byte(value)=>Self::Byte {decimal:value.to_string()},Value::Short(value)=>Self::Short {decimal:value.to_string()},
        Value::Int(value)=>Self::Int {decimal:value.to_string()},Value::Long(value)=>Self::Long {decimal:value.to_string()},
        Value::Float(bits)=>Self::Float {bits:format!("{bits:08x}")},Value::Double(bits)=>Self::Double {bits:format!("{bits:016x}")},
        Value::Char(value)=>Self::Char {utf16_code_unit:*value},Value::String(value)=>Self::String {value:value.to_string()},
        Value::List(values)=>Self::List {values:values.iter().map(Self::from_value).collect::<Result<_>>()?},
        Value::Node(_)=>return Err(wrong_value()),
    })}
    fn into_value(self)->Result<Value> {Value::from_scalar(&self)}
}
impl GraphValue for Vec<Scalar> {
    fn from_value(value:&Value)->Result<Self>{match value {Value::List(values)=>values.iter().map(Scalar::from_value).collect(),_=>Err(wrong_value())}}
    fn into_value(self)->Result<Value> {Ok(Value::List(self.into_iter().map(GraphValue::into_value).collect::<Result<_>>()?))}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FieldCardinality {One,Optional,List}
#[derive(Clone,Debug)]
pub struct FieldRule {pub name:&'static str,pub cardinality:FieldCardinality,pub default:Option<Value>,pub always_include:bool}
impl FieldRule {
    pub fn one(name:&'static str,default:Value)->Self {Self {name,cardinality:FieldCardinality::One,default:Some(default),always_include:false}}
    pub fn contained_one(name:&'static str)->Self {Self {name,cardinality:FieldCardinality::One,default:Some(Value::Null),always_include:true}}
    pub fn optional(name:&'static str)->Self {Self {name,cardinality:FieldCardinality::Optional,default:None,always_include:false}}
    pub fn list(name:&'static str)->Self {Self {name,cardinality:FieldCardinality::List,default:None,always_include:false}}
}
pub trait NodeClass:Clone+Copy+'static {
    const LABEL:&'static str;
    const LABELS:&'static [&'static str];
    fn fields()->&'static [FieldRule];
    fn fields_for(label:&str)->&'static [FieldRule];
    fn allowed_neighbors(edge:&str,direction:Direction)->&'static [&'static str];
    fn accepts(label:&str)->bool {Self::LABELS.contains(&label)}
}
pub trait ConcreteNodeClass:NodeClass {const NODE_KIND:u16;}
#[derive(Clone,Debug)]
pub struct TypedReference<C:NodeClass> {reference:NodeReference,class:PhantomData<C>}
impl<C:NodeClass> TypedReference<C> {
    pub fn validate(reference:NodeReference,graph:Option<&Graph>)->Result<Self> {
        let label=match &reference {
            NodeReference::Detached(node)=>node.label()?,
            NodeReference::Stored(node)=>graph.ok_or_else(||Error::Invalid("a stored typed reference requires its graph".into()))?
                .get(*node)?.ok_or_else(||Error::Invalid("typed reference points to a deleted node".into()))?.label.clone(),
        };
        if !C::accepts(&label) {return Err(Error::ClassCast(format!("node {label} is not a {}",C::LABEL)));}
        Ok(Self::checked(reference))
    }
    pub(crate) fn checked(reference:NodeReference)->Self {Self {reference,class:PhantomData}}
    pub fn raw(&self)->NodeReference {self.reference.clone()}
    pub fn stored_ref(&self)->Result<Option<NodeHandle>> {match &self.reference {NodeReference::Stored(v)=>Ok(Some(*v)),NodeReference::Detached(v)=>v.stored_ref()}}
}
impl<C:NodeClass> From<TypedReference<C>> for NodeReference {fn from(value:TypedReference<C>)->Self {value.reference}}
impl<C:NodeClass> From<TypedReference<C>> for PropertyInput {fn from(value:TypedReference<C>)->Self {Self::Node(value.reference)}}

#[derive(Clone,Copy)]
pub struct TypedNode<'a,C:NodeClass> {graph:&'a Graph,handle:NodeHandle,class:PhantomData<C>}
impl<C:NodeClass> std::fmt::Debug for TypedNode<'_,C> {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {f.debug_struct(C::LABEL).field("handle",&self.handle).finish()}
}
impl<'a,C:NodeClass> TypedNode<'a,C> {
    pub fn new(graph:&'a Graph,handle:NodeHandle)->Result<Self> {
        TypedReference::<C>::validate(handle.into(),Some(graph))?;Ok(Self::known(graph,handle))
    }
    fn known(graph:&'a Graph,handle:NodeHandle)->Self {Self {graph,handle,class:PhantomData}}
    pub fn graph(&self)->&'a Graph {self.graph}
    pub fn handle(&self)->NodeHandle {self.handle}
    pub fn label(&self)->&str {&self.graph.node(self.handle.id).expect("borrowed graph keeps the validated node alive").label}
    pub fn reference(&self)->TypedReference<C> {TypedReference::checked(self.handle.into())}
    pub fn cast<T:NodeClass>(&self)->Result<TypedNode<'a,T>> {TypedNode::new(self.graph,self.handle)}
    pub fn required<T:GraphValue>(&self,name:&str)->Result<T> {
        T::from_value(&self.graph.property(self.handle,name)?.ok_or(Error::NoSuchElement("mandatory CPG property is absent"))?)
    }
    pub fn optional<T:GraphValue>(&self,name:&str)->Result<Option<T>> {
        match self.graph.property(self.handle,name)? {None|Some(Value::Null)=>Ok(None),Some(value)=>Ok(Some(T::from_value(&value)?))}
    }
    pub fn list<T:GraphValue>(&self,name:&str)->Result<Vec<T>> {self.graph.property_values(self.handle,name)?.iter().map(T::from_value).collect()}
    pub fn list_nullable<T:GraphValue>(&self,name:&str)->Result<Vec<Option<T>>> {self.graph.property_values(self.handle,name)?.iter()
        .map(|value|if matches!(value,Value::Null) {Ok(None)}else{Ok(Some(T::from_value(value)?))}).collect()}
    pub fn set(&self,diff:&mut DiffGraphBuilder,name:&str,value:PropertyInput) {diff.set_node_property(self.handle,name,value);}
    pub fn contained<T:NodeClass>(&self,name:&str)->Result<Vec<Option<TypedNode<'a,T>>>> {
        self.graph.property_values(self.handle,name)?.iter().map(|value|match value {
            Value::Null=>Ok(None),Value::Node(handle)=>Ok(Some(TypedNode::new(self.graph,*handle)?)),_=>Err(wrong_value()),
        }).collect()
    }
    pub fn neighbors<T:NodeClass>(&self,edge:&str,direction:Direction)->Result<NodeTraversal<'a,T>> {
        let graph=self.graph;let neighbors=graph.neighbors(self.handle,direction,Some(edge))?;
        let iter=neighbors.into_iter().filter(move |handle|graph.node(handle.id).is_some_and(|node|T::accepts(&node.label)))
            .map(move |handle|Ok(TypedNode::known(graph,handle)));
        Ok(NodeTraversal::from_iter(graph,iter))
    }
    pub fn neighbor_one<T:NodeClass>(&self,edge:&str,direction:Direction)->Result<TypedNode<'a,T>> {
        self.neighbors(edge,direction)?.next().transpose()?.ok_or_else(||Error::SchemaViolation(format!(
            "{} edge with label {edge} to an adjacent {} is mandatory, but not defined for this {} node with id={}, (seq={})",
            if direction==Direction::Outgoing {"OUT"}else{"IN"},T::LABEL,self.label(),self.handle.id.0,self.handle.id.sequence())))
    }
    pub fn neighbor_optional<T:NodeClass>(&self,edge:&str,direction:Direction)->Result<Option<TypedNode<'a,T>>> {self.neighbors(edge,direction)?.next().transpose()}
    pub fn properties_map(&self)->Result<BTreeMap<String,Value>> {properties_map(C::fields_for(self.label()),|name|self.graph.property(self.handle,name))}
}
impl<C:NodeClass> From<TypedNode<'_,C>> for NodeReference {fn from(value:TypedNode<'_,C>)->Self {Self::Stored(value.handle)}}
impl<C:NodeClass> From<TypedNode<'_,C>> for TypedReference<C> {fn from(value:TypedNode<'_,C>)->Self {value.reference()}}

#[derive(Clone,Debug)]
pub struct TypedNew<C:ConcreteNodeClass> {node:DetachedNode,class:PhantomData<C>}
impl<C:ConcreteNodeClass> Default for TypedNew<C> {
    fn default()->Self {
        let node=DetachedNode::new(C::LABEL);
        for field in C::fields() {
            let values=if field.cardinality==FieldCardinality::One {vec![field.default.clone().unwrap_or(Value::Null)]}else{Vec::new()};
            node.set(field.name,PropertyInput::List(values.into_iter().map(PropertyInput::Value).collect()))
                .expect("a newly created detached node has an unpoisoned lock");
        }
        Self {node,class:PhantomData}
    }
}
fn input_values(input:Option<PropertyInput>)->Result<Vec<Value>> {
    fn element(value:PropertyInput)->Result<Value> {match value {
        PropertyInput::Value(value)=>Ok(value),PropertyInput::Node(NodeReference::Stored(node))=>Ok(Value::Node(node)),
        PropertyInput::Node(NodeReference::Detached(_))=>Err(Error::Invalid("a detached node requires a typed contained-reference accessor".into())),
        PropertyInput::List(values)=>Ok(Value::List(values.into_iter().map(element).collect::<Result<_>>()?)),
    }}
    match input {
        None|Some(PropertyInput::Value(Value::Null))=>Ok(Vec::new()),Some(PropertyInput::Value(Value::List(values)))=>Ok(values),
        Some(PropertyInput::List(values))=>values.into_iter().map(element).collect(),Some(value)=>Ok(vec![element(value)?]),
    }
}
impl<C:ConcreteNodeClass> TypedNew<C> {
    pub fn new()->Self {Self::default()}
    pub fn as_detached(&self)->DetachedNode {self.node.clone()}
    pub fn reference(&self)->TypedReference<C> {TypedReference::checked(self.node.clone().into())}
    pub fn stored_ref(&self)->Result<Option<NodeHandle>> {self.node.stored_ref()}
    pub fn copy(&self)->Result<Self> {Ok(Self {node:self.node.copy_unstored()?,class:PhantomData})}
    pub fn set(&self,name:&str,value:PropertyInput)->Result<&Self> {self.node.set(name,value)?;Ok(self)}
    pub fn required<T:GraphValue>(&self,name:&str)->Result<T> {
        let values=input_values(self.node.property_input(name)?)?;
        if values.len()>1 {return Err(Error::Invalid("multiple values in a mandatory detached property".into()));}
        T::from_value(values.first().ok_or(Error::NoSuchElement("mandatory detached property is absent"))?)
    }
    pub fn optional<T:GraphValue>(&self,name:&str)->Result<Option<T>> {
        let values=input_values(self.node.property_input(name)?)?;
        if values.len()>1 {return Err(Error::Invalid("multiple values in an optional detached property".into()));}
        match values.first() {None|Some(Value::Null)=>Ok(None),Some(value)=>Ok(Some(T::from_value(value)?))}
    }
    pub fn list<T:GraphValue>(&self,name:&str)->Result<Vec<T>> {input_values(self.node.property_input(name)?)?.iter().map(T::from_value).collect()}
    pub fn list_nullable<T:GraphValue>(&self,name:&str)->Result<Vec<Option<T>>> {input_values(self.node.property_input(name)?)?.iter()
        .map(|value|if matches!(value,Value::Null) {Ok(None)}else{Ok(Some(T::from_value(value)?))}).collect()}
    pub fn contained<T:NodeClass>(&self,name:&str,graph:Option<&Graph>)->Result<Vec<Option<TypedReference<T>>>> {
        let values=match self.node.property_input(name)? {None=>Vec::new(),Some(PropertyInput::List(values))=>values,
            Some(PropertyInput::Value(Value::List(values)))=>values.into_iter().map(PropertyInput::Value).collect(),Some(value)=>vec![value]};
        values.into_iter().map(|value|match value {
            PropertyInput::Value(Value::Null)=>Ok(None),PropertyInput::Node(reference)=>Ok(Some(TypedReference::validate(reference,graph)?)),
            PropertyInput::Value(Value::Node(handle))=>Ok(Some(TypedReference::validate(handle.into(),graph)?)),_=>Err(wrong_value()),
        }).collect()
    }
    pub fn is_valid_neighbor<T:ConcreteNodeClass>(&self,other:&TypedNew<T>,edge:&str,direction:Direction)->bool {
        let _=other;C::allowed_neighbors(edge,direction).contains(&T::LABEL)
    }
    pub fn properties_map(&self)->Result<BTreeMap<String,PropertyInput>> {
        let mut result=BTreeMap::new();
        for rule in C::fields() {
            let values=match self.node.property_input(rule.name)? {
                None|Some(PropertyInput::Value(Value::Null))=>Vec::new(),
                Some(PropertyInput::List(v))=>v,Some(PropertyInput::Value(Value::List(v)))=>v.into_iter().map(PropertyInput::Value).collect(),Some(v)=>vec![v],
            };
            let include=rule.always_include || match rule.cardinality {
                FieldCardinality::List=>!values.is_empty(),
                FieldCardinality::Optional=>values.first().is_some_and(|v|!matches!(v,PropertyInput::Value(Value::Null))),
                FieldCardinality::One=>values.first().is_some_and(|v|match v {
                    PropertyInput::Value(v)=>rule.default.as_ref().is_none_or(|default|!semantic_eq(v,default)),_=>true,
                }),
            };
            if include {let input=if rule.cardinality==FieldCardinality::List {PropertyInput::List(values)}else{values.into_iter().next().unwrap_or(PropertyInput::Value(Value::Null))};
                result.insert(rule.name.to_owned(),input);}
        }
        Ok(result)
    }
}
impl<C:ConcreteNodeClass> From<TypedNew<C>> for DetachedNode {fn from(value:TypedNew<C>)->Self {value.node}}
impl<C:ConcreteNodeClass> From<TypedNew<C>> for NodeReference {fn from(value:TypedNew<C>)->Self {Self::Detached(value.node)}}
impl<C:ConcreteNodeClass> From<TypedNew<C>> for TypedReference<C> {fn from(value:TypedNew<C>)->Self {value.reference()}}
impl<C:ConcreteNodeClass> From<&TypedNew<C>> for TypedReference<C> {fn from(value:&TypedNew<C>)->Self {value.reference()}}

pub fn semantic_eq(left:&Value,right:&Value)->bool {match (left,right) {
    (Value::Float(a),Value::Float(b))=>f32::from_bits(*a)==f32::from_bits(*b),
    (Value::Double(a),Value::Double(b))=>f64::from_bits(*a)==f64::from_bits(*b),
    (Value::List(a),Value::List(b))=>a.len()==b.len()&&a.iter().zip(b).all(|(a,b)|semantic_eq(a,b)),_=>left==right,
}}
fn properties_map(fields:&[FieldRule],mut get:impl FnMut(&str)->Result<Option<Value>>)->Result<BTreeMap<String,Value>> {
    let mut result=BTreeMap::new();
    for field in fields {if let Some(value)=get(field.name)? {
        let include=field.always_include || match field.cardinality {FieldCardinality::One=>field.default.as_ref().is_none_or(|v|!semantic_eq(&value,v)),
            FieldCardinality::Optional=>!matches!(value,Value::Null),FieldCardinality::List=>!matches!(&value,Value::List(v) if v.is_empty())};
        if include {result.insert(field.name.to_owned(),value);}
    }}Ok(result)
}
pub struct NodeTraversal<'a,C:NodeClass> {graph:&'a Graph,iter:Box<dyn Iterator<Item=Result<TypedNode<'a,C>>>+'a>}
impl<'a,C:NodeClass> Iterator for NodeTraversal<'a,C> {
    type Item=Result<TypedNode<'a,C>>;fn next(&mut self)->Option<Self::Item>{self.iter.next()}
}
impl<'a,C:NodeClass> NodeTraversal<'a,C> {
    pub fn from_iter(graph:&'a Graph,iter:impl Iterator<Item=Result<TypedNode<'a,C>>>+'a)->Self {Self {graph,iter:Box::new(iter)}}
    pub fn all(graph:&'a Graph)->Self {Self::from_iter(graph,graph.all_nodes().filter(|node|C::accepts(&node.label)).map(move |node|Ok(TypedNode::known(graph,node.handle))))}
    pub fn from_ids<I:IntoIterator<Item=NodeId>+'a>(graph:&'a Graph,ids:I)->Self where I::IntoIter:'a {
        Self::from_iter(graph,ids.into_iter().filter_map(move |id|graph.node(id).filter(|node|C::accepts(&node.label)).map(|node|Ok(TypedNode::known(graph,node.handle)))))
    }
    pub fn try_filter(self,mut predicate:impl FnMut(&TypedNode<'a,C>)->Result<bool>+'a)->Self {
        Self::from_iter(self.graph,self.iter.filter_map(move |item|match item {Err(error)=>Some(Err(error)),Ok(node)=>match predicate(&node) {
            Ok(true)=>Some(Ok(node)),Ok(false)=>None,Err(error)=>Some(Err(error)),
        }}))
    }
    pub fn exact(self,name:&'static str,values:Vec<Value>,negate:bool)->Self {self.try_filter(move |node| {
        let property=node.graph.property(node.handle,name)?;
        let matches=match property {Some(Value::List(elements))=>elements.iter().any(|element|values.iter().any(|value|semantic_eq(element,value))),
            Some(element)=>values.iter().any(|value|semantic_eq(&element,value)),None=>false};Ok(matches!=negate)
    })}
    pub fn cast<T:NodeClass>(self)->NodeTraversal<'a,T> {
        NodeTraversal::from_iter(self.graph,self.iter.filter_map(|item|match item {Err(error)=>Some(Err(error)),
            Ok(node) if T::accepts(node.label())=>Some(Ok(TypedNode::known(node.graph,node.handle))),_=>None}))
    }
    pub fn adjacent<T:NodeClass>(self,edge:&'static str,direction:Direction)->NodeTraversal<'a,T> {
        let graph=self.graph;NodeTraversal::from_iter(graph,self.iter.flat_map(move |item|match item {
            Err(error)=>NodeTraversal::from_iter(graph,std::iter::once(Err(error))),Ok(node)=>match node.neighbors(edge,direction) {
                Ok(traversal)=>traversal,Err(error)=>NodeTraversal::from_iter(graph,std::iter::once(Err(error))),
            }
        }))
    }
}
pub trait EdgeClass:Clone+Copy+'static {const LABEL:&'static str;const EDGE_KIND:u16;}
#[derive(Clone,Copy)]
pub struct TypedEdge<'a,E:EdgeClass>{graph:&'a Graph,handle:EdgeHandle,class:PhantomData<E>}
impl<'a,E:EdgeClass> TypedEdge<'a,E> {
    pub fn all(graph:&'a Graph)->impl Iterator<Item=Result<Self>>+'a {graph.all_edges().into_iter().filter(|edge|edge.label==E::LABEL).map(move |edge|Self::new(graph,edge.handle))}
    pub fn new(graph:&'a Graph,handle:EdgeHandle)->Result<Self> {
        let edge=graph.edge(handle)?.ok_or(Error::NoSuchElement("edge is absent"))?;
        if edge.label!=E::LABEL {return Err(Error::ClassCast(format!("edge {} is not {}",edge.label,E::LABEL)));}
        Ok(Self {graph,handle,class:PhantomData})
    }
    pub fn handle(&self)->EdgeHandle {self.handle}
    pub fn source<C:NodeClass>(&self)->Result<TypedNode<'a,C>> {TypedNode::new(self.graph,self.graph.edge(self.handle)?.ok_or(Error::NoSuchElement("edge is absent"))?.source)}
    pub fn target<C:NodeClass>(&self)->Result<TypedNode<'a,C>> {TypedNode::new(self.graph,self.graph.edge(self.handle)?.ok_or(Error::NoSuchElement("edge is absent"))?.target)}
    pub fn property<T:GraphValue>(&self)->Result<T> {T::from_value(&self.graph.edge(self.handle)?.ok_or(Error::NoSuchElement("edge is absent"))?.property)}
    pub fn optional_property<T:GraphValue>(&self)->Result<Option<T>> {let value=&self.graph.edge(self.handle)?.ok_or(Error::NoSuchElement("edge is absent"))?.property;
        if matches!(value,Value::Null) {Ok(None)}else{Ok(Some(T::from_value(value)?))}}
    pub fn set(&self,diff:&mut DiffGraphBuilder,value:Value) {diff.set_edge_property(self.handle,value);}
}
