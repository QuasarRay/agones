#!/usr/bin/env python3
"""Validate and atomically checkpoint the one migration ledger and handoff.

Uses bundled Pydantic with a generated JSON schema. No Goal controls.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess

from state_schema import State

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "verification/agones"
STATE = ROOT / "AGONES_RS_STATE.json"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text())


def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_suffix(".tmp")
    temp.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")
    temp.replace(path)


def check(state):
    State.model_validate(state)
    assert read(ROOT / "AGONES_RS_STATE.schema.json") == State.model_json_schema(), "schema/model drift"
    assert state["contract_sha256"] == digest(ROOT / "AGONES_RS_CONTINUATION_PROMPT.md"), "contract changed"
    baseline = read(OUT / "baseline.json")
    assert baseline["commit"] == state["baseline_id"]
    assert baseline["tree_inventory_sha256"] == digest(OUT / "baseline-tree.tsv")
    assert baseline["test_inventory_sha256"] == digest(OUT / "baseline-tests.json")
    for item in state["work_items"]:
        if item["stage"] in ("DISCOVERED", "IMPLEMENTING"):
            continue
        assert item["implementation_gate"], f"{item['id']}: missing implementation gate"
        gate = read(ROOT / item["implementation_gate"])
        assert gate["status"] == "IMPLEMENTED"
        assert gate["contract_sha256"] == state["contract_sha256"], "stale gate contract"
        for path, expected in gate["production_files"].items():
            assert digest(ROOT / path) == expected, f"stale gate: {path}"
        for ref in gate["ordinary_checks"]:
            run = read(ROOT / ref)
            assert run["status"] == "PASSED" and run["exit_code"] == 0, f"failed check: {ref}"
            assert digest(ROOT / run["log"]) == run["log_sha256"], f"missing/corrupt log: {ref}"
        assert gate["ordinary_checks"], "empty check selection"
        for ref in item["evidence_refs"]:
            evidence = read(ROOT / ref)
            for field in ("semantic_bindings", "artifact_bindings"):
                for path, expected in evidence.get(field, {}).items():
                    assert digest(ROOT / path) == expected, f"stale/missing evidence: {path}"
    print(f"Validated ledger revision {state['revision']}; migration {state['migration_status']}.")


def handoff(state):
    active = state["work_items"][0]
    open_cells = sum(c["status"] != "VERIFIED" for c in state["architecture_matrix"])
    text = f"""# Agones-rs handoff

Generated from AGONES_RS_STATE.json revision {state['revision']}.
Contract SHA-256: `{state['contract_sha256']}`.
Original reference: `{state['baseline_id']}`, scope `{state['scope_id']}`.
Workspace: `{state['workspace']['branch']}` at `{state['workspace']['head']}`.
Native Goal remains active; full migration is **{state['migration_status']}**.

Active work: **{active['id']}**, stage `{active['stage']}`, disposition `{active['disposition']}`.
Next action: {active['next_action']}
Implementation gate: `{active['implementation_gate']}`.

Read the active source and affected contract appendices only. Baseline metadata is in
`verification/agones/baseline.json`, `scope-v1.json`, `baseline-tree.tsv`, and `baseline-tests.json`.
The census has 490 first-party Go top-level test-like declarations; dynamic cases,
assertions and non-Go test identities remain unresolved. This is no completeness claim.
Preserved input: `AGONES_RS_PROMPT_AUDIT_RESOLUTION.md` is an earlier prompt audit,
not product implementation evidence; its old revised-prompt hash is historical.

All {open_cells} unverified `_2` form/location cells remain required architecture work.
Last refactoring decision: {state['refactoring_reviews'][-1]['decision']}.
Next review: {state['refactoring_reviews'][-1]['next_review_trigger']}.
No new macros or general frameworks are prerequisites for the next ordinary feature.

Delivery: `{state['delivery']['remote_result']}` to `agones-rs-dev`; never modify main.
Connected GitHub app has write access; local gh CLI is logged out.
Recover live jobs from the ledger and `verification/agones/runs/*.json` by comparing
the stored process identity with /proc before waiting, attaching or stopping anything.
Initial Cargo network failures are recorded; resolution subsequently succeeded.
Use Cargo.lock, Rust 1.98.1 and locked checks. Never count missing proof tools as success.

