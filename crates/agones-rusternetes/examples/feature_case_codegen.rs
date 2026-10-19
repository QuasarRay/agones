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

// This family emits concrete typed fixtures, not a JSON decoder in a proof.
// The whole decoded value and its source JSON must match the supported shape.
fn gs_enum(value: Option<&str>, kind: &str) -> rust::Tokens {
    match value {
        None => quote! { None },
        Some(value) => {
            let value = quoted(value);
            quote! { Some(agones_core::$kind::from($value)) }
        }
    }
}

fn gs_health(value: &agones_core::Health) -> rust::Tokens {
    let disabled = value.disabled.to_string();
    let period = format!("{}i32", value.period_seconds);
    let failure = format!("{}i32", value.failure_threshold);
    let delay = format!("{}i32", value.initial_delay_seconds);
    quote! { agones_core::Health { disabled: $disabled, period_seconds: $period,
    failure_threshold: $failure, initial_delay_seconds: $delay } }
}

fn gs_sdk(value: &agones_core::SdkServer) -> rust::Tokens {
    let log = gs_enum(
        value.log_level.as_ref().map(|v| v.as_str()),
        "SdkServerLogLevel",
    );
    let grpc = format!("{}i32", value.grpc_port);
    let http = format!("{}i32", value.http_port);
    quote! { agones_core::SdkServer { log_level: $log, grpc_port: $grpc, http_port: $http } }
}

fn gs_map(value: &Value, lists: bool) -> rust::Tokens {
    if value.is_null() {
        return quote! { None };
    }
    let mut entries = rust::Tokens::new();
    for (key, item) in value.as_object().expect("source string-keyed map") {
        let key = quoted(key.as_str());
        let capacity = format!("{}i64", item["capacity"].as_i64().unwrap());
        let value = if lists {
            let values = if item["values"].is_null() {
                quote! { agones_core::GoSlice::default() }
            } else {
                let mut elements = rust::Tokens::new();
                for value in item["values"].as_array().unwrap() {
                    let value = quoted(value.as_str().unwrap());
                    elements.append(quote! { $value.to_owned(), });
                }
                quote! { vec![$elements].into() }
            };
            quote! { agones_core::ListStatus { capacity: $capacity, values: $values } }
        } else {
            let count = format!("{}i64", item["count"].as_i64().unwrap());
            quote! { agones_core::CounterStatus { count: $count, capacity: $capacity } }
        };
        entries.append(quote! { values.insert($key.to_owned(), $value); });
    }
    // Match source make/map assignment without invoking the unrelated bulk
    // sorted-tree builder for a fixed literal. The complete value is checked
    // against source JSON by the ordinary fixture/output test.
    quote! { Some({ let mut values = std::collections::BTreeMap::new(); $entries values }) }
}

