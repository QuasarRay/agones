"""Refine the two supplied route prompts in place, preserving substantive appendices."""
from pathlib import Path
import hashlib
import json
import re

ROOT=Path(__file__).resolve().parents[3]
DATA=Path(__file__).resolve().parent
PROMPTS=Path('/home/astral/Projects/Fenris-Creations-Prompts/Agones-rs-Prompts')
CONTRACT='/home/astral/Projects/agones/roadmap/0000-contract.md'
POLICY='/home/astral/Projects/agones/roadmap/alternation-policy.md'
INVOCATION=f'/goal Finish both Agones-rs and the complete required native CPG port and CPG -> Genco -> Rust generation pipeline under {CONTRACT} and {POLICY}. Reuse the one matching native Goal and existing implementation. Follow the numbered dependency-ready roadmap, shared accounting policy and literal Agones/Scala path mappings. Make CGP AND derive-deftly the joint architecture, programming paradigm and lifecycle owners; give Python-familiar valid Rust equal status with idiomatic Rust. Make ZERO attempts to build, test, probe or verify any project component until all required production implementation, generated outputs, integration and architecture are authored and connected. Treat FIRST_FULL_GENERATION_COMPLETE as a milestone. Continue through the shared post-gate assurance and authorized delivery requirements until both objectives are fully complete. Set no token budget unless I explicitly provide one.'

SHARED=f'''## Shared roadmap authority and exact source ownership

1. This route entrypoint refines the existing prompt; it participates in the **one combined objective**, not a competing Goal. Read [{CONTRACT}]({CONTRACT}), the current numbered milestone, and the separately authoritative [execution/accounting policy]({POLICY}). The [combined roadmap](/home/astral/Projects/agones/roadmap.md) is the deterministic portable edition; load bounded sections during execution.
2. Finish BOTH scopes. Preserve the full CPG port, including handwritten schema/build modules unused by Agones, and the complete source-derived generated Agones artifact as well as production Agones-rs and final assurance/delivery. The policy, not a cheaper-route preference, determines execution. Compare actuals with [cost-analysis.md](/home/astral/Projects/agones/cost-analysis.md) and record real milestone observations in [cost-analysis-comparison/](/home/astral/Projects/agones/cost-analysis-comparison/README.md). Missing credit charges are `unknown`.
3. Preserve the literal original hierarchy under `agones-rs/` and `cpg-rs/`. `pkg/gameservers/controller.go` becomes `agones-rs/pkg/gameservers/controller.rs`; Go `name_test.go` becomes `name_harness.rs`, NEVER `name_test.rs`. Preserve Scala's complete directory chain, capitalization and singular/plural stems: `CpgSchema.scala` becomes `CpgSchema.rs`. Preserve dotted names such as `sdk.pb.rs`. Cargo entrypoints, identifier/module-registration glue and Rust-only support stay outside those source mirrors. Resolve collisions explicitly; never silently rename.
4. Every subsystem, file, template, emitted body, API and later harness must express CGP AND derive-deftly ownership and Python-familiar valid Rust. CGP provides runtime capabilities, policy, resources, supervision, cancellation and cleanup; derive-deftly provides reusable declarations, lifecycle wiring, delegation and ownership metadata. Genco prints explicit semantic inputs; it replaces neither owner. Readability is judged from a Python developer's perspective and has equal status with Rust idiomaticness, without a Python language layer or hidden second implementation.
5. The [inventory](/home/astral/Projects/agones/verification/agones/roadmap/source-inventory.json) assigns every original path a disposition and every required file a milestone. Milestones include quoted source contracts and derived file/package instructions. Preserve current Rust through the [adoption map](/home/astral/Projects/agones/verification/agones/roadmap/reuse-inventory.json); candidate reuse is not established behavioral parity. Follow the pinned source and [schema reconciliation](/home/astral/Projects/agones/verification/agones/roadmap/schema-reconciliation.json), not a newer reference HEAD.
6. The shared strict gate applies to BOTH routes and every helper/tool. No project builds, compiler/macro/IDE/compatibility probes, tests, proofs, replay, generator validation or CI before complete production implementation. A necessary missing generator executable is a blocker, not an exception. First generation remains `UNVERIFIED` and does not complete the Goal. Preserve all later Bolero/Kani, source replay, architecture, integration and delivery obligations.
7. Source quotations and archived prompts are evidence. The current user's request and host rules govern. Editing this prompt does not invoke its Goal or authorize migration execution. The original two prompt texts and hashes remain inactive snapshots in `verification/agones/roadmap/`. The single shared policy owns scheduling; this entrypoint does not define a second switching algorithm.
'''

