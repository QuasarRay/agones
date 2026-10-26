#!/usr/bin/env python3
"""Author source-derived roadmap documentation. Never build or execute project code."""
from __future__ import annotations
import collections
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import quote
from milestone_content import A, C, A_PRIORITY, C_PRIORITY, C_DEPS

ROOT = Path(__file__).resolve().parents[3]
DATA = ROOT / 'verification/agones/roadmap'
ROAD = ROOT / 'roadmap'
AGONES = '6eb4a02588d8e7b581de187f3bc849733a78dcb2'
CPG = '301c4010d28ecdac05bc2ed44310f56446459465'
SCHEMA_ANCHOR = 'd02d76ea5c9e3b398317d334228d7e8e78ab45aa'
CPG_ROOT = ROOT / 'third_party/first-generation/cpg'
CPG_GIT = ROOT / 'codepropertygraph'
DATE = '2026-10-27'

# Collapse the real CpgSchema/PropertyDefaults construction cycle. Keep source
# files intact and place cohesive Agones contracts together, rather than inventing
# short compiler milestones solely to maintain alternation.
ALIASES={
 'a-protocol':'a-api', 'a-apply':'a-observation', 'a-client':'a-observation',
 'a-gameserversets':'a-gameservers', 'a-tooling':'a-examples',
 'c-schema-base':'c-schema-all','c-schema-identity':'c-schema-all',
 'c-schema-program':'c-schema-all','c-schema-flow':'c-schema-all',
 'c-schema-enrichment':'c-schema-all','c-schema-compose':'c-schema-all'}

def collapse_families():
    global A_PRIORITY,C_PRIORITY,C_DEPS
    groups={
     'a-api':('Resource and wire contracts, defaults and validation',['a-protocol','a-api']),
     'a-observation':('Apply, clients, informers and listers',['a-apply','a-client','a-observation']),
     'a-gameservers':('GameServer and GameServerSet lifecycle',['a-gameservers','a-gameserversets']),
     'a-examples':('Examples, source tooling and compatibility inputs',['a-examples','a-tooling']),
     'c-schema-all':('Complete schema construction and mutually dependent declarations',['c-schema-base','c-schema-identity','c-schema-program','c-schema-flow','c-schema-enrichment','c-schema-compose'])}
    for target,(title,keys) in groups.items():
        table=A if target.startswith('a-') else C
        actions=[action for key in keys for action in table[key][-1]]
        if target.startswith('a-'):
            source_doc=table[keys[-1]][1]
            replacement=(title,source_doc,actions)
        else:replacement=(title,actions)
        for key in keys:table.pop(key)
        table[target]=replacement
    def remap(keys):return list(dict.fromkeys(ALIASES.get(k,k) for k in keys))
    A_PRIORITY=remap(A_PRIORITY);C_PRIORITY=remap(C_PRIORITY)
    deps={}
    for key,values in C_DEPS.items():
        target=ALIASES.get(key,key)
        deps.setdefault(target,set()).update(ALIASES.get(v,v) for v in values)
    for k,v in deps.items():v.discard(k)
    deps['c-passes'].add('c-serialization')
    C_DEPS={k:sorted(v) for k,v in deps.items()}

collapse_families()

def sha(data):
    return hashlib.sha256(data if isinstance(data, bytes) else data.encode()).hexdigest()

def git(cwd, *args):
    return subprocess.check_output(['git', '-C', str(cwd), *args])

def dump(path, data):
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n')

def tree(repo, rev):
    rows = []
    for item in git(repo, 'ls-tree', '-r', '-l', '-z', rev).split(b'\0'):
        if not item:
            continue
        head, path = item.split(b'\t', 1)
        mode, kind, blob, size = head.decode().split()
        rows.append(dict(path=path.decode(), git_blob=blob, bytes=int(size) if size != '-' else 0, mode=mode, git_type=kind))
    return rows

