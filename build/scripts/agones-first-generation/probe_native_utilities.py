#!/usr/bin/env python3
"""Generate bounded, original-JDK comparison inputs for the native UTF-8 port."""
import hashlib
import itertools
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
JDK = Path('/home/astral/.local/share/agones-rs/tools/temurin-jdk-21.0.13-9-install/jdk-21.0.13+9')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    inputs = ROOT / 'verification/agones/first-generation-utf8-inputs.bin'
    expected = ROOT / 'verification/agones/first-generation-utf8-reference.json'
    source = Path(__file__).with_name('UtilityUtf8Reference.java')
    data = bytearray(b'\0')
    cases = 1

    def add(value):
        nonlocal cases
        data.append(len(value))
        data.extend(value)
        cases += 1

    for first in range(256):
        add(bytes([first]))
        for second in range(256):
            add(bytes([first, second]))
    boundaries = [0, 0x41, 0x7f, 0x80, 0x8f, 0x90, 0x9f, 0xa0, 0xbf, 0xc0, 0xff]
    for first in range(0xe0, 0xf5):
        for trailing in itertools.product(boundaries, repeat=2):
            add(bytes([first, *trailing]))
        if first >= 0xf0:
            for trailing in itertools.product(boundaries, repeat=3):
                add(bytes([first, *trailing]))
    for text in ['', '\ufeff', '\ufeff\ufeffa\r\n\u2028b', '𐀀\U0010ffff', 'a\0é\u007f']:
        add(text.encode('utf-8'))
    prior = json.loads(expected.read_text()) if expected.exists() else None
    if inputs.exists():
        assert prior is not None and sha(inputs) == prior['input_sha256'], 'preserve edited UTF-8 inputs'
    inputs.write_bytes(data)
    destination = ROOT / '.git/agones-first-generation-utility-probe'
    destination.mkdir(exist_ok=True)
    javac = [str(JDK / 'bin/javac'), '-d', str(destination), str(source)]
    java = [str(JDK / 'bin/java'), '-cp', str(destination), 'UtilityUtf8Reference', str(inputs)]
    subprocess.run(javac, check=True)
    result = subprocess.run(java, check=True, capture_output=True, text=True)
    reference = json.loads(result.stdout)
    assert reference['cases'] == cases
    reference.update(schema_version=1, purpose='Focused native CPG utility comparison; no generated Agones output',
                     input_file=str(inputs.relative_to(ROOT)), input_sha256=sha(inputs),
                     probe_source=str(source.relative_to(ROOT)), probe_source_sha256=sha(source),
                     script_sha256=sha(Path(__file__)), javac_command=javac, java_command=java,
                     java_binary_sha256=sha(JDK / 'bin/java'), jdk_release_sha256=sha(JDK / 'release'))
    expected.write_text(json.dumps(reference, indent=2, sort_keys=True) + '\n')
    print(json.dumps({k: reference[k] for k in ['cases', 'input_sha256', 'utf16_output_sha256']}))


if __name__ == '__main__':
    main()
