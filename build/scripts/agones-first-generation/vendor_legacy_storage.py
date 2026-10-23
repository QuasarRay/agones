#!/usr/bin/env python3
"""Retain pinned published source for the legacy CPG storage import dependency.

This records Maven source artifacts separately from the already pinned CPG and
Flatgraph Git trees. It does not claim a full H2/OverflowDB repository checkout.
"""
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
from zipfile import ZipFile

ROOT=Path(__file__).resolve().parents[3]

def sha(data):return hashlib.sha256(data).hexdigest()

def main():
    cache=ROOT/'.git/agones-first-generation-sources/legacy-storage'
    acquisition=json.loads((cache/'acquisition.json').read_text())
    lock=json.loads((ROOT/'verification/agones/first-generation-proto-reference-maven-lock.json').read_text())
    pins=[]
    for item in acquisition:
        name=item['name'];directory=ROOT/'third_party/first-generation'/('h2-mvstore' if name=='h2-mvstore' else 'overflowdb-core')
        archive=ROOT/item['path'];assert sha(archive.read_bytes())==item['sha256']
        assert not directory.exists(),'recover an existing source extraction before retry'
        directory.mkdir();files={}
        with ZipFile(archive) as source:
            for entry in source.infolist():
                if entry.is_dir():continue
                path=Path(entry.filename)
                assert not path.is_absolute() and '..' not in path.parts
                data=source.read(entry);target=directory/path;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(data)
                files[str(path)]={'sha256':sha(data),'bytes':len(data)}
        relative=item['url'].removeprefix('https://repo.maven.apache.org/maven2/').replace('-sources.jar','.pom')
        pom=ROOT/'.git/agones-first-generation-cpg-build/coursier/https/repo.maven.apache.org/maven2'/relative
        (directory/'upstream-source.pom').write_bytes(pom.read_bytes())
        guidance=[]
        for parent in [directory]+sorted(p for p in directory.rglob('*') if p.is_dir()):
            path=parent/'AGENTS.MD'
            if not path.exists():
                text='# AGENTS.MD\n\nProject guidance is inherited from [the repository root]('+os.path.relpath(ROOT/'AGENTS.MD',parent)+').\nThis guidance overlay is not upstream Maven source.\n'
                path.write_text(text);guidance.append({'path':str(path.relative_to(directory)),'sha256':sha(text.encode())})
        pins.append(dict(item,destination=str(directory.relative_to(ROOT)),files=files,source_file_count=len(files),
            pom_sha256=sha(pom.read_bytes()),guidance_overlays=guidance,
            license='MPL-2.0 OR EPL-1.0' if name=='h2-mvstore' else 'Apache-2.0',
            license_provenance='Retained Java source copyright/license headers and original published Maven POM license declarations'))
    receipt=ROOT/'verification/agones/first-generation-legacy-storage-pins.json';assert not receipt.exists()
    receipt.write_text(json.dumps({'schema_version':1,'status':'VENDORED_PUBLISHED_SOURCE','purpose':'Native read-only legacy OverflowDB/H2 compatibility import into Grafeo',
        'packages':pins,'selected_binary_closure':'verification/agones/first-generation-proto-reference-maven-lock.json',
        'selected_binary_closure_sha256':sha((ROOT/'verification/agones/first-generation-proto-reference-maven-lock.json').read_bytes()),
        'patches':[],'script_sha256':sha(Path(__file__).read_bytes()),'finished_at':datetime.now(timezone.utc).isoformat()},indent=2,sort_keys=True)+'\n')
    print(json.dumps({'packages':len(pins),'upstream_files':sum(p['source_file_count'] for p in pins)}))

if __name__=='__main__':main()
