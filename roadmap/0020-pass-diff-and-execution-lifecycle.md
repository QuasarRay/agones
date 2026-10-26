<a id="m0020"></a>


# M0020 — Pass, diff and execution lifecycle


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0020-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0012](0012-native-cpg-facade-and-graph-ownership.md), [M0014](0014-serialized-graph-and-overlay-behavior.md), [M0004](0004-native-cpg-utilities-and-effect-ownership.md)


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


1. Port sequential and fork/join pass behavior, accumulators, callbacks, converters, finish-on-failure/unwind, ordered diff merge and application count.

2. Preserve task ownership, exception propagation, execution-context selection and cleanup order. Do not replace concurrent source semantics with an undocumented stronger sequential model.

3. CGP is the runtime lifecycle owner and derive-deftly the repeatable declaration/wiring owner. Keep source policy visible rather than hiding it in opaque templates.


## Architecture, readability and authored outputs


1. Own 4 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `cpg:codepropertygraph/src/main/scala/io/shiftleft/passes` -> module `codepropertygraph::src::main::scala::io::shiftleft::passes`


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


<a id="file-7d5f8060264a7100"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/passes/Converters.java`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c5b1ec430a92a32ba4994693e4c922a31b92a299`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/passes/Converters.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ac28451a01f2b5c7f6.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/passes/Converters.java, lines 4–14](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/passes/Converters.java#L4-L14). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `acfcfe527534ba22e3e7a477afc5006d892be94e8d98d3d2fd75add7e104300c`.

> 
> public class Converters {
> 
>   /**
>    * Convert a scala sequence into a Java list
>    * */
>   public static <T> List<T> toJava(scala.collection.Seq<T> seq) {
>     return scala.jdk.javaapi.CollectionConverters.asJava(seq);
>   }
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-eacc84dc027d2579"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/passes/CpgPass.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e609872d61a5928e38aa0985bbc480c3b309af81`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/passes/CpgPass.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c63fb9d8b9752117b9.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/passes/CpgPass.scala, lines 12–30](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/passes/CpgPass.scala#L12-L30). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d82244ee209701e9c93bc7c135e6b355f1207d8ba430325d3111f59d28eb9d0a`.

> 
> /** A single-threaded CPG pass. This is the simplest pass to implement: override [[run]] and add desired graph
>   * modifications to the provided [[DiffGraphBuilder]].
>   *
>   * Internally implemented as a [[ForkJoinParallelCpgPass]] with a single part and parallelism disabled.
>   *
>   * @param cpg
>   *   the code property graph to modify
>   * @param outName
>   *   optional name for output
>   */
> abstract class CpgPass(cpg: Cpg, outName: String = "") extends ForkJoinParallelCpgPass[AnyRef](cpg, outName) {
> 
>   /** The main method to implement. Add all desired graph changes (nodes, edges, properties) to the provided builder.
>     *
>     * @param builder
>     *   the [[DiffGraphBuilder]] that accumulates graph modifications
>     */
>   def run(builder: DiffGraphBuilder): Unit


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 49 lexically identified declarations, including `CpgPass` (L23), `run` (L30), `generateParts` (L32), `runOnPart` (L34), `isParallel` (L37), `ForkJoinParallelCpgPass` (L75), `runOnPart` (L85), `createAccumulator` (L87), `runOnPart` (L88), `onAccumulatorComplete` (L89), `mergeAccumulator` (L90), `ForkJoinParallelCpgPassWithAccumulator` (L109), `DiffGraphBuilder` (L113), `generateParts` (L116), `init` (L119), `finish` (L124), `runOnPart` (L135), `isParallel` (L138), `createAccumulator` (L141), `mergeAccumulator` (L144), `onAccumulatorComplete` (L154), `createAndApply` (L160), `nanosStart` (L162), `diffGraph` (L168). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/passes/CpgPass.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f7f0a7b825816d28"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/passes/ParallelIteratorExecutor.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `dce54019ee67a59080119b01dfbdd41e3e85de3d`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/passes/ParallelIteratorExecutor.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ca7ffbd9a638bb08f1.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/passes/ParallelIteratorExecutor.scala, lines 5–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/passes/ParallelIteratorExecutor.scala#L5-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cf98dcbf645c9304a1c75328965371a3e28f9c8a3be38e0e9818b5842cd7c9ba`.

> 
> class ParallelIteratorExecutor[T](iterator: Iterator[T])(implicit executionContext: ExecutionContext) {
>   def map[D](func: T => D): Iterator[D] = {
>     val futures = Future.traverse(iterator) { element =>
>       Future {
>         func(element)
>       }
>     }
>     Await.result(futures, Duration.Inf)
>   }
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 3 lexically identified declarations, including `ParallelIteratorExecutor` (L6), `map` (L7), `futures` (L8). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/passes/ParallelIteratorExecutor.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:samples/pass/src/main/scala/io/shiftleft/passes/mypass` -> module `samples::pass::src::main::scala::io::shiftleft::passes::mypass`


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


<a id="file-2f7d762569e09471"></a>


### File `samples/pass/src/main/scala/io/shiftleft/passes/mypass/SamplePass.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `057dc3f17e4a6001d80a6f8ef365ace1fe943d78`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:samples/pass/src/main/scala/io/shiftleft/passes/mypass/SamplePass.scala`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d2821c87c51be5d890.** [cpg/samples/pass/src/main/scala/io/shiftleft/passes/mypass/SamplePass.scala, lines 8–22](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/samples/pass/src/main/scala/io/shiftleft/passes/mypass/SamplePass.scala#L8-L22). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `665808a4161e8a4149e78b49fb9a365e7a4f67c59841083b3b162c61fc2e4522`.

> 
> class SamplePass(cpg: Cpg) extends SimpleCpgPass(cpg) {
>   override def run(builder: DiffGraphBuilder): Unit = {
>     val fileNode = NewFile().name("foo")
>     builder.addNode(fileNode)
>   }
> }
> 
> object SamplePassMain extends App {
>   val filename      = "src_cpg.bin.zip"
>   val odbConfig     = new overflowdb.Config().withStorageLocation(filename)
>   val loaderConfig  = CpgLoaderConfig().withOverflowConfig(odbConfig)
>   val cpg           = CpgLoader.loadFromOverflowDb(loaderConfig)
>   val serializedCpg = new SerializedCpg("/tmp/dst.bin.zip")
>   val pass          = new SamplePass(cpg)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Account for 10 lexically identified declarations, including `SamplePass` (L9), `run` (L10), `fileNode` (L11), `SamplePassMain` (L16), `filename` (L17), `odbConfig` (L18), `loaderConfig` (L19), `cpg` (L20), `serializedCpg` (L21), `pass` (L22). The full declaration list is recorded under `cpg:samples/pass/src/main/scala/io/shiftleft/passes/mypass/SamplePass.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
