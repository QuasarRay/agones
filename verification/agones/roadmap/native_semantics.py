"""Native-Rust documentation policy. This module does not execute project code."""
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[3]
DATA = Path(__file__).resolve().parent
POLICY_ID = 'AGONES_NATIVE_RUST_V1'
DISPOSITIONS = ['AGONES_REQUIRED', 'RUST_NATIVE', 'GO_ONLY_SUPERSEDED']

DOMAIN = {
 'a-foundation': 'Agones version/configuration inputs, feature gates, environment precedence and API-group constants',
 'a-utilities': 'Agones error classification, log event fields, CRD readiness, HTTP/TLS endpoints, admission results, watcher notifications, retry/backoff, signal-driven shutdown and queue lifecycle',
 'a-protocol': 'Agones SDK/allocation message fields, field numbers, service methods, streaming directions, presence and status contracts',
 'a-api': 'Agones resource and wire schemas, served versions, selectors, metadata, defaults, validation, state transitions and SDK/allocation contracts',
 'a-apply': 'Agones declarative resource field presence, append/replace decisions and submission inputs',
 'a-client': 'Agones request resource/version/namespace, verbs, patch payloads, deadlines and watch contracts',
 'a-observation': 'Agones apply/request construction, resource selection, list/watch continuity, cache observations and shutdown',
 'a-cloud': 'Agones disruption decisions, required labels/annotations and shutdown hooks through Rusternetes',
 'a-metrics': 'Agones metric identities, labels, event values, aggregation and collector lifetime',
 'a-ports': 'Agones port ranges, protocols, collision/exhaustion outcomes and allocation/release policy',
 'a-gameservers': 'Agones GameServer/GameServerSet construction, reconciliation, scheduling, state/health, scaling, finalizers and cleanup',
 'a-gameserversets': 'Agones replica calculations, allocation-aware scaling and scheduling decisions',
 'a-fleets': 'Agones Fleet rollout, replica and surge/unavailability arithmetic, resource versions and allocated-server handling',
 'a-fleetautoscalers': 'Agones scaler policies, bounded counts, webhook decisions, schedules and failure outcomes',
 'a-allocation': 'Agones selection, priorities, counts/lists, contention, allocation responses and remote allocation protocols',
 'a-processor': 'Agones allocation batch, retry, deduplication and response policy',
 'a-sdkserver': 'Agones SDK lifecycle RPCs, health, streams, metadata, player/counter/list updates and disconnect/shutdown behavior',
 'a-sdkclients': 'Agones SDK requests, endpoints, streaming, health and shutdown contracts',
 'a-entrypoints': 'Agones commands, configuration precedence, authentication, readiness, child-service composition and graceful shutdown',
 'a-examples': 'Agones example requests/resources and required build/report/version/schema/documentation outputs',
 'a-tooling': 'Agones build/report/version/schema/documentation inputs, command surfaces and deterministic output identities',
 'a-delivery-inputs': 'Agones deployment, packaging, feature, certificate and service configuration',
 'a-macros': 'The required Rust metaprogramming forms, production declarations and ordinary Cargo integration',
 'a-assurance': 'Independent Agones-specific test invariants, cases, fixtures and production observations',
 'a-delivery': 'Agones domain/API acceptance, architecture, literal source paths and authorized delivery',
}

NATIVE_FACILITIES = {
 'a-utilities': 'std::error::Error and typed Result, std::fmt, existing Rust logging integrations, Tokio/futures tasks and synchronization, selected native HTTP/TLS clients/servers and filesystem-watcher facilities',
 'a-api': 'Rust structs/enums, Option/Result, serde/protobuf bindings, checked domain conversions and existing resource declarations',
 'a-observation': 'Rust builders, existing Rusternetes resource/storage capabilities and Tokio/futures streams/tasks',
 'a-entrypoints': 'Existing Rust configuration, logging, HTTP/TLS and Tokio lifecycle facilities',
 'a-assurance': 'Rust-native harnesses, Bolero/Kani and Rusternetes integration after the global gate',
}