fn gs_fixture(input: &Value, function: &str) -> rust::Tokens {
    use agones_core::{Container, GameServer};
    let typed: GameServer = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(
        serde_json::to_value(&typed).unwrap(),
        *input,
        "source fixture must round-trip completely before typed lowering"
    );
    let mut supported = GameServer::default();
    supported.spec.container = typed.spec.container.clone();
    supported.spec.ports = typed.spec.ports.clone();
    supported.spec.health = typed.spec.health.clone();
    supported.spec.sdk_server = typed.spec.sdk_server.clone();
    supported.spec.scheduling = typed.spec.scheduling.clone();
    supported.spec.eviction = typed.spec.eviction.clone();
    supported.spec.counters = typed.spec.counters.clone();
    supported.spec.lists = typed.spec.lists.clone();
    supported.spec.template.spec.containers = typed.spec.template.spec.containers.clone();
    supported.spec.template.metadata.annotations = typed.spec.template.metadata.annotations.clone();
    supported.status.state = typed.status.state.clone();
    assert_eq!(supported, typed, "unsupported whole source fixture shape");
    let container = quoted(typed.spec.container.as_str());
    let mut ports = rust::Tokens::new();
    for port in &typed.spec.ports {
        let name = quoted(port.name.as_str());
        let range = quoted(port.range.as_str());
        let policy = gs_enum(port.port_policy.as_ref().map(|v| v.as_str()), "PortPolicy");
        let protocol = gs_enum(port.protocol.as_ref().map(|v| v.as_str()), "Protocol");
        let owner = match &port.container {
            None => quote! { None },
            Some(value) => {
                let value = quoted(value.as_str());
                quote! { Some($value.to_owned()) }
            }
        };
        let container_port = format!("{}i32", port.container_port);
        let host_port = format!("{}i32", port.host_port);
        ports.append(
            quote! { agones_core::GameServerPort { name: $name.to_owned(),
            range: $range.to_owned(), port_policy: $policy, protocol: $protocol,
            container: $owner, container_port: $container_port, host_port: $host_port }, },
        );
    }
    let ports = if typed.spec.ports.is_nil() {
        quote! { agones_core::GoSlice::default() }
    } else {
        quote! { vec![$ports].into() }
    };
    let mut containers = rust::Tokens::new();
    for value in &typed.spec.template.spec.containers {
        assert_eq!(
            *value,
            Container {
                name: value.name.clone(),
                image: value.image.clone(),
                ..Container::default()
            },
            "unsupported source Container field"
        );
        let name = quoted(value.name.as_str());
        let image = quoted(value.image.as_str());
        containers.append(quote! { agones_core::Container { name: $name.to_owned(),
        image: $image.to_owned(), ..Default::default() }, });
    }
    let containers = if typed.spec.template.spec.containers.is_nil() {
        quote! { agones_core::GoSlice::default() }
    } else {
        quote! { vec![$containers].into() }
    };
    let mut annotations = rust::Tokens::new();
    for (key, value) in &typed.spec.template.metadata.annotations {
        let key = quoted(key.as_str());
        let value = quoted(value.as_str());
        annotations.append(quote! { ($key.to_owned(), $value.to_owned()), });
    }
    let health = gs_health(&typed.spec.health);
    let sdk = gs_sdk(&typed.spec.sdk_server);
    let scheduling = gs_enum(
        typed.spec.scheduling.as_ref().map(|v| v.as_str()),
        "SchedulingStrategy",
    );
    let state = gs_enum(
        typed.status.state.as_ref().map(|v| v.as_str()),
        "GameServerState",
    );
    let eviction = match &typed.spec.eviction {
        None => quote! { None },
        Some(value) => {
            let safe = gs_enum(value.safe.as_ref().map(|v| v.as_str()), "EvictionSafe");
            quote! { Some(agones_core::Eviction { safe: $safe }) }
        }
    };
    let counters = gs_map(&input["spec"]["counters"], false);
    let lists = gs_map(&input["spec"]["lists"], true);
    quote! {
        fn $function() -> agones_core::GameServer {
            let mut value = agones_core::GameServer::default();
            value.spec.container = $container.to_owned();
            value.spec.ports = $ports;
            value.spec.health = $health;
            value.spec.sdk_server = $sdk;
            value.spec.scheduling = $scheduling;
            value.spec.eviction = $eviction;
            value.spec.counters = $counters;
            value.spec.lists = $lists;
            value.spec.template.spec.containers = $containers;
            value.spec.template.metadata.annotations = std::collections::BTreeMap::from([$annotations]);
            value.status.state = $state;
            value
        }
    }
}

fn gs_guard(guard: &str, expected: &Value) -> bool {
    if let Some(inner) = guard.strip_prefix("!(").and_then(|s| s.strip_suffix(')')) {
        return !gs_guard(inner, expected);
    }
    let field = |name| {
        expected
            .get(name)
            .expect("missing source conditional context")
    };
    match guard {
        "test.expected.alphaPlayerCapacity != nil" => !field("alpha_capacity").is_null(),
        "len(test.expected.evictionSafeSpec) > 0" => {
            !field("eviction_spec").as_str().unwrap().is_empty()
        }
        "len(test.expected.evictionSafeStatus) > 0" => {
            !field("eviction_status").as_str().unwrap().is_empty()
        }
        "test.expected.counterSpec != nil" => !field("counter_spec").is_null(),
        "test.expected.listSpec != nil" => !field("list_spec").is_null(),
        other => panic!("unsupported source assertion guard: {other}"),
    }
}

