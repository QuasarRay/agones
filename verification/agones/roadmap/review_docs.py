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
from native_semantics import POLICY_ID, DISPOSITIONS

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
        key=(q['repository'],q['path'],q['revision'])
        if key not in blobcache:
            repo=sources[q['repository']]
            if q['revision']=='WORKTREE_UNCOMMITTED':
                b=Path(q['local_path']).read_bytes()
                if q.get('permalink') is not None:quote_commit_mismatches.append(key)
            else:
                folder=ROOT/'codepropertygraph' if q['repository']=='cpg' else Path(repo['local_root'])
                b=git(folder,'show',q['revision']+':'+q['path'])
            blobcache[key]=b
        b=blobcache[key]
        exact='\n'.join(b.decode('utf-8',errors='replace').splitlines()[q['start_line']-1:q['end_line']])
        if exact!=q['text'] or digest(b)!=q['source_sha256']:quote_errors.append(q['id'])
    check(not quote_errors,'Every quote matches exact recorded source bytes and line interval',quote_errors)
    check(not quote_commit_mismatches,'Committed quotations use exact Git bytes; uncommitted guidance has no fabricated commit permalink',quote_commit_mismatches)
    roadmap_files=sorted(p for p in (ROOT/'roadmap').iterdir() if p.suffix.lower()=='.md')
    check(all('**Direct quotation —' in p.read_text() for p in roadmap_files if p.name!='AGENTS.MD'),'Every roadmap document has attributed quotations; AGENTS.MD remains the exact authority copy')
    check(all('CGP' in t and 'derive-deftly' in t and 'Python' in t for t in texts.values()),'Joint architectural ownership and Pythonic readability in every milestone')
    check(all(not re.search(r'@@[A-Z_]+@@','\n'.join(line for line in p.read_text().splitlines() if not line.startswith('>'))) for p in roadmap_files),'No unexpanded document template placeholders')
    forecasts=read('forecast-baseline-v1.json')['milestones']
    check({f['milestone_id'] for f in forecasts}==set(byid) and len(forecasts)==len(byid),'One immutable forecast ID for each milestone')
    before=(ROOT/'roadmap.md').read_bytes();assembly=assemble(ROOT)
    check(before==(ROOT/'roadmap.md').read_bytes(),'Combined roadmap equals deterministic assembly of every roadmap Markdown file')
    check({p.name for p in roadmap_files}=={Path(p['path']).name for p in assembly['order']},'Assembly includes all canonical Markdown components exactly once')
    snapshot=read('native-rust-v1-inputs.json')
    state=json.loads((ROOT/'AGONES_RS_STATE.json').read_text());execution=state['delivery']['roadmap_execution']
    check(execution['cursor']==snapshot['execution_before']['cursor'],'M0003 cursor is preserved under the semantic amendment')
    check(all(execution[k]==v for k,v in snapshot['execution_before'].items() if k not in ('cursor',)),'Goal attachment, gate, phases and forecast identities are preserved')
    check(all(execution['milestones'][key]==snapshot['milestones_before'][key] for key in ('M0001','M0002')),'Completed M0001/M0002 records retain their historical contents')
    m3=execution['milestones']['M0003'];old_m3=snapshot['milestones_before']['M0003']
    check(all(m3.get(k)==v for k,v in old_m3.items()),'M0003 pre-amendment fields and usage observations remain intact')
    reports=[p for p in (ROOT/'cost-analysis-comparison').glob('*.md') if p.name not in ('README.md','AGENTS.MD')]
    check(all(str(p) in snapshot['protected_files'] and digest(p.read_bytes())==snapshot['protected_files'][str(p)] for p in reports),'Actual comparison reports are preserved without fabricated new completions',[p.name for p in reports])
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
                target=re.sub(r':\d+$','',target)
                path=Path(target) if target.startswith('/') else p.parent/target
                links.append(str(path))
                if not path.exists():badlinks.append(dict(document=str(p.relative_to(ROOT)) if ROOT in p.parents else str(p),target=target))
    check(not badlinks,'Authored local Markdown references resolve; quoted source links remain verbatim',badlinks)
    changed=[p for p,h in snapshot['protected_files'].items() if not Path(p).exists() or digest(Path(p).read_bytes())!=h]
    check(not changed,'All snapshotted production/dirty files, original prompts, Goal attachment, forecasts and measurements retain their bytes',changed)
    check(digest((ROOT/snapshot['history_archive']).read_bytes())==snapshot['history_archive_sha256'],'Pre-amendment history archive retains its original identity')
    guidance=(ROOT/'AGENTS.MD').read_bytes()
    check(all(p.read_bytes()==guidance for p in (ROOT/'roadmap/AGENTS.MD',DATA/'AGENTS.MD',ROOT/'verification/AGENTS.MD',ROOT/'verification/agones/AGENTS.MD',ROOT/'cost-analysis-comparison/AGENTS.MD')),'All affected guidance copies match current root AGENTS.MD exactly')
    # Coverage review of instructions, not execution/verification of any implementation.
    sem=read('semantic-obligations.json');obs=sem['obligations'];obids={o['id'] for o in obs}
    check(sem['policy']==POLICY_ID and set(o['disposition'] for o in obs)==set(DISPOSITIONS),'All three native/domain/superseded obligation dispositions are represented')
    check(all(set(r.get('semantic_obligation_ids',[]))<=obids and len(r.get('semantic_obligation_ids',[]))>=2 for r in detail if r['repository']=='agones'),'Every required Agones file has native and domain obligation records')
    check(all(POLICY_ID in text and 'Existing Rust ecosystem facilities' in text for text in texts.values()),'Every milestone applies native Rust ecosystem adoption and the semantic boundary')
    check(all(o.get('authority')==POLICY_ID and o['status'].startswith('NOT_A_COMPLETION_BLOCKER') for o in obs if o['disposition']=='GO_ONLY_SUPERSEDED'),'Go-only records are explicitly superseded rather than unresolved emulation blockers')
    active_paths=roadmap_files+[ROOT/'AGONES_RS_CONTINUATION_PROMPT.md',ROOT/'AGONES_CPG_FIRST_GENERATION_GOAL_PROMPT.md']+[Path(prompt_info[k]['path']) for k in ('agones-rs-prompt-v3.md','agones-rs-cpg-prompt-v1.md')]
    banned=['Implement all required Go semantic families','Preserve channel/select outcomes','Preserve source panic/recover boundaries','Preserve source floating width','Retain deferred cleanup order','Preserve original widths, nil/empty','goroutines/channels/select, context/clocks','Cover the source\'s actual types, expressions/statements, evaluation order, mutation/aliasing','Match the original behavior for evaluation order/count']
    contradictions=[]
    for p in active_paths:
        for i,line in enumerate(p.read_text().splitlines(),1):
            if not line.startswith('>') and any(x in line for x in banned):contradictions.append(dict(path=str(p),line=i,text=line))
    check(not contradictions,'Active instructions contain no superseded blanket Go-emulation requirements',contradictions)
    # Construct prompt text in memory only; do not run the prompt updater or project generators.
    import refine_prompts
    from native_semantics import rewrite_prompt
    future=[rewrite_prompt(refine_prompts.manual()),rewrite_prompt(refine_prompts.cpg())]
    future_banned=banned+['An approximation must still implement the source operation concretely.','Translate source types, declarations, control/data flow, effects, and public contracts']
    check(not any(any(term in text for term in future_banned) for text in future),'Prompt authoring transformations cannot restore obsolete source/runtime obligations')
    partial=json.loads((ROOT/'verification/agones/roadmap-M0003-partial-map.json').read_text())
    check(partial.get('semantic_policy')==POLICY_ID and partial['milestone_complete'] is False and all(x['disposition']=='AGONES_REQUIRED' for x in partial['known_remaining']),'M0003 remains incomplete with explicit Agones-only active gaps')
    check(all(execution[k+'_sha256']==digest((ROOT/execution[k]).read_bytes()) for k in ('contract','alternation_policy','combined_roadmap')),'Current contract, policy and combined hashes match the amended documents')
    check(all(digest((ROOT/p).read_bytes())==h for p,h in execution['current_record_bindings'].items()),'Current control-record bindings match amended records')
    check(execution.get('semantic_policy')==POLICY_ID and 'GO_ONLY_SUPERSEDED' in m3.get('go_only_obligations',''),'The canonical execution ledger retires Go-only completion blockers')
    for key in ('agones-rs-prompt-v3.md','agones-rs-cpg-prompt-v1.md'):
        text=Path(prompt_info[key]['path']).read_text()
        check('agones/roadmap/0000-contract.md' in text and 'agones/roadmap/alternation-policy.md' in text,key+': shared roadmap/policy entrypoint')
        forbidden=['then immediately stop','complete this generation Goal, and stop','Focused builds and synthetic fixtures for that machinery are permitted','Follow G5\'s more token/credit-efficient route','This recorded selection controls this contract']
        check(not any(x in text for x in forbidden),key+': obsolete selection, pre-check and terminal-stop clauses removed')
    result=dict(review_kind='DOCUMENT_ONLY',semantic_policy=POLICY_ID,project_builds_tests_probes_proofs_or_verification_executed=False,status='PASS' if not failures else 'FAIL',counts=dict(source_paths=len(rows),detailed_file_entries=len(detail),package_entries=len(package_rows),milestones=len(milestones),unique_exact_quotes=len(quoted),roadmap_markdown_components=len(roadmap_files),resolved_local_link_occurrences=len(links),semantic_obligations=len(obs),protected_files=len(snapshot['protected_files'])),checks=checks,limits=['Static documentation review does not prove implementation or semantic correctness.','External URL reachability was not exhaustively requested; quotations were matched to pinned Git bytes or explicitly attributed working-tree guidance.','Native adoption and domain refinement remain implementation work; no code migration or verification credit follows from superseding Go-only obligations.','Historical original forecasts and actual usage remain under their original scope.'])
    (DATA/'document-review.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(dict(status=result['status'],counts=result['counts'],failed_checks=failures,bad_local_links=badlinks,quote_commit_mismatches=quote_commit_mismatches),indent=2))
    if failures:sys.exit(1)

if __name__=='__main__':main()
