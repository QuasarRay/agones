#!/usr/bin/env python3
"""Run one unchanged Bolero/Kani selection in a bounded CI service cgroup.

The recorder and Actions runner stay outside this cgroup. Missing controls are
an explicit failed preflight; there is no unprotected fallback.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import uuid

ROOT = Path(__file__).resolve().parents[3]
SCRIPT = Path(__file__).resolve()
MEMORY_BYTES = 12 * 1024**3
MIN_HOST_BYTES = 14 * 1024**3
CPU_COUNT = 3
TIMEOUT_SECONDS = 900
ENVELOPE_PREFIX = "AGONES_MODEL_ENVELOPE "
BUILD_ENV = ("PATH", "HOME", "CARGO_HOME", "RUSTUP_HOME", "CARGO_BUILD_JOBS",
             "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTUP_TOOLCHAIN",
             "AGONES_BUILD_VERSION", "LANG", "LC_ALL", "TZ")


def selection(command):
    if command == ["--probe"]:
        return ["/usr/bin/true"]
    if (len(command) != 6 or command[:3] != ["cargo", "bolero", "test"]
            or not re.fullmatch(r"[a-z][a-z0-9_]*", command[3])
            or command[4:] != ["--engine", "kani"]):
        raise ValueError("select exactly one real cargo bolero test HARNESS --engine kani")
    return command


def host_memory(proc=Path("/proc")):
    row = next(line for line in (proc / "meminfo").read_text().splitlines()
               if line.startswith("MemTotal:"))
    return int(row.split()[1]) * 1024


def verify_envelope(unit, proc=Path("/proc"), cgroups=Path("/sys/fs/cgroup")):
    lines = (proc / "self/cgroup").read_text().splitlines()
    unified = [line[3:] for line in lines if line.startswith("0::")]
    if len(unified) != 1:
        raise ValueError("one cgroup v2 membership is required")
    relative = Path(unified[0].lstrip("/"))
    if ".." in relative.parts or relative.name != unit:
        raise ValueError("verifier is outside the intended service cgroup")
    group = cgroups / relative
    observed = {name: (group / name).read_text().strip() for name in
                ("memory.max", "memory.swap.max", "memory.oom.group", "cpu.max")}
    if (observed["memory.max"] != str(MEMORY_BYTES)
            or observed["memory.swap.max"] != "0"
            or observed["memory.oom.group"] != "1"):
        raise ValueError("required whole-tree memory/OOM controls are not active")
    quota, period = observed["cpu.max"].split()
    if quota == "max" or int(quota) != CPU_COUNT * int(period):
        raise ValueError("required CPU headroom control is not active")
    return {"schema_version": 1, "unit": unit, "cgroup": unified[0],
            "observed_controls": observed, "memory_bytes": MEMORY_BYTES,
            "cpu_count": CPU_COUNT, "timeout_seconds": TIMEOUT_SECONDS,
            "kill_after_seconds": 15, "service_runtime_seconds": 930,
            "host_memory_bytes": host_memory(proc), "uid": os.getuid(),
            "gid": os.getgid(), "script_sha256": hashlib.sha256(SCRIPT.read_bytes()).hexdigest()}


def service_command(command, unit, environ):
    # --expand-environment=no preserves literal argument bytes (not shell code).
    result = ["sudo", "--non-interactive", "systemd-run", "--wait", "--pipe",
              "--service-type=exec", "--expand-environment=no", f"--unit={unit}",
              f"--uid={os.getuid()}", f"--gid={os.getgid()}",
              f"--working-directory={Path.cwd()}",
              f"--property=MemoryMax={MEMORY_BYTES}", "--property=MemorySwapMax=0",
              "--property=MemoryAccounting=yes", "--property=OOMPolicy=kill",
              f"--property=CPUQuota={CPU_COUNT * 100}%", "--property=CPUAccounting=yes",
              "--property=KillMode=control-group", "--property=RuntimeMaxSec=930",
              "--property=TimeoutStopSec=15"]
    result.extend(f"--setenv={key}={environ[key]}" for key in BUILD_ENV if key in environ)
    return result + [sys.executable, str(SCRIPT), "--inside", unit, *command]


def main(args):
    if args[:1] == ["--inside"]:
        unit, *command = args[1:]
        actual = selection(command)
        envelope = verify_envelope(unit)
        print(ENVELOPE_PREFIX + json.dumps(envelope, sort_keys=True), flush=True)
        os.execvp("timeout", ["timeout", "--signal=TERM", "--kill-after=15s",
                             f"{TIMEOUT_SECONDS}s", "/usr/bin/time", "-v", *actual])
    selection(args)
    if (Path("/proc/1/comm").read_text().strip() != "systemd"
            or not Path("/sys/fs/cgroup/cgroup.controllers").is_file()):
        raise ValueError("systemd and cgroup v2 are required; refusing unprotected execution")
    if host_memory() < MIN_HOST_BYTES or (os.cpu_count() or 0) < CPU_COUNT + 1:
        raise ValueError("host needs at least 14 GiB RAM and four CPUs for the fixed CI envelope")
    unit = f"agones-model-{uuid.uuid4().hex}.service"
    result = subprocess.run(service_command(args, unit, os.environ), stdin=subprocess.DEVNULL)
    # systemd-run --wait prints exit state and resource accounting, even on OOM.
    # Retain a failed unit's diagnostic properties before releasing its metadata.
    subprocess.run(["systemctl", "show", unit, "--property=Result,ExecMainCode,ExecMainStatus,MemoryPeak"],
                   timeout=15, check=False)
    subprocess.run(["sudo", "--non-interactive", "systemctl", "reset-failed", unit],
                   timeout=15, check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return result.returncode


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except (OSError, ValueError, subprocess.TimeoutExpired) as error:
        print(f"Model protection failed: {error}", file=sys.stderr, flush=True)
        raise SystemExit(125)