fn gs_relation(site: &Value, assertion: &Value) -> rust::Tokens {
    let expected = &assertion["expected"];
    let kind = assertion["kind"].as_str().unwrap();
    assert_eq!(assertion["kind"], site["kind"]);
    assert_eq!(assertion["source_passed"], true);
    match kind {
        "Equal" => assert_eq!(assertion["observed"], *expected, "captured source relation"),
        "Contains" => assert!(assertion["observed"].as_array().unwrap().contains(expected)),
        "Nil" | "NoError" => assert!(expected.is_null() && assertion["observed"].is_null()),
        other => panic!("unsupported source relation: {other}"),
    }
    let text = || quoted(expected.as_str().expect("source string expected"));
    match (site["actual_expression"].as_str().unwrap(), kind) {
        ("err", "NoError") => quote! { parsed.is_ok() },
        ("test.gameServer.Annotations[VersionAnnotation]", "Equal") => {
            let expected = text();
            quote! { gs.metadata.annotations.get(agones_core::VERSION_ANNOTATION).map(String::as_str) == Some($expected) }
        }
        ("test.gameServer.ObjectMeta.Finalizers", "Contains") => {
            let expected = text();
            quote! { gs.metadata.finalizers.iter().any(|value| value == $expected) }
        }
        ("spec.Container", "Equal") => {
            let expected = text();
            quote! { gs.spec.container == $expected }
        }
        ("spec.Ports[0].Protocol", "Equal") => {
            let expected = text();
            quote! { gs.spec.ports[0].protocol.as_ref().map_or("", |v| v.as_str()) == $expected }
        }
        ("spec.Ports[0].Range", "Equal") => {
            let expected = text();
            quote! { gs.spec.ports[0].range == $expected }
        }
        ("test.gameServer.Status.State", "Equal") => {
            let expected = text();
            quote! { gs.status.state.as_ref().map_or("", |v| v.as_str()) == $expected }
        }
        ("test.gameServer.Spec.Scheduling", "Equal") => {
            let expected = text();
            quote! { gs.spec.scheduling.as_ref().map_or("", |v| v.as_str()) == $expected }
        }
        ("test.gameServer.Spec.Health", "Equal") => {
            let expected = gs_health(&serde_json::from_value(expected.clone()).unwrap());
            quote! { gs.spec.health == $expected }
        }
        ("test.gameServer.Spec.SdkServer", "Equal") => {
            let expected = gs_sdk(&serde_json::from_value(expected.clone()).unwrap());
            quote! { gs.spec.sdk_server == $expected }
        }
        ("test.gameServer.Status.Players.Capacity", "Equal") => {
            let expected = format!("{}i64", expected.as_i64().unwrap());
            quote! { gs.status.players.as_ref().map(|p| p.capacity) == Some($expected) }
        }
        ("spec.Eviction.Safe", "Equal") => {
            let expected = text();
            quote! { gs.spec.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == $expected }
        }
        ("test.gameServer.Status.Eviction.Safe", "Equal") => {
            let expected = text();
            quote! { gs.status.eviction.as_ref().and_then(|v| v.safe.as_ref()).map_or("", |v| v.as_str()) == $expected }
        }
        (expression, "Nil") => {
            let field = match expression {
                "test.gameServer.Spec.Players" => "gs.spec.players",
                "test.gameServer.Status.Players" => "gs.status.players",
                "spec.Eviction" => "gs.spec.eviction",
                "test.gameServer.Status.Eviction" => "gs.status.eviction",
                "test.gameServer.Spec.Counters" => "gs.spec.counters",
                "test.gameServer.Status.Counters" => "gs.status.counters",
                "test.gameServer.Spec.Lists" => "gs.spec.lists",
                "test.gameServer.Status.Lists" => "gs.status.lists",
                other => panic!("unsupported nil observation: {other}"),
            };
            quote! { $field.is_none() }
        }
        ("test.gameServer.Status.Counters", "Equal") => {
            let expected = gs_map(expected, false);
            quote! { std::cmp::PartialEq::eq(&gs.status.counters, &*std::mem::ManuallyDrop::new($expected)) }
        }
        ("test.gameServer.Status.Lists", "Equal") => {
            let expected = gs_map(expected, true);
            quote! { std::cmp::PartialEq::eq(&gs.status.lists, &*std::mem::ManuallyDrop::new($expected)) }
        }
        (expression, relation) => {
            panic!("unsupported source observation/relation: {expression}/{relation}")
        }
    }
}

