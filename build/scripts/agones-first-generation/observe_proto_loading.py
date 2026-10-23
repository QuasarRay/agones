#!/usr/bin/env python3
"""Run the original pinned CPG implementation for focused loader/archive observations."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import warnings
import zipfile

ROOT = Path(__file__).resolve().parents[3]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--attempt', type=int, default=1)
    args = parser.parse_args()
    suffix = '' if args.attempt == 1 else f'-{args.attempt}'
    evidence = ROOT / 'verification/agones'
    receipt = evidence / f'first-generation-proto-reference-run{suffix}.json'
    assert not receipt.exists(), 'recover and preserve existing observation attempt'
    source = Path(__file__).with_name('ProtoLoadingReference.scala')
    cache = ROOT / '.git/agones-first-generation-sources/cpg'
    target = cache / 'codepropertygraph/src/main/scala/ProtoLoadingReference.scala'
    if target.exists() and target.read_bytes() != source.read_bytes():
        previous = [json.loads(path.read_text()) for path in evidence.glob('first-generation-proto-reference-run*.json')]
        assert any(row['source_sha256'] == sha(target) for row in previous), 'preserve unowned observation source'
        (ROOT / '.git' / f'proto-loading-source-{sha(target)}.scala').write_bytes(target.read_bytes())
    target.write_bytes(source.read_bytes())
    pin = json.loads((evidence / 'first-generation-protoc-pin.json').read_text())
    assert sha(Path(pin['binary'])) == pin['binary_sha256']
    installed = cache / 'proto-bindings/protoc/bin/protoc'
    installed.parent.mkdir(parents=True, exist_ok=True)
    if installed.exists():
        assert sha(installed) == pin['binary_sha256']
    else:
        shutil.copyfile(pin['binary'], installed)
        installed.chmod(0o755)
    fixture = evidence / 'first-generation-zip-reference-input.zip'
    if not fixture.exists():
        with warnings.catch_warnings():
            warnings.simplefilter('ignore', UserWarning)
            with zipfile.ZipFile(fixture, 'x') as archive:
                for name, data in [('top.txt', b'top'), ('nested/old.bin', b'old'), ('nested/', b''),
                                   ('nested/deeper/item', b'deep'), ('zeta', b'z'), ('nested/new.bin', b'new'),
                                   ('top.txt', b'overwritten'), ('back\\slash', b'backslash'), ('/leading.txt', b'leading'),
                                   ('repeated//slash.txt', b'normalized')]:
                    archive.writestr(name, data, compress_type=zipfile.ZIP_DEFLATED)
    original = json.loads((evidence / 'first-generation-graph-reference-run-3.json').read_text())
    command = original['command'][:original['command'].index('-jar') + 2]
    output = evidence / 'first-generation-proto-reference.json'
    assert not output.exists(), 'preserve existing reference output'
    command += ['set schema/libraryDependencies += "com.lihaoyi" %% "ujson" % "4.4.3"',
                'set codepropertygraph/libraryDependencies += "com.lihaoyi" %% "ujson" % "4.4.3"',
                f'codepropertygraph/Compile/runMain ProtoLoadingReference {output}', 'show codepropertygraph/Compile/fullClasspath']
    environment = dict(os.environ, **original['environment'])
    for key in ('JAVA_TOOL_OPTIONS', 'JDK_JAVA_OPTIONS', '_JAVA_OPTIONS'):
        environment.pop(key, None)
    log = ROOT / f'.git/agones-first-generation-proto-reference{suffix}.log'
    record = dict(schema_version=1, status='INTENT_RECORDED', source_commit=original['source_commit'],
                  source_file=str(source.relative_to(ROOT)), source_sha256=sha(source), script_sha256=sha(Path(__file__)),
                  command=command, working_directory=str(cache), environment=original['environment'],
                  protoc_pin_sha256=sha(evidence / 'first-generation-protoc-pin.json'),
                  fixture_sha256=sha(fixture), log=str(log.relative_to(ROOT)), output=str(output.relative_to(ROOT)),
                  purpose='Original CPG support comparison; no generated Agones code', started_at=datetime.now(timezone.utc).isoformat())

    def save():
        receipt.write_text(json.dumps(record, indent=2, sort_keys=True) + '\n')

    save()
    try:
        with log.open('wb') as stream:
            process = subprocess.Popen(command, cwd=cache, env=environment, stdout=stream, stderr=subprocess.STDOUT)
            record.update(status='RUNNING', pid=process.pid, process_identity=Path(f'/proc/{process.pid}/stat').read_text())
            save()
            code = process.wait()
        record.update(exit_code=code, status='OBSERVED' if code == 0 and output.exists() else 'FAILED')
        if code == 0:
            record['outputs'] = {str(path.relative_to(ROOT)): sha(path) for path in sorted(evidence.glob('first-generation-proto-reference*')) if path != receipt and path.suffix in ('.json', '.pb')}
            record['outputs']['verification/agones/first-generation-serialized-reference.zip'] = sha(evidence / 'first-generation-serialized-reference.zip')
        return code
    finally:
        record.update(finished_at=datetime.now(timezone.utc).isoformat(), log_sha256=sha(log))
        save()
        print(json.dumps({key: record.get(key) for key in ('status', 'exit_code')}))


if __name__ == '__main__':
    raise SystemExit(main())
