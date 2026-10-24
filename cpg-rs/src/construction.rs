//! Typed lowering of the pinned schema constructor's observed mutations to native Rust.
//! The generated factories call ObjectSchemaBuilder directly; a JVM is never a runtime dependency.
use crate::{content_digest, schema::*, Error, Result};
use cgp::prelude::*;
use genco::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
#[path="construction_modules.rs"]
mod schema_modules;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceFrame {
    #[serde(rename="class")] pub class_name: String,
    pub method: String, pub file: String, pub line: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag="op",rename_all_fields="camelCase",deny_unknown_fields)]
pub enum Instruction {
    #[serde(rename="builder")] Builder { id:String,domain_short_name:String,base_package:String,additional_traversals_packages:Vec<String> },
    #[serde(rename="property")] Property { id:String,builder:String,name:String,value_type:ValueType,comment:Option<String>,schema_info:Option<SchemaInfo> },
    #[serde(rename="node")] Node { id:String,name:String,kind:NodeKind,comment:Option<String>,schema_info:Option<SchemaInfo> },
    #[serde(rename="edge")] Edge { id:String,name:String,comment:Option<String>,schema_info:Option<SchemaInfo> },
    #[serde(rename="constant")] Constant { id:String,name:String,value:String,value_type:ValueType,comment:Option<String>,schema_info:Option<SchemaInfo> },
    #[serde(rename="protoId")] ProtoId { target:String,value:i32 },
    #[serde(rename="mandatory")] Mandatory { target:String,value:Scalar },
    #[serde(rename="asList")] AsList { target:String },
    #[serde(rename="starter")] Starter { target:String,value:Option<String> },
    #[serde(rename="addProperty")] AddProperty { target:String,property:String },
    #[serde(rename="extend")] Extend { target:String,bases:Vec<String> },
    #[serde(rename="marker")] Marker { target:String,name:String },
    #[serde(rename="primaryKey")] PrimaryKey { target:String,property:Option<String> },
    #[serde(rename="contained")] Contained { target:String,node:String,name:String,cardinality:Cardinality,comment:Option<String> },
    #[serde(rename="connect")] Connect { out:String,edge:String,#[serde(rename="in")] incoming:String,
        out_cardinality:EdgeCardinality,in_cardinality:EdgeCardinality,out_step:String,out_doc:String,in_step:String,in_doc:String },
    #[serde(rename="edgeProperty")] EdgeProperty { target:String,property:Option<String> },
    #[serde(rename="edgeAccessor")] EdgeAccessor { target:String,value:Option<String> },
    #[serde(rename="constants")] Constants { builder:String,category:String,values:Vec<String> },
    #[serde(rename="noWarn")] NoWarn { builder:String,node:String,property:String },
    #[serde(rename="protoOptions")] ProtoOptions { builder:String,options:Option<ProtoOptions> },
    #[serde(rename="build")] Build { builder:String },
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Event { pub index:usize,pub source_frames:Vec<SourceFrame>,pub instruction:Instruction }
impl<'de> Deserialize<'de> for Event {
    fn deserialize<D: Deserializer<'de>>(deserializer:D) -> std::result::Result<Self,D::Error> {
        use serde::de::Error;
        let mut map = BTreeMap::<String,Value>::deserialize(deserializer)?;
        let index = serde_json::from_value(map.remove("index").ok_or_else(||D::Error::custom("missing event index"))?).map_err(D::Error::custom)?;
        let source_frames = serde_json::from_value(map.remove("sourceFrames").ok_or_else(||D::Error::custom("missing original source frames"))?).map_err(D::Error::custom)?;
        let instruction = serde_json::from_value(serde_json::to_value(map).map_err(D::Error::custom)?).map_err(D::Error::custom)?;
        Ok(Self {index,source_frames,instruction})
    }
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant { pub events:Vec<Event>,pub schema:Schema }
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct ConstructionTrace { pub format_version:u32,pub domain:Variant,pub protobuf:Variant }

