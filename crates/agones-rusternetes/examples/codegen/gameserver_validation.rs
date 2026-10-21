use super::*;

pub(super) fn emit_gameserver_validation_cases(root: &Path, check: bool) {
    let ir_path = "verification/agones/gameserver-validation.ir.json";
    let raw = fs::read(root.join(ir_path)).unwrap();
    let ir: Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(ir["feature_id"], "RESOURCE-QUANTITY-001");
    assert_eq!(ir["classification"], "EXACT_CASE_ASSERTIONS");
    assert_eq!(ir["case_count"], 25);
    assert_eq!(ir["source_assertion_occurrences"], 25);
    validate_gate(root, ir["implementation_gate"].as_str().unwrap(), ir["implementation_gate_sha256"].as_str().unwrap());
    let supporting = ir["supporting_implementation_gates"].as_object().expect("gated TypeMeta and complete error support");
    assert_eq!(supporting.len(), 2);
    for feature in ["GS-TYPEMETA-001", "GS-VALIDATION-ERROR-DATA-001"] {
        let binding = &supporting[feature];
        let path = binding["path"].as_str().unwrap();
        validate_gate(root, path, binding["sha256"].as_str().unwrap());
        let gate: Value = serde_json::from_slice(&fs::read(root.join(path)).unwrap()).unwrap();
        assert_eq!(gate["feature_id"], feature);
    }
    for (path, expected) in ir["source_files"].as_object().unwrap() {
        assert_eq!(hash(&fs::read(root.join(path)).unwrap()), expected.as_str().unwrap());
    }
    for (path, expected) in ir["loader_source_files"].as_object().unwrap() {
        assert_eq!(hash(&fs::read(root.join(path)).unwrap()), expected.as_str().unwrap(), "stale source loader");
    }
    assert_eq!(hash(ir["capture_program"].as_str().unwrap().as_bytes()), ir["capture_program_sha256"]);
    assert_eq!(ir["source_actions"], json!(["if tc.applyDefaults {\n\ttc.gs.ApplyDefaults()\n}", "errs := tc.gs.Validate(fakeAPIHooks{})", "assert.ElementsMatch(t, tc.want, errs, \"ErrorList check\")"]));
    let mut tokens = gameserver_features::validation_error_relation("source_gs_validate_errors_match");
    let mut ordinary = rust::Tokens::new();
    let mut assertions = Vec::new();
    let mut harnesses = Vec::new();
    let mut ids = std::collections::BTreeSet::new();
    let cases = ir["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 25);
    for (index, case) in cases.iter().enumerate() {
        assert_eq!(case["source_assertions_passed"], true);
        assert_eq!(case["quantity_construction_roundtrip_checked"], true);
        assert_eq!(case["expected_errors"], case["observed_errors"]);
        let id = case["id"].as_str().unwrap();
        assert!(ids.insert(id));
        let fixture = format!("source_gs_validate_fixture_{index:03}");
        let base = format!("source_gs_validate_base_{index:03}");
        let harness = format!("source_gs_validate_case_{index:03}");
        let assertion = format!("source_gs_validate_assertion_{index:03}");
        let original = case["assertions"].as_array().unwrap();
        assert_eq!(original.len(), 1);
        assert_eq!(original[0]["kind"], "ElementsMatch");
        let origin = quoted(original[0]["id"].as_str().unwrap());
        let presence = &case["fixture_presence"];
        assert_eq!(presence.as_object().unwrap().len(), 4);
        assert_eq!(presence["players_present"], false);
        let counters = presence["counters_present"].as_bool().unwrap();
        let lists = presence["lists_present"].as_bool().unwrap();
        let nil_ports = presence["ports_nil"].as_bool().unwrap();
        let typed: agones_core::GameServer = serde_json::from_value(case["before"].clone()).unwrap();
        assert_eq!(typed.spec.ports.is_nil(), nil_ports);
        let mut restore = rust::Tokens::new();
        if counters && typed.spec.counters.is_none() { restore.append(quote! { value.spec.counters = Some(std::collections::BTreeMap::new()); }); }
        if lists && typed.spec.lists.is_none() { restore.append(quote! { value.spec.lists = Some(std::collections::BTreeMap::new()); }); }
        tokens.append(gs_fixture(&case["before"], &base));
        tokens.append(quote! { fn $(&fixture)() -> agones_core::GameServer { let mut value = $(&base)(); $restore value } });
        let mut expected = rust::Tokens::new();
        if let Some(errors) = case["expected_errors"].as_array() {
            for error in errors { let error = gameserver_features::source_validation_error(error); expected.append(quote! { $error, }); }
        } else { assert!(case["expected_errors"].is_null()); }
        let apply = case["apply_defaults"].as_bool().unwrap().to_string();
        let cover_text = format!("source GameServer Validate case: {id}");
        let cover = quoted(cover_text.as_str());
        tokens.append(quote! {
            fn $(&assertion)(relation: bool) { assert!(relation, $origin); }
            #[test]
            #[cfg_attr(kani, kani::proof)]
            #[cfg_attr(kani, kani::unwind(128))]
            fn $(&harness)() {
                use agones_core::CanApplyGameServerDefaults;
                bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| {
                    let mut gs = $(&fixture)();
                    let context = agones_core::GameServerDefaultingContext::default();
                    if $(&apply) { context.apply_game_server_defaults(&mut gs); }
                    let actual = gs.validate_with_context(&context);
                    let expected = vec![$expected];
                    $(&assertion)(source_gs_validate_errors_match(&actual, &expected));
                    #[cfg(kani)] { kani::cover!(true, $cover); }
                    std::mem::forget((gs, context, actual, expected));
                });
            }
        });
        let before = quoted(serde_json::to_string(&case["before"]).unwrap());
        let after = quoted(serde_json::to_string(&case["after"]).unwrap());
        let replay = format!("source_gs_validate_whole_fixture_{index:03}");
        let counters = counters.to_string(); let lists = lists.to_string(); let nil_ports = nil_ports.to_string();
        ordinary.append(quote! {
            #[cfg(not(kani))]
            #[test]
            fn $replay() {
                use agones_core::CanApplyGameServerDefaults;
                let mut value = $(&fixture)();
                let before: serde_json::Value = serde_json::from_str($before).unwrap();
                assert_eq!(serde_json::to_value(&value).unwrap(), before);
                assert_eq!(value.spec.counters.is_some(), $(&counters));
                assert_eq!(value.spec.lists.is_some(), $(&lists));
                assert!(!value.spec.players.is_some());
                assert_eq!(value.spec.ports.is_nil(), $(&nil_ports));
                if $(&apply) { agones_core::GameServerDefaultingContext::default().apply_game_server_defaults(&mut value); }
                let after: serde_json::Value = serde_json::from_str($after).unwrap();
                assert_eq!(serde_json::to_value(&value).unwrap(), after);
            }
        });
        assertions.push(json!({"function":assertion,"origin":original[0]["id"]}));
        harnesses.push(json!({"harness":harness,"case_id":id,"assertion_functions":[assertion],"required_covers":[cover_text,"the generator should produce at least one valid value"],"unwind":128,"raw_domain":"unit","preconditions":[]}));
    }
    ordinary.append(quote! {
        #[cfg(not(kani))]
        #[test]
        fn source_gs_validate_relation_rejects_mutated_error_elements_and_multiplicity() {
            let error = agones_core::ValidationError { kind: agones_core::ValidationErrorKind::Invalid, path:"spec".into(), bad_value:Some("value".into()), detail:"detail".into(), origin:String::new(), covered_by_declarative:false };
            assert!(source_gs_validate_errors_match(&[error.clone()], &[error.clone()]));
            assert!(!source_gs_validate_errors_match(&[error.clone(), error.clone()], &[error.clone()]));
            for field in 0..6 {
                let mut wrong = error.clone();
                match field { 0 => wrong.kind = agones_core::ValidationErrorKind::Forbidden, 1 => wrong.path = "other".into(), 2 => wrong.bad_value = None, 3 => wrong.detail = "other".into(), 4 => wrong.origin = "format=other".into(), _ => wrong.covered_by_declarative = true }
                assert!(!source_gs_validate_errors_match(&[error.clone()], &[wrong]));
            }
        }
    });
    tokens.append(ordinary);
    let source = format!("// Generated from gated typed TestGameServerValidate IR. Do not edit.\n{}", format_rust(tokens.to_string().unwrap()));
    let output = "crates/agones-core/tests/generated/gameserver_validation_cases.rs";
    let generators = ["crates/agones-rusternetes/examples/feature_case_codegen.rs", "crates/agones-rusternetes/examples/codegen/gameserver_features.rs", "crates/agones-rusternetes/examples/codegen/gameserver_validation.rs"];
    let bindings = generators.into_iter().map(|p| (p.to_owned(), json!(hash(&fs::read(root.join(p)).unwrap())))).collect::<serde_json::Map<_,_>>();
    let mut metadata = json!({"schema_version":1,"feature_id":"RESOURCE-QUANTITY-001","generator":"Genco0.17.10with pinned rustfmt","generator_path":generators[0],"generator_sha256":bindings[generators[0]],"generator_source_files":bindings,"ir":ir_path,"ir_sha256":hash(&raw),"output":output,"output_sha256":hash(source.as_bytes()),"case_count":25,"assertion_count":25,"assertions":assertions,"harnesses":harnesses,"lifetime_mapping":{"rule":"GS-DEFAULTS-001-OBSERVATION-LIFETIME-1","boundary":"After source observation and covers, retain harness-owned fixture/context/error values in both engines; actual production bodies remain unchanged.","ordinary_cleanup":"Separate complete fixture/default-action and adapter replay uses normal destruction.","residual":"Formal post-observation cleanup/lifecycle remain required."},"fixture_mapping":"Complete wire fixture, typed quantity construction and explicit source presence. Go capture rejects quantities whose normalized construction state is lost by String round-trip. Unknown source fields fail closed. Nil/empty init-container sequences map to the same empty traversal; no standalone identity claim.","scope":"All25original cases and25qualified ElementsMatch occurrences with exact conditional defaulting. Extra whole-before/after native checks are replay evidence, not extra source assertion credit. No formal/conformance/lifecycle/_2completion claim."});
    metadata["validation_error_mapping"] = gameserver_features::validation_error_mapping(root);
    metadata["multiset_mapping"] = gameserver_features::validation_multiset_mapping(root);
    save_or_check(root, output, &source, check);
    save_or_check(root, "verification/agones/gameserver-validation-codegen.json", &(serde_json::to_string_pretty(&metadata).unwrap()+"\n"), check);
}
