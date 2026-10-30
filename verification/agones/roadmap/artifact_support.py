"""Documentation inventory, forecast and assembly helpers; never run project code."""
from pathlib import Path
import collections
import hashlib
import json
import re

NEW_OUTPUTS = {
 'a-foundation': [('rust-support/agones/Cargo.toml', 'Cargo package and pinned dependency ownership outside the source mirror'), ('rust-support/agones/src/lib.rs', 'Explicit source-module registration and public contract wiring'), ('rust-support/agones/src/lifecycle.rs', 'CGP startup, cancellation, supervision, drain and cleanup capabilities'), ('rust-support/agones/src/declarations.rs', 'derive-deftly ownership and repeated wiring declarations')],
 'a-macros': [('rust-support/agones/metaprogramming/Cargo.toml', 'Normal Cargo facade with internal compiler-shim units'), ('rust-support/agones/metaprogramming/src/lib.rs', 'All four required _2 forms and same-logical-crate/cross-crate composition'), ('rust-support/agones/metaprogramming/src/templates.rs', 'Shared typed expansion and derive-deftly templates with CGP consumers')],
 'c-build': [('rust-support/cpg/Cargo.toml', 'Pinned native CPG dependencies and external source-module paths'), ('rust-support/cpg/src/lib.rs', 'Exact-case Scala module registration without moving src/main/scala'), ('rust-support/cpg/src/build_contracts.rs', 'Native Rust equivalents of SBT/Bazel/launcher behavior with exact task/source mappings'), ('rust-support/cpg/src/declarations.rs', 'Native builder identity, typed metadata and derive-deftly wiring'), ('rust-support/cpg/src/lifecycle.rs', 'CGP stage resources, failure, cancellation and cleanup ownership')],
 'c-bundle': [('rust-support/cpg/src/bundle.rs', 'Versioned complete frontend bundle reader and source/configuration identities'), ('rust-support/cpg/src/frontend.rs', 'Pinned gosrc2cpg/goastgen invocation and export/enrichment contracts')],
 'c-normalization': [('rust-support/cpg/src/normalized.rs', 'Lossless normalized nodes/edges/properties and conflict/unknown facts'), ('rust-support/cpg/src/provenance.rs', 'Source-file/body/configuration/rule identity propagation')],
 'c-ir': [('rust-support/cpg/src/translation_ir.rs', 'Typed values, ownership, effects, control flow and observation contracts'), ('rust-support/cpg/src/interfaces.rs', 'Public Agones/Rusternetes types and explicit external-call bindings')],
 'c-lowering-data': [('rust-support/cpg/src/lower_values.rs', 'Agones domain values and API/schema constraints mapped to native Rust types and libraries'), ('rust-support/cpg/src/lower_initialization.rs', 'Agones configuration and startup obligations implemented with native Rust initialization')],
 'c-lowering-control': [('rust-support/cpg/src/lower_control.rs', 'Agones decisions and failure outcomes mapped to Rust control flow and Result'), ('rust-support/cpg/src/lower_concurrency.rs', 'Agones asynchronous operations using existing Rust tasks, channels and cancellation capabilities'), ('rust-support/cpg/src/runtime_support.rs', 'Thin native Rust ecosystem integration and Agones lifecycle policy under CGP; no Go runtime')],
 'c-emission': [('rust-support/cpg/src/emit_rust.rs', 'Typed-IR-to-Genco source and import emission'), ('rust-support/cpg/src/output_layout.rs', 'Literal path mapping, collision rejection and external module wiring'), ('rust-support/cpg/src/emit_templates.rs', 'Central CGP/derive-deftly output architecture and readable transformations')],
 'c-publication': [('rust-support/cpg/src/publication.rs', 'Staging, non-circular digests, flush and atomic publication'), ('rust-support/cpg/src/recovery.rs', 'Content-bound interrupted-operation recovery'), ('rust-support/cpg/src/adoption.rs', 'Explicit production ownership transfer and preservation of manual meaning'), ('generated/agones-first-generation/evidence/completion.json', 'Transactional complete source-derived artifact record with UNVERIFIED assurances')],
 'a-delivery': [('verification/agones/delivery/agones-acceptance.json', 'Post-gate source-qualified evidence and delivery preparation record')],
 'c-delivery': [('verification/agones/delivery/combined-completion.json', 'Final both-route evidence, delivered revision and native Goal completion eligibility')],
}

