#!/usr/bin/env python3
"""Freeze the source tree and check mechanically inherited project guidance.

This is metadata accounting, not semantic extraction or a job/goal scheduler.
Existing guidance is never overwritten, including third-party guidance.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
BASELINE = "6eb4a02588d8e7b581de187f3bc849733a78dcb2"
OUT = ROOT / "verification/agones"


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def guidance(apply):
    paths = set(git("ls-files", "--cached", "--others", "--exclude-standard", "-z").decode().strip("\0").split("\0"))
    tracked = set(git("ls-files", "--cached", "-z").decode().strip("\0").split("\0"))
    directories = {ROOT}
    for path in paths:
        parent = (ROOT / path).parent
        while parent != ROOT:
            directories.add(parent)
            parent = parent.parent
    inherited = "# AGENTS.MD\n\nProject guidance is inherited from [the repository root]({relative}).\nRead that file once; preserve any additional ancestor or local guidance.\nThis pointer adds no scope, permissions, or exceptions.\n"
    missing, existing, created, not_versioned = [], [], [], []
    import os
    for directory in sorted(directories):
        files = [p for p in directory.iterdir() if p.name.upper() == "AGENTS.MD"]
        if files:
            existing.extend(str(p.relative_to(ROOT)) for p in files)
            not_versioned.extend(str(p.relative_to(ROOT)) for p in files if str(p.relative_to(ROOT)) not in tracked)
            continue
        path = directory / "AGENTS.MD"
        if apply:
            relative = os.path.relpath(ROOT / "AGENTS.MD", directory)
            path.write_text(inherited.format(relative=relative))
            created.append(str(path.relative_to(ROOT)))
            not_versioned.append(str(path.relative_to(ROOT)))
        else:
            missing.append(str(path.relative_to(ROOT)))
    return {"directories": len(directories), "missing": missing, "preserved_existing": existing, "created": created, "not_versioned": not_versioned,
            "ownership": "Inherited operational guidance includes tracked vendor, generated and documentation directories; existing third-party instructions are preserved unchanged."}


def census():
    OUT.mkdir(parents=True, exist_ok=True)
    tree = git("ls-tree", "-r", BASELINE).decode()
    (OUT / "baseline-tree.tsv").write_text(tree)
    records = [line.split("\t", 1)[1] for line in tree.splitlines()]
    roots = sorted({p.split("/", 1)[0] for p in records if "/" in p})
    write_json(OUT / "scope-v1.json", {
        "schema_version": 1, "revision": 1, "baseline": BASELINE,
        "scope_id": "agones-original-v1", "inventory_sha256": sha(tree.encode()),
        "discovered_roots": roots,
        "default_classification": "REQUIRED", "exclusions": [],
        "surfaces": [{"id": name, "classification": "REQUIRED", "semantic_obligations": "UNRESOLVED"} for name in [
            "GameServer", "GameServerSet", "Fleet", "FleetAutoscaler", "GameServerAllocation",
            "GameServerAllocationPolicy/multicluster", "allocator-grpc-rest-mtls-auth",
            "SDK-streaming-health-counters-lists", "SDK-rust", "SDK-other-language-compatibility",
            "schema-protobuf-openapi-defaulting-admission-validation", "selectors-metadata-resource-version-deletion",
            "feature-gates-deprecated-fields", "configuration-telemetry-lifecycle", "deployment-and-generated-artifacts"]],
        "notes": ["All 8240 pinned files are retained, including vendor and discovered roots; no convenience exclusions.",
                  "Mixed orchestrator-specific and portable surfaces remain REQUIRED until split with provenance.",
                  "Compatibility for non-Rust language SDKs is required; rewriting each SDK in Rust is not implied.",
                  "Existing Rust files in the reference commit are recovery inputs, not evidence of original behavior parity.",
                  "Dynamic subtests, table rows and assertion counts remain unresolved. Unknown counts prevent completion."]})
    tests = subprocess.check_output(["go", "run", "./build/scripts/agones-rs/census.go", BASELINE], cwd=ROOT)
    (OUT / "baseline-tests.json").write_bytes(tests)
    parsed = json.loads(tests)
    write_json(OUT / "baseline.json", {
        "schema_version": 1, "repository": "https://github.com/QuasarRay/agones",
        "commit": BASELINE, "tree": git("rev-parse", BASELINE+"^{tree}").decode().strip(),
        "selection": "A1(3): default inspected fork commit; no earlier migration baseline found",
        "upstream_lineage": "UNRESOLVED", "files": len(records),
        "tree_inventory_sha256": sha(tree.encode()), "test_inventory_sha256": sha(tests),
        "top_level_test_like_declarations": len(parsed),
        "dynamic_run_sites": sum(t["dynamic_run_sites"] for t in parsed),
        "nested_case_assertion_count": "UNRESOLVED",
        "metadata_limits": "Standard go/parser identifies top-level test-like declarations and literal Run labels, not package/type-resolved semantics or dynamic expansions. Vendor test semantics are dependency inputs, not first-party tests.",
        "go_environment": subprocess.check_output(["go", "env", "-json", "GOVERSION", "GOOS", "GOARCH", "GOFLAGS", "CGO_ENABLED"], cwd=ROOT).decode(),
        "contract_sha256": sha((ROOT / "AGONES_RS_CONTINUATION_PROMPT.md").read_bytes()),
        "preserved_untracked_inputs": ["AGONES_RS_CONTINUATION_PROMPT.md", "AGONES_RS_PROMPT_AUDIT_RESOLUTION.md"]})


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=["census", "guidance-check", "guidance-apply"])
    args = parser.parse_args()
    if args.action == "census":
        census()
    else:
        report = guidance(args.action == "guidance-apply")
        OUT.mkdir(parents=True, exist_ok=True)
        write_json(OUT / "guidance.json", report)
        print(json.dumps({k: v if not isinstance(v, list) else len(v) for k, v in report.items()}))
        if report["missing"] or (args.action == "guidance-check" and report["not_versioned"]):
            raise SystemExit(1)