Checks and source bindings are in immutable run receipts; orthogonal assurance
counts belong in the feature's evidence manifest. A passed implementation gate
does not mean source extraction, Bolero/Kani proof, integration or delivery passed.
"""
    (ROOT / "AGONES_RS_HANDOFF.md").write_text(text)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=["check", "checkpoint"])
    parser.add_argument("--work-item", help="Select an existing work item as the active packet")
    parser.add_argument("--stage")
    parser.add_argument("--gate")
    parser.add_argument("--next-action")
    parser.add_argument("--refactoring-review")
    parser.add_argument("--evidence-ref")
    parser.add_argument("--add-work-item")
    parser.add_argument("--goal-receipt")
    parser.add_argument("--resolve-blocker", action="append", default=[])
    parser.add_argument("--delivery-plan")
    parser.add_argument("--artifact-policy")
    parser.add_argument("--event", default="CHECKPOINT")
    parser.add_argument("--reason", default="Retain current source, checks and next action")
    args = parser.parse_args()
    state = read(STATE)
    if args.action == "check":
        check(state)
    else:
        old_hash = digest(STATE)
        write(OUT / "states" / f"{old_hash}.json", state)
        if args.work_item:
            selected = [i for i in state["work_items"] if i["id"] == args.work_item]
            assert len(selected) == 1, "unknown or duplicate work item"
            state["work_items"] = selected + [i for i in state["work_items"] if i["id"] != args.work_item]
        item = state["work_items"][0]
        if args.stage and args.stage != item["stage"]:
            transitions = read(ROOT / "AGONES_RS_STATE.schema.json")["x-legal-stage-transitions"]
            assert args.stage in transitions[item["stage"]], "illegal stage transition"
            item["stage"] = args.stage
        if args.gate:
            item["implementation_gate"] = args.gate
        if args.next_action:
            item["next_action"] = args.next_action
        if args.refactoring_review:
            state["refactoring_reviews"].append(read(ROOT / args.refactoring_review))
            for cell in state["architecture_matrix"]:
                cell["next_review_trigger"] = state["refactoring_reviews"][-1]["next_review_trigger"]
        if args.evidence_ref and args.evidence_ref not in item["evidence_refs"]:
            item["evidence_refs"].append(args.evidence_ref)
        if args.add_work_item:
            additional = read(ROOT / args.add_work_item)
            assert additional["id"] not in {i["id"] for i in state["work_items"]}
            state["work_items"].append(additional)
        if args.goal_receipt:
            actual = read(ROOT / args.goal_receipt)["goal"]
            assert actual["status"] == "active", "host Goal does not authorize continuation"
            state["goal"].update(status=actual["status"], reported_usage={"tokens":actual["tokensUsed"], "time_seconds":actual["timeUsedSeconds"], "observed_at":actual["updatedAt"]})
        for blocker in state["blockers"]:
            if blocker["id"] in args.resolve_blocker:
                blocker["status"] = "RESOLVED"
                blocker["resolution"] = args.reason
        if args.delivery_plan:
            state["delivery"].update(read(ROOT / args.delivery_plan))
        if args.artifact_policy:
            state["artifact_policy"] = read(ROOT / args.artifact_policy)
        state.update(revision=state["revision"]+1, previous_revision=state["revision"], previous_sha256=old_hash)
        state["workspace"]["head"] = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
        state["workspace"]["dirty_patch_sha256"] = hashlib.sha256(subprocess.check_output(["git", "diff", "--binary", "HEAD"], cwd=ROOT)).hexdigest()
        state["jobs"] = []
        for path in sorted((OUT/"runs").glob("*.json")):
            run = read(path)
            state["jobs"].append({"receipt":str(path.relative_to(ROOT)), **{
                key:run.get(key) for key in ("owner_token", "command", "cwd", "pid", "process_identity", "started_at", "finished_at", "status", "exit_code", "log", "attempt")},
                "source_binding_sha256":hashlib.sha256(json.dumps(run["source_files"],sort_keys=True).encode()).hexdigest(),
                "resource_usage":"unknown; run timing and exit state are retained"})
        latest = read(sorted((OUT/"runs").glob("*.json"), key=lambda p:p.stat().st_mtime)[-1])
        state["workspace"]["last_validated_implementation"] = item["implementation_gate"]
        state["workspace"]["untracked_source_binding_sha256"] = hashlib.sha256(json.dumps(latest["source_files"],sort_keys=True).encode()).hexdigest()
        state["workspace"]["source_binding_receipt"] = str(max((OUT/"runs").glob("*.json"),key=lambda p:p.stat().st_mtime).relative_to(ROOT))
        check(state)
        write(STATE, state)
        receipt = {"schema_version":1, "event":args.event, "reason":args.reason,
                   "predecessor_revision":state["previous_revision"], "predecessor_sha256":old_hash,
                   "result_revision":state["revision"], "state_sha256":digest(STATE),
                   "gate_snapshot":item["implementation_gate"], "affected_obligations":item["test_ids"],
                   "authorization":"User execution contract", "timestamp":datetime.now(timezone.utc).isoformat()}
        write(OUT/"transitions"/f"{state['revision']:04d}-{args.event.lower()}.json",receipt)
        handoff(state)
