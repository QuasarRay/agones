<a id="m0037"></a>


# M0037 — Agones acceptance and delivery preparation


**Route:** `MANUAL`. **Phase:** `POST_IMPLEMENTATION_VERIFICATION`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0037-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0035](0035-agones-harnesses-and-final-behavioral-assurance.md), [M0036](0036-cpg-schema-lowering-and-generation-assurance.md)


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


1. POST-GATE ONLY. Finish joint architecture, Pythonic readability, source parity, source-path fidelity, all _2 cells and required assurance evidence for the exact delivered source snapshot.

2. Prepare exact-head delivery manifests, review changes and authorized PR material for agones-rs-dev. Final integration/publication occurs in the shared closing milestone after both routes are ready; do not mark a merge complete in advance.

3. Confirm every required scope row has current evidence and all source/external obligations are resolved. Completing this route does not by itself complete the shared Goal.


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


These examples explain reading order and borrowing. Actual APIs must come from the selected pinned dependencies. The owning CGP provider supplies the collections; derive-deftly supplies repeated declarations/projections. Preserve source ordering and avoid redundant clones or a comprehension around complicated effects.


## Package/module and file instructions


This is a required cross-cutting implementation milestone. Its concrete outputs are the stage interfaces/rules/transaction or delivery artifacts specified above. Place new Rust-only machinery under `rust-support/agones` or `rust-support/cpg`, keeping source mirrors unchanged. Bind output ownership in the existing ledger; a design document or empty provider does not complete it.


## Explicit new support/output ownership

These are planned Rust-only support or evidence files, not original-source mirror renames. Reuse an existing equivalent through a recorded path mapping; do not create a competing implementation merely to match this suggested support filename.

### New support/output `verification/agones/delivery/agones-acceptance.json`

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

**Derived file obligation.** Post-gate source-qualified evidence and delivery preparation record. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.



## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
