#!/usr/bin/env python3
"""Pin the exact upstream CPG protoc for build-time descriptor/loader comparisons."""
import hashlib
import json
from pathlib import Path
import subprocess
import urllib.request

ROOT = Path(__file__).resolve().parents[3]
BASE = 'https://repo.maven.apache.org/maven2/com/google/protobuf/protoc/3.20.1/'


def main():
    receipt = ROOT / 'verification/agones/first-generation-protoc-pin.json'
    destination = Path('/home/astral/.local/share/agones-rs/tools/protoc-3.20.1-maven')
    destination.mkdir(exist_ok=True)
    binary = destination / 'protoc'
    if receipt.exists():
        record = json.loads(receipt.read_text())
        assert hashlib.sha256(binary.read_bytes()).hexdigest() == record['binary_sha256']
        print('Reused pinned protoc 3.20.1')
        return
    assert not binary.exists(), 'preserve unowned compiler binary'
    name = 'protoc-3.20.1-linux-x86_64.exe'
    with urllib.request.urlopen(BASE + name + '.sha1', timeout=30) as stream:
        expected_sha1 = stream.read().decode().strip().split()[0]
    with urllib.request.urlopen(BASE + name, timeout=30) as stream:
        data = stream.read()
    assert hashlib.sha1(data).hexdigest() == expected_sha1
    binary.write_bytes(data)
    binary.chmod(0o755)
    version = subprocess.check_output([str(binary), '--version'], text=True).strip()
    assert version == 'libprotoc 3.20.1'
    with urllib.request.urlopen(BASE + 'protoc-3.20.1.pom', timeout=30) as stream:
        pom = stream.read()
    (destination / 'protoc-3.20.1.pom').write_bytes(pom)
    record = dict(schema_version=1, version='3.20.1', binary=str(binary), binary_sha256=hashlib.sha256(data).hexdigest(),
                  upstream_sha1=expected_sha1, artifact_url=BASE + name, version_output=version,
                  pom_sha256=hashlib.sha256(pom).hexdigest(), pom_path=str(destination / 'protoc-3.20.1.pom'),
                  source_build_declaration='third_party/first-generation/cpg/proto-bindings/build.sbt',
                  source_build_declaration_sha256=hashlib.sha256((ROOT / 'third_party/first-generation/cpg/proto-bindings/build.sbt').read_bytes()).hexdigest(),
                  purpose='Exact original CPG comparison compiler; native Rust binding generation uses the pinned pure Rust generator',
                  script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
    receipt.write_text(json.dumps(record, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'version':version,'binary_sha256':record['binary_sha256']}))


if __name__ == '__main__':
    main()
