#!/usr/bin/env python3
"""Record one foreground check with its exact command, source binding and result."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "verification/agones/runs"
OUT.mkdir(parents=True, exist_ok=True)


def main(args):
    name, *command = args
    run_cwd = ROOT
    quiet = False
    if command[:1] == ["--quiet"]:
        quiet = True
        command = command[1:]
    if command[:1] == ["--cwd"]:
        run_cwd = (ROOT / command[1]).resolve()
        command = command[2:]
    path = OUT / (name + ".json")
    if path.exists():
        raise SystemExit("A run ID is immutable; select a new ID after diagnosing the previous result")
    files = subprocess.check_output(["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=ROOT).decode().strip("\0").split("\0")
    bindings = {p: hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in files if (ROOT/p).is_file() and (p.startswith(("crates/", "build/scripts/agones-rs/")) or p in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", ".cargo/config.toml", ".cargo/config", "go.mod", "go.sum", "AGONES_RS_CONTINUATION_PROMPT.md") or (Path(p).parent == Path("verification/agones") and p.endswith(("-witnesses.json", ".ir.json", "-reference.json", "-expected.bin", "-table.json"))))}
    record = {"schema_version": 1, "owner_token": os.urandom(16).hex(), "command": command, "cwd": str(run_cwd),
              "started_at": datetime.now(timezone.utc).isoformat(), "source_files": bindings,
              "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
              "status": "INTENT_RECORDED", "classification": "REPO_SAFE", "log": "verification/agones/runs/"+name+".log",
              "exit_code": None, "attempt": 1}
    record["reference_inventory_sha256"] = hashlib.sha256((ROOT/"verification/agones/baseline-tree.tsv").read_bytes()).hexdigest()
    record["tracked_patch_sha256"] = hashlib.sha256(subprocess.check_output(["git", "diff", "--binary", "HEAD"], cwd=ROOT)).hexdigest()
    record["runner_sha256"] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    record["build_inputs"] = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
                              for p in sorted((ROOT/"install/helm/agones").rglob("*.yaml"))}

    def save():
        temp = path.with_suffix(".tmp")
        temp.write_text(json.dumps(record, indent=2, sort_keys=True)+"\n")
        temp.replace(path)


    save()
    process = None
    log_hash = hashlib.sha256()
    with (OUT/(name+".log")).open("wb") as log:
        try:
            process = subprocess.Popen(command, cwd=run_cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            record.update(status="RUNNING", pid=process.pid, process_identity=Path(f"/proc/{process.pid}/stat").read_text())
            save()
            # A CBMC line can be very large and need not be valid UTF-8. Bound
            # recorder memory while retaining and hashing every original byte.
            with process.stdout:
                for chunk in iter(lambda: process.stdout.read1(64 * 1024), b""):
                    log.write(chunk)
                    log.flush()
                    log_hash.update(chunk)
                    if not quiet:
                        sys.stdout.buffer.write(chunk)
                        sys.stdout.buffer.flush()
            code = process.wait()
        except (OSError, KeyboardInterrupt) as error:
            if process is not None and process.poll() is None:
                process.terminate()
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()
            code = 130 if isinstance(error, KeyboardInterrupt) else 125
            record["recorder_error"] = type(error).__name__
    record.update(status="PASSED" if code==0 else "FAILED", exit_code=code,
                  finished_at=datetime.now(timezone.utc).isoformat(), log_sha256=log_hash.hexdigest())
    save()
    if quiet:
        print(f"{name}: {record['status']}; log {record['log']}")
    return code


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
