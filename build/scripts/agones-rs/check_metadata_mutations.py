#!/usr/bin/env python3
"""Check source metadata invariants against isolated, compiling action faults."""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[3]
path = "crates/agones-core/tests/generated/object_metadata_cases.rs"
original = (ROOT / path).read_text()
metadata = json.loads((ROOT / "verification/agones/object-metadata-codegen.json").read_text())
assert hashlib.sha256(original.encode()).hexdigest() == metadata["output_sha256"]
# Disable only the additional native snapshot/differential helper in these
# isolated targets, so a mutant must fail an original qualified source assertion.
start = original.index("fn source_metadata_native_projection(")
body = original.index("{", start)
end = original.index("\nfn source_metadata_assertion_000", body)
source = original[:body] + "{ let _ = (labels,annotations,expected_labels,expected_annotations,actual,reference); }\n" + original[end:]

def remove_update(receiver, field):
    pattern = rf'{receiver}\s*\.\s*{field}\s*\.\s*insert\(\s*"\$\$\$\$"\.to_owned\(\),\s*"no"\.to_owned\(\)\s*\);'
    changed, count = re.subn(pattern, "", source)
    assert count == 1, (receiver, field, count)
    return changed

copy = "let mut invalid_1 = mp.clone();"
assert source.count(copy) == 1
assert source.count('"spec.metadata"') == 6
cases = [
    ("control", source, None),
    ("missing_label_update", remove_update("invalid_0", "labels"), "::line-331"),
    ("missing_annotation_update", remove_update("invalid_1", "annotations"), "::line-339"),
    ("missing_copy_reset", source.replace(copy, "let mut invalid_1 = invalid_0.clone();"), "::line-339"),
    ("wrong_field_path", source.replace('"spec.metadata"', '"wrong.metadata"'), "::line-332"),
]
identity = lambda p: (p["name"], p["version"], p.get("source"), p.get("checksum"))
allowed = set(map(identity, tomllib.loads((ROOT / "Cargo.lock").read_text())["package"]))
env = {**os.environ, "CARGO_NET_OFFLINE": "true", "CARGO_BUILD_JOBS": "2",
       "CARGO_TARGET_DIR": str(ROOT / "target"), "AGONES_BUILD_VERSION": "dev"}
results = []
for name, code, expected_origin in cases:
    with tempfile.TemporaryDirectory(prefix="agones-metadata-action-fault-") as directory:
        scratch = Path(directory)
        (scratch / "src").mkdir()
        package = "agones_metadata_action_" + name
        manifest = f'[package]\nname="{package}"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\n'
        manifest += 'agones-core={path=' + json.dumps(str(ROOT / "crates/agones-core")) + '}\n'
        manifest += 'cgp="=0.8.0-alpha"\nbolero="=0.13.4"\nserde_json="1.0"\n'
        manifest += '[lints.rust]\nunexpected_cfgs={level="warn",check-cfg=["cfg(kani)"]}\n'
        (scratch / "Cargo.toml").write_text(manifest)
        (scratch / "Cargo.lock").write_bytes((ROOT / "Cargo.lock").read_bytes())
        (scratch / "src/lib.rs").write_text(code)
        base = ["cargo", "+1.98.1"]
        prepared = subprocess.run(base + ["update", "--workspace", "--offline", "--manifest-path", str(scratch / "Cargo.toml")], env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        assert prepared.returncode == 0, prepared.stdout
        lock = tomllib.loads((scratch / "Cargo.lock").read_text())
        assert all(identity(p) in allowed or p["name"] == package for p in lock["package"]), "mutation dependency drift"
        checked = subprocess.run(base + ["test", "--locked", "--offline", "--manifest-path", str(scratch / "Cargo.toml"), "--lib", "source_metadata_case_005", "--", "--exact"], env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        if expected_origin is None:
            assert checked.returncode == 0 and "1 passed; 0 failed" in checked.stdout, checked.stdout
        else:
            assert checked.returncode == 101 and "test result: FAILED" in checked.stdout, checked.stdout
            assert "TestMetaPatchValidate" + expected_origin in checked.stdout, checked.stdout
        results.append({"name": name, "package_identity": package, "status": "CONTROL_PASSED" if expected_origin is None else "SOURCE_ASSERTION_FAULT_DETECTED", "code_sha256": hashlib.sha256(code.encode()).hexdigest(), "exit_code": checked.returncode, "expected_origin": expected_origin, "failure_output": checked.stdout if expected_origin else None})
assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == metadata["output_sha256"]
report = {"schema_version": 1, "feature_id": "META-VALIDATE-001", "generated_source": path,
          "generated_source_sha256": metadata["output_sha256"], "source_ir_sha256": metadata["ir_sha256"],
          "control_passed": 1, "action_faults_detected": 4, "results": results,
          "scope": "Ordinary concrete mutation evidence. Original source assertions detect missing label/annotation updates, copy-state reset and field path faults. Additional native projections are disabled only in isolated control crates. Production and generated artifacts remain unchanged. No backend proof credit."}
(ROOT / "verification/agones/object-metadata-negative-controls.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
print(json.dumps({r["name"]: r["status"] for r in results}))
