#!/usr/bin/env python3
"""Require exact feature input/query/error assertions and all declared covers."""
import argparse
import hashlib
import json

from kani_results import ROOT, read_results, require_claims_and_covers, require_run_command

parser = argparse.ArgumentParser()
parser.add_argument("family", choices=["input", "query", "error-ascii", "sdk-version", "sdk-source", "health", "gameserver-source", "gameserver-features", "gameserver-validation-source", "gameserver-general-validation-source", "object-metadata-source"])
parser.add_argument("run_id")
parser.add_argument("--output", required=True)
parser.add_argument("--case-index", type=int)
args = parser.parse_args()
rows = read_results(args.run_id)
if args.family == "input":
    harness = "feature_input_common_domain"
    claims = [r for r in rows if r["symbol"].startswith(harness+"::{closure#0}.assertion.")]
    assert len(claims) == 3
    assert all(r["location"].startswith("crates/agones-core/tests/feature_input_domain.rs:") for r in claims)
    required = ["Set override selected", "changed flag selected", "nonempty environment selected",
                "missing environment default", "empty environment default", "explicit empty Set override",
                "explicit empty changed flag", "maximum modeled input length", "raw non-UTF-8 byte selected",
                "the generator should produce at least one valid value"]
    scope = "Production select_feature_gate_input and independent reference/invariant. Each optional raw byte string has length 0..2; no filtering, unwind 8. No OS/command/lifecycle proof or original exact-case assertion credit."
elif args.family == "error-ascii":
    harness = "feature_error_ascii_common_domain"
    claims = [r for r in rows if r["symbol"].startswith(harness+"::{closure#0}.assertion.")]
    assert len(claims) == 3
    assert all(r["location"].startswith("crates/agones-core/tests/feature_error_ascii_domain.rs:") for r in claims)
    required = ["empty ASCII quote input", "maximum modeled ASCII length", "double quote escape",
                "backslash escape", "apostrophe remains literal", "Go-specific named control escape",
                "canonical lowercase hex control escape", "printable ASCII remains literal",
                "the generator should produce at least one valid value"]
    scope = "Production quote_feature_error_bytes and independent ASCII reference/token invariant. ASCII strings length 0..2; no filtering, unwind 16. No non-ASCII, whole-error, lifecycle or original exact-case assertion credit."
elif args.family == "sdk-version":
    harness = "sdk_version_common_domain"
    claims = [r for r in rows if r["symbol"].startswith(harness+"::{closure#0}.assertion.")]
    assert len(claims) == 3
    assert all(r["location"].startswith("crates/agones-core/tests/sdk_version_domain.rs:") for r in claims)
    required = ["empty SDK version", "maximum modeled SDK byte length", "SDK version NUL preserved",
                "valid two-byte SDK scalar", "valid three-byte SDK scalar", "valid four-byte SDK scalar",
                "invalid leading SDK byte", "overlong SDK byte sequence", "surrogate SDK byte sequence",
                "truncated SDK byte prefix", "literal replacement SDK scalar",
                "invalid SDK continuation byte", "SDK scalar above Unicode maximum",
                "the generator should produce at least one valid value"]
    scope = "Actual CGP EncodeAgonesVersionAnnotation provider, independent byte-grammar reference and separate standard UTF-8 observation relation. Raw byte strings length 0..4, no filtering, unwind 16. No unbounded string, Cargo/OS, surrounding lifecycle or original exact-case assertion credit."
