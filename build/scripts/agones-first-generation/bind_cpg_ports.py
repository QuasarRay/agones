#!/usr/bin/env python3
"""Bind reviewed handwritten ports and generated ProtoIds to the retained source census.

This indexes concrete implementations. It does not infer whole-port completion from
passing tests, an operation trace, or a whole-file delegation label.
"""
import hashlib
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
EVIDENCE = ROOT / 'verification/agones'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--controls',default='verification/agones/first-generation-cpg-runtime-controls.json')
    args=parser.parse_args()
    census_file = EVIDENCE / 'first-generation-cpg-body-inventory.json'
    census_bytes = census_file.read_bytes()
    census = json.loads(census_bytes)
    bodies = {body['id']: (file, body) for file in census['files'] for body in file['bodies']}
    mapping_file = EVIDENCE / 'first-generation-cpg-port-map.json'
    prior = json.loads(mapping_file.read_text())
    assert prior['source_census_sha256'] == sha(census_bytes)
    entries = {row['source_body_id']: row for row in prior['entries'] if row['status'] == 'IMPLEMENTED_LITERAL_OR_DEFAULT_BEHAVIOR'}
    construction_file = ROOT / 'cpg-rs/generated/construction.json'
    construction = json.loads(construction_file.read_text())
    constructors = (ROOT / 'cpg-rs/generated/schema_constructors.rs').read_bytes()
    assert sha(constructors) == construction['outputSha256']
    for row in construction['mappings']:
        if 'sourceBodyId' not in row:
            continue
        assert row['sourceBodyId'] not in entries
        file, body = bodies[row['sourceBodyId']]
        assert row['sourceBodySha256'] == body['bodySha256']
        assert row['sourceBlob'] == file['blob'] and row['sourcePath'] == file['path']
        start, end = row['startByte'], row['endByteExclusive']
        assert sha(constructors[start:end]) == row['sha256']
        entries[body['id']] = dict(source_body_id=body['id'], source_body_sha256=body['bodySha256'],
            source_path=file['path'], source_blob=file['blob'], source_symbol=body['qualifiedName'], source_span=body['bodySpan'],
            rust_file='cpg-rs/generated/schema_constructors.rs', rust_symbol='cpg_rs::cpg_schema::' + row['rustSymbol'],
            rust_start_byte=start, rust_end_byte_exclusive=end, rust_content_sha256=row['sha256'],
            status='IMPLEMENTED_SOURCE_DERIVED_PROTO_ID_LITERAL',
            assurance='Exact source numeric literal, generated symbol/range identity and focused controls; no formal-refinement credit.')
    bindings_file = ROOT / 'cpg-rs/port-bindings.json'
    bindings = json.loads(bindings_file.read_text())
    assert bindings['source_census_sha256'] == sha(census_bytes)
    for binding in bindings['entries']:
        file, body = bodies[binding['source_body_id']]
        assert binding['source_body_sha256'] == body['bodySha256']
        data = (ROOT / binding['rust_file']).read_bytes()
        begin = binding['rust_begin'].encode()
        end_marker = binding['rust_end'].encode() if binding['rust_end'] else None
        assert data.count(begin) == 1, (binding['rust_symbol'], 'ambiguous range start')
        start = data.index(begin)
        if end_marker:
            assert data.count(end_marker) == 1, (binding['rust_symbol'], 'ambiguous range end')
            end = data.index(end_marker, start + len(begin))
        else:
            end = len(data)
        assert end > start
        assert body['id'] not in entries
        entries[body['id']] = dict(source_body_id=body['id'], source_body_sha256=body['bodySha256'],
            source_path=file['path'], source_blob=file['blob'], source_symbol=body['qualifiedName'], source_span=body['bodySpan'],
            rust_file=binding['rust_file'], rust_symbol=binding['rust_symbol'], rust_start_byte=start,
            rust_end_byte_exclusive=end, rust_content_sha256=sha(data[start:end]), status='IMPLEMENTED_NATIVE_BEHAVIOR',
            behavior=binding['behavior'], assurance='Reviewed native implementation plus focused runtime controls; no formal-refinement credit.')
    for identifier, row in entries.items():
        file, body = bodies[identifier]
        original = (ROOT / 'third_party/first-generation/cpg' / file['path']).read_bytes()
        span = body['bodySpan']
        assert sha(original) == file['sha256']
        assert sha(original[span['startByte']:span['endByteExclusive']]) == row['source_body_sha256']
        rust = (ROOT / row['rust_file']).read_bytes()
        assert sha(rust[row['rust_start_byte']:row['rust_end_byte_exclusive']]) == row['rust_content_sha256']
    controls_file = ROOT / args.controls
    controls = json.loads(controls_file.read_text())
    assert controls['status'] == 'PASSED'
    for path, digest in controls['source_inputs'].items():
        assert sha((ROOT / path).read_bytes()) == digest, path
    for row in controls['steps']:
        assert row['exit_code'] == 0 and sha((ROOT / row['log']).read_bytes()) == row['log_sha256']
    prior.update(entries=sorted(entries.values(), key=lambda row: (row['source_path'], row['source_span']['startByte'], row['source_body_id'])),
        remaining_candidate_ids=sorted(set(bodies) - set(entries)),
        rust_controls_sha256=sha(controls_file.read_bytes()),
        construction_receipt_sha256=sha(construction_file.read_bytes()),
        reviewed_bindings_sha256=sha(bindings_file.read_bytes()),
        mapping_script_sha256=sha(Path(__file__).read_bytes()), status='PARTIAL_PORT_REMAINS_INCOMPLETE',
        whole_cpg_port_complete=False,
        counts=dict(candidate_bodies=len(bodies), mapped_bodies=len(entries), remaining_candidate_bodies=len(bodies) - len(entries)))
    mapping_file.write_text(json.dumps(prior, indent=2, sort_keys=True) + '\n')
    print(json.dumps(prior['counts']))


if __name__ == '__main__':
    main()
