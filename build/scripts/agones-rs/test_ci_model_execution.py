"""Failure controls for bounded execution and exact-byte evidence recording."""
import contextlib
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

import protect_model as protection
from kani_results import require_bolero_command

spec = importlib.util.spec_from_file_location("check_recorder", Path(__file__).with_name("run.py"))
recorder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(recorder)


class ExecutionControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)

    def envelope_fixture(self):
        proc, groups = self.root / "proc", self.root / "groups"
        (proc / "self").mkdir(parents=True)
        unit = "agones-model-" + "a" * 32 + ".service"
        (proc / "self/cgroup").write_text(f"0::/system.slice/{unit}\n")
        (proc / "meminfo").write_text("MemTotal:       16000000 kB\n")
        group = groups / "system.slice" / unit
        group.mkdir(parents=True)
        for key, value in {"memory.max": str(protection.MEMORY_BYTES), "memory.swap.max": "0",
                           "memory.oom.group": "1", "cpu.max": "300000 100000"}.items():
            (group / key).write_text(value + "\n")
        return proc, groups, group, unit

    def test_observed_controls_fail_closed(self):
        proc, groups, group, unit = self.envelope_fixture()
        self.assertEqual(protection.verify_envelope(unit, proc, groups)["memory_bytes"], protection.MEMORY_BYTES)
        for key, bad in [("memory.max", "max"), ("memory.max", str(4 * 1024**3)),
                         ("memory.swap.max", "max"), ("memory.oom.group", "0"),
                         ("cpu.max", "max 100000"), ("cpu.max", "400000 100000")]:
            with self.subTest(key=key, bad=bad):
                previous = (group / key).read_text()
                (group / key).write_text(bad)
                with self.assertRaises(ValueError):
                    protection.verify_envelope(unit, proc, groups)
                (group / key).write_text(previous)
        (proc / "self/cgroup").write_text("0::/wrong.service\n")
        with self.assertRaises(ValueError):
            protection.verify_envelope(unit, proc, groups)

    def test_service_keeps_recorder_outside_and_forwards_only_build_environment(self):
        command = ["cargo", "bolero", "test", "example_domain", "--engine", "kani"]
        self.assertEqual(protection.selection(command), command)
        for bad in [[], command + ["--unwind", "1"], ["sh", "-c", "true"], command[:-1]+["libfuzzer"]]:
            with self.assertRaises(ValueError):
                protection.selection(bad)
        argv = protection.service_command(command, "test.service",
                                          {"PATH": "/literal/$path", "HOME": "/home/runner", "GITHUB_TOKEN": "secret"})
        self.assertEqual(argv[:5], ["sudo", "--non-interactive", "systemd-run", "--wait", "--pipe"])
        self.assertIn("--expand-environment=no", argv)
        self.assertIn("--property=KillMode=control-group", argv)
        self.assertIn("--property=RuntimeMaxSec=930", argv)
        self.assertIn("--setenv=PATH=/literal/$path", argv)
        self.assertFalse(any("GITHUB_TOKEN" in part or "secret" in part for part in argv))
        self.assertEqual(argv[-6:], command)

    def test_proof_route_requires_bound_actual_envelope(self):
        proc, groups, _, unit = self.envelope_fixture()
        envelope = protection.verify_envelope(unit, proc, groups)
        command = ["cargo", "bolero", "test", "example_domain", "--engine", "kani"]
        wrapped = ["python3", "../../build/scripts/agones-rs/protect_model.py", *command]
        run = {"command": wrapped, "source_files": {
            "build/scripts/agones-rs/protect_model.py": envelope["script_sha256"]}}
        raw = protection.ENVELOPE_PREFIX + json.dumps(envelope) + "\n"
        require_bolero_command(run, "example_domain", raw)
        require_bolero_command({"command": command}, "example_domain", "")
        for altered, log in [(run, ""), (run, raw + raw),
                             ({**run, "command": ["fake-wrapper", *command]}, raw),
                             ({**run, "command": wrapped[:-1]+["libfuzzer"]}, raw),
                             ({**run, "source_files": {"build/scripts/agones-rs/protect_model.py": "0" * 64}}, raw),
                             (run, raw.replace(str(protection.MEMORY_BYTES), "1"))]:
            with self.assertRaises(AssertionError):
                require_bolero_command(altered, "example_domain", log)

    def initialize_recorder(self):
        subprocess.run(["git", "init", "-q", str(self.root)], check=True)
        subprocess.run(["git", "-C", str(self.root), "-c", "user.name=Control",
                        "-c", "user.email=control@example.invalid", "commit", "-q", "--allow-empty", "-m", "control"], check=True)
        output = self.root / "verification/agones/runs"
        output.mkdir(parents=True)
        (self.root / "verification/agones/baseline-tree.tsv").write_text("control\n")
        return output

    def test_recorder_preserves_large_binary_line_and_terminal_failure(self):
        output = self.initialize_recorder()
        payload = b"x" * (8 * 1024**2) + b"\xff\x00\xfe"
        script = "import sys; sys.stdout.buffer.write(b'x'*(8*1024**2)+b'\\xff\\x00\\xfe'); sys.exit(23)"
        with patch.object(recorder, "ROOT", self.root), patch.object(recorder, "OUT", output), contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(recorder.main(["binary", "--quiet", sys.executable, "-c", script]), 23)
            record = json.loads((output / "binary.json").read_text())
            self.assertEqual(record["status"], "FAILED")
            self.assertEqual(record["exit_code"], 23)
            self.assertTrue(record["finished_at"])
            self.assertEqual((output / "binary.log").read_bytes(), payload)
            self.assertEqual(record["log_sha256"], hashlib.sha256(payload).hexdigest())
            with self.assertRaises(SystemExit):
                recorder.main(["binary", "--quiet", "/usr/bin/true"])

    def test_recorder_spawn_failure_is_terminal_and_never_passes(self):
        output = self.initialize_recorder()
        with patch.object(recorder, "ROOT", self.root), patch.object(recorder, "OUT", output), contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(recorder.main(["missing", "--quiet", str(self.root / "missing-command")]), 125)
            record = json.loads((output / "missing.json").read_text())
            self.assertEqual(record["status"], "FAILED")
            self.assertEqual(record["recorder_error"], "FileNotFoundError")
            self.assertEqual(record["log_sha256"], hashlib.sha256(b"").hexdigest())


if __name__ == "__main__":
    unittest.main()
