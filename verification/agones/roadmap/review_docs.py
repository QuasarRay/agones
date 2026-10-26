"""Document-only coverage, quotation, naming and assembly consistency review.

Does not import, build, execute or verify any Agones/CPG implementation.
"""
from pathlib import Path
from urllib.parse import unquote, urlsplit
import collections
import hashlib
import json
import re
import subprocess
import sys
from artifact_support import assemble

ROOT=Path(__file__).resolve().parents[3]
DATA=Path(__file__).resolve().parent

def read(name):return json.loads((DATA/name).read_text())
def digest(data):return hashlib.sha256(data).hexdigest()
def git(repo,*args):return subprocess.check_output(['git','-C',str(repo),*args])

def main():
    failures=[];checks=[]
    def check(condition,label,detail=None):
        checks.append(dict(check=label,result='PASS' if condition else 'FAIL',detail=detail))
        if not condition:failures.append(label)
    inv=read('source-inventory.json');rows=inv['files'];milestones=read('milestones.json')['milestones']
    byid={m['id']:m for m in milestones};quoted=read('source-quotes.json')['quotes'];sources=read('sources.json')['repositories']
    snapshots={repo:(ROOT/'codepropertygraph' if repo=='cpg' else ROOT, sources[repo]['revision']) for repo in ('agones','cpg')}
    for repo,(root,rev) in snapshots.items():
        original=set(git(root,'ls-tree','-r','--name-only','-z',rev).decode().strip('\0').split('\0'))
        recorded=[r['path'] for r in rows if r['repository']==repo]
        check(original==set(recorded) and len(recorded)==len(set(recorded)),f'{repo}: complete unique pinned-tree path inventory',len(recorded))
    firstgo=[r for r in rows if r['repository']=='agones' and not r['path'].startswith('vendor/') and r['path'].endswith('.go')]
    harnesses=[r for r in firstgo if r['path'].endswith('_test.go')]
    check(len(firstgo)==401 and len(harnesses)==84 and len({r['package'] for r in firstgo})==116,'Agones initial census reconciled with pinned tree')
    texts={m['id']:(ROOT/'roadmap'/m['filename']).read_text() for m in milestones}
    detail=[r for r in rows if r['detailed']]
    ownership_errors=[]
    for r in detail:
        own=byid.get(r['milestone_id'])
        if not own or texts[r['milestone_id']].count(f'<a id="{r["entry_anchor"]}"></a>')!=1:ownership_errors.append(r['id'])
        elif f'### File `{r["path"]}`' not in texts[r['milestone_id']]:ownership_errors.append(r['id'])
    check(not ownership_errors,'Each required source file has one explicit owned roadmap entry',ownership_errors)
    check(all(r['detailed'] for r in firstgo),'Every first-party Go file has a detailed entry')
    check(all(r['detailed'] for r in rows if r['repository']=='cpg'),'Every CPG path has a detailed entry')
    mapping_errors=[]
    for r in rows:
        d=r['destination'];p=r['path']
        if d.startswith('RETAIN_SOURCE_REFERENCE:'):continue
        expected=p[:-8]+'_harness.rs' if p.endswith('_test.go') else p[:-3]+'.rs' if p.endswith('.go') else p.rsplit('.',1)[0]+'.rs' if p.endswith(('.scala','.java')) else p
        if d!=('agones-rs/' if r['repository']=='agones' else 'cpg-rs/')+expected:mapping_errors.append(r['id'])
    check(not mapping_errors,'Exact path/case/stem mappings including Go harness exception',mapping_errors)
    check(not read('destination-collisions.json')['collisions'],'No exact destination collisions in frozen map')
    seen=set();dependency_errors=[]
    for i,m in enumerate(milestones):
        if not set(m['dependencies'])<=seen:dependency_errors.append(m['id'])
        if i and milestones[i-1]['route']==m['route']:dependency_errors.append(m['id']+': route repetition')
        seen.add(m['id'])
    check(not dependency_errors,'Topological milestone ordering and prescribed route alternation',dependency_errors)
    prod={m['id'] for m in milestones if m['phase']=='IMPLEMENTATION_ONLY'}
    check(all(prod<=set(m['dependencies']) for m in milestones if m['key'].endswith('assurance')),'Both assurance milestones depend on all production milestones')
    package_rows=read('package-index.json')['packages']
    check(set(r['id'] for r in detail)=={f for p in package_rows for f in p['files']},'Every detailed file is also owned by a package/module entry')
    quote_errors=[];quote_commit_mismatches=[];blobcache={}
    for q in quoted:
        key=(q['repository'],q['path'])
        if key not in blobcache:
            repo=sources[q['repository']]
            if q['repository'] in ('agones','cpg'):
                folder=ROOT/'codepropertygraph' if q['repository']=='cpg' else ROOT
                b=git(folder,'show',q['revision']+':'+q['path'])
            else:b=(Path(repo['local_root'])/q['path']).read_bytes()
            blobcache[key]=b
            if q['repository'] not in ('agones','cpg'):
                committed=git(Path(repo['local_root']),'show',q['revision']+':'+q['path'])
                if b!=committed:quote_commit_mismatches.append(key)
        b=blobcache[key]
        exact='\n'.join(b.decode('utf-8',errors='replace').splitlines()[q['start_line']-1:q['end_line']])
        if exact!=q['text'] or digest(b)!=q['source_sha256']:quote_errors.append(q['id'])
    check(not quote_errors,'Every quote matches exact recorded source bytes and line interval',quote_errors)
    check(not quote_commit_mismatches,'Current-reference quoted files match their permalink commit',quote_commit_mismatches)
    roadmap_files=sorted(p for p in (ROOT/'roadmap').iterdir() if p.suffix.lower()=='.md')
    check(all('**Direct quotation —' in p.read_text() for p in roadmap_files),'Every roadmap Markdown file contains attributed direct source quotation')
    check(all('CGP' in t and 'derive-deftly' in t and 'Python' in t for t in texts.values()),'Joint architectural ownership and Pythonic readability in every milestone')
    check(all(not re.search(r'@@[A-Z_]+@@','\n'.join(line for line in p.read_text().splitlines() if not line.startswith('>'))) for p in roadmap_files),'No unexpanded document template placeholders')
    forecasts=read('forecast-baseline-v1.json')['milestones']
    check({f['milestone_id'] for f in forecasts}==set(byid) and len(forecasts)==len(byid),'One immutable forecast ID for each milestone')
    before=(ROOT/'roadmap.md').read_bytes();assembly=assemble(ROOT)
    check(before==(ROOT/'roadmap.md').read_bytes(),'Combined roadmap equals deterministic assembly of every roadmap Markdown file')
    check({p.name for p in roadmap_files}=={Path(p['path']).name for p in assembly['order']},'Assembly includes all canonical Markdown components exactly once')
    check(all(v['status']=='PLANNED_NOT_EXECUTED' for v in milestones),'No implementation milestone is falsely recorded as executed')
    reports=[p.name for p in (ROOT/'cost-analysis-comparison').glob('*.md') if p.name not in ('README.md','AGENTS.MD')]
    check(not reports,'No fabricated historical actual-cost comparison reports',reports)
    links=[]
    prompt_info=read('prompt-reconciliation.json')
    docs=roadmap_files+[ROOT/'roadmap.md',ROOT/'cost-analysis.md',ROOT/'cost-analysis-comparison/README.md']+[Path(prompt_info[k]['path']) for k in ('agones-rs-prompt-v3.md','agones-rs-cpg-prompt-v1.md')]
    badlinks=[]
    for p in docs:
        for line in p.read_text().splitlines():
            if line.startswith('>'):continue
            for target in re.findall(r'\]\(([^\s)]+)\)',line):
                if target.startswith(('https:','http:','mailto:','#')):continue
                target=unquote(target.split('#',1)[0])
                if not target:continue
                path=Path(target) if target.startswith('/') else p.parent/target
                links.append(str(path))
                if not path.exists():badlinks.append(dict(document=str(p.relative_to(ROOT)) if ROOT in p.parents else str(p),target=target))
    check(not badlinks,'Authored local Markdown references resolve; quoted source links remain verbatim',badlinks)
    protected=read('authoring-inputs.json')
    changed=[p for p,h in protected['protected_files'].items() if digest((ROOT/p).read_bytes())!=h]
    check(not changed,'Original source, state, root prompts, Cargo manifest and checkpoint preserved',changed)
    check(digest(git(ROOT,'diff','--binary'))==protected['tracked_diff_sha256'],'No tracked implementation changes in Agones')
    for key in ('agones-rs-prompt-v3.md','agones-rs-cpg-prompt-v1.md'):
        text=Path(prompt_info[key]['path']).read_text()
        check('agones/roadmap/0000-contract.md' in text and 'agones/roadmap/alternation-policy.md' in text,key+': shared roadmap/policy entrypoint')
        forbidden=['then immediately stop','complete this generation Goal, and stop','Focused builds and synthetic fixtures for that machinery are permitted','Follow G5\'s more token/credit-efficient route','This recorded selection controls this contract']
        check(not any(x in text for x in forbidden),key+': obsolete selection, pre-check and terminal-stop clauses removed')
    result=dict(review_kind='DOCUMENT_ONLY',project_builds_tests_probes_proofs_or_verification_executed=False,status='PASS' if not failures else 'FAIL',counts=dict(source_paths=len(rows),detailed_file_entries=len(detail),package_entries=len(package_rows),milestones=len(milestones),unique_exact_quotes=len(quoted),roadmap_markdown_components=len(roadmap_files),resolved_local_link_occurrences=len(links)),checks=checks,limits=['Static documentation review does not prove semantic completeness, code correctness or executable tool availability.','External URL reachability was not exhaustively requested; permalink repository/revision/path identities were checked against local Git/source bytes.','Existing implementation adoption owners are provisional until exact source-to-symbol/consumer mappings are authored.','Implicit bodies, build/platform/configuration variants and dynamic test instances remain explicit source-accounting obligations.'])
    (DATA/'document-review.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(dict(status=result['status'],counts=result['counts'],failed_checks=failures,bad_local_links=badlinks,quote_commit_mismatches=quote_commit_mismatches),indent=2))
    if failures:sys.exit(1)

if __name__=='__main__':main()
