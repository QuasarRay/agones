use super::*;

pub(super) fn source_validation_error_record(error: &Value) -> agones_core::ValidationError {
    use agones_core::ValidationErrorKind::*;
    assert_eq!(
        error.as_object().unwrap().len(),
        6,
        "unmodeled source error field"
    );
    let kind = match error["Type"].as_str().unwrap() {
        "FieldValueInvalid" => Invalid,
        "FieldValueForbidden" => Forbidden,
        "FieldValueRequired" => Required,
        "FieldValueDuplicate" => Duplicate,
        "FieldValueNotSupported" => NotSupported,
        "FieldValueTooLong" => TooLong,
        other => panic!("unmodeled source error type: {other}"),
    };
    // AGONES-ERROR-CONSTRUCTOR-EMPTY-1: the pinned Required/Forbidden
    // constructors store the empty Go string. Kind+None encodes that image
    // losslessly. A nil/nonempty value is outside the image and must reject.
    let bad_value = if matches!(kind, Required | Forbidden) {
        assert_eq!(error["BadValue"], "", "outside the source constructor image");
        None
    } else if error["BadValue"].is_null() {
        None
    } else {
        Some(error["BadValue"].as_str().expect("unmodeled non-string bad value").to_owned())
    };
    agones_core::ValidationError {
        kind, path: error["Field"].as_str().unwrap().to_owned(), bad_value,
        detail: error["Detail"].as_str().unwrap().to_owned(),
        origin: error["Origin"].as_str().expect("non-string origin").to_owned(),
        covered_by_declarative: error["CoveredByDeclarative"].as_bool().expect("non-boolean coverage"),
    }
}

pub(super) fn reconstruct_source_validation_error(error: &agones_core::ValidationError) -> Value {
    use agones_core::ValidationErrorKind::*;
    let kind = match error.kind { Invalid => "FieldValueInvalid", Forbidden => "FieldValueForbidden",
        Required => "FieldValueRequired", Duplicate => "FieldValueDuplicate",
        NotSupported => "FieldValueNotSupported", TooLong => "FieldValueTooLong" };
    let bad_value = if matches!(error.kind, Required | Forbidden) {
        assert!(error.bad_value.is_none(), "outside the normalized constructor image");
        json!("")
    } else { json!(error.bad_value) };
    json!({"Type":kind,"Field":error.path,"BadValue":bad_value,"Detail":error.detail,
        "Origin":error.origin,"CoveredByDeclarative":error.covered_by_declarative})
}

pub(super) fn source_validation_error(error: &Value) -> rust::Tokens {
    let record = source_validation_error_record(error);
    assert_eq!(reconstruct_source_validation_error(&record), *error, "lossy source error mapping");
    let kind = format!("{:?}", record.kind);
    let path = quoted(&record.path);
    let detail = quoted(&record.detail);
    let origin = quoted(&record.origin);
    let covered = record.covered_by_declarative.to_string();
    let bad = match record.bad_value { None => quote! { None }, Some(value) => {
        let value = quoted(value); quote! { Some($value.to_owned()) }
    }};
    quote! { agones_core::ValidationError { kind: agones_core::ValidationErrorKind::$kind,
    path: $path.to_owned(), bad_value: $bad, detail: $detail.to_owned(),
    origin: $origin.to_owned(), covered_by_declarative: $covered } }
}

pub(super) fn validation_error_relation(name: &str) -> rust::Tokens {
    quote! {
        fn $name(actual: &[agones_core::ValidationError], expected: &[agones_core::ValidationError]) -> bool {
            match expected {
                [] => actual.is_empty(),
                [error] => matches!(actual, [value] if value == error),
                _ => actual.len() == expected.len() && expected.iter().all(|error| {
                    actual.iter().filter(|value| *value == error).count()
                        == expected.iter().filter(|value| *value == error).count()
                }),
            }
        }
    }
}

pub(super) fn validation_multiset_mapping(root: &Path) -> Value {
    let path = "verification/agones/expected-multiset-relation.json";
    let raw = fs::read(root.join(path)).unwrap();
    let relation: Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(relation["schema_version"], 1);
    assert_eq!(relation["rule_id"], "AGONES-ELEMENTSMATCH-EXPECTED-COUNTS-1");
    json!({
        "rule": "AGONES-ELEMENTSMATCH-EXPECTED-COUNTS-1",
        "manifest": path,
        "manifest_sha256": hash(&raw),
        "relation": "Equal lengths and exact full-record multiplicities for every expected error; no field or duplicate omission."
    })
}

