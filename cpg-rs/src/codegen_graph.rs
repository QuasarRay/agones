//! Schema-driven typed graph domain emitted by Genco as part of domain.rs.
use super::*;
use std::collections::BTreeSet;

#[derive(Clone,Debug,Default,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct GraphApiReport {
    pub concrete_node_classes:usize,pub abstract_node_classes:usize,pub stored_property_accessors:usize,
    pub detached_property_accessors:usize,pub contained_accessors:usize,pub neighbor_steps:usize,
    pub edge_classes:usize,pub starters:usize,
}
fn runtime_scalar(value:&Scalar)->Result<rust::Tokens> {
    Ok(match value {
        Scalar::Null=>quote! { crate::graph::Value::Null },
        Scalar::String {value}=>{let value=quoted(value.as_str());quote! {crate::graph::Value::string($value)}},
        Scalar::Boolean {value}=>{let value=value.to_string();quote! {crate::graph::Value::Boolean($value)}},
        Scalar::Byte {decimal}=>quote! {crate::graph::Value::Byte($decimal)},Scalar::Short {decimal}=>quote! {crate::graph::Value::Short($decimal)},
        Scalar::Int {decimal}=>quote! {crate::graph::Value::Int($decimal)},Scalar::Long {decimal}=>quote! {crate::graph::Value::Long($decimal)},
        Scalar::Float {bits}=>{let bits=format!("0x{bits}u32");quote! {crate::graph::Value::Float($bits)}},
        Scalar::Double {bits}=>{let bits=format!("0x{bits}u64");quote! {crate::graph::Value::Double($bits)}},
        Scalar::Char {utf16_code_unit}=>{let unit=utf16_code_unit.to_string();quote! {crate::graph::Value::Char($unit)}},
        Scalar::List {values}=>{let mut items=rust::Tokens::new();for value in values {let value=runtime_scalar(value)?;quote_in! {items=>$value,};}
            quote! {crate::graph::Value::List(vec![$items])}},
    })
}
fn graph_value_type(kind:ValueType)->rust::Tokens {
    match kind {ValueType::NodeRef=>quote! {crate::graph::NodeHandle},ValueType::Unknown=>quote! {crate::graph::Value},_=>value_type(kind)}
}
fn graph_property_type(property:&Property)->rust::Tokens {
    let value=graph_value_type(property.value_type);
    match property.cardinality {Cardinality::List=>quote! {Vec<$value>},Cardinality::ZeroOrOne=>quote! {Option<$value>},_=>value}
}
fn classes(schema:&Schema)->Vec<&NodeType> {std::iter::once(&schema.any_node).chain(&schema.node_base_types).chain(&schema.node_types).collect()}
fn labels_for(schema:&Schema,node:&NodeType)->BTreeSet<String> {
    schema.node_types.iter().filter(|candidate|node.kind==NodeKind::Any || candidate.name==node.name || candidate.ancestors.contains(&node.name))
        .map(|node|node.name.clone()).collect()
}
fn adjacency<'a>(schema:&'a Schema,node:&NodeType,direction:bool)->Vec<&'a AdjacentNode> {
    let nodes=schema.named_nodes();let mut owners=vec![nodes[node.name.as_str()]];
    for ancestor in &node.ancestors {owners.push(nodes[ancestor.as_str()]);}
    if node.kind!=NodeKind::Concrete {owners.extend(schema.node_types.iter().filter(|candidate|node.kind==NodeKind::Any || candidate.ancestors.contains(&node.name)));}
    let mut result=Vec::new();
    for owner in owners {for edge in if direction {&owner.out_edges}else{&owner.in_edges} {if !result.contains(&edge) {result.push(edge);}}}
    result
}
fn common_class<'a>(schema:&'a Schema,labels:&BTreeSet<String>)->&'a NodeType {
    classes(schema).into_iter().filter(|candidate|labels.is_subset(&labels_for(schema,candidate)))
        .min_by_key(|candidate|(labels_for(schema,candidate).len(),std::cmp::Reverse(candidate.ancestors.len()),candidate.name.as_str()))
        .unwrap_or(&schema.any_node)
}
fn marker_specs(schema:&Schema,node:&NodeType,properties:&BTreeMap<&str,&Property>)->Result<rust::Tokens> {
    let mut rules=rust::Tokens::new();
    for name in &node.all_properties {
        let property=properties[name.as_str()];let name=quoted(name.as_str());
        match &property.cardinality {
            Cardinality::One {default:Some(value),..}=>{let value=runtime_scalar(value)?;quote_in! {rules=>FieldRule::one($name,$value),};},
            Cardinality::One {default:None,..}=>quote_in! {rules=>FieldRule::one($name,crate::graph::Value::Null),},
            Cardinality::ZeroOrOne=>quote_in! {rules=>FieldRule::optional($name),},Cardinality::List=>quote_in! {rules=>FieldRule::list($name),},
        }
    }
    for contained in &node.contained_nodes {
        let name=quoted(contained.local_name.as_str());
        match contained.cardinality {Cardinality::One {..}=>quote_in! {rules=>FieldRule::contained_one($name),},
            Cardinality::ZeroOrOne=>quote_in! {rules=>FieldRule::optional($name),},Cardinality::List=>quote_in! {rules=>FieldRule::list($name),}}
    }
    let _=schema;Ok(rules)
}
fn property_methods(property:&Property,new:bool)->rust::Tokens {
    let mut methods=rust::Tokens::new();let field=field_name(&property.name);let name=quoted(property.name.as_str());
    let getter=if new {format!("get_{}",field.trim_start_matches("r#"))}else{field.clone()};
    let ty=graph_value_type(property.value_type);let result_ty=graph_property_type(property);
    let get=match property.cardinality {Cardinality::List=>quote! {self.list::<$ty>($name)},Cardinality::ZeroOrOne=>quote! {self.optional::<$ty>($name)},_=>quote! {self.required::<$ty>($name)}};
    quote_in! {methods=>pub fn $getter(&self)->crate::Result<$result_ty> {$get} }
    let setter=if new {field.clone()}else{format!("set_{}",field.trim_start_matches("r#"))};
    let arg=if property.value_type==ValueType::String && matches!(property.cardinality,Cardinality::One {..}) {quote! {impl Into<String>}}
        else if matches!(property.cardinality,Cardinality::List) && property.value_type==ValueType::String {quote! {impl IntoIterator<Item=impl Into<String>>}}
        else if matches!(property.cardinality,Cardinality::List) {quote! {impl IntoIterator<Item=$ty>}}else{result_ty.clone()};
    let encode=match property.cardinality {
        Cardinality::List if property.value_type==ValueType::String=>quote! {let values=value.into_iter().map(|v|crate::graph::typed::GraphValue::into_value(Into::<String>::into(v)).map(crate::graph::PropertyInput::Value)).collect::<crate::Result<Vec<_>>>()?;crate::graph::PropertyInput::List(values)},
        Cardinality::List=>quote! {let values=value.into_iter().map(|v|crate::graph::typed::GraphValue::into_value(v).map(crate::graph::PropertyInput::Value)).collect::<crate::Result<Vec<_>>>()?;crate::graph::PropertyInput::List(values)},
        Cardinality::ZeroOrOne=>quote! {let values=value.into_iter().map(|v|crate::graph::typed::GraphValue::into_value(v).map(crate::graph::PropertyInput::Value)).collect::<crate::Result<Vec<_>>>()?;crate::graph::PropertyInput::List(values)},
        _ if property.value_type==ValueType::String=>quote! {crate::graph::PropertyInput::Value(crate::graph::typed::GraphValue::into_value(Into::<String>::into(value))?)},
        _=>quote! {crate::graph::PropertyInput::Value(crate::graph::typed::GraphValue::into_value(value)?)},
    };
    if new {quote_in! {methods=>pub fn $setter(&self,value:$arg)->crate::Result<&Self> {let input={$encode};self.set($name,input)} }}
    else {quote_in! {methods=>pub fn $setter(&self,diff:&mut crate::graph::DiffGraphBuilder,value:$arg)->crate::Result<&Self> {let input={$encode};self.set(diff,$name,input);Ok(self)} }}
    if property.value_type==ValueType::String {
        let null_getter=format!("{}_nullable",getter.trim_start_matches("r#"));let null_setter=format!("{}_nullable",setter.trim_start_matches("r#"));
        let (nullable_ty,getter_body,encoder)=if matches!(property.cardinality,Cardinality::List) {
            (quote! {Vec<Option<String>>},quote! {self.list_nullable::<String>($name)},
                quote! {crate::graph::PropertyInput::List(value.into_iter().map(|v|crate::graph::PropertyInput::Value(v.map(crate::graph::Value::string).unwrap_or(crate::graph::Value::Null))).collect())})
        }else{
            let encoder=if matches!(property.cardinality,Cardinality::One {..}) {quote! {crate::graph::PropertyInput::List(vec![crate::graph::PropertyInput::Value(value.map(crate::graph::Value::string).unwrap_or(crate::graph::Value::Null))])}}
                else{quote! {crate::graph::PropertyInput::List(value.into_iter().map(|v|crate::graph::PropertyInput::Value(crate::graph::Value::string(v))).collect())}};
            (quote! {Option<String>},quote! {self.optional::<String>($name)},encoder)
        };
        quote_in! {methods=>pub fn $null_getter(&self)->crate::Result<$nullable_ty> {$getter_body} }
        if new {quote_in! {methods=>pub fn $null_setter(&self,value:$nullable_ty)->crate::Result<&Self> {self.set($name,$encoder)} }}
        else {quote_in! {methods=>pub fn $null_setter(&self,diff:&mut crate::graph::DiffGraphBuilder,value:$nullable_ty)->crate::Result<&Self> {self.set(diff,$name,$encoder);Ok(self)} }}
    }
    methods
}
fn contained_methods(schema:&Schema,field:&ContainedNode,new:bool)->Result<rust::Tokens> {
    let target=schema.nodes().find(|node|node.name==field.node_type).ok_or_else(||Error::Invalid("contained class is missing".into()))?;
    let target=&target.class_name;let name=quoted(field.local_name.as_str());let setter=field_name(&field.local_name);
    let getter=if new {format!("get_{}",setter.trim_start_matches("r#"))}else{setter.clone()};let mut methods=rust::Tokens::new();
    let target_type=if new {quote! {crate::graph::typed::TypedReference<super::markers::$target>}}else{quote! {crate::graph::typed::TypedNode<'a,super::markers::$target>}};
    let get=if new {quote! {self.contained::<super::markers::$target>($name,graph)?}}else{quote! {self.contained::<super::markers::$target>($name)?}};
    let result_type=if matches!(field.cardinality,Cardinality::List) {quote! {Vec<Option<$target_type>>}}else{quote! {Option<$target_type>}};
    let body=if matches!(field.cardinality,Cardinality::List) {quote! {Ok($get)}}else{quote! {let mut values=$get;if values.len()>1 {return Err(crate::Error::SourceNotImplemented("multiple values in a single contained property"));}Ok(values.pop().flatten())}};
    if new {quote_in! {methods=>pub fn $getter(&self,graph:Option<&crate::graph::Graph>)->crate::Result<$result_type> {$body} }}
    else {quote_in! {methods=>pub fn $getter(&self)->crate::Result<$result_type> {$body} }}
    let set=if new {setter.clone()}else{format!("set_{}",setter.trim_start_matches("r#"))};
    let reference=quote! {crate::graph::typed::TypedReference<super::markers::$target>};
    let arg=if matches!(field.cardinality,Cardinality::List) {quote! {impl IntoIterator<Item=impl Into<$reference>>}}else{quote! {impl Into<$reference>}};
    let input=if matches!(field.cardinality,Cardinality::List) {quote! {crate::graph::PropertyInput::List(value.into_iter().map(|v|crate::graph::PropertyInput::Node(Into::<$reference>::into(v).raw())).collect())}}
        else{quote! {crate::graph::PropertyInput::Node(Into::<$reference>::into(value).raw())}};
    if new {quote_in! {methods=>pub fn $set(&self,value:$arg)->crate::Result<&Self> {self.set($name,$input)} }}
    else {quote_in! {methods=>pub fn $set(&self,diff:&mut crate::graph::DiffGraphBuilder,value:$arg)->crate::Result<&Self> {self.set(diff,$name,$input);Ok(self)} }}
    let clear=format!("clear_{}",setter.trim_start_matches("r#"));
    let clear_input=if matches!(field.cardinality,Cardinality::One {..}) {quote! {crate::graph::PropertyInput::List(vec![crate::graph::PropertyInput::Value(crate::graph::Value::Null)])}}
        else{quote! {crate::graph::PropertyInput::List(Vec::new())}};
    if new {quote_in! {methods=>pub fn $clear(&self)->crate::Result<&Self> {self.set($name,$clear_input)} }}
    else {quote_in! {methods=>pub fn $clear(&self,diff:&mut crate::graph::DiffGraphBuilder)->crate::Result<&Self> {self.set(diff,$name,$clear_input);Ok(self)} }}
    if matches!(field.cardinality,Cardinality::List) {
        let nullable=format!("{}_nullable",set.trim_start_matches("r#"));let input=quote! {crate::graph::PropertyInput::List(value.into_iter().map(|v|v.map(|v|crate::graph::PropertyInput::Node(v.raw())).unwrap_or(crate::graph::PropertyInput::Value(crate::graph::Value::Null))).collect())};
        if new {quote_in! {methods=>pub fn $nullable(&self,value:impl IntoIterator<Item=Option<$reference>>)->crate::Result<&Self> {self.set($name,$input)} }}
        else {quote_in! {methods=>pub fn $nullable(&self,diff:&mut crate::graph::DiffGraphBuilder,value:impl IntoIterator<Item=Option<$reference>>)->crate::Result<&Self> {self.set(diff,$name,$input);Ok(self)} }}
    }
    Ok(methods)
}
fn traversal_property(property:&Property)->rust::Tokens {
    let field=field_name(&property.name);let exact=format!("{}_exact",field.trim_start_matches("r#"));let not=format!("{}_not_exact",field.trim_start_matches("r#"));
    let predicate=format!("{}_where",field.trim_start_matches("r#"));let name=quoted(property.name.as_str());let ty=graph_value_type(property.value_type);let result_ty=graph_property_type(property);
    let mut tokens=rust::Tokens::new();
    quote_in! {tokens=>
        pub fn $field(self)->impl Iterator<Item=crate::Result<$result_ty>>+'a {self.map(|node|node.and_then(|node|node.$field()))}
        pub fn $exact(self,values:impl IntoIterator<Item=$ty>)->crate::Result<Self> {let values=values.into_iter().map(crate::graph::typed::GraphValue::into_value).collect::<crate::Result<Vec<_>>>()?;Ok(self.exact($name,values,false))}
        pub fn $not(self,values:impl IntoIterator<Item=$ty>)->crate::Result<Self> {let values=values.into_iter().map(crate::graph::typed::GraphValue::into_value).collect::<crate::Result<Vec<_>>>()?;Ok(self.exact($name,values,true))}
        pub fn $predicate(self,mut predicate:impl FnMut($result_ty)->bool+'a)->Self {self.try_filter(move |node|Ok(predicate(node.$field()?)))}
    };
    tokens
}
fn neighbors(schema:&Schema,node:&NodeType,methods:&mut rust::Tokens,traversal:&mut rust::Tokens)->Result<usize> {
    let mut count=0;let mut used=BTreeSet::new();
    for (outgoing,direction) in [(true,quote! {crate::graph::Direction::Outgoing}),(false,quote! {crate::graph::Direction::Incoming})] {
        let mut groups:BTreeMap<&str,Vec<&AdjacentNode>>=BTreeMap::new();for edge in adjacency(schema,node,outgoing) {groups.entry(&edge.edge).or_default().push(edge);}
        for (edge,rows) in groups {
            let edge_name=quoted(edge);let mut labels=BTreeSet::new();let nodes=schema.named_nodes();
            for row in &rows {labels.extend(labels_for(schema,nodes[row.neighbor.as_str()]));}
            let target=&common_class(schema,&labels).class_name;let suffix=if outgoing {"out"}else{"in"};
            let step=format!("{}_{}",snake_case(edge),suffix);
            quote_in! {methods=>pub fn $step(&self)->crate::Result<crate::graph::typed::NodeTraversal<'a,super::markers::$target>> {self.neighbors($edge_name,$direction)} }
            quote_in! {traversal=>pub fn $step(self)->crate::graph::typed::NodeTraversal<'a,super::markers::$target> {self.adjacent($edge_name,$direction)} }
            used.insert(step);count+=1;
            for row in rows {
                let target=&nodes[row.neighbor.as_str()].class_name;
                let specific=format!("{}_via_{}_{}",snake_case(target),snake_case(edge),suffix);
                if used.insert(specific.clone()) {
                    quote_in! {methods=>pub fn $specific(&self)->crate::Result<crate::graph::typed::NodeTraversal<'a,super::markers::$target>> {self.neighbors($edge_name,$direction)} }
                    quote_in! {traversal=>pub fn $specific(self)->crate::graph::typed::NodeTraversal<'a,super::markers::$target> {self.adjacent($edge_name,$direction)} }count+=1;
                }
                if let Some(custom)=&row.custom_step_name {let custom=field_name(custom);
                    if !used.insert(custom.clone()) {continue;}
                    let (ty,body)=match row.cardinality {
                        EdgeCardinality::One=>(quote! {crate::graph::typed::TypedNode<'a,super::markers::$target>},quote! {self.neighbor_one($edge_name,$direction)}),
                        EdgeCardinality::ZeroOrOne=>(quote! {Option<crate::graph::typed::TypedNode<'a,super::markers::$target>>},quote! {self.neighbor_optional($edge_name,$direction)}),
                        EdgeCardinality::List=>(quote! {crate::graph::typed::NodeTraversal<'a,super::markers::$target>},quote! {self.neighbors($edge_name,$direction)}),
                    };
                    quote_in! {methods=>pub fn $custom(&self)->crate::Result<$ty> {$body} }count+=1;
                }
            }
        }
    }
    Ok(count)
}
pub fn generate(schema:&Schema)->Result<(rust::Tokens,GraphApiReport)> {
    let properties:BTreeMap<_,_>=schema.properties.iter().map(|p|(p.name.as_str(),p)).collect();let mut result=GraphApiReport::default();
    let mut markers=rust::Tokens::new();let mut stored=rust::Tokens::new();let mut new_nodes=rust::Tokens::new();let mut upcasts=rust::Tokens::new();
    let mut class_fields=rust::Tokens::new();let mut starter_methods=rust::Tokens::new();let mut edge_markers=rust::Tokens::new();let mut edge_types=rust::Tokens::new();
    for node in classes(schema) {
        let class=&node.class_name;let label=quoted(node.name.as_str());let labels=labels_for(schema,node);let mut accepted=rust::Tokens::new();
        for name in &labels {let name=quoted(name.as_str());quote_in! {accepted=>$name,};}
        let rules=marker_specs(schema,node,&properties)?;let mut allowed=rust::Tokens::new();
        for (outgoing,direction) in [(true,quote! {crate::graph::Direction::Outgoing}),(false,quote! {crate::graph::Direction::Incoming})] {
            let mut groups:BTreeMap<&str,BTreeSet<String>>=BTreeMap::new();let nodes=schema.named_nodes();
            for edge in adjacency(schema,node,outgoing) {groups.entry(&edge.edge).or_default().extend(labels_for(schema,nodes[edge.neighbor.as_str()]));}
            for (edge,labels) in groups {let edge=quoted(edge);let mut targets=rust::Tokens::new();for label in labels {let label=quoted(label);quote_in! {targets=>$label,};}
                quote_in! {allowed=>($direction,$edge)=>&[$targets],};}
        }
        header(&mut markers,format!("// CPG_TYPED_CLASS:{}",node.name));
        quote_in! {markers=>
            #[derive(Clone,Copy,Debug,PartialEq,Eq)] pub struct $class;
            impl crate::graph::typed::NodeClass for $class {
                const LABEL:&'static str=$label;const LABELS:&'static [&'static str]=&[$accepted];
                fn fields()->&'static [crate::graph::typed::FieldRule] {
                    use crate::graph::typed::FieldRule;
                    static FIELDS:std::sync::OnceLock<Vec<FieldRule>>=std::sync::OnceLock::new();FIELDS.get_or_init(||vec![$rules])
                }
                fn fields_for(label:&str)->&'static [crate::graph::typed::FieldRule] {super::class_fields(label)}
                fn allowed_neighbors(edge:&str,direction:crate::graph::Direction)->&'static [&'static str] {match (direction,edge) {$allowed _=>&[]}}
            }
        };
        let mut getters=rust::Tokens::new();let mut setters=rust::Tokens::new();let mut traversal=rust::Tokens::new();
        for name in &node.all_properties {let property=properties[name.as_str()];let get=property_methods(property,false);let set=property_methods(property,true);let query=traversal_property(property);
            quote_in! {getters=>$get};quote_in! {setters=>$set};quote_in! {traversal=>$query};result.stored_property_accessors+=1;
            if node.kind==NodeKind::Concrete {result.detached_property_accessors+=1;}
        }
        for field in &node.contained_nodes {let get=contained_methods(schema,field,false)?;let set=contained_methods(schema,field,true)?;
            quote_in! {getters=>$get};quote_in! {setters=>$set};result.contained_accessors+=1;}
        result.neighbor_steps+=neighbors(schema,node,&mut getters,&mut traversal)?;
        header(&mut stored,format!("// CPG_STORED_CLASS:{}",node.name));
        quote_in! {stored=>
            pub type $class<'a>=crate::graph::typed::TypedNode<'a,super::markers::$class>;
            impl<'a> crate::graph::typed::TypedNode<'a,super::markers::$class> {$getters}
            impl<'a> crate::graph::typed::NodeTraversal<'a,super::markers::$class> {$traversal}
        };
        if node.kind==NodeKind::Concrete {
            let kind=schema.node_types.iter().position(|n|n.name==node.name).expect("concrete class comes from schema").to_string();
            quote_in! {markers=>impl crate::graph::typed::ConcreteNodeClass for $class {const NODE_KIND:u16=$kind;} }
            let name=format!("New{class}");header(&mut new_nodes,format!("// CPG_NEW_CLASS:{}",node.name));
            quote_in! {new_nodes=>pub type $name=crate::graph::typed::TypedNew<super::markers::$class>;impl crate::graph::typed::TypedNew<super::markers::$class> {$setters} }
            quote_in! {class_fields=>$label=><markers::$class as crate::graph::typed::NodeClass>::fields(),};result.concrete_node_classes+=1;
        }else{result.abstract_node_classes+=1;}
        let mut ancestors=node.ancestors.clone();if node.kind!=NodeKind::Any {ancestors.push(schema.any_node.name.clone());}
        for ancestor in ancestors {let target=&schema.nodes().find(|n|n.name==ancestor).ok_or_else(||Error::Invalid("missing graph marker ancestor".into()))?.class_name;
            quote_in! {upcasts=>
                impl From<crate::graph::typed::TypedReference<markers::$class>> for crate::graph::typed::TypedReference<markers::$target> {
                    fn from(value:crate::graph::typed::TypedReference<markers::$class>)->Self {Self::checked(value.raw())}
                }
                impl From<crate::graph::typed::TypedNode<'_,markers::$class>> for crate::graph::typed::TypedReference<markers::$target> {
                    fn from(value:crate::graph::typed::TypedNode<'_,markers::$class>)->Self {Self::checked(value.handle().into())}
                }
            };
            if node.kind==NodeKind::Concrete {quote_in! {upcasts=>
                impl From<&crate::graph::typed::TypedNew<markers::$class>> for crate::graph::typed::TypedReference<markers::$target> {
                    fn from(value:&crate::graph::typed::TypedNew<markers::$class>)->Self {Self::checked(value.as_detached().into())}
                }
                impl From<crate::graph::typed::TypedNew<markers::$class>> for crate::graph::typed::TypedReference<markers::$target> {
                    fn from(value:crate::graph::typed::TypedNew<markers::$class>)->Self {Self::checked(value.as_detached().into())}
                }
            };}
        }
        if let Some(starter)=&node.starter_name {let name=field_name(starter);
            quote_in! {starter_methods=>fn $name(&self)->crate::graph::typed::NodeTraversal<'_,markers::$class> {crate::graph::typed::NodeTraversal::all(self.cpg_graph())} }
            result.starters+=1;
        }
    }
    for (kind,edge) in schema.edges.iter().enumerate() {
        let class=&edge.class_name;let alias=format!("{class}Edge");let label=quoted(edge.name.as_str());let kind=kind.to_string();let mut methods=rust::Tokens::new();
        if let Some(property)=edge.property.as_deref().map(|name|properties[name]) {
            let field=field_name(&property.name);let set=format!("set_{}",field.trim_start_matches("r#"));let ty=graph_value_type(property.value_type);
            let nullable=matches!(property.value_type,ValueType::String|ValueType::NodeRef|ValueType::Unknown);
            if nullable {quote_in! {methods=>pub fn $field(&self)->crate::Result<Option<$ty>> {self.optional_property()}
                pub fn $set(&self,diff:&mut crate::graph::DiffGraphBuilder,value:Option<$ty>)->crate::Result<&Self> {self.set(diff,value.map(crate::graph::typed::GraphValue::into_value).transpose()?.unwrap_or(crate::graph::Value::Null));Ok(self)} }}
            else{quote_in! {methods=>pub fn $field(&self)->crate::Result<$ty> {self.property()}
                pub fn $set(&self,diff:&mut crate::graph::DiffGraphBuilder,value:$ty)->crate::Result<&Self> {self.set(diff,crate::graph::typed::GraphValue::into_value(value)?);Ok(self)} }}
        }
        quote_in! {edge_markers=>#[derive(Clone,Copy,Debug)]pub struct $class;impl crate::graph::typed::EdgeClass for $class {const LABEL:&'static str=$label;const EDGE_KIND:u16=$kind;} }
        header(&mut edge_types,format!("// CPG_TYPED_EDGE:{}",edge.name));
        quote_in! {edge_types=>pub type $alias<'a>=crate::graph::typed::TypedEdge<'a,super::edge_markers::$class>;impl crate::graph::typed::TypedEdge<'_,super::edge_markers::$class> {$methods} }
        let starter=format!("{}_edges",snake_case(&edge.name));quote_in! {starter_methods=>fn $starter(&self)->impl Iterator<Item=crate::Result<edges::$alias<'_>>> {edges::$alias::all(self.cpg_graph())} }
        result.edge_classes+=1;
    }
    let any=&schema.any_node.class_name;let mut output=rust::Tokens::new();
    quote_in! {output=>
        pub mod markers {$markers} pub mod stored {$stored} pub mod new_nodes {$new_nodes}
        pub mod edge_markers {$edge_markers} pub mod edges {$edge_types}
        fn class_fields(label:&str)->&'static [crate::graph::typed::FieldRule] {match label {$class_fields _=>&[]}}
        $upcasts
        pub trait CpgStarters {
            fn cpg_graph(&self)->&crate::graph::Graph;
            fn all(&self)->crate::graph::typed::NodeTraversal<'_,markers::$any> {crate::graph::typed::NodeTraversal::all(self.cpg_graph())}
            fn ids<I:IntoIterator<Item=crate::graph::NodeId>>(&self,ids:I)->crate::graph::typed::NodeTraversal<'_,markers::$any> {
                crate::graph::typed::NodeTraversal::from_ids(self.cpg_graph(),ids.into_iter().collect::<Vec<_>>())
            }
            $starter_methods
        }
        impl CpgStarters for crate::graph::Graph {fn cpg_graph(&self)->&crate::graph::Graph {self}}
        impl CpgStarters for crate::graph::Cpg {fn cpg_graph(&self)->&crate::graph::Graph {self.graph()}}
    };
    Ok((output,result))
}