elif args.family == "gameserver-features":
    harness = "gameserver_feature_validation_common_domain"
    claims = [r for r in rows if r["symbol"].startswith(harness+"::{closure#0}.assertion.")]
    assert len(claims) == 3
    assert all(r["location"].startswith("crates/agones-core/tests/gameserver_feature_validation_domain.rs:") for r in claims)
    run = json.loads((ROOT / f"verification/agones/runs/{args.run_id}.json").read_text())
    assert run["command"][-5:] == ["bolero", "test", harness, "--engine", "kani"]
    domain = json.loads((ROOT / "verification/agones/gameserver-feature-validation-domain.json").read_text())
    assert domain["harness"] == harness and domain["unwind"] == 128 and domain["preconditions"] == []
    gate = json.loads((ROOT / domain["implementation_gate"]).read_text())
    assert gate["status"] == "IMPLEMENTED" and gate["feature_id"] == "GS-VALIDATE-FEATURES-001"
    proof_paths = set(gate["production_files"]) | {"crates/agones-core/tests/gameserver_feature_validation_domain.rs"}
    for path in proof_paths:
        current = hashlib.sha256((ROOT / path).read_bytes()).hexdigest()
        if path in gate["production_files"]:
            assert current == gate["production_files"][path], f"stale implementation gate: {path}"
        if path.startswith("crates/") or path in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", ".cargo/config.toml"):
            assert current == run["source_files"][path], f"proof used different source: {path}"
    required = ["disabled PlayerTracking absent players", "disabled PlayerTracking present players forbidden",
                "enabled PlayerTracking absent players", "enabled PlayerTracking present players accepted",
                "present zero player capacity remains present", "disabled minimum player capacity remains forbidden",
                "enabled maximum player capacity remains accepted", "the generator should produce at least one valid value"]
    assert domain["required_covers"] == required
    assert domain["lifetime_mapping"]["rule"] == "GS-DEFAULTS-001-OBSERVATION-LIFETIME-1"
    mapping = domain["validation_error_mapping"]
    assert mapping["rule"] == "AGONES-ERROR-CONSTRUCTOR-EMPTY-1"
    assert mapping["fields"] == ["Type", "Field", "BadValue", "Detail", "Origin", "CoveredByDeclarative"]
    assert hashlib.sha256((ROOT / mapping["source"]).read_bytes()).hexdigest() == mapping["source_sha256"]
    scope = "Actual CGP ValidateAgonesGameServerFeatures provider and independent nonallocating match-table reference checked against the separate complete six-field error relation. Arbitrary enabled/present booleans and signed i64 capacity, no filtering, fixed spec field path, unwind128. The deterministic relation fixes count/kind/path/optional bad value/detail/origin/declarative flag for both observations. Source Required/Forbidden empty bad values use the pinned bidirectional constructor-image rule. No qualified original table assertion, unbounded field path, full validation, admission, lifecycle or live-cluster proof credit. Harness owners are retained after observations under the existing lifetime rule; formal post-observation cleanup remains required."
elif args.family == "health":
    harness = "health_defaulting_common_domain"
    claims = [r for r in rows if r["symbol"].startswith(harness+"::{closure#0}.assertion.")]
    assert len(claims) == 3
    assert all(r["location"].startswith("crates/agones-core/tests/health_defaulting_domain.rs:") for r in claims)
    required = ["disabled health keeps nonpositive values", "enabled health sets all three defaults",
                "small positive health values stay unchanged", "zero health threshold receives source default",
                "signed health extremes remain exact", "disabled minimum delay remains exact",
                "mixed health thresholds default independently", "the generator should produce at least one valid value"]
    scope = "Actual GameServerSpec::apply_defaults health observation with other spec fields zero, independent reference and relational invariant. One bool and three arbitrary i32 inputs, no filtering, unwind 16. No whole GameServer/admission/lifecycle proof or original exact-case assertion credit."
