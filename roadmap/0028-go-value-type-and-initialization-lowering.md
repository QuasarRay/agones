<a id="m0028"></a>


# M0028 — Go value, type and initialization lowering


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0028-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0026](0026-typed-translation-ir-and-ownership-effect-interfaces.md)


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


1. Implement source-bound rules for integer widths/overflow, constants/conversions, floating bits, strings/bytes/runes, nil/empty containers, interfaces/type assertions, pointers/aliasing, structs/tags, slices/maps and generics.

2. Preserve package/global initialization, evaluation order, short-circuiting, multiple values and external-call contracts. Carry required runtime support through CGP capabilities.

3. Generate readable valid Rust, not an embedded Go interpreter. Use derive-deftly and comprehensions where their evaluation and ownership semantics match the source.


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

### New support/output `rust-support/cpg/src/lower_values.rs`

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

**Derived file obligation.** Value/type/numeric/container/aliasing rules. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/cpg/src/lower_initialization.rs`

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

**Derived file obligation.** Declarations, package initialization and ordered evaluation. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.



## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

## Specific source obligations for this new stage

**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-2e7977975fcb052b68.** [agones/pkg/apis/agones/v1/gameserver.go, lines 350–380](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/apis/agones/v1/gameserver.go#L350-L380). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7df5b0d1da0985786664c31a0f27eb01ffcabeabdabcdf519193c5b7184fbaa0`.

> 	IDs      []string `json:"ids"`
> }
> 
> // CounterStatus stores the current counter values and maximum capacity
> type CounterStatus struct {
> 	Count    int64 `json:"count"`
> 	Capacity int64 `json:"capacity"`
> }
> 
> // ListStatus stores the current list values and maximum capacity
> type ListStatus struct {
> 	Capacity int64    `json:"capacity"`
> 	Values   []string `json:"values"`
> }
> 
> // ApplyDefaults applies default values to the GameServer if they are not already populated
> func (gs *GameServer) ApplyDefaults() {
> 	// VersionAnnotation is the annotation that stores
> 	// the version of sdk which runs in a sidecar
> 	if gs.ObjectMeta.Annotations == nil {
> 		gs.ObjectMeta.Annotations = map[string]string{}
> 	}
> 	gs.ObjectMeta.Annotations[VersionAnnotation] = pkg.Version
> 	gs.ObjectMeta.Finalizers = append(gs.ObjectMeta.Finalizers, FinalizerName)
> 
> 	gs.Spec.ApplyDefaults()
> 	gs.applyStatusDefaults()
> }
> 
> // ApplyDefaults applies default values to the GameServerSpec if they are not already populated
> func (gss *GameServerSpec) ApplyDefaults() {

**Derived stage requirement.** Translate these real defaulting operations, method calls, field writes and absence/default decisions through typed lowering. Preserve original widths, nil/empty distinctions and observable sequencing; do not substitute hand-copied output for CPG provenance. No dedicated upstream Go-to-Rust translator specification is claimed; this stage is engineered from the quoted source obligations and the user-approved shared contract.