SUPERSEDED = {
 'cleanup': (r'\bdefer\b', 'Go defer execution, panic/unwind ordering and runtime cleanup mechanics', 'Rust RAII/Drop, scoped guards and explicit async shutdown under the CGP owner'),
 'tasks': (r'\bgo\s+\w|\bgoroutine\b|\bchan\b|\bselect\s*\{', 'Go goroutine scheduling, channel/select mechanics and channel representation', 'Tokio/futures tasks, Rust channels/streams and explicit cancellation/supervision'),
 'panic': (r'\bpanic\(|\brecover\(', 'Go panic/recover/Goexit values, stack rendering and runtime failure behavior', 'Rust Result/error policy and owned-task failure handling without a Go runtime'),
 'presence': (r'\bnil\b|omitempty', 'Go typed-nil interfaces and nil slice/map/header identity', 'Option and native containers, with serde/protobuf presence only where an Agones contract requires it'),
 'collections': (r'\bmap\[|\[\]|\bmake\(', 'Go map hashing/range/growth, slice headers/capacity and allocation/alias layouts', 'Rust collections, borrowing and explicit domain ordering or duplicate policy'),
 'numbers': (r'\b(?:float32|float64|int|uint|uintptr)\b', 'Go machine-word, conversion, overflow, float-printing and ABI behavior', 'Rust domain types and checked conversions; preserve only documented API/schema ranges and Agones arithmetic'),
 'formatting': (r'fmt\.|klog\.|logrus\.|reflect\.|runtime\.', 'Go fmt/strconv/reflection, klog/logrus internals, GC/allocator/layout and runtime diagnostics', 'std::fmt, std::error::Error and established Rust logging/serialization facilities'),
}

def domain_for(row):
    path=row['path']
    if path.startswith(('pkg/sdk/','pkg/allocation/go/','proto/')):return DOMAIN['a-protocol']
    if path.startswith('pkg/client/applyconfiguration'):return DOMAIN['a-apply']
    if path.startswith('pkg/client/clientset'):return DOMAIN['a-client']
    if path.startswith('pkg/gameserversets/'):return DOMAIN['a-gameserversets']
    return DOMAIN.get(row['family'], 'The Agones-specific behavior and compatibility data identified by this file and its package contract')

def obligations(row, text):
    refs=[dict(repository=row['repository'],revision=row['source_revision'],path=row['path'],blob=row['git_blob'])]
    result=[dict(id=row['id']+'::agones-contract',disposition='AGONES_REQUIRED',requirement=domain_for(row),source_refs=refs,status='REQUIRES_DOMAIN_MAPPING; file/declaration counts are not behavioral acceptance'),
            dict(id=row['id']+'::rust-facilities',disposition='RUST_NATIVE',requirement=NATIVE_FACILITIES.get(row['family'],'The Rust standard library and existing selected Rust ecosystem dependencies under CGP capabilities'),source_refs=refs,status='ADOPTION_AND_CONSUMER_MAPPING_REQUIRED')]
    for key,(pattern,old,native) in SUPERSEDED.items():
        hits=[i for i,l in enumerate(text.splitlines(),1) if re.search(pattern,l)]
        if hits:
            result.append(dict(id=row['id']+'::go-only-'+key,disposition='GO_ONLY_SUPERSEDED',superseded_requirement=old,native_replacement=native,source_refs=[dict(**refs[0],source_lines=hits)],authority=POLICY_ID,status='NOT_A_COMPLETION_BLOCKER; preserve Agones obligations through native Rust'))
    return result

def native_traits(row,text):
    result=[]
    for o in obligations(row,text)[2:]:
        result.append('GO_ONLY_SUPERSEDED: '+o['superseded_requirement']+'. RUST_NATIVE: '+o['native_replacement']+'. Retain any independently evidenced Agones rule; do not reconstruct the Go mechanism.')
    return result

