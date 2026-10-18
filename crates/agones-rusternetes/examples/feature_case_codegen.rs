//! Narrow, deterministic Genco emission from the gated feature-query IR.
//! Run with --check to verify the owned generated source and metadata.
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use genco::{lang::rust, prelude::*, tokens::quoted};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
struct Family {
    schema_version: u32,
    classification: String,
    feature_id: String,
    cases: Vec<Case>,
    defaults: Vec<DefaultGate>,
    implementation_gate: String,
    implementation_gate_sha256: String,
    input_implementation_gate: Option<String>,
    input_implementation_gate_sha256: Option<String>,
    source_files: std::collections::BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct DefaultGate {
    name: String,
    enabled: bool,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    disposition: String,
    assertion_count: usize,
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct Step {
    kind: Kind,
    #[serde(default)]
    value: String,
    enabled: bool,
    origin: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Kind {
    Parse,
    ParseEncoded,
    EnableAll,
    AssertNoError,
    AssertError,
    AssertErrorContains,
    AssertEnabled,
    AssertEncoding,
    SetEnv,
    BindFlags,
    ParseFlags,
    SetEnvKeyReplacer,
    BindEnv,
    ParseEnv,
}

impl Kind {
    fn is_input(&self) -> bool {
        matches!(
            self,
            Self::SetEnv
                | Self::BindFlags
                | Self::ParseFlags
                | Self::SetEnvKeyReplacer
                | Self::BindEnv
                | Self::ParseEnv
        )
    }
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn validate_gate(root: &Path, path: &str, expected: &str) {
    let raw = fs::read(root.join(path)).expect("gate is available");
    assert_eq!(hash(&raw), expected, "gate content binding");
    let gate: Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(gate["status"], "IMPLEMENTED");
    let files = gate["production_files"]
        .as_object()
        .expect("production closure");
    assert!(!files.is_empty());
    for (path, expected) in files {
        assert_eq!(
            hash(&fs::read(root.join(path)).unwrap()),
            expected.as_str().unwrap(),
            "stale gate: {path}"
        );
    }
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let raw = fs::read(root.join("verification/agones/feature-gates.ir.json")).unwrap();
    let family: Family = serde_json::from_slice(&raw).unwrap();
    assert_eq!(family.schema_version, 1, "unsupported typed IR schema");
    assert_eq!(family.classification, "EXACT_CASE");
    assert_eq!(family.feature_id, "FG-CONFIG-001");
    validate_gate(
        root,
        &family.implementation_gate,
        &family.implementation_gate_sha256,
    );
    if let Some(path) = &family.input_implementation_gate {
        validate_gate(
            root,
            path,
            family
                .input_implementation_gate_sha256
                .as_deref()
                .expect("input gate binding"),
        );
    }
    for (path, expected) in &family.source_files {
        assert_eq!(
            &hash(&fs::read(root.join(path)).unwrap()),
            expected,
            "stale source IR: {path}"
        );
    }
    assert_eq!(family.defaults.len(), 2);
    assert_eq!(
        (&*family.defaults[0].name, family.defaults[0].enabled),
        ("Example", true)
    );
    assert_eq!(
        (&*family.defaults[1].name, family.defaults[1].enabled),
        ("Test", false)
    );

    let mut arms = rust::Tokens::new();
    let mut assertion_functions = rust::Tokens::new();
    let mut case_functions = rust::Tokens::new();
    let mut harnesses = Vec::new();
    let mut claims = Vec::new();
    let mut case_ids = Vec::new();
    let mut excluded = Vec::new();
    for case in family.cases {
        if case.steps.iter().any(|s| s.kind.is_input()) {
            excluded.push(json!({"id":case.id, "assertions":case.assertion_count, "required":true, "reason":"OS/command binding effects are explicit external model boundaries. Runtime and process replay are separate; this generated proof selects only query cases."}));
            continue;
        }
        assert_eq!(case.disposition, "READY");
        let index = case_ids.len();
        case_ids.push(case.id.clone());
        let mut body = rust::Tokens::new();
        let before = claims.len();
        for step in case.steps {
            let value = quoted(step.value.as_str());
            let condition: Option<rust::Tokens> = match step.kind {
                Kind::Parse => {
                    body.append(quote! { last_result = features.replace(&context, $value); });
                    None
                }
                Kind::ParseEncoded => {
                    body.append(
                        quote! { last_result = features.replace(&context, &features.encode()); },
                    );
                    None
                }
                Kind::EnableAll => {
                    body.append(quote! { features = context.all_features_enabled(); });
                    None
                }
                Kind::AssertNoError => Some(quote! { last_result.is_ok() }),
                Kind::AssertError => Some(quote! { last_result.is_err() }),
                Kind::AssertErrorContains => {
                    Some(quote! { last_result.as_ref().unwrap_err().to_string().contains($value) })
                }
                Kind::AssertEnabled => {
                    let expected = if step.enabled { "true" } else { "false" };
                    Some(quote! { features.enabled($value) == $expected })
                }
                Kind::AssertEncoding => Some(quote! { features.encode() == $value }),
                _ => unreachable!("input cases were explicitly excluded"),
            };
            if let Some(condition) = condition {
                let function = format!("source_assertion_{:03}", claims.len());
                let function_name = function.as_str();
                let origin = quoted(step.origin.as_str());
                assertion_functions.append(quote! {
                    fn $function_name(observed_relation: bool) { assert!(observed_relation, $origin); }
                });
                body.append(quote! { $function_name($condition); });
                claims.push(json!({"function":function,"origin":step.origin,"kind":format!("{:?}",step.kind),"value":step.value,"enabled":step.enabled}));
            }
        }
        assert_eq!(claims.len() - before, case.assertion_count, "{}", case.id);
        let cover_text = format!("source query case: {}", case.id);
        let cover = quoted(cover_text.as_str());
        let case_function = format!("check_source_query_case_{index:03}");
        let case_name = case_function.as_str();
        let harness = format!("source_query_case_{index:03}");
        let harness_name = harness.as_str();
        case_functions.append(quote! {
            fn $case_name() {
                let context = agones_core::FeatureGateContext { feature_gate_defaults: &[("Example", true), ("Test", false)] };
                let mut features = agones_core::FeatureGates::default();
                let mut last_result = Ok(());
                $body
                #[cfg(kani)] { kani::cover!(true, $cover); }
            }
            #[test]
            #[cfg_attr(kani, kani::proof)]
            #[cfg_attr(kani, kani::unwind(64))]
            fn $harness_name() {
                bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| $case_name());
            }
        });
        arms.append(quote! { $index => $case_name(), });
        harnesses.push(json!({"harness":harness,"case_id":case.id,"assertion_functions":claims[before..].iter().map(|c|c["function"].as_str().unwrap()).collect::<Vec<_>>(),"required_covers":[cover_text,"the generator should produce at least one valid value"],"raw_domain":"unit","unwind":64}));
    }
    assert_eq!(case_ids.len(), 5, "exact nonempty query selection");
    assert_eq!(claims.len(), 17, "all query source assertions selected");
    assert_eq!(excluded.len(), 1, "input effect boundary remains visible");
    let tokens: rust::Tokens = quote! {
        // Generated by feature_case_codegen from gated typed IR. Do not edit.
        use agones_core::CanParseFeatureGates;
        $assertion_functions
        $case_functions
        fn check_source_query_case(index: u8) {
            match index {
                $arms
                _ => unreachable!("the shared domain projects to cases 0..5"),
            }
        }
    };
    let unformatted = format!(
        "// Generated by feature_case_codegen from gated typed IR. Do not edit.\n{}",
        tokens.to_string().unwrap()
    );
    let mut formatter = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("pinned Rust formatter");
    formatter
        .stdin
        .take()
        .unwrap()
        .write_all(unformatted.as_bytes())
        .unwrap();
    let formatted = formatter.wait_with_output().unwrap();
    assert!(formatted.status.success(), "Rust formatting failed");
    let source = String::from_utf8(formatted.stdout).unwrap();
    let meta = json!({
        "schema_version":1,"generator":"crates/agones-rusternetes/examples/feature_case_codegen.rs",
        "generator_sha256":hash(&fs::read(root.join("crates/agones-rusternetes/examples/feature_case_codegen.rs")).unwrap()),
        "ir":"verification/agones/feature-gates.ir.json","ir_sha256":hash(&raw),
        "output":"crates/agones-core/tests/generated/feature_gate_cases.rs","output_sha256":hash(source.as_bytes()),
        "harness_prefix":"source_query_case_","harnesses":harnesses,"case_ids":case_ids,"assertions":claims,
        "required_covers":case_ids.iter().map(|id|format!("source query case: {id}")).chain(std::iter::once("the generator should produce at least one valid value".into())).collect::<Vec<_>>(),
        "classification":"EXACT_CASE","raw_domain":"disjoint union of five fixed source cases; one unit value per case under both engines","projection":"one generated harness per exact case","preconditions":[],"unwind":64,
        "required_unmodeled_cases":excluded,
        "scope":"Actual production query parser and exact source assertions for all five selected cases. No unbounded input generalization, alternate production implementation, OS/command proof, or translated reference-model proof is credited. Original Go tests apply the same independent source relations separately."
    });
    let metadata = serde_json::to_string_pretty(&meta).unwrap() + "\n";
    let check = std::env::args()
        .skip(1)
        .any(|argument| argument == "--check");
    for (path, contents) in [
        (
            "crates/agones-core/tests/generated/feature_gate_cases.rs",
            source,
        ),
        ("verification/agones/feature-query-codegen.json", metadata),
    ] {
        let path = root.join(path);
        if check {
            assert_eq!(
                fs::read_to_string(&path).expect("generated output exists"),
                contents,
                "generation drift: {}",
                path.display()
            );
        } else {
            fs::write(path, contents).unwrap();
        }
    }
    println!(
        "{} five query cases, 17 source assertions; one required input case remains outside this formal model",
        if check { "Checked" } else { "Generated" }
    );
}