fn mutation(op:&Instruction) -> bool {
    !matches!(op,Instruction::Builder{..}|Instruction::ProtoOptions{..}|Instruction::Build{..})
}
impl ConstructionTrace {
    pub fn load(reader:impl std::io::Read) -> Result<Self> {
        let trace:Self=serde_json::from_reader(reader)?;
        if trace.format_version!=1 { return Err(Error::Invalid("unrecognized constructor trace format".into())); }
        for variant in [&trace.domain,&trace.protobuf] {
            variant.schema.validate()?;
            if variant.events.is_empty() { return Err(Error::Invalid("empty constructor trace".into())); }
            let (builder_id,domain,pkg,traversals)=match &variant.events[0].instruction {
                Instruction::Builder{id,domain_short_name,base_package,additional_traversals_packages}=>
                    (id,domain_short_name,base_package,additional_traversals_packages),
                _=>return Err(Error::Invalid("construction trace lacks initial builder".into())),
            };
            if domain!=&variant.schema.domain_short_name || pkg!=&variant.schema.base_package ||
               traversals!=&variant.schema.additional_traversals_packages {
                return Err(Error::Invalid("builder configuration differs from its complete observed model".into()));
            }
            let mut builds=0;
            let mut options=Vec::new();
            for (index,event) in variant.events.iter().enumerate() {
                if event.index!=index || event.source_frames.is_empty() { return Err(Error::Invalid(format!("event {index} lacks contiguous identity or original provenance"))); }
                match &event.instruction {
                    Instruction::Builder{..} if index!=0=>return Err(Error::Invalid("multiple constructor builders".into())),
                    Instruction::Build{builder}=>{
                        if builder!=builder_id || index+1!=variant.events.len(){return Err(Error::Invalid("schema build must terminate its constructor".into()));}
                        builds+=1;
                    }
                    Instruction::ProtoOptions{builder,options:configured}=>{
                        if builder!=builder_id || variant.events[..index].iter().any(|event|mutation(&event.instruction)) {
                            return Err(Error::Invalid("prototype options configured after mutations or on another builder".into()));
                        }
                        options.push(configured);
                    }
                    _=>{}
                }
            }
            if builds!=1 || options.len()>1 || options.first().map(|value|*value).unwrap_or(&None)!=&variant.schema.proto_options {
                return Err(Error::Invalid("factory/build/options observations are inconsistent".into()));
            }
        }
        let domain:Vec<_>=trace.domain.events.iter().filter(|event|mutation(&event.instruction)).map(|event|&event.instruction).collect();
        let protobuf:Vec<_>=trace.protobuf.events.iter().filter(|event|mutation(&event.instruction)).map(|event|&event.instruction).collect();
        if domain!=protobuf { return Err(Error::Invalid("domain/protobuf constructors have different mutation programs; cannot fuse them".into())); }
        Ok(trace)
    }
}

