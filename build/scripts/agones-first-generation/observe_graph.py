#!/usr/bin/env python3
"""Compile only pinned CPG tooling to retain an independent graph observation."""
from datetime import datetime, timezone
import argparse
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
    args = parser.parse_args()
    assert args.attempt > 0
    suffix = '' if args.attempt == 1 else f'-{args.attempt}'
    receipt = ROOT / f'verification/agones/first-generation-graph-reference-run{suffix}.json'
    assert not receipt.exists(), 'recover the existing observation before retrying'
    original = json.loads((ROOT / 'verification/agones/first-generation-cpg-regeneration.json').read_text())
    command = original['command'][:original['command'].index('-jar') + 2]
    environment = dict(os.environ, **original['environment'])
    for key in ('JAVA_TOOL_OPTIONS', 'JDK_JAVA_OPTIONS', '_JAVA_OPTIONS'):
        environment.pop(key, None)
    source = Path(__file__).with_name('GraphReference.scala')
    cache = ROOT / '.git/agones-first-generation-sources/cpg'
    target = cache / 'domainClasses/src/main/scala/GraphReference.scala'
    target.parent.mkdir(parents=True, exist_ok=True)
    if target.exists():
        if target.read_bytes() != source.read_bytes():
            previous = [json.loads(path.read_text()) for path in (ROOT / 'verification/agones').glob('first-generation-graph-reference-run*.json')]
            assert any(row.get('source_sha256') == sha(target) for row in previous), 'preserve unowned differing observation input'
            retained = ROOT / '.git' / f'graph-reference-source-{sha(target)}.scala'
            retained.write_bytes(target.read_bytes())
    target.write_bytes(source.read_bytes())
    output = ROOT / 'verification/agones/first-generation-graph-reference.json'
    assert not output.exists(), 'preserve existing reference output'
    command += ['set schema/libraryDependencies += "com.lihaoyi" %% "ujson" % "4.4.3"',
                'set domainClasses/libraryDependencies += "com.lihaoyi" %% "ujson" % "4.4.3"',
                f'domainClasses/Compile/runMain GraphReference {output}', 'show domainClasses/Compile/fullClasspath']
    log = ROOT / f'.git/agones-first-generation-graph-reference{suffix}.log'
    record = dict(schema_version=1, status='INTENT_RECORDED', command=command, working_directory=str(cache),
                  source_commit='301c4010d28ecdac05bc2ed44310f56446459465', flatgraph_version='0.1.36',
                  source_file=str(source.relative_to(ROOT)), source_sha256=sha(source),
                  script_sha256=sha(Path(__file__)), log=str(log.relative_to(ROOT)),
                  output=str(output.relative_to(ROOT)), environment=original['environment'],
                  purpose='Original CPG graph comparison only; no generated Agones output',
                  source_vendor_manifests={f'verification/agones/first-generation-vendor-{name}.json':
                      sha(ROOT / f'verification/agones/first-generation-vendor-{name}.json') for name in ('cpg', 'flatgraph')},
                  build_only_source_overlays={str(path.relative_to(cache)): sha(path) for path in sorted(cache.rglob('*.scala'))
                      if '/target/' not in str(path) and not (ROOT / 'third_party/first-generation/cpg' / path.relative_to(cache)).exists()},
                  started_at=datetime.now(timezone.utc).isoformat())

    def save():
        receipt.write_text(json.dumps(record, indent=2, sort_keys=True) + '\n')

    save()
    try:
        with log.open('wb') as stream:
            process = subprocess.Popen(command, cwd=cache, env=environment, stdout=stream, stderr=subprocess.STDOUT)
            record.update(status='RUNNING', pid=process.pid, process_identity=Path(f'/proc/{process.pid}/stat').read_text())
            save()
            record['exit_code'] = process.wait()
        record['status'] = 'OBSERVED' if record['exit_code'] == 0 and output.exists() else 'FAILED'
        if output.exists():
            record['output_sha256'] = sha(output)
        return record['exit_code']
    finally:
        record.update(finished_at=datetime.now(timezone.utc).isoformat(), log_sha256=sha(log))
        save()
        print(json.dumps({k: record.get(k) for k in ('status', 'exit_code', 'output_sha256')}))


if __name__ == '__main__':
    raise SystemExit(main())