def new_output_sections(author, milestone):
    rows = NEW_OUTPUTS.get(milestone['key'], [])
    if not rows:
        return ''
    parts = ['## Explicit new support/output ownership\n', 'These are planned Rust-only support or evidence files, not original-source mirror renames. Reuse an existing equivalent through a recorded path mapping; do not create a competing implementation merely to match this suggested support filename.\n']
    for path, purpose in rows:
        parts.extend([f'### New support/output `{path}`\n', author.family_quote(milestone['key']), f'**Derived file obligation.** {purpose}. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.\n'])
    return '\n'.join(parts)

def scenario_costs(milestones, forecasts):
    byid = {m['id']: m for m in milestones}
    levels = ['low', 'base', 'high']
    def total(predicate):
        return {level: sum(f['tokens'][level] for f in forecasts if predicate(f, byid[f['milestone_id']])) for level in levels}
    manual_prod = total(lambda f,m: m['route']=='MANUAL' and m['phase']=='IMPLEMENTATION_ONLY')
    manual_post = total(lambda f,m: m['route']=='MANUAL' and m['phase']!='IMPLEMENTATION_ONLY')
    cpg_prod = total(lambda f,m: m['route']=='CPG' and m['phase']=='IMPLEMENTATION_ONLY')
    cpg_post = total(lambda f,m: m['route']=='CPG' and m['phase']!='IMPLEMENTATION_ONLY')
    reserve_rates = dict(zip(levels,[.10,.25,.60]))
    adoption_rates = dict(zip(levels,[.35,.55,.85]))
    switch_rates = dict(zip(levels,[2000,10000,40000]))
    scenarios = {}
    for name in ('direct_agones_only','cpg_to_finished_agones','both_mandatory_alternation'):
        scenario = {}
        for level in levels:
            if name=='direct_agones_only':
                production=manual_prod[level];assurance=manual_post[level];switching=0
            elif name=='cpg_to_finished_agones':
                production=cpg_prod[level]+round(manual_prod[level]*adoption_rates[level]);assurance=manual_post[level]+cpg_post[level];switching=0
            else:
                production=manual_prod[level]+cpg_prod[level];assurance=manual_post[level]+cpg_post[level];switching=(len(milestones)-1)*switch_rates[level]
            reserve=round(production*reserve_rates[level])
            scenario[level]=dict(production=production,assurance_and_delivery=assurance,delayed_rework_reserve=reserve,switching_and_recovery=switching,total=production+assurance+reserve+switching)
        scenarios[name]=scenario
    return dict(unit='tokens; unvalidated planning assumption, not measured usage',manual_production=manual_prod,manual_assurance=manual_post,cpg_production=cpg_prod,cpg_assurance=cpg_post,rework_reserve_rates=reserve_rates,manual_fraction_required_in_cpg_route=adoption_rates,switch_boundary_tokens=switch_rates,scenarios=scenarios,credits='unknown')

def assemble(root):
    road=root/'roadmap'
    numbered=sorted(p for p in road.iterdir() if re.match(r'^\d{4}-.*\.md$',p.name))
    rest=sorted(p for p in road.iterdir() if p.suffix.lower()=='.md' and p not in numbered)
    policy=road/'alternation-policy.md'
    ordered=numbered+[policy]+[p for p in rest if p!=policy]
    headers={p.name: ('shared-contract' if p.name.startswith('0000') else 'm'+p.name[:4] if re.match(r'^\d{4}',p.name) else 'alternation-policy' if p.name==policy.name else 'roadmap-guidance') for p in ordered}
    out=['# Combined Agones-rs and CPG roadmap\n', 'Generated deterministically from every Markdown file in `roadmap/`. Edit the directory edition, then run `python3 -B verification/agones/roadmap/build_docs.py --assemble` to reassemble documentation only. Section contents are preserved with local links rebased for this root-level edition. Read only the active milestone during execution.\n', '## Components\n']
    out.extend(f'- [{p.name}](#{headers[p.name]})' for p in ordered)
    manifest=[]
    for p in ordered:
        raw=p.read_text()
        def rebase(m):
            target=m.group(2)
            if target.startswith('../'):target=target[3:]
            elif target.split('#')[0] in headers:target='#'+(target.split('#',1)[1] if '#' in target else headers[target])
            return m.group(1)+target+m.group(3)
        # Do not modify quotations; any relative link in an excerpt remains source text.
        body='\n'.join(line if line.startswith('>') else re.sub(r'(\]\()([^\s)]+)(\))',rebase,line) for line in raw.splitlines())+'\n'
        if p.name=='AGENTS.MD':body='<a id="roadmap-guidance"></a>\n\n'+body
        out.extend(['\n---\n',f'<!-- BEGIN roadmap/{p.name}; SHA-256 {hashlib.sha256(raw.encode()).hexdigest()} -->\n',body,f'<!-- END roadmap/{p.name} -->\n'])
        manifest.append(dict(path='roadmap/'+p.name,sha256=hashlib.sha256(raw.encode()).hexdigest(),anchor=headers[p.name]))
    content='\n'.join(out).rstrip()+'\n'
    (root/'roadmap.md').write_text(content)
    return dict(order=manifest,combined_sha256=hashlib.sha256(content.encode()).hexdigest(),transform='Rebase non-quoted relative Markdown links only; preserve all canonical text otherwise.')

