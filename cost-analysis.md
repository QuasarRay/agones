# Agones-rs route cost analysis

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

The [source inventory](verification/agones/roadmap/source-inventory.json), [schema reconciliation](verification/agones/roadmap/schema-reconciliation.json), [existing implementation ownership map](verification/agones/roadmap/reuse-inventory.json) and [source identities](verification/agones/roadmap/sources.json) are the audit inputs. The ownership map contains 245 existing Rust/Python implementation/tool files; it is a preservation/adoption index, not a claim that each body has been reconciled. Exact source-to-current-body adoption remains an implementation obligation with explicit unresolved status.

The selected frontend uses Joern `9f2171242e48d7fed3021087d45838a83c1e39da` with CPG v1.7.79 and goastgen `11b21e395aa6aa39a41ca9216e1d15d08c4ef0b8`; retain Flatgraph v0.1.36 source semantics, Grafeo 0.5.44 native storage and selected Genco 0.17.10. An arbitrary installed binary is not evidence of this closure. The checkpoint and historical receipts are dated evidence, not executable availability guarantees.

## 3. What each route still owes

**Direct Agones.** Resolve current source-to-Rust ownership, place original-name files in the literal mirror, finish APIs/configuration/defaulting, clients/watch/queue contracts, ports, GameServer/GameServerSet/Fleet/autoscaling, allocation/processor, SDK streams/health/counters/lists, entrypoints and delivery inputs. Complete CGP/derive-deftly lifecycle ownership and the required `_2` family. Only after all production work on both required routes is authored may assurance begin. In the direct-only counterfactual, its gate would cover only that counterfactual's production scope.

**CPG pipeline.** Reconcile CPGSchema reuse; preserve constructor dependencies, mutable schema identity and full schema facts; finish graph/domain/loading/persistence/passes/build tooling; finish compatible source extraction/bundles, normalization, typed IR, Go value/control/concurrency/error lowering and runtime support; emit readable valid Rust through Genco; publish complete source/body provenance transactionally; author real production adoption; then establish source/schema/generator/Agones assurance. A schema dump, copied Rust body or source-shaped stub does not complete these stages.

