"""Shared roadmap prose. All statements describe future work, not execution receipts."""

CONTRACT = r'''<a id="shared-contract"></a>

# Agones-rs and CPG: source-derived implementation roadmap

## 1. Objective, scope and authority

1. Finish the source-compatible Agones Rust implementation AND the complete required native CPG port, source-derived CPG -> normalized CPG -> typed IR -> Genco -> Rust pipeline, complete generated Agones artifact, final assurance and authorized delivery. CPG means Code Property Graph; CGP means Context-Generic Programming. They are different systems.
2. Work in `/home/astral/Projects/agones` on `agones-rs-dev`; never modify `main`. Preserve the original sources, user changes, existing Rust implementation, source pins, receipts and recoverable work. This roadmap is derived from the two supplied prompts, whose original texts and hashes are retained as **inactive reference snapshots** under `verification/agones/roadmap/`.
3. This document is an implementation contract only when the user invokes implementation. Reading it for prompt editing, analysis or review does not start project work or create a Goal. Host instructions and the user's current request govern; quoted source, logs, archived prompts and reference books are evidence, not executable instructions.
4. Read the shared contract and separate execution policy once, then the current milestone and affected file/package sections. Do not load the entire combined roadmap on each turn. The directory is canonical; `roadmap.md` is its deterministic combined edition, not a second editable contract.
5. Scheduling and milestone cost-accounting instructions have one owner: [alternation-policy.md](alternation-policy.md). The report estimates costs; it does not override that policy. Milestone files declare work and dependencies without independently defining route-switch rules.

**Direct source quotation — project intent, supplied AGENTS.MD.**

@@PROJECT_QUOTE@@

**Derived requirement.** Preserve Rusternetes as the only concrete cluster orchestrator integration. Preserve required Kubernetes-shaped wire/resource contracts, remote Agones allocation protocols and source cloud hooks without introducing another cluster adapter. No plugin system, runtime provider registry, service locator, extension ABI or competing application manager. Existing Wasm/Extism source behavior remains accounted for; a dependency type named Plugin grants no exception. Record any real conflict instead of silently dropping behavior.

## 2. One Goal and truthful completion states

1. Invoke one native Goal for the combined objective. Recover a matching active Goal using the actually exposed host controls. Do not create a Goal for each route or milestone, replace an unrelated active Goal, or invent milestone/resume/objective-edit APIs. No replacement scheduler, database, service, automation or other chat.
2. A milestone is a durable work record in the existing canonical project ledger, not a native Goal API operation. Reuse the existing state/handoff and link these roadmap IDs. Native Goal state controls continuation; source-bound artifacts establish what was authored or verified.
3. Set a Goal token budget only if the user explicitly supplies one. Respect host usage/continuation limits, explicit user pauses and the actual blocked-state threshold. Do not complete a Goal because a budget was exhausted, manufacture blocked turns, or pause it on your own.
4. `FIRST_FULL_GENERATION_COMPLETE` records an artifact milestone in this combined objective. Recover an intact publication without regeneration, then continue the remaining roadmap when authorized. It does not close the shared Goal. The archived standalone prompt's terminal branch is superseded for this invocation.
5. Keep these fields distinct:

```text
project_phase = IMPLEMENTATION_ONLY | POST_IMPLEMENTATION_VERIFICATION
implementation_status = IN_PROGRESS | IMPLEMENTATION_COMPLETE_UNVERIFIED
milestone_stage = PLANNED | IMPLEMENTING | IMPLEMENTED_UNVERIFIED |
                  ASSURANCE_IN_PROGRESS | VERIFIED | DELIVERED
milestone_disposition = READY | BLOCKED | STALE | UNSUPPORTED | COMPLETE
verification_status = NOT_STARTED | IN_PROGRESS | INCOMPLETE | COMPLETE
migration_status = IN_PROGRESS | COMPLETE
```

6. `IMPLEMENTED_UNVERIFIED` means required source-owned behavior is authored and connected. It is not compilation, correct behavior, formal proof or production readiness. Mark the shared Goal complete only after both full scopes, all required architecture/readability/naming obligations, final assurance and authorized delivery are complete. Retain exact unresolved identities.

## 3. Pinned source and exhaustive accounting

1. Agones reference: `6eb4a02588d8e7b581de187f3bc849733a78dcb2`. CPG reference selected by the retained frontend: `301c4010d28ecdac05bc2ed44310f56446459465` (v1.7.79). Preserve the recorded compatible Joern, goastgen, Flatgraph, Grafeo and Genco identities. Reference checkouts and their newer HEADs are listed separately in [sources.json](../verification/agones/roadmap/sources.json); do not upgrade pins merely because source is vendored.
2. The frozen [source inventory](../verification/agones/roadmap/source-inventory.json) accounts for @@AGONES_COUNT@@ Agones paths and @@CPG_COUNT@@ CPG paths. @@DETAIL_COUNT@@ files have detailed roadmap entries. Every other path has an explicit retained-reference, existing-migration, guidance or third-party disposition; none is silently dropped. File/declaration counts do not establish complete semantic body coverage.
3. Preserve deprecated-but-served behavior and all source configurations. Required production scope includes initializers, closures/callbacks, generated source, API/schema/proto data, configuration, resources, build/tool behavior and external-call support. Enumerate Go build-tag/platform/cgo/module/replacement/vendor variants before claiming a complete body universe. Existing dynamic test/assertion counts remain unresolved until post-gate extraction.
4. Full CPG scope includes every handwritten production module and its build/schema machinery, including modules unused by Agones. Regenerate generated domain/protobuf material from complete inputs. Do not reimplement underlying third-party libraries wholesale, but retain and map all CPG semantics provided through those dependencies.
5. Both recorded CPG revisions have identical `schema/src/main/scala` source blobs. Twenty-five corresponding native module files are present in CPGSchema. [schema-reconciliation.json](../verification/agones/roadmap/schema-reconciliation.json) records the source identities and reuse candidates. This is source/path reconciliation only: Rust behavior, builder semantics, dependencies and complete body coverage remain unverified. Do not subtract module counts from the historical 695 candidate-body gap.
6. Preserve original fixture-qualified identities and source/file/body/configuration provenance. A rename retains aliases/tombstones; exclusions require explicit user authority. Unknown required production scope prevents the global gate; unknown required assurance scope prevents final completion.

7. Use [entry-index.tsv](../verification/agones/roadmap/entry-index.tsv) to load only the current milestone overview and affected file sections by line range. The [unresolved-scope register](../verification/agones/roadmap/unresolved-scope.json) makes remaining body/configuration, reuse, executable and assurance accounting explicit. Neither the combined document nor the full JSON census belongs in every agent context.

8. Quotations retain their original repository ownership and applicable license/NOTICE terms. Preserve the fork's [LICENSE.MD](../LICENSE.MD), [LICENSE-LEGACY.MD](../LICENSE-LEGACY.MD), [CPG license](../third_party/first-generation/cpg/LICENSE) and upstream notices in the retained trees; additional license paths are indexed in sources.json. The surrounding engineered requirements do not relabel implementation excerpts as official prose or change ownership of quoted material.

## 4. Literal hierarchy and one owner per output

1. Preserve the original source-relative hierarchy inside `agones-rs/` and `cpg-rs/`. Preserve every directory spelling, case, filename stem and singular/plural form. Retain CPG paths such as `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/` literally.
2. Apply only the language mapping: `.go` -> `.rs`, `.scala` -> `.rs`, `.java` -> `.rs`. The Go test exception takes precedence: `name_test.go` -> `name_harness.rs`, never `name_test.rs`. Scala `IOUtilsTest.scala` becomes `IOUtilsTest.rs`; do not invent a separate Scala test renaming rule.
3. Preserve names such as `sdk.pb.go` -> `sdk.pb.rs`, `zz_generated.deepcopy.go` -> `zz_generated.deepcopy.rs`, and `CpgSchema.scala` -> `CpgSchema.rs`. Use explicit path attributes, raw identifiers where legal, or private registration identifiers in Rust-only glue. Record necessary identifier mappings without changing source filenames or public compatibility names.
4. Put Cargo entrypoints, registration and new language-specific support in `rust-support/agones/` and `rust-support/cpg/`, outside the mirrored hierarchy. These support paths are additive Rust machinery, not purported upstream files. Retain non-Rust build inputs by name and map any executable source build semantics to concrete Rust support.
5. Keep existing `crates/`, `cpg-rs/src/` and CPGSchema work until a source-to-destination adoption map identifies every replaced body, caller and owner. Never reset, silently stash, delete or overwrite user work to obtain a clean mirror.
6. A source body has one canonical production owner. Generated output is staged separately in `generated/agones-first-generation/` with the same source-relative mirror. Author and connect all required adoption/ownership-transfer paths before the global gate; preserve manual edits or move their meaning into source-bound rules. After the gate, establish integration evidence and repair defects under the gate-reopening rule. Handwritten bodies are not CPG-derived merely because a generator copied them.
7. Name collisions, illegal Rust identifiers and source layouts requiring support glue are explicit inventory facts. Do not silently normalize names. Retain source and target mapping even when one shared implementation serves multiple source bodies.

## 5. CGP AND derive-deftly own architecture and lifecycle

@@CGP_QUOTE@@

@@DEFTLY_QUOTE@@

**Definition.** Joint ownership means one architectural cycle:

```text
source contract -> typed declaration -> derive-deftly template
 -> CGP provider implementation/delegation -> static production context
 -> CGP lifecycle consumer -> explicit effects and owned resources
```

1. CGP owns capabilities, providers, associated/context-provided types, dependency direction, runtime policy, mutable resources and child tasks. Derive-deftly owns reusable declaration-to-implementation structure, lifecycle wiring, delegation adapters and ownership metadata. Both must occur on real production paths in both projects; decorative dependencies and toy demonstrations do not count.
2. Derive-deftly operates at compile time. The generated Rust participates in runtime ownership through CGP. Do not claim that a template macro itself schedules tasks or that CGP automatically supplies application-specific lifecycle semantics.
3. Record one subsystem ownership entry: source/feature IDs; declaration and template; CGP capability/provider; production context; mutable state/resource owner; effect boundary; initialization, readiness, cancellation, drain and cleanup responsibilities.
4. Include every controller, allocator, server, SDK stream, queue, loader, pass, generator stage, publisher, background task and shutdown path. The entrypoint constructs the context and invokes the lifecycle consumer. No detached application tasks or parallel hand-maintained manager may bypass composition.
5. Preserve source policy for partial/repeated startup, auxiliary versus primary failures, retry exhaustion, panics, lost watches, conflicts, shutdown during I/O, drain deadlines and cleanup. Parent owners cancel and join owned children; runtime libraries supply effects through capabilities.
6. Distinguish public/wire types, portable policy and Rusternetes transport/storage types. Keep fallible conversions at the adapter boundary and preserve required unknown/optional fields. A neutral name is not architectural independence. Reusable policy consumes minimum capabilities rather than a universal orchestrator registry.
7. Reuse the existing Grafeo canonical graph/persistence owner in CPG. Borrowed views and caches are disposable projections, not independently writable graphs. Preserve source identities, numeric/encoding semantics and storage compatibility boundaries.

## 6. Pythonic readability is a coequal design requirement

@@COMPRENDE_QUOTE@@

@@BETTER_QUOTE@@

1. Evaluate authored Rust, templates, generated Rust, APIs, examples, diagnostics and later harnesses from a Python developer's viewpoint. Pythonic readability and idiomatic Rust have equal standing. Neither permits changing source semantics, memory safety, ownership or public compatibility.
2. Prefer small named functions, explicit data transformations, straightforward control flow, concise comprehensions/iterators and declarative composition. Explain non-obvious borrowing, moves and error propagation beside the reusable pattern. Keep domain behavior readable without inspecting an expansion engine.
3. Use comprende's `c!` for concise mapping/filtering/collection operations; use better_comprehension when explicit collection choice, pattern/binding or lazy iteration improves clarity. Pick one convention for overlap and reuse it. Do not duplicate a transformation to exercise both libraries.
4. Preserve eager/lazy execution, evaluation count and order, short-circuiting, ownership, allocation, duplicates and permitted output order. A Rust HashMap does not acquire Python insertion ordering. Use an ordinary loop when an effect or cleanup sequence is clearer; record a short local reason for a non-obvious exception.
5. Keep valid Rust: explicit types, traits/generics, Option/Result and normal error propagation. No Python interpreter, dynamic object emulation, unnecessary boxing/cloning, runtime macro registry or new authoring language.
6. Preserve literal source names even where ordinary Rust spelling conventions differ. Make the registration layer explain the exception. Do not make Python familiarity an excuse to rename source files or conceal error/resource ownership.

**Worked reading pattern (schematic, not a compiled example):**

```rust
let selected = c![resource.id() for resource in &resources
                  if policy.accepts(resource)];
let decision = decide_update(&current, &desired)?;
context.apply_update(decision).await?;
```

The reader sees selection, decision and effect in order. CGP supplies the effect capability; derive-deftly supplies repeated declarations and wiring. The actual operation, errors and ordering come from the quoted source contract.

## 7. Absolute implementation-first gate

1. Default to `IMPLEMENTATION_ONLY`. Make ZERO attempts to test or verify any project component until all required production implementation across BOTH routes is authored and connected and every required generation output is present. A completed feature, crate, generator stage or milestone is insufficient.
2. Before the gate, prohibit Cargo check/build/test/clippy/doc/doctest, formatter/lint campaigns, rust-analyzer diagnostics, compiler/macro/API probes, fixture/smoke/example/application runs, fuzzing, benchmarks, coverage, source replay, Bolero/Kani/proofs, schema/generator validation, regeneration comparisons and CI. Do not evade this through wrappers, hooks, remote jobs or other agents.
3. Defer new semantic property extraction, harness/extraction infrastructure and assurance fixtures. Reading source tests as specifications and authoring this roadmap is permitted; executing tests or generating a property campaign is not.
4. Permitted work is source/doc reading, source-to-destination bookkeeping, necessary implementation edits, declarations/templates/rules, production configuration, local checkpoints and content identity/recovery. An already runnable generator may emit required implementation files without validation/test hooks. Its output accounting is not correctness evidence.
5. Do not compile a new generator, bootstrap project code, or run a compatibility probe under a generation label. If emission needs a forbidden build, preserve the exact dependency blocker. Do not move the output requirement behind the gate or label incomplete generation complete to escape this constraint. The cost report's finite scenarios are conditional on a permitted executable path.
6. The global gate requires every required production row, all source-owned error/edge paths, Rusternetes integration, complete CGP/derive-deftly lifecycle wiring, production generators/support/output, and all four required metaprogramming forms in both locations. No unknown production scope, required stubs, fake success, disconnected bodies or omitted source operations may remain.
7. Record `IMPLEMENTATION_COMPLETE_UNVERIFIED` with contract/baseline/scope IDs, source snapshot/dirty binding, inventory revision, exact authored source/entrypoint mappings, ownership, required generated artifact identity and zero known missing required implementation. This is an authored-completeness declaration, not proof that the program is correct.
8. Only then enter `POST_IMPLEMENTATION_VERIFICATION`, create required assurance tooling and execute deferred checks. Defects or production refactors close the gate: finish the repair, renew the global record, and resume affected checks. Harness-only corrections after the gate remain assurance work. Missing/stale/unsupported results never pass.

## 8. Complete generation and recoverable publication

1. Preserve the pipeline `CPG bundle -> normalized CPG -> typed translation IR -> Genco tokens -> generated workspace`. Persist AST/type enrichment with source/configuration identity; do not bypass CPG with an untracked Go-text emitter.
2. Preserve complete schema facts: hidden elements, inheritance, types/defaults/cardinalities, identifiers/encodings, endpoints/order, contained relationships and mutable identity. Keep standard Schema2Json behavior separate from the richer internal representation.
3. Implement all required Go semantic families, including initialization, generics, interfaces, aliasing, numeric conversions, strings/maps/slices, errors, defer/panic/recover, goroutines/channels/select, context/clocks and external calls. Retain unresolved facts and unsupported operations as required gaps.
4. Every required body needs a substantive source-derived implementation and operation/support mapping. A declaration, comment, missing helper, success constant or unsupported-operation placeholder is insufficient. Trace legitimate source panic/no-op/error behavior separately from omissions. Record approximations without claiming correctness.
5. Publish a separate complete artifact transactionally: freeze inputs; stage output and receipts; emit identity mappings and payload digests; write a completion record binding the manifest without circular self-hashing; flush/close; atomically publish the complete directory. A staged flag or multi-step copy is not publication.
6. Retain contract/source/configuration/schema/frontend/tool/dependency/rule/IR/emitter identities, exact required/emitted/missing sets, body-to-output spans, known approximations and source rights/notices. Set compilation, correctness, compatibility and production readiness to `UNVERIFIED` until later evidence establishes each category.
7. Recover intact published artifacts using minimal identity/digest bookkeeping. Preserve incomplete transactions and avoid duplicate jobs. No generation receipt alone completes the shared Goal or authorizes overwriting existing manual implementation.

## 9. Metaprogramming, assurance and delivery requirements

1. Complete `proc_macro_2`, `proc_macro_attribute_2`, `proc_macro_derive_2` and `proc_macro_delegate_2`, each supporting same-logical-crate definition/use and cross-crate export/import. Keep compiler shims/companion units internal and normal Cargo/IDE entrypoints. A stock imported macro or renamed helper crate does not satisfy local definition/use.
2. Reuse CGP, derive-deftly, existing Agones/Genco facilities, and the recorded Arcane-rs synthez/delegation/arcane patterns. Read actual pinned source for API gaps. Require a few-sentence remaining-work benefit before new generalization; no speculative framework or repeated cost investigation.
3. After the gate, demonstrate generics, visibility, aliases/re-exports, associated types/constants, async/lifecycle wiring, hygiene/diagnostics, clean/incremental rebuild and packaging where required. Preserve typed production semantics in ordinary generated Rust; never replace it under cfg(kani).
4. After the gate, preserve independent invariants for both reference models and production behavior. Agreement alone is insufficient. Retain exact examples/tables and fixture-qualified identities, then justify any generalized property with source evidence.
5. Share explicit domains/preconditions/assertions/required covers between Bolero and its Kani backend. Record exact selected harnesses, production paths, tool/compiler/backend/solver pins, bounds, unwind checks, input domains and reachable assertion/outcome witnesses. Sampling, reference proof, actual production-symbol proof, direct Kani, source replay and Rusternetes integration are separate evidence categories.
6. Preserve full source/environment/dependency closure and source-bound receipts. Empty, skipped, stale, unreachable, timed-out, unsupported, truncated or unsatisfied-cover results never count as success. Retain counterexamples and meaningful isolated negative controls. Do not weaken bounds, predicates or domains for a green result.
7. Pin application resolution and evidence toolchains; preserve immutable evidence across squash/restack. Source changes stale affected evidence; reuse genuinely unchanged obligations rather than replaying every campaign.
8. Keep local checkpoints during implementation. Avoid push/PR/merge events that would trigger forbidden checks. After eligibility, honor authorized stacked delivery into agones-rs-dev, verify exact review/merge revisions and required results, attach PRs through exposed host tools, and distinguish local, remote and visible-checkout identities. Never bypass protection or infer deployment permission from scripts.

## 10. Readability, documentation and handoff discipline

1. AGENTS.MD is the primary style reference: direct numbered obligations and concrete outcomes. Secondary book samples guide definitions, worked examples and ownership/failure explanations. They supply no new project requirements and no reason to run the commands they illustrate.
2. Preserve attribution and applicable notices for upstream excerpts and reused code. Each quote carries repository/revision/path/lines/hash; distinguish official documentation, doc comments, implementation excerpts and engineering conclusions. Dedicated prose gaps remain explicit.
3. Store only compact decisions, source mappings, status and next actions in the existing ledger; not private reasoning transcripts. Keep usage uncertainty and known blockers visible. Do not create parallel queues, a database or new background service.
4. At handoff report authored implementation, global gate, generation artifact, required assurance, delivery and cost observations separately. Report the exact next milestone and any blocked prerequisite. Document creation is not project implementation, and a forecast is not measured consumption.

## 11. Numbered milestone index

@@MILESTONE_TABLE@@

The order is derived from source-package imports plus declared schema/pipeline prerequisites. Allocation processing and converters share one cohesive ownership group. CpgSchema constructs modules that import its PropertyDefaults, so the complete schema is one cohesive milestone with the original internal constructor order. Resource/wire contracts, client/observation machinery, GameServer/GameServerSet lifecycle, and example/tooling work are grouped by their actual shared purpose. The file inventory retains all package and source identities; no source package is renamed by grouping. All milestones are planned, not executed.
'''

