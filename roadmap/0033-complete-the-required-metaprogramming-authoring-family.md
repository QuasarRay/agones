<a id="m0033"></a>


# M0033 — Complete the required metaprogramming authoring family


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0033-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0023](0023-allocation-processing-and-multi-cluster-contracts.md), [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0013](0013-cloud-hooks-through-the-permitted-rusternetes-boundary.md), [M0031](0031-build-deployment-and-configuration-implementation.md), [M0027](0027-production-entrypoints-and-complete-lifecycle-composition.md), [M0029](0029-examples-source-tooling-and-compatibility-inputs.md), [M0021](0021-fleetautoscaler-policies-and-external-decisions.md), [M0015](0015-fleet-reconciliation-and-rollout-policy.md), [M0001](0001-source-mirror-version-and-shared-contracts.md), [M0019](0019-gameserver-and-gameserverset-lifecycle.md), [M0017](0017-telemetry-and-metric-semantics.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0011](0011-port-ownership-and-allocation-policy.md), [M0009](0009-source-compatible-clients-and-existing-rust-sdk-reuse.md), [M0025](0025-sdk-server-streaming-health-and-mutation-behavior.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Complete proc_macro_2, proc_macro_attribute_2, proc_macro_derive_2 and proc_macro_delegate_2 as one proc_macro2-oriented family with same-logical-crate definition/use and cross-crate export/import. Compiler shims and companion units stay internal.

2. Reuse CGP, derive-deftly, agones-macros, Genco and the pinned Arcane-rs synthez/delegation/arcane facilities before inventing machinery. Record concrete reuse gaps and a short positive remaining-work argument.

3. Keep host/target dependencies acyclic, establish companion manifests before Cargo resolution, preserve source spans/hygiene and ordinary Cargo/IDE entrypoints. Author all required capabilities now; defer compile/IDE/packaging fixtures until the global gate.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.




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

### New support/output `rust-support/agones/metaprogramming/Cargo.toml`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it

**Derived file obligation.** Normal Cargo facade with internal compiler-shim units. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/agones/metaprogramming/src/lib.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it

**Derived file obligation.** All four required _2 forms and same-logical-crate/cross-crate composition. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/agones/metaprogramming/src/templates.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-05cf777d9cd2947e20.** [agones/CONTRIBUTING.md, lines 3–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/CONTRIBUTING.md#L3-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dff06cce1e198764ba9532959c4094bd6c6943e7969cae02994840ce835ea3b`.

> We'd love to accept your patches and contributions to this project. There are
> just a few small guidelines you need to follow.
> 
> ## Contributor License Agreement
> 
> Contributions to this project must be accompanied by a Contributor License
> Agreement. You (or your employer) retain the copyright to your contribution,
> this simply gives us permission to use and redistribute your contributions as
> part of the project. Head over to <https://cla.developers.google.com/> to see
> your current agreements on file or to sign a new one.
> 
> You generally only need to submit a CLA once, so if you've already submitted one
> (even if it was for a different project), you probably don't need to do it

**Derived file obligation.** Shared typed expansion and derive-deftly templates with CGP consumers. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.



## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