**Both under alternation.** Share API and lifecycle contracts, schema declarations, effect adapters, lowering rules and final evidence where their bindings permit reuse. Keep a complete separate generated artifact and one canonical production owner per adopted output. The model budgets both workstreams conservatively; it does not assume all manual implementation effort disappears when generation is available. Do not duplicate behavior solely to measure which route would have been cheaper.

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-4aa5b67b0b028cd01b.** [genco/README.md, lines 10–17](https://github.com/udoprog/genco/blob/98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb/README.md#L10-L17). Revision `98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb`; source SHA-256 `0e04298e93fe8231a498b29d8752708f44cef62545e7cda4860b2cc80a96bbf2`.

> Central to genco are the [`quote!`] and [`quote_in!`] procedural macros which
> ease the construction of [token streams].
> 
> This project solves the following language-specific concerns:
> 
> * **Imports** — Generates and groups [import statements] as they are used.
>   So you only import what you use, with no redundancy. We also do our best
>   to [solve namespace conflicts].


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
| Direct completion of Agones-rs only | 2.51 M | 8.67 M | 31.99 M |
| Full CPG pipeline → finished Agones-rs | 3.85 M | 14.60 M | 62.20 M |
| Both complete under mandatory alternation | 4.71 M | 16.92 M | 66.35 M |

`M` means million tokens in the host's eventual comparable token unit. The host may expose a different accounting basis or only partial counters; reconcile definitions before comparing. Credits remain `unknown` in every scenario. Large ranges reflect the unresolved behavioral universe, reuse/adoption quality, constrained assurance timing and compiler/runtime semantics.

| Scenario | Production | Assurance/delivery | Delayed repair reserve | Boundary/recovery | Total tokens |
|---|---:|---:|---:|---:|---:|
| Direct completion of Agones-rs only / low | 1,098,000 | 1,307,000 | 109,800 | 0 | 2,514,800 |
| Direct completion of Agones-rs only / base | 3,462,000 | 4,341,000 | 865,500 | 0 | 8,668,500 |
| Direct completion of Agones-rs only / high | 11,135,000 | 14,173,000 | 6,681,000 | 0 | 31,989,000 |
| Full CPG pipeline → finished Agones-rs / low | 2,026,300 | 1,622,000 | 202,630 | 0 | 3,850,930 |
| Full CPG pipeline → finished Agones-rs / base | 7,334,100 | 5,434,000 | 1,833,525 | 0 | 14,601,625 |
| Full CPG pipeline → finished Agones-rs / high | 27,731,750 | 17,829,000 | 16,639,050 | 0 | 62,199,800 |
| Both complete under mandatory alternation / low | 2,740,000 | 1,622,000 | 274,000 | 74,000 | 4,710,000 |
| Both complete under mandatory alternation / base | 8,892,000 | 5,434,000 | 2,223,000 | 370,000 | 16,919,000 |
| Both complete under mandatory alternation / high | 29,402,000 | 17,829,000 | 17,641,200 | 1,480,000 | 66,352,200 |

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
| [M0001](roadmap/0001-source-mirror-version-and-shared-contracts.md) | MANUAL | Source mirror, version and shared contracts | 14,000 | 35,000 | 103,000 |
| [M0002](roadmap/0002-pinned-cpg-source-build-machinery-and-rust-support-boundaries.md) | CPG | Pinned CPG source, build machinery and Rust support boundaries | 16,000 | 40,000 | 120,000 |
| [M0003](roadmap/0003-errors-queues-clocks-webhooks-and-utility-effects.md) | MANUAL | Errors, queues, clocks, webhooks and utility effects | 44,000 | 135,000 | 427,000 |
| [M0004](roadmap/0004-native-cpg-utilities-and-effect-ownership.md) | CPG | Native CPG utilities and effect ownership | 12,000 | 28,000 | 80,000 |
| [M0005](roadmap/0005-resource-and-wire-contracts-defaults-and-validation.md) | MANUAL | Resource and wire contracts, defaults and validation | 113,000 | 366,000 | 1,177,000 |
| [M0006](roadmap/0006-complete-schema-construction-and-mutually-dependent-declarations.md) | CPG | Complete schema construction and mutually dependent declarations | 65,000 | 205,000 | 655,000 |
| [M0007](roadmap/0007-apply-clients-informers-and-listers.md) | MANUAL | Apply, clients, informers and listers | 14,000 | 37,000 | 108,000 |
| [M0008](roadmap/0008-schema-export-protobuf-tool-inputs-and-launcher-behavior.md) | CPG | Schema export, protobuf tool inputs and launcher behavior | 8,000 | 17,000 | 43,000 |
| [M0009](roadmap/0009-source-compatible-clients-and-existing-rust-sdk-reuse.md) | MANUAL | Source-compatible clients and existing Rust SDK reuse | 42,000 | 130,000 | 411,000 |
| [M0010](roadmap/0010-complete-schema-derived-domain-hierarchy.md) | CPG | Complete schema-derived domain hierarchy | 151,000 | 491,000 | 1,583,000 |
| [M0011](roadmap/0011-port-ownership-and-allocation-policy.md) | MANUAL | Port ownership and allocation policy | 16,000 | 40,000 | 119,000 |
| [M0012](roadmap/0012-native-cpg-facade-and-graph-ownership.md) | CPG | Native CPG facade and graph ownership | 7,000 | 12,000 | 28,000 |
| [M0013](roadmap/0013-cloud-hooks-through-the-permitted-rusternetes-boundary.md) | MANUAL | Cloud hooks through the permitted Rusternetes boundary | 20,000 | 54,000 | 164,000 |
| [M0014](roadmap/0014-serialized-graph-and-overlay-behavior.md) | CPG | Serialized graph and overlay behavior | 6,000 | 10,000 | 20,000 |
| [M0015](roadmap/0015-fleet-reconciliation-and-rollout-policy.md) | MANUAL | Fleet reconciliation and rollout policy | 27,000 | 80,000 | 248,000 |
| [M0016](roadmap/0016-protobuf-loading-archive-ownership-and-node-mapping.md) | CPG | Protobuf loading, archive ownership and node mapping | 11,000 | 24,000 | 68,000 |
| [M0017](roadmap/0017-telemetry-and-metric-semantics.md) | MANUAL | Telemetry and metric semantics | 40,000 | 121,000 | 383,000 |
| [M0018](roadmap/0018-format-detection-and-native-storage-compatibility.md) | CPG | Format detection and native storage compatibility | 7,000 | 13,000 | 32,000 |
| [M0019](roadmap/0019-gameserver-and-gameserverset-lifecycle.md) | MANUAL | GameServer and GameServerSet lifecycle | 92,000 | 294,000 | 945,000 |
| [M0020](roadmap/0020-pass-diff-and-execution-lifecycle.md) | CPG | Pass, diff and execution lifecycle | 14,000 | 35,000 | 103,000 |
| [M0021](roadmap/0021-fleetautoscaler-policies-and-external-decisions.md) | MANUAL | FleetAutoscaler policies and external decisions | 38,000 | 114,000 | 360,000 |
| [M0022](roadmap/0022-pinned-frontend-and-self-contained-cpg-input-bundle.md) | CPG | Pinned frontend and self-contained CPG input bundle | 125,000 | 405,000 | 1,305,000 |
| [M0023](roadmap/0023-allocation-processing-and-multi-cluster-contracts.md) | MANUAL | Allocation processing and multi-cluster contracts | 80,000 | 255,000 | 816,000 |
| [M0024](roadmap/0024-normalized-cpg-with-source-identity-preservation.md) | CPG | Normalized CPG with source identity preservation | 105,000 | 325,000 | 1,105,000 |
| [M0025](roadmap/0025-sdk-server-streaming-health-and-mutation-behavior.md) | MANUAL | SDK server streaming, health and mutation behavior | 60,000 | 188,000 | 600,000 |
| [M0026](roadmap/0026-typed-translation-ir-and-ownership-effect-interfaces.md) | CPG | Typed translation IR and ownership/effect interfaces | 185,000 | 605,000 | 2,005,000 |
| [M0027](roadmap/0027-production-entrypoints-and-complete-lifecycle-composition.md) | MANUAL | Production entrypoints and complete lifecycle composition | 72,000 | 227,000 | 727,000 |
| [M0028](roadmap/0028-go-value-type-and-initialization-lowering.md) | CPG | Go value, type and initialization lowering | 285,000 | 1,005,000 | 3,605,000 |
| [M0029](roadmap/0029-examples-source-tooling-and-compatibility-inputs.md) | MANUAL | Examples, source tooling and compatibility inputs | 129,000 | 419,000 | 1,352,000 |
| [M0030](roadmap/0030-go-control-errors-concurrency-and-runtime-lowering.md) | CPG | Go control, errors, concurrency and runtime lowering | 455,000 | 1,605,000 | 5,505,000 |
| [M0031](roadmap/0031-build-deployment-and-configuration-implementation.md) | MANUAL | Build, deployment and configuration implementation | 142,000 | 462,000 | 1,490,000 |
| [M0032](roadmap/0032-genco-emission-into-the-exact-rust-source-mirror.md) | CPG | Genco emission into the exact Rust source mirror | 125,000 | 425,000 | 1,405,000 |
| [M0033](roadmap/0033-complete-the-required-metaprogramming-authoring-family.md) | MANUAL | Complete the required metaprogramming authoring family | 155,000 | 505,000 | 1,705,000 |
| [M0034](roadmap/0034-full-generation-coverage-receipts-and-recoverable-publication.md) | CPG | Full generation, coverage receipts and recoverable publication | 65,000 | 185,000 | 605,000 |
| [M0035](roadmap/0035-agones-harnesses-and-final-behavioral-assurance.md) | MANUAL | Agones harnesses and final behavioral assurance | 1,219,000 | 4,045,000 | 13,125,000 |
| [M0036](roadmap/0036-cpg-schema-lowering-and-generation-assurance.md) | CPG | CPG, schema, lowering and generation assurance | 19,000 | 45,000 | 128,000 |
| [M0037](roadmap/0037-agones-acceptance-and-delivery-preparation.md) | MANUAL | Agones acceptance and delivery preparation | 88,000 | 296,000 | 1,048,000 |
| [M0038](roadmap/0038-generated-agones-integration-and-shared-objective-closure.md) | CPG | Generated Agones integration and shared objective closure | 296,000 | 1,048,000 | 3,528,000 |

## 10. Analysis stop condition

This bounded source/document inspection is sufficient for the initial decision and forecast. Do not spend more tokens attempting to manufacture precision. Revise only affected future assumptions from attributable milestone observations or concrete new source facts. Forecast uncertainty does not weaken required behavior, source coverage, alternation, architecture, naming, assurance or delivery.
