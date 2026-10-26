<a id="m0004"></a>


# M0004 — Native CPG utilities and effect ownership


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0004-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0002](0002-pinned-cpg-source-build-machinery-and-rust-support-boundaries.md)


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


1. Port exact file/stream/BOM/UTF-8 behavior, temporary-copy cleanup, root discovery, interning identity and iterable acquisition semantics.

2. Expose current/global execution-context policy and resource lifetimes through CGP. Use derive-deftly for repeated projections and adapters; preserve error ordering and cleanup on failure.

3. Do not replace source utility semantics with a convenient standard-library approximation without a source-bound mapping. Keep valid Rust concise for Python readers.


## Architecture, readability and authored outputs


1. Own 6 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


<a id="file-8d4d331cafdc537a"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/Implicits.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7cb5ccb6c80bc495172b2e77f55780adf9e519b7`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/Implicits.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-22c5df5208d1300cb0.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/Implicits.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/Implicits.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `533f97a97cfe4b7e9b7a1c962ff9178f35a33ac5d0b09973c25bd74ef3ba6441`.

> 
> object Implicits {
>   private val logger: Logger = LoggerFactory.getLogger(Implicits.getClass)
> 
>   implicit class IterableOnceDeco[T](val iterable: IterableOnce[T]) extends AnyVal {
>     @deprecated(
>       "please use `.loneElement` from flatgraph (mixed into the generated `language` packages) instead, which has a better name and will throw if the iterable has more than one element (rather than just log.warn)",
>       since = "1.7.42 (July 2025)"
>     )
>     def onlyChecked: T = {
>       if (iterable.iterator.hasNext) {
>         val res = iterable.iterator.next()
>         if (iterable.iterator.hasNext) {
>           logger.warn("iterator was expected to have exactly one element, but it actually has more")
>         }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 5 lexically identified declarations, including `Implicits` (L5), `logger` (L6), `IterableOnceDeco` (L8), `onlyChecked` (L13), `res` (L15). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/Implicits.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:codepropertygraph/src/main/scala/io/shiftleft/utils` -> module `codepropertygraph::src::main::scala::io::shiftleft::utils`


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


