use super::*;

fn map_entries(value: &Value, target: &str) -> rust::Tokens {
    let mut tokens = rust::Tokens::new();
    if let Some(entries) = value.as_object() {
        for (key, value) in entries {
            let key = quoted(key);
            let value = quoted(value.as_str().expect("source metadata string"));
            tokens.append(quote! { $target.insert($key.to_owned(), $value.to_owned()); });
        }
    } else {
        assert!(value.is_null(), "unsupported source metadata map image");
    }
    tokens
}

fn reference_errors(value: &Value, name: &str) -> rust::Tokens {
    let mut errors = rust::Tokens::new();
    for error in value.as_array().expect("complete captured error list") {
        let error = gameserver_features::source_validation_error(error);
        errors.append(quote! { $error, });
    }
    quote! { let $name: Vec<agones_core::ValidationError> = vec![$errors]; }
}

fn reference_assertion_holds(assertion: &Value, errors: &[agones_core::ValidationError]) -> bool {
    match assertion["kind"].as_str().unwrap() {
        "length" => errors.len() == assertion["expected"].as_u64().unwrap() as usize,
        "field" => errors
            .get(assertion["index"].as_u64().unwrap() as usize)
            .is_some_and(|error| error.path == assertion["expected"].as_str().unwrap()),
        "type" => {
            assert_eq!(assertion["expected"], "FieldValueInvalid");
            errors
                .get(assertion["index"].as_u64().unwrap() as usize)
                .is_some_and(|error| error.kind == agones_core::ValidationErrorKind::Invalid)
        }
        other => panic!("unmodeled metadata assertion: {other}"),
    }
}

fn assertion_expression(assertion: &Value, observation: &str) -> rust::Tokens {
    match assertion["kind"].as_str().unwrap() {
        "length" => {
            let expected = assertion["expected"].as_u64().unwrap().to_string();
            quote! { $observation.len() == $expected }
        }
        "field" => {
            let index = assertion["index"].as_u64().unwrap().to_string();
            let expected = quoted(assertion["expected"].as_str().unwrap());
            quote! { $observation.get($index).is_some_and(|error| error.path == $expected) }
        }
        "type" => {
            assert_eq!(assertion["expected"], "FieldValueInvalid");
            let index = assertion["index"].as_u64().unwrap().to_string();
            quote! { $observation.get($index).is_some_and(|error| error.kind == agones_core::ValidationErrorKind::Invalid) }
        }
        other => panic!("unmodeled metadata assertion: {other}"),
    }
}

struct SourceAssertions {
    functions: rust::Tokens,
    manifest: Vec<Value>,
    identities: std::collections::BTreeSet<String>,
}

impl SourceAssertions {
    fn append(
        &mut self,
        assertion: &Value,
        reference: &[agones_core::ValidationError],
        actual_name: &str,
        reference_name: &str,
    ) -> rust::Tokens {
        assert!(
            reference_assertion_holds(assertion, reference),
            "Go observation violates independent source assertion: {}",
            assertion["id"]
        );
        let id = assertion["id"].as_str().unwrap();
        assert!(
            self.identities.insert(id.to_owned()),
            "duplicate source assertion"
        );
        let name = format!("source_metadata_assertion_{:03}", self.manifest.len());
        let origin = quoted(id);
        self.functions
            .append(quote! { fn $(&name)(relation: bool) { assert!(relation, $origin); } });
        self.manifest.push(json!({"function":name,"origin":id,"relation":assertion["kind"],"source_line":assertion["line"]}));
        let production = assertion_expression(assertion, actual_name);
        let model = assertion_expression(assertion, reference_name);
        quote! { $(&name)(($model) && ($production)); }
    }
}

fn native_projection(input: &str, actual: &str, reference: &str, step: &Value) -> rust::Tokens {
    let labels = quoted(serde_json::to_string(&step["labels"]).unwrap());
    let annotations = quoted(serde_json::to_string(&step["annotations"]).unwrap());
    quote! {
        #[cfg(not(kani))] {
            source_metadata_native_projection(&$input.labels, &$input.annotations, $labels, $annotations, &$actual, &$reference);
        }
    }
}

fn decoded_errors(step: &Value) -> Vec<agones_core::ValidationError> {
    step["observed_errors"]
        .as_array()
        .unwrap()
        .iter()
        .map(gameserver_features::source_validation_error_record)
        .collect()
}