fn emit_gameserver_cases(root: &Path, check: bool) {
    use std::collections::{BTreeMap, BTreeSet};
    let ir_path = "verification/agones/gameserver-defaulting.ir.json";
    let raw = fs::read(root.join(ir_path)).unwrap();
    let ir: Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(ir["schema_version"], 1);
    assert_eq!(ir["feature_id"], "GS-DEFAULTS-001");
    assert_eq!(ir["classification"], "EXACT_CASE_ASSERTIONS");
    validate_gate(
        root,
        ir["implementation_gate"].as_str().unwrap(),
        ir["implementation_gate_sha256"].as_str().unwrap(),
    );
    for (path, expected) in ir["source_files"].as_object().unwrap() {
        assert_eq!(
            hash(&fs::read(root.join(path)).unwrap()),
            expected.as_str().unwrap(),
            "stale source: {path}"
        );
    }
    assert_eq!(
        hash(ir["capture_program"].as_str().unwrap().as_bytes()),
        ir["capture_program_sha256"]
    );
    assert_eq!(
        ir["source_actions"],
        json!([
            "err := runtime.ParseFeatures(test.featureFlags)",
            "test.gameServer.ApplyDefaults()",
            "spec := test.gameServer.Spec"
        ])
    );
    let sites: BTreeMap<_, _> = ir["source_assertion_sites"]
        .as_array()
        .unwrap()
        .iter()
        .map(|site| (site["line"].as_u64().unwrap(), site))
        .collect();
    assert_eq!(sites.len(), 23);
    let cases = ir["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 17);
    let mut tokens = rust::Tokens::new();
    let mut ordinary = rust::Tokens::new();
    let mut assertions = Vec::new();
    let mut harnesses = Vec::new();
    let mut identities = BTreeSet::new();
    for (index, case) in cases.iter().enumerate() {
        let case_id = case["id"].as_str().unwrap();
        let fixture = format!("source_gs_fixture_{index:03}");
        tokens.append(gs_fixture(&case["before"], &fixture));
        let selected: Vec<_> = sites
            .values()
            .filter(|site| {
                site["guards"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|guard| gs_guard(guard.as_str().unwrap(), &case["expected_context"]))
            })
            .map(|site| site["line"].as_u64().unwrap())
            .collect();
        let captured = case["assertions"].as_array().unwrap();
        assert_eq!(
            selected,
            captured
                .iter()
                .map(|a| a["line"].as_u64().unwrap())
                .collect::<Vec<_>>(),
            "source conditional selection"
        );
        let mut steps = rust::Tokens::new();
        let flags = quoted(case["feature_flags"].as_str().unwrap());
        // The fixed input value is independent of parsing: neither operation
        // reads the other's state. For nonempty queries, delay its physical
        // allocation until first use, keeping unrelated heap out of parsing.
        // No original observation reads GameServer before NoError.
        let deferred_fixture = !case["feature_flags"].as_str().unwrap().is_empty();
        if deferred_fixture {
            steps.append(quote! { let parsed = agones_core::FeatureGateContext::default().parse_feature_gates($flags); });
        } else {
            steps.append(quote! { let mut gs = $(&fixture)();
            let parsed = agones_core::FeatureGateContext::default().parse_feature_gates($flags); });
        }
        let mut applied = false;
        let mut functions = Vec::new();
        for assertion in captured {
            let line = assertion["line"].as_u64().unwrap();
            let origin = assertion["id"].as_str().unwrap();
            assert_eq!(origin, format!("{case_id}::line-{line}"));
            assert!(
                identities.insert(origin.to_owned()),
                "duplicate source assertion identity"
            );
            if assertion["kind"] != "NoError" && !applied {
                if deferred_fixture {
                    steps.append(quote! { let mut gs = $(&fixture)(); });
                }
                steps.append(quote! { let context = agones_core::GameServerDefaultingContext::from_feature_gates(parsed.as_ref().unwrap());
                    gs.apply_defaults_with_context(&context); });
                applied = true;
            }
            assert_eq!(applied, line != 405, "source parse/default ordering");
            let function = format!("source_gs_assertion_{:04}", assertions.len());
            let message = quoted(origin);
            tokens.append(
                quote! { fn $(&function)(relation: bool) { assert!(relation, $message); } },
            );
            let relation = gs_relation(sites[&line], assertion);
            steps.append(quote! { $(&function)($relation); });
            assertions.push(
                json!({"function":function,"origin":origin,"case_id":case_id,"line":line,
                "kind":assertion["kind"],"actual_expression":sites[&line]["actual_expression"],
                "expected":assertion["expected"],"guards":sites[&line]["guards"]}),
            );
            functions.push(function);
        }
        assert!(applied);
        let cover_text = format!("source GameServer case reached all assertions: {case_id}");
        let cover = quoted(cover_text.as_str());
        steps.append(quote! { #[cfg(kani)] { kani::cover!(true, $cover); } });
        // The source observes state before callback return and imposes no
        // garbage-collection schedule. Retain harness-owned values after that
        // boundary in both engines; production calls/internal effects are real.
        // The ordinary whole-fixture test below still uses normal destruction.
        steps.append(quote! {
            std::mem::forget(gs);
            std::mem::forget(parsed);
            std::mem::forget(context);
        });
        let harness = format!("source_gs_case_{index:03}");
        tokens.append(quote! {
            #[test]
            #[cfg_attr(kani, kani::proof)]
            #[cfg_attr(kani, kani::unwind(64))]
            fn $(&harness)() {
                bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { $steps });
            }
        });
        let before = serde_json::to_string(&case["before"]).unwrap();
        let before = quoted(before.as_str());
        let after = serde_json::to_string(&case["after"]).unwrap();
        let after = quoted(after.as_str());
        let id = quoted(case_id);
        ordinary.append(quote! {
            {
                let mut value = $(&fixture)();
                let input: serde_json::Value = serde_json::from_str($before).unwrap();
                let decoded: agones_core::GameServer = serde_json::from_value(input.clone()).unwrap();
                assert_eq!(value, decoded, $id);
                assert_eq!(serde_json::to_value(&value).unwrap(), input, $id);
                let parsed = agones_core::FeatureGateContext::default().parse_feature_gates($flags).unwrap();
                value.apply_defaults_with_context(&agones_core::GameServerDefaultingContext::from_feature_gates(&parsed));
                let expected: serde_json::Value = serde_json::from_str($after).unwrap();
                assert_eq!(serde_json::to_value(value).unwrap(), expected, $id);
            }
        });
        harnesses.push(json!({"harness":harness,"case_id":case_id,"fixture":fixture,"assertion_functions":functions,
            "feature_flags":case["feature_flags"],"required_covers":[cover_text,"the generator should produce at least one valid value"],
            "raw_domain":"unit","preconditions":[],"unwind":64,
            "lifetime_rule":"GS-DEFAULTS-001-OBSERVATION-LIFETIME-1"}));
        if deferred_fixture {
            harnesses.last_mut().unwrap()["fixture_materialization"] =
                json!("INDEPENDENT_CONSTANT_AFTER_NO_ERROR");
        }
    }
    assert_eq!(assertions.len(), 304);
    tokens.append(quote! {
        #[cfg(not(kani))]
        #[test]
        fn source_gs_typed_fixtures_and_complete_outputs_match_capture() { $ordinary }
    });
    let source = format!(
        "// Generated from gated typed GameServer IR by feature_case_codegen --gameserver-defaulting. Do not edit.\nuse agones_core::CanParseFeatureGates;\n{}",
        format_rust(tokens.to_string().unwrap())
    );
    let output = "crates/agones-core/tests/generated/gameserver_defaulting_cases.rs";
    let metadata = json!({"schema_version":1,"feature_id":"GS-DEFAULTS-001","classification":"EXACT_CASE",
        "generator":"Genco 0.17.10 with pinned rustfmt","generator_path":"crates/agones-rusternetes/examples/feature_case_codegen.rs",
        "generator_sha256":hash(&fs::read(root.join("crates/agones-rusternetes/examples/feature_case_codegen.rs")).unwrap()),
        "ir":ir_path,"ir_sha256":hash(&raw),"output":output,"output_sha256":hash(source.as_bytes()),
        "case_count":cases.len(),"assertion_count":assertions.len(),"harnesses":harnesses,"assertions":assertions,
        "source_assertion_sites":ir["source_assertion_sites"],
        "zero_occurrence_sites_not_credited":sites.values().filter(|s|s["fixture_occurrences"]==0).collect::<Vec<_>>(),
        "fixture_mapping":"Whole captured canonical GameServer equals a typed constructor and round-trips exactly at generation and ordinary replay. Proof harnesses contain no JSON decoder. Source parse/assert/default/assert order and conditional assertion selection are retained.",
        "lifetime_mapping":{"rule":"GS-DEFAULTS-001-OBSERVATION-LIFETIME-1",
            "boundary":"After every original state observation and the case cover, retain the harness-owned GameServer, parsed feature result and context through std::mem::forget in both engines. The source callback specifies no GC/deallocation observation or schedule.",
            "production":"Actual parser and CGP/defaulting bodies, including all internal allocation/destruction effects, remain unchanged.",
            "expected_map_literals":"Construct fixed expected maps through make/insert and retain their owner through ManuallyDrop while invoking actual PartialEq. This matches the source expected fixture ownership; no expected field/relation is omitted.",
            "constant_fixture_materialization":"For nonempty queries, the independent fixed typed fixture is physically allocated after the parser/NoError observation and before its first use. It contains no feature/global effects, and the parser has no fixture capability. The source input value and action/observation order are unchanged; allocation/GC schedules are not asserted by the source callback.",
            "ordinary_cleanup":"The separate whole-fixture/output test exercises normal Rust destruction for every exact fixture.",
            "residual":"No formal post-observation heap-cleanup, application cleanup/lifecycle or executor credit; those obligations remain required separately."},
        "scope":"All 17 exact source cases and 304 executed occurrences only. Three zero-occurrence sites remain open. Sampling and generated/compiled harnesses give no formal credit. The 17 SDK version identities overlapping this family must not be double-counted; no generalized GameServer, admission, lifecycle or executor proof is claimed."});
    save_or_check(root, output, &source, check);
    save_or_check(
        root,
        "verification/agones/gameserver-defaulting-codegen.json",
        &(serde_json::to_string_pretty(&metadata).unwrap() + "\n"),
        check,
    );
    println!(
        "{} 17 exact GameServer cases and 304 source assertion occurrences; three zero-occurrence sites remain open",
        if check { "Checked" } else { "Generated" }
    );
}

#[cfg(test)]
mod gameserver_codegen_controls {
    use super::*;

    fn ir() -> Value {
        serde_json::from_str(include_str!(
            "../../../verification/agones/gameserver-defaulting.ir.json"
        ))
        .unwrap()
    }

    #[test]
    fn whole_fixture_lowering_rejects_lost_wire_and_unmodeled_values() {
        let original = ir()["cases"][0]["before"].clone();
        for (name, mut input) in [
            ("unknown root wire field", original.clone()),
            ("nil versus allocated status slice", original.clone()),
            ("container resources", original.clone()),
            ("signed port width", original.clone()),
        ] {
            match name {
                "unknown root wire field" => input["unknownField"] = json!(true),
                "nil versus allocated status slice" => input["status"]["addresses"] = json!([]),
                "container resources" => {
                    input["spec"]["template"]["spec"]["containers"][0]["resources"] =
                        json!({"limits":{"cpu":"1"}})
                }
                "signed port width" => {
                    input["spec"]["ports"][0]["containerPort"] = json!(2147483648i64)
                }
                _ => unreachable!(),
            }
            assert!(
                std::panic::catch_unwind(|| gs_fixture(&input, "fixture")).is_err(),
                "accepted {name}"
            );
        }
    }

    #[test]
    fn source_relations_and_conditional_context_fail_closed() {
        let data = ir();
        let site = &data["source_assertion_sites"][1];
        let mut assertion = data["cases"][0]["assertions"][1].clone();
        assertion["expected"] = json!("changed independent expectation");
        assert!(std::panic::catch_unwind(|| gs_relation(site, &assertion)).is_err());
        let mut unsupported = site.clone();
        unsupported["actual_expression"] = json!("test.gameServer.Unsupported");
        let assertion = &data["cases"][0]["assertions"][1];
        assert!(std::panic::catch_unwind(|| gs_relation(&unsupported, assertion)).is_err());
        assert!(std::panic::catch_unwind(|| gs_guard("unknown source guard", &json!({}))).is_err());
        assert!(
            std::panic::catch_unwind(|| gs_guard(
                "test.expected.alphaPlayerCapacity != nil",
                &json!({})
            ))
            .is_err()
        );
        let expected = &data["cases"][0]["expected_context"];
        assert!(!gs_guard(
            "test.expected.alphaPlayerCapacity != nil",
            expected
        ));
        assert!(gs_guard(
            "!(test.expected.alphaPlayerCapacity != nil)",
            expected
        ));
        assert!(gs_guard("test.expected.counterSpec != nil", expected));
        assert!(!gs_guard("!(test.expected.counterSpec != nil)", expected));
    }
}

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    assert!(
        arguments.iter().all(|argument| argument == "--check"
            || argument == "--go-quote-table"
            || argument == "--sdk-version"
            || argument == "--gameserver-defaulting"),
        "unknown generation argument"
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let modes = [
        "--sdk-version",
        "--go-quote-table",
        "--gameserver-defaulting",
    ];
    assert!(
        modes
            .iter()
            .filter(|mode| arguments.iter().any(|arg| arg == **mode))
            .count()
            <= 1,
        "select one generation family"
    );
    if arguments
        .iter()
        .any(|argument| argument == "--gameserver-defaulting")
    {
        emit_gameserver_cases(root, arguments.iter().any(|argument| argument == "--check"));
        return;
    }
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