#[derive(Clone, Debug)]
enum Binding { Builder, Property(String),Node(String),Edge(String),Constant(String) }
fn reference<'a>(symbols:&'a BTreeMap<String,Binding>,id:&str) -> Result<&'a Binding> {
    symbols.get(id).ok_or_else(||Error::Invalid(format!("schema operand used before definition: {id}")))
}
fn named(symbols:&BTreeMap<String,Binding>,id:&str,kind:&str) -> Result<String> {
    match (kind,reference(symbols,id)?) {
        ("property",Binding::Property(name))|("node",Binding::Node(name))|("edge",Binding::Edge(name)) => Ok(name.clone()),
        _ => Err(Error::Invalid(format!("schema operand {id} is not a {kind}"))),
    }
}
fn owned(value:&str) -> rust::Tokens { let value=quoted(value);quote! { $value.to_owned() } }
fn optional(value:Option<&str>) -> rust::Tokens {
    match value { Some(value)=>{let value=owned(value);quote! { Some($value) }},None=>quote! { None } }
}
fn vector(items:impl IntoIterator<Item=rust::Tokens>) -> rust::Tokens {
    let mut values=rust::Tokens::new();
    for value in items { quote_in! { values => $value, }; }
    quote! { vec![$values] }
}
fn scalar(value:&Scalar) -> rust::Tokens {
    match value {
        Scalar::Null=>quote! { Scalar::Null },
        Scalar::String{value}=>{let value=owned(value);quote! { Scalar::String { value:$value } }},
        Scalar::Boolean{value}=>{let value=value.to_string();quote! { Scalar::Boolean { value:$value } }},
        Scalar::Byte{decimal}|Scalar::Short{decimal}|Scalar::Int{decimal}|Scalar::Long{decimal}=>{
            let kind=match value {Scalar::Byte{..}=>"Byte",Scalar::Short{..}=>"Short",Scalar::Int{..}=>"Int",_=>"Long"};
            let decimal=owned(decimal);quote! { Scalar::$kind { decimal:$decimal } }
        },
        Scalar::Float{bits}|Scalar::Double{bits}=>{
            let kind=if matches!(value,Scalar::Float{..}) {"Float"} else {"Double"};
            let bits=owned(bits);quote! { Scalar::$kind { bits:$bits } }
        },
        Scalar::Char{utf16_code_unit}=>{let value=utf16_code_unit.to_string();quote! { Scalar::Char { utf16_code_unit:$value } }},
        Scalar::List{values}=>{let values=vector(values.iter().map(scalar));quote! { Scalar::List { values:$values } }},
    }
}
fn cardinality(value:&Cardinality) -> rust::Tokens {
    match value {
        Cardinality::ZeroOrOne=>quote! { Cardinality::ZeroOrOne },Cardinality::List=>quote! { Cardinality::List },
        Cardinality::One{default_wrapper_is_null,default}=>{
            let null=default_wrapper_is_null.to_string();
            let default=match default {Some(value)=>{let value=scalar(value);quote! { Some($value) }},None=>quote! { None }};
            quote! { Cardinality::One { default_wrapper_is_null:$null,default:$default } }
        }
    }
}
fn info(value:&SchemaInfo) -> rust::Tokens {
    let definition=owned(&value.definition_class);let class=owned(&value.schema_class);let name=owned(&value.schema_name);
    let description=owned(&value.description);let index=value.doc_index.to_string();let provided=value.provided_by_frontend.to_string();
    quote! { SchemaInfo {definition_class:$definition,schema_class:$class,schema_name:$name,
        doc_index:$index,description:$description,provided_by_frontend:$provided} }
}
fn proto_options(value:&ProtoOptions) -> rust::Tokens {
    let package=owned(&value.package);let outer=owned(&value.java_outer_classname);let java=owned(&value.java_package);
    let go=owned(&value.go_package);let csharp=owned(&value.csharp_namespace);let mut mappings=rust::Tokens::new();
    for (key,value) in &value.uncommon_proto_enum_name_mappings {
        let key=owned(key);let value=owned(value);quote_in! { mappings => ($key,$value), };
    }
    quote! { ProtoOptions {package:$package,java_outer_classname:$outer,java_package:$java,go_package:$go,csharp_namespace:$csharp,
        uncommon_proto_enum_name_mappings:std::collections::BTreeMap::from([$mappings])} }
}
fn builder(op:&Instruction) -> Result<()> {
    if matches!(op,Instruction::Builder{..}) {Ok(())}else{Err(Error::Invalid("constructor must begin with builder definition".into()))}
}

#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ConstructionReport {
    #[serde(default)] pub schema_objects_sha256:String,
    #[serde(default)] pub schema_module_generator_sha256:String,
    #[serde(default)] pub schema_module_sources_sha256:BTreeMap<String,String>,
    #[serde(default)] pub schema_modules:usize,
    #[serde(default)] pub schema_module_fields:usize,
    pub format_version:u32,pub trace_sha256:String,pub generator_sha256:String,pub schema_implementation_sha256:String,
    pub cargo_lock_sha256:String,pub output_sha256:String,pub domain_events:usize,pub protobuf_events:usize,
    pub emitted_mutations:usize,pub mappings:Vec<Value>,
    #[serde(default)] pub runtime_module_sha256:String,
    #[serde(default)] pub cargo_manifest_sha256:String,
    #[serde(default)] pub proto_id_constants:usize,
    #[serde(default)] pub source_inventory_sha256:String,
    #[serde(default)] pub proto_ids_source_sha256:String,
}

