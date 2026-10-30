<a id="m0018"></a>


# M0018 — Format detection and native storage compatibility


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0018-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0016](0016-protobuf-loading-archive-ownership-and-node-mapping.md)


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


1. Port CpgLoader dispatch and deprecated loader paths for source-supported proto, Flatgraph and legacy OverflowDB formats, reusing existing native compatibility code.

2. Retain format probes, overwrite/missing-input order, destination creation, persistent versus memory-only behavior and source failure side effects. Grafeo remains canonical storage; format compatibility is not a second storage owner.

3. Keep native parsing/import transformations explicit and Python-readable; reuse source-backed Rust dependencies without delegating core behavior to Scala.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.

The complete native CPG graph/schema/build objective remains required. Preserve its functional graph, schema and interchange contracts through Rust. Go source facts may be retained for analysis/provenance; generated Agones must use the same native Rust boundary as handwritten Agones.




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


<a id="file-3d36c6010a35b50b"></a>


### File `codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoader.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `53cba2d4a2fd9edc904820d2c7a94ef7b08c70b2`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoader.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ebb753b911a495fbe4.** [cpg/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoader.scala, lines 10–24](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoader.scala#L10-L24). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dc71090886ff899c3319bbbcd64758abd98d6c7e3193a02d558568d6676ec742`.

> 
> object CpgLoader {
>   private val logger: Logger = LoggerFactory.getLogger(getClass)
> 
>   /** Load a Code Property Graph from the given file */
>   def load(filename: String): Cpg =
>     load(Paths.get(filename))
> 
>   /** Load a Code Property Graph from the given file - persist in given second file. I.e. the given input file will not
>     * be modified, all changes will be written to the given 'persistTo' file. *
>     */
>   def load(from: String, persistTo: String): Cpg =
>     load(Paths.get(from), Paths.get(persistTo))
> 
>   /** Load a Code Property Graph from the given file


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete CPG functional behavior through native Rust facilities in the exact destination, preserving CPG graph/schema operations, interchange data, errors and resource outcomes. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 17 lexically identified declarations, including `CpgLoader` (L11), `logger` (L12), `load` (L15), `load` (L21), `load` (L33), `absolutePath` (L34), `load` (L56), `absolutePath` (L57), `isProtoFormat` (L79), `isProtoFormat` (L83), `isOverflowDbFormat` (L86), `isFlatgraphFormat` (L89), `loadFromOverflowDb` (L93), `isLegacyCpg` (L101), `isLegacyCpg` (L106), `probeFirstBytes` (L109), `buffer` (L111). The full declaration list is recorded under `cpg:codepropertygraph/src/main/scala/io/shiftleft/codepropertygraph/cpgloading/CpgLoader.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