POLICY = r'''<a id="alternation-policy"></a>

# Mandatory route alternation and milestone cost accounting

## 1. Authority and source context

This file is the sole authoritative scheduling and cost-accounting policy for the combined roadmap. The milestone files contain work and dependency declarations. The cost report contains forecasts and conclusions. Neither independently changes this policy. The combined edition reproduces this file as a distinct component.

@@CGP_QUOTE@@

**Derived distinction.** CGP's static context/provider selection is application architecture. Switching the agent's implementation workstream is an execution-policy decision authorized by the user; it is not a runtime plugin or provider registry.

@@PROJECT_QUOTE@@

**Derived requirement.** Preserve work incrementally, keep the original reference and exact source mirror, and record concrete outcomes. The user's later strict gate postpones verification-triggering delivery; it does not remove the final delivery obligation.

## 2. One Goal, mandatory alternation

1. Recover one matching native Goal for the combined objective and one canonical repository ledger. Use only actual exposed controls such as get_goal/create_goal/update_goal. Create only on explicit invocation authorization. Never replace the Goal when changing routes and never mark it complete after a single milestone.
2. Start at the first unfinished dependency-ready numbered milestone. With no existing cursor, start with the MANUAL route. After each genuinely completed milestone, switch MANUAL -> CPG or CPG -> MANUAL. Preserve the cursor across turns, compaction and interruption; a new turn is not a new milestone.
3. Before selecting work, require all declared production prerequisites to be implemented and the milestone's phase to be open. Use the numbered dependency order; do not jump to a convenient later feature, fabricate completion, or split trivial fragments solely to create an alternation event.
4. If the required route has unfinished work but its next milestone is blocked, record the exact dependency/unlock condition and preserve the cursor. Perform useful work within that milestone if possible. Do not bypass the required route. Respect the native host's blocked-state threshold rather than manufacturing turns or unilaterally pausing.
5. Once a route has **fully completed all its required production, generation, assurance and delivery obligations**, continue the remaining route without artificial or empty milestones. A temporarily finished implementation phase is not completion of the entire route.
6. The strict global implementation gate applies to both routes. An unavailable generator or prohibited required build is a real blocker; do not start early verification or move missing outputs beyond the gate to bypass it.
7. Cost comparisons may improve reuse, packet size, forecasts and implementation tactics. They do not select a different route contrary to this mandatory policy. Do not replace it with an adaptive “choose the cheaper route” rule.
8. At FIRST_FULL_GENERATION_COMPLETE, record the artifact milestone and continue the shared roadmap under this policy. Do not take the archived standalone prompt's terminal Goal-completion branch.

## 3. Capture actual usage without inventing it

1. At milestone entry, record the actual Goal/thread/run identity, model/settings if exposed, current source snapshot, forecast ID/version and the raw host usage observation. At completion, record the corresponding ending observation.
2. Prefer the host's cumulative Goal token counter when its units and attribution are known. Preserve the raw field names and values; do not assume a new schema. Actual milestone tokens equal end minus start only for compatible, monotonic counters with complete boundaries and no unrelated included work.
3. If the host updates counters at turn completion, save a PROVISIONAL comparison, close the work milestone truthfully, and finalize usage from the next persisted observation. Do not repeatedly poll for an unavailable counter or rerun implementation to measure it.
4. For interruptions/resets, preserve disjoint observation segments with identities and timestamps; sum only known non-overlapping attributable deltas. Missing boundaries or mixed work remain UNKNOWN/UNATTRIBUTED rather than zero. Do not subtract counters from different Goals or sessions without a documented shared scope.
5. Record actual credit charges only if directly attributable charges are exposed. Otherwise set credits to unknown. Do not infer credits from tokens, money prices, account-wide usage percentages, reset windows, or unrelated activity. Input/cached/reasoning/output counts must not be double-counted when already included in a reported total.
6. Charge reading/planning, implementation, retries, context recovery, documentation/accounting and route-switch work exactly once. Shared setup belongs to a shared bucket or one explicitly documented allocation; never charge it in both routes. Roadmap authoring and historical cumulative usage are not silently assigned to the first implementation milestone.

## 4. Save the predicted-versus-actual comparison

1. After every completed milestone, create `cost-analysis-comparison/NNNN-MNNNN.md`. Preserve the original forecast row `F-MNNNN-v1` from cost-analysis.md and forecast-baseline-v1.json; do not rewrite it to match the observation.
2. Include milestone/route/Goal/source identities, delivered scope, prediction version, raw starting/ending observations, attributable actual tokens, predicted low/base/high, credits, interruptions/shared overhead, deviations and resulting future forecast changes.
3. When numeric values are available, calculate `delta = actual - predicted_base`, `absolute_difference = abs(actual - predicted_base)`, `ratio = actual / predicted_base` only when the denominator is positive, and whether actual lies inside the planning range. Do not treat that range as a statistical confidence interval.
4. Explain mismatches in terms of concrete scope, semantic complexity, reusable machinery, source gaps, failed attempts, context cost or deferred repair. Do not compare a small manual utility to compiler construction as if they were identical work. Use matched work classes and total remaining objective cost for route conclusions.
5. Add a compact historical record to the existing ledger and link the report. Forecast updates are append-only revisions in the comparison record or a new forecast file. Preserve the initial baseline, source assumptions and every prior comparison.
6. Keep the recurring comparison to approximately one page plus raw observations when needed. Refresh only affected assumptions. Do not repeat the full repository census, launch a benchmark/verification campaign, or implement duplicate behavior solely to measure cost.

## 5. Failure and recovery decisions

| Situation | Required action |
| --- | --- |
| Milestone source work incomplete | Keep its cursor and gaps open; no completion event or route switch. |
| Completed milestone, ending counter not finalized | Save PROVISIONAL accounting and recover the final counter next turn; do not repeat work. |
| No attributable token counter or credit charge | Record unknown with the exact unavailable boundary; preserve progress. |
| Counter reset or unrelated work included | Use defensible disjoint segments or label unattributed; never manufacture a delta. |
| Required route blocked | Retain the route and exact blocker; do not silently work the opposite route. |
| One route fully complete | Finish remaining required work without no-op alternation. |
| Generated artifact already published | Recover its identity, retain UNVERIFIED status where applicable, continue remaining shared milestones. |
| A production defect discovered after the gate | Close the gate, finish repairs, renew it, then resume affected assurance. |
| Host limit or user pause | Preserve state and obey it; do not invent completion, new Goals or automations. |

Quote metadata, source names and marker counts do not establish correctness. A milestone and its usage report are different records: implementation completion may be known while cost attribution remains unknown.
'''

