//! Derive parameterized schema modules from the pinned source declarations and
//! already-lowered constructor operations. Foreign operands are bound to module
//! arguments by object identity, never by a schema-element name lookup.
use super::*;
use std::collections::BTreeSet;

struct Field { name: String, ids: Vec<String>, kind: &'static str, body: Value, events: Vec<usize> }
struct Module { name: String, source: Value, class: Value, parameters: Vec<(String, String)>, fields: Vec<Field>, events: Vec<usize>, info: SchemaInfo }
pub(super) struct ModuleOutput {
    pub tokens: rust::Tokens, pub bindings: Vec<Value>, pub event_symbols: BTreeMap<usize, String>,
    pub source_digests: BTreeMap<String, String>, pub modules: usize, pub fields: usize,
}
fn invalid(message: impl Into<String>) -> Error { Error::Invalid(message.into()) }
fn variable(binding: &Binding) -> Result<&str> {
    match binding { Binding::Property(value)|Binding::Node(value)|Binding::Edge(value)|Binding::Constant(value) => Ok(value), _ => Err(invalid("builder is not an object operand")) }
}
fn operands(instruction: &Instruction) -> Vec<&str> {
    match instruction {
        Instruction::ProtoId{target,..}|Instruction::Mandatory{target,..}|Instruction::AsList{target}|
        Instruction::Starter{target,..}|Instruction::Marker{target,..}|Instruction::EdgeAccessor{target,..} => vec![target],
        Instruction::AddProperty{target,property} => vec![target,property],
        Instruction::Extend{target,bases} => std::iter::once(target.as_str()).chain(bases.iter().map(String::as_str)).collect(),
        Instruction::PrimaryKey{target,property}|Instruction::EdgeProperty{target,property} => std::iter::once(target.as_str()).chain(property.as_deref()).collect(),
        Instruction::Contained{target,node,..} => vec![target,node], Instruction::Connect{out,edge,incoming,..} => vec![out,edge,incoming],
        Instruction::Constants{values,..} => values.iter().map(String::as_str).collect(), Instruction::NoWarn{node,property,..} => vec![node,property], _ => vec![],
    }
}
fn definition(instruction: &Instruction) -> Option<&str> {
    match instruction { Instruction::Property{id,..}|Instruction::Node{id,..}|Instruction::Edge{id,..}|Instruction::Constant{id,..} => Some(id), _ => None }
}
fn metadata(instruction: &Instruction) -> Option<&SchemaInfo> {
    match instruction { Instruction::Property{schema_info,..}|Instruction::Node{schema_info,..}|Instruction::Edge{schema_info,..}|Instruction::Constant{schema_info,..} => schema_info.as_ref(), _ => None }
}
fn marker(tokens: &mut rust::Tokens, kind: &str, name: &str) { tokens.append(format!("// CPG_SCHEMA_{kind}:{name}")); tokens.push(); }

pub(super) fn emit(trace: &ConstructionTrace, inventory: &Value, symbols: &BTreeMap<String, Binding>,
    infos: &BTreeMap<String, String>, operations: &BTreeMap<usize, rust::Tokens>, repository: &Path) -> Result<ModuleOutput> {
    let files = inventory["files"].as_array().ok_or_else(||invalid("missing source inventory files"))?;
    let mut groups: Vec<(String, Vec<usize>)> = vec![];
    for event in trace.domain.events.iter().filter(|event| mutation(&event.instruction)) {
        let name = event.source_frames[0].file.strip_suffix(".scala").ok_or_else(||invalid("non-Scala schema operation"))?;
        if groups.last().is_none_or(|(previous,_)| previous != name) {
            if groups.iter().any(|(previous,_)| previous == name) { return Err(invalid("interleaved schema modules require explicit lowering")); }
            groups.push((name.to_owned(),vec![]));
        }
        groups.last_mut().unwrap().1.push(event.index);
    }
    let mut source_digests = BTreeMap::new(); let mut modules = vec![];
    let mut owners: BTreeMap<String, (String,String)> = BTreeMap::new();
    let any_ids: BTreeSet<_> = trace.domain.events.iter().filter_map(|event| match &event.instruction { Instruction::Node{id,kind:NodeKind::Any,..} => Some(id.as_str()), _ => None }).collect();
    for (name, events) in groups {
        let path = format!("schema/src/main/scala/io/shiftleft/codepropertygraph/schema/{name}.scala");
        let source = files.iter().find(|file| file["path"] == path).ok_or_else(||invalid("schema module absent from census"))?;
        let bytes = std::fs::read(repository.join("third_party/first-generation/cpg").join(&path))?;
        if source["sha256"] != content_digest(&bytes) { return Err(invalid(format!("changed schema module {name}"))); }
        source_digests.insert(path, content_digest(&bytes));
        let bodies = source["bodies"].as_array().ok_or_else(||invalid("missing module bodies"))?;
        let class = bodies.iter().find(|body| body["qualifiedName"] == format!("{name}::Schema")).ok_or_else(||invalid("missing schema class"))?;
        let declaration = class["declarationSpan"]["startByte"].as_u64().ok_or_else(||invalid("missing class declaration span"))? as usize;
        let start = class["bodySpan"]["startByte"].as_u64().ok_or_else(||invalid("missing class body span"))? as usize;
        let header = std::str::from_utf8(bytes.get(declaration..start).ok_or_else(||invalid("invalid class span"))?).map_err(|error|invalid(error.to_string()))?;
        let parameters = header.strip_prefix("class Schema(").and_then(|header| header.trim_end().strip_suffix(')')).ok_or_else(||invalid("schema constructor signature needs explicit lowering"))?;
        let mut dependencies = vec![];
        for parameter in parameters.split(',') {
            let (parameter, ty) = parameter.split_once(':').ok_or_else(||invalid("untyped schema constructor parameter"))?;
            if ty.trim() == "SchemaBuilder" { continue; }
            let class = ty.trim().strip_suffix(".Schema").ok_or_else(||invalid("non-module schema parameter"))?;
            if !class.chars().all(|value| value.is_ascii_alphanumeric()) { return Err(invalid("invalid schema module class")); }
            dependencies.push((format!("_arg_{}",crate::codegen::snake_case(parameter.trim())), class.to_owned()));
        }
        let mut fields = vec![];
        for body in bodies.iter().filter(|body| body["kind"] == "VALUE_INITIALIZER") {
            let Some(field_name) = body["qualifiedName"].as_str().and_then(|qualified| qualified.strip_prefix(&format!("{name}::Schema::"))) else { continue; };
            if field_name == "schemaInfo" { continue; }
            if field_name.contains("::") { return Err(invalid("nested schema field requires explicit lowering")); }
            let matching: Vec<_> = events.iter().copied().filter(|index| {
                let line = trace.domain.events[*index].source_frames[0].line as u64;
                body["bodySpan"]["startLine"].as_u64().unwrap() <= line && line <= body["bodySpan"]["endLine"].as_u64().unwrap()
            }).collect();
            let initializers: Vec<_> = matching.iter().filter_map(|index| match &trace.domain.events[*index].instruction {
                Instruction::Property{id,..} => Some((vec![id.clone()],"PropertyRef")),
                Instruction::Node{id,..} => Some((vec![id.clone()],"NodeRef")),
                Instruction::Edge{id,..} => Some((vec![id.clone()],"EdgeRef")),
                Instruction::Constants{values,..} => Some((values.clone(),"Vec<ConstantRef>")), _ => None,
            }).collect();
            if initializers.len() != 1 { return Err(invalid(format!("{name}.{field_name} lacks one complete source-bound initializer"))); }
            let (ids, kind) = initializers.into_iter().next().unwrap(); let field_name = crate::codegen::field_name(field_name);
            for id in &ids { if kind != "Vec<ConstantRef>" && owners.insert(id.clone(),(name.clone(),field_name.clone())).is_some() { return Err(invalid("object exported by multiple schema fields")); } }
            fields.push(Field { name: field_name, ids, kind, body: body.clone(), events: matching });
        }
        let schema_info = events.iter().find_map(|index| metadata(&trace.domain.events[*index].instruction)).ok_or_else(||invalid("schema module lacks metadata"))?.clone();
        modules.push(Module { name, source: source.clone(), class: class.clone(), parameters: dependencies, fields, events, info: schema_info });
    }
    let mut tokens = rust::Tokens::new(); let mut bindings = vec![]; let mut event_symbols = BTreeMap::new();
    for module in &modules {
        let class = format!("{}Schema",module.name); let class = class.as_str(); let mut fields = rust::Tokens::new();
        for field in &module.fields { let name=field.name.as_str();let ty=field.kind;quote_in! {fields=>pub $name:$ty,}; }
        marker(&mut tokens,"MODULE",&module.name); quote_in! {tokens=>#[derive(Clone,Debug)] pub struct $class {$fields}};
        let mut parameters = rust::Tokens::new(); let mut arguments = rust::Tokens::new();
        for (parameter,kind) in &module.parameters { let parameter=parameter.as_str();let kind=format!("{kind}Schema");quote_in! {parameters=>$parameter:&$kind,};quote_in! {arguments=>$parameter,}; }
        let mut body = rust::Tokens::new();
        let definitions: BTreeSet<_> = module.events.iter().filter_map(|index| definition(&trace.domain.events[*index].instruction)).collect();
        let foreign: BTreeSet<_> = module.events.iter().flat_map(|index| operands(&trace.domain.events[*index].instruction)).filter(|id|!definitions.contains(id)).collect();
        for id in foreign {
            let var=variable(reference(symbols,id)?)?;
            if any_ids.contains(id) { quote_in! {body=>let $var=builder.any_node();}; continue; }
            let (owner,field) = owners.get(id).ok_or_else(||invalid(format!("foreign schema operand {id} has no public module field")))?;
            let parameter = module.parameters.iter().find(|(_,kind)|kind==owner).ok_or_else(||invalid(format!("{} references undeclared dependency {owner}",module.name)))?.0.as_str();
            let field=field.as_str();quote_in! {body=>let $var=$parameter.$field.clone();};
        }
        let mut used_info = BTreeSet::new();
        for index in &module.events {
            if let Some(metadata) = metadata(&trace.domain.events[*index].instruction) {
                let key=serde_json::to_string(metadata)?;
                if used_info.insert(key.clone()) { let var=infos[&key].as_str();let value=info(metadata);quote_in! {body=>let $var=$value;}; }
            }
        }
        for index in &module.events { let operation=&operations[index];quote_in! {body=>$operation};event_symbols.insert(*index,format!("{}Schema::new",module.name)); }
        let mut result = rust::Tokens::new();
        for field in &module.fields {
            let name=field.name.as_str();
            let value=if field.kind == "Vec<ConstantRef>" {vector(field.ids.iter().map(|id| {let var=variable(&symbols[id]).unwrap();quote! {$var.clone()}}))}
                else {let var=variable(&symbols[&field.ids[0]])?;quote! {$var}};
            quote_in! {result=>$name:$value,};
        }
        let info_value=info(&module.info);let doc_index=module.info.doc_index.to_string();let description=quoted(module.info.description.as_str());let provided=module.info.provided_by_frontend.to_string();
        quote_in! {tokens=>impl $class {
            pub fn new(builder:&mut ObjectSchemaBuilder,$(&parameters))->Result<Self>{$body Ok(Self{$result})}
            pub fn apply(builder:&mut ObjectSchemaBuilder,$(&parameters))->Result<Self>{Self::new(builder,$arguments)}
            pub fn schema_info()->SchemaInfo{$info_value}
        }
        impl SchemaBase for $class { fn doc_index(&self)->i32{$doc_index} fn description(&self)->&str{$description} fn provided_by_frontend(&self)->bool{$provided} }};
        marker(&mut tokens,"MODULE_END",&module.name);
        let source_body=|body:&Value,role:&str|json!({"sourceBodyId":body["id"],"sourcePath":module.source["path"],"sourceBlob":module.source["blob"],"sourceSpan":body["bodySpan"],"sourceBodySha256":body["bodySha256"],"module":module.name,"schemaModuleRole":role});
        bindings.push(source_body(&module.class,"constructor"));
        for field in &module.fields { let mut binding=source_body(&field.body,"field");binding["field"]=json!(field.name);binding["events"]=json!(field.events);bindings.push(binding); }
        for body in module.source["bodies"].as_array().unwrap() {
            let qualified=body["qualifiedName"].as_str().unwrap();
            let role=if qualified==format!("{}::apply",module.name){Some("apply")}
                else if qualified==format!("{}::Schema::schemaInfo",module.name){Some("schema_info")}
                else if ["docIndex","description","providedByFrontend"].iter().any(|name|qualified==format!("{}::{name}",module.name)){Some("metadata")}else{None};
            if let Some(role)=role {bindings.push(source_body(body,role));}
        }
    }
    let mut population=rust::Tokens::new();let mut composition_fields=rust::Tokens::new();let mut composition_values=rust::Tokens::new();let mut available=BTreeMap::new();
    for module in &modules {
        let name=crate::codegen::field_name(&module.name);let name_ref=name.as_str();let class=format!("{}Schema",module.name);let class=class.as_str();
        let mut arguments=rust::Tokens::new();for (_,kind) in &module.parameters {let argument=available.get(kind).ok_or_else(||invalid("schema composition dependency is not initialized"))?;quote_in! {arguments=>&$argument,};}
        quote_in! {population=>let $name_ref=$class::new(builder,$arguments)?;};quote_in! {composition_fields=>pub $name_ref:$class,};quote_in! {composition_values=>$name_ref,};
        available.insert(module.name.clone(),name);
    }
    marker(&mut tokens,"COMPOSITION","CpgSchema");
    quote_in! {tokens=>#[derive(Clone,Debug)] pub struct CpgSchema{$composition_fields}
        impl CpgSchema {pub fn new(builder:&mut ObjectSchemaBuilder)->Result<Self>{$population Ok(Self{$composition_values})}}
        pub fn populate(builder:&mut ObjectSchemaBuilder)->Result<()>{CpgSchema::new(builder)?;Ok(())}};
    marker(&mut tokens,"COMPOSITION_END","CpgSchema");
    Ok(ModuleOutput { tokens, bindings, event_symbols, source_digests, modules:modules.len(), fields:modules.iter().map(|module|module.fields.len()).sum() })
}

pub(super) fn bind(output: &ModuleOutput, rendered: &str) -> Result<Vec<Value>> {
    let mut bindings=vec![];
    for binding in &output.bindings {
        let module=binding["module"].as_str().unwrap();
        let begin=format!("// CPG_SCHEMA_MODULE:{module}\n");let end=format!("// CPG_SCHEMA_MODULE_END:{module}\n");
        let start=rendered.find(&begin).ok_or_else(||invalid("missing schema module start"))?;
        let end=rendered.find(&end).ok_or_else(||invalid("missing schema module end"))?;
        let mut binding=binding.clone();binding["rustSymbol"]=json!(format!("{module}Schema::new"));binding["path"]=json!("schema_constructors.rs");
        binding["startByte"]=json!(start);binding["endByteExclusive"]=json!(end);binding["sha256"]=json!(content_digest(&rendered.as_bytes()[start..end]));bindings.push(binding);
    }
    Ok(bindings)
}