def contents(repo, rows):
    """Read pinned blobs in one batch; do not check out or change source trees."""
    selected = [r for r in rows if r['git_type'] == 'blob']
    proc = subprocess.Popen(['git', '-C', str(repo), 'cat-file', '--batch'], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    result = {}
    for row in selected:
        proc.stdin.write((row['git_blob'] + '\n').encode())
        proc.stdin.flush()
        header = proc.stdout.readline().decode().split()
        payload = proc.stdout.read(int(header[2]))
        proc.stdout.read(1)
        result[row['path']] = payload
    proc.stdin.close()
    proc.wait()
    if proc.returncode:
        raise RuntimeError('Source blob read failed')
    return result

def import_paths(text):
    values = []
    for block in re.findall(r'(?ms)^import\s*\((.*?)^\)', text):
        values.extend(re.findall(r'"([^"]+)"', block))
    values.extend(re.findall(r'(?m)^import\s+(?:[\w.]+\s+)?"([^"]+)"', text))
    return sorted(set(values))

def go_symbols(text):
    result = []
    patterns = [
        ('function', r'^func\s+(?:\([^\n]*?\)\s+)?([A-Za-z_]\w*)'),
        ('type', r'^type\s+([A-Za-z_]\w*)'),
        ('constant', r'^const\s+([A-Za-z_]\w*)\s+'),
        ('global', r'^var\s+([A-Za-z_]\w*)\s+')]
    for line, text_line in enumerate(text.splitlines(), 1):
        for kind, pattern in patterns:
            match = re.match(pattern, text_line)
            if match:
                result.append(dict(name=match.group(1), kind=kind, line=line, declaration=text_line.strip()))
    return result

def scala_symbols(text):
    result = []
    for line, value in enumerate(text.splitlines(), 1):
        m = re.match(r'\s*(?:(?:private|protected|final|override|sealed|abstract|lazy|implicit|case)\s+)*(class|object|trait|def|val|type)\s+([A-Za-z_$][\w$]*)', value)
        if m:
            result.append(dict(name=m.group(2), kind=m.group(1), line=line, declaration=value.strip()))
    return result

def manual_family(path):
    if path.startswith(('test/', 'pkg/testing/')) or path.endswith('_test.go') or '/fuzz_test/' in path or '/fake/' in path:
        return 'a-assurance'
    if path.startswith(('pkg/sdk/', 'pkg/allocation/go/')):
        return 'a-protocol'
    if path.startswith('pkg/apis/'):
        return 'a-api'
    if path.startswith('pkg/client/applyconfiguration'):
        return 'a-apply'
    if path.startswith('pkg/client/clientset'):
        return 'a-client'
    if path.startswith(('pkg/client/listers', 'pkg/client/informers')):
        return 'a-observation'
    if path.startswith('pkg/cloudproduct'):
        return 'a-cloud'
    if path.startswith('pkg/metrics'):
        return 'a-metrics'
    if path.startswith('pkg/portallocator'):
        return 'a-ports'
    for name in ('gameservers', 'gameserversets', 'fleets', 'fleetautoscalers'):
        if path.startswith(f'pkg/{name}/'):
            return 'a-' + name
    if path.startswith('pkg/gameserverallocations/processor'):
        return 'a-allocation'
    if path.startswith(('pkg/gameserverallocations', 'pkg/allocation/converters')):
        return 'a-allocation'
    if path.startswith('pkg/sdkserver'):
        return 'a-sdkserver'
    if path.startswith('cmd/'):
        return 'a-entrypoints'
    if path.startswith('sdks/'):
        return 'a-sdkclients'
    if path.startswith('examples/'):
        return 'a-examples'
    if path.startswith(('build/', 'site/')) and path.endswith('.go'):
        return 'a-tooling'
    if path.startswith('proto/'):
        return 'a-protocol'
    if path.startswith(('install/', 'build/', '.github/', 'ci/')):
        return 'a-delivery-inputs'
    if path.startswith('pkg/util/'):
        return 'a-utilities'
    return 'a-foundation'

def cpg_family(path, row):
    if '/src/test/' in path:
        return 'c-assurance'
    stem = Path(path).stem
    if '/schema/' in '/' + path and path.endswith('.scala'):
        if stem in ('SchemaBase', 'Base', 'Operators'):
            return 'c-schema-base'
        if stem in ('FileSystem', 'Namespace', 'Configuration', 'MetaData', 'Type'):
            return 'c-schema-identity'
        if stem in ('Method', 'Ast', 'CallGraph'):
            return 'c-schema-program'
        if stem in ('Cfg', 'Dominators', 'Pdg'):
            return 'c-schema-flow'
        if stem in ('Shortcuts', 'Comment', 'Tags', 'Binding', 'Annotation', 'Finding', 'Hidden'):
            return 'c-schema-enrichment'
        return 'c-schema-compose'
    if path.startswith('domainClasses/'):
        return 'c-domain'
    if 'cpgloading' in path:
        if stem in ('ZipArchive', 'ProtoToGraphNodeMappings', 'ProtoCpgLoader'):
            return 'c-protobuf-loading'
        return 'c-storage-loading'
    if '/passes/' in path:
        return 'c-passes'
    if '/utils/' in path or stem == 'Implicits':
        return 'c-utilities'
    if stem == 'SerializedCpg':
        return 'c-serialization'
    if path.startswith('codepropertygraph/src/main'):
        return 'c-graph'
    if path.startswith(('schema2json', 'proto-bindings')):
        return 'c-schema-tools'
    return 'c-build'

def inventory():
    arows = tree(ROOT, AGONES)
    crows = tree(CPG_GIT, CPG)
    asrc = contents(ROOT, [r for r in arows if not r['path'].startswith('vendor/')])
    csrc = contents(CPG_GIT, crows)
    classes = {r['path']: r for r in json.loads((ROOT/'verification/agones/first-generation-cpg-classification.json').read_text())['files']}
    for repo, rows, sources in [('agones', arows, asrc), ('cpg', crows, csrc)]:
        for row in rows:
            p = row['path']
            payload = sources.get(p)
            txt = payload.decode('utf-8', errors='replace') if payload is not None and b'\0' not in payload else ''
            row.update(repository=repo, source_revision=AGONES if repo == 'agones' else CPG)
            row['sha256'] = sha(payload) if payload is not None else None
            row['package'] = str(Path(p).parent)
            row['symbols'] = go_symbols(txt) if p.endswith('.go') else scala_symbols(txt) if p.endswith(('.scala', '.java')) else []
            row['source_lines'] = len(txt.splitlines())
            row['id'] = repo + ':' + p
            if repo == 'agones':
                row['imports'] = import_paths(txt) if p.endswith('.go') else []
                row['family'] = manual_family(p)
                generated = ('Code generated' in txt[:3500] or 'DO NOT EDIT' in txt[:3500])
                if p.startswith('vendor/'):
                    disposition = 'THIRD_PARTY_REFERENCE'
                elif p.startswith('crates/') or p in ('Cargo.toml', 'Context-Generic Programming Patterns (DRAFT).pdf'):
                    disposition = 'PRESERVE_EXISTING_MIGRATION_INPUT'
                elif p.endswith('AGENTS.MD'):
                    disposition = 'GUIDANCE_REFERENCE'
                elif p.endswith('.go'):
                    disposition = 'DEFERRED_HARNESS' if p.endswith('_test.go') else 'DEFERRED_ASSURANCE_SUPPORT' if row['family'] == 'a-assurance' else 'GENERATED_PRODUCTION' if generated else 'HANDWRITTEN_PRODUCTION'
                elif p.startswith(('test/', 'pkg/testing/')):
                    disposition = 'DEFERRED_ASSURANCE_ASSET'
                elif p.startswith(('install/', 'proto/', 'build/', '.github/', 'ci/')) or p in ('go.mod', 'go.sum', '.dockerignore', '.gcloudignore', '.gitattributes', '.gitignore', '.golangci.yml', '.yamllint', 'cloudbuild.yaml'):
                    disposition = 'REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT'
                elif p.startswith('sdks/rust/'):
                    disposition = 'REUSE_RUST_SDK'
                elif p.startswith('examples/') and Path(p).suffix in ('.yaml', '.yml', '.json', '.proto'):
                    disposition = 'REQUIRED_COMPATIBILITY_EXAMPLE'
                else:
                    disposition = 'RETAINED_REFERENCE'
                row['detailed'] = disposition not in ('THIRD_PARTY_REFERENCE', 'GUIDANCE_REFERENCE', 'RETAINED_REFERENCE', 'PRESERVE_EXISTING_MIGRATION_INPUT')
            else:
                cls = classes[p]
                row['family'] = cpg_family(p, cls)
                row['classification'] = cls['classification']
                row['source_role'] = cls['role']
                disposition = 'GENERATED_PRODUCTION' if cls['classification'] == 'GENERATED' else 'DEFERRED_HARNESS' if '/src/test/' in p else 'HANDWRITTEN_PRODUCTION' if cls.get('required_handwritten_port_accounting') else 'REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT' if cls['role'] == 'PRODUCTION_RESOURCE' else 'RETAINED_REFERENCE'
                row['detailed'] = True
                row['imports'] = []
            row['family']=ALIASES.get(row['family'],row['family'])
            row['disposition'] = disposition
            if disposition in ('THIRD_PARTY_REFERENCE','GUIDANCE_REFERENCE','RETAINED_REFERENCE','PRESERVE_EXISTING_MIGRATION_INPUT'):
                row['destination'] = 'RETAIN_SOURCE_REFERENCE:' + p
            else:
                dest = p[:-8] + '_harness.rs' if p.endswith('_test.go') else p[:-3] + '.rs' if p.endswith('.go') else p.rsplit('.',1)[0] + '.rs' if p.endswith(('.scala','.java')) else p
                row['destination'] = ('agones-rs/' if repo == 'agones' else 'cpg-rs/') + dest
    return arows, crows, asrc, csrc

def schedule(arows):
    packages = {r['package']:r['family'] for r in arows if r['path'].endswith('.go') and r['family']!='a-assurance' and r['detailed']}
    edges = {k:set() for k in A if k!='a-processor'}
    for r in arows:
        if r['family']=='a-assurance':continue
        for imp in r.get('imports',[]):
            if imp.startswith('agones.dev/agones/'):
                p=imp.removeprefix('agones.dev/agones/')
                if p in packages and packages[p]!=r['family']:edges[r['family']].add(packages[p])
    aprod=set(edges)-{'a-assurance','a-delivery'}
    edges['a-macros'].update(aprod-{'a-macros'})
    edges.update({k:set(v) for k,v in C_DEPS.items()})
    cprod=set(C)-{'c-assurance','c-delivery'}
    edges['c-publication'].update(aprod)
    edges['a-assurance'].update(aprod|cprod)
    edges['c-assurance'].update(aprod|cprod)
    edges['a-delivery'].update(['a-assurance','c-assurance'])
    edges['c-delivery'].add('a-delivery')
    priorities={'MANUAL':[x for x in A_PRIORITY if x in edges], 'CPG':C_PRIORITY}
    output=[];done=set();route='MANUAL'
    while len(done)<len(edges):
        choices=[k for k in priorities[route] if k not in done and edges[k]<=done]
        if not choices:
            unfinished=[k for k in priorities[route] if k not in done]
            if not unfinished:
                route='CPG' if route=='MANUAL' else 'MANUAL';continue
            raise RuntimeError('Cannot author a dependency-ready schedule: '+str((route,{k:sorted(edges[k]-done) for k in unfinished})))
        key=choices[0];index=len(output)+1
        title=(A[key][0] if route=='MANUAL' else C[key][0])
        if key=='a-delivery':title='Agones acceptance and delivery preparation'
        slug=re.sub('[^a-z0-9]+','-',title.lower()).strip('-')
        output.append(dict(id=f'M{index:04}',number=index,key=key,route=route,title=title,filename=f'{index:04}-{slug}.md',dependencies_keys=sorted(edges[key]),phase='POST_IMPLEMENTATION_VERIFICATION' if key.endswith(('assurance','delivery')) else 'IMPLEMENTATION_ONLY',status='PLANNED_NOT_EXECUTED'))
        done.add(key);route='CPG' if route=='MANUAL' else 'MANUAL'
    bykey={m['key']:m for m in output}
    for m in output:m['dependencies']=[bykey[k]['id'] for k in m['dependencies_keys']]
    return output

REPOSITORIES={
 'project-guidance':dict(url='https://github.com/QuasarRay/agones',root=ROOT),
 'agones':dict(url='https://github.com/QuasarRay/agones',revision=AGONES,root=ROOT),
 'cpg':dict(url='https://github.com/ShiftLeftSecurity/codepropertygraph',revision=CPG,root=CPG_ROOT),
 'codepropertygraph-reference':dict(url='https://github.com/ShiftLeftSecurity/codepropertygraph',root=CPG_GIT),
 'CPGSchema':dict(url='https://github.com/QuasarRay/CPGSchema',root=ROOT/'CPGSchema'),
 'cgp':dict(url='https://github.com/contextgeneric/cgp',root=ROOT/'cgp'),
 'derive-deftly':dict(url='https://gitlab.torproject.org/Diziet/rust-derive-deftly',root=ROOT/'derive-deftly'),
 'comprende':dict(url='https://github.com/QuasarRay/comprende',root=ROOT/'comprende'),
 'better-comprehension-in-rust':dict(url='https://github.com/Natural-selection1/better-comprehension-in-rust',root=ROOT/'better-comprehension-in-rust'),
 'genco':dict(url='https://github.com/udoprog/genco',root=ROOT/'genco'),
 'kubernetes-website':dict(url='https://github.com/kubernetes/website',root=ROOT/'kubernetes-website'),
}

class Author:
    def __init__(self,arows,crows,asrc,csrc,milestones):
        self.rows=arows+crows;self.sources={'agones':asrc,'cpg':csrc};self.milestones=milestones
        self.bykey={m['key']:m for m in milestones};self.quotes={};self.packages=[]
        for name,r in REPOSITORIES.items():
            if 'revision' not in r:r['revision']=git(r['root'],'rev-parse','HEAD').decode().strip()
        for row in self.rows:
            row['milestone_id']=self.bykey[row['family']]['id'] if row['detailed'] else None
            row['entry_anchor']='file-'+sha(row['id'])[:16] if row['detailed'] else None

    def raw(self,repo,path):
        if repo in self.sources:return self.sources[repo].get(path,b'')
        return (REPOSITORIES[repo]['root']/path).read_bytes()

    def quote(self,repo,path,start=1,end=None,kind='UPSTREAM_IMPLEMENTATION_EXCERPT'):
        data=self.raw(repo,path);lines=data.decode('utf-8',errors='replace').splitlines()
        if not lines:raise ValueError('Cannot quote empty source '+repo+':'+path)
        start=max(1,min(start,len(lines)));end=min(len(lines),end or start+11)
        text='\n'.join(lines[start-1:end]);r=REPOSITORIES[repo]
        ident='Q-'+sha(repo+'\0'+r['revision']+'\0'+path+'\0'+str(start)+'\0'+str(end)+'\0'+text)[:18]
        prefix='/-/blob/' if 'gitlab.' in r['url'] else '/blob/'
        url=r['url']+prefix+r['revision']+'/'+quote(path,safe='/')+f'#L{start}-L{end}'
        self.quotes[ident]=dict(id=ident,repository=repo,repository_url=r['url'],revision=r['revision'],path=path,start_line=start,end_line=end,source_sha256=sha(data),kind=kind,text=text,permalink=url)
        q=self.quotes[ident]
        return f'**Direct quotation — {kind}; {ident}.** [{repo}/{path}, lines {start}–{end}]({url}). Revision `{r["revision"]}`; source SHA-256 `{q["source_sha256"]}`.\n\n'+ '\n'.join('> '+x for x in text.splitlines())+'\n'

    def passage(self,repo,path,needle=None,length=12,kind='UPSTREAM_DOCUMENTATION'):
        lines=self.raw(repo,path).decode('utf-8',errors='replace').splitlines()
        if not lines:raise ValueError((repo,path))
        index=0
        if needle:
            hits=[i for i,x in enumerate(lines) if needle.lower() in x.lower()]
            if hits:index=hits[0]
        elif lines[0]=='---':
            index=next((i+1 for i in range(1,len(lines)) if lines[i]=='---'),0)
        else:
            index=next((i for i,x in enumerate(lines) if x.strip() and not x.startswith(('#','[!','![','<!--','<'))),0)
        return self.quote(repo,path,index+1,index+length,kind)

    def family_quote(self,key):
        if key.startswith('a-'):
            p=A[key][1]
            if p not in self.sources['agones']:p='README.md'
            return self.passage('agones',p,length=13)
        if key in ('c-bundle','c-normalization','c-ir','c-lowering-data','c-lowering-control','c-emission','c-publication','c-delivery'):
            return self.passage('genco','README.md','Central to genco',13)
        needle='Domain class generation' if 'schema' in key or key=='c-domain' else 'Loading a codepropertygraph' if 'loading' in key or key=='c-graph' else 'Scripts' if key=='c-build' else 'Usage as a dependency'
        return self.passage('cpg','README.md',needle,12)

    def file_quote(self,row):
        repo,path=row['repository'],row['path'];data=self.raw(repo,path)
        if not data or b'\0' in data:
            row['documentation_gap']='Binary/empty resource has no prose to quote; quote the upstream loader/test contract and retain exact blob/hash.'
            for p,b in self.sources[repo].items():
                if b'\0' not in b and Path(path).name.encode() in b and p!=path:
                    lines=b.decode('utf-8',errors='replace').splitlines()
                    i=next(i for i,l in enumerate(lines) if Path(path).name in l)
                    return self.quote(repo,p,max(1,i-3),i+10,'UPSTREAM_RESOURCE_CONSUMER_EXCERPT')
            return self.passage(repo,'README.md',length=12)
        lines=data.decode('utf-8',errors='replace').splitlines()
        symbols=row['symbols']
        index=next((s['line']-1 for s in symbols if s['kind'] in ('function','def','class','object','trait','type')),None)
        if index is None:
            index=next((i for i,l in enumerate(lines) if l.strip() and not re.match(r'\s*(//|/\*|\*|#|<!--|Copyright|Licensed|SPDX|package |import )',l)),0)
        start=index
        while start>0 and index-start<12 and (lines[start-1].strip().startswith(('//','/*','*')) or not lines[start-1].strip()):start-=1
        kind='UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT' if any(l.strip().startswith(('//','/**','*')) for l in lines[start:index]) else 'UPSTREAM_IMPLEMENTATION_EXCERPT'
        if Path(path).suffix.lower() in ('.md','.rst','.txt') and 'LICENSE' not in path:kind='UPSTREAM_DOCUMENTATION'
        row['documentation_gap']='No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs.'
        return self.quote(repo,path,start+1,min(len(lines),max(index+8,start+15)),kind)

    def traits(self,row):
        text=self.raw(row['repository'],row['path']).decode('utf-8',errors='replace')
        rules=[(r'\bdefer\b','Retain deferred cleanup order and its interaction with returns/errors.'),(r'\bgo\s+\w|\bgoroutine\b','Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.'),(r'\bselect\s*\{|\bchan\b','Preserve channel/select outcomes and blocking/cancellation semantics; do not turn nondeterminism into fixed ordering.'),(r'\bpanic\(|\brecover\(','Preserve source panic/recover boundaries; do not use an unsupported panic as a translation placeholder.'),(r'\bnil\b|omitempty|Option\[','Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.'),(r'\bmap\[|HashMap','Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.'),(r'Float|Double|float32|float64','Preserve source floating width, narrowing, exceptional values and relevant bits.'),(r'\bcontext\.|cancel|shutdown|Shutdown|Close\(|close\(','Retain resource ownership, cancellation, shutdown and partial-failure cleanup.'),(r'newProperty|addProperty|addNodeType|addEdgeType','Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.')]
        found=[msg for pattern,msg in rules if re.search(pattern,text)]
        return found[:6]

    def file_entry(self,row,key):
        q=self.file_quote(row);symbols=row['symbols'];p=row['path']
        out=[f'<a id="{row["entry_anchor"]}"></a>\n',f'### File `{p}`\n',f'**Source:** `{row["repository"]}` at `{row["source_revision"]}`; Git blob `{row["git_blob"]}`. **Disposition:** `{row["disposition"]}`. **Destination:** `{row["destination"]}`.\n',q,'**Reference boundary.** '+row['documentation_gap']+' Quoted commands are source evidence, not permission to execute them.\n','**Derived implementation instructions.**\n']
        if row['disposition']=='GENERATED_PRODUCTION':
            out.append('1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.')
        elif row['disposition'].startswith('DEFERRED_'):
            out.append('1. Author this harness/helper/fixture only after the global implementation gate. Preserve the source case/helper identity, setup/actions/observations and independent invariants; never substitute production-derived expectations.')
        elif row['disposition']=='RETAINED_REFERENCE':
            out.append('1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.')
        else:
            out.append('1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.')
        if symbols:
            names=', '.join('`'+s['name']+'` (L'+str(s['line'])+')' for s in symbols[:24])
            out.append(f'2. Account for {len(symbols)} lexically identified declarations, including {names}. The full declaration list is recorded under `{row["id"]}` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.')
        else:
            out.append('2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.')
        if row['imports']:
            own=[x for x in row['imports'] if x.startswith('agones.dev/agones/')]
            if own:out.append('3. Consume the already implemented source packages '+', '.join('`'+x+'`' for x in own)+'. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.')
        for n,rule in enumerate(self.traits(row),4):out.append(f'{n}. {rule}')
        if row['repository']=='cpg' and p.endswith(('.sbt','.bzl','.sh')) and row['disposition']=='HANDWRITTEN_PRODUCTION':
            out.append('9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.')
        out.append('10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.')
        out.append('11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.')
        if row['repository']=='cpg' and '/schema/' in '/'+p and p.endswith('.scala'):
            candidate=ROOT/'CPGSchema/src'/Path(p).stem/'mod.rs'
            if candidate.exists():out.append(f'12. Reuse candidate: `{candidate.relative_to(ROOT)}` (SHA-256 `{sha(candidate.read_bytes())}`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.')
        out.append('13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.\n')
        return '\n\n'.join(out)

    def milestone(self,m):
        from artifact_support import new_output_sections
        key=m['key'];rows=[r for r in self.rows if r['detailed'] and r['family']==key]
        actions=A[key][2] if key.startswith('a-') else C[key][1]
        if key=='a-delivery':actions=[actions[0], 'Prepare exact-head delivery manifests, review changes and authorized PR material for agones-rs-dev. Final integration/publication occurs in the shared closing milestone after both routes are ready; do not mark a merge complete in advance.', actions[2]]
        deps=', '.join(f'[{self.bykey[k]["id"]}]({self.bykey[k]["filename"]})' for k in m['dependencies_keys']) or 'The shared contract and pinned source inventory.'
        out=[f'<a id="{m["id"].lower()}"></a>\n',f'# {m["id"]} — {m["title"]}\n',f'**Route:** `{m["route"]}`. **Phase:** `{m["phase"]}`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-{m["id"]}-v1` in [cost-analysis.md](../cost-analysis.md).\n',f'**Prerequisites:** {deps}\n','## Source contract and interpretation\n',self.family_quote(key),'**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.\n','## Implementation sequence\n']
        out.extend(f'{i}. {x}' for i,x in enumerate(actions,1))
        out.extend(['\n## Architecture, readability and authored outputs\n',f'1. Own {len(rows)} inventoried file entries in this milestone'+(' plus the concrete new translation/support interfaces described above.' if not rows else '. Every required file below has a unique milestone owner.'), '2. Record typed declarations -> derive-deftly templates -> CGP provider implementations/delegation -> production context -> lifecycle consumer. Identify each mutable state/resource owner and effect boundary.', '3. Apply Pythonic readability and idiomatic Rust as equal design criteria. Comprehensions simplify mapping/filtering; explicit small functions clarify effects. Do not add a Python interpreter, dynamic object layer, reflection registry or competing manager.', '4. Preserve source-relative directories, exact case/stems and singular/plural forms. Map Go `_test.go` only to `_harness.rs`; keep Scala Test/Tests stems unchanged. Register unconventional names outside the mirror.', '\n**Python-familiar valid Rust example (schematic; not executed):**\n', '```rust\nuse comprende::c;\nuse better_comprehension::vector;\n\nlet ready_names = c![server.name().to_owned()\n                    for server in &servers if server.is_ready()];\nlet selected_ids = vector![node.id() for node in nodes.iter()\n                          if node.has_required_label()];\n```\n','These examples explain reading order and borrowing. Actual APIs must come from the selected pinned dependencies. The owning CGP provider supplies the collections; derive-deftly supplies repeated declarations/projections. Preserve source ordering and avoid redundant clones or a comprehension around complicated effects.\n','## Package/module and file instructions\n'])
        if not rows:
            out.append('This is a required cross-cutting implementation milestone. Its concrete outputs are the stage interfaces/rules/transaction or delivery artifacts specified above. Place new Rust-only machinery under `rust-support/agones` or `rust-support/cpg`, keeping source mirrors unchanged. Bind output ownership in the existing ledger; a design document or empty provider does not complete it.\n')
        grouped=collections.defaultdict(list)
        for r in rows:grouped[(r['repository'],r['package'])].append(r)
        for (repo,pkg),files in sorted(grouped.items()):
            module='::'.join(pkg.split('/')) if pkg!='.' else '<crate root>'
            out.append(f'## Package `{repo}:{pkg}` -> module `{module}`\n')
            docpath=pkg+'/doc.go' if pkg!='.' else 'doc.go'
            if repo=='agones' and docpath in self.sources[repo]:
                q=self.passage(repo,docpath,'Package ',12,'UPSTREAM_DOCUMENTATION_COMMENT')
            else:q=self.family_quote(key)
            out.extend([q, '**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.\n'])
            self.packages.append(dict(repository=repo,package=pkg,milestone_id=m['id'],files=[r['id'] for r in files],module=module))
            for row in sorted(files,key=lambda x:x['path']):out.append(self.file_entry(row,key))
        out.append(new_output_sections(self,m))
        out.extend(['\n## Completion and deferred assurance\n', '1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.', '2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.', '3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.', '4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.\n'])
        return '\n\n'.join(out).rstrip()+'\n'

    def calculate_forecasts(self):
        fixed={
         'c-bundle':(120000,400000,1300000),'c-normalization':(100000,320000,1100000),
         'c-ir':(180000,600000,2000000),'c-lowering-data':(280000,1000000,3600000),
         'c-lowering-control':(450000,1600000,5500000),'c-emission':(120000,420000,1400000),
         'c-publication':(60000,180000,600000),'a-macros':(150000,500000,1700000),
         'a-delivery':(50000,180000,650000),'c-delivery':(180000,650000,2200000)}
        result=[]
        for m in self.milestones:
            rows=[r for r in self.rows if r['detailed'] and r['family']==m['key']]
            units=0
            for r in rows:
                d=r['disposition'];size=max(.25,r['source_lines']/200)
                if d=='GENERATED_PRODUCTION':units+=.04*size
                elif d=='RETAINED_REFERENCE':units+=.02
                elif d.startswith('DEFERRED'):units+=.4*size
                elif r['path'].endswith(('.go','.scala','.java','.rs')):units+=size*(.45 if r['repository']=='cpg' and '/schema/' in '/'+r['path'] else .7)
                else:units+=.08*size
            base=[round((5000+units*rate)/1000)*1000 for rate in (6000,20000,65000)]
            if m['key'] in fixed:base=[x+y for x,y in zip(base,fixed[m['key']])]
            if m['phase']=='POST_IMPLEMENTATION_VERIFICATION':base=[round(x*1.6/1000)*1000 for x in base]
            result.append(dict(id='F-'+m['id']+'-v1',milestone_id=m['id'],route=m['route'],phase=m['phase'],file_count=len(rows),planning_units=round(units,3),tokens=dict(zip(('low','base','high'),base)),credits='unknown',calibration='UNVALIDATED_PLANNING_ASSUMPTIONS',forecast_version=1))
        return result

def schema_reconciliation(crows):
    before={r['path']:r for r in tree(CPG_GIT,SCHEMA_ANCHOR)}
    result=[]
    for row in crows:
        p=row['path']
        if not (p.startswith('schema/src/main/scala/') and p.endswith('.scala')):continue
        candidate=ROOT/'CPGSchema/src'/Path(p).stem/'mod.rs'
        result.append(dict(path=p,anchor_blob=before.get(p,{}).get('git_blob'),pipeline_blob=row['git_blob'],source_identical=before.get(p,{}).get('git_blob')==row['git_blob'],reuse_candidate=str(candidate.relative_to(ROOT)),candidate_exists=candidate.exists(),candidate_sha256=sha(candidate.read_bytes()) if candidate.exists() else None,behavior_status='UNVERIFIED',body_mapping_status='REQUIRES_SOURCE_TO_NATIVE_SYMBOL_RECONCILIATION',destination=row['destination'],milestone_id=row['milestone_id']))
    return dict(anchor=SCHEMA_ANCHOR,pipeline_revision=CPG,method='Read-only Git blob comparison and current candidate path/hash inventory; no compiler or project verification',source_files=len(result),all_source_blobs_identical=all(r['source_identical'] for r in result),files=result)

def reuse_inventory(author):
    files=[]
    for base in ('crates','cpg-rs/src','CPGSchema/src','verification/agones'):
        folder=ROOT/base
        if not folder.exists():continue
        for p in sorted(folder.rglob('*')):
            if not p.is_file() or p.suffix not in ('.rs','.py') or DATA in p.parents or 'target' in p.parts:continue
            rel=str(p.relative_to(ROOT));low=rel.lower()
            if rel.startswith('CPGSchema/'):
                stem=p.parent.name if p.name=='mod.rs' else p.stem
                candidate=next((r for r in author.rows if r['repository']=='cpg' and Path(r['path']).stem==stem),None)
                family=candidate['family'] if candidate else 'c-schema-all'
            elif rel.startswith('cpg-rs/'):
                if any(t in low for t in ('test','harness')):family='c-assurance'
                elif any(t in low for t in ('lower','translat')):family='c-ir'
                elif any(t in low for t in ('schema','domain')):family='c-schema-all'
                elif any(t in low for t in ('genco','emit','generat')):family='c-emission'
                else:family='c-graph'
            elif '/tests/' in rel:family='a-assurance'
            elif 'macros' in rel:family='a-macros'
            elif 'util/' in rel:family='a-utilities'
            elif 'fleetautoscaler' in low:family='a-fleetautoscalers'
            elif 'gameserverset' in low:family='a-gameserversets'
            elif 'fleet' in low:family='a-fleets'
            elif any(t in low for t in ('allocation','processor')):family='a-allocation'
            elif 'port' in low:family='a-ports'
            elif 'sdk' in low:family='a-sdkserver'
            elif 'controller' in low:family='a-gameservers'
            elif rel.startswith('verification/'):
                family='c-emission' if any(t in low for t in ('first_generation','first-generation','cpg')) else 'a-assurance'
            else:family='a-api'
            family=ALIASES.get(family,family)
            m=author.bykey[family]
            files.append(dict(path=rel,sha256=sha(p.read_bytes()),milestone_id=m['id'],route=m['route'],owner_status='PROVISIONAL_ADOPTION_OWNER_REQUIRES_SYMBOL_RECONCILIATION',destination='PRESERVE_IN_PLACE_UNTIL_SOURCE_MAPPING_AND_ALL_REFERENCES_ACCOUNTED',replacement_status='NOT_AUTHORIZED_BY_FILE_EXISTENCE',notes='Preserve original; inspect source mappings and consumers before relocating. This path-based index does not establish source behavior or completeness.'))
    return dict(purpose='Preserve current implementation and assign adoption-review ownership without moving code',files=files)

def main():
    from roadmap_text import CONTRACT,POLICY,COMPARISON_README
    from artifact_support import assemble,scenario_costs,cost_report,NEW_OUTPUTS
    if '--assemble' in sys.argv:
        dump(DATA/'assembly.json',assemble(ROOT));return
    if any(p.name not in ('README.md','AGENTS.MD') for p in (ROOT/'cost-analysis-comparison').glob('*.md')):
        raise RuntimeError('Actual comparison records exist: preserve v1; use --assemble for documentation or author an explicit new forecast version.')
    arows,crows,asrc,csrc=inventory()
    milestones=schedule(arows)
    author=Author(arows,crows,asrc,csrc,milestones)
    old_manifest=DATA/'milestones.json'
    if old_manifest.exists():
        keep={m['filename'] for m in milestones}
        for old in json.loads(old_manifest.read_text())['milestones']:
            p=ROAD/old['filename']
            if p.name not in keep and p.exists():
                if 'PLANNED_NOT_EXECUTED' not in p.read_text():raise RuntimeError('Refusing to remove an executed/edited milestone '+str(p))
                p.unlink()
    for m in milestones:(ROAD/m['filename']).write_text(author.milestone(m))
    schema=schema_reconciliation(crows)
    dump(DATA/'schema-reconciliation.json',schema)
    reuse=reuse_inventory(author);dump(DATA/'reuse-inventory.json',reuse)
    forecasts=author.calculate_forecasts();scenarios=scenario_costs(milestones,forecasts)
    baseline=dict(version=1,date=DATE,status='PLANNING_NOT_MEASURED',immutable_when_execution_begins=True,source_snapshots={'agones':AGONES,'cpg':CPG},milestones=forecasts,aggregate_model=scenarios,credit_forecast='unknown',no_goal_budget_authorized=True)
    dump(DATA/'forecast-baseline-v1.json',baseline)
    replacements={
     '@@PROJECT_QUOTE@@':author.quote('project-guidance','AGENTS.MD',19,21,'USER_SUPPLIED_PROJECT_GUIDANCE'),
     '@@CGP_QUOTE@@':author.quote('cgp','README.md',9,15,'UPSTREAM_DOCUMENTATION'),
     '@@DEFTLY_QUOTE@@':author.quote('derive-deftly','README.md',6,15,'UPSTREAM_DOCUMENTATION'),
     '@@COMPRENDE_QUOTE@@':author.quote('comprende','README.md',14,29,'UPSTREAM_DOCUMENTATION'),
     '@@BETTER_QUOTE@@':author.quote('better-comprehension-in-rust','README.md',30,46,'UPSTREAM_DOCUMENTATION'),
     '@@AGONES_COUNT@@':str(len(arows)),'@@CPG_COUNT@@':str(len(crows)),'@@DETAIL_COUNT@@':str(sum(r['detailed'] for r in author.rows)),
     '@@MILESTONE_TABLE@@':'| Milestone | Route | Prerequisites | Work |\n|---|---|---|---|\n'+'\n'.join(f'| [{m["id"]}]({m["filename"]}) | {m["route"]} | '+(', '.join(m['dependencies']) or 'Contract')+f' | {m["title"]} |' for m in milestones)}
    for name,source in [('0000-contract.md',CONTRACT),('alternation-policy.md',POLICY)]:
        for old,new in replacements.items():source=source.replace(old,new)
        (ROAD/name).write_text(source)
    # Add the actual schema constructor sequence, not alphabetic inventory order.
    schema_path='schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CpgSchema.scala'
    constructor_quote=author.quote('cpg',schema_path,7,32,'UPSTREAM_IMPLEMENTATION_EXCERPT')
    compose=ROAD/author.bykey['c-schema-all']['filename']
    compose.write_text(compose.read_text()+'\n## Constructor order, the source cycle and identity\n\n'+constructor_quote+'\n**Derived requirement.** Preserve this exact constructor order and the shared SchemaBuilder/handle identities. CpgSchema constructs the modules, while several modules import CpgSchema.PropertyDefaults. The schema is therefore one cohesive milestone, including defaults, SchemaBase, ProtoIds, ProtoSerialize and Protogen. Establish declarations/constants first inside this milestone, then constructors in the quoted order and finally instance.build. File-entry ordering is an inventory order, not permission to reorder construction or create a second owner of constants.\n')
    # Kubernetes reference is normative for resource/watch/controller contract interpretation.
    for key,path,needle in [('a-observation','content/en/docs/reference/using-api/api-concepts.md','watch'),('a-gameservers','content/en/docs/concepts/architecture/controller.md','control loop')]:
        p=ROAD/author.bykey[key]['filename']
        p.write_text(p.read_text()+'\n## Upstream platform contract reference\n\n'+author.passage('kubernetes-website',path,needle,14)+'\n**Derived requirement.** Preserve resource/watch/reconciliation observations at the Rusternetes capability boundary. This source describes Kubernetes contracts; it does not authorize a second orchestrator integration or a pre-gate probe.\n')
    # CPGSchema is a reuse reference, not an authority that can replace pinned Scala behavior.
    p=ROAD/author.bykey['c-schema-all']['filename']
    p.write_text(p.read_text()+'\n## Native schema reuse boundary\n\n'+author.passage('CPGSchema','README.md',None,15)+'\n**Derived requirement.** Reconcile source blobs and each Rust constructor/body mapping using schema-reconciliation.json. Reuse compatible implementation, preserve original files until references are accounted for, and leave behavior status UNVERIFIED until the global gate.\n')
    # Tie new compiler stages to specific upstream representation/behavior contracts,
    # without presenting our new Go-to-Rust design as a pre-existing official spec.
    ast='schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.scala'
    supplementary={
      'c-bundle':('cpg',ast,71,89,'Retain verbatim code, source location and sibling order as identified facts in the source-bound bundle. Preserve additional frontend/type facts explicitly rather than assuming AST code text alone is enough.'),
      'c-normalization':('cpg',ast,58,89,'Carry ORDER, CODE and source-position meaning losslessly into normalized records. Keep absent/default/unknown states explicit. Canonical storage order cannot redefine source evaluation order.'),
      'c-ir':('cpg',ast,14,27,'Represent expressions, source types and common control structures in typed IR with provenance. Any desugaring must retain source effects, order and observable outcomes; Genco cannot infer the missing semantics.'),
      'c-lowering-data':('agones','pkg/apis/agones/v1/gameserver.go',350,380,'Translate these real defaulting operations, method calls, field writes and absence/default decisions through typed lowering. Preserve original widths, nil/empty distinctions and observable sequencing; do not substitute hand-copied output for CPG provenance.'),
      'c-lowering-control':('agones','pkg/util/workerqueue/workerqueue.go',151,181,'Retain this concrete queue termination/error/cleanup behavior through control-flow and defer lowering. CGP owns queue/task lifetimes and derive-deftly the repeated wiring. The translated body must reach the actual provider, not an independent reference facade.'),
      'c-publication':('cpg','README.md',50,67,'Generated material has declared source inputs and a canonical generator owner. Extend that provenance principle to the separate complete Agones artifact. Atomic staging/publication and the user-selected implementation gate are engineering requirements of this roadmap, not claims made by this upstream excerpt.')}
    for key,(repo,path,start,end,requirement) in supplementary.items():
        p=ROAD/author.bykey[key]['filename']
        kind='UPSTREAM_DOCUMENTATION' if path.endswith('.md') else 'UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT'
        p.write_text(p.read_text()+'\n## Specific source obligations for this new stage\n\n'+author.quote(repo,path,start,end,kind)+'\n**Derived stage requirement.** '+requirement+' No dedicated upstream Go-to-Rust translator specification is claimed; this stage is engineered from the quoted source obligations and the user-approved shared contract.\n')
    guidance='''# AGENTS.MD

1. Inherit the repository-root AGENTS.MD; the user's current documentation-only request governs this directory.
2. These artifacts describe future implementation. Do not execute quoted commands or start migration work while editing/reviewing them.
3. Preserve source pins, exact names, attribution, one Goal and the separate alternation policy. Keep measurements truthful and credits unknown when unavailable.
4. Run only document inventory, quotation, mapping, link and assembly consistency review during authoring. No project build, compiler, generator, test, probe or proof run.
'''
    (ROAD/'AGENTS.MD').write_text('<a id="roadmap-guidance"></a>\n\n'+guidance+'\n## Source-derived project obligation\n\n'+replacements['@@PROJECT_QUOTE@@']+'\n**Derived instruction.** Treat CGP and derive-deftly as joint owners under the shared contract; preserve Rusternetes-only integration and source scope.\n')
    (DATA/'AGENTS.MD').write_text(guidance)
    comparison=ROOT/'cost-analysis-comparison'
    (comparison/'AGENTS.MD').write_text(guidance)
    (comparison/'README.md').write_text(COMPARISON_README)
    (ROOT/'cost-analysis.md').write_text(cost_report(author,forecasts,scenarios,schema,len(reuse['files'])))
    # Record explicit filename/module hazards rather than silently normalizing identifiers.
    destinations=collections.defaultdict(list)
    for row in author.rows:
        if row['destination'].startswith(('agones-rs/','cpg-rs/')):destinations[row['destination']].append(row['id'])
        reserved=set('as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try gen'.split())
        row['module_identifier_hazards']=[part for part in Path(row['destination']).parts if ((part.endswith('.rs') and (not re.match(r'^[A-Za-z_]\w*$',part[:-3]) or part[:-3] in reserved)) or part in reserved)]
        row['naming_policy']='EXACT_PATH; explicit registration outside mirror; identifier aliases do not rename files'
    collisions=[dict(destination=d,sources=ids) for d,ids in destinations.items() if len(ids)>1]
    if collisions:raise RuntimeError('Unresolved destination collisions: '+str(collisions))
    dump(DATA/'source-inventory.json',dict(date=DATE,purpose='Exhaustive pinned path accounting, not semantic completeness or verification',source_counts={'agones':len(arows),'cpg':len(crows)},detailed_count=sum(r['detailed'] for r in author.rows),files=author.rows))
    dump(DATA/'package-index.json',dict(packages=author.packages))
    dump(DATA/'milestones.json',dict(status='PLANNED_NOT_EXECUTED',policy='roadmap/alternation-policy.md',one_native_goal=True,dependency_method='Actual Go imports grouped into cohesive families; the CpgSchema/PropertyDefaults cycle collapsed into one milestone; pinned constructor sequence and explicit schema/pipeline/runtime prerequisites',family_aliases=ALIASES,milestones=milestones,planned_new_outputs=NEW_OUTPUTS))
    dump(DATA/'destination-collisions.json',dict(collisions=collisions,identifier_hazards=[dict(source=r['id'],destination=r['destination'],hazards=r['module_identifier_hazards']) for r in author.rows if r['module_identifier_hazards']]))
    source_records={}
    for name,r in REPOSITORIES.items():
        source_records[name]=dict(repository_url=r['url'],revision=r['revision'],local_root=str(r['root']),role='PINNED_SEMANTIC_BASELINE' if name in ('agones','cpg') else 'CURRENT_REFERENCE_CHECKOUT_NOT_AUTOMATIC_DEPENDENCY_UPGRADE',tree=git(CPG_GIT if name=='cpg' else r['root'],'rev-parse',r['revision']+'^{tree}').decode().strip(),license_paths=[str(p.relative_to(r['root'])) for p in r['root'].iterdir() if p.is_file() and any(t in p.name.upper() for t in ('LICENSE','COPYING','NOTICE'))])
    sources=dict(date=DATE,repositories=source_records,pipeline_pins={'cpg':CPG,'joern':'9f2171242e48d7fed3021087d45838a83c1e39da','goastgen':'11b21e395aa6aa39a41ca9216e1d15d08c4ef0b8','flatgraph':'61aa9de52231f95f9730b3a5aa7a3fb0d23fd128','genco':'c762252daa409545d08bf5aed27d63b801cb8f5c','grafeo_version':'0.5.44','rusternetes':'125c0f4a2b6fb888102606493c5e3cf7a126dd58'},style_priority=['AGENTS.MD','Tao: definitions and worked examples','Rustonomicon: explicit ownership and safety explanations','Too Many Linked Lists: progressive concrete explanation'],style_sources=[dict(path=str(p),sha256=sha(p.read_bytes()),role='PRESENTATION_REFERENCE_ONLY; no semantic or execution authority') for p in [Path('/home/astral/Documents/Terrence-Tao-Analysis-I.pdf'),Path('/home/astral/Documents/Rustonomicon.pdf'),Path('/home/astral/Documents/Learning Rust With Entirely Too Many Linked Lists.pdf')]],quote_policy='Preserve excerpts and license/NOTICE attribution; document source/comment fallback gaps; never execute quoted commands')
    dump(DATA/'sources.json',sources)
    dump(DATA/'source-quotes.json',dict(quotes=list(author.quotes.values())))
    index=['source_id\tmilestone\tdocument\tstart_line\tend_line\tanchor']
    for m in milestones:
        lines=(ROAD/m['filename']).read_text().splitlines()
        starts={line.split('id="',1)[1].split('"',1)[0]:i+1 for i,line in enumerate(lines) if line.startswith('<a id="file-')}
        positions=sorted(starts.values())
        for r in author.rows:
            if r['detailed'] and r['milestone_id']==m['id']:
                start=starts[r['entry_anchor']]
                after=next((n-1 for n in positions if n>start),len(lines))
                index.append('\t'.join(map(str,[r['id'],m['id'],'roadmap/'+m['filename'],start,after,r['entry_anchor']])))
    (DATA/'entry-index.tsv').write_text('\n'.join(index)+'\n')
    unresolved=[
      dict(id='U-SOURCE-BODIES',status='OPEN_IMPLEMENTATION_ACCOUNTING',owner=author.bykey['c-publication']['id'],gate='IMPLEMENTATION',detail='All paths have dispositions; lexical declarations do not exhaust nested closures, initializers, callbacks and configuration-qualified bodies. Each source owner must close these identities before production completion.'),
      dict(id='U-BUILD-VARIANTS',status='OPEN_IMPLEMENTATION_ACCOUNTING',owner=author.bykey['a-foundation']['id'],gate='IMPLEMENTATION',detail='Resolve Go build tags, platform/cgo variants, module/replacement/vendor configuration and corresponding body/resource scope from pinned source. No compiler probe is authorized.'),
      dict(id='U-REUSE-MAPPINGS',status='OPEN_IMPLEMENTATION_ACCOUNTING',owner='PER_FILE_IN_REUSE_INVENTORY',gate='IMPLEMENTATION',detail='245 current files have preservation/adoption-review owners. Exact source-to-Rust symbols, consumers and replacements remain to be reconciled; source/schema blob identity alone is not behavioral reuse credit.'),
      dict(id='U-EMITTER-AVAILABILITY',status='UNRESOLVED_EXECUTION_PREREQUISITE',owner=author.bykey['c-build']['id'],gate='IMPLEMENTATION',detail='No permitted already runnable complete emitter path was established by document authoring. If required emission needs a prohibited build, record the concrete dependency blocker; never bypass the gate.'),
      dict(id='U-ASSURANCE-INSTANCES',status='DEFERRED_UNTIL_GLOBAL_GATE',owner=[author.bykey['a-assurance']['id'],author.bykey['c-assurance']['id']],gate='FINAL_ASSURANCE',detail='Dynamic table/fixture-qualified assertions, model domains and actual Bolero/Kani/compatibility results remain unmeasured. Source path accounting is not assurance completion.'),
      dict(id='U-ACTUAL-COST',status='NO_IMPLEMENTATION_MILESTONES_EXECUTED',owner='roadmap/alternation-policy.md',gate='ACCOUNTING_NOT_EXECUTION_PERMISSION',detail='No attributable new milestone boundary pairs or credit charges exist. Preserve unknowns and provisional measurements; do not fabricate totals.'),
    ]
    dump(DATA/'unresolved-scope.json',dict(purpose='Explicit outstanding implementation/accounting obligations, not documentation defects or permission to execute',items=unresolved))
    dump(DATA/'assembly.json',assemble(ROOT))
    print(json.dumps(dict(milestones=len(milestones),source_files=len(author.rows),detailed_files=sum(r['detailed'] for r in author.rows),package_entries=len(author.packages),quotes=len(author.quotes),reusable_files=len(reuse['files']),scenario_totals={k:{v:x['total'] for v,x in values.items()} for k,values in scenarios['scenarios'].items()}),indent=2))

if __name__=='__main__':main()
