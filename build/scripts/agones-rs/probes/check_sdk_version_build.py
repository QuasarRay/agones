"""Ordinary normal-Cargo build-input/invalidation checks; no semantic extraction."""
import json
import os
import subprocess
import tempfile
from pathlib import Path

command = ["cargo", "+1.98.1", "build", "--locked", "--offline", "--message-format=json",
           "-p", "agones-core", "--example", "build_version_snapshot"]
project = Path.cwd()


def snapshot(environment, directory, outside=False):
    build = command + (["--manifest-path", str(project / "crates/agones-core/Cargo.toml")] if outside else [])
    result = subprocess.run(build, cwd=directory, env=environment, capture_output=True, check=True)
    messages = [json.loads(line) for line in result.stdout.splitlines() if line.startswith(b"{")]
    artifacts = [m for m in messages if m.get("reason") == "compiler-artifact"]
    executable = next(m["executable"] for m in artifacts if m["target"]["name"] == "build_version_snapshot")
    library = next(m for m in artifacts if m["target"]["name"] == "agones_core" and "lib" in m["target"]["kind"])
    observed = json.loads(subprocess.run([executable], env=environment, capture_output=True, check=True).stdout)
    return observed, library["fresh"]


fixtures = [
    ("unconfigured", None, b"dev", "dev"),
    ("release", b"1.54.0-dev+abc", b"1.54.0-dev+abc", "1.54.0-dev+abc"),
    ("explicit-empty", b"", b"", ""),
    ("escaped-text", b'v"\\\n\t', b'v"\\\n\t', 'v"\\\n\t'),
    ("raw-truncated", b"\xe2\x82", b"\xe2\x82", "��"),
    ("raw-surrogate", b"\xed\xa0\x80", b"\xed\xa0\x80", "���"),
    ("unconfigured-after-overrides", None, b"dev", "dev"),
    ("unchanged-workspace-build", None, b"dev", "dev"),
]
observations = []
for name, configured, expected_bytes, expected in fixtures:
    environment = os.environb.copy()
    environment.pop(b"AGONES_BUILD_VERSION", None)
    if configured is not None:
        environment[b"AGONES_BUILD_VERSION"] = configured
    observed, fresh = snapshot(environment, project)
    assert observed["raw_bytes"] == list(expected_bytes), (name, observed)
    for key in ["constant", "provider", "game_server", "fleet"]:
        assert observed[key] == expected, (name, key, observed)
    if name in ["unconfigured", "release", "explicit-empty"]:
        original_command = ["go", "run"]
        if configured is not None:
            original_command += ["-ldflags", "-X agones.dev/agones/pkg.Version=" + configured.decode("ascii")]
        original_command += ["./build/scripts/agones-rs/probes/sdk_version_replay.go", "--compiled"]
        original = json.loads(subprocess.run(original_command, capture_output=True, check=True).stdout)
        assert original["raw_hex"] == expected_bytes.hex(), (name, original)
        for key in ["constant", "provider", "game_server", "fleet", "pod"]:
            assert original[key] == expected, (name, key, original)
    if name == "unchanged-workspace-build":
        assert fresh, "ordinary unchanged workspace builds must reuse the core artifact"
    observations.append({"id": name, "raw_hex": expected_bytes.hex(), "annotation": expected, "core_artifact_fresh": fresh})
with tempfile.TemporaryDirectory(prefix="agones-version-build-") as outside:
    for name, configured, expected_bytes, expected in [
        ("outside-unconfigured", None, b"dev", "dev"),
        ("outside-raw", b"\xe2\x82", b"\xe2\x82", "��"),
        ("outside-restored", None, b"dev", "dev"),
    ]:
        environment = os.environb.copy()
        environment.pop(b"AGONES_BUILD_VERSION", None)
        if configured is not None:
            environment[b"AGONES_BUILD_VERSION"] = configured
        observed, fresh = snapshot(environment, outside, True)
        assert observed["raw_bytes"] == list(expected_bytes), (name, observed)
        assert all(observed[key] == expected for key in ["constant", "provider", "game_server", "fleet"]), (name, observed)
        observations.append({"id": name, "raw_hex": expected_bytes.hex(), "annotation": expected, "core_artifact_fresh": fresh})
# Restore the caller's ordinary Cargo configuration in the same target cache.
# The caller's environment is never mutated or printed.
snapshot(os.environb.copy(), project)
print(json.dumps({"schema_version": 1, "classification": "ORDINARY_BUILD_INPUT_CHECK",
                  "command": command, "normal_cargo_incremental_builds": len(observations),
                  "no_cargo_clean": True, "observations": observations}, ensure_ascii=False))