#[cgp_component(SchemaConstructorGenerator)]
pub trait CanGenerateSchemaConstructors {
    fn generate_schema_constructors(&self,input:&Path,output:&Path)->Result<ConstructionReport>;
}
#[cgp_new_provider]
impl<Context> SchemaConstructorGenerator<Context> for CompileSchemaConstructors {
    fn generate_schema_constructors(_context:&Context,input:&Path,output:&Path)->Result<ConstructionReport> {
        generate_constructors(input,output)
    }
}

/// Emit every recorded mutation. Instruction discrimination is compile-time state, not a runtime provider registry.
pub fn generate_constructors(input:&Path,output:&Path) -> Result<ConstructionReport> {
    let input_bytes=std::fs::read(input)?;let trace=ConstructionTrace::load(input_bytes.as_slice())?;
    builder(&trace.domain.events[0].instruction)?;builder(&trace.protobuf.events[0].instruction)?;
    let mut symbols=BTreeMap::new();
    let mut tokens=rust::Tokens::new();tokens.append("// Generated native CPG schema construction. Do not edit.");tokens.push();
    quote_in! { tokens => use crate::schema::*;use crate::Result; };
    let mut infos:BTreeMap<String,String>=BTreeMap::new();
    for event in &trace.domain.events {
        let data=match &event.instruction {
            Instruction::Property{schema_info,..}|Instruction::Node{schema_info,..}|Instruction::Edge{schema_info,..}|Instruction::Constant{schema_info,..}=>schema_info.as_ref(),
            _=>None,
        };
        if let Some(data)=data {
            let key=serde_json::to_string(data)?;
            if !infos.contains_key(&key) {
                let name=format!("info_{}",infos.len());
                infos.insert(key,name);
            }
        }
    }
    let info_ref=|data:&Option<SchemaInfo>|->Result<rust::Tokens> {
        Ok(match data {
            Some(value)=>{let name=infos.get(&serde_json::to_string(value)?).ok_or_else(||Error::Invalid("missing source schema metadata".into()))?.as_str();quote! {Some($name.clone())}},
            None=>quote! {None},
        })
    };
    let mut mutation_indices=Vec::new();
    let mut operations=BTreeMap::new();
    for event in &trace.domain.events {
        let mut program=rust::Tokens::new();
        if mutation(&event.instruction) {
            program.append(format!("// CPG_CONSTRUCTION_OP:{}",event.index));program.push();mutation_indices.push(event.index);
        }
        match &event.instruction {
            Instruction::Builder{id,..}=>{if !symbols.is_empty(){return Err(Error::Invalid("multiple schema builders in one constructor".into()));}symbols.insert(id.clone(),Binding::Builder);}
            Instruction::Property{id,builder,name,value_type,comment,schema_info}=>{
                if !matches!(reference(&symbols,builder)?,Binding::Builder){return Err(Error::Invalid("property attached to non-builder".into()));}
                let var=format!("_object_{}",symbols.len());let token_var=var.as_str();
                let name_value=owned(name);let ty=format!("{value_type:?}");let comment=quoted(comment.as_deref().unwrap_or(""));let metadata=info_ref(schema_info)?;
                quote_in! {program=>let $token_var=builder.add_property($name_value,ValueType::$ty,$comment,$metadata);};
                symbols.insert(id.clone(),Binding::Property(var));
            }
            Instruction::Node{id,name,kind,comment,schema_info}=>{
                let var=format!("_object_{}",symbols.len());let token_var=var.as_str();
                let name_value=owned(name);let comment=quoted(comment.as_deref().unwrap_or(""));let metadata=info_ref(schema_info)?;
                match kind {
                    NodeKind::Any=>quote_in! {program=>let $token_var=builder.any_node();},
                    NodeKind::Base=>quote_in! {program=>let $token_var=builder.add_node_base_type($name_value,$comment,$metadata);},
                    NodeKind::Concrete=>quote_in! {program=>let $token_var=builder.add_node_type($name_value,$comment,$metadata);},
                };
                symbols.insert(id.clone(),Binding::Node(var));
            }
            Instruction::Edge{id,name,comment,schema_info}=>{
                let var=format!("_object_{}",symbols.len());let token_var=var.as_str();
                let value=owned(name);let comment=quoted(comment.as_deref().unwrap_or(""));let metadata=info_ref(schema_info)?;
                quote_in! {program=>let $token_var=builder.add_edge_type($value,$comment,$metadata);};symbols.insert(id.clone(),Binding::Edge(var));
            }
            Instruction::Constant{id,name,value,value_type,comment,schema_info}=>{
                let var=format!("constant_{}",symbols.len());let var_ref=var.as_str();
                let name=owned(name);let value=owned(value);let ty=format!("{value_type:?}");let comment=optional(comment.as_deref());let metadata=info_ref(schema_info)?;
                quote_in! {program=>let $var_ref=builder.constant(Constant{name:$name,value:$value,value_type:ValueType::$ty,proto_id:None,comment:$comment,schema_info:$metadata});};
                symbols.insert(id.clone(),Binding::Constant(var));
            }
            Instruction::ProtoId{target,value}=>{
                let value=value.to_string();
                let var=match reference(&symbols,target)? {
                    Binding::Property(var)|Binding::Node(var)|Binding::Edge(var)|Binding::Constant(var)=>var.as_str(),
                    Binding::Builder=>return Err(Error::Invalid("builder cannot have a proto id".into())),
                };
                quote_in! {program=>$var.proto_id($value);};
            }
            Instruction::Mandatory{target,value}=>{let var=named(&symbols,target,"property")?;let var=var.as_str();let value=scalar(value);quote_in! {program=>$var.mandatory($value)?;};}
            Instruction::AsList{target}=>{let var=named(&symbols,target,"property")?;let var=var.as_str();quote_in! {program=>$var.as_list();};}
            Instruction::Starter{target,value}=>{let var=named(&symbols,target,"node")?;let var=var.as_str();let value=optional(value.as_deref());quote_in! {program=>$var.starter_name($value);};}
            Instruction::AddProperty{target,property}=>{let node=named(&symbols,target,"node")?;let node=node.as_str();let property=named(&symbols,property,"property")?;let property=property.as_str();quote_in! {program=>$node.add_property(&$property);};}
            Instruction::Extend{target,bases}=>{for base in bases {let node=named(&symbols,target,"node")?;let node=node.as_str();let base=named(&symbols,base,"node")?;let base=base.as_str();quote_in! {program=>$node.extend(&$base)?;};}}
            Instruction::Marker{target,name}=>{let node=named(&symbols,target,"node")?;let node=node.as_str();let name=owned(name);quote_in! {program=>$node.add_marker_trait($name);};}
            Instruction::PrimaryKey{target,property}=>{let node=named(&symbols,target,"node")?;let node=node.as_str();let property=match property {Some(id)=>{let var=named(&symbols,id,"property")?;let var=var.as_str();quote! {Some(&$var)}},None=>quote! {None}};quote_in! {program=>$node.primary_key($property);};}
            Instruction::Contained{target,node,name,cardinality:card,comment}=>{
                let parent=named(&symbols,target,"node")?;let parent=parent.as_str();let child=named(&symbols,node,"node")?;let child=child.as_str();let name=owned(name);let card=cardinality(card);let comment=quoted(comment.as_deref().unwrap_or(""));
                quote_in! {program=>$parent.add_contained_node(&$child,$name,$card,$comment);};
            }
            Instruction::Connect{out,edge,incoming,out_cardinality,in_cardinality,out_step,out_doc,in_step,in_doc}=>{
                let out=named(&symbols,out,"node")?;let out=out.as_str();let edge=named(&symbols,edge,"edge")?;let edge=edge.as_str();let incoming=named(&symbols,incoming,"node")?;let incoming=incoming.as_str();
                let out_card=format!("{out_cardinality:?}");let in_card=format!("{in_cardinality:?}");let out_step=owned(out_step);let out_doc=owned(out_doc);let in_step=owned(in_step);let in_doc=owned(in_doc);
                quote_in! {program=>$out.add_out_edge(&$edge,&$incoming,EdgeConnection{out_cardinality:EdgeCardinality::$out_card,in_cardinality:EdgeCardinality::$in_card,
                    out_step:$out_step,out_doc:$out_doc,in_step:$in_step,in_doc:$in_doc});};
            }
            Instruction::EdgeProperty{target,property}=>{let edge=named(&symbols,target,"edge")?;let edge=edge.as_str();let property=match property {Some(id)=>{let var=named(&symbols,id,"property")?;let var=var.as_str();quote! {Some(&$var)}},None=>quote! {None}};quote_in! {program=>$edge.with_property($property);};}
            Instruction::EdgeAccessor{target,value}=>{let edge=named(&symbols,target,"edge")?;let edge=edge.as_str();let value=optional(value.as_deref());quote_in! {program=>$edge.with_default_accessor_name($value);};}
            Instruction::Constants{builder:builder_id,category,values}=>{
                if !matches!(reference(&symbols,builder_id)?,Binding::Builder){return Err(Error::Invalid("constant category target is not a builder".into()));}
                let mut constants=rust::Tokens::new();
                for id in values {match reference(&symbols,id)? {Binding::Constant(var)=>{let var=var.as_str();quote_in! {constants=>$var.clone(),};},_=>return Err(Error::Invalid("constant category contains a non-constant".into()))}}
                let category=owned(category);quote_in! {program=>builder.add_constants($category,[$constants]);};
            }
            Instruction::NoWarn{node,property,..}=>{let node=named(&symbols,node,"node")?;let node=node.as_str();let property=named(&symbols,property,"property")?;let property=property.as_str();quote_in! {program=>builder.dont_warn_for_duplicate_property(&$node,&$property);};}
            Instruction::ProtoOptions{..}|Instruction::Build{..}=>{},
        }
        program.push();operations.insert(event.index,program);
    }
    let repository=Path::new(env!("CARGO_MANIFEST_DIR")).parent().ok_or_else(||Error::Invalid("missing repository parent".into()))?;
    let inventory_bytes=std::fs::read(repository.join("verification/agones/first-generation-cpg-body-inventory.json"))?;
    let inventory:Value=serde_json::from_slice(&inventory_bytes)?;
    let module_output=schema_modules::emit(&trace,&inventory,&symbols,&infos,&operations,repository)?;
    let module_tokens=&module_output.tokens;
    quote_in! {tokens=>$module_tokens};
    tokens.append("// CPG_CONSTRUCTION_END");tokens.push();
    let mut parts=rust::Tokens::new();
    for encoded in infos.keys(){let value:SchemaInfo=serde_json::from_str(encoded)?;let value=info(&value);quote_in! {parts=>$value,};}
    quote_in! {tokens=>pub fn schema_parts()->Vec<SchemaInfo>{vec![$parts]}};
    for (name,variant) in [("domain",&trace.domain),("protobuf",&trace.protobuf)] {
        let schema=&variant.schema;let domain=owned(&schema.domain_short_name);let package=owned(&schema.base_package);
        let traversals=vector(schema.additional_traversals_packages.iter().map(|value|owned(value)));let function=format!("{name}_schema");
        let mut configure=rust::Tokens::new();
        if let Some(options)=&schema.proto_options{let value=proto_options(options);quote_in! {configure=>builder.proto_options(Some($value));};}
        tokens.append(format!("// CPG_SCHEMA_FACTORY:{name}"));tokens.push();
        let object_function=format!("{name}_schema_objects");let object_function=object_function.as_str();
        quote_in! {tokens=>pub fn $object_function()->Result<ObjectSchema>{let mut builder=ObjectSchemaBuilder::new($domain,$package,$traversals);$configure populate(&mut builder)?;builder.build()}
            pub fn $function()->Result<Schema>{$object_function()?.to_model()}};
    }
    // ProtoIds contains deprecated/unused aliases not recoverable from a final schema snapshot.

    let source_path="schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoIds.scala";
    let file=inventory["files"].as_array().and_then(|files|files.iter().find(|file|file["path"]==source_path))
        .ok_or_else(||Error::Invalid("ProtoIds missing from handwritten source census".into()))?;
    let original=std::fs::read(repository.join("third_party/first-generation/cpg").join(source_path))?;
    if content_digest(&original)!=file["sha256"].as_str().unwrap_or("") {
        return Err(Error::Invalid("changed pinned ProtoIds source".into()));
    }
    tokens.append("// CPG_PROTO_IDS");tokens.push();
    let mut ids=rust::Tokens::new();let mut constants=Vec::new();
    for body in file["bodies"].as_array().ok_or_else(||Error::Invalid("missing ProtoIds body identities".into()))? {
        if body["kind"]!="VALUE_INITIALIZER"{continue;}
        let qualified=body["qualifiedName"].as_str().ok_or_else(||Error::Invalid("missing ProtoIds symbol".into()))?;
        let name=qualified.strip_prefix("ProtoIds::").ok_or_else(||Error::Invalid("nested ProtoIds initializer requires explicit translation".into()))?;
        let start=body["bodySpan"]["startByte"].as_u64().ok_or_else(||Error::Invalid("missing source initializer span".into()))? as usize;
        let end=body["bodySpan"]["endByteExclusive"].as_u64().ok_or_else(||Error::Invalid("missing source initializer span".into()))? as usize;
        let bytes=original.get(start..end).ok_or_else(||Error::Invalid("invalid ProtoIds source range".into()))?;
        let value=std::str::from_utf8(bytes).map_err(|error|Error::Invalid(error.to_string()))?.trim().parse::<i32>()
            .map_err(|_|Error::Invalid(format!("nonliteral ProtoIds initializer {name} must be translated")))?;
        let symbol=crate::codegen::snake_case(name).to_uppercase();
        if !symbol.bytes().all(|c|c.is_ascii_alphanumeric()||c==b'_'){return Err(Error::Invalid("invalid Rust ProtoIds identifier".into()));}
        ids.append(format!("// CPG_PROTO_ID:{name}"));ids.push();let literal=value.to_string();
        let token_symbol=symbol.as_str();
        quote_in! {ids=>pub const $token_symbol:i32=$literal;};ids.push();
        constants.push((name.to_owned(),symbol,body.clone()));
    }
    quote_in! {tokens=>pub mod proto_ids {$ids}};
    let rendered=tokens.to_string().map_err(|error|Error::Invalid(error.to_string()))?;
    let mut mappings=schema_modules::bind(&module_output,&rendered)?;
    let protobuf_mutations:Vec<_>=trace.protobuf.events.iter().filter(|event|mutation(&event.instruction)).collect();
    for (position,index) in mutation_indices.iter().enumerate() {
        let marker=format!("// CPG_CONSTRUCTION_OP:{index}");
        let end_marker=mutation_indices.get(position+1).map(|index|format!("// CPG_CONSTRUCTION_OP:{index}")).unwrap_or_else(||"// CPG_CONSTRUCTION_END".into());
        let start=rendered.find(&marker).ok_or_else(||Error::Invalid("missing constructor operation marker".into()))?;
        let end=rendered[start+marker.len()..].find(&end_marker).map(|offset|start+marker.len()+offset).ok_or_else(||Error::Invalid("missing constructor operation boundary".into()))?;
        let event=&trace.domain.events[*index];
        let protobuf_event=protobuf_mutations[position];
        mappings.push(json!({"eventIndex":index,"protobufEventIndex":protobuf_event.index,"instruction":event.instruction,"sourceFrames":event.source_frames,
            "sourceFramesByVariant":{"domain":event.source_frames,"protobuf":protobuf_event.source_frames},
            "rustSymbol":module_output.event_symbols[index],"path":"schema_constructors.rs","startByte":start,"endByteExclusive":end,
            "sha256":content_digest(&rendered.as_bytes()[start..end])}));
    }
    for (name,variant) in [("domain",&trace.domain),("protobuf",&trace.protobuf)] {
        let marker=format!("// CPG_SCHEMA_FACTORY:{name}");
        let start=rendered.find(&marker).ok_or_else(||Error::Invalid("missing factory boundary".into()))?;
        let end=if name=="domain" {rendered[start+marker.len()..].find("// CPG_SCHEMA_FACTORY:protobuf").map(|offset|start+marker.len()+offset)
            .ok_or_else(||Error::Invalid("missing protobuf factory".into()))?}else{
                rendered[start..].find("// CPG_PROTO_IDS").map(|offset|start+offset).ok_or_else(||Error::Invalid("missing ProtoIds section".into()))?};
        for event in variant.events.iter().filter(|event|!mutation(&event.instruction)) {
            mappings.push(json!({"variant":name,"eventIndex":event.index,"instruction":event.instruction,
                "sourceFrames":event.source_frames,"rustSymbol":format!("{name}_schema"),"path":"schema_constructors.rs",
                "startByte":start,"endByteExclusive":end,"sha256":content_digest(&rendered.as_bytes()[start..end])}));
        }
    }
    for (name,symbol,body) in &constants {
        let marker=format!("// CPG_PROTO_ID:{name}\n");
        let start=rendered.find(&marker).ok_or_else(||Error::Invalid("missing translated ProtoIds literal".into()))?;
        let end=rendered[start..].find(';').map(|offset|start+offset+1).ok_or_else(||Error::Invalid("missing ProtoIds constant declaration".into()))?;
        mappings.push(json!({"sourceBodyId":body["id"],"sourcePath":source_path,"sourceBlob":file["blob"],
            "sourceSpan":body["bodySpan"],"sourceBodySha256":body["bodySha256"],"rustSymbol":format!("proto_ids::{symbol}"),
            "path":"schema_constructors.rs","startByte":start,"endByteExclusive":end,
            "sha256":content_digest(&rendered.as_bytes()[start..end])}));
    }
    let report=ConstructionReport{schema_objects_sha256:content_digest(include_bytes!("schema_objects.rs")),
        schema_module_generator_sha256:content_digest(include_bytes!("construction_modules.rs")),schema_module_sources_sha256:module_output.source_digests,
        schema_modules:module_output.modules,schema_module_fields:module_output.fields,format_version:3,trace_sha256:content_digest(&input_bytes),generator_sha256:content_digest(include_bytes!("construction.rs")),
        schema_implementation_sha256:content_digest(include_bytes!("schema.rs")),cargo_lock_sha256:content_digest(include_bytes!("../Cargo.lock")),
        output_sha256:content_digest(rendered.as_bytes()),domain_events:trace.domain.events.len(),protobuf_events:trace.protobuf.events.len(),
        emitted_mutations:mutation_indices.len(),mappings,runtime_module_sha256:content_digest(include_bytes!("lib.rs")),
        cargo_manifest_sha256:content_digest(include_bytes!("../Cargo.toml")),proto_id_constants:constants.len(),
        source_inventory_sha256:content_digest(&inventory_bytes),proto_ids_source_sha256:content_digest(&original)};
    std::fs::create_dir_all(output)?;let path=output.join("schema_constructors.rs");let receipt=output.join("construction.json");
    if path.exists() {
        let prior:ConstructionReport=serde_json::from_slice(&std::fs::read(&receipt)?)?;
        if content_digest(&std::fs::read(&path)?)!=prior.output_sha256{return Err(Error::Invalid("native schema output has user edits; cannot overwrite".into()));}
    }
    let temporary=output.join("schema_constructors.rs.tmp");if temporary.exists(){return Err(Error::Invalid("recover interrupted constructor emission".into()));}
    std::fs::write(&temporary,rendered)?;std::fs::rename(temporary,path)?;std::fs::write(receipt,serde_json::to_vec_pretty(&report)?)?;
    Ok(report)
}
