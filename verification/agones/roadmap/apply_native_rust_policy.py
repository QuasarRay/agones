"""Apply the authorized documentation/control amendment, never production code.

The frozen inventory, source quotations, execution cursor, measurements and
forecast baseline survive this amendment. --refresh-roadmap only renders docs.
"""
from pathlib import Path
import ast
import gzip
import hashlib
import json
import pprint
import re
import subprocess
from native_semantics import (ROOT,DATA,POLICY_ID,DISPOSITIONS,rewrite_prose,
                              shared_contract_section,guidance_quote,amendment_next_action,rewrite_prompt)

def digest(b):return hashlib.sha256(b).hexdigest()
def read(path):return json.loads(Path(path).read_text())
def save(path,data):Path(path).write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n')

def revise_templates():
    p=DATA/'milestone_content.py';source=p.read_text();parsed=ast.parse(source)
    values={}
    for node in parsed.body:
        if isinstance(node,ast.Assign) and len(node.targets)==1 and isinstance(node.targets[0],ast.Name) and node.targets[0].id in ('A','C'):
            values[node.targets[0].id]=(node,ast.literal_eval(node.value))
    a=values['A'][1];c=values['C'][1]
    for k,v in list(a.items()):a[k]=(v[0],v[1],[rewrite_prose(x).strip() for x in v[2]])
    a['a-utilities']=(a['a-utilities'][0],a['a-utilities'][1],[
      'Implement Agones error classes, event/log fields, readiness and CRD behavior, admission outcomes, queue retry/backoff and shutdown. Identify domain requirements from the quoted Agones utility files; source dependencies are references rather than additional Go porting objectives.',
      'Use existing Rust error/formatting/logging, HTTP/TLS, serialization, filesystem notification, signal, timer and async queue/task facilities through CGP. Prefer selected native implementations and thin Agones adapters; do not reconstruct Go standard-library/runtime behavior.',
      'Keep one lifecycle/resource owner and derive-deftly declarations for repeated wiring. Retire Go-only formatting/reflection/typed-nil/panic/channel/map/slice/GC/allocator/ABI gaps explicitly. Preserve unresolved Agones contracts; M0003 remains implementing until native production consumers are complete.'])
    a['a-api']=(a['a-api'][0],a['a-api'][1],[
      a['a-api'][2][0],
      'Implement Agones constructors, defaults, validation, state transitions and resource/schema support using Rust types, serde/protobuf bindings and native ownership. Preserve field presence, ranges and errors only where required by an Agones API/schema contract; never reproduce Go nil/interface or copy/runtime mechanics.',
      a['a-api'][2][2]])
    a['a-assurance']=(a['a-assurance'][0],a['a-assurance'][1],[
      'POST-GATE ONLY. Retain every original source::fixture::test identity. Preserve independent Agones cases, setup/actions/observations and invariants; classify individual Go-only assertions GO_ONLY_SUPERSEDED with source evidence and requirement-10 authority. Go name_test.go maps to name_harness.rs.',
      'Use typed extraction, independent Agones domains and non-vacuity witnesses. Generate Bolero properties with the required Kani backend and add appropriate Rust-native coverage. Keep sampling, reference/production proofs, replay and Rusternetes evidence distinct; no Go runtime equivalence campaign.',
      'Bind assertions to real production consumers and required Agones API/domain outcomes. Unknown Agones obligations remain open; superseded Go mechanisms do not block acceptance. Production defects reopen the implementation gate before further assurance.'])
    for k,v in list(c.items()):c[k]=(v[0],[rewrite_prose(x,scope='cpg').strip() for x in v[1]])
    c['c-utilities']=(c['c-utilities'][0],[
      'Implement CPG-required file/stream/encoding behavior, temporary-copy cleanup, root discovery and graph interning through existing native Rust facilities. Preserve CPG functional/interchange contracts without implementing a Scala/JVM or Go runtime.',
      c['c-utilities'][1][1],
      'Reuse the Rust standard library and selected ecosystem utilities before custom code. Record the concrete CPG contract/integration gap for any adapter; keep valid Rust concise for Python readers.'])
    c['c-ir']=(c['c-ir'][0],[
      'Define a native-Rust-oriented typed IR for Agones domain values, decisions, calls, effects, configuration and lifecycle operations. Bind each node to source/body/configuration and rule identities; distinguish AGONES_REQUIRED, RUST_NATIVE and GO_ONLY_SUPERSEDED obligations.',
      'Map Agones public contracts and Rusternetes capabilities to real native Rust destination interfaces and library operations. Keep original Go AST/type facts as provenance and analysis data; they do not prescribe target runtime behavior.',
      'Required Agones operations must have real implementations. Excluded Go-only mechanisms retain supersession records, not unsupported-operation blockers. Genco emits the typed native Rust plan; CGP and derive-deftly own stages and repeated structure.'])
    c['c-lowering-data']=('Native Rust domain values, types and initialization',[
      'Lower Agones resource/configuration values to Rust structs/enums, native collections, Option/Result and existing serializers. Preserve API/protobuf field names, ranges, presence and domain arithmetic only where required by Agones contracts.',
      'Implement Agones startup/configuration and external-call requirements using native Rust initialization, ownership, checked conversions and selected libraries. Do not emulate Go integers/floating formatting, typed-nil interfaces, reflection, aliases, map/slice headers, allocation or ABI behavior.',
      'Maintain source-to-Agones-obligation-to-Rust-operation provenance. Generate readable Python-familiar valid Rust and reusable derive-deftly declarations; Go-only constructs receive explicit supersession rather than compatibility helpers.'])
    c['c-lowering-control']=('Native Rust decisions, errors, concurrency and lifecycle',[
      'Lower Agones decisions and failure outcomes into Rust control flow, Result and native library operations. Use RAII/scoped guards for resources and explicit async shutdown where Agones requires it.',
      'Use existing Rust tasks, channels/streams, synchronization, clocks and cancellation through CGP-owned services. Preserve Agones retry, readiness, notification and shutdown contracts; never emulate goroutine scheduling, channel/select mechanics, defer/panic/recover or Goexit.',
      'Generate the same native lifecycle architecture used by manual Agones. Unknown Agones requirements remain gaps; Go runtime details are GO_ONLY_SUPERSEDED. Keep every required domain operation connected to a real provider and native production consumer.'])
    c['c-assurance']=(c['c-assurance'][0],[c['c-assurance'][1][0],
      'Establish native CPG graph/schema/persistence/loading/pass/interchange evidence. Validate generated Agones against independent domain invariants and API observations, with Rust-native assurance for infrastructure. Retain source test IDs but supersede Go-only assertions explicitly.',
      c['c-assurance'][1][2]])
    # Edit the actual declaration tables so even direct template readers see the new rules.
    lines=source.splitlines(keepends=True)
    for name,(node,_) in sorted(values.items(),key=lambda item:item[1][0].lineno,reverse=True):
        val=a if name=='A' else c
        lines[node.lineno-1:node.end_lineno]=[name+' = '+pprint.pformat(val,width=115,sort_dicts=False)+'\n']
    p.write_text(''.join(lines))
    # Shared text is authored from the same policy; no full baseline renderer runs.
    p=DATA/'roadmap_text.py';t=rewrite_prose(p.read_text())
    t=t.replace('3. Implement all required Go semantic families, including initialization, generics, interfaces, aliasing, numeric conversions, strings/maps/slices, errors, defer/panic/recover, goroutines/channels/select, context/clocks and external calls. Retain unresolved facts and unsupported operations as required gaps.',
       '3. Translate every required Agones domain operation into native Rust types, control flow, errors, library calls and CGP lifecycle capabilities. Preserve source AST/types as input facts. Classify Go-only language/runtime mechanisms as superseded; do not generate a Go compatibility runtime.')
    t=t.replace('4. Every required body needs a substantive source-derived implementation and operation/support mapping. A declaration, comment, missing helper, success constant or unsupported-operation placeholder is insufficient. Trace legitimate source panic/no-op/error behavior separately from omissions. Record approximations without claiming correctness.',
       '4. Every required Agones operation needs a substantive implementation or real native Rust library operation with source mapping and a production consumer. Preserve independently evidenced domain failures/no-ops using Rust error policy. Retain Go-only body/operation identities with explicit supersession; declarations, comments, missing helpers and fabricated success cannot discharge Agones obligations.')
    t=t.replace('All milestones are planned, not executed.','Milestone definitions retain their IDs/order; actual execution and completion records remain in the canonical ledger.')
    t=t.replace('3. Preserve deprecated-but-served behavior and all source configurations. Required production scope includes initializers, closures/callbacks, generated source, API/schema/proto data, configuration, resources, build/tool behavior and external-call support. Enumerate Go build-tag/platform/cgo/module/replacement/vendor variants before claiming a complete body universe. Existing dynamic test/assertion counts remain unresolved until post-gate extraction.',
       '3. Preserve deprecated-but-served Agones contracts. Account for Agones initialization, callbacks, generated bindings, API/schema/protocol data, resources, tool outputs and external effects. Retain Go build-tag/platform/cgo/module/vendor facts as source-analysis inputs; map required Agones target/features to native Rust facilities. This is not a Go compiler/runtime/ABI profile objective. Dynamic Agones test/assertion accounting remains deferred until the global gate.')
    t=t.replace('No implementation milestone was executed while authoring this roadmap. This directory therefore contains instructions, not fabricated actual-cost reports.','The initial roadmap was authored on 27 October 2026. Existing milestone reports now record real execution; preserve those measurements and their original forecasts. New reports require attributable observations.')
    if 'from native_semantics import shared_contract_section' not in t:
        t='from native_semantics import shared_contract_section\n'+t
    if 'CONTRACT = CONTRACT.replace' not in t:
        t+='\n# Apply the current user-authorized semantic contract in every rendering mode.\nCONTRACT = CONTRACT.replace("## 2. One Goal", shared_contract_section()+"## 2. One Goal", 1)\n'
    p.write_text(t)
    # Remove obsolete Go-specific trait rules from the renderer itself.
    p=DATA/'build_docs.py';t=p.read_text();start=t.index('        rules=[',t.index('    def traits(self,row):'));end=t.index('\n',start)
    t=t[:start]+"        rules=[(r'Float|Double','Preserve CPG schema/interchange scalar representations with native Rust types and serializers.'),(r'HashMap','Use native Rust collections while preserving required CPG graph key/edge contracts.'),(r'cancel|shutdown|Close','Keep native Rust resources, cancellation and failure cleanup under CGP.'),(r'newProperty|addProperty|addNodeType|addEdgeType','Retain CPG declaration names, defaults, cardinalities, IDs, relationships and shared builder identity.')]"+t[end:]
    t=t.replace('Preserve original widths, nil/empty distinctions and observable sequencing; do not substitute hand-copied output for CPG provenance.','Preserve Agones-required API ranges/presence and domain effects using Rust types and native serialization; do not substitute hand-copied output for CPG provenance.')
    t=t.replace('through control-flow and defer lowering','through native Rust control flow, owned tasks and RAII cleanup')
    # Initial authoring mode also copies exact guidance and applies the semantic section.
    if "    guidance='''# AGENTS.MD" in t:
        a=t.index("    guidance='''# AGENTS.MD");b=t.index("    comparison=ROOT/'cost-analysis-comparison'",a)
        t=t[:a]+"    guidance=(ROOT/'AGENTS.MD').read_text()\n    (ROAD/'AGENTS.MD').write_text(guidance)\n    (DATA/'AGENTS.MD').write_text(guidance)\n"+t[b:]
    p.write_text(t)