elif args.family in ("gameserver-source", "gameserver-validation-source", "gameserver-general-validation-source", "object-metadata-source"):
    object_metadata = args.family == "object-metadata-source"
    general = args.family == "gameserver-general-validation-source"
    validation = args.family != "gameserver-source"
    name, feature, case_count, assertion_count = (
        ("object-metadata", "META-VALIDATE-001", 6, 21) if object_metadata else
        ("gameserver-validation", "RESOURCE-QUANTITY-001", 25, 25) if general else
        ("gameserver-feature-validation", "GS-VALIDATE-FEATURES-001", 5, 10) if validation else
        ("gameserver-defaulting", "GS-DEFAULTS-001", 17, 304)
    )
    metadata = json.loads((ROOT / f"verification/agones/{name}-codegen.json").read_text())
    assert metadata["feature_id"] == feature
    assert metadata["case_count"] == case_count and metadata["assertion_count"] == assertion_count
    assert len(metadata["harnesses"]) == case_count and len(metadata["assertions"]) == assertion_count
    assert len({item["harness"] for item in metadata["harnesses"]}) == case_count
    assert len({item["case_id"] for item in metadata["harnesses"]}) == case_count
    assert len({item["function"] for item in metadata["assertions"]}) == assertion_count
    assert len({item["origin"] for item in metadata["assertions"]}) == assertion_count
    assert args.case_index is not None and 0 <= args.case_index < case_count, "select the exact source case"
    selected = metadata["harnesses"][args.case_index]
    harness = selected["harness"]
    run = json.loads((ROOT / f"verification/agones/runs/{args.run_id}.json").read_text())
    for key, binding in [("output", "output_sha256"), ("ir", "ir_sha256"), ("generator_path", "generator_sha256")]:
        path = metadata[key]
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == metadata[binding], f"stale {key}"
        assert run["source_files"][path] == metadata[binding], f"proof used different {key}"
    command = run["command"]
    assert ["bolero", "test", harness, "--engine", "kani"] == command[-5:], "require the selected real Bolero/Kani route"
    if validation:
        sources = metadata["generator_source_files"]
        expected_sources = {metadata["generator_path"], "crates/agones-rusternetes/examples/codegen/gameserver_features.rs"}
        if object_metadata:
            expected_sources.add("crates/agones-rusternetes/examples/codegen/object_metadata.rs")
        if general:
            expected_sources.add("crates/agones-rusternetes/examples/codegen/gameserver_validation.rs")
        assert set(sources) == expected_sources
        for path, expected in sources.items():
            assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == expected, f"stale generator module: {path}"
            assert run["source_files"][path] == expected, f"proof used different generator module: {path}"
        ir = json.loads((ROOT / metadata["ir"]).read_text())
        source_cases = ir["cases"] if not object_metadata else ir["overflow"]["cases"] + [
            {**ir["metadata_patch"], "assertions": [assertion for step in ir["metadata_patch"]["validation_steps"] for assertion in step["assertions"]]}
        ]
        assert ir["feature_id"] == feature and ir["case_count"] == len(source_cases) == case_count
        if object_metadata:
            assert ir["classification"] == "EXACT_CASE_AND_WHOLE_TEST_ASSERTIONS"
            assert ir["source_assertion_occurrences"] == 21 and ir["validation_observations"] == 11
            assert len(ir["metadata_patch"]["validation_steps"]) == 6
            assert sum(a["kind"] == "clone_patch" for a in ir["metadata_patch"]["source_actions"]) == 2
            assert {a["origin"] for a in metadata["assertions"]} == {a["id"] for case in source_cases for a in case["assertions"]}
            for entry, case in zip(metadata["harnesses"], source_cases):
                functions = {a["function"]: a["origin"] for a in metadata["assertions"]}
                assert entry["case_id"] == case["id"]
                assert [functions[f] for f in entry["assertion_functions"]] == [a["id"] for a in case["assertions"]]
            assert ir["source_files"] and ir["loader_source_files"]
            for field in ("source_files", "loader_source_files"):
                for path, expected in ir[field].items():
                    assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == expected, f"stale original metadata dependency: {path}"
            evidence_path = metadata["source_capture_evidence"]
            evidence_raw = (ROOT / evidence_path).read_bytes()
            assert hashlib.sha256(evidence_raw).hexdigest() == metadata["source_capture_evidence_sha256"]
            evidence = json.loads(evidence_raw)
            assert evidence["ir_sha256"] == metadata["ir_sha256"]
            provenance = evidence["go_runtime_restored"]
            for path, expected in [(provenance["manifest"], provenance["manifest_sha256"]),
                                   (provenance["unicode_reference"], provenance["reference_sha256"])]:
                assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == expected
            assert ir["runtime_version"] == provenance["version"] == "go1.27.1-X:nodwarf5"
        gate_path = ir["implementation_gate"]
        gate_raw = (ROOT / gate_path).read_bytes()
        assert hashlib.sha256(gate_raw).hexdigest() == ir["implementation_gate_sha256"]
        gate = json.loads(gate_raw)
        assert gate["feature_id"] == feature and gate["status"] == "IMPLEMENTED"
        for path, expected in gate["production_files"].items():
            assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == expected, f"stale production: {path}"
            if path.startswith("crates/") or path in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", ".cargo/config.toml"):
                assert run["source_files"][path] == expected, f"proof used different production: {path}"
        assert selected["case_id"] == source_cases[args.case_index]["id"]
        prefix = "source_metadata" if object_metadata else "source_gs_validate" if general else "source_gsv"
        assert harness == f"{prefix}_case_{args.case_index:03}"
        assert len(selected["assertion_functions"]) == (len(source_cases[args.case_index]["assertions"]) if object_metadata else 1 if general else 2)
        mapping = metadata["validation_error_mapping"]
        assert mapping["rule"] == "AGONES-ERROR-CONSTRUCTOR-EMPTY-1"
        assert mapping["fields"] == ["Type", "Field", "BadValue", "Detail", "Origin", "CoveredByDeclarative"]
        assert hashlib.sha256((ROOT / mapping["source"]).read_bytes()).hexdigest() == mapping["source_sha256"]
        if not object_metadata:
            multiset = metadata["multiset_mapping"]
            assert multiset["rule"] == "AGONES-ELEMENTSMATCH-EXPECTED-COUNTS-1"
            assert hashlib.sha256((ROOT / multiset["manifest"]).read_bytes()).hexdigest() == multiset["manifest_sha256"]
    functions = {claim["function"]: claim for claim in metadata["assertions"] if claim["function"] in selected["assertion_functions"]}
    assert len(functions) == len(selected["assertion_functions"])
    if validation:
        assert {claim["origin"] for claim in functions.values()} == {
            claim["id"] for claim in source_cases[args.case_index]["assertions"]
        }, "selected assertion origins differ from source IR"
    names = lambda row: row["symbol"].split(".assertion.")[0].split("::")[-1]
    all_functions = {claim["function"] for claim in metadata["assertions"]}
    assert {names(row) for row in rows if names(row) in all_functions} == set(functions), "unexpected original source assertion selection"
    claims = [row for row in rows if names(row) in functions]
    assert len(claims) == len(functions), "missing/duplicate original source assertions"
    assert {names(row) for row in claims} == set(functions)
    assert all(row["location"] and row["location"].startswith(metadata["output"]+":") for row in claims)
    for row in claims:
        row["origin"] = functions[names(row)]["origin"]
    required = selected["required_covers"]
    if validation:
        assert metadata["lifetime_mapping"]["rule"] == "GS-DEFAULTS-001-OBSERVATION-LIFETIME-1"
        assert selected["raw_domain"] == "unit" and selected["preconditions"] == [] and selected["unwind"] == 128
        assert len(required) == len(set(required)) == 2
        scope = "Selected exact source validation case: "+selected["case_id"]+". "+metadata["scope"]
    else:
        scope = ("Selected exact source GameServer case only: "+selected["case_id"]+
                 ". Unit input, no preconditions, unwind "+str(selected["unwind"])+
                 "; typed whole source fixture, actual feature parser and CGP defaulting provider, literal original relations. "
                 "After all observations/case cover, the harness retains its owners under "+selected["lifetime_rule"]+". "
                 "No formal post-observation heap cleanup is claimed. "
                 "Other cases, zero-occurrence sites, general domains, admission/lifecycle and executor behavior receive no credit.")
