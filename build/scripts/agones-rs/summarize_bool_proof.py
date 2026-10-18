#!/usr/bin/env python3
"""Fail closed on incomplete Kani output for the declared feature-bool harness."""
import json
import argparse

from kani_results import ROOT, read_results, require_claims_and_covers
parser = argparse.ArgumentParser()
parser.add_argument("run_id", nargs="?", default="feature-bool-bolero-kani-input-final")
parser.add_argument("--output", default="verification/agones/feature-bool-kani-properties.json")
args = parser.parse_args()
run_name = args.run_id
rows = read_results(run_name)
claims = [row for row in rows if row["symbol"].startswith("feature_bool_common_domain::{closure#0}.assertion.")]
assert len(claims) == 3 and all(row["status"] == "SUCCESS" for row in claims)
assert all(row["location"].startswith("crates/agones-core/tests/feature_bool_domain.rs:") for row in claims)
required = {"accepted true spelling", "accepted false spelling", "rejected spelling", "empty input", "maximum modeled length", "the generator should produce at least one valid value"}
covers = require_claims_and_covers(rows, claims, required)
unreachable = [row for row in rows if row["status"] == "UNREACHABLE"]
summary = {"schema_version": 1, "run": f"verification/agones/runs/{run_name}.json",
           "selected_harnesses": ["feature_bool_common_domain"], "harness_count": 1,
           "property_count": len(rows), "assertions": claims, "covers": covers,
           "unreachable_not_credited": unreachable,
           "status_counts": {status: sum(row["status"] == status for row in rows) for status in ("SUCCESS", "SATISFIED", "UNREACHABLE")},
           "scope": "Only the actual production parse_feature_bool and independent reference/invariant over byte strings of length 0..5, unwind 8; no unbounded query, environment or controller proof."}
destination = ROOT / args.output
destination.parent.mkdir(parents=True, exist_ok=True)
destination.write_text(json.dumps(summary, indent=2, sort_keys=True)+"\n")
print(json.dumps({key: summary[key] for key in ("harness_count", "property_count", "status_counts")}))