def agones_file_entry(author,row,key):
    q=author.file_quote(row)
    raw=author.raw('agones',row['path']).decode('utf-8',errors='replace')
    entries=obligations(row,raw)
    row['semantic_policy']=POLICY_ID
    row['semantic_obligation_ids']=[o['id'] for o in entries]
    if not hasattr(author,'semantic_obligations'):author.semantic_obligations=[]
    author.semantic_obligations.extend(entries)
    out=[f'<a id="{row["entry_anchor"]}"></a>\n',f'### File `{row["path"]}`\n',f'**Source:** `agones` at `{row["source_revision"]}`; Git blob `{row["git_blob"]}`. **Disposition:** `{row["disposition"]}`. **Destination:** `{row["destination"]}`.\n',q,
         '**Reference boundary.** '+row['documentation_gap']+' The excerpt records the original implementation. Go syntax and mechanisms in it are historical reference inputs; they do not require runtime or language emulation.\n',
         '**Derived implementation instructions.**\n',
         '1. **AGONES_REQUIRED:** '+domain_for(row)+'. Identify this file’s concrete Agones decisions, inputs, outputs, failures and externally required effects using its excerpt and package documentation. Implement those obligations completely; map every relevant source declaration/body to its native Rust consumer.\n',
         '2. **RUST_NATIVE:** '+NATIVE_FACILITIES.get(key,'Use the Rust standard library and existing selected ecosystem libraries for ordinary data, error, formatting, I/O and concurrency behavior')+'. Reuse a capable facility before custom code. Record the concrete Agones policy or integration gap for any necessary extension; a library written in Go is a dependency reference, not another porting objective.\n']
    d=row['disposition']
    if d=='GENERATED_PRODUCTION':out.append('3. Regenerate this exact output from canonical schema/protocol/domain declarations using Rust bindings. Retain required wire fields and Agones operations, while emitting native Rust types and library calls; do not reproduce Go generated-runtime helpers.\n')
    elif d.startswith('DEFERRED_'):out.append('3. POST-GATE ONLY: retain source::fixture::test identities and independent Agones invariants. Mark Go-only assertions GO_ONLY_SUPERSEDED with source IDs and authority. Add Rust-native coverage of the replacement; do not weaken any Agones invariant or claim the Go-only assertion passed.\n')
    else:out.append('3. Preserve API/resource/protocol field names, ranges, presence, ordering, defaults and errors only where the Agones contract requires them. Use Rust types, Option/Result, checked domain conversions and existing serializers. Go implementation details do not independently create compatibility requirements.\n')
    symbols=row['symbols']
    if symbols:
        names=', '.join('`'+s['name']+'` (L'+str(s['line'])+')' for s in symbols[:24])
        out.append(f'4. Account for {len(symbols)} lexically identified declarations, including {names}. The complete list remains in `{row["id"]}` in the frozen inventory. Body mappings may use real Rust library implementations; declarations and lexical counts alone establish no completion.\n')
    else:out.append('4. Preserve required Agones resource/configuration content and source identity. Retain binary resource bytes and attribution; implement any needed tool behavior with native Rust facilities.\n')
    for n,rule in enumerate(native_traits(row,raw),5):out.append(f'{n}. {rule}\n')
    out.extend(['12. Keep one CGP capability/provider/context and resource owner per production effect. Derive-deftly supplies repeated declarations, ownership metadata and wiring. Native Rust libraries perform infrastructure work; no Go compatibility engine or second application manager sits behind the provider.\n',
                '13. Use Python-familiar valid Rust: clear names, small functions, comprehensions/readable iterators and explicit transformations. Preserve required Agones decisions and Rust ownership/error semantics. Keep the exact filename, case and singular/plural stem; module glue stays outside the mirror.\n',
                '14. Completion record: source/body IDs, AGONES_REQUIRED/RUST_NATIVE/GO_ONLY_SUPERSEDED decisions, Agones evidence, selected Rust facility, actual production consumer, CGP/derive-deftly owners and unresolved Agones gaps. Superseded Go mechanisms are not missing implementation. Keep implementation and all deferred assurance statuses truthful.\n'])
    return '\n\n'.join(out)

