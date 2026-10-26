<a id="m0036"></a>


# M0036 — CPG, schema, lowering and generation assurance


**Route:** `CPG`. **Phase:** `POST_IMPLEMENTATION_VERIFICATION`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0036-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0023](0023-allocation-processing-and-multi-cluster-contracts.md), [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0013](0013-cloud-hooks-through-the-permitted-rusternetes-boundary.md), [M0031](0031-build-deployment-and-configuration-implementation.md), [M0027](0027-production-entrypoints-and-complete-lifecycle-composition.md), [M0029](0029-examples-source-tooling-and-compatibility-inputs.md), [M0021](0021-fleetautoscaler-policies-and-external-decisions.md), [M0015](0015-fleet-reconciliation-and-rollout-policy.md), [M0001](0001-source-mirror-version-and-shared-contracts.md), [M0019](0019-gameserver-and-gameserverset-lifecycle.md), [M0033](0033-complete-the-required-metaprogramming-authoring-family.md), [M0017](0017-telemetry-and-metric-semantics.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0011](0011-port-ownership-and-allocation-policy.md), [M0009](0009-source-compatible-clients-and-existing-rust-sdk-reuse.md), [M0025](0025-sdk-server-streaming-health-and-mutation-behavior.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md), [M0002](0002-pinned-cpg-source-build-machinery-and-rust-support-boundaries.md), [M0022](0022-pinned-frontend-and-self-contained-cpg-input-bundle.md), [M0010](0010-complete-schema-derived-domain-hierarchy.md), [M0032](0032-genco-emission-into-the-exact-rust-source-mirror.md), [M0012](0012-native-cpg-facade-and-graph-ownership.md), [M0026](0026-typed-translation-ir-and-ownership-effect-interfaces.md), [M0030](0030-go-control-errors-concurrency-and-runtime-lowering.md), [M0028](0028-go-value-type-and-initialization-lowering.md), [M0024](0024-normalized-cpg-with-source-identity-preservation.md), [M0020](0020-pass-diff-and-execution-lifecycle.md), [M0016](0016-protobuf-loading-archive-ownership-and-node-mapping.md), [M0034](0034-full-generation-coverage-receipts-and-recoverable-publication.md), [M0006](0006-complete-schema-construction-and-mutually-dependent-declarations.md), [M0008](0008-schema-export-protobuf-tool-inputs-and-launcher-behavior.md), [M0014](0014-serialized-graph-and-overlay-behavior.md), [M0018](0018-format-detection-and-native-storage-compatibility.md), [M0004](0004-native-cpg-utilities-and-effect-ownership.md)


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


1. POST-GATE ONLY. Preserve every original Scala fixture/test identity and assertion. Translate harnesses with original Scala filename stems, including Test/Tests capitalization; the special _harness rule is for Go _test.go files.

2. Establish native/source schema, identity, graph, persistence, loading, failure, pass and generated-domain evidence. Validate lowering with independent invariants, concrete Go/Rust replay and negative controls; keep model proofs separate from runtime observations.

3. Exercise generator determinism, actual source/body coverage, generated compilation, naming, CGP/derive-deftly boundaries and exact tool pins. Unknown/stale/unsupported/empty selections remain unresolved.


## Architecture, readability and authored outputs


1. Own 7 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `cpg:codepropertygraph/src/test/resources` -> module `codepropertygraph::src::test::resources`


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


<a id="file-482b494e6b6d7f61"></a>


### File `codepropertygraph/src/test/resources/cpg.fg`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `bea288575dff4f918bf4aea1af49cf2e9b7fad4d`. **Disposition:** `DEFERRED_HARNESS`. **Destination:** `cpg-rs/codepropertygraph/src/test/resources/cpg.fg`.