pub(super) fn emit_object_metadata_cases(root: &Path, check: bool) {
    let ir_path = "verification/agones/object-metadata.ir.json";
    let raw = fs::read(root.join(ir_path)).unwrap();
    let ir: Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(ir["schema_version"], 1);
    assert_eq!(ir["feature_id"], "META-VALIDATE-001");
    assert_eq!(ir["classification"], "EXACT_CASE_AND_WHOLE_TEST_ASSERTIONS");
    assert_eq!(ir["case_count"], 6);
    assert_eq!(ir["source_assertion_occurrences"], 21);
    assert_eq!(ir["validation_observations"], 11);
    validate_gate(
        root,
        ir["implementation_gate"].as_str().unwrap(),
        ir["implementation_gate_sha256"].as_str().unwrap(),
    );
    for field in ["source_files", "loader_source_files"] {
        for (path, expected) in ir[field].as_object().unwrap() {
            assert_eq!(
                hash(&fs::read(root.join(path)).unwrap()),
                expected.as_str().unwrap(),
                "stale metadata dependency: {path}"
            );
        }
    }
    for program in ir["capture_programs"].as_array().unwrap() {
        assert_eq!(
            hash(program["source"].as_str().unwrap().as_bytes()),
            program["sha256"]
        );
    }
    let evidence_path = "verification/agones/object-metadata-source-evidence.json";
    let evidence_raw = fs::read(root.join(evidence_path)).unwrap();
    let evidence: Value = serde_json::from_slice(&evidence_raw).unwrap();
    assert_eq!(evidence["ir_sha256"], hash(&raw));
    for (path, digest) in [
        (
            evidence["go_runtime_restored"]["manifest"]
                .as_str()
                .unwrap(),
            &evidence["go_runtime_restored"]["manifest_sha256"],
        ),
        (
            evidence["go_runtime_restored"]["unicode_reference"]
                .as_str()
                .unwrap(),
            &evidence["go_runtime_restored"]["reference_sha256"],
        ),
    ] {
        assert_eq!(
            hash(&fs::read(root.join(path)).unwrap()),
            digest.as_str().unwrap()
        );
    }

    let mut assertions = SourceAssertions {
        functions: rust::Tokens::new(),
        manifest: Vec::new(),
        identities: Default::default(),
    };
    let mut programs = rust::Tokens::new();
    let mut tests = rust::Tokens::new();
    let mut integration = rust::Tokens::new();
    let mut harnesses = Vec::new();
    let cases = ir["overflow"]["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 5);
    let mut bodies = Vec::new();
    for (case_index, case) in cases.iter().enumerate() {
        assert_eq!(case["source_assertions_passed"], true);
        let labels = map_entries(&case["labels"], "input.labels");
        let annotations = map_entries(&case["annotations"], "input.annotations");
        let reference = reference_errors(&case["observed_errors"], "reference");
        let projection = native_projection("input", "actual", "reference", case);
        let reference_values = decoded_errors(case);
        let mut checks = rust::Tokens::new();
        let mut selected = Vec::new();
        for assertion in case["assertions"].as_array().unwrap() {
            selected.push(format!(
                "source_metadata_assertion_{:03}",
                assertions.manifest.len()
            ));
            checks.append(assertions.append(assertion, &reference_values, "actual", "reference"));
        }
        let input = if case["labels"]
            .as_object()
            .is_some_and(|map| !map.is_empty())
            || case["annotations"]
                .as_object()
                .is_some_and(|map| !map.is_empty())
        {
            quote! { let mut input = agones_core::AllocationOverflow::default(); }
        } else {
            quote! { let input = agones_core::AllocationOverflow::default(); }
        };
        let native_name = format!("source_metadata_rusternetes_overflow_{case_index:03}");
        integration.append(quote! {
            #[test]
            fn $native_name() {
                use agones_rusternetes::RusternetesCustomResource;
                $(&input)
                $(&labels) $(&annotations)
                $(&reference)
                let mut game = agones_core::GameServer::default();
                game.spec.template.spec.containers = vec![agones_core::Container { name:"game".into(), image:"game:v1".into(), ..Default::default() }].into();
                game.apply_defaults();
                let mut fleet = agones_core::Fleet::default();
                fleet.metadata.name = "fleet".into();
                fleet.spec.template.spec = game.spec.clone();
                fleet.spec.allocation_overflow = Some(input.clone());
                fleet.apply_defaults();
                let fleet = agones_core::Fleet::from_rusternetes_custom_resource(fleet.to_rusternetes_custom_resource().unwrap()).unwrap();
                assert_eq!(fleet.spec.allocation_overflow.as_ref(), Some(&input));
                let mut set = agones_core::GameServerSet::default();
                set.metadata.name = "set".into();
                set.spec.template.spec = game.spec;
                set.spec.allocation_overflow = Some(input.clone());
                let set = agones_core::GameServerSet::from_rusternetes_custom_resource(set.to_rusternetes_custom_resource().unwrap()).unwrap();
                assert_eq!(set.spec.allocation_overflow.as_ref(), Some(&input));
                for actual in [agones_rusternetes::validate_fleet(&fleet, &agones_rusternetes::NoopApiHooks).unwrap(),
                               agones_rusternetes::validate_game_server_set_creation(&set, &agones_rusternetes::NoopApiHooks).unwrap()] {
                    $(&checks)
                    assert_eq!(actual, reference);
                }
            }
        });
        let body = quote! {
            let context = SourceMetadataContext;
            $input
            $labels $annotations
            let actual = input.validate_with_context(&context, "spec.allocationOverflow");
            $reference
            $projection
            $checks
        };
        bodies.push((
            case["id"].as_str().unwrap().to_owned(),
            body,
            vec![
                "input".to_owned(),
                "context".to_owned(),
                "actual".to_owned(),
                "reference".to_owned(),
            ],
            selected,
        ));
    }
    let integration_assertions = assertions.functions.clone();

    let patch = &ir["metadata_patch"];
    assert_eq!(patch["source_assertions_passed"], true);
    let steps = patch["validation_steps"].as_array().unwrap();
    let mut body = quote! { let context = SourceMetadataContext; };
    let mut names = std::collections::BTreeMap::new();
    let mut retained = vec!["context".to_owned()];
    let mut copies = 0;
    let mut validations = 0;
    let mut actual_name = String::new();
    let mut reference_name = String::new();
    let mut selected = Vec::new();
    let mut step_assertions = Vec::new();
    let mut path = String::new();
    for action in patch["source_actions"].as_array().unwrap() {
        match action["kind"].as_str().unwrap() {
            "parallel" => {} // Explicit serial isolated-instance scheduling projection.
            "new_patch" => {
                assert_eq!(action["labels_nil"], true);
                assert_eq!(action["annotations_nil"], true);
                assert!(names.insert("mp", "mp".to_owned()).is_none());
                retained.push("mp".to_owned());
                body.append(quote! { let mut mp = agones_core::MetaPatch::default(); });
            }
            "field_path" => {
                let segments = action["segments"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s.as_str().unwrap())
                    .collect::<Vec<_>>();
                assert!(!segments.is_empty() && segments.iter().all(|s| !s.is_empty()));
                path = segments.join(".");
            }
            "set_map" | "insert_map" => {
                let receiver = &names[action["receiver"].as_str().unwrap()];
                let field = action["field"].as_str().unwrap();
                assert!(matches!(field, "labels" | "annotations"));
                let target = format!("{receiver}.{field}");
                if action["kind"] == "set_map" {
                    body.append(quote! { $(&target) = std::collections::BTreeMap::new(); });
                    for pair in action["entries"].as_array().unwrap() {
                        assert_eq!(pair.as_array().unwrap().len(), 2);
                        let key = quoted(pair[0].as_str().unwrap());
                        let value = quoted(pair[1].as_str().unwrap());
                        body.append(
                            quote! { $(&target).insert($key.to_owned(), $value.to_owned()); },
                        );
                    }
                } else {
                    let key = quoted(action["key"].as_str().unwrap());
                    let value = quoted(action["value"].as_str().unwrap());
                    body.append(quote! { $(&target).insert($key.to_owned(), $value.to_owned()); });
                }
            }
            "clone_patch" => {
                assert_eq!(action["target"], "invalid");
                assert_eq!(action["receiver"], "mp");
                assert_eq!(action["declare"], copies == 0);
                let name = format!("invalid_{copies}");
                copies += 1;
                body.append(quote! { let mut $(&name) = mp.clone(); });
                retained.push(name.clone());
                names.insert("invalid", name);
            }
            "validate" => {
                let step = &steps[validations];
                assert_eq!(step["source_line"], action["line"]);
                let receiver = &names[action["receiver"].as_str().unwrap()];
                actual_name = format!("actual_{validations:03}");
                reference_name = format!("reference_{validations:03}");
                let prefix = quoted(&path);
                body.append(quote! { let $(&actual_name) = $receiver.validate_with_context(&context, $prefix); });
                body.append(reference_errors(&step["observed_errors"], &reference_name));
                body.append(native_projection(
                    receiver,
                    &actual_name,
                    &reference_name,
                    step,
                ));
                retained.extend([actual_name.clone(), reference_name.clone()]);
                validations += 1;
            }
            "assertion" => {
                assert!(validations > 0);
                let assertion = &action["assertion"];
                let expected = steps[validations - 1]["assertions"].as_array().unwrap();
                let offset = step_assertions
                    .iter()
                    .filter(|(step, _): &&(usize, String)| *step == validations - 1)
                    .count();
                assert_eq!(
                    assertion, &expected[offset],
                    "source assertion order changed"
                );
                selected.push(format!(
                    "source_metadata_assertion_{:03}",
                    assertions.manifest.len()
                ));
                body.append(assertions.append(
                    assertion,
                    &decoded_errors(&steps[validations - 1]),
                    &actual_name,
                    &reference_name,
                ));
                step_assertions.push((
                    validations - 1,
                    assertion["id"].as_str().unwrap().to_owned(),
                ));
            }
            other => panic!("unsupported whole MetaPatch action: {other}"),
        }
    }
    assert_eq!(copies, 2);
    assert_eq!(validations, 6);
    assert_eq!(step_assertions.len(), 10);
    bodies.push((
        patch["id"].as_str().unwrap().to_owned(),
        body,
        retained,
        selected,
    ));
    assert_eq!(assertions.manifest.len(), 21);
    let captured_ids = evidence["qualified_assertion_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(assertions.identities, captured_ids);

    for (index, (id, body, retained, selected)) in bodies.into_iter().enumerate() {
        let program = format!("source_metadata_program_{index:03}");
        let harness = format!("source_metadata_case_{index:03}");
        let replay = format!("source_metadata_native_replay_{index:03}");
        let cover_text = format!("source metadata validation case: {id}");
        let cover = quoted(&cover_text);
        let mut keep = rust::Tokens::new();
        for name in retained {
            keep.append(quote! { $name, });
        }
        programs.append(quote! {
            fn $(&program)(retain_observations: bool) {
                $body
                #[cfg(kani)] { kani::cover!(true, $cover); }
                if retain_observations { std::mem::forget(($keep)); }
            }
        });
        tests.append(quote! {
            #[test]
            #[cfg_attr(kani, kani::proof)]
            #[cfg_attr(kani, kani::unwind(128))]
            fn $(&harness)() {
                bolero::check!().with_type::<()>().with_iterations(1).for_each(|_| { $(&program)(true); });
            }
            #[cfg(not(kani))]
            #[test]
            fn $replay() { $(&program)(false); }
        });
        harnesses.push(json!({"harness":harness,"case_id":id,"assertion_functions":selected,"required_covers":[cover_text,"the generator should produce at least one valid value"],"unwind":128,"raw_domain":"unit","preconditions":[],"production_symbols":["AllocationOverflow::validate_with_context or MetaPatch::validate_with_context","ValidateAgonesObjectMetadata::validate_object_metadata"]}));
    }
    let functions = assertions.functions;
    let tokens = quote! {
        use cgp::prelude::*;
        use agones_core::ObjectMetadataValidatorComponent;
        struct SourceMetadataContext;
        delegate_and_check_components! {
            SourceMetadataContext { ObjectMetadataValidatorComponent: agones_core::ValidateAgonesObjectMetadata, }
        }
        #[cfg(not(kani))]
        fn source_metadata_native_projection(labels: &std::collections::BTreeMap<String,String>, annotations: &std::collections::BTreeMap<String,String>, expected_labels: &str, expected_annotations: &str, actual: &[agones_core::ValidationError], reference: &[agones_core::ValidationError]) {
            let expected_labels: Option<std::collections::BTreeMap<String,String>> = serde_json::from_str(expected_labels).unwrap();
            let expected_annotations: Option<std::collections::BTreeMap<String,String>> = serde_json::from_str(expected_annotations).unwrap();
            assert_eq!(labels, &expected_labels.unwrap_or_default());
            assert_eq!(annotations, &expected_annotations.unwrap_or_default());
            assert_eq!(actual, reference);
        }
        $functions
        $programs
        $tests
    };
    let source = format!(
        "// Generated from gated complete object-metadata source IR; do not edit.\n{}",
        format_rust(tokens.to_string().unwrap())
    );
    let output = "crates/agones-core/tests/generated/object_metadata_cases.rs";
    let integration_output = "crates/agones-rusternetes/tests/generated/object_metadata_cases.rs";
    let integration_source = format!(
        "// Generated original AllocationOverflow cases through actual Rusternetes conversions and validation; do not edit.\n{}",
        format_rust(
            quote! { $integration_assertions $integration }
                .to_string()
                .unwrap()
        )
    );
    let generator_paths = [
        "crates/agones-rusternetes/examples/feature_case_codegen.rs",
        "crates/agones-rusternetes/examples/codegen/object_metadata.rs",
        "crates/agones-rusternetes/examples/codegen/gameserver_features.rs",
    ];
    let generator_bindings = generator_paths
        .into_iter()
        .map(|p| (p.to_owned(), json!(hash(&fs::read(root.join(p)).unwrap()))))
        .collect::<serde_json::Map<_, _>>();
    let metadata = json!({"schema_version":1,"feature_id":"META-VALIDATE-001","generator":"Genco0.17.10 with pinned rustfmt","generator_path":generator_paths[0],"generator_sha256":generator_bindings[generator_paths[0]],"generator_source_files":generator_bindings,"ir":ir_path,"ir_sha256":hash(&raw),"source_capture_evidence":evidence_path,"source_capture_evidence_sha256":hash(&evidence_raw),"output":output,"output_sha256":hash(source.as_bytes()),"integration_output":integration_output,"integration_output_sha256":hash(integration_source.as_bytes()),"case_count":6,"validation_observations":11,"assertion_count":21,"assertions":assertions.manifest,"harnesses":harnesses,"validation_error_mapping":gameserver_features::validation_error_mapping(root),"lifetime_mapping":{"rule":"GS-DEFAULTS-001-OBSERVATION-LIFETIME-1","ordinary_cleanup":"Separate complete source replays use normal destruction.","residual":"Formal post-observation cleanup/lifecycle remain required."},"mapping":{"kind":"EXACT_CASE","independent_invariants":"Each original length/type/field assertion checks the separately captured Go reference and actual Rust production observation. Six-field differential equality and input-state snapshots are additional ordinary replay checks.","nil_empty":"Original nil/empty map construction actions and flags remain in IR and source action order. Both map images refine the same empty validation traversal; no standalone nil/empty identity or broader API claim.","deep_copy":"Both original DeepCopy calls clone the current mp into independent owned map states. Earlier copy and validation observations are retained in SSA bindings until final source observations and covers.","lifetime_rule":"GS-DEFAULTS-001-OBSERVATION-LIFETIME-1; retain all fixture, copy and observation values after source assertions/covers in both Bolero/Kani. Separate ordinary replays use normal destruction. Post-observation cleanup/lifecycle remains required.","scheduling":"t.Parallel is a serial isolated-instance replay; overflow fixture declaration order is one permitted map iteration order. No executor/concurrency proof."},"formal_credit":0,"scope":"All5AllocationOverflow cases and whole6-call MetaPatch test, preserving all21qualified assertions. All5overflow cases also traverse actual Rusternetes Fleet/GameServerSet conversions and typed validation. A Rusternetes allocation endpoint is not implemented by this change. Exact cases do not establish an unbounded key/value law or complete API/lifecycle/live Rusternetes/_2 migration."});
    save_or_check(root, output, &source, check);
    save_or_check(root, integration_output, &integration_source, check);
    save_or_check(
        root,
        "verification/agones/object-metadata-codegen.json",
        &(serde_json::to_string_pretty(&metadata).unwrap() + "\n"),
        check,
    );
}
