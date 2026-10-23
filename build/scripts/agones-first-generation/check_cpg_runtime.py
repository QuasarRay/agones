#!/usr/bin/env python3
"""One source-bound focused check of CPG tooling, never generated Agones."""
from datetime import datetime, timezone
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[3]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--attempt',type=int,default=1)
    args=parser.parse_args()
    assert args.attempt>0
    suffix='' if args.attempt==1 else f'-{args.attempt}'
    receipt = ROOT / f'verification/agones/first-generation-cpg-runtime-controls{suffix}.json'
    assert not receipt.exists(), 'recover and retain existing checks before launching another job'
    inputs = sorted(path for path in (ROOT / 'cpg-rs').rglob('*') if path.is_file() and 'generated' not in path.parts)
    inputs += [ROOT / 'verification/agones' / name for name in (
        'first-generation-graph-reference.json', 'first-generation-graph-reference-run-3.json',
        'first-generation-utf8-inputs.bin', 'first-generation-utf8-reference.json',
        'first-generation-cpg-reference.proto', 'first-generation-schema-construction-trace.json',
        'first-generation-complete-schema-bootstrap.json', 'first-generation-cpg-body-inventory.json')]
    inputs += [ROOT/'verification/agones'/name for name in ('first-generation-proto-reference.json',
        'first-generation-proto-reference-run-2.json','first-generation-integer-prefix-reference.json',
        'first-generation-protoc-pin.json','first-generation-protoc-descriptor-reference.json',
        'first-generation-flatgraph-reference.json','first-generation-flatgraph-reference-run-1.json',
        'first-generation-grafeo-pin.json','first-generation-rust-toolchain.json')]
    inputs += [ROOT/'verification/agones'/name for name in (
        'first-generation-legacy-storage-pins.json','first-generation-legacy-reference.json',
        'first-generation-legacy-reference-run-1.json','first-generation-legacy-failures.json',
        'first-generation-legacy-failures-run-1.json','first-generation-legacy-modes.json',
        'first-generation-legacy-modes-run-1.json','first-generation-legacy-metadata.json',
        'first-generation-legacy-metadata-run-1.json')]
    inputs += [path for pattern in ('first-generation-*.pb','first-generation-*.zip','first-generation-*.fg') for path in (ROOT/'verification/agones').glob(pattern)]
    inputs += [path for path in (ROOT/'third_party/first-generation/grafeo').rglob('*') if path.is_file()]
    inputs += list((ROOT/'verification/agones').glob('first-generation-legacy-*.odb'))
    inputs += [path for name in ('h2-mvstore','overflowdb-core') for path in (ROOT/'third_party/first-generation'/name).rglob('*') if path.is_file()]
    inputs=sorted(set(inputs))
    expected_tests={str(path.relative_to(ROOT)):len(re.findall(r'(?m)^#\[test\]$',path.read_text())) for path in sorted((ROOT/'cpg-rs/tests').glob('*.rs'))}
    record = dict(schema_version=1, status='INTENT_RECORDED', purpose='Native CPG support checks before Agones emission',
        source_inputs={str(path.relative_to(ROOT)): sha(path) for path in inputs},
        script_sha256=sha(Path(__file__)), expected_test_sources=expected_tests,started_at=datetime.now(timezone.utc).isoformat(), steps=[])

    def save():
        receipt.write_text(json.dumps(record, indent=2, sort_keys=True) + '\n')

    save()
    assert not (ROOT / 'generated/agones-first-generation/evidence/FIRST_FULL_GENERATION_COMPLETE.json').exists()
    base = ['cargo', 'run', '--manifest-path', 'cpg-rs/Cargo.toml', '--locked', '--offline', '--no-default-features', '--bin']
    commands = [base + ['cpg-domain', '--', 'verification/agones/first-generation-complete-schema-bootstrap.json', 'cpg-rs/generated'],
                base + ['cpg-schema-constructors', '--', 'verification/agones/first-generation-schema-construction-trace.json', 'cpg-rs/generated'],
                ['cargo','run','--manifest-path','cpg-rs/Cargo.toml','--locked','--offline','--no-default-features','--features','native-schema',
                 '--bin','cpg-protobuf-bindings','--','cpg-rs/generated'],
                ['cargo', 'test', '--manifest-path', 'cpg-rs/Cargo.toml', '--locked', '--offline', '--all-features']]
    environment = dict(os.environ, CARGO_TARGET_DIR='/home/astral/.cache/agones-first-generation/cpg-rs-target')
    try:
        for index, command in enumerate(commands):
            log = ROOT / f'.git/agones-first-generation-cpg-runtime-control{suffix}-{index}.log'
            if index==len(commands)-1:
                record['generated_inputs']={str(path.relative_to(ROOT)):sha(path) for path in sorted((ROOT/'cpg-rs/generated').iterdir()) if path.is_file()}
            step = dict(command=command, status='RUNNING', log=str(log.relative_to(ROOT)))
            record['steps'].append(step)
            with log.open('wb') as stream:
                process = subprocess.Popen(command, cwd=ROOT, env=environment, stdout=stream, stderr=subprocess.STDOUT)
                step.update(pid=process.pid, process_identity=Path(f'/proc/{process.pid}/stat').read_text())
                record['status'] = 'RUNNING'
                save()
                code = process.wait()
            step.update(exit_code=code, log_sha256=sha(log), status='PASSED' if code == 0 else 'FAILED')
            save()
            if code != 0:
                record['status'] = 'FAILED'
                return code
        tests = (ROOT / record['steps'][-1]['log']).read_text()
        results = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', tests)
        expected=sum(expected_tests.values())
        assert results and sum(int(passed) for passed, _, _ in results) == expected
        assert all(failed == ignored == '0' for _, failed, ignored in results)
        for path, digest in record['source_inputs'].items():
            assert sha(ROOT / path) == digest, f'input changed during checks: {path}'
        for path,digest in record['generated_inputs'].items():
            assert sha(ROOT/path)==digest,f'generated input changed during checks: {path}'
        record.update(status='PASSED', focused_tests=expected, generated_receipts={name: sha(ROOT / 'cpg-rs/generated' / name)
            for name in ('generation.json', 'construction.json','protobuf.json')})
        return 0
    except Exception as error:
        record.update(status='FAILED', error=f'{type(error).__name__}: {error}')
        raise
    finally:
        record['finished_at'] = datetime.now(timezone.utc).isoformat()
        save()
        print(json.dumps({key: record.get(key) for key in ('status', 'focused_tests', 'error')}))


if __name__ == '__main__':
    raise SystemExit(main())
