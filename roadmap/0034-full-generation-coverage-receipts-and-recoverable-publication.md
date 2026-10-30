<a id="m0034"></a>


# M0034 — Full generation, coverage receipts and recoverable publication


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0034-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0023](0023-allocation-processing-and-multi-cluster-contracts.md), [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0013](0013-cloud-hooks-through-the-permitted-rusternetes-boundary.md), [M0031](0031-build-deployment-and-configuration-implementation.md), [M0027](0027-production-entrypoints-and-complete-lifecycle-composition.md), [M0029](0029-examples-source-tooling-and-compatibility-inputs.md), [M0021](0021-fleetautoscaler-policies-and-external-decisions.md), [M0015](0015-fleet-reconciliation-and-rollout-policy.md), [M0001](0001-source-mirror-version-and-shared-contracts.md), [M0019](0019-gameserver-and-gameserverset-lifecycle.md), [M0033](0033-complete-the-required-metaprogramming-authoring-family.md), [M0017](0017-telemetry-and-metric-semantics.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0011](0011-port-ownership-and-allocation-policy.md), [M0009](0009-source-compatible-clients-and-existing-rust-sdk-reuse.md), [M0025](0025-sdk-server-streaming-health-and-mutation-behavior.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md), [M0032](0032-genco-emission-into-the-exact-rust-source-mirror.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-71f6b7430be0dfa8be.** [genco/README.md, lines 10–22](https://github.com/udoprog/genco/blob/98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb/README.md#L10-L22). Revision `98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb`; source SHA-256 `0e04298e93fe8231a498b29d8752708f44cef62545e7cda4860b2cc80a96bbf2`.

> Central to genco are the [`quote!`] and [`quote_in!`] procedural macros which
> ease the construction of [token streams].
> 
> This project solves the following language-specific concerns:
> 
> * **Imports** — Generates and groups [import statements] as they are used.
>   So you only import what you use, with no redundancy. We also do our best
>   to [solve namespace conflicts].
> 
> * **String Quoting** — genco knows how to [quote strings]. And can even
>   [interpolate] values *into* the quoted string if it's supported by the
>   language.


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Implement and, only with a permitted already runnable emitter, perform the complete generation transaction into generated/agones-first-generation. Reconcile every required file/body/output with the independent source inventory.

2. Publish manifests, source-to-IR-to-output mappings, rule/schema/tool/input digests and FIRST_FULL_GENERATION_COMPLETE atomically. Interrupted staging is not a completed artifact; recover intact publication without duplicate work.

3. Complete required production adoption interfaces, ownership transfers and call paths before the global gate, preserving the separate source-derived artifact. The artifact remains UNVERIFIED. A missing executable/build is a blocker and prevents this milestone and the global implementation gate from completing. First generation is a milestone of the combined objective, not a command to stop the Goal.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.

The complete native CPG graph/schema/build objective remains required. Preserve its functional graph, schema and interchange contracts through Rust. Go source facts may be retained for analysis/provenance; generated Agones must use the same native Rust boundary as handwritten Agones.




## Architecture, readability and authored outputs


1. Own 0 inventoried file entries in this milestone plus the concrete new translation/support interfaces described above.

2. Record typed declarations -> derive-deftly templates -> CGP provider implementations/delegation -> production context -> lifecycle consumer. Identify each mutable state/resource owner and effect boundary.

3. Apply Pythonic readability and idiomatic Rust as equal design criteria. Comprehensions simplify mapping/filtering; explicit small functions clarify effects. Do not add a Python interpreter, dynamic object layer, reflection registry or competing manager.

4. Preserve source-relative directories, exact case/stems and singular/plural forms. Map Go `_test.go` only to `_harness.rs`; keep Scala Test/Tests stems unchanged. Register unconventional names outside the mirror.


**Python-familiar valid Rust example (schematic; not executed):**


```rust
use comprende::c;
use better_comprehension::vector;

let ready_names = c![server.name().to_owned()
                    for server in &servers if server.is_ready()];
let selected_ids = vector![node.id() for node in nodes.iter()
                          if node.has_required_label()];
```


These examples explain reading order and borrowing. Actual APIs must come from the selected pinned dependencies. The owning CGP provider supplies the collections; derive-deftly supplies repeated declarations/projections. Preserve Agones-required ordering and avoid redundant clones or a comprehension around complicated effects.


## Package/module and file instructions


This is a required cross-cutting implementation milestone. Its concrete outputs are the stage interfaces/rules/transaction or delivery artifacts specified above. Place new Rust-only machinery under `rust-support/agones` or `rust-support/cpg`, keeping source mirrors unchanged. Bind output ownership in the existing ledger; a design document or empty provider does not complete it.


## Explicit new support/output ownership

These are planned Rust-only support or evidence files, not original-source mirror renames. Reuse an existing equivalent through a recorded path mapping; do not create a competing implementation merely to match this suggested support filename.

### New support/output `rust-support/cpg/src/publication.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-71f6b7430be0dfa8be.** [genco/README.md, lines 10–22](https://github.com/udoprog/genco/blob/98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb/README.md#L10-L22). Revision `98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb`; source SHA-256 `0e04298e93fe8231a498b29d8752708f44cef62545e7cda4860b2cc80a96bbf2`.

> Central to genco are the [`quote!`] and [`quote_in!`] procedural macros which
> ease the construction of [token streams].
> 
> This project solves the following language-specific concerns:
> 
> * **Imports** — Generates and groups [import statements] as they are used.
>   So you only import what you use, with no redundancy. We also do our best
>   to [solve namespace conflicts].
> 
> * **String Quoting** — genco knows how to [quote strings]. And can even
>   [interpolate] values *into* the quoted string if it's supported by the
>   language.

**Derived file obligation.** Staging, non-circular digests, flush and atomic publication. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/cpg/src/recovery.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-71f6b7430be0dfa8be.** [genco/README.md, lines 10–22](https://github.com/udoprog/genco/blob/98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb/README.md#L10-L22). Revision `98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb`; source SHA-256 `0e04298e93fe8231a498b29d8752708f44cef62545e7cda4860b2cc80a96bbf2`.

> Central to genco are the [`quote!`] and [`quote_in!`] procedural macros which
> ease the construction of [token streams].
> 
> This project solves the following language-specific concerns:
> 
> * **Imports** — Generates and groups [import statements] as they are used.
>   So you only import what you use, with no redundancy. We also do our best
>   to [solve namespace conflicts].
> 
> * **String Quoting** — genco knows how to [quote strings]. And can even
>   [interpolate] values *into* the quoted string if it's supported by the
>   language.

**Derived file obligation.** Content-bound interrupted-operation recovery. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/cpg/src/adoption.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-71f6b7430be0dfa8be.** [genco/README.md, lines 10–22](https://github.com/udoprog/genco/blob/98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb/README.md#L10-L22). Revision `98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb`; source SHA-256 `0e04298e93fe8231a498b29d8752708f44cef62545e7cda4860b2cc80a96bbf2`.

> Central to genco are the [`quote!`] and [`quote_in!`] procedural macros which
> ease the construction of [token streams].
> 
> This project solves the following language-specific concerns:
> 
> * **Imports** — Generates and groups [import statements] as they are used.
>   So you only import what you use, with no redundancy. We also do our best
>   to [solve namespace conflicts].
> 
> * **String Quoting** — genco knows how to [quote strings]. And can even
>   [interpolate] values *into* the quoted string if it's supported by the
>   language.

**Derived file obligation.** Explicit production ownership transfer and preservation of manual meaning. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `generated/agones-first-generation/evidence/completion.json`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-71f6b7430be0dfa8be.** [genco/README.md, lines 10–22](https://github.com/udoprog/genco/blob/98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb/README.md#L10-L22). Revision `98d9226f57f6d3e721ab77cfc21fff28d9b5c9cb`; source SHA-256 `0e04298e93fe8231a498b29d8752708f44cef62545e7cda4860b2cc80a96bbf2`.

> Central to genco are the [`quote!`] and [`quote_in!`] procedural macros which
> ease the construction of [token streams].
> 
> This project solves the following language-specific concerns:
> 
> * **Imports** — Generates and groups [import statements] as they are used.
>   So you only import what you use, with no redundancy. We also do our best
>   to [solve namespace conflicts].
> 
> * **String Quoting** — genco knows how to [quote strings]. And can even
>   [interpolate] values *into* the quoted string if it's supported by the
>   language.

**Derived file obligation.** Transactional complete source-derived artifact record with UNVERIFIED assurances. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.



## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

## Specific source obligations for this new stage

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2b20aea020f9aee181.** [cpg/README.md, lines 50–67](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L50-L67). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Domain class generation
> 
> The Scala domain classes under `domainClasses/src/main/generated/` are generated from the schema defined in `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/`. They are checked into the repository, so you only need to regenerate them when the schema changes.
> 
> Generation is handled by the `sbt-flatgraph` plugin via the `generateDomainClasses` task in the `schema` subproject. It reads `CpgSchema.instance`, which composes all schema modules, and writes Scala source files to `domainClasses/src/main/generated/`.
> 
> The task is wired as a dependency of `domainClasses/compile` and `stage`, so it runs automatically when you compile or stage:
> 
> ```
> sbt compile   # triggers generateDomainClasses, then compiles
> sbt stage     # triggers generateDomainClasses, then packages
> ```
> 
> To run generation on its own:
> 
> ```
> sbt schema/generateDomainClasses
> ```

**Derived stage requirement.** Generated material has declared source inputs and a canonical generator owner. Extend that provenance principle to the separate complete Agones artifact. Atomic staging/publication and the user-selected implementation gate are engineering requirements of this roadmap, not claims made by this upstream excerpt. No dedicated upstream Go-to-Rust translator specification is claimed; this stage is engineered from the quoted source obligations and the user-approved shared contract.