pub(super) fn validation_error_mapping(root: &Path) -> Value {
    let source = "vendor/k8s.io/apimachinery/pkg/util/validation/field/errors.go";
    let digest = hash(&fs::read(root.join(source)).unwrap());
    assert_eq!(digest, "747db5374cdd58712b46d10775272508212d3201fa05c30105a5531debc666d4", "changed pinned source error constructors");
    json!({"rule":"AGONES-ERROR-CONSTRUCTOR-EMPTY-1","source":source,"source_sha256":digest,
        "fields":["Type","Field","BadValue","Detail","Origin","CoveredByDeclarative"],
        "forward":"Require the exact empty Go string for Required/Forbidden and encode it as kind+None. Other supported kinds preserve string versus null distinctly. Copy every other field exactly.",
        "inverse":"Reconstruct the fixed empty source string from Required/Forbidden+None; reject a non-None value. Other kinds reconstruct their original string/null. Every lowered error must round-trip in full.",
        "unsupported":"Non-string/non-null bad values, unknown kinds/fields and values outside the declared constructor image reject; their obligations remain open.",
        "native_fault_controls":"All six error fields and multiset multiplicity are independently observable. Bidirectional constructor-image controls exercise both directions and reject nonempty/nil source values."})
}

pub(super) fn emit_gameserver_feature_cases(root: &Path, check: bool) {
    let ir_path = "verification/agones/gameserver-feature-validation.ir.json";
    let raw = fs::read(root.join(ir_path)).unwrap();
    let ir: Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(ir["feature_id"], "GS-VALIDATE-FEATURES-001");
    assert_eq!(ir["classification"], "EXACT_CASE_ASSERTIONS");
    assert_eq!(ir["case_count"], 5);
    assert_eq!(ir["source_assertion_occurrences"], 10);
    validate_gate(
        root,
        ir["implementation_gate"].as_str().unwrap(),
        ir["implementation_gate_sha256"].as_str().unwrap(),
    );
    for (path, expected) in ir["source_files"].as_object().unwrap() {
        assert_eq!(
            hash(&fs::read(root.join(path)).unwrap()),
            expected.as_str().unwrap(),
            "stale original source: {path}"
        );
    }
    assert_eq!(
        hash(ir["capture_program"].as_str().unwrap().as_bytes()),
        ir["capture_program_sha256"]
    );
    assert_eq!(
        ir["source_actions"],
        json!([
            "err := runtime.ParseFeatures(tc.feature)",
            "assert.NoError(t, err)",
            "errs := tc.gs.Validate(fakeAPIHooks{})",
            "assert.ElementsMatch(t, tc.want, errs, \"ErrorList check\")"
        ])
    );
    let mut tokens = validation_error_relation("source_gsv_errors_match");
    let mut ordinary: rust::Tokens = quote! {
        #[cfg(not(kani))]
        #[test]
        fn source_gsv_multiset_relation_preserves_elements_and_multiplicity() {
            let first = agones_core::ValidationError {
                kind: agones_core::ValidationErrorKind::Invalid,
                path: "spec.first".into(), bad_value: None, detail: "detail".into(),
                origin: String::new(), covered_by_declarative: false,
            };
            let mut second = first.clone(); second.path = "spec.second".into();
            assert!(source_gsv_errors_match(&[first.clone(), second.clone()], &[second.clone(), first.clone()]));
            assert!(!source_gsv_errors_match(&[first.clone(), first.clone()], &[first.clone(), second.clone()]));
            assert!(!source_gsv_errors_match(&[], &[first.clone()]));
            for field in 0..6 {
                let mut wrong = first.clone();
                match field {
                    0 => wrong.kind = agones_core::ValidationErrorKind::Forbidden,
                    1 => wrong.path = "other".into(),
                    2 => wrong.bad_value = Some("bad".into()),
                    3 => wrong.detail = "other".into(),
                    4 => wrong.origin = "format=other".into(),
                    _ => wrong.covered_by_declarative = true,
                }
                assert!(!source_gsv_errors_match(&[first.clone()], &[wrong]));
            }
            let mut third = first.clone(); third.detail = "third class".into();
            let records = [first, second, third];
            let mut lists = vec![Vec::<usize>::new()];
            for length in 1..=3 {
                for encoded in 0..3usize.pow(length) {
                    let mut value = encoded;
                    let ids = (0..length).map(|_| {
                        let id = value % 3;
                        value /= 3;
                        id
                    }).collect::<Vec<_>>();
                    lists.push(ids);
                }
            }
            assert_eq!(lists.len(), 40);
            for actual_ids in &lists {
                for expected_ids in &lists {
                    let mut actual_counts = [0usize; 3];
                    let mut expected_counts = [0usize; 3];
                    for id in actual_ids { actual_counts[*id] += 1; }
                    for id in expected_ids { expected_counts[*id] += 1; }
                    let actual = actual_ids.iter().map(|id| records[*id].clone()).collect::<Vec<_>>();
                    let expected = expected_ids.iter().map(|id| records[*id].clone()).collect::<Vec<_>>();
                    assert_eq!(source_gsv_errors_match(&actual, &expected), actual_counts == expected_counts);
                }
            }
        }
    };
    let mut assertions = Vec::new();
    let mut harnesses = Vec::new();
    let cases = ir["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 5);
    let mut identities = std::collections::BTreeSet::new();
    for (index, case) in cases.iter().enumerate() {
        assert_eq!(case["source_assertions_passed"], true);
        assert_eq!(
            case["expected_errors"], case["observed_errors"],
            "changed literal source expectation"
        );
        let case_id = case["id"].as_str().unwrap();
        assert!(identities.insert(case_id));
        let base = format!("source_gsv_base_fixture_{index:03}");
        let fixture = format!("source_gsv_fixture_{index:03}");
        let harness = format!("source_gsv_case_{index:03}");
        let presence = &case["fixture_presence"];
        assert_eq!(presence.as_object().unwrap().len(), 4);
        let counters = presence["counters_present"].as_bool().unwrap();
        let lists = presence["lists_present"].as_bool().unwrap();
        let players = presence["players_present"].as_bool().unwrap();
        let nil_ports = presence["ports_nil"].as_bool().unwrap();
        let typed: agones_core::GameServer =
            serde_json::from_value(case["before"].clone()).unwrap();
        assert!(
            !players && typed.spec.players.is_none(),
            "present Players is a separate required source-contract proof"
        );
        assert!(
            counters || typed.spec.counters.is_none(),
            "lost source Counters presence"
        );
        assert!(
            lists || typed.spec.lists.is_none(),
            "lost source Lists presence"
        );
        assert_eq!(
            typed.spec.ports.is_nil(),
            nil_ports,
            "unmodeled empty allocated port slice"
        );
        tokens.append(gs_fixture(&case["before"], &base));
        let mut restore = rust::Tokens::new();
        if counters && typed.spec.counters.is_none() {
            restore
                .append(quote! { value.spec.counters = Some(std::collections::BTreeMap::new()); });
        }
        if lists && typed.spec.lists.is_none() {
            restore.append(quote! { value.spec.lists = Some(std::collections::BTreeMap::new()); });
        }
        tokens.append(quote! { fn $(&fixture)() -> agones_core::GameServer { let mut value = $(&base)(); $restore value } });
        let original = case["assertions"].as_array().unwrap();
        assert_eq!(original.len(), 2);
        assert_eq!(original[0]["kind"], "NoError");
        assert_eq!(original[1]["kind"], "ElementsMatch");
        let parse_assertion = format!("source_gsv_assertion_{:03}", index * 2);
        let error_assertion = format!("source_gsv_assertion_{:03}", index * 2 + 1);
        let parse_id = quoted(original[0]["id"].as_str().unwrap());
        let errors_id = quoted(original[1]["id"].as_str().unwrap());
        let flags = quoted(case["feature_flags"].as_str().unwrap());
        let mut expected = rust::Tokens::new();
        if let Some(errors) = case["expected_errors"].as_array() {
            for error in errors {
                let error = source_validation_error(error);
                expected.append(quote! { $error, });
            }
        } else {
            assert!(case["expected_errors"].is_null());
        }
        let cover_text = format!("source GameServer validation case: {case_id}");
        let cover = quoted(cover_text.as_str());
        tokens.append(quote! {
            fn $(&parse_assertion)(relation: bool) { assert!(relation, $parse_id); }
            fn $(&error_assertion)(relation: bool) { assert!(relation, $errors_id); }
            #[test]
            #[cfg_attr(kani, kani::proof)]
            #[cfg_attr(kani, kani::unwind(128))]
            fn $(&harness)() {
                use agones_core::CanParseFeatureGates;
                bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| {
                    let gs = $(&fixture)();
                    let parsed = agones_core::FeatureGateContext::default().parse_feature_gates($flags);
                    $(&parse_assertion)(parsed.is_ok());
                    let gates = parsed.unwrap();
                    let context = agones_core::GameServerDefaultingContext::from_feature_gates(&gates);
                    let actual = gs.validate_with_context(&context);
                    let expected = vec![$expected];
                    $(&error_assertion)(source_gsv_errors_match(&actual, &expected));
                    #[cfg(kani)] { kani::cover!(true, $cover); }
                    std::mem::forget((gs, gates, context, expected, actual));
                });
            }
        });
        let wire = quoted(serde_json::to_string(&case["before"]).unwrap());
        let replay = format!("source_gsv_whole_fixture_{index:03}");
        let [counters, lists, players, nil_ports] =
            [counters, lists, players, nil_ports].map(|value| value.to_string());
        ordinary.append(quote! {
            #[cfg(not(kani))]
            #[test]
            fn $replay() {
                let value = $(&fixture)();
                let raw: serde_json::Value = serde_json::from_str($wire).unwrap();
                assert_eq!(serde_json::to_value(&value).unwrap(), raw);
                assert_eq!(value.spec.counters.is_some(), $counters);
                assert_eq!(value.spec.lists.is_some(), $lists);
                assert_eq!(value.spec.players.is_some(), $players);
                assert_eq!(value.spec.ports.is_nil(), $nil_ports);
            }
        });
        for (function, origin) in [
            (&parse_assertion, &original[0]["id"]),
            (&error_assertion, &original[1]["id"]),
        ] {
            assertions.push(json!({"function":function,"origin":origin}));
        }
        harnesses.push(json!({"harness":harness,"case_id":case_id,"assertion_functions":[parse_assertion,error_assertion],"required_covers":[cover_text,"the generator should produce at least one valid value"],"unwind":128,"raw_domain":"unit","preconditions":[]}));
    }
    tokens.append(ordinary);
    let source = format!(
        "// Generated from gated typed validation IR by feature_case_codegen --gameserver-features. Do not edit.\n{}",
        format_rust(tokens.to_string().unwrap())
    );
    let output = "crates/agones-core/tests/generated/gameserver_feature_validation_cases.rs";
    let generator = "crates/agones-rusternetes/examples/feature_case_codegen.rs";
    let mut metadata = json!({"schema_version":1,"feature_id":"GS-VALIDATE-FEATURES-001","generator":"Genco 0.17.10 with pinned rustfmt","generator_path":generator,"generator_sha256":hash(&fs::read(root.join(generator)).unwrap()),"generator_source_files":{generator:hash(&fs::read(root.join(generator)).unwrap()),"crates/agones-rusternetes/examples/codegen/gameserver_features.rs":hash(&fs::read(root.join("crates/agones-rusternetes/examples/codegen/gameserver_features.rs")).unwrap())},"ir":ir_path,"ir_sha256":hash(&raw),"output":output,"output_sha256":hash(source.as_bytes()),"case_count":5,"assertion_count":10,"assertions":assertions,"harnesses":harnesses,"fixture_mapping":"Whole source wire object plus explicit nil/allocated Counters/Lists/Ports/Players witnesses. Source empty maps omitted by JSON are restored as allocated empty maps. Unknown fields/error metadata fail closed.","lifetime_mapping":{"rule":"GS-DEFAULTS-001-OBSERVATION-LIFETIME-1","boundary":"After every original observation and cover, retain the harness-owned fixture, parsed features, context and error lists in both engines. Source callbacks specify no GC/deallocation observation. Actual production bodies remain unchanged.","ordinary_cleanup":"Separate complete-fixture and actual adapter replay uses normal destruction.","residual":"Formal post-observation cleanup and lifecycle remain required."},"scope":"All five exact original validation cases, ten qualified source assertions. Observation lifetime follows the existing retained-owner rule; no formal post-observation cleanup credit. No original case sets Players. Actual core validation and source literals are modeled; original Go/resource/hooks use ordinary replay. Full domain validation, present-Players contract proof, lifecycle/conformance and all eight _2 cells remain required."});
    metadata["validation_error_mapping"] = validation_error_mapping(root);
    metadata["multiset_mapping"] = validation_multiset_mapping(root);
    save_or_check(root, output, &source, check);
    save_or_check(
        root,
        "verification/agones/gameserver-feature-validation-codegen.json",
        &(serde_json::to_string_pretty(&metadata).unwrap() + "\n"),
        check,
    );
}
