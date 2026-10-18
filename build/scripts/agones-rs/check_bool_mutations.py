#!/usr/bin/env python3
"""Bounded lexical fault controls in isolated Rust crate identities/targets."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
source = (ROOT / "crates/agones-core/src/features.rs").read_text()
start = source.index("pub fn parse_feature_bool(")
end = source.index("\n}\n", start)+2
function = source[start:end]
fixtures = [(b"1", True), (b"t", True), (b"T", True), (b"TRUE", True), (b"true", True), (b"True", True),
            (b"0", False), (b"f", False), (b"F", False), (b"FALSE", False), (b"false", False), (b"False", False),
            (b"", None), (b"tRuE", None), (b" true", None), (b"false ", None), (b"\x00", None)]
assertions = "\n".join(f"assert_eq!(parse_feature_bool(&{list(token)}), " + ("None" if value is None else f"Some({str(value).lower()})") + ");" for token,value in fixtures)
mutations = {
    "flip_false": function.replace("match value {", 'if value == b"False" { return Some(true); }\n    match value {'),
    "accept_mixed_case": function.replace("match value {", 'if value == b"tRuE" { return Some(true); }\n    match value {'),
}
results=[]
with tempfile.TemporaryDirectory(prefix="agones-bool-negative-controls-") as scratch:
    for name, body in [("control", function), *mutations.items()]:
        path=Path(scratch)/f"{name}.rs"
        path.write_text(body+"\n#[test]\nfn source_witnesses_detect_faults() {\n"+assertions+"\n}\n")
        binary=Path(scratch)/name
        subprocess.run(["rustc", "--edition=2024", "--test", "--crate-name", f"agones_feature_bool_{name}", str(path), "-o", str(binary)], check=True)
        result=subprocess.run([str(binary)], capture_output=True, text=True)
        expected=0 if name=="control" else 101
        assert result.returncode==expected, (name,result.stdout,result.stderr)
        results.append({"name":name,"crate_identity":f"agones_feature_bool_{name}","source_sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"expected_exit":expected,"actual_exit":result.returncode,"witness_count":len(fixtures),"result":"CONTROL_PASSED" if name=="control" else "FAULT_DETECTED","failure_output":result.stdout if name!="control" else None})
report={"schema_version":1,"production_unchanged_sha256":hashlib.sha256((ROOT/"crates/agones-core/src/features.rs").read_bytes()).hexdigest(),"function_span_sha256":hashlib.sha256(function.encode()).hexdigest(),"results":results,"scope":"Concrete negative controls only; these isolated targets are not production artifacts or formal proofs."}
(ROOT/"verification/agones/feature-bool-negative-controls.json").write_text(json.dumps(report,indent=2,sort_keys=True)+"\n")
print(json.dumps({r['name']:r['result'] for r in results}))
