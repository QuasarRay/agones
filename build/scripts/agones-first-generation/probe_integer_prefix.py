#!/usr/bin/env python3
"""Retain pinned JDK decimal character data and all BMP-code-unit parse outcomes."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
JDK = Path('/home/astral/.local/share/agones-rs/tools/temurin-jdk-21.0.13-9-install/jdk-21.0.13+9')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    source = Path(__file__).with_name('IntegerPrefixReference.java')
    classes = ROOT / '.git/agones-first-generation-utility-probe'
    classes.mkdir(exist_ok=True)
    subprocess.run([str(JDK / 'bin/javac'), '-d', str(classes), str(source)], check=True)
    result = subprocess.run([str(JDK / 'bin/java'), '-cp', str(classes), 'IntegerPrefixReference'], check=True, capture_output=True, text=True)
    value = json.loads(result.stdout)
    target = ROOT / 'cpg-rs/templates/java-decimal-zeroes.json'
    receipt = ROOT / 'verification/agones/first-generation-integer-prefix-reference.json'
    if target.exists():
        assert receipt.exists() and sha(target) == json.loads(receipt.read_text())['table_sha256']
    target.write_text(json.dumps(value['decimalZeroes'], indent=2) + '\n')
    value.update(table=str(target.relative_to(ROOT)), table_sha256=sha(target), probe_source_sha256=sha(source),
                 script_sha256=sha(Path(__file__)), java_sha256=sha(JDK / 'bin/java'),
                 scala_to_int_delegation='java.lang.Integer.parseInt',
                 delegation_bytecode='.git/agones-first-generation-string-ops-bytecode.txt',
                 delegation_bytecode_sha256=sha(ROOT / '.git/agones-first-generation-string-ops-bytecode.txt'))
    receipt.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')
    print(json.dumps({key: value[key] for key in ('utf16Cases', 'outputSha256')}))


if __name__ == '__main__':
    main()
