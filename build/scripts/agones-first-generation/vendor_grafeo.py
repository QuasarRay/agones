#!/usr/bin/env python3
"""Vendor the exact published Grafeo library packages used by cpg-rs.

Crates.io archives bind Cargo checksums and .cargo_vcs_info.json to the pinned
release commit. Packaging-generated manifests remain explicit; this is not a
claim that the full Git repository, bindings, benchmarks, or examples were copied.
"""
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import tarfile
import tomllib
import subprocess
import urllib.request

ROOT=Path(__file__).resolve().parents[3]
COMMIT='2150b0fe171834bc618ffb8be18864ade7e0fd17'
NAMES=('grafeo','grafeo-common','grafeo-core','grafeo-adapters','grafeo-storage','grafeo-engine')

def sha(data):
    return hashlib.sha256(data).hexdigest()

def main():
    destination=ROOT/'third_party/first-generation/grafeo'
    receipt=ROOT/'verification/agones/first-generation-grafeo-pin.json'
    if receipt.exists():
        record=json.loads(receipt.read_text())
        assert record['status']=='VENDORED_PUBLISHED_LIBRARY_SOURCE'
        for package in record['packages']:
            for name,item in package['files'].items():
                assert sha((destination/package['name']/name).read_bytes())==item['sha256'],name
        for name,item in record['upstream_root_files'].items():
            assert sha((destination/name).read_bytes())==item['sha256'],name
        print('Reused verified Grafeo library source packages')
        return
    assert not destination.exists(),'recover existing Grafeo source before retry'
    cargo_cache=Path(os.environ.get('CARGO_HOME',str(Path.home()/'.cargo')))/'registry/cache'
    lock=tomllib.loads((ROOT/'cpg-rs/Cargo.lock').read_text())
    packages=[]
    staging=ROOT/'.git/agones-first-generation-sources/grafeo-library-staging'
    staging.mkdir(exist_ok=True)
    for name in NAMES:
        [package]=[p for p in lock['package'] if p['name']==name]
        assert package['version']=='0.5.44'
        [archive]=list(cargo_cache.glob('*/'+name+'-0.5.44.crate'))
        assert sha(archive.read_bytes())==package['checksum']
        files={}
        target=staging/name
        with tarfile.open(archive,'r:gz') as tar:
            for member in tar:
                if member.isdir():continue
                path=Path(member.name)
                assert path.parts[0]==name+'-0.5.44' and not path.is_absolute() and '..' not in path.parts
                assert member.isfile(),'unexpected source package symlink or special file'
                relative=Path(*path.parts[1:]);data=tar.extractfile(member).read()
                output=target/relative;output.parent.mkdir(parents=True,exist_ok=True)
                if output.exists():assert output.read_bytes()==data,'changed retained package source'
                else:output.write_bytes(data)
                files[str(relative)]={'sha256':sha(data),'bytes':len(data)}
        vcs=json.loads((target/'.cargo_vcs_info.json').read_text())
        assert vcs['git']['sha1']==COMMIT and vcs['path_in_vcs']=='crates/'+name
        licenses=[p for p in files if Path(p).name.upper().startswith(('LICENSE','NOTICE','COPYING'))]
        manifest=tomllib.loads((target/'Cargo.toml').read_text())
        assert manifest['package']['license']=='Apache-2.0'
        packages.append({'name':name,'version':'0.5.44','checksum':package['checksum'],
            'archive':str(archive),'vcs':vcs,'license':'Apache-2.0','license_files':licenses,'files':files,
            'packaging_manifests':['Cargo.toml','Cargo.toml.orig'],'file_count':len(files),
            'license_files_omitted_from_published_package':not bool(licenses)})
    root_files={}
    for name in ('LICENSE','NOTICE'):
        data=urllib.request.urlopen('https://raw.githubusercontent.com/GrafeoDB/grafeo/'+COMMIT+'/'+name,timeout=20).read()
        row=subprocess.check_output(['git','-C',str(ROOT/'.git/agones-first-generation-sources/grafeo'),
            'ls-tree',COMMIT,'--',name],text=True).split()
        blob=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()
        assert row[2]==blob
        target=staging/name
        if target.exists():assert target.read_bytes()==data
        else:target.write_bytes(data)
        root_files[name]={'git_blob':blob,'sha256':sha(data),'bytes':len(data),'source_path':name}
    staging.rename(destination)
    guidance=[]
    for directory in [destination]+sorted(p for p in destination.rglob('*') if p.is_dir()):
        path=directory/'AGENTS.MD'
        if not path.exists():
            text='# AGENTS.MD\n\nProject guidance is inherited from [the repository root]('+os.path.relpath(ROOT/'AGENTS.MD',directory)+').\nThis guidance overlay is not upstream Grafeo package source.\n'
            path.write_text(text);guidance.append({'path':str(path.relative_to(destination)),'sha256':sha(text.encode())})
    record={'schema_version':1,'status':'VENDORED_PUBLISHED_LIBRARY_SOURCE','repository':'https://github.com/GrafeoDB/grafeo',
        'release':'v0.5.44','commit':COMMIT,'tree':'b681d4cd510c5e687b405a4719d6523b1914d87c',
        'role':'Canonical cpg-rs graph database and persistence engine',
        'features':['edge','wal','grafeo-file'],'scope':'Six complete published Rust library packages; Cargo packaging manifests are retained alongside Cargo.toml.orig. The full repository is not claimed vendored.',
        'packages':packages,'upstream_root_files':root_files,'patches':[],'guidance_overlays':guidance,
        'prior_repository_archive_attempt':'verification/agones/first-generation-vendor-grafeo.json',
        'script_sha256':sha(Path(__file__).read_bytes()),'finished_at':datetime.now(timezone.utc).isoformat()}
    receipt.write_text(json.dumps(record,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'status':record['status'],'packages':len(packages),'files':sum(p['file_count'] for p in packages)}))

if __name__=='__main__':main()