# Exact substitutions for the authored prose. Quotes are never passed through this map.
REPLACEMENTS = {
 'source-compatible Agones Rust implementation':'Agones-compatible Rust implementation with native Rust semantics',
 'preserving initialization, branches, errors, side effects and observable data':'preserving Agones-specific initialization, decisions, API failures and effects through native Rust',
 'complete source-owned behavior':'complete Agones-specific behavior using existing Rust facilities',
 'all source-owned error/edge paths':'all required Agones error/edge paths using Rust-native error and ownership policy',
 'No unknown production scope, required stubs, fake success, disconnected bodies or omitted source operations may remain.':'No unknown required Agones production scope, required stubs, fake success or disconnected domain operations may remain. GO_ONLY_SUPERSEDED mechanisms are excluded with explicit source-bound records.',
 'Neither permits changing source semantics, memory safety, ownership or public compatibility.':'Both require native Rust semantics, memory safety and ownership, while preserving Agones-specific public contracts.',
 'Preserve eager/lazy execution, evaluation count and order, short-circuiting, ownership, allocation, duplicates and permitted output order.':'Use Rust-native eager/lazy evaluation, short-circuiting, ownership and allocation. Preserve Agones-required effects, duplicate handling and output ordering explicitly.',
 'Preserve source ordering and avoid redundant clones':'Preserve Agones-required ordering and avoid redundant clones',
 'Preserve ownership, evaluation order and errors.':'Use Rust ownership, evaluation and errors while preserving Agones-required results.',
 'The actual operation, errors and ordering come from the quoted source contract.':'The quoted source supplies Agones operations and domain evidence; Rust supplies language/runtime behavior.',
 'Retain source integer widths and error paths.':'Use native Rust domain types and errors; retain ranges or wire encodings only when required by an Agones API/schema contract.',
 'Keep nil, omitted, empty and default values distinct; retain source integer widths and error paths.':'Represent Agones-required presence, defaults, ranges and API errors through Rust types and native serialization.',
 'copy semantics and generated schema/deep-copy support':'Rust ownership/clone behavior and generated resource/schema support',
 'pointer/empty distinctions':'Agones-required presence distinctions expressed through native Rust types',
 'from the exact source bodies':'from the source-derived Agones contracts using native Rust libraries',
 'deletion and source failure behavior':'deletion and Agones-required failure outcomes',
 'Comprehensions must preserve the source order or explicitly retain its allowed unordered outcomes.':'Comprehensions use Rust iteration semantics and retain ordering only where Agones requires it.',
 'Retain the source cancellation and cleanup path for every spawned operation.':'Implement Agones cancellation/shutdown requirements using CGP-owned Rust tasks, RAII and explicit async cleanup.',
 'source-compatible async/cancellation behavior':'Agones-required async outcomes and native Rust cancellation/lifecycle policy',
 'source parity':'Agones domain/API parity implemented with native Rust semantics',
 'source invariants':'Agones-specific invariants',
 'source-test invariants':'Agones-specific test invariants',
 'source panic/recover boundaries':'Rust-native task failure/error boundaries',
 'Preserve source policy for partial/repeated startup, auxiliary versus primary failures, retry exhaustion, panics, lost watches, conflicts, shutdown during I/O, drain deadlines and cleanup.':'Preserve Agones policy for startup, retry exhaustion, lost watches, conflicts, shutdown and drain deadlines. Use Rust errors, task supervision and RAII cleanup; do not reproduce Go panic/recover or runtime behavior.',
 'Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling.':'Preserve Agones external contracts and exact module/file spelling; express initialization, visibility and ownership through native Rust.',
 'The quoted package/topic contract and each file’s source excerpt jointly define this work.':'Use the quoted package/topic contract and file excerpt to identify Agones obligations; classify Go-only mechanisms as superseded under requirement 10.',
}

def rewrite_prose(text,scope='agones'):
    """Rewrite only authored lines; verbatim source blockquotes stay byte-identical."""
    out=[]
    for line in text.splitlines():
        if not line.startswith('>'):
            replacements=REPLACEMENTS if scope=='agones' else {
             'Implement the complete source-owned behavior':'Implement the complete CPG functional behavior through native Rust facilities',
             'preserving initialization, branches, errors, side effects and observable data':'preserving CPG graph/schema operations, interchange data, errors and resource outcomes',
             'Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling.':'Preserve CPG functional and interchange contracts and exact module/file spelling; implement initialization and visibility with native Rust.',
             'The quoted package/topic contract and each file’s source excerpt jointly define this work.':'The quoted package/topic and file excerpts establish CPG functional obligations. Rust libraries provide native implementation; no Go runtime is required.',
             'Preserve ownership, evaluation order and errors.':'Use native Rust ownership and errors while preserving required CPG operations and data ordering.'}
            for old,new in replacements.items():line=line.replace(old,new)
            if scope=='agones':
                line=line.replace('source semantics','Agones-specific contracts and native Rust semantics')
                line=line.replace('Go semantic translation','Agones-domain translation into native Rust')
                line=line.replace('Go semantic lowering','Agones-domain lowering to native Rust')
        out.append(line)
    return '\n'.join(out)+('\n' if text.endswith('\n') else '')