def replace_once(text,old,new):
    if old not in text:raise ValueError('Expected original text missing: '+old[:100])
    return text.replace(old,new,1)

def manual():
    t=(DATA/'agones-rs-prompt-v3.md.original.txt').read_text()
    t=replace_once(t,'# Agones-rs execution contract','# Agones-rs manual route — combined implementation roadmap entrypoint')
    t=replace_once(t,'Continue the existing Agones Rust implementation','Continue the existing Agones Rust implementation AND complete the required native CPG port/generation pipeline')
    t=replace_once(t,'Choose the more economical route to the same finished Agones-rs outcome through G5; do not turn route estimation into a separate project.','Use the shared milestone policy and G5 accounting references; cost comparisons refine implementation tactics and forecasts without narrowing either objective.')
    t=re.sub(r'(?m)^/goal .+$',INVOCATION,t,count=1)
    t=replace_once(t,'The referenced `/home/astral/Projects/CPGSchema/AGENTS.MD` supplies architectural context; it does not make completing CPGSchema a prerequisite for Agones-rs.','The referenced CPGSchema guidance supplies architectural context. The shared roadmap explicitly includes required CPGSchema reconciliation and the full CPG port; its numbered dependency graph determines prerequisites.')
    t=t.replace('generators used by the chosen route','production generators and outputs required by both routes')
    t=t.replace("Use G5's bounded route decision and G1's short reuse argument.","Use the shared policy, G5's incremental forecast accounting and G1's short reuse argument.")
    t=t.replace('Stop expensive estimation immediately and resume the recorded route.','Keep recurring comparisons short and resume the milestone required by the shared policy.')
    t=t.replace("-> reuse G5's route decision; one bounded comparison only if needed","-> recover the current roadmap milestone and its immutable forecast")
    t=t.replace('-> select dependency-closing production work','-> follow the separate policy and the next dependency-ready numbered milestone')
    t=t.replace('Reuse A\'s source identity and metadata census and G5\'s route decision.','Reuse A\'s source identity and metadata census and the shared roadmap milestone record.')
    t=t.replace('| Route decision | DIRECT_AGONES or CPG_GENCO_RUST, observed remaining work and known artifacts, qualitative cost/confidence, selected route, concrete reconsideration trigger |','| Milestone and cost | Shared roadmap ID/route, dependency readiness, forecast identity, observed usage boundaries, attribution gaps, completion/comparison record, next action under the separate policy |')
    start=t.index('**G5 —')
    t=t[:start]+'''**G5 — Compare the immutable forecast with attributable milestone consumption.** The full initial analysis is [cost-analysis.md](/home/astral/Projects/agones/cost-analysis.md); the sole scheduling/accounting authority is [alternation-policy.md](/home/astral/Projects/agones/roadmap/alternation-policy.md). Use its real Goal counter boundaries, provisional/final reporting and unknown-credit rules. Preserve the original forecast and record short explanations plus changes to future estimates. Never infer per-milestone charges from account-wide usage percentages or the historical cumulative Goal total. Do not repeat the repository-wide cost analysis or implement duplicate behavior to benchmark routes.

The earlier cheaper-route selection and recorded default-to-direct decision are superseded. The qualitative report still explains why direct work is likely cheaper to first finished Agones-rs; this observation does not remove the complete CPG deliverable or control execution. The shared roadmap is the required work sequence. Missing required production generation or adoption cannot be deferred past the global implementation gate.
'''
    t=t.replace('`migration_status=COMPLETE` additionally requires','`migration_status=COMPLETE` for BOTH full objectives additionally requires')
    t=t.replace('selected route, zero remaining known required implementation gaps','current milestone/route, zero remaining known required implementation gaps across BOTH objectives')
    t=t.replace('joint architecture progress, selected route, and delivery separately','joint architecture progress, current policy-selected milestone/route, and delivery separately')
    position=t.index('## Core requirements')
    return t[:position]+SHARED+'\n'+t[position:]