COMPARISON_README = r'''# Milestone cost comparisons

No implementation milestone was executed while authoring this roadmap. This directory therefore contains instructions, not fabricated actual-cost reports.

The sole operational policy is [roadmap/alternation-policy.md](../roadmap/alternation-policy.md). Copy the following structure into `NNNN-MNNNN.md` only for a real completed milestone. Leave unavailable fields unknown; use PROVISIONAL when ending counters have not finalized.

```yaml
record_kind: milestone_cost_comparison
status: PROVISIONAL | FINAL | UNKNOWN_ATTRIBUTION
milestone_id: MNNNN
route: MANUAL | CPG
goal_id: unknown
thread_id: unknown
model_and_settings: unknown
source_snapshot: unknown
forecast_id: F-MNNNN-v1
forecast_baseline_sha256: unknown
predicted_tokens: {low: unknown, base: unknown, high: unknown}
start_observation: {timestamp: unknown, source: unknown, raw: unknown}
end_observation: {timestamp: unknown, source: unknown, raw: unknown}
actual_tokens: unknown
actual_credits: unknown
token_delta_from_base: unknown
absolute_token_difference_from_base: unknown
actual_to_base_ratio: unknown
inside_planning_range: unknown
shared_or_unattributed_cost: unknown
```

## Delivered scope

Record actual authored/verified/delivered artifacts and remaining gaps separately. Link exact source and milestone identities; do not expand a partial milestone to a whole-route claim.

## Cost calculation and explanation

Show the attributable counter segments, calculation, excluded/unattributed work, baseline comparison and concrete reason for deviation. Unknown is not zero. Preserve model changes and cache-accounting semantics without double counting.

## Future forecast revision

Record changed assumptions, affected future milestone IDs, revised ranges and their rationale. Retain the original prediction; mandatory route policy remains unchanged. Do not start a new analysis or verification campaign to fill unavailable measurements.

The historical 9,834,797-token Goal snapshot is a mixed/unattributed observation, not an actual cost for any new roadmap milestone. Import older milestone measurements only when both boundaries and attribution are documented.
'''
