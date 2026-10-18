#!/usr/bin/env python3
"""Require exact feature input/query/error assertions and all declared covers."""
import argparse
import hashlib
import json

from kani_results import ROOT, read_results, require_claims_and_covers

parser = argparse.ArgumentParser()
parser.add_argument("family", choices=["input", "query", "error-ascii", "sdk-version", "sdk-source", "health", "gameserver-source"])
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
elif args.family == "gameserver-source":
    metadata = json.loads((ROOT / "verification/agones/gameserver-defaulting-codegen.json").read_text())
    assert metadata["feature_id"] == "GS-DEFAULTS-001"
    assert metadata["case_count"] == 17 and metadata["assertion_count"] == 304
    assert args.case_index is not None and 0 <= args.case_index < 17, "select the exact source case"
    selected = metadata["harnesses"][args.case_index]
    harness = selected["harness"]
    run = json.loads((ROOT / f"verification/agones/runs/{args.run_id}.json").read_text())
    for key, binding in [("output", "output_sha256"), ("ir", "ir_sha256"), ("generator_path", "generator_sha256")]:
        path = metadata[key]
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == metadata[binding], f"stale {key}"
        assert run["source_files"][path] == metadata[binding], f"proof used different {key}"
    command = run["command"]
    assert ["bolero", "test", harness, "--engine", "kani"] == command[-5:], "require the selected real Bolero/Kani route"
    functions = {claim["function"]: claim for claim in metadata["assertions"] if claim["function"] in selected["assertion_functions"]}
    assert len(functions) == len(selected["assertion_functions"])
    names = lambda row: row["symbol"].split(".assertion.")[0].split("::")[-1]
    claims = [row for row in rows if names(row) in functions]
    assert len(claims) == len(functions), "missing/duplicate original source assertions"
    assert {names(row) for row in claims} == set(functions)
    assert all(row["location"] and row["location"].startswith(metadata["output"]+":") for row in claims)
    for row in claims:
        row["origin"] = functions[names(row)]["origin"]
    required = selected["required_covers"]
    scope = ("Selected exact source GameServer case only: "+selected["case_id"]+
             ". Unit input, no preconditions, unwind "+str(selected["unwind"])+
             "; typed whole source fixture, actual feature parser and CGP defaulting provider, literal original relations. "
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
covers = require_claims_and_covers(rows, claims, required)
summary = {"schema_version":1, "family":args.family, "run":f"verification/agones/runs/{args.run_id}.json",
           "selected_harnesses":[harness], "harness_count":1, "property_count":len(rows),
           "assertions":claims, "covers":covers,
           "status_counts":{status:sum(row["status"]==status for row in rows) for status in ("SUCCESS","SATISFIED","UNREACHABLE")},
           "unreachable_not_credited":[r for r in rows if r["status"]=="UNREACHABLE"], "scope":scope}
destination = ROOT / args.output
destination.parent.mkdir(parents=True, exist_ok=True)
destination.write_text(json.dumps(summary,indent=2,sort_keys=True)+"\n")
print(json.dumps({"harness_count":1,"claimed_assertions":len(claims),"required_covers":len(covers),"property_count":len(rows),"status_counts":summary["status_counts"]}))