def cpg():
    old=(DATA/'agones-rs-cpg-prompt-v1.md.original.txt').read_text()
    # Keep the original A/B provenance, schema, bundle, lowering and receipt requirements,
    # reconciling their obsolete pre-check and terminal-generation clauses explicitly.
    a=old[old.index('## Appendix A'):old.index('## Appendix C')]
    changes={
      'and clean regeneration comparisons in an isolated directory':'and recorded generation provenance. Fresh regeneration comparisons are deferred until the global gate',
      'Focused schema fixtures should expose missing metadata and ordering/default/inheritance errors before full Agones emission.':'Record schema obligations during implementation; schema fixtures and ordering/default/inheritance checks are deferred until the global gate.',
      'Demonstrate structural/property preservation on focused fixtures before accepting a format adapter.':'Implement structural/property preservation from source contracts; demonstrate it on fixtures only after the shared global gate.',
      '### B3. Explicit translation stages and CGP ownership':'### B3. Explicit translation stages and joint CGP/derive-deftly ownership',
      'Define typed stage inputs/outputs, error/diagnostic results, source maps, versioned rule identities, and content bindings.':'Define typed stage inputs/outputs, error/diagnostic results, source maps, versioned rule identities, and content bindings. Derive-deftly owns reusable declarations, repeated stage implementations, lifecycle wiring and ownership metadata; CGP owns capabilities, runtime policy, resources and context composition.',
      'terminal artifact publication':'transactional artifact publication',
      'Imperfect emitted ownership/types, Rust compilation defects, or behavioral mistakes in an implemented rule are acceptable at this milestone; do not turn their repair into a prerequisite once complete generation is ready. Never claim an approximation is verified behavior.':'A complete generated artifact is recorded as UNVERIFIED, not as correct Rust or completed migration. Known missing source behavior prevents implementation completion. Later discovered ownership/type/behavior defects reopen the production gate for repair; first generation does not terminate this combined Goal. Never claim an approximation is verified behavior.',
      'CGP owns generated application composition and lifecycle as well as generation lifecycle.':'CGP AND derive-deftly jointly own generated application composition and lifecycle as well as the generation architecture. Use typed declarations and reusable templates to produce the CGP provider/wiring structure, with runtime policy and resources remaining explicit in providers.',
      'Reusable struct/enum-driven templates for repeated domain metadata, conversions, and implementation scaffolding.':'Joint architecture owner: struct/enum-driven implementation rules, lifecycle/delegation wiring, ownership metadata, repeated domain behavior and composition.',
      'Reuse existing capability notes and relevant source pins; check actual crate API/AST-version compatibility. Implement only machinery needed for this CPG port and first-generation pipeline. The broader same-logical-crate/cross-crate acceptance matrix remains a later migration requirement and cannot extend this goal after the terminal event.':'Reuse existing capability notes and relevant source pins; inspect API/AST source compatibility without executing probes. Implement concrete needed machinery progressively and finish all four same-logical-crate/cross-crate capabilities before the shared gate. Their full acceptance matrix is required after the gate and remains part of this combined Goal.',
      'smallest gap, and focused check.':'smallest gap, and the deferred post-gate evidence obligation.',
      'never replace current crates with the newly generated workspace.':'never blindly overwrite current crates with generated output. Author required adoption paths and source-to-canonical-owner transfers before the gate; preserve manual meaning and references, retain the separate generated artifact, and establish integration evidence after the gate.',
      'The terminal artifact must include':'The generation artifact must include',
      'revalidate its source identity and inventory bindings on execution':'recover its source identity and inventory bindings using permitted read-only bookkeeping',
    }
    for x,y in changes.items():a=replace_once(a,x,y)
    a=a.replace('The supplied CPG revision `d02d76ea5c9e3b398317d334228d7e8e78ab45aa` and Joern revision', 'The earlier supplied CPG revision `d02d76ea5c9e3b398317d334228d7e8e78ab45aa` and Joern revision')
    a=a.replace('At the supplied pin, `Schema2Json.scala`','At the earlier supplied source anchor, `Schema2Json.scala`')
    a=a.replace('regeneration comparison, and destination or disposition.','historical regeneration comparison (or explicitly deferred fresh comparison), and destination or disposition.')
    a=a.replace('### A2. Automatically distinguish', 'The retained pipeline now pins CPG `301c4010d28ecdac05bc2ed44310f56446459465` (v1.7.79). Current reference HEADs are separate. The roadmap records identical schema source blobs at the older CPGSchema anchor and this pipeline revision; native behavior reuse still requires exact body mappings. Do not upgrade pins or treat the old 394/695 map as current remaining work.\n\n### A2. Automatically distinguish')
    top=f'''# CPG → Genco → Rust — combined implementation roadmap entrypoint

## Objective and invocation

Finish the complete required native Code Property Graph port, including all handwritten schema/build behavior and unused production modules; implement the source-derived CPG → normalized CPG → typed IR → Genco → Rust pipeline; generate complete Agones output; AND finish production Agones-rs, assurance and authorized delivery under one native Goal. Preserve existing implementation and the two source mirrors. First generation is a required UNVERIFIED artifact milestone, not the end of the objective.

```text
{INVOCATION}
```

CPG means Code Property Graph. CGP means Context-Generic Programming. Genco emits source from explicit semantic inputs. CGP AND derive-deftly jointly own the architecture and lifecycle throughout both workstreams.

{SHARED}

## Compact execution contract

1. Recover current pins, source inventories, existing work, jobs, manifests and intact generation transactions. Preserve originals and user changes. A historical generation event supplies artifact evidence, not full combined completion.
2. Port every required handwritten CPG module/body, including schema and build machinery unused by Agones. Regenerate domain/protobuf material from complete inputs. Translate every required Agones production body without interpreter wrappers, copied handwritten output credit or semantic placeholders.
3. Retain one source-derived pipeline and versioned source/body/configuration/rule identities. Persist missing AST/type facts as bundle enrichment; never bypass CPG with an independent direct-source emitter.
4. Keep CGP AND derive-deftly central, Pythonic valid Rust coequal with Rust idioms, Rusternetes the only orchestrator and no plugin system. Preserve literal source-relative Go/Scala names; place Cargo/module support outside the mirrors.
5. Make ZERO attempts at project verification, including focused support builds and synthetic fixtures, until ALL required production implementation on both routes is authored and connected. The shared contract governs permitted emission-only operations and missing-executable blockers.
6. Implement complete schema, native port, bundle, IR, rules, runtime support, production adoption and transaction recovery. Receipt construction is intrinsic implementation bookkeeping; it grants no parsing, type, behavioral or proof credit.
7. Publish a complete generated artifact transactionally and record `FIRST_FULL_GENERATION_COMPLETE`; retain UNVERIFIED assurances and continue the shared objective under the separate policy. Do not regenerate an intact artifact merely to resume.
8. Use one matching native Goal and the existing canonical state/checkpoint links. Milestones are durable records, not invented native APIs. Read only the current milestone and relevant appendices; set no Goal token budget without explicit user instruction.

The numbered roadmap expands these retained obligations into source-derived file/package work. Appendix A owns source provenance/classification; B owns schema/translation/interfaces; C owns recovery; D owns transactional generation and combined completion. Scheduling and cost-accounting mechanics remain solely in the separate policy.

'''
    c='''## Appendix C — Native Goal, checkpoints and recovery

### C1. Use the actual native Goal controls

On implementation invocation, inspect the exposed controls and objective. Reuse the matching combined Goal. Create one only when the user's invocation authorizes it and no conflicting active Goal exists. `get_goal`, `create_goal` and `update_goal` are examples of exposed controls, not a milestone API. Respect live schemas, user pauses, host limits and the actual blocked-state threshold. Do not manufacture blocked turns, pause independently, set an unsolicited token budget or mark complete because a limit was reached. If controls/accounting are unavailable, report the limitation and retain ordinary authorized checkpoints; do not build a substitute scheduler.

### C2. Reuse one canonical project ledger

Reuse `AGONES_RS_STATE.json`, `AGONES_RS_HANDOFF.md` and the existing first-generation checkpoint as linked records. Add the current roadmap milestone, source bindings, owned outputs, gap/blocker, artifact path and usage-observation references to that canonical state when implementation is invoked. Do not create a parallel migration queue. Bind reusable work to the full relevant source/configuration/schema/rule/tool/dependency closure; additions, deletions and renames stale affected work. Preserve historical check receipts as historical; do not refresh them before the gate.

Checkpoint before long operations or interruption boundaries. Recover jobs by command, source binding, process/start identity or host job token and outputs; a PID alone is insufficient. Reuse caches and unchanged artifacts without rescanning everything. No duplicate jobs, deterministic retries without a changed hypothesis, destructive reset, silent stash or overwrite. No other chats, subagents, services or automations unless independently authorized.

### C3. Recover a generation milestone without declaring the Goal complete

Recover the checkpointed artifact's receipt using only necessary read-only identity/digest bookkeeping. An intact published `FIRST_FULL_GENERATION_COMPLETE` record establishes that transaction even if the external checkpoint lagged. Recover its milestone and accounting record, preserve its sealed source identities and continue the shared roadmap. It does not prove the native CPG port, generated behavior, final assurance or delivery complete. Do not regenerate or mutate an intact historical artifact merely because unrelated reference HEADs changed.

An incomplete staging directory, prepared record outside the committed artifact, missing required payload or damaged identity binding remains incomplete. Preserve recoverable content-bound work and the exact blocker. Never overwrite an unrelated artifact, infer completion from a directory name or fabricate the event. Required production repairs use new source-bound transactions with explicit ownership, and follow the global gate rather than a terminal first-generation stop.

## Appendix D — Transactional generation and full-objective completion

### D1. Implementation readiness before full emission

Finish all independent source/configuration/body inventories, CPG handwritten-port mappings, complete schema/domain data, bundle/IR/rule support, output obligations and runtime capabilities before the complete generation operation. Resolve required missing semantics through real implementation. All fresh fixtures, support compilation, API/macro probes, architecture validation and generator checks wait for the shared gate. Already runnable pinned generation tools may perform authorized source extraction/emission without check hooks; a required forbidden build is an explicit dependency blocker.

Do not disguise a full generation as a benchmark, preview, dry run or accumulated fixture shards. Intrinsic emission identity accounting and content hashes construct the artifact and its provenance; they are not a general verification campaign or proof of equivalent behavior. Finish required production adoption/call paths before the global gate, not in a supposedly post-gate integration implementation phase.

### D2. Publish a self-contained artifact transactionally

Choose and checkpoint a separate final artifact path, normally `generated/agones-first-generation/`, reusing an existing compatible location. It contains a literal `agones-rs/` source mirror, external `rust-support/agones/` Cargo/module wiring, complete resources/runtime providers and B5 evidence. Keep it outside the tooling build graph so later tooling checks do not accidentally build output before permission.

1. Freeze contract, source/configuration inventory, schema, bundle, IR, rules, tool and dependency identities. Resolve complete required file/body/resource/support sets before publication.
2. Emit into a new owned staging location. Produce actual non-placeholder bodies, exact paths and source-to-graph-to-IR-to-output mappings as part of emission. Never overwrite manual output or count copied handwritten bodies as translated.
3. Account for emitted identities, paths, ranges, resources and digests intrinsically during emission. Missing/placeholder/unresolved required identities prevent publication; equal counts alone are insufficient. No parsing, compilation, tests, formatting, semantic comparison or repair hooks attach to this operation.
4. Write B5 receipts and a non-circular payload digest manifest. The completion record binds the manifest rather than hashing itself. Include artifact identity/path, required/emitted/missing identity sets, known approximations and compilation/correctness/compatibility/production-readiness fields set to `UNVERIFIED`.
5. Flush and close files and writers, then atomically publish the complete self-contained directory on the appropriate filesystem. A flag in staging, partial copy or non-atomic multi-step move is not the completed transaction. Preserve interrupted staging with its identity and recover without overwriting unrelated work.
6. Record the atomic publication as `FIRST_FULL_GENERATION_COMPLETE`. Update the existing milestone/checkpoint/accounting references under the sole policy and continue the combined objective. A stale external checkpoint cannot invalidate an intact publication. This local artifact publication is distinct from Git push, PR, merge, release or deployment.

### D3. Continue until both full objectives are complete

First generation remains explicitly UNVERIFIED. The same Goal continues through all remaining production work and, only after the global implementation gate, source-test/property extraction, Bolero with Kani, source replay, native schema/graph/pipeline assurance, full `_2` matrix, Rusternetes integration and authorized delivery. Fix discovered production defects by closing the gate, finishing repairs, renewing the implementation record and then resuming affected assurance. Preserve generated-source provenance through repairs to rules/templates rather than untracked output patches.

Mark the matching native Goal complete only when the shared contract's full completion conditions hold for BOTH objectives, with no required unresolved work. Report host-required final usage if applicable. A pause, limit or actual external blocker yields an incomplete report with preserved paths, exact remaining obligations and the unlock condition. Neither prompt engineering, design, a partial port nor an unverified generated artifact is full completion.

### D4. Required decision scenarios

| Scenario | Required decision |
|---|---|
| All required bodies have substantive generated translations, but output has not been compiled | Record the complete generated artifact as UNVERIFIED; continue the shared objective without early checks. |
| A required source body, nested operation or runtime helper is absent/stubbed | Retain incomplete status and implement missing source-derived rules/support; never shrink expected scope or hand-patch generated output. |
| The frontend reports success but omits a required file/body | Retain unresolved coverage; repair source-bound extraction/enrichment before claiming complete generation. |
| Schema export loses hidden elements, defaults, identifiers, types or relationships | Finish the complete schema representation and dependent rules; Schema2Json's partial view is not full schema fidelity. |
| Interruption occurs after atomic publication | Recover intact artifact identity and its milestone; continue the one Goal without duplicate generation. |
| Interruption occurs before publication | Recover staging and keep milestone incomplete until the transaction is genuinely complete. |
| A needed new generator cannot run without a prohibited build | Record the exact dependency blocker; neither a renamed milestone nor output deferral opens the gate. |
| Unused handwritten CPG production behavior is missing | Keep CPG scope incomplete; unused-by-Agones is not an exclusion. |
| Full generation is complete but assurance/delivery remains | Continue required roadmap work in the same Goal. First generation is not the final completion event. |
'''
    return top+a+c