def refresh_roadmap():
    """Render documentation from frozen inputs without regenerating forecasts/state."""
    import build_docs as b
    from roadmap_text import CONTRACT,POLICY
    from artifact_support import assemble,NEW_OUTPUTS
    inventory=read(DATA/'source-inventory.json');manifest=read(DATA/'milestones.json')
    milestones=manifest['milestones'];rows=inventory['files']
    source_records=read(DATA/'sources.json')
    for name,repo in b.REPOSITORIES.items():
        if name in source_records['repositories']:repo['revision']=source_records['repositories'][name]['revision']
    a=[r for r in rows if r['repository']=='agones'];c=[r for r in rows if r['repository']=='cpg']
    for records in (a,c):
        for r in records:r['git_type']='blob'
    asrc=b.contents(ROOT,[r for r in a if not r['path'].startswith('vendor/')]);csrc=b.contents(b.CPG_GIT,c)
    author=b.Author(a,c,asrc,csrc,milestones)
    # Historical committed guidance is read at its recorded revision, not relabeled as current.
    historic=b.git(ROOT,'show',b.REPOSITORIES['project-guidance']['revision']+':AGENTS.MD')
    author.sources['project-guidance']={'AGENTS.MD':historic}
    bykey={m['key']:m for m in milestones}
    title_changes={'c-lowering-data':'Native Rust domain values, types and initialization','c-lowering-control':'Native Rust decisions, errors, concurrency and lifecycle'}
    for m in milestones:
        path=ROOT/'roadmap'/m['filename'];old=path.read_text()
        if m['key'] in title_changes:m['title']=title_changes[m['key']]
        extras=re.search(r'(?m)^## (?:Constructor order[^\n]*|Native schema reuse boundary|Upstream platform contract reference|Specific source obligations for this new stage)',old)
        tail=old[extras.start():] if extras else ''
        tail=tail.replace('Earlier schema milestones implement the referenced constructors before this complete composition.','The complete mutually dependent schema remains one cohesive milestone.')
        tail=tail.replace('Preserve original widths, nil/empty distinctions and observable sequencing','Preserve Agones-required API ranges, presence and domain effects using native Rust types and serialization')
        tail=tail.replace('through control-flow and defer lowering','through native Rust control flow, owned tasks and RAII cleanup')
        tail=tail.replace('Any desugaring must retain source effects, order and observable outcomes','Any domain transformation retains Agones-required effects and outcomes while adopting native Rust language semantics')
        path.write_text(author.milestone(m)+'\n'+rewrite_prose(tail))
    quote,current_quote=guidance_quote()
    replacements={
      '@@PROJECT_QUOTE@@':quote,'@@CGP_QUOTE@@':author.quote('cgp','README.md',9,15,'UPSTREAM_DOCUMENTATION'),
      '@@DEFTLY_QUOTE@@':author.quote('derive-deftly','README.md',6,15,'UPSTREAM_DOCUMENTATION'),
      '@@COMPRENDE_QUOTE@@':author.quote('comprende','README.md',14,29,'UPSTREAM_DOCUMENTATION'),
      '@@BETTER_QUOTE@@':author.quote('better-comprehension-in-rust','README.md',30,46,'UPSTREAM_DOCUMENTATION'),
      '@@AGONES_COUNT@@':str(len(a)),'@@CPG_COUNT@@':str(len(c)),'@@DETAIL_COUNT@@':str(sum(r['detailed'] for r in rows)),
      '@@MILESTONE_TABLE@@':'| Milestone | Route | Prerequisites | Work |\n|---|---|---|---|\n'+'\n'.join(f'| [{m["id"]}]({m["filename"]}) | {m["route"]} | '+(', '.join(m['dependencies']) or 'Contract')+f' | {m["title"]} |' for m in milestones)}
    for name,text in [('0000-contract.md',CONTRACT),('alternation-policy.md',POLICY)]:
        for old,new in replacements.items():text=text.replace(old,new)
        if name.startswith('0000') and '## Native Rust policy — AGONES_NATIVE_RUST_V1' not in text:
            text=text.replace('## 2. One Goal',shared_contract_section()+'## 2. One Goal',1)
        elif not name.startswith('0000'):text+='\n## Contract amendments and source scope\n\nApply AGONES_NATIVE_RUST_V1 from the shared contract. Completed milestone history and actual measurements remain intact. GO_ONLY_SUPERSEDED obligations do not block completion; unresolved Agones obligations do. This corrects scope without creating a route-switch event or replacing the native Goal.\n'
        (ROOT/'roadmap'/name).write_text(rewrite_prose(text))
    root_guidance=(ROOT/'AGENTS.MD').read_bytes()
    for p in (ROOT/'roadmap/AGENTS.MD',DATA/'AGENTS.MD',ROOT/'verification/AGENTS.MD',ROOT/'verification/agones/AGENTS.MD',ROOT/'cost-analysis-comparison/AGENTS.MD'):p.write_bytes(root_guidance)
    inventory['semantic_policy']=POLICY_ID;save(DATA/'source-inventory.json',inventory)
    save(DATA/'package-index.json',dict(packages=author.packages))
    manifest['semantic_policy']=POLICY_ID;manifest['status']='DEFINITIONS_WITH_LIVE_EXECUTION_IN_CANONICAL_LEDGER';manifest['planned_new_outputs']=NEW_OUTPUTS
    save(DATA/'milestones.json',manifest)
    original_quotes=read(DATA/'source-quotes.json');merged={q['id']:q for q in original_quotes['quotes']}
    merged.update(author.quotes);merged[current_quote['id']]=current_quote
    original_quotes['quotes']=list(merged.values());save(DATA/'source-quotes.json',original_quotes)
    source_records['repositories']['project-guidance-current']=dict(repository_url='https://github.com/QuasarRay/agones',revision='WORKTREE_UNCOMMITTED',local_root=str(ROOT),source_sha256=digest(root_guidance),role='CURRENT_USER_AUTHORITY; historical committed references retained separately',base_revision=b.git(ROOT,'rev-parse','HEAD').decode().strip(),permalink=None)
    source_records['semantic_policy']=POLICY_ID;save(DATA/'sources.json',source_records)
    save(DATA/'semantic-obligations.json',dict(policy=POLICY_ID,authority_quote=current_quote['id'],classification_unit='Individual obligation within a source file/body; mixed files retain all Agones requirements',dispositions=DISPOSITIONS,obligations=getattr(author,'semantic_obligations',[]),coverage_status='SOURCE_ACCOUNTING_NOT_ACCEPTANCE; domain refinements remain implementation work'))
    index=['source_id\tmilestone\tdocument\tstart_line\tend_line\tanchor']
    for m in milestones:
        lines=(ROOT/'roadmap'/m['filename']).read_text().splitlines();starts={l.split('id="',1)[1].split('"',1)[0]:i+1 for i,l in enumerate(lines) if l.startswith('<a id="file-')};positions=sorted(starts.values())
        for r in rows:
            if r['detailed'] and r['milestone_id']==m['id']:
                start=starts[r['entry_anchor']];end=next((n-1 for n in positions if n>start),len(lines))
                index.append('\t'.join(map(str,[r['id'],m['id'],'roadmap/'+m['filename'],start,end,r['entry_anchor']])))
    (DATA/'entry-index.tsv').write_text('\n'.join(index)+'\n')
    save(DATA/'assembly.json',assemble(ROOT))

