<a id="m0026"></a>


# M0026 — Typed translation IR and ownership/effect interfaces


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0026-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0024](0024-normalized-cpg-with-source-identity-preservation.md)


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


1. Implement typed IR for declarations, types, values, control flow, calls, effects, initialization and source observations. Every node binds source/body/configuration and rule identities.

2. Map the already implemented Agones public contracts and Rusternetes capabilities into explicit destination interfaces. Keep graph facts, inferred facts and translation policy distinguishable.

3. Specify errors and unsupported constructs as unresolved work, not emitted fake success. Genco consumes typed IR; CGP/derive-deftly own stages and repeated structure.


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

### New support/output `rust-support/cpg/src/translation_ir.rs`

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

**Derived file obligation.** Typed values, ownership, effects, control flow and observation contracts. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/cpg/src/interfaces.rs`

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

**Derived file obligation.** Public Agones/Rusternetes types and explicit external-call bindings. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.



## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

## Specific source obligations for this new stage

**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-2edc1423cfacf07fa7.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.scala, lines 14–27](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.scala#L14-L27). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dd3fe0ad05f427e94fedcaff66234a64014f868e1fd203fafd9519fe4886fb93`.

>     """
>       |The Abstract Syntax Tree (AST) Layer provides syntax trees for all compilation units.
>       |All nodes of the tree inherit from the same base class (`AST_NODE`) and are connected
>       |to their child nodes via outgoing `AST` edges.
>       |
>       |Syntax trees are typed, that is, when possible, types for all expressions are stored
>       |in the tree. Moreover, common control structure types are defined in the specification,
>       |making it possible to translate trees into corresponding control flow graphs if only
>       |these common control structure types are used, possibly by desugaring on the side of
>       |the language frontend. For cases where this is not an option,
>       |the AST specification provides means of storing language-dependent information in the
>       |AST that can be interpreted by language-dependent control flow construction passes.
>       |
>       |This layer MUST be created by the frontend.

**Derived stage requirement.** Represent expressions, source types and common control structures in typed IR with provenance. Any desugaring must retain source effects, order and observable outcomes; Genco cannot infer the missing semantics. No dedicated upstream Go-to-Rust translator specification is claimed; this stage is engineered from the quoted source obligations and the user-approved shared contract.