def main():
    result={}
    for name,text in [('agones-rs-prompt-v3.md',manual()),('agones-rs-cpg-prompt-v1.md',cpg())]:
        p=PROMPTS/name
        original=(DATA/(name+'.original.txt')).read_bytes()
        p.write_text(text)
        result[name]=dict(path=str(p),original_sha256=hashlib.sha256(original).hexdigest(),revised_sha256=hashlib.sha256(text.encode()).hexdigest(),original_bytes=len(original),revised_bytes=len(text.encode()))
    result['reconciliation']={
     'scope':'Both full objectives; no first-generation-only terminal stop',
     'manual_retained':'Core requirements and A-F/C0-C8/D/E architecture, source, property, evidence, recovery and delivery appendices retained; obsolete G5 selection replaced',
     'cpg_retained':'A/B source classification, complete schema, bundle, lowering, source-body coverage and receipt contract retained and reconciled; C/D recovery and transaction semantics retained in combined form',
     'priority':'Shared roadmap plus separate policy supersede obsolete cheaper-route selection and pre-generation support checks',
     'implementation_gate':'All production implementation on both routes before any project assurance',
     'naming':'Exact source mirrors, Go harness exception, literal Scala chain/case, external Rust-only glue',
     'measurement':'Actual attributable observations only; credits unknown when unavailable',
     'originals':'Inactive reference snapshots preserved; protected root prompts unchanged'}
    (DATA/'prompt-reconciliation.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))

if __name__=='__main__':main()
