<a id="m0012"></a>


# M0012 — Native CPG facade and graph ownership


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0012-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0010](0010-complete-schema-derived-domain-hierarchy.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-9816c9c65ca73478ff.** [cpg/README.md, lines 87–98](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L87-L98). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Loading a codepropertygraph
> 
> Here's how you can load a cpg into ShiftLeft Tinkergraph [3] in the sbt console - the next section will list some queries you can interactively run from there.
> 
> There are some sample cpgs in this repository in the `resources/testcode/cpgs` directory.
> You can run `./regenerate-test-cpgs.sh` to update them, but this requires the proprietary java2cpg installed locally.
> 
> ### [Tinkergraph (in memory reference db)](http://tinkerpop.apache.org/docs/current/reference/#tinkergraph-gremlin)
> ```
> sbt semanticcpg/console
> ```
> ```scala


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Port Cpg and package-level APIs with exact graph construction, borrowed access, mutation, context/close and persistence behavior.

2. Keep Grafeo as the existing canonical graph/persistence owner, with disposable projections rather than a parallel writable graph. Preserve source IDs, tombstones, property values and edge/adjacency order.

3. CGP owns graph/effect/lifecycle policy; derive-deftly generates repeated typed adapters and ownership metadata. All operations must reach real native implementation.


## Architecture, readability and authored outputs


1. Own 2 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `cpg:codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph` -> module `codepropertygraph::src::main::scala::io::shiftleft::codepropertygraph`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-9816c9c65ca73478ff.** [cpg/README.md, lines 87–98](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L87-L98). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Loading a codepropertygraph
> 
> Here's how you can load a cpg into ShiftLeft Tinkergraph [3] in the sbt console - the next section will list some queries you can interactively run from there.
> 
> There are some sample cpgs in this repository in the `resources/testcode/cpgs` directory.
> You can run `./regenerate-test-cpgs.sh` to update them, but this requires the proprietary java2cpg installed locally.
> 
> ### [Tinkergraph (in memory reference db)](http://tinkerpop.apache.org/docs/current/reference/#tinkergraph-gremlin)
> ```
> sbt semanticcpg/console
> ```
> ```scala


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-488e0c6532100669"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/Cpg.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `729689a3ce07649993cec04839370af836e48caa`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/Cpg.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a62bf1fb37a69fb4fc.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/Cpg.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/Cpg.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f97456c01af22ffa61ce2fafdda324ec164d98316752f2913978f4720f10530e`.

> 
> /** TODO this is now being generated as well - for now we'll just forward calls to `generated.Cpg` next step is to
>   * remove this class and move remove the `generated` part from the generated package
>   */
> object Cpg {
>   val defaultDocSearchPackage: DocSearchPackages = generated.Cpg.defaultDocSearchPackage
> 
>   /** Syntactic sugar for `new Cpg(graph)`. Usage: `Cpg(graph)` or simply `Cpg` if you have an `implicit Graph` in scope
>     */
>   def apply(implicit graph: Graph) = generated.Cpg(graph)
> 
>   /** Create an empty code property graph
>     */
>   def emptyCpg: generated.Cpg =
>     generated.Cpg(emptyGraph)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 6 lexically identified declarations, including `Cpg` (L10), `defaultDocSearchPackage` (L11), `apply` (L15), `emptyCpg` (L19), `withStorage` (L29), `emptyGraph` (L32). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/Cpg.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ba44733ea3b6505a"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/package.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `92a816279133c7fc49bcd7d100a226e7a433fda8`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/package.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b83eccec996673abc3.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/package.scala, lines 4–5](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/package.scala#L4-L5). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f13a8b0acf24a165b5bba1da23f2a03078ee00fddb2734639d891b471ee49988`.

>   type Cpg = io.shiftleft.codepropertygraph.generated.Cpg
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `Cpg` (L4). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/package.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
