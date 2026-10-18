#!/usr/bin/env python3
"""Select CI checks from the same explicit source/evidence dependency policy."""
import fnmatch
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
policy = json.loads((ROOT/"verification/agones/check-dependencies.json").read_text())
base = os.environ.get("AGONES_CHECK_BASE", "")
head = os.environ.get("AGONES_CHECK_HEAD", "HEAD")
if not base or set(base) == {"0"}:
    changed = None
else:
    result = subprocess.run(["git", "diff", "--name-only", "-z", base, head], cwd=ROOT, capture_output=True)
    changed = result.stdout.decode().strip("\0").split("\0") if result.returncode == 0 else None
    if changed == [""]:
        changed = []
matches = lambda path, group: any(fnmatch.fnmatchcase(path, pattern) for pattern in policy[group])
unknown = changed is None or any(not any(matches(path, group) for group in ("rust", "model", "metadata_only")) for path in changed)
selected = {group: unknown or any(matches(path, group) for path in (changed or [])) for group in ("rust", "model")}
if "GITHUB_OUTPUT" in os.environ:
    with Path(os.environ["GITHUB_OUTPUT"]).open("a") as output:
        for group, enabled in selected.items():
            output.write(f"{group}={str(enabled).lower()}\n")
print(json.dumps({"selected":selected,"unknown_dependency_change":unknown,"retained_evidence":"validated by checkpoint.py; unchanged proof artifacts are explicitly reused"}))