GAPS={
 'M0003-APISERVER':['Finish Agones discovery/resource routes, required content negotiation and supported API encodings using selected Rust server/serialization facilities.'],
 'M0003-FSWATCH':['Adopt a native Rust watcher and connect required certificate/configuration notifications, errors and shutdown to the production CGP owner.'],
 'M0003-HTTP-HTTPS':['Finish Agones endpoints, mTLS/certificate reload, configured timeouts, streaming and graceful drain through selected native HTTP/TLS libraries.'],
 'M0003-RUNTIME':['Complete native Rust service/task ownership, supervision, failure reporting, cancellation and cleanup required by Agones.'],
 'M0003-SIGNALS':['Connect Rust-native signal handling to Agones graceful shutdown and join/cleanup policy.'],
 'M0003-WEBHOOKS':['Complete Agones admission request/response, UID, allow/deny, patch and domain-error contracts using native serializers and HTTP services.'],
 'M0003-WORKQUEUE':['Complete Agones queue key/retry/backoff/readiness/shutdown policy through native Rust task/queue capabilities.'],
 'M0003-ERROR-METADATA-FORMATTING':['Preserve Agones error classification, API status, required messages and log fields through typed Rust errors, std::fmt and Rust logging facilities.'],
 'M0003-FEATURE-INPUT-CONSUMERS':['Connect every Agones feature/configuration input to its real native Rust production consumer.'],
 'M0003-CRD-ADOPTION':['Complete Agones resource registration/readiness and schema consumers through Rusternetes and native Rust capabilities.'],
 'M0003-WHOLE-BODY-RECONCILIATION':['Map all required Agones utility operations to native Rust implementations/library calls with actual consumers; retain per-operation supersession for Go-only mechanisms.'],
 'M0003-HTTP-BOUNDARY-PROFILES':['Resolve Agones-required request/response/authentication/timeouts/cancellation behavior at native Rust HTTP boundaries; HTTP library internals are not a separate Go port.'],
 'M0003-GATEWAY':['Complete Agones gateway routes, SDK/protobuf/JSON fields and required error/status/presence mappings through selected native Rust bindings.'],
}

