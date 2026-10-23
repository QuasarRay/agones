//! Body identities and retained ASTs for every required handwritten CPG source.
use cpg_rs::{Error, Result};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
use tree_sitter::{Node, Parser};

fn hash(bytes: &[u8]) -> String { cpg_rs::content_digest(bytes) }
fn text(node: Node<'_>, bytes: &[u8]) -> String {
    String::from_utf8_lossy(&bytes[node.byte_range()]).into_owned()
}
fn span(node: Node<'_>) -> Value {
    json!({"startByte":node.start_byte(),"endByteExclusive":node.end_byte(),
           "startLine":node.start_position().row+1,"endLine":node.end_position().row+1,
           "startColumnBytes":node.start_position().column,"endColumnBytes":node.end_position().column})
}
struct Inventory<'a> {
    binding: &'a str,
    source: &'a [u8],
    ast: Vec<Value>,
    bodies: Vec<Value>,
    declarations: Vec<Value>,
    errors: Vec<Value>,
}
impl Inventory<'_> {
    fn visit(&mut self, node: Node<'_>, owners: &[String], enclosing_body: Option<&str>) -> usize {
        let index = self.ast.len();
        self.ast.push(Value::Null);
        if node.is_error() || node.is_missing() {
            self.errors.push(json!({"kind":node.kind(),"missing":node.is_missing(),"span":span(node),"text":text(node,self.source)}));
        }
        let kind = node.kind();
        let is_owner = matches!(kind, "class_definition"|"trait_definition"|"object_definition"|"enum_definition"|"package_object"|
            "class_declaration"|"interface_declaration"|"enum_declaration");
        let is_function = matches!(kind, "function_definition"|"function_declaration"|"method_declaration"|"constructor_declaration");
        let is_initializer = matches!(kind, "val_definition"|"var_definition"|"variable_declarator"|"static_initializer");
        let is_lambda = matches!(kind, "lambda_expression"|"anonymous_function") ||
            kind == "case_block" && node.parent().is_some_and(|parent| !matches!(parent.kind(),"match_expression"|"try_expression"|"catch_clause"));
        let is_statement = node.parent().is_some_and(|parent| matches!(parent.kind(),"compilation_unit"|"template_body"|"class_body")) &&
            matches!(kind,"call_expression"|"infix_expression"|"assignment_expression"|"expression_statement");
        let name = node.child_by_field_name("name").or_else(|| node.child_by_field_name("pattern"))
            .map(|name| text(name,self.source)).unwrap_or_else(|| format!("{}{}@{}",'$',kind,node.start_byte()));
        let qualified = owners.iter().chain(std::iter::once(&name)).cloned().collect::<Vec<_>>().join("::");
        let body = node.child_by_field_name("body").or_else(|| node.child_by_field_name("value"));
        let body_kind = if is_owner && body.is_some() { Some("CONSTRUCTION") }
            else if is_function && body.is_some() { Some("FUNCTION") }
            else if is_initializer { Some("VALUE_INITIALIZER") }
            else if is_lambda { Some("LAMBDA_OR_PARTIAL_FUNCTION") }
            else if is_statement { Some("CONSTRUCTION_STATEMENT") } else { None };
        let mut next_body = enclosing_body.map(str::to_owned);
        if is_owner || is_function || is_initializer || matches!(kind,"type_definition"|"val_declaration"|"var_declaration") {
            self.declarations.push(json!({"kind":kind,"qualifiedName":qualified,"span":span(node),"astNode":index}));
        }
        if let Some(body_kind) = body_kind {
            let executable = body.unwrap_or(node);
            let identity = format!("{}\0{qualified}\0{kind}\0{}:{}",self.binding,node.start_byte(),node.end_byte());
            let id = hash(identity.as_bytes());
            self.bodies.push(json!({"id":id,"qualifiedName":qualified,"kind":body_kind,"syntaxKind":kind,
                "declarationSpan":span(node),"bodySpan":span(executable),"astNode":index,
                "bodySha256":hash(&self.source[executable.byte_range()]),"enclosingBody":enclosing_body,
                "portStatus":"UNMAPPED"}));
            next_body = Some(id);
        }
        let mut nested_owners = owners.to_vec();
        if is_owner || is_function || is_initializer || is_lambda { nested_owners.push(name); }
        let mut cursor = node.walk();
        let children: Vec<_> = node.named_children(&mut cursor).map(|child|
            self.visit(child,&nested_owners,next_body.as_deref())).collect();
        self.ast[index] = json!({"kind":kind,"span":span(node),"children":children});
        index
    }
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 { return Err(Error::Invalid("usage: cpg-source-inventory <agones-repository-root> <output.json>".into())); }
    let root = Path::new(&args[1]);
    let classification_bytes = std::fs::read(root.join("verification/agones/first-generation-cpg-classification.json"))?;
    let classification: Value = serde_json::from_slice(&classification_bytes)?;
    let files = classification["files"].as_array().ok_or_else(|| Error::Invalid("classification lacks file inventory".into()))?;
    let mut scala = Parser::new();
    scala.set_language(&tree_sitter_scala::LANGUAGE.into()).map_err(|error| Error::Invalid(error.to_string()))?;
    let mut java = Parser::new();
    java.set_language(&tree_sitter_java::LANGUAGE.into()).map_err(|error| Error::Invalid(error.to_string()))?;
    let mut results = Vec::new();
    let mut unresolved = Vec::new();
    let mut kinds = BTreeMap::<String,usize>::new();
    let mut total_bodies = 0;
    for file in files.iter().filter(|file| file["required_handwritten_port_accounting"] == true) {
        let path = file["path"].as_str().ok_or_else(|| Error::Invalid("file identity lacks path".into()))?;
        let extension = Path::new(path).extension().and_then(|value| value.to_str()).unwrap_or("");
        let parser = match extension {
            "scala"|"sbt" => &mut scala,
            "java" => &mut java,
            _ => { unresolved.push(json!({"path":path,"reason":"BUILD_LANGUAGE_OR_LAUNCHER_EQUIVALENT_REQUIRES_MAPPING","blob":file["git_blob"]})); continue; }
        };
        let source = std::fs::read(root.join("third_party/first-generation/cpg").join(path))?;
        if hash(&source) != file["sha256"].as_str().unwrap_or("") { return Err(Error::Invalid(format!("changed CPG source {path}"))); }
        let tree = parser.parse(&source,None).ok_or_else(|| Error::Invalid(format!("parser cancelled for {path}")))?;
        let binding = format!("{}\0{path}\0{}",classification["source_commit"],file["git_blob"]);
        let mut inventory = Inventory { binding:&binding,source:&source,ast:vec![],bodies:vec![],declarations:vec![],errors:vec![] };
        inventory.visit(tree.root_node(),&[],None);
        if !inventory.errors.is_empty() { unresolved.push(json!({"path":path,"reason":"PARSER_DIAGNOSTICS","errors":inventory.errors})); }
        for body in &inventory.bodies { *kinds.entry(body["kind"].as_str().unwrap_or("").into()).or_default() += 1; }
        total_bodies += inventory.bodies.len();
        results.push(json!({"path":path,"blob":file["git_blob"],"sha256":file["sha256"],"role":file["role"],
            "parserLanguage":extension,"declarations":inventory.declarations,"bodies":inventory.bodies,"ast":inventory.ast,
            "parserErrors":inventory.errors}));
    }
    let result = json!({"schemaVersion":1,"sourceCommit":classification["source_commit"],
        "classificationSha256":hash(&classification_bytes),"files":results,"bodyCandidates":total_bodies,
        "bodyKinds":kinds,"requiredUnresolved":unresolved,"portCompletion":"UNMAPPED",
        "scope":"Conservative syntax inventory includes local initializers and singleton/class construction. Implicit placeholder closures require semantic reconciliation against retained ASTs."});
    let output = Path::new(&args[2]);
    if output.exists() { return Err(Error::Invalid("recover the existing source-bound inventory instead of overwriting it".into())); }
    std::fs::write(output,serde_json::to_vec_pretty(&result)?)?;
    println!("{}",json!({"files":results.len(),"bodyCandidates":total_bodies,"unresolvedFiles":unresolved.len()}));
    Ok(())
}
