#!/usr/bin/env python3
"""Compile only pinned CPG tooling to retain an independent graph observation."""
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
    receipt = ROOT / 'verification/agones/first-generation-graph-reference-run.json'
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
        assert target.read_bytes() == source.read_bytes(), 'preserve differing observation input'
    target.write_bytes(source.read_bytes())
    output = ROOT / 'verification/agones/first-generation-graph-reference.json'
    assert not output.exists(), 'preserve existing reference output'
    command += ['set domainClasses/libraryDependencies += "com.lihaoyi" %% "ujson" % "4.4.3"',
                f'domainClasses/Compile/runMain GraphReference {output}', 'show domainClasses/Compile/fullClasspath']
    log = ROOT / '.git/agones-first-generation-graph-reference.log'
    record = dict(schema_version=1, status='INTENT_RECORDED', command=command, working_directory=str(cache),
                  source_commit='301c4010d28ecdac05bc2ed44310f56446459465', flatgraph_version='0.1.36',
                  source_file=str(source.relative_to(ROOT)), source_sha256=sha(source),
                  script_sha256=sha(Path(__file__)), log=str(log.relative_to(ROOT)),
                  output=str(output.relative_to(ROOT)), environment=original['environment'],
                  purpose='Original CPG graph comparison only; no generated Agones output',
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