def revise_execution_records():
    snapshot=read(DATA/'native-rust-v1-inputs.json')
    paths=sorted((ROOT/'verification/agones').glob('roadmap-M0003-*.json'))
    record_changes=[]
    for path in paths:
        if 'usage' in path.name:continue
        data=read(path)
        if data.get('semantic_policy')==POLICY_ID:continue
        before=digest(path.read_bytes());retired=[]
        def walk(value,pointer=''):
            if isinstance(value,dict):
                for key in list(value):
                    sub=pointer+'/'+key
                    if key in ('remaining','known_remaining','required','next_action'):
                        old=value.pop(key);value['historical_'+key+'_before_native_rust_v1']=old
                        retired.append(dict(pointer=sub,source_text_sha256=digest(json.dumps(old,ensure_ascii=False,sort_keys=True).encode()),disposition='MIXED_REQUIREMENTS_REPLACED_BY_CANONICAL_AGONES_GAPS',authority=POLICY_ID))
                        if key=='remaining':value[key]=['Use the current M0003 Agones-required gap list in roadmap-M0003-partial-map.json. Review existing native library adoption and consumers; Go-only reconstruction is superseded.']
                    elif not key.startswith('historical_'):walk(value[key],sub)
            elif isinstance(value,list):
                for i,item in enumerate(value):walk(item,pointer+'/'+str(i))
        walk(data)
        data['semantic_policy']=POLICY_ID
        data['semantic_scope']='Existing authored operations/source profiles are historical implementation descriptions. Only current Agones-required gaps and native Rust adoption remain active; no Go runtime or language equivalence obligation survives.'
        data['semantic_supersession']=retired
        data['existing_compatibility_code']='PRESERVE_EXISTING_CODE; inspect consumers and reuse, replace or retire it during implementation. No Rust source was changed by this amendment.'
        if path.name=='roadmap-M0003-partial-map.json':
            previous=data['historical_known_remaining_before_native_rust_v1']
            original_ids=[x['id'] for x in previous]
            if set(original_ids)!=set(GAPS):raise RuntimeError('Unrecognized M0003 gap identities; preserve and reconcile explicitly.')
            data['known_remaining']=[dict(id=i,semantic_policy=POLICY_ID,disposition='AGONES_REQUIRED',required=GAPS[i],native_implementation='RUST_NATIVE: selected standard-library/ecosystem facility through CGP; derive-deftly supplies repeated wiring',go_only_disposition='GO_ONLY_SUPERSEDED: no Go language/runtime reconstruction',status='OPEN; REQUIRES_NATIVE_CONSUMER_RECONCILIATION') for i in original_ids]
            data['milestone_complete']=False
        # File digests are current identity bookkeeping; historical originals are archived.
        def bind(v):
            if isinstance(v,dict):
                for k,x in v.items():
                    if k.startswith('historical_'):continue
                    if isinstance(x,dict) and ('sha256' in k or k=='root_binding'):
                        for rel in list(x):
                            q=ROOT/rel
                            if isinstance(x[rel],str) and q.is_file() and q.suffix.lower() in ('.md','.json'):x[rel]=digest(q.read_bytes())
                    else:bind(x)
            elif isinstance(v,list):
                for x in v:bind(x)
        bind(data);save(path,data);record_changes.append(dict(path=str(path.relative_to(ROOT)),before_sha256=before,after_sha256=digest(path.read_bytes()),superseded_requirement_groups=len(retired)))
    state_path=ROOT/'AGONES_RS_STATE.json';state=read(state_path);e=state['delivery']['roadmap_execution']
    if e.get('semantic_policy')!=POLICY_ID:
        e['historical_next_action_before_native_rust_v1']=e['next_action']
        e['semantic_policy']=POLICY_ID;e['next_action']=amendment_next_action()
        e['contract_amendment']='verification/agones/roadmap/native-rust-policy-amendment.md'
        e['contract_amendment_authority']='Current root AGENTS.MD requirement 10 and the explicit 30 October 2026 user-approved plan; immutable original Goal attachment is historical where superseded.'
        state['delivery']['historical_next_action_before_native_rust_v1']=state['delivery'].get('next_action')
        state['delivery']['next_action']=amendment_next_action()
        current=e['milestones']['M0003'];current['semantic_policy']=POLICY_ID
        current['active_completion_criteria']=[v for values in GAPS.values() for v in values]
        current['go_only_obligations']='GO_ONLY_SUPERSEDED; no Go runtime/language/stdlib reconstruction is a completion blocker.'
        current['native_reconciliation_record']='verification/agones/roadmap-M0003-partial-map.json'
        if isinstance(state.get('revision'),int):state['revision']+=1
        save(state_path,state)
    return record_changes

