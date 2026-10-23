#!/usr/bin/env python3
"""Pinned original storage generation/inspection for native CPG support comparisons."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--attempt', type=int, default=1)
    parser.add_argument('--mode', choices=['generate', 'inspect', 'inspect-free'], default='generate')
    parser.add_argument('--input')
    parser.add_argument('--name', default='first-generation-flatgraph-reference')
    args = parser.parse_args()
    assert '/' not in args.name and '..' not in args.name
    evidence = ROOT / 'verification/agones'
    receipt = evidence / f'{args.name}-run-{args.attempt}.json'
    output = evidence / f'{args.name}.json'
    assert not receipt.exists() and not output.exists(), 'recover existing comparison artifacts'
    source = Path(__file__).with_name('StorageReference.scala')
    cache = ROOT / '.git/agones-first-generation-sources/cpg'
    target = cache / 'codepropertygraph/src/main/scala/StorageReference.scala'
    if target.exists() and target.read_bytes() != source.read_bytes():
        previous = [json.loads(path.read_text()) for path in evidence.glob('*flatgraph*-run-*.json')]
        assert any(row.get('source_sha256') == sha(target) for row in previous)
        (ROOT / '.git' / f'storage-reference-source-{sha(target)}.scala').write_bytes(target.read_bytes())
    target.write_bytes(source.read_bytes())
    previous = json.loads((evidence / 'first-generation-proto-reference-run-2.json').read_text())
    command = previous['command'][:previous['command'].index('-jar') + 2]
    environment = dict(os.environ, **previous['environment'])
    for key in ('JAVA_TOOL_OPTIONS', 'JDK_JAVA_OPTIONS', '_JAVA_OPTIONS'):
        environment.pop(key, None)
    main_args = f'{args.mode} {output}'
    if args.input:
        main_args += ' ' + str((ROOT / args.input).resolve())
    command += ['set schema/libraryDependencies += "com.lihaoyi" %% "ujson" % "4.4.3"',
                'set codepropertygraph/libraryDependencies += "com.lihaoyi" %% "ujson" % "4.4.3"',
                f'codepropertygraph/Compile/runMain StorageReference {main_args}']
    log = ROOT / f'.git/{args.name}-{args.attempt}.log'
    record = dict(schema_version=1, status='INTENT_RECORDED', mode=args.mode, source_file=str(source.relative_to(ROOT)),
                  source_sha256=sha(source), script_sha256=sha(Path(__file__)), command=command,
                  environment=previous['environment'], working_directory=str(cache), log=str(log.relative_to(ROOT)),
                  output=str(output.relative_to(ROOT)), started_at=datetime.now(timezone.utc).isoformat(),
                  purpose='Original storage comparison only; no generated Agones code')
    if args.input:
        record['input'] = args.input
        record['input_sha256'] = sha(ROOT / args.input)

    def save():
        receipt.write_text(json.dumps(record, indent=2, sort_keys=True) + '\n')

    save()
    try:
        with log.open('wb') as stream:
            process = subprocess.Popen(command, cwd=cache, env=environment, stdout=stream, stderr=subprocess.STDOUT)
            record.update(status='RUNNING', pid=process.pid, process_identity=Path(f'/proc/{process.pid}/stat').read_text())
            save()
            code = process.wait()
        record.update(status='OBSERVED' if code == 0 and output.exists() else 'FAILED', exit_code=code)
        if output.exists():
            record['output_sha256'] = sha(output)
        if code == 0 and args.mode == 'generate':
            record['binary_outputs'] = {str(path.relative_to(ROOT)): sha(path) for path in evidence.glob('first-generation-flatgraph-*.fg')}
        return code
    finally:
        record.update(finished_at=datetime.now(timezone.utc).isoformat(), log_sha256=sha(log))
        save()
        print(json.dumps({key: record.get(key) for key in ('status', 'exit_code', 'output_sha256')}))


if __name__ == '__main__':
    raise SystemExit(main())
