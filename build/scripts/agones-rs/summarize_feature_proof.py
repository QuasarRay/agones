#!/usr/bin/env python3
"""Require exact feature input/query assertions and all declared covers."""
import argparse
import json

from kani_results import ROOT, read_results, require_claims_and_covers

parser = argparse.ArgumentParser()
parser.add_argument("family", choices=["input", "query"])
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