def update_prompts():
    info=read(DATA/'prompt-reconciliation.json')
    paths=[ROOT/'AGONES_RS_CONTINUATION_PROMPT.md',ROOT/'AGONES_CPG_FIRST_GENERATION_GOAL_PROMPT.md']+[Path(info[k]['path']) for k in ('agones-rs-prompt-v3.md','agones-rs-cpg-prompt-v1.md')]
    changed=[]
    for p in paths:
        old=p.read_text();t=rewrite_prose(old)
        if '## Native Rust policy — AGONES_NATIVE_RUST_V1' not in t:
            pos=t.find('\n',t.find('\n')+1)
            t=t[:pos+1]+'\n'+shared_contract_section()+t[pos+1:]
        lines=[]
        for line in t.splitlines():
            if line.startswith('Build a construct/support inventory from **all** required source bodies'):
                line='Build a source-bound Agones obligation inventory before emission. Preserve Go AST/type facts as analysis inputs, then map domain operations and required API constraints to native Rust types, errors, control flow and selected library calls. Classify each obligation AGONES_REQUIRED, RUST_NATIVE or GO_ONLY_SUPERSEDED. Go formatting/reflection, typed-nil, panic/recover/Goexit, goroutine/channel/select, map/slice internals, GC, allocator and ABI emulation are excluded.'
            elif line.startswith('Every required body must have a substantive source-derived Rust implementation.'):
                line='Every required Agones operation must have a substantive native Rust implementation or library operation with a real production consumer and source mapping. Callbacks and nested domain decisions remain accounted for. Retain source IDs for Go-only mechanisms with explicit supersession. Missing domain behavior, empty providers, absent helpers and fabricated success are incomplete; a real native library call implementing the contract is valid reuse.'
            elif 'Match the original behavior for evaluation order/count' in line:
                line=re.sub(r'Match the original behavior for evaluation order/count.*?duplicate-key handling\.', 'Use Rust-native evaluation, ownership, borrowing, allocation and collection behavior. Preserve ordering, duplicates, errors and effects only where an Agones contract requires them.',line)
            elif line.startswith('| Semantic lowering |'):
                line='| Semantic lowering | Recover Agones domain operations and required API constraints, map them to Rust-native IR/library operations, and retain explicit Go-only supersession provenance. |'
            line=line.replace('Implement explicit CGP stages and typed interfaces, Go semantic lowering, Genco emission, required support','Implement CGP stages and typed interfaces, Agones-domain lowering into native Rust, Genco emission and native integration')
            line=line.replace('Keep source semantics in reviewable rules/IR','Keep Agones contracts and native Rust implementation choices in reviewable rules/IR')
            line=line.replace('An approximation must still implement the source operation concretely. An unimplemented construct is a coverage gap, not an approximation.','An approximation cannot omit an Agones domain operation. Native Rust replacements for Go-only mechanisms are required semantic changes with supersession records, not unsupported constructs or missing runtime helpers.')
            line=line.replace('Compilation errors and behavioral flaws do not extend this goal.','Generation alone remains unverified; the shared combined-roadmap invocation continues through its production, assurance and delivery requirements.')
            lines.append(line)
        t='\n'.join(lines)+'\n';p.write_text(t)
        changed.append(dict(path=str(p),before_sha256=digest(old.encode()),after_sha256=digest(t.encode())))
        for key in ('agones-rs-prompt-v3.md','agones-rs-cpg-prompt-v1.md'):
            if str(p)==info[key]['path']:
                info[key]['native_policy_before_sha256']=digest(old.encode());info[key]['revised_sha256']=digest(t.encode());info[key]['revised_bytes']=len(t.encode())
    info['semantic_policy']=POLICY_ID;info['native_rust_amendment']=changed;save(DATA/'prompt-reconciliation.json',info)
    # Future prompt authoring also applies the same amendment, preserving original snapshots.
    p=DATA/'refine_prompts.py';t=p.read_text()
    if 'from native_semantics import' not in t:t=t.replace('import re\n','import re\nfrom native_semantics import rewrite_prose, shared_contract_section, rewrite_prompt\n',1)
    elif 'shared_contract_section, rewrite_prompt' not in t:t=t.replace('from native_semantics import rewrite_prose, shared_contract_section','from native_semantics import rewrite_prose, shared_contract_section, rewrite_prompt')
    if 'text=rewrite_prose(text)' not in t:t=t.replace('        p.write_text(text)','        text=rewrite_prose(text)\n        text=text.replace("## Core requirements",shared_contract_section()+"## Core requirements",1) if "## Core requirements" in text else text.replace("## Compact execution contract",shared_contract_section()+"## Compact execution contract",1)\n        p.write_text(text)',1)
    t=t.replace('        text=rewrite_prose(text)','        text=rewrite_prompt(text)')
    p.write_text(t)