**Direct quotation — UPSTREAM_RESOURCE_CONSUMER_EXCERPT; Q-b70a4d2b3bdfc8532f.** [cpg/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoaderTests.scala, lines 74–83](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoaderTests.scala#L74-L83). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `def299dfb28fe6f0e90d74597ca6742f7059c50c5a9e5c34ca27591046dcb92d`.

>     a[FileNotFoundException] should be thrownBy CpgLoader.load("invalid/path/cpg.bin.zip")
>   }
> 
>   /** this test graph was created by c2cpg for https://github.com/joernio/joern/blob/master/tests/code/c/test.c */
>   val flatgraphCpg = Paths.get(ProjectRoot.relativise("codepropertygraph/src/test/resources/cpg.fg"))
> 
>   /** flatgraph writes to this file on 'close', so we'll create a temporary copy */
>   private def temporaryFlatgraphCpg(): TempFileCopy =
>     TempFileCopy(flatgraphCpg)
> }


**Reference boundary.** Binary/empty resource has no prose to quote; quote the upstream loader/test contract and retain exact blob/hash. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Author this harness/helper/fixture only after the global implementation gate. Preserve the source case/helper identity, setup/actions/observations and independent invariants; never substitute production-derived expectations.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-537b32e621e9ef05"></a>


### File `codepropertygraph/src/test/resources/cpg.odb`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f78b226f0f3360dd596468b33b228a5d22479e47`. **Disposition:** `DEFERRED_HARNESS`. **Destination:** `cpg-rs/codepropertygraph/src/test/resources/cpg.odb`.


