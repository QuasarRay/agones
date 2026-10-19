#!/usr/bin/env python3
"""Select CI work using one dependency policy and available verified evidence."""
import fnmatch
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
POLICY = "verification/agones/check-dependencies.json"
GROUPS = ("rust", "model", "metadata_only")


def git(root, *args):
    return subprocess.check_output(["git", *args], cwd=root, stderr=subprocess.DEVNULL).decode().rstrip("\n")


def matches(path, policy, group):
    return any(fnmatch.fnmatchcase(path, pattern) for pattern in policy[group])


def changed(root, base, head):
    if not base or set(base) == {"0"}:
        return None
    try:
        return [p for p in git(root, "diff", "--name-only", "-z", base, head).split("\0") if p]
    except subprocess.CalledProcessError:
        return None


def checked_file(root, name, expected):
    path = Path(name)
    assert not path.is_absolute() and ".." not in path.parts
    raw = (root / path).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == expected
    return raw


def reusable_model(root, head, policy):
    """A receipt is reusable only for the same declared model program/inputs."""
    try:
        evidence = json.loads((root / "verification/agones/gameserver-defaulting-evidence.json").read_text())
        observed = evidence["assurance"]["current_ci"]
        name = observed["receipt"]
        receipt = json.loads(checked_file(root, name, evidence["artifact_bindings"][name]))
        base = receipt["head"]
        assert re.fullmatch(r"[0-9a-f]{40}", base) and observed["head"] == base
        assert receipt["status"] == "PASSED" and receipt["selected"]["model"] is True
        assert any(j["name"] == "feature-bool-model-check" and j["status"] == "completed"
                   and j["conclusion"] == "success" for j in receipt["jobs"])
        assert receipt["proof_summaries"]
        for proof in receipt["proof_summaries"]:
            counts = proof["status_counts"]
            assert set(counts) <= {"SUCCESS", "SATISFIED", "UNREACHABLE"}
            assert all(type(n) is int and n >= 0 for n in counts.values())
            assert type(proof["harness_count"]) is int and proof["harness_count"] > 0
            assert type(proof["property_count"]) is int and counts.get("SUCCESS", 0) > 0
            assert counts.get("SATISFIED", 0) > 0 and sum(counts.values()) == proof["property_count"]
        assert receipt["artifact_bindings"] and receipt["proof_retention"]
        for path, expected in receipt["artifact_bindings"].items():
            checked_file(root, path, expected)
        for path in receipt["proof_retention"].values():
            retained = json.loads(checked_file(root, path, evidence["artifact_bindings"][path]))
            assert retained["artifact_bindings"]
            for artifact, expected in retained["artifact_bindings"].items():
                checked_file(root, artifact, expected)
        subprocess.run(["git", "merge-base", "--is-ancestor", base, head], cwd=root,
                       check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        assert git(root, "rev-parse", base + "^{tree}") == receipt["tree"]
        previous = json.loads(git(root, "show", base + ":" + POLICY))
        # The policy schedules checks; it is not a model input. A conservative
        # extension is safe only when all newly declared inputs also match.
        assert set(previous["model"]) <= set(policy["model"])
        assert all(set(previous[group]) == set(policy[group]) for group in ("rust", "metadata_only"))
        assert previous.get("unknown_change_policy") == policy.get("unknown_change_policy")
        delta = changed(root, base, head)
        assert delta is not None
        for path in delta:
            if path == POLICY:
                continue
            assert any(matches(path, policy, group) for group in GROUPS)
            assert not matches(path, policy, "model")
        return {"head": base, "receipt": name, "tree": receipt["tree"]}
    except (AssertionError, KeyError, TypeError, ValueError, OSError, subprocess.CalledProcessError):
        return None


def select(root, base, head):
    policy = json.loads((root / POLICY).read_text())
    delta = changed(root, base, head)
    unknown = delta is None or any(not any(matches(p, policy, g) for g in GROUPS) for p in delta)
    selected = {g: unknown or any(matches(p, policy, g) for p in (delta or [])) for g in ("rust", "model")}
    if delta is not None and POLICY in delta:
        selected["model"] = True
    reuse = reusable_model(root, head, policy) if selected["model"] else None
    if reuse:
        selected["model"] = False
    return {"selected": selected, "unknown_dependency_change": unknown,
            "model_evidence_reuse": reuse,
            "retained_evidence": "checkpoint.py validates bindings; reuse requires unchanged declared inputs and retained complete proof artifacts"}


if __name__ == "__main__":
    if os.environ.get("GITHUB_ACTIONS") == "true":
        subprocess.run([sys.executable, str(Path(__file__).with_name("test_select_checks.py"))], check=True)
    result = select(ROOT, os.environ.get("AGONES_CHECK_BASE", ""), os.environ.get("AGONES_CHECK_HEAD", "HEAD"))
    if "GITHUB_OUTPUT" in os.environ:
        with Path(os.environ["GITHUB_OUTPUT"]).open("a") as output:
            for group, enabled in result["selected"].items():
                output.write(f"{group}={str(enabled).lower()}\n")
    print(json.dumps(result))