def finish_records(changes):
    snap=read(DATA/'native-rust-v1-inputs.json');e=read(ROOT/'AGONES_RS_STATE.json')['delivery']['roadmap_execution']
    quote,q=guidance_quote()
    amendment=shared_contract_section()+'''## Amendment execution record

1. Authorized on 30 October 2026. This is documentation/control-record maintenance; no production code, project executable, test, compiler or generator was run.
2. The current M0003/MANUAL/IMPLEMENTING cursor, one native Goal, global implementation gate, prior completed milestones, source IDs and route order are preserved. Its pending requirements are now the canonical Agones utility gap list, with native Rust adoption and explicit Go-only supersession.
3. Existing compatibility code is preserved as working implementation requiring reuse/replacement/retirement decisions. Removing an obsolete requirement does not mark that code migrated, M0003 complete or any assurance successful.
4. Historical source/profile descriptions, superseded gap text, the original Goal attachment and usage observations retain their identities. Pre-amendment documents/control records are recoverable from native-rust-v1-history.json.gz; native-rust-v1-inputs.json binds the archive and protected files.
5. Forecast-baseline-v1.json and prior actual-cost reports retain their bytes. The original forecast assumed broader source-semantic work and is no longer a current scope estimate. No replacement numerical estimate or historical usage reassignment was made. Future measured comparisons must identify this semantic-policy boundary; unavailable charges remain unknown.
6. The updated guidance is an uncommitted working-tree source. Its path, line range and content hash are authoritative for this amendment; no commit permalink is fabricated. Upstream excerpts remain quoted verbatim under their original revisions.
'''
    (DATA/'native-rust-policy-amendment.md').write_text(amendment)
    handoff=f'''# Agones-rs and CPG handoff

1. Canonical ledger: `AGONES_RS_STATE.json`. Apply the current root AGENTS.MD, `roadmap/0000-contract.md`, the separate `roadmap/alternation-policy.md`, and semantic policy `{POLICY_ID}`. The immutable original Goal attachment remains a historical input where the later user-authorized amendment supersedes it.
2. Current cursor: M0003 / MANUAL / IMPLEMENTING / READY. Preserve the existing native Goal identity/status; this documentation task did not invoke host Goal controls or start migration execution. M0001 and M0002 retain their historical IMPLEMENTED_UNVERIFIED records. Neither is newly tested or recertified by this amendment.
3. Phase: IMPLEMENTATION_ONLY. Global BOTH-route gate: CLOSED. Verification: NOT_STARTED. No project build/test/probe/proof/generator/CI run is authorized until the complete required production scope and outputs are authored. A needed prohibited emitter build remains an explicit blocker.

## Immediate next action

{amendment_next_action()}

## Current semantic obligations and records

1. Preserve Agones resource/API/protocol contracts, defaults, validation, allocation/scheduling rules, lifecycle/health/counter/list behavior and required operational outcomes. Use native Rust ownership, errors, formatting/logging, HTTP/TLS/serialization and async/effect facilities through CGP; derive-deftly supplies declarations, metadata and wiring.
2. Active M0003 gaps are in `verification/agones/roadmap-M0003-partial-map.json`. Preserve all 14 file owners, but reconcile their actual Agones obligations and native production consumers. The absence of a named-body gap is not semantic completion.
3. Go fmt/reflection/typed-nil/panic/recover/Goexit, goroutine/channel/select, map/slice layout, GC/allocator/ABI reconstruction is GO_ONLY_SUPERSEDED. Existing implementations and their historical authored-operation records remain available; they are not additional completion prerequisites.
4. Preserve unresolved Agones endpoints, error/status classes, retry/backoff, readiness, admission, watching, configuration, signal shutdown and resource/task lifecycle requirements. Retiring a Go mechanism cannot remove a domain requirement.
5. Retain the complete native CPG graph/schema/tooling and source-derived generation objectives. CPG input may retain original Go facts for provenance; generated Agones emits native Rust and real library calls, under the same policy as the manual route.
6. Preserve M0003 usage-start/progress observations and completed M0001/M0002 cost comparisons. No new implementation completion, route switch, fabricated measurement or credit charge is recorded by this amendment. Original forecast v1 is retained as a historical baseline.

## Recovery and provenance

Read `verification/agones/roadmap/native-rust-policy-amendment.md` and the current milestone/affected file entries. The amendment inputs and compressed history preserve previous state/handoff, source profiles and gap text. Historical compatibility requirements do not reactivate on resumption. Exact source paths, harness naming, CGP/derive-deftly ownership, Pythonic valid Rust and final post-gate assurance/delivery remain required.
'''
    (ROOT/'AGONES_RS_HANDOFF.md').write_text(handoff)
    p=ROOT/'cost-analysis.md';t=p.read_text()
    notice='''> **30 October 2026 scope amendment — AGONES_NATIVE_RUST_V1.** The v1 tables and assumptions below are retained as the original forecast. Active work now preserves Agones-specific contracts using native Rust and existing ecosystem facilities; Go runtime/language/standard-library emulation is superseded. These old ranges are not a refreshed remaining-work estimate. Existing milestone measurements remain attributable to their original scope; no historical cost was reassigned and no new numerical estimate was produced. See [the contract amendment](verification/agones/roadmap/native-rust-policy-amendment.md).

'''
    if '30 October 2026 scope amendment' not in t:t=t.replace('\n','\n\n'+notice,1)
    p.write_text(t)
    p=ROOT/'cost-analysis-comparison/README.md';t=p.read_text()
    if POLICY_ID not in t:t+='\n## Native Rust scope boundary\n\nRecord semantic policy AGONES_NATIVE_RUST_V1 for future work. Preserve original forecasts and completed comparison reports. Identify pre-/post-amendment scope without guessing an unavailable usage boundary; do not relabel previously spent tokens as native migration work or zero out superseded effort. Credits remain unknown unless attributable.\n'
    p.write_text(t)
    p=DATA/'README.md';t=p.read_text()
    if POLICY_ID not in t:t+='\n## Native Rust amendment\n\nAGONES_NATIVE_RUST_V1 is recorded in native-rust-policy-amendment.md. semantic-obligations.json classifies individual Agones obligations. Use build_docs.py --refresh-roadmap for execution-aware documentation rendering; it preserves the frozen source inventory and original forecast. Full initial-authoring mode remains unavailable once actual comparisons exist. Guidance copies contain the exact current root AGENTS.MD text. Historical snapshots and measurements remain immutable.\n'
    p.write_text(t)
    state=read(ROOT/'AGONES_RS_STATE.json');e=state['delivery']['roadmap_execution']
    for key,path in [('contract','roadmap/0000-contract.md'),('alternation_policy','roadmap/alternation-policy.md'),('combined_roadmap','roadmap.md')]:e[key+'_sha256']=digest((ROOT/path).read_bytes())
    e['contract_amendment_sha256']=digest((DATA/'native-rust-policy-amendment.md').read_bytes())
    e['semantic_obligations']='verification/agones/roadmap/semantic-obligations.json'
    e['semantic_obligations_sha256']=digest((DATA/'semantic-obligations.json').read_bytes())
    e['current_record_bindings']={x['path']:digest((ROOT/x['path']).read_bytes()) for x in changes}
    save(ROOT/'AGONES_RS_STATE.json',state)
    save(DATA/'native-rust-v1-amendment.json',dict(id=POLICY_ID,date='2026-10-30',authority=q,scope='DOCUMENTATION_AND_CONTROL_RECORDS_ONLY',preserved_cursor=snap['execution_before']['cursor'],original_forecast_sha256=snap['execution_before']['forecast_baseline_sha256'],record_changes=changes,production_changes=False,project_execution=False,goal_controls_invoked=False,comparison_policy='Preserve actual history; do not infer missing policy-boundary counters',history_archive=snap['history_archive']))

def main():
    if not (DATA/'native-rust-v1-inputs.json').exists():raise RuntimeError('Capture the immutable pre-amendment inputs first.')
    if (DATA/'native-rust-v1-amendment.json').exists():raise RuntimeError('The amendment is already applied. Use build_docs.py --refresh-roadmap for document-only refresh; preserve the original amendment history and execution records.')
    revise_templates()
    # Guidance must be copied before control-record content bindings are refreshed.
    for p in (ROOT/'roadmap/AGENTS.MD',DATA/'AGENTS.MD',ROOT/'verification/AGENTS.MD',ROOT/'verification/agones/AGENTS.MD',ROOT/'cost-analysis-comparison/AGENTS.MD'):p.write_bytes((ROOT/'AGENTS.MD').read_bytes())
    changes=revise_execution_records()
    refresh_roadmap()
    update_prompts()
    finish_records(changes)
    print(json.dumps(dict(policy=POLICY_ID,milestone_cursor=read(ROOT/'AGONES_RS_STATE.json')['delivery']['roadmap_execution']['cursor'],updated_control_records=len(changes),production_code_changed=False,forecast_reestimated=False),indent=2))

if __name__=='__main__':main()