def rewrite_prompt(text):
    text=rewrite_prose(text)
    lines=[]
    for line in text.splitlines():
        if line.startswith('Build a construct/support inventory from **all** required source bodies'):
            line='Build a source-bound Agones obligation inventory before emission. Preserve Go AST/type facts as analysis inputs, then map Agones domain operations and required API constraints to native Rust types, errors, control flow and selected library calls. Record AGONES_REQUIRED, RUST_NATIVE and GO_ONLY_SUPERSEDED per obligation. Go language/runtime/standard-library emulation is excluded.'
        elif line.startswith('Every required body must have a substantive source-derived Rust implementation.'):
            line='Every required Agones operation must have a substantive native Rust implementation or real library operation with source mapping and a production consumer. Retain callbacks and nested domain decisions; record Go-only source/body identities as superseded. Empty providers, missing helpers and fabricated success do not discharge Agones behavior.'
        elif line.startswith('| Semantic lowering |'):
            line='| Semantic lowering | Recover Agones domain operations and required API constraints, map them to native Rust IR/library operations, and retain explicit Go-only supersession provenance. |'
        line=re.sub(r'Match the original behavior for evaluation order/count.*?duplicate-key handling\.', 'Use native Rust evaluation, ownership, allocation and collections; preserve ordering, duplicate/error handling and effects only where Agones requires them.',line)
        line=line.replace('An approximation must still implement the source operation concretely. An unimplemented construct is a coverage gap, not an approximation.','An approximation cannot omit an Agones operation. Rust-native replacements for Go-only mechanisms are required semantic changes with explicit supersession, not missing runtime helpers or unsupported constructs.')
        lines.append(line)
    return '\n'.join(lines)+'\n'

def milestone_policy(m):
    native='''## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.

'''
    if m['route']=='CPG':
        native+='The complete native CPG graph/schema/build objective remains required. Preserve its functional graph, schema and interchange contracts through Rust. Go source facts may be retained for analysis/provenance; generated Agones must use the same native Rust boundary as handwritten Agones.\n\n'
    return native

def guidance_quote():
    p=ROOT/'AGENTS.MD';data=p.read_bytes();lines=data.decode().splitlines()
    first=next(i+1 for i,l in enumerate(lines) if l.startswith('10. Existing Rust'))
    value=lines[first-1];sha=hashlib.sha256(data).hexdigest()
    ident='Q-'+hashlib.sha256((str(p)+'\0'+sha+'\0'+str(first)+'\0'+value).encode()).hexdigest()[:18]
    record=dict(id=ident,repository='project-guidance-current',repository_url='https://github.com/QuasarRay/agones',revision='WORKTREE_UNCOMMITTED',path='AGENTS.MD',local_path=str(p),start_line=first,end_line=first,source_sha256=sha,kind='USER_SUPPLIED_CURRENT_PROJECT_GUIDANCE',text=value,permalink=None,source_origin='CURRENT_WORKING_TREE; no commit permalink is asserted')
    rendered=f'**Direct quotation — USER_SUPPLIED_CURRENT_PROJECT_GUIDANCE; {ident}.** [Current AGENTS.MD, line {first}]({p}:{first}). Source: uncommitted working tree on 30 October 2026; SHA-256 `{sha}`.\n\n> {value}\n'
    return rendered,record

