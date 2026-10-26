<a id="m0014"></a>


# M0014 — Serialized graph and overlay behavior


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0014-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0012](0012-native-cpg-facade-and-graph-ownership.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-6f91f11036b3f09126.** [cpg/README.md, lines 26–37](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L26-L37). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Usage as a dependency
> `build.sbt`:
> ```
> libraryDependencies += "io.shiftleft" %% "codepropertygraph" % "x.y.z"
> ```
> 
> # Building the code
> 
> The build process has been verified on Linux and it should be possible
> to build on OS X and BSD systems as well. The build process requires
> the following prerequisites:


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Port SerializedCpg archive/overlay serialization, entry naming, order, counters, overwrite behavior, mounting and close/publication semantics.

2. Preserve unknown protobuf fields, source scalar representations, partial failure side effects and cleanup ownership. Do not treat a successful archive write as semantic equivalence.

3. Keep serialization data transformations readable, with CGP I/O capabilities and derive-deftly-generated record/projection structure.


## Architecture, readability and authored outputs


1. Own 1 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `cpg:codepropertygraph/src/main/scala/io/shiftleft` -> module `codepropertygraph::src::main::scala::io::shiftleft`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-6f91f11036b3f09126.** [cpg/README.md, lines 26–37](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L26-L37). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Usage as a dependency
> `build.sbt`:
> ```
> libraryDependencies += "io.shiftleft" %% "codepropertygraph" % "x.y.z"
> ```
> 
> # Building the code
> 
> The build process has been verified on Linux and it should be possible
> to build on OS X and BSD systems as well. The build process requires
> the following prerequisites:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-cf83cc8dcd820ec2"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/SerializedCpg.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `11d72d9603ab23660dbaa775e2f5521b34ada588`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/SerializedCpg.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d9a2647499dff5b41b.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/SerializedCpg.scala, lines 11–25](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/SerializedCpg.scala#L11-L25). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `ecdf78f57053a357711d0efe56f0f7be60448de84a4bbcb421eddfd3d0393cb1`.

> class SerializedCpg extends AutoCloseable {
> 
>   /** We allow creating a dummy serialized CPG that does not do anything.
>     */
>   private var zipFileSystem: FileSystem = null
>   private var counter                   = 0
> 
>   /** Create Serialized CPG from existing file. If the file does not exist, an empty Serialized CPG is created.
>     */
>   def this(filename: String) = {
>     this()
>     initZipFilesystem(filename)
>   }
> 
>   def isEmpty: Boolean = zipFileSystem == null


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 12 lexically identified declarations, including `SerializedCpg` (L11), `this` (L20), `isEmpty` (L25), `initZipFilesystem` (L29), `env` (L30), `fileUri` (L34), `outputUri` (L35), `addOverlay` (L42), `pathInZip` (L44), `outputStream` (L46), `addOverlay` (L53), `close` (L60). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/SerializedCpg.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