**Direct quotation — UPSTREAM_RESOURCE_CONSUMER_EXCERPT; Q-aab994a1c047c353f2.** [cpg/.gitignore, lines 3–16](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/.gitignore#L3-L16). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9e091a95ae7edc12c30803b4aef85e1f51f24d230d4f7c98cf3f80f4280a4f52`.

> /codepropertygraph/src/main/proto/io/shiftleft/codepropertygraph/protos.proto
> target/
> codepropertygraph/project/
> /codepropertygraph/src/main/resources/cpg.json
> /codepropertygraph/src/test/resources/cpg.odb.fg
> private-key.pem
> travis_wait_*
> **/*.pyc
> /protoc
> project/.bloop
> .bloop
> 
> .project
> *.class


**Reference boundary.** Binary/empty resource has no prose to quote; quote the upstream loader/test contract and retain exact blob/hash. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Author this harness/helper/fixture only after the global implementation gate. Preserve the source case/helper identity, setup/actions/observations and independent invariants; never substitute production-derived expectations.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph` -> module `codepropertygraph::src::test::scala::io::shiftleft::codepropertygraph`


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


<a id="file-59bc3ceac52d4084"></a>


### File `codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/StepsTest.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `d6bd6bf1dff8eb3e01d757960ea835bae7e511c7`. **Disposition:** `DEFERRED_HARNESS`. **Destination:** `cpg-rs/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/StepsTest.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8b9838e0f25ca3e2f8.** [cpg/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/StepsTest.scala, lines 10–24](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/StepsTest.scala#L10-L24). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0b8fdde378c94bba5b7f42201c0812db485a86dd89e3f2af4b54e5734aa7211b`.

> 
> class StepsTest extends AnyWordSpec with Matchers {
> 
>   ".help step" should {
>     given AvailableWidthProvider = ConstantWidth(120)
>     given DocSearchPackages      = generated.Cpg.defaultDocSearchPackage
> 
>     "show domain overview" in {
>       val domainStartersHelp = Cpg.emptyCpg.help
>       domainStartersHelp should include(".comment")
>       domainStartersHelp should include("A source code comment")
>     }
> 
>     "provide node-specific overview" in {
>       val methodStepsHelp = Cpg.emptyCpg.method.help


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Author this harness/helper/fixture only after the global implementation gate. Preserve the source case/helper identity, setup/actions/observations and independent invariants; never substitute production-derived expectations.

2. Account for 6 lexically identified declarations, including `StepsTest` (L11), `domainStartersHelp` (L18), `methodStepsHelp` (L24), `methodStepsHelpVerbose` (L27), `methodTraversal` (L35), `stringTraversal` (L41). The full declaration list is recorded under `cpg:codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/StepsTest.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading` -> module `codepropertygraph::src::test::scala::io::shiftleft::codepropertygraph::cpgloading`


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


<a id="file-f97dd493cb65ca90"></a>


### File `codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoaderTests.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `35a242461830932f73d55f9372fc1a25b20e5183`. **Disposition:** `DEFERRED_HARNESS`. **Destination:** `cpg-rs/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoaderTests.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f98a911406972d4d08.** [cpg/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoaderTests.scala, lines 16–30](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoaderTests.scala#L16-L30). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `def299dfb28fe6f0e90d74597ca6742f7059c50c5a9e5c34ca27591046dcb92d`.

> 
> /** CpgLoader allows CPGs to be loaded from the CPG protobuf file format (based on Google protocol buffers), overflowdb
>   * and flatgraph binaries.
>   */
> class CpgLoaderTests extends AnyWordSpec with Matchers with BeforeAndAfterAll {
> 
>   var cpgProtoFile: Path = uninitialized
> 
>   override def beforeAll(): Unit = {
>     cpgProtoFile = TestProtoCpg.createTestProtoCpg
>     Files.exists(cpgProtoFile) shouldBe true
>   }
> 
>   override def afterAll(): Unit = {
>     Files.delete(cpgProtoFile)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Author this harness/helper/fixture only after the global implementation gate. Preserve the source case/helper identity, setup/actions/observations and independent invariants; never substitute production-derived expectations.

2. Account for 11 lexically identified declarations, including `CpgLoaderTests` (L20), `beforeAll` (L24), `afterAll` (L29), `odbCpg` (L40), `flatgraphCpg` (L48), `cpg` (L49), `persistTo` (L55), `flatgraphCpg` (L66), `cpg` (L67), `flatgraphCpg` (L78), `temporaryFlatgraphCpg` (L81). The full declaration list is recorded under `cpg:codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoaderTests.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a4c1ed09dfd1fde8"></a>


### File `codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/TestProtoCpg.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9bb1c4faec91a6cb80b2f25a6f49169697ae97c4`. **Disposition:** `DEFERRED_HARNESS`. **Destination:** `cpg-rs/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/TestProtoCpg.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f2c9e15c6d60853517.** [cpg/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/TestProtoCpg.scala, lines 9–23](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/TestProtoCpg.scala#L9-L23). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dc2d78fe0b6462872e2f56507b0b874afabb2e0a3defc42ed2e1786d8143d1c5`.

> 
> object TestProtoCpg {
> 
>   def createTestProtoCpg: Path = {
>     val outDir    = better.files.File.newTemporaryDirectory("cpgloadertests")
>     val outStream = new FileOutputStream((outDir / "1.proto").pathAsString)
>     CpgStruct
>       .newBuilder()
>       .addNode(
>         CpgStruct.Node
>           .newBuilder()
>           .setKey(1)
>           .setType(CpgStruct.Node.NodeType.valueOf("METHOD"))
>           .addProperty(
>             CpgStruct.Node.Property


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Author this harness/helper/fixture only after the global implementation gate. Preserve the source case/helper identity, setup/actions/observations and independent invariants; never substitute production-derived expectations.

2. Account for 5 lexically identified declarations, including `TestProtoCpg` (L10), `createTestProtoCpg` (L12), `outDir` (L13), `outStream` (L14), `zipFile` (L40). The full declaration list is recorded under `cpg:codepropertygraph/src/test/scala/io/shiftleft/codepropertygraph/cpgloading/TestProtoCpg.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

5. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:codepropertygraph/src/test/scala/io/shiftleft/passes` -> module `codepropertygraph::src::test::scala::io::shiftleft::passes`


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


<a id="file-756861700b6c1bca"></a>


### File `codepropertygraph/src/test/scala/io/shiftleft/passes/CpgPassNewTests.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `db353d542ef1eac810dcd26a3c72c98b049111fa`. **Disposition:** `DEFERRED_HARNESS`. **Destination:** `cpg-rs/codepropertygraph/src/test/scala/io/shiftleft/passes/CpgPassNewTests.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b17450784c24095c06.** [cpg/codepropertygraph/src/test/scala/io/shiftleft/passes/CpgPassNewTests.scala, lines 12–26](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/test/scala/io/shiftleft/passes/CpgPassNewTests.scala#L12-L26). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `daa6b826490fd6a9a61560a1aa6e99af25768e07c1b20f2e107034a414948e6e`.

> 
> class CpgPassNewTests extends AnyWordSpec with Matchers {
> 
>   private object Fixture {
>     def apply(f: (Cpg, CpgPassBase) => Unit): Unit = {
>       val cpg = Cpg.empty
>       class MyPass(cpg: Cpg) extends CpgPass(cpg, "MyPass") {
>         override def run(builder: DiffGraphBuilder): Unit = {
>           val builder2 = Cpg.newDiffGraphBuilder
>           builder.addNode(NewFile().name("foo"))
>           builder2.addNode(NewFile().name("bar"))
>           builder.absorb(builder2)
>         }
>       }
>       val pass = new MyPass(cpg)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Author this harness/helper/fixture only after the global implementation gate. Preserve the source case/helper identity, setup/actions/observations and independent invariants; never substitute production-derived expectations.

2. Account for 64 lexically identified declarations, including `CpgPassNewTests` (L13), `Fixture` (L15), `apply` (L16), `cpg` (L17), `MyPass` (L18), `run` (L19), `builder2` (L20), `pass` (L26), `cpg` (L38), `pass` (L39), `run` (L40), `file1` (L41), `file2` (L42), `cpg` (L57), `events` (L58), `pass` (L59), `init` (L60), `generateParts` (L61), `runOnPart` (L62), `finish` (L63), `cpg` (L73), `events` (L74), `pass` (L75), `init` (L76). The full declaration list is recorded under `cpg:codepropertygraph/src/test/scala/io/shiftleft/passes/CpgPassNewTests.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:codepropertygraph/src/test/scala/io/shiftleft/utils` -> module `codepropertygraph::src::test::scala::io::shiftleft::utils`


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


<a id="file-c40b2d637fe6a9ea"></a>


### File `codepropertygraph/src/test/scala/io/shiftleft/utils/IOUtilsTest.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `33e3216e3892a691e669a1cf0a31bfa24dc69e56`. **Disposition:** `DEFERRED_HARNESS`. **Destination:** `cpg-rs/codepropertygraph/src/test/scala/io/shiftleft/utils/IOUtilsTest.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-307695a47b2e162408.** [cpg/codepropertygraph/src/test/scala/io/shiftleft/utils/IOUtilsTest.scala, lines 8–22](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/test/scala/io/shiftleft/utils/IOUtilsTest.scala#L8-L22). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `affec19a0785e5cb097e0657892e0399af8fb689bc71905afaf0f9cf1c6bf62e`.

> 
> class IOUtilsTest extends AnyWordSpec with Matchers {
> 
>   private val Utf8Bom = Array(0xef.toByte, 0xbb.toByte, 0xbf.toByte)
> 
>   private def withTempFile(content: Array[Byte])(f: Path => Unit): Unit = {
>     val path = Files.createTempFile("io-utils-test", ".tmp")
>     try {
>       Files.write(path, content)
>       f(path)
>     } finally {
>       Files.deleteIfExists(path)
>     }
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Author this harness/helper/fixture only after the global implementation gate. Preserve the source case/helper identity, setup/actions/observations and independent invariants; never substitute production-derived expectations.

2. Account for 5 lexically identified declarations, including `IOUtilsTest` (L9), `Utf8Bom` (L11), `withTempFile` (L13), `path` (L14), `utf8` (L23). The full declaration list is recorded under `cpg:codepropertygraph/src/test/scala/io/shiftleft/utils/IOUtilsTest.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