elif args.family == "sdk-source":
    metadata = json.loads((ROOT / "verification/agones/sdk-version-codegen.json").read_text())
    harness = metadata["harness"]
    functions = {claim["function"]: claim for claim in metadata["assertions"]}
    claims = [r for r in rows if r["symbol"].split(".assertion.")[0].split("::")[-1] in functions]
    assert len(claims) == len(functions) == 2
    assert {r["symbol"].split(".assertion.")[0].split("::")[-1] for r in claims} == set(functions)
    assert all(r["location"].startswith("crates/agones-core/tests/generated/sdk_version_cases.rs:") for r in claims)
    for row in claims:
        row["origin"] = functions[row["symbol"].split(".assertion.")[0].split("::")[-1]]["origin"]
    required = metadata["required_covers"]
    scope = metadata["scope"]
else:
    metadata = json.loads((ROOT / "verification/agones/feature-query-codegen.json").read_text())
    assert args.case_index is not None and 0 <= args.case_index < 5, "select the exact source case"
    selected = metadata["harnesses"][args.case_index]
    harness = selected["harness"]
    functions = {claim["function"]:claim for claim in metadata["assertions"] if claim["function"] in selected["assertion_functions"]}
    claims = [r for r in rows if r["symbol"].split(".assertion.")[0] in functions]
    assert len(claims) == len(functions) == len(selected["assertion_functions"])
    assert {r["symbol"].split(".assertion.")[0] for r in claims} == set(functions)
    assert all(r["location"].startswith("crates/agones-core/tests/generated/feature_gate_cases.rs:") for r in claims)
    for row in claims:
        row["origin"] = functions[row["symbol"].split(".assertion.")[0]]["origin"]
    required = selected["required_covers"]
    scope = "Selected exact source case only: "+selected["case_id"]+". "+metadata["scope"]
require_run_command(args.run_id, harness)
covers = require_claims_and_covers(rows, claims, required)
summary = {"schema_version":1, "family":args.family, "run":f"verification/agones/runs/{args.run_id}.json",
           "selected_harnesses":[harness], "harness_count":1, "property_count":len(rows),
           "assertions":claims, "covers":covers,
           "status_counts":{status:sum(row["status"]==status for row in rows) for status in ("SUCCESS","SATISFIED","UNREACHABLE")},
           "unreachable_not_credited":[r for r in rows if r["status"]=="UNREACHABLE"], "scope":scope}
if args.family in ("gameserver-source", "gameserver-validation-source", "gameserver-general-validation-source"):
    summary["lifetime_mapping"] = metadata["lifetime_mapping"]
elif args.family == "gameserver-features":
    summary["lifetime_mapping"] = domain["lifetime_mapping"]
destination = ROOT / args.output
destination.parent.mkdir(parents=True, exist_ok=True)
destination.write_text(json.dumps(summary,indent=2,sort_keys=True)+"\n")
print(json.dumps({"harness_count":1,"claimed_assertions":len(claims),"required_covers":len(covers),"property_count":len(rows),"status_counts":summary["status_counts"]}))