<a id="file-0f99ed77d841071c"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/utils/ExecutionContextProvider.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `ec91fb3ccdc01b1e7dac82b70b843abcd9d497a3`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/utils/ExecutionContextProvider.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-cd46f5773aa061d094.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/utils/ExecutionContextProvider.scala, lines 10–29](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/utils/ExecutionContextProvider.scala#L10-L29). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0c2252a49b7492c41ab8de32601b39a0b131f5e2a41166831698a8a568b4d4fa`.

>   *
>   * So all multi-threading in codepropertygraph must make sure to either:
>   *   1. Code that uses `new Thread` must set up MDC correctly. For example, consider the Writer background-threads
>   *      spawned in ParallelCpgPass.scala, or consider the background-threads spawned in overflowdb. 2. Code that
>   *      directly (or scala-implicitly) uses threadpools / scala-style executors / java-style ExecutorService should
>   *      call e.g. `implicit val ec:ExecutionContextExecutor =
>   *      io.shiftleft.utils.ExecutionContextProvider.getExecutionContext` instead of `implicit val
>   *      ec:ExecutionContextExecutor = scala.concurrent.ExecutionContext.global` or `import
>   *      scala.concurrent.ExecutionContext.Implicits` 3. Code that uses java parallel stream (e.g.
>   *      ForkJoinParallelCpgPass) needs no changes -- the java stream standard library automatically picks up the right
>   *      threadpool that sets up MDC correctly.
>   */
> object ExecutionContextProvider {
> 
>   def getExecutionContext: ExecutionContextExecutor = {
>     // as of scala 2.13.5, this is always a subclass of ForkJoinPool and hence implements ExecutionContextExecutorService:
>     //  1. If we extract the executionContext from the pool, then we must be ForkJoinPool
>     //  2. The implemention of ExecutionContext.global happens to subclasses from ForkJoinPool, but doesn't expose this finer type in the API
>     // So if you need an ExecutorService for some java stuff, feel free to cast this.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 2 lexically identified declarations, including `ExecutionContextProvider` (L22), `getExecutionContext` (L24). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/utils/ExecutionContextProvider.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-450c7da6dd0b3788"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/utils/IOUtils.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e10f2a3188f7f33ac84d1f4e5b0e91072057c814`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/utils/IOUtils.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8cac626cf0e2b57756.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/utils/IOUtils.scala, lines 8–22](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/utils/IOUtils.scala#L8-L22). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `227764718f09e1ae1e697982cc9957d50d36eb7f120c799c1fe66791f1e69658`.

> 
> object IOUtils {
> 
>   /** Creates a new UTF-8 decoder. Sadly, instances of CharsetDecoder are not thread-safe as the doc states: 'Instances
>     * of this class are not safe for use by multiple concurrent threads.' (copied from:
>     * [[java.nio.charset.CharsetDecoder]])
>     *
>     * As we are using it in a [[io.shiftleft.passes.ForkJoinParallelCpgPass]] it needs to be thread-safe. Hence, we make
>     * sure to create a new instance everytime.
>     */
>   private def createDecoder(): CharsetDecoder =
>     Codec.UTF8.decoder
>       .onMalformedInput(CodingErrorAction.REPLACE)
>       .onUnmappableCharacter(CodingErrorAction.REPLACE)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 12 lexically identified declarations, including `IOUtils` (L9), `createDecoder` (L18), `skipBOMIfPresent` (L26), `contentFromBufferedSource` (L33), `reader` (L34), `lines` (L36), `readLinesInFile` (L54), `readEntireFile` (L66), `bytes` (L67), `offset` (L70), `Utf8BomLength` (L74), `startsWithUtf8Bom` (L76). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/utils/IOUtils.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d7aabcfdd29e8cc5"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/utils/ProjectRoot.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0d2d8f4332d335fcdbe841ce4d18703b0dc8bae5`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/utils/ProjectRoot.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c5c6aaba4a3eb8696d.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/utils/ProjectRoot.scala, lines 6–24](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/utils/ProjectRoot.scala#L6-L24). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c02119b19375b7638e5ccf5fb85044874cb92126163e5a2a5cba4f67397fa11d`.

> 
> /** Finds the relative location of the project root.
>   *
>   * Used in tests which rely on the working directory - unfortunately Intellij and sbt have different default working
>   * directories for executing tests from subprojects: while sbt defaults to the project root, intellij defaults to the
>   * subproject.
>   *
>   * Previously a consistent behaviour was achieved by setting `Test / baseDirectory := (ThisBuild / Test / run /
>   * baseDirectory).value`, however that broke the bsp build within Intellij - it simply wouldn't recognise subprojects
>   * with this setting any more.
>   */
> object ProjectRoot {
>   val FileNameToSearchForEnvVar = "FILE_IN_PROJECT_ROOT"
> 
>   object SearchDepthExceededError extends Error
> 
>   private val SEARCH_DEPTH = 4
>   private lazy val filenameToSearchFor =
>     sys.env


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 9 lexically identified declarations, including `ProjectRoot` (L17), `FileNameToSearchForEnvVar` (L18), `SearchDepthExceededError` (L20), `SEARCH_DEPTH` (L22), `filenameToSearchFor` (L23), `relativise` (L28), `findRelativePath` (L31), `pathPrefix` (L33), `find` (L42). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/utils/ProjectRoot.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-79cd4ddd42ef2aaf"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/utils/StringInterner.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c4c00e4f2fe395fd9af965dfed892778706dccd8`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/utils/StringInterner.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-cbd257a95515c7f1a8.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/utils/StringInterner.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/utils/StringInterner.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c313f2dc460417a74156f81340eba8fe42f0d411b65390a632c851b928fb3118`.

> 
> /** Interface for deduplicating strings used by CPGLoader. JVM with G1 GC does this automatically but it is not
>   * guaranteed to happen and when performing CPU heavy calculations doing this manually ensures that we won't run out of
>   * memory.
>   */
> trait StringInterner {
>   def intern(s: String): String
> }
> 
> object StringInterner {
>   val DefaultMaxStringLength: Int = 1024
>   val DefaultInitialSize: Int     = 64 * 1024
> 
>   val noop: StringInterner = new StringInterner {
>     override def intern(s: String): String = s


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 11 lexically identified declarations, including `StringInterner` (L9), `intern` (L10), `StringInterner` (L13), `DefaultMaxStringLength` (L14), `DefaultInitialSize` (L15), `noop` (L17), `intern` (L18), `makeStrongInterner` (L30), `stringCache` (L34), `intern` (L36), `cached` (L38). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/utils/StringInterner.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-943482066bc5a844"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/utils/TempFileCopy.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `67acb231f8109753cbd109cb184ecd07d2047ea3`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/utils/TempFileCopy.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2a4610e5cce79b0c3f.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/utils/TempFileCopy.scala, lines 6–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/utils/TempFileCopy.scala#L6-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6e3348840b7a2bddf1c9cd734d711b3886fc88f2c8b75495ce70e722e7baabba`.

> 
> class TempFileCopy(sourcePath: Path, prefix: String = "resource", suffix: String = ".tmp") extends AutoCloseable {
>   val path: Path = {
>     val temp = Files.createTempFile(prefix, suffix)
>     Files.copy(sourcePath, temp, StandardCopyOption.REPLACE_EXISTING)
>     temp
>   }
> 
>   override def close() = Files.deleteIfExists(path)
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 4 lexically identified declarations, including `TempFileCopy` (L7), `path` (L8), `temp` (L9), `close` (L14). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/utils/TempFileCopy.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
