#!/usr/bin/env python3
"""Ordinary Cargo checks of the existing resource derive, including aliases."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[3]
PINNED = tomllib.loads((ROOT / "Cargo.lock").read_text())
identity = lambda p: (p["name"], p["version"], p.get("source"), p.get("checksum"))
allowed = set(map(identity, PINNED["package"]))
results = []
attributes = 'group="agones.dev", version="v1", kind="GameServer", plural="gameservers"'
positive = '''
trait AgonesApiResource {}
#[derive(derive_tools::AgonesApiResource)]
#[agones_resource(ATTRS)]
struct Shape<T, const N: usize> where T: Clone {
    marker: std::marker::PhantomData<[T;N]>,
}
#[test] fn identity() {
    assert_eq!(<Shape<String,3> as renamed_core::AgonesApiResource>::API_VERSION,"agones.dev/v1");
    assert_eq!(<Shape<String,3> as renamed_core::AgonesApiResource>::KIND,"GameServer");
    assert_eq!(<Shape<String,3> as renamed_core::AgonesApiResource>::PLURAL,"gameservers");
}
'''
cases = [
    ("renamed", True, positive.replace("ATTRS", attributes), None),
    ("facade", True, 'mod facade { pub use renamed_core as runtime; }\n' +
        positive.replace("ATTRS", attributes+', core_path="crate::facade::runtime"'), None),
    ("missing-runtime", False,
        '#[derive(derive_tools::AgonesApiResource)] #[agones_resource('+attributes+')] struct Shape;',
        "AgonesApiResource requires an agones-core dependency or core_path"),
    ("invalid-path", True, positive.replace("ATTRS", attributes+', core_path="not::"'),
        "core_path must be a valid Rust path"),
    ("duplicate-path", True, positive.replace("ATTRS", attributes+', core_path="renamed_core", core_path="renamed_core"'),
        "duplicate core_path"),
]
env = dict(os.environ, CARGO_NET_OFFLINE="true", CARGO_BUILD_JOBS="4",
           CARGO_TARGET_DIR=str(ROOT / "target"), AGONES_BUILD_VERSION="dev")
for name, runtime, source, diagnostic in cases:
    with tempfile.TemporaryDirectory(prefix="agones-resource-derive-") as directory:
        path = Path(directory)
        (path/"src").mkdir()
        manifest = '[package]\nname="resource-derive-fixture"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\n'
        manifest += 'derive_tools={package="agones-macros",path='+json.dumps(str(ROOT/"crates/agones-macros"))+'}\n'
        if runtime:
            manifest += 'renamed_core={package="agones-core",path='+json.dumps(str(ROOT/"crates/agones-core"))+'}\n'
        (path/"Cargo.toml").write_text(manifest)
        (path/"Cargo.lock").write_bytes((ROOT/"Cargo.lock").read_bytes())
        (path/"src/lib.rs").write_text(source)
        prepare = subprocess.run(["cargo", "+1.98.1", "update", "--workspace", "--offline", "--manifest-path", str(path/"Cargo.toml")],
                                 env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        assert prepare.returncode == 0, prepare.stdout
        resolution = tomllib.loads((path/"Cargo.lock").read_text())
        assert all(identity(p) in allowed or p["name"] == "resource-derive-fixture" for p in resolution["package"]), "fixture dependency resolution drift"
        result = subprocess.run(["cargo", "+1.98.1", "test", "--locked", "--offline", "--manifest-path", str(path/"Cargo.toml")],
                                env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        if diagnostic is None:
            assert result.returncode == 0 and "1 passed; 0 failed" in result.stdout, result.stdout
        else:
            assert result.returncode != 0 and diagnostic in result.stdout, result.stdout
        results.append(dict(name=name, status="PASSED", expectation="COMPILES_AND_RUNS" if diagnostic is None else "COMPILE_REJECTED",
                            expected_diagnostic=diagnostic, manifest=manifest, source=source,
                            package_identities_unchanged=True))
assert len(results) == 5
print(json.dumps(dict(schema_version=1, classification="CARGO_RESOURCE_DERIVE_HYGIENE_NOT_2_MATRIX_CREDIT",
                     positive_cases=2, compile_fail_cases=3, cases=results), indent=2))