def shared_contract_section():
    quote,_=guidance_quote()
    return '''## Native Rust policy — AGONES_NATIVE_RUST_V1

'''+quote+'''
**Derived requirements.**

1. Preserve Agones-specific behavior: resource/API/protocol contracts, validation/defaults, allocation/scheduling/scaling decisions, lifecycle transitions, health, counters/lists and required operational outcomes. Rust supplies the language/runtime semantics throughout handwritten code, templates and generated output.
2. Use the Rust ecosystem to its fullest practical extent. Start with std and existing selected dependencies: Tokio/futures, serde and protocol bindings, selected native HTTP/TLS, Rusternetes, native filesystem notification and Rust logging facilities. Inspect supported APIs before adding infrastructure. Record a concrete Agones policy or integration gap for custom code; do not recreate an entire dependency because the original used a Go library.
3. **AGONES_REQUIRED** identifies domain/API obligations with source/docs evidence. **RUST_NATIVE** identifies the Rust library, type or effect providing infrastructure. **GO_ONLY_SUPERSEDED** retains the source identity and reason a Go-only mechanism is no longer required. Apply these within mixed files; never discard an Agones rule because its original implementation used a superseded mechanism.
4. Do not recreate Go fmt/strconv/reflection, typed-nil/interface method-set behavior, panic/recover/Goexit, goroutine scheduling/channel/select semantics, map hashing/range/growth, slice headers, GC, allocator or ABI behavior. These are not milestone gaps or final acceptance requirements. Existing compatibility code must be reviewed for reuse, replacement or retirement during implementation; file existence creates no obligation to finish that machinery.
5. Preserve JSON/protobuf/API names, field numbers, presence, ranges, defaults, required ordering and error responses where an Agones contract requires them. Use Rust types, Option/Result, checked domain arithmetic and serialization attributes/adapters. Wire compatibility does not authorize language/runtime emulation. Business errors remain required even when their Go implementation previously panicked; implement the intended domain outcome with Rust error policy.
6. Rust tasks, async cancellation, synchronization and RAII implement service lifetimes. CGP owns capabilities, runtime policy, resources, supervision and shutdown; derive-deftly generates repeated declarations and wiring. There is no separate Go runtime or compatibility manager behind these providers.
7. CPG retains source AST/types and graph facts as analysis/provenance inputs. The typed translation IR distinguishes Agones obligations from native implementation choices and superseded Go mechanisms. Genco emits Rust types, Rust library calls, CGP composition and derive-deftly declarations; it emits no Go interpreter or runtime compatibility engine. The complete native CPG graph/schema objective remains required.
8. Source-body coverage means every required Agones operation is mapped to a real Rust implementation or selected library facility with a production consumer. Retire Go-only obligations explicitly; do not erase source identities, substitute placeholders or weaken Agones behavior. Unknown Agones requirements remain open; unknown Go runtime internals are not blockers. Generation/coverage receipts bind this semantic-policy ID, Agones obligation IDs, native-library mappings and exact superseded Go-only identities; historical raw body counts cannot silently redefine the amended required set.
9. After the global gate, preserve independent Agones invariants and fixture identities in both reference-domain and production observations. Supersede Go-only assertions with authority/source IDs and add relevant Rust-native coverage. Do not prove Go runtime equivalence or count a superseded assertion as passed.
10. This amendment corrects active instructions and existing ledgers without restarting the Goal or changing alternation. Historical snapshots, prior measurements and forecast v1 remain historical. The current M0003 cursor remains IMPLEMENTING; no completion or verification credit follows from this policy change. The global production gate requires native adoption of required Agones operations and removal of Go-emulation machinery from their production paths; a Rust facade around that machinery is insufficient. Preserve existing code until its consumers and replacement/retirement are accounted for.

'''

def amendment_next_action():
    return ('Continue M0003 MANUAL under AGONES_NATIVE_RUST_V1 without a route switch. Reconcile the 14 utility-file obligations against Agones domain/API contracts. Reuse existing Rust facilities for HTTP/TLS, serialization, errors, formatting/logging, filesystem notifications, clocks, signals, queues and owned async tasks. Finish concrete production consumers, Agones retry/readiness/admission/shutdown behavior and CGP/derive-deftly wiring. Retire Go-only fmt/reflection/typed-nil/panic/channel/map/slice/GC/allocator/ABI reconstruction from active requirements; review existing compatibility code for native reuse, replacement or retirement. Preserve dirty implementation and historical usage. M0003 remains incomplete and unverified; the whole-project implementation gate remains CLOSED.')
