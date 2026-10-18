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

fn format_rust(source: String) -> String {
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
        .write_all(source.as_bytes())
        .unwrap();
    let output = formatter.wait_with_output().unwrap();
    assert!(output.status.success(), "Rust formatting failed");
    String::from_utf8(output.stdout).unwrap()
}

fn save_or_check(root: &Path, path: &str, contents: &str, check: bool) {
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

fn emit_go_quote_table(root: &Path, check: bool) {
    let input = "verification/agones/go-print-table.json";
    let raw = fs::read(root.join(input)).unwrap();
    let data: Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(data["schema_version"], 1);
    assert_eq!(data["purpose"], "PRODUCTION_PRINTABLE_RUNE_DATA");
    assert_eq!(data["go_version"], "go1.27.1-X:nodwarf5");
    assert_eq!(data["unicode_version"], "17.0.0");
    let ranges: Vec<[u32; 2]> = serde_json::from_value(data["ranges"].clone()).unwrap();
    assert!(!ranges.is_empty());
    assert!(
        ranges
            .iter()
            .all(|pair| pair[0] <= pair[1] && pair[1] <= 0x10ffff)
    );
    assert!(ranges.windows(2).all(|pair| pair[0][1] + 1 < pair[1][0]));
    let bitmap =
        fs::read(root.join(data["independent_category_bitmap"].as_str().unwrap())).unwrap();
    assert_eq!(hash(&bitmap), data["independent_category_bitmap_sha256"]);
    assert_eq!(bitmap.len(), 0x110000 / 8);
    let mut entries = rust::Tokens::new();
    for [start, end] in &ranges {
        let start = format!("0x{start:x}");
        let end = format!("0x{end:x}");
        entries.append(quote! { ($start,$end), });
    }
    let tokens: rust::Tokens = quote! { const GO_PRINT_RANGES: &[(u32,u32)] = &[$entries]; };
    let source = format_rust(format!(
        "// Generated production data from Go strconv.IsPrint, Go 1.27.1 / Unicode 17.0.0.\n// Copyright 2009, 2013 The Go Authors. BSD license: ../../GO-STRCONV-LICENSE.\n// Regenerate through feature_case_codegen --go-quote-table; do not edit.\n{}",
        tokens.to_string().unwrap()
    ));
    let output = "crates/agones-core/src/generated/go_print_ranges.rs";
    let metadata = json!({"schema_version":1,"purpose":"PRODUCTION_DATA_COMPILATION","generator":"crates/agones-rusternetes/examples/feature_case_codegen.rs","generator_sha256":hash(&fs::read(root.join("crates/agones-rusternetes/examples/feature_case_codegen.rs")).unwrap()),"input":input,"input_sha256":hash(&raw),"output":output,"output_sha256":hash(source.as_bytes()),"range_count":ranges.len(),"go_source_hashes":data["source_hashes"],"independent_category_bitmap_sha256":data["independent_category_bitmap_sha256"],"assurance":"Production data and ordinary independent category validation only; no test-obligation extraction or formal proof credit."});
    save_or_check(root, output, &source, check);
    save_or_check(
        root,
        "verification/agones/go-print-codegen.json",
        &(serde_json::to_string_pretty(&metadata).unwrap() + "\n"),
        check,
    );
    println!(
        "{} {} Go printable-rune ranges",
        if check { "Checked" } else { "Generated" },
        ranges.len()
    );
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

fn emit_sdk_version_case(root: &Path, check: bool) {
    let ir_path = "verification/agones/sdk-version.ir.json";
    let raw = fs::read(root.join(ir_path)).unwrap();
    let ir: Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(ir["feature_id"], "SDK-VERSION-001");
    assert_eq!(ir["classification"], "EXACT_CASE_ASSERTION_PROJECTIONS");
    validate_gate(
        root,
        ir["implementation_gate"].as_str().unwrap(),
        ir["implementation_gate_sha256"].as_str().unwrap(),
    );
    for (path, expected) in ir["source_files"].as_object().unwrap() {
        assert_eq!(
            hash(&fs::read(root.join(path)).unwrap()),
            expected.as_str().unwrap(),
            "stale SDK source: {path}"
        );
    }
    let cases = ir["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 18);
    let selected = cases
        .iter()
        .filter(|case| case["kind"] == "FLEET_VERSION")
        .collect::<Vec<_>>();
    assert_eq!(selected.len(), 1);
    let case = selected[0];
    let assertions = case["assertion_ids"].as_array().unwrap();
    assert_eq!(assertions.len(), 2);
    assert_eq!(
        case["steps"],
        json!([
            "CONSTRUCT_EXACT_SOURCE_FIXTURE",
            "APPLY_DEFAULTS",
            "ASSERT_SDK_VERSION",
            "APPLY_DEFAULTS",
            "ASSERT_SDK_VERSION"
        ])
    );
    // The narrow typed constructor must equal the entire captured fixture.
    // Reject new fixture shapes rather than dropping fields or normalizing JSON.
    let typed: agones_core::Fleet = serde_json::from_value(case["input_json"].clone()).unwrap();
    let mut zero = agones_core::Fleet::default();
    zero.spec.scheduling = Some(agones_core::SchedulingStrategy::Unknown(String::new()));
    assert_eq!(typed, zero, "unsupported changed source Fleet zero fixture");
    let expected = quoted(case["expected_version"].as_str().unwrap());
    let fixture = serde_json::to_string(&case["input_json"]).unwrap();
    let fixture = quoted(fixture.as_str());
    let first = quoted(assertions[0].as_str().unwrap());
    let second = quoted(assertions[1].as_str().unwrap());
    let cover_text = format!("source SDK case: {}", case["id"].as_str().unwrap());
    let cover = quoted(cover_text.as_str());
    let tokens: rust::Tokens = quote! {
        fn source_sdk_fleet_fixture() -> agones_core::Fleet {
            let mut fleet = agones_core::Fleet::default();
            fleet.spec.scheduling = Some(agones_core::SchedulingStrategy::Unknown(String::new()));
            fleet
        }
        fn source_sdk_assertion_000(relation: bool) { assert!(relation, $first); }
        fn source_sdk_assertion_001(relation: bool) { assert!(relation, $second); }
        #[test]
        #[cfg_attr(kani, kani::proof)]
        #[cfg_attr(kani, kani::unwind(32))]
        fn source_sdk_fleet_case() {
            bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| {
                let context = agones_core::AgonesBuildVersionContext::from_bytes($expected.as_bytes());
                let mut fleet = source_sdk_fleet_fixture();
                fleet.apply_defaults_with_version_context(&context);
                source_sdk_assertion_000(fleet.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some($expected));
                fleet.apply_defaults_with_version_context(&context);
                source_sdk_assertion_001(fleet.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some($expected));
                #[cfg(kani)] { kani::cover!(true, $cover); }
            });
        }
        #[cfg(not(kani))]
        #[test]
        fn source_sdk_typed_fleet_fixture_matches_exact_capture() {
            let captured: agones_core::Fleet = serde_json::from_str($fixture).unwrap();
            assert_eq!(source_sdk_fleet_fixture(), captured);
        }
    };
    let source = format!(
        "// Generated from gated typed SDK IR by feature_case_codegen --sdk-version. Do not edit.\n{}",
        format_rust(tokens.to_string().unwrap())
    );
    let output = "crates/agones-core/tests/generated/sdk_version_cases.rs";
    let metadata = json!({
        "schema_version":1,"feature_id":"SDK-VERSION-001","generator":"Genco 0.17.10 with pinned rustfmt",
        "generator_sha256":hash(&fs::read(root.join("crates/agones-rusternetes/examples/feature_case_codegen.rs")).unwrap()),
        "ir":ir_path,"ir_sha256":hash(&raw),"output":output,"output_sha256":hash(source.as_bytes()),
        "harness":"source_sdk_fleet_case","case_id":case["id"],"assertions":[{"function":"source_sdk_assertion_000","origin":assertions[0]},{"function":"source_sdk_assertion_001","origin":assertions[1]}],
        "required_covers":[cover_text,"the generator should produce at least one valid value"],
        "raw_domain":"unit","unwind":32,"preconditions":[],
        "fixture_mapping":"Whole captured Fleet fixture equals the emitted typed constructor at generation and ordinary replay; no JSON decoder is substituted into production or claimed formally verified.",
        "required_unmodeled_cases":cases.iter().filter(|case| case["kind"] != "FLEET_VERSION").collect::<Vec<_>>(),
        "required_unselected_assertion_sites":ir["required_unselected_assertion_sites"],
        "scope":"Two exact source Fleet version observations only. All 17 GameServer version assertions and every other baseline obligation remain required. No generalized Fleet defaulting or lifecycle claim."
    });
    save_or_check(root, output, &source, check);
    save_or_check(
        root,
        "verification/agones/sdk-version-codegen.json",
        &(serde_json::to_string_pretty(&metadata).unwrap() + "\n"),
        check,
    );
    println!(
        "{} one exact Fleet case and two SDK assertions; 17 GameServer version assertions remain formal-required",
        if check { "Checked" } else { "Generated" }
    );
}

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    assert!(
        arguments.iter().all(|argument| argument == "--check"
            || argument == "--go-quote-table"
            || argument == "--sdk-version"),
        "unknown generation argument"
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    if arguments.iter().any(|argument| argument == "--sdk-version") {
        assert!(
            !arguments
                .iter()
                .any(|argument| argument == "--go-quote-table")
        );
        emit_sdk_version_case(root, arguments.iter().any(|argument| argument == "--check"));
        return;
    }
    if std::env::args().any(|argument| argument == "--go-quote-table") {
        emit_go_quote_table(root, std::env::args().any(|argument| argument == "--check"));
        return;
    }
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
    let source = format_rust(unformatted);
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
        save_or_check(root, path, &contents, check);
    }
    println!(
        "{} five query cases, 17 source assertions; one required input case remains outside this formal model",
        if check { "Checked" } else { "Generated" }
    );
}
