<a id="m0016"></a>


# M0016 — Protobuf loading, archive ownership and node mapping


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0016-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0008](0008-schema-export-protobuf-tool-inputs-and-launcher-behavior.md), [M0014](0014-serialized-graph-and-overlay-behavior.md)


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


1. Port ZipArchive, ProtoToGraphNodeMappings and ProtoCpgLoader including two-pass loading, node/type de-duplication, aliases, overlays and import ordering.

2. Retain missing/invalid entries, repeated import, persistent/open/create behavior, failure classifications and source mutations that occur before failure.

3. Compose loaders and resource lifetimes through CGP with derive-deftly-generated mapping scaffolding; do not collapse reference identity into names.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.

The complete native CPG graph/schema/build objective remains required. Preserve its functional graph, schema and interchange contracts through Rust. Go source facts may be retained for analysis/provenance; generated Agones must use the same native Rust boundary as handwritten Agones.




## Architecture, readability and authored outputs


1. Own 3 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `cpg:codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading` -> module `codepropertygraph::src::main::scala::io::shiftleft::codepropertygraph::cpgloading`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve CPG functional and interchange contracts and exact module/file spelling; implement initialization and visibility with native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic and file excerpts establish CPG functional obligations. Rust libraries provide native implementation; no Go runtime is required.


<a id="file-7803e5a4b29da8d2"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoCpgLoader.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8116ef347b07e44d64ff4e0c2dd5890f65f4283b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoCpgLoader.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0d57fd0190f5cad7fd.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoCpgLoader.scala, lines 17–31](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoCpgLoader.scala#L17-L31). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9ef6c33a088d477ff0fdc000684497531403f98a0210effaa8cde51a6724c30b`.

> 
> object ProtoCpgLoader {
>   private val logger: Logger = LoggerFactory.getLogger(getClass)
> 
>   def loadFromProtoZip(fileName: String, storagePath: Option[Path]): Cpg = {
>     measureAndReport {
>       Using.Manager { use =>
>         def cpgProtos: Iterator[CpgStruct] = {
>           use(new ZipArchive(fileName)).entries.iterator.map { entry =>
>             val inputStream = use(Files.newInputStream(entry))
>             CpgStruct.parseFrom(inputStream)
>           }
>         }
>         loadFromListOfProtos(() => cpgProtos, storagePath)
>       }.get


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete CPG functional behavior through native Rust facilities in the exact destination, preserving CPG graph/schema operations, interchange data, errors and resource outcomes. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 31 lexically identified declarations, including `ProtoCpgLoader` (L18), `logger` (L19), `loadFromProtoZip` (L21), `cpgProtos` (L24), `inputStream` (L26), `loadFromListOfProtos` (L39), `interner` (L41), `protoToGraphNodeMappings` (L42), `cpg` (L43), `diffGraph` (L51), `protoNodeId` (L54), `gNode` (L55), `loadFromListOfProtos` (L79), `openOrCreateCpg` (L82), `nodesIter` (L89), `addNodesRaw` (L92), `diffGraph` (L97), `label` (L99), `newNode` (L104), `extractPropertyValue` (L112), `extractEdgePropertyValue` (L124), `protoProperties` (L125), `loadOverlays` (L136), `loadOverlays` (L139). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoCpgLoader.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-09888ab7ae5d188c"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoToGraphNodeMappings.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `91f4c54573fea20c515f4971e112777e74d8756d`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoToGraphNodeMappings.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e0d2a319d65202b515.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoToGraphNodeMappings.scala, lines 8–25](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoToGraphNodeMappings.scala#L8-L25). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5f9ba534522d10e33193c0b4ce1807345901721a8876d1ce0e299509bd422657`.

> 
> /** Mutable data structure to preserve mapping between proto and cpg nodes during ProtoToCpg import.
>   *
>   * Context: we need to run two passes: 1) add nodes and 2) set node properties and add edges (this is due to
>   * flatgraph-specific implementation details)
>   *
>   * Because of that, we need to remember the mapping from proto node id to gnode. Typically, that's just a plain
>   * mapping. But there's one special case for TYPE nodes: some (parallel) frontends create duplicate TYPE nodes which we
>   * need to deduplicate...
>   */
> class ProtoToGraphNodeMappings {
>   private val protoNodeIdToGNode  = mutable.LongMap.empty[DNode]
>   private val typeFullNameToGNode = mutable.Map.empty[String, DNode]
> 
>   def add(protoNode: CpgStruct.Node, node: DNode): Unit = {
>     protoNodeIdToGNode += protoNode.getKey -> node
>     typeFullNameMaybe(protoNode).foreach(typeFullName => typeFullNameToGNode += typeFullName -> node)
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete CPG functional behavior through native Rust facilities in the exact destination, preserving CPG graph/schema operations, interchange data, errors and resource outcomes. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 9 lexically identified declarations, including `ProtoToGraphNodeMappings` (L18), `protoNodeIdToGNode` (L19), `typeFullNameToGNode` (L20), `add` (L22), `contains` (L27), `find` (L30), `findGNode` (L39), `findGNode` (L48), `typeFullNameMaybe` (L51). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ProtoToGraphNodeMappings.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0ca7b884edd518a0"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ZipArchive.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `27cf7e2d9a59e9cf7753123070bcb40e2f88028c`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ZipArchive.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b20625810884c547c3.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ZipArchive.scala, lines 10–24](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ZipArchive.scala#L10-L24). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b919fb9113d24fe51a08764ebeb738f984a29db56ba362e5b5df3d3abcdf482c`.

> 
> class ZipArchive(inputFile: String) extends Closeable {
>   private val zipFileSystem: FileSystem = FileSystems.newFileSystem(Paths.get(inputFile), null: ClassLoader)
> 
>   private def root: Path = zipFileSystem.getRootDirectories.iterator.next
> 
>   private def walk(rootPath: Path): Seq[Path] = {
>     val entries = ArraySeq.newBuilder[Path]
>     Files.walkFileTree(
>       rootPath,
>       new SimpleFileVisitor[Path]() {
>         override def visitFile(file: Path, attrs: BasicFileAttributes): FileVisitResult = {
>           if (attrs.isRegularFile)
>             entries += file
>           FileVisitResult.CONTINUE


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete CPG functional behavior through native Rust facilities in the exact destination, preserving CPG graph/schema operations, interchange data, errors and resource outcomes. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 9 lexically identified declarations, including `ZipArchive` (L11), `zipFileSystem` (L12), `root` (L14), `walk` (L16), `entries` (L17), `visitFile` (L21), `entries` (L31), `getFileEntries` (L33), `close` (L35). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/ZipArchive.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Keep native Rust resources, cancellation and failure cleanup under CGP.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