def cost_report(author, forecasts, scenarios, schema, reuse_count):
    levels=['low','base','high']
    def million(x):return f'{x/1_000_000:.2f} M'
    table=[]
    labels={'direct_agones_only':'Direct completion of Agones-rs only','cpg_to_finished_agones':'Full CPG pipeline → finished Agones-rs','both_mandatory_alternation':'Both complete under mandatory alternation'}
    for key,values in scenarios['scenarios'].items():
        table.append('| '+labels[key]+' | '+' | '.join(million(values[v]['total']) for v in levels)+' |')
    detail=[]
    for name,values in scenarios['scenarios'].items():
        for level,v in values.items():
            detail.append('| '+labels[name]+' / '+level+' | '+' | '.join(f'{v[k]:,}' for k in ('production','assurance_and_delivery','delayed_rework_reserve','switching_and_recovery','total'))+' |')
    milestone_rows=[]
    for m,f in zip(author.milestones,forecasts):
        milestone_rows.append(f'| [{m["id"]}](roadmap/{m["filename"]}) | {m["route"]} | {m["title"]} | '+ ' | '.join(f'{f["tokens"][v]:,}' for v in levels)+' |')
    report='''# Agones-rs route cost analysis

**Forecast version:** 1, 27 October 2026. **Scope:** remaining work, conditional on the strict implementation gate being feasible. **Evidence class:** source inspection and explicit planning assumptions, not a benchmark. **Actual attributable credits:** unknown.

## 1. Decision and the three different finish lines

1. **Direct completion is presently the better-supported route to the first finished Agones-rs.** Existing Rust behavior, source-obligation mappings, CGP contexts and Rusternetes integration can be reused directly. The CPG route must additionally finish a complete native CPG port, frontend/bundle contracts, semantic IR, Go lowering, runtime support, emission, production adoption and final assurance. Genco removes source-printing work; it does not supply these semantics. Confidence in the qualitative ranking is moderate; confidence in the numerical ranges is low.
2. **Finishing CPG remains a separate required outcome.** It includes all required handwritten CPG behavior, schema/build machinery even when unused by Agones, generated domain material and the full source-derived Agones pipeline. Existing CPGSchema work changes the reuse opportunity, but it does not establish behavior parity or eliminate compiler work.
3. **The authorized execution strategy completes both under mandatory alternation.** Its total normally exceeds the cheaper single-product counterfactual because it owes both outputs and route-boundary overhead. This report does not authorize choosing one route, pausing the other or bypassing a blocked required route. Operational rules have one owner: [alternation-policy.md](roadmap/alternation-policy.md).

The cheapest route to one deliverable is not the same question as the cost of the two-deliverable objective. The direct and CPG-only columns below are counterfactual comparisons; the both-routes column is the execution forecast. No migration milestone was executed to produce this report.

## 2. Pinned evidence and reusable work

| Item | Observed source fact | Consequence for this estimate |
|---|---|---|
| Agones baseline | `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; 8,240 tree paths; 401 non-vendor Go files, including 84 `_test.go`, in 116 directories | Complete path inventory is available. File counts are not behavior counts or completion percentages. |
| CPG pipeline baseline | `301c4010d28ecdac05bc2ed44310f56446459465`, v1.7.79; 355 upstream paths | The compatible pipeline snapshot, rather than current reference HEAD, governs translation. |
| Historical CPG classification | 250 generated, 102 handwritten, 3 unresolved nonproduction assets | Reuse the classification evidence; retain individual dispositions and all required handwritten/build behaviors. |
| Historical body inventory | 1,089 conservative syntax candidates in 54 files; full closure universe unresolved | These are source-accounting candidates, not independent equal-cost tasks. |
| Historical native map | 394 mapped / 695 remaining at the older checkpoint | Do not price 695 bodies as today's remaining work. Newer CPGSchema modules and current mappings require reconciliation. |
| CPGSchema source anchor | `d02d76ea5c9e3b398317d334228d7e8e78ab45aa` | Compare this anchor to the pipeline revision before claiming reuse. |
| Schema reconciliation now | All 25 source `.scala` files under `schema/src/main/scala` have identical blobs at the two revisions; 25 corresponding native module files exist | Concrete candidate reuse is established at the source/path level. Rust behavior, builder identity and full source-to-body mapping remain unverified. |
| Existing Agones/native CPG implementation | Existing crates, Rusternetes providers, source mappings, native schema/graph and Genco work | Preserve and map reuse, rather than restart either route. File existence and historical checks are not present acceptance. |
| Current reference checkouts | Recorded separately in `sources.json`, including CGP, derive-deftly, comprende, better comprehension, Genco and Kubernetes docs | New reference HEADs are not silently substituted for selected dependency versions. |

The [source inventory](verification/agones/roadmap/source-inventory.json), [schema reconciliation](verification/agones/roadmap/schema-reconciliation.json), [existing implementation ownership map](verification/agones/roadmap/reuse-inventory.json) and [source identities](verification/agones/roadmap/sources.json) are the audit inputs. The ownership map contains @@REUSE_COUNT@@ existing Rust/Python implementation/tool files; it is a preservation/adoption index, not a claim that each body has been reconciled. Exact source-to-current-body adoption remains an implementation obligation with explicit unresolved status.

The selected frontend uses Joern `9f2171242e48d7fed3021087d45838a83c1e39da` with CPG v1.7.79 and goastgen `11b21e395aa6aa39a41ca9216e1d15d08c4ef0b8`; retain Flatgraph v0.1.36 source semantics, Grafeo 0.5.44 native storage and selected Genco 0.17.10. An arbitrary installed binary is not evidence of this closure. The checkpoint and historical receipts are dated evidence, not executable availability guarantees.

## 3. What each route still owes

**Direct Agones.** Resolve current source-to-Rust ownership, place original-name files in the literal mirror, finish APIs/configuration/defaulting, clients/watch/queue contracts, ports, GameServer/GameServerSet/Fleet/autoscaling, allocation/processor, SDK streams/health/counters/lists, entrypoints and delivery inputs. Complete CGP/derive-deftly lifecycle ownership and the required `_2` family. Only after all production work on both required routes is authored may assurance begin. In the direct-only counterfactual, its gate would cover only that counterfactual's production scope.

**CPG pipeline.** Reconcile CPGSchema reuse; preserve constructor dependencies, mutable schema identity and full schema facts; finish graph/domain/loading/persistence/passes/build tooling; finish compatible source extraction/bundles, normalization, typed IR, Go value/control/concurrency/error lowering and runtime support; emit readable valid Rust through Genco; publish complete source/body provenance transactionally; author real production adoption; then establish source/schema/generator/Agones assurance. A schema dump, copied Rust body or source-shaped stub does not complete these stages.

**Both under alternation.** Share API and lifecycle contracts, schema declarations, effect adapters, lowering rules and final evidence where their bindings permit reuse. Keep a complete separate generated artifact and one canonical production owner per adopted output. The model budgets both workstreams conservatively; it does not assume all manual implementation effort disappears when generation is available. Do not duplicate behavior solely to measure which route would have been cheaper.

@@GENCO_QUOTE@@

**Derived cost implication.** Token emission reduces repeated syntax and imports. Semantic translation, ownership/effects, source coverage, integration and evidence remain independent work. CGP and derive-deftly reduce repeated wiring only when declarations/providers/templates are reused by real production consumers.

## 4. Forecast method and explicit assumptions

These numbers are **unvalidated planning envelopes**, not measured productivity, billing estimates, confidence intervals, minimum/maximum guarantees or promised token budgets. No price schedule or conversion from tokens to credits is assumed. The model is transparent so the first attributable observations can replace its assumptions without pretending they were historical data.

For each source-owning milestone, define `size = max(0.25, source_lines / 200)` and sum rough planning units: handwritten code `0.7 × size`; reusable CPG schema code `0.45 × size`; generated source `0.04 × size`; deferred harness/support `0.4 × size`; other required artifacts `0.08 × size`; retained reference paths `0.02`. These weights model reading/adaptation/accounting, not fractions implemented. Binary and empty resources remain distinct inventory records. Existing code is not blindly subtracted by line count; its possible savings are reflected in the low scenario and the reduced schema factor.

Each milestone gets 5,000 setup/context tokens plus 6,000 / 20,000 / 65,000 tokens per unit, rounded to thousands. Cross-cutting implementation stages get explicit additive low/base/high allowances in `build_docs.py`: bundle 0.12/0.40/1.30 M; normalization 0.10/0.32/1.10 M; typed IR 0.18/0.60/2.00 M; data lowering 0.28/1.00/3.60 M; control/runtime lowering 0.45/1.60/5.50 M; emission 0.12/0.42/1.40 M; publication/adoption 0.06/0.18/0.60 M; required `_2` completion 0.15/0.50/1.70 M; manual delivery 0.05/0.18/0.65 M; combined delivery/integration assurance 0.18/0.65/2.20 M. Post-gate milestone allowances are multiplied by 1.6 for evidence construction and interpretation. These rates are engineering judgment, not fitted coefficients.

Assumptions for aggregate scenarios:

1. **Direct:** all manual production, manual assurance/delivery, plus delayed-production-rework reserve. It does not claim to finish the separately required CPG objective.
2. **CPG to finished Agones:** all CPG production and assurance/delivery, manual assurance/delivery, and 35% / 55% / 85% of manual production for unavoidable application contracts, lifecycle/runtime integration and adoption. This explicit overlap assumption is a major uncertainty; generated code is not presumed ready for use.
3. **Both:** all milestone allowances plus boundary/recovery overhead. Shared stage work is charged to its owning milestone once. Later consumption of an existing artifact is not a second implementation credit.
4. Add production-repair reserve of 10% / 25% / 60% to account for errors discovered after delayed verification, compiler/ownership issues, dependency mismatches and integration defects. This reserve is additional repair work; the assurance allowance covers writing/running/interpreting required evidence. Charge real repair work to one segment, never both reserve and milestone actuals.
5. Boundary overhead is 2,000 / 10,000 / 40,000 tokens per planned route boundary, including small checkpoint reloads, interruption recovery and accounting. Ordinary within-milestone context loading is already in milestone rates. No full re-analysis is budgeted after every milestone.
6. Required tooling is available through permitted already runnable executables, or the blocker is resolved by later explicit user direction. Tool installation/build/compatibility uncertainty is included as planning work, **not authorization** to violate the gate. The present forecast cannot promise elapsed completion while this prerequisite is unresolved.

The baseline [forecast-baseline-v1.json](verification/agones/roadmap/forecast-baseline-v1.json) contains milestone values, aggregate formulas, IDs, assumptions and provenance. Freeze v1 when execution begins; later revisions are new files with explicit changes. The baseline SHA-256 belongs in each comparison record. Predicted values are not Goal budget settings.

## 5. Conditional remaining-token scenarios

| Finish line | Low assumption | Base assumption | High assumption |
|---|---:|---:|---:|
@@SCENARIO_TABLE@@

`M` means million tokens in the host's eventual comparable token unit. The host may expose a different accounting basis or only partial counters; reconcile definitions before comparing. Credits remain `unknown` in every scenario. Large ranges reflect the unresolved behavioral universe, reuse/adoption quality, constrained assurance timing and compiler/runtime semantics.

| Scenario | Production | Assurance/delivery | Delayed repair reserve | Boundary/recovery | Total tokens |
|---|---:|---:|---:|---:|---:|
@@DETAIL_TABLE@@

The both-route sum is conservative about duplicated engineering; it is not a reason to implement the same body twice. A future source-bound reuse decision may lower the remaining forecast, preserving this original baseline. Already spent tokens are sunk and excluded from all remaining totals.

## 6. Speed, difficulty and break-even conditions

The direct route likely needs fewer new semantic layers and can apply the current Rust implementation immediately. It is consequently likely easier and faster **to first finished Agones-rs**, conditional on comparable models, throughput and interruption rates. No hours or calendar-date claim is defensible from the available evidence. Mandatory alternation intentionally changes time to the first deliverable because CPG work is on the execution path.

The pipeline becomes cheaper for the same first deliverable only when:

```text
remaining native CPG + frontend/IR/lowering + emission + pipeline assurance
    + extra generated-output adoption/repair
< manual implementation avoided + manual-only rework avoided
```

Shared final Agones assurance cancels only where the obligations and production bindings are genuinely identical. The historical native port-map count cannot establish either side. Existing reusable schema modules improve the left side, but cannot alone show that Go concurrency, aliasing, initialization, failure behavior and Rusternetes effect lowering are complete. Repeated migrations or future regeneration can amortize tooling cost; those future benefits are outside this two-output task and are not counted as present savings.

The low/base/high columns are joint assumptions, not independent statistical bounds. Their overlap means the ranking can change if pipeline readiness and direct rework differ substantially from today's evidence. Compare actuals by delivered source obligations and stage type; raw tokens from unlike milestones do not constitute a controlled experiment or prove a causal route advantage.

## 7. Actual usage, sunk cost and missing measurements

`AGONES_RS_STATE.json` retains a historical cumulative Goal observation of **9,834,797 tokens** (`observed_at: 1792576037`). It includes mixed work without boundaries adequate to assign it to either route. Preserve it as historical/unattributed; do not split it by file counts, ratios, current milestone ownership or account usage percentages. It did not calibrate this forecast.

No new implementation milestone has actual usage in this report. Authoring this roadmap is shared planning work, and this session has no attributable implementation-milestone start/end pair. Do not convert its token use into fabricated milestone cost. Account-wide percentages, subscription limits and reset windows are not charges. Credit consumption remains `unknown` until an attributable charge is available with provenance.

After each real completed milestone, record the start/end observations, raw counter source and accounting domain, attributable delta, immutable prediction, signed and absolute differences, ratio where defined, delivered scope, deviation explanation, shared/rework/interruption segments and revised future forecast. See [comparison instructions](cost-analysis-comparison/README.md) and the sole [accounting policy](roadmap/alternation-policy.md). If an end counter finalizes only after the turn, retain PROVISIONAL and finalize from the next persisted observation. Resets/missing boundaries are unknown attribution, never zero cost.

## 8. Risks and explicit blockers

1. **Strict gate feasibility:** a newly authored emitter may require a prohibited build before full output exists. No allowed executable was established by this documentation task; absence at a few expected paths is not an exhaustive tool search. Record the exact missing executable/source closure as a dependency blocker when encountered. Do not move generation or production integration behind the gate, run a disguised probe, or claim the milestone complete.
2. **Coverage:** all source-tree paths are inventoried, but nested/implicit bodies, platform/build-tag configurations and dynamic test instances still need explicit source accounting. Lexical declarations are not a complete semantic universe. Unknown required production scope blocks the gate.
3. **Reuse:** schema source blobs match while Rust behavior remains unverified; current pins and reference HEADs differ. Preserve exact bindings, make narrow source-supported adaptations, and defer compatibility execution.
4. **Architecture and names:** exact Go/Scala hierarchy, unusual filenames, external module glue and full CGP/derive-deftly lifecycle ownership create real adaptation work. Pythonic readability is coequal with idiomatic Rust and cannot erase source semantics.
5. **Delayed evidence:** errors can propagate across more authored code before discovery. The user-selected gate is binding; the report prices uncertainty rather than weakening it.
6. **Alternation dependencies:** the authored schedule is topologically ordered. New source discoveries can reveal a missing prerequisite. Mark the required milestone blocked and repair its prerequisite under the policy; never invent empty milestones to claim alternation.

## 9. Immutable milestone prediction index

Values below exclude the separately tabulated aggregate repair reserve and boundary overhead. Each report references `F-MNNNN-v1`. All milestones are PLANNED_NOT_EXECUTED.

| Milestone | Route | Work | Low tokens | Base tokens | High tokens |
|---|---|---|---:|---:|---:|
@@MILESTONE_TABLE@@

## 10. Analysis stop condition

This bounded source/document inspection is sufficient for the initial decision and forecast. Do not spend more tokens attempting to manufacture precision. Revise only affected future assumptions from attributable milestone observations or concrete new source facts. Forecast uncertainty does not weaken required behavior, source coverage, alternation, architecture, naming, assurance or delivery.
'''
    return report.replace('@@REUSE_COUNT@@',str(reuse_count)).replace('@@GENCO_QUOTE@@',author.passage('genco','README.md','Central to genco',8)).replace('@@SCENARIO_TABLE@@','\n'.join(table)).replace('@@DETAIL_TABLE@@','\n'.join(detail)).replace('@@MILESTONE_TABLE@@','\n'.join(milestone_rows))
