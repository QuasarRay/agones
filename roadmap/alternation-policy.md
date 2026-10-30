<a id="alternation-policy"></a>

# Mandatory route alternation and milestone cost accounting

## 1. Authority and source context

This file is the sole authoritative scheduling and cost-accounting policy for the combined roadmap. The milestone files contain work and dependency declarations. The cost report contains forecasts and conclusions. Neither independently changes this policy. The combined edition reproduces this file as a distinct component.

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-6d72c7c9ff86207d9b.** [cgp/README.md, lines 9–15](https://github.com/contextgeneric/cgp/blob/eca0c5fda60d87cf048460f5abdd3b6e82da5e0f/README.md#L9-L15). Revision `eca0c5fda60d87cf048460f5abdd3b6e82da5e0f`; source SHA-256 `8f0eedcfac8dfdfca7d5da152b488fc48150310dc2cac074c456943854f5c8b3`.

> **A language extension for Rust, with pluggable trait implementations at compile-time.**
> 
> Context-Generic Programming (CGP) is a library on stable Rust that lifts the language's one-implementation-per-type limit off your traits: it lets one interface have many interchangeable implementations and lets each *context* — an application, a test, a deployment — plug in the one it uses. The choice is written in one readable place and resolved entirely during compilation, so it compiles down to direct calls: there is no runtime container, no reflection, and nothing left in the binary for an implementation a context does not use.
> 
> > [!IMPORTANT]
> > This repository describes **v0.8.0**, which is **not compatible** with v0.7.0 — v0.8.0 changes and removes syntax that v0.7.0 used. **All documentation in this repository describes v0.8.0 only.** For v0.7.0, refer to its published crate documentation instead.


**Derived distinction.** CGP's static context/provider selection is application architecture. Switching the agent's implementation workstream is an execution-policy decision authorized by the user; it is not a runtime plugin or provider registry.

**Direct quotation — USER_SUPPLIED_CURRENT_PROJECT_GUIDANCE; Q-9ea3891cc21daae253.** [Current AGENTS.MD, line 23](/home/astral/Projects/agones/AGENTS.MD:23). Source: uncommitted working tree on 30 October 2026; SHA-256 `d11c9403e7e0d16150c0f54f7bbdfe01bba04c9a7a1618eae4985d86981b3311`.

> 10. Existing Rust Ecosystem must be used to the fullest potential. Do NOT try to recreate Go semantics in Agones Rust reimplementation. Application behavior and semantics must only correspond to original Agones exclusively in Agones-specific semantics, all traces of go semantics must be fully removed and replaced with Rust Semantics in Rust reimplementation of Agones.


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

## Contract amendments and source scope

Apply AGONES_NATIVE_RUST_V1 from the shared contract. Completed milestone history and actual measurements remain intact. GO_ONLY_SUPERSEDED obligations do not block completion; unresolved Agones obligations do. This corrects scope without creating a route-switch event or replacing the native Goal.
