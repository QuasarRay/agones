"""Shared strict parsing for the current one-harness Bolero/Kani receipts."""
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[3]


def parse_complete_log(log):
    assert "Complete - 1 successfully verified harnesses, 0 failures, 1 total." in log
    rows = []
    for block in re.split(r"\n(?=Check \d+:)", log):
        start = re.match(r"Check (\d+): ([^\n]+)", block)
        if not start:
            continue
        status = re.search(r"- Status: ([A-Z]+)", block)
        description = re.search(r'- Description: "(.*?)"(?:\n|$)', block, re.S)
        location = re.search(r"- Location: ([^\n]+)", block)
        assert status and description, f"incomplete check {start[1]}"
        # CBMC may emit global pointer checks without any source-location
        # metadata. Keep them explicitly unlocated; semantic claims/covers
        # still require their exact per-family source locations.
        assert location or re.fullmatch(r"pointer_dereference\.\d+", start[2]), f"incomplete check {start[1]}"
        rows.append(dict(id=int(start[1]), symbol=start[2], status=status[1],
                         description=description[1], location=location[1] if location else None))
    assert rows, "empty property selection"
    assert [row["id"] for row in rows] == list(range(1, len(rows)+1)), "missing/duplicate result IDs"
    assert all(row["status"] in ("SUCCESS", "SATISFIED", "UNREACHABLE") for row in rows), "failing/inconclusive property"
    return rows


def read_results(run_name):
    run = json.loads((ROOT / f"verification/agones/runs/{run_name}.json").read_text())
    assert run["exit_code"] == 0 and run["status"] == "PASSED"
    raw = (ROOT / run["log"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == run["log_sha256"]
    return parse_complete_log(raw.decode())


def require_claims_and_covers(rows, claims, required_covers):
    assert claims and all(row["status"] == "SUCCESS" for row in claims), "unproved/unreachable source claim"
    covers = [row for row in rows if ".cover." in row["symbol"]]
    assert len(covers) == len(required_covers) and {row["description"] for row in covers} == set(required_covers), "missing/extra cover"
    assert all(row["status"] == "SATISFIED" for row in covers), "unsatisfied cover"
    return covers
