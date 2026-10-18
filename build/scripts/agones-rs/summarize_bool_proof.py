#!/usr/bin/env python3
"""Fail closed on incomplete Kani output for the declared feature-bool harness."""
import hashlib
import json
from pathlib import Path
import re
import sys
import argparse

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "verification/agones"
parser = argparse.ArgumentParser()
parser.add_argument("run_id", nargs="?", default="feature-bool-bolero-kani-final")
parser.add_argument("--output", default="verification/agones/feature-bool-kani-properties.json")
args = parser.parse_args()
run_name = args.run_id
run = json.loads((OUT / f"runs/{run_name}.json").read_text())
assert run["exit_code"] == 0 and run["status"] == "PASSED"
raw = (ROOT / run["log"]).read_bytes()
assert hashlib.sha256(raw).hexdigest() == run["log_sha256"]
log = raw.decode()
assert "Complete - 1 successfully verified harnesses, 0 failures, 1 total." in log
rows = []
for block in re.split(r"\n(?=Check \d+:)", log):
    start = re.match(r"Check (\d+): ([^\n]+)", block)
    if not start:
        continue
    status = re.search(r"- Status: ([A-Z]+)", block)
    description = re.search(r'- Description: "(.*?)"\n\s+- Location:', block, re.S)
    location = re.search(r"- Location: ([^\n]+)", block)
    assert status and description and location, f"incomplete check {start[1]}"
    rows.append(dict(id=int(start[1]), symbol=start[2], status=status[1],
                     description=description[1], location=location[1]))
assert [row["id"] for row in rows] == list(range(1, len(rows)+1)), "missing/duplicate result IDs"
assert all(row["status"] in ("SUCCESS", "SATISFIED", "UNREACHABLE") for row in rows)
claims = [row for row in rows if row["symbol"].startswith("feature_bool_common_domain::{closure#0}.assertion.")]
assert len(claims) == 3 and all(row["status"] == "SUCCESS" for row in claims)
assert all(row["location"].startswith("crates/agones-core/tests/feature_bool_domain.rs:") for row in claims)
covers = [row for row in rows if ".cover." in row["symbol"]]
required = {"accepted true spelling", "accepted false spelling", "rejected spelling", "empty input", "maximum modeled length", "the generator should produce at least one valid value"}
assert len(covers) == 6 and {row["description"] for row in covers} == required
assert all(row["status"] == "SATISFIED" for row in covers)
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
