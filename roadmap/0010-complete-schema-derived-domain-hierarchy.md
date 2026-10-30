<a id="m0010"></a>


# M0010 — Complete schema-derived domain hierarchy


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0010-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0006](0006-complete-schema-construction-and-mutually-dependent-declarations.md), [M0004](0004-native-cpg-utilities-and-effect-ownership.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c29272595b076bd119.** [cpg/README.md, lines 50–61](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L50-L61). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Domain class generation
> 
> The Scala domain classes under `domainClasses/src/main/generated/` are generated from the schema defined in `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/`. They are checked into the repository, so you only need to regenerate them when the schema changes.
> 
> Generation is handled by the `sbt-flatgraph` plugin via the `generateDomainClasses` task in the `schema` subproject. It reads `CpgSchema.instance`, which composes all schema modules, and writes Scala source files to `domainClasses/src/main/generated/`.
> 
> The task is wired as a dependency of `domainClasses/compile` and `stage`, so it runs automatically when you compile or stage:
> 
> ```
> sbt compile   # triggers generateDomainClasses, then compiles
> sbt stage     # triggers generateDomainClasses, then packages
> ```


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Regenerate every source-generated node, edge, starter, traversal, property accessor and domain class from the complete schema; never hand-port the generated Scala corpus.

2. Preserve every original generated source-relative directory and filename stem in Rust. Register case-sensitive/dotted paths explicitly in external module glue.

3. Preserve stored/detached/reference types, inheritance, defaults, cardinalities, contained-node relationships and neighbor ordering. Reuse native Grafeo ownership and projections; derive-deftly and Genco share canonical inputs with one output owner.


## Native Rust semantic boundary

1. Apply `AGONES_NATIVE_RUST_V1` and requirement 10 of the current root AGENTS.MD. Existing Rust ecosystem facilities are the default for data, errors, formatting, logging, I/O, serialization, tasks and synchronization. Custom implementation requires an identified Agones policy or concrete integration gap.
2. Record `AGONES_REQUIRED`, `RUST_NATIVE` and `GO_ONLY_SUPERSEDED` per obligation, including within mixed files. Preserve source IDs and evidence. A Go implementation mechanism does not create a runtime compatibility obligation.
3. Preserve Agones API/domain outcomes through native Rust. Do not implement Go formatting/reflection, typed-nil interfaces, panic/recover/Goexit, goroutine/channel/select behavior, map/slice internals, GC, allocator or ABI emulation. Use Rust ownership, Result/Option, RAII, existing libraries and CGP-owned tasks instead.
4. Quotes containing Go code remain source references. Source-body mappings may point to a real native Rust library operation plus the necessary Agones policy. A mapping must identify the production consumer; a label or empty wrapper is insufficient.
5. Keep CGP AND derive-deftly as joint architectural/lifecycle owners and Python-familiar valid Rust as a coequal readability requirement. The global implementation-first gate and separate alternation policy remain binding.

The complete native CPG graph/schema/build objective remains required. Preserve its functional graph, schema and interchange contracts through Rust. Go source facts may be retained for analysis/provenance; generated Agones must use the same native Rust boundary as handwritten Agones.




## Architecture, readability and authored outputs


1. Own 251 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `cpg:domainClasses` -> module `domainClasses`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c29272595b076bd119.** [cpg/README.md, lines 50–61](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L50-L61). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Domain class generation
> 
> The Scala domain classes under `domainClasses/src/main/generated/` are generated from the schema defined in `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/`. They are checked into the repository, so you only need to regenerate them when the schema changes.
> 
> Generation is handled by the `sbt-flatgraph` plugin via the `generateDomainClasses` task in the `schema` subproject. It reads `CpgSchema.instance`, which composes all schema modules, and writes Scala source files to `domainClasses/src/main/generated/`.
> 
> The task is wired as a dependency of `domainClasses/compile` and `stage`, so it runs automatically when you compile or stage:
> 
> ```
> sbt compile   # triggers generateDomainClasses, then compiles
> sbt stage     # triggers generateDomainClasses, then packages
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve CPG functional and interchange contracts and exact module/file spelling; implement initialization and visibility with native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic and file excerpts establish CPG functional obligations. Rust libraries provide native implementation; no Go runtime is required.


<a id="file-3f8f62b866b7d5f8"></a>


### File `domainClasses/build.sbt`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `57b8b1875809d09e662f288de6297641415e69b2`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/build.sbt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ecb730479ade529a23.** [cpg/domainClasses/build.sbt, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/build.sbt#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `189f0ec2f2b65f496feb62c128cd7792a5f78c034d793f865681082c3365c1e2`.

> name := "codepropertygraph-domain-classes"
> 
> libraryDependencies ++= Seq(
>   "io.joern" %% "flatgraph-core" % Versions.flatgraph,
>   "io.joern" %% "flatgraph-help" % Versions.flatgraph,
> )
> 
> lazy val generatedSrcDir = settingKey[File]("root for generated sources - we want to check those in")
> generatedSrcDir := (Compile/sourceDirectory).value / "generated"
> Compile/unmanagedSourceDirectories += generatedSrcDir.value
> Compile/compile := (Compile/compile).dependsOn(Projects.schema/Compile/generateDomainClasses).value
> 
> /* generated sources occasionally have some warnings..
>  * we're trying to minimise them on a best effort basis, but don't want
>  * to fail the build because of them */


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete CPG functional behavior through native Rust facilities in the exact destination, preserving CPG graph/schema operations, interchange data, errors and resource outcomes. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated` -> module `domainClasses::src::main::generated::io::shiftleft::codepropertygraph::generated`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c29272595b076bd119.** [cpg/README.md, lines 50–61](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L50-L61). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Domain class generation
> 
> The Scala domain classes under `domainClasses/src/main/generated/` are generated from the schema defined in `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/`. They are checked into the repository, so you only need to regenerate them when the schema changes.
> 
> Generation is handled by the `sbt-flatgraph` plugin via the `generateDomainClasses` task in the `schema` subproject. It reads `CpgSchema.instance`, which composes all schema modules, and writes Scala source files to `domainClasses/src/main/generated/`.
> 
> The task is wired as a dependency of `domainClasses/compile` and `stage`, so it runs automatically when you compile or stage:
> 
> ```
> sbt compile   # triggers generateDomainClasses, then compiles
> sbt stage     # triggers generateDomainClasses, then packages
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve CPG functional and interchange contracts and exact module/file spelling; implement initialization and visibility with native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic and file excerpts establish CPG functional obligations. Rust libraries provide native implementation; no Go runtime is required.


<a id="file-401f99b917716ac6"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/ControlStructureTypes.java`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5de64e163c6f8b905188f9dfc3406f25679c7134`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/ControlStructureTypes.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-afc75a2e3651e0387f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/ControlStructureTypes.java, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/ControlStructureTypes.java#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `aa0f0e708948a4b2eae363aedd9c80e8df78819aecc1fe3b00bd31df6280134f`.

> 
> public class ControlStructureTypes {
> 
> /** Represents a break statement. Labeled breaks are expected to have a JUMP_LABEL
> node AST child with ORDER 1 */
> public static final String BREAK = "BREAK";
> 
> /** Represents a continue statement. Labeled continues are expected to have a JUMP_LABEL
> node AST child with ORDER 1 */
> public static final String CONTINUE = "CONTINUE";
> 
> /** Represents a while statement */
> public static final String WHILE = "WHILE";
> 
> /** Represents a do statement */


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3a4efc68d0797dd0"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Cpg.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `219b65ff6bd4d50af3c22fedbc90e2a30bcb2958`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Cpg.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f84e3b0afe428cb90c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Cpg.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Cpg.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `79e1564da878d07fea3e839993454c168eb31ce6786ca2250d7c2052f44005c0`.

> 
> object Cpg {
>   val defaultDocSearchPackage = DocSearchPackages.default.withAdditionalPackage(getClass.getPackage.getName)
> 
>   @scala.annotation.implicitNotFound("""If you're using flatgraph purely without a schema and associated generated domain classes, you can
> start with `given DocSearchPackages = DocSearchPackages.default`.
> If you have generated domain classes, use `given DocSearchPackages = MyDomain.defaultDocSearchPackage`.
> If you have additional custom extension steps that specify help texts via @Doc annotations, use `given DocSearchPackages = MyDomain.defaultDocSearchPackage.withAdditionalPackage("my.custom.package)"`
> """)
>   def help(implicit searchPackageNames: DocSearchPackages, availableWidthProvider: AvailableWidthProvider) =
>     flatgraph.help.TraversalHelp(searchPackageNames).forTraversalSources(verbose = false)
> 
>   @scala.annotation.implicitNotFound("""If you're using flatgraph purely without a schema and associated generated domain classes, you can
> start with `given DocSearchPackages = DocSearchPackages.default`.
> If you have generated domain classes, use `given DocSearchPackages = MyDomain.defaultDocSearchPackage`.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 86 lexically identified declarations, including `Cpg` (L7), `defaultDocSearchPackage` (L8), `help` (L15), `helpVerbose` (L23), `empty` (L26), `from` (L28), `graph` (L29), `withStorage` (L38), `graph` (L39), `newDiffGraphBuilder` (L43), `Cpg` (L46), `graph` (L47), `help` (L49), `helpVerbose` (L51), `close` (L54), `toString` (L57), `CpgNodeStarters` (L62), `all` (L65), `id` (L67), `ids` (L70), `annotation` (L80), `annotationLiteral` (L84), `annotationParameter` (L89), `annotationParameterAssign` (L94). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Cpg.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Keep native Rust resources, cancellation and failure cleanup under CGP.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a39e42630f1a5829"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/DispatchTypes.java`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `151e9ee209b65b39da8834cddd43586344f6d612`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/DispatchTypes.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0b49def6c6ff96922b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/DispatchTypes.java, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/DispatchTypes.java#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `de169dcc29a5f0b700365b5dac740666e31a857bc22f912796dd6c236ecb23c0`.

> 
> public class DispatchTypes {
> 
> /** For statically dispatched calls the call target is known before program execution */
> public static final String STATIC_DISPATCH = "STATIC_DISPATCH";
> 
> /** For dynamically dispatched calls the target is determined during runtime */
> public static final String DYNAMIC_DISPATCH = "DYNAMIC_DISPATCH";
> 
> /** For macro expansions, code is inlined. */
> public static final String INLINED = "INLINED";
> 
> public static Set<String> ALL = new HashSet<String>() {{
> add(STATIC_DISPATCH);
> add(DYNAMIC_DISPATCH);


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-92d7fac6df0b1180"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/EdgeTypes.java`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `728053e7462ad88ead118883a1fe93b490628737`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/EdgeTypes.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-edb7452845844ad5ff.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/EdgeTypes.java, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/EdgeTypes.java#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `15b8ab58f3e996cdc175460c1ec48ddcbfd5fdc1ad435c162f722964a5d77b08`.

> 
> public class EdgeTypes {
> 
> /** This edge represents an alias relation between a type declaration and a type.
> The language frontend MUST NOT create `ALIAS_OF` edges as they are created
> automatically based on `ALIAS_TYPE_FULL_NAME` fields when the CPG is first loaded. */
> public static final String ALIAS_OF = "ALIAS_OF";
> 
> /** Argument edges connect call sites (node type `CALL`) to their arguments
> (node type `EXPRESSION`) as well as `RETURN` nodes to the expressions
> that return. */
> public static final String ARGUMENT = "ARGUMENT";
> 
> /** This edge connects a parent node to its child in the syntax tree. */
> public static final String AST = "AST";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7b017a55270ce173"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/EvaluationStrategies.java`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `34f39d1c9727b8fcaa6a8aa2aca1f0d1c1feefd8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/EvaluationStrategies.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-335f5259f25baf316d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/EvaluationStrategies.java, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/EvaluationStrategies.java#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0b87371ebf43ed9787b66738861fddf179217775e43f594f34acae9ba63b39f6`.

> 
> public class EvaluationStrategies {
> 
> /** A parameter or return of a function is passed by reference which means an address is used behind the scenes */
> public static final String BY_REFERENCE = "BY_REFERENCE";
> 
> /** Only applicable to object parameter or return values. The pointer to the object is passed by value but the object itself is not copied and changes to it are thus propagated out of the method context */
> public static final String BY_SHARING = "BY_SHARING";
> 
> /** A parameter or return of a function passed by value which means a flat copy is used */
> public static final String BY_VALUE = "BY_VALUE";
> 
> public static Set<String> ALL = new HashSet<String>() {{
> add(BY_REFERENCE);
> add(BY_SHARING);


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-33e816a3ade8776f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/GraphSchema.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `d213f35c896cb1e8139ebd2a7724d59b1de1eade`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/GraphSchema.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f35a01f32689a85f7c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/GraphSchema.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/GraphSchema.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `4c293f0bdfbb2b1f657f57347996d1f291028b80db69c4ec1d87ca0eeb5d501f`.

> 
> object GraphSchema extends flatgraph.Schema {
>   private val nodeLabels = IndexedSeq(
>     "ANNOTATION",
>     "ANNOTATION_LITERAL",
>     "ANNOTATION_PARAMETER",
>     "ANNOTATION_PARAMETER_ASSIGN",
>     "ARRAY_INITIALIZER",
>     "BINDING",
>     "BLOCK",
>     "CALL",
>     "CLOSURE_BINDING",
>     "COMMENT",
>     "CONFIG_FILE",
>     "CONTROL_STRUCTURE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 32 lexically identified declarations, including `GraphSchema` (L5), `nodeLabels` (L6), `nodeKindByLabel` (L51), `edgeLabels` (L52), `edgeKindByLabel` (L88), `edgePropertyAllocators` (L89), `nodeFactories` (L125), `edgeFactories` (L170), `nodePropertyAllocators` (L206), `normalNodePropertyNames` (L265), `nodePropertyByLabel` (L320), `nodePropertyDescriptors` (L325), `nodePropertyDescriptors` (L326), `newNodeInsertionHelpers` (L1085), `_newNodeInserters` (L1086), `getNumberOfNodeKinds` (L1482), `getNumberOfEdgeKinds` (L1483), `getNodeLabel` (L1484), `getNodeKindByLabel` (L1485), `getEdgeLabel` (L1486), `getEdgeKindByLabel` (L1487), `getNodePropertyNames` (L1488), `getEdgePropertyName` (L1861), `getPropertyLabel` (L1868). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/GraphSchema.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4bd594138ec3cd89"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Languages.java`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `08e5de83d0d60166e2fba2b17ca9463386f47a43`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Languages.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d414ac7478c20e34e3.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Languages.java, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Languages.java#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a9cf7cc62fbc0abe05166a95d4fe105607b09500ae47c78d587175b7306fc7f9`.

> 
> public class Languages {
> 
> 
> public static final String JAVA = "JAVA";
> 
> 
> public static final String JAVASCRIPT = "JAVASCRIPT";
> 
> 
> public static final String GOLANG = "GOLANG";
> 
> 
> public static final String CSHARP = "CSHARP";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-032710d54a34dddf"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/ModifierTypes.java`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e0d963442bfd3bc8352e9bd01d7c504b4032b531`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/ModifierTypes.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5c699315d107292a8b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/ModifierTypes.java, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/ModifierTypes.java#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `63523ac4e2753c95b91fc4fd2be9ca401a3e432c2df6bf7af85f503d4a4b15a8`.

> 
> public class ModifierTypes {
> 
> /** The static modifier */
> public static final String STATIC = "STATIC";
> 
> /** The public modifier */
> public static final String PUBLIC = "PUBLIC";
> 
> /** The protected modifier */
> public static final String PROTECTED = "PROTECTED";
> 
> /** The private modifier */
> public static final String PRIVATE = "PRIVATE";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-69fa7fbd3d7c8181"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/NodeTypes.java`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e8c828939b91cb0addeee17b2ab83f823e1bf5d4`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/NodeTypes.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-46a357485dfba2b0a9.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/NodeTypes.java, lines 25–39](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/NodeTypes.java#L25-L39). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `8a5b77b40dbe9a7169e238dcebef4d7d1085fdd5b36d3a7a2da02eaefc17fb50`.

> 
> /** `BINDING` nodes represent name-signature pairs that can be resolved at a
> type declaration (`TYPE_DECL`). They are connected to `TYPE_DECL` nodes via
> incoming `BINDS` edges. The bound method is either associated with an outgoing
> `REF` edge to a `METHOD` or with the `METHOD_FULL_NAME` property. The `REF` edge
> if present has priority. */
> public static final String BINDING = "BINDING";
> 
> /** This node represents a compound statement. Compound statements are used in many languages to allow
> grouping a sequence of statements. For example, in C and Java, compound statements
> are statements enclosed by curly braces. Function/Method bodies are compound
> statements. We do not use the term "compound statement" because "statement" would
> imply that the block does not yield a value upon evaluation, that is, that it is
> not an expression. This is true in languages such as C and Java, but not for languages
> such as Scala where the value of the block is given by that of the last expression it


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 2 lexically identified declarations, including `declaration` (L27), `declaration` (L151). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/NodeTypes.java` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e582e4d5332c3f3c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Operators.java`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `05d62192cb355dc98c67ad8736c715615efaf182`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Operators.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e0314cd040c82fc436.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Operators.java, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Operators.java#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5610f7a11c167dee75031a02439cb9103571a424ea59902f26a8910bf4dea76c`.

> 
> public class Operators {
> 
> 
> public static final String addition = "<operator>.addition";
> 
> 
> public static final String subtraction = "<operator>.subtraction";
> 
> 
> public static final String multiplication = "<operator>.multiplication";
> 
> 
> public static final String division = "<operator>.division";


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Keep native Rust resources, cancellation and failure cleanup under CGP.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-03b804f24a3dd516"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Properties.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `6a4fe5134f8e8f65156d4dc9de548ba2a6eeee3a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Properties.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0483c59f4d513b9258.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Properties.scala, lines 2–16](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Properties.scala#L2-L16). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d920348e88a39aac87e97de092aa57f8e52d7437c1139cf85646f656ec4f3388`.

> 
> object Properties {
> 
>   /** This property holds the fully qualified name of the type that the node is a type alias of.
>     */
>   val AliasTypeFullName = flatgraph.OptionalPropertyKey[String](kind = 0, name = "ALIAS_TYPE_FULL_NAME")
> 
>   /** AST-children of CALL nodes have an argument index, that is used to match call-site arguments with callee
>     * parameters. Explicit parameters are numbered from 1 to N, while index 0 is reserved for implicit self / this
>     * parameter. CALLs without implicit parameter therefore have arguments starting with index 1. AST-children of BLOCK
>     * nodes may have an argument index as well; in this case, the last argument index determines the return expression
>     * of a BLOCK expression. If the `PARAMETER_NAME` field is set, then the `ARGUMENT_INDEX` field is ignored. It is
>     * suggested to set it to -1.
>     */
>   val ArgumentIndex = flatgraph.SinglePropertyKey[Int](kind = 1, name = "ARGUMENT_INDEX", default = -1: Int)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 54 lexically identified declarations, including `Properties` (L3), `AliasTypeFullName` (L7), `ArgumentIndex` (L16), `ArgumentLabel` (L25), `ArgumentName` (L31), `AstParentFullName` (L34), `AstParentType` (L40), `CanonicalName` (L48), `ClosureBindingId` (L53), `Code` (L56), `ColumnNumber` (L60), `ColumnNumberEnd` (L64), `ContainedRef` (L69), `Content` (L74), `ControlStructureType` (L80), `DependencyGroupId` (L84), `DispatchType` (L91), `DynamicTypeHintFullName` (L94), `EvaluationStrategy` (L102), `EvidenceDescription` (L108), `ExplicitAs` (L114), `Filename` (L121), `FullName` (L126), `GenericSignature` (L132). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/Properties.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-005334dd969707ea"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyDefaults.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5a274683ecb28985d126178fcdd43f4ad4c707ed`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyDefaults.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9113fb55f2e9b5263f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyDefaults.scala, lines 2–16](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyDefaults.scala#L2-L16). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6ecab6c27dc91b164d75035da9ec4293e8fc710dfe19681da8a13509dbd18b2d`.

> 
> object PropertyDefaults {
>   val ArgumentIndex        = -1: Int
>   val AstParentFullName    = "<empty>"
>   val AstParentType        = "<empty>"
>   val CanonicalName        = "<empty>"
>   val Code                 = "<empty>"
>   val ContainedRef         = "<empty>"
>   val Content              = "<empty>"
>   val ControlStructureType = "<empty>"
>   val DispatchType         = "<empty>"
>   val EvaluationStrategy   = "<empty>"
>   val Filename             = "<empty>"
>   val FullName             = "<empty>"
>   val GenericSignature     = "<empty>"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 31 lexically identified declarations, including `PropertyDefaults` (L3), `ArgumentIndex` (L4), `AstParentFullName` (L5), `AstParentType` (L6), `CanonicalName` (L7), `Code` (L8), `ContainedRef` (L9), `Content` (L10), `ControlStructureType` (L11), `DispatchType` (L12), `EvaluationStrategy` (L13), `Filename` (L14), `FullName` (L15), `GenericSignature` (L16), `Index` (L17), `IsExternal` (L18), `IsVariadic` (L19), `Key` (L20), `Language` (L21), `MethodFullName` (L22), `ModifierType` (L23), `Name` (L24), `Order` (L25), `ParserTypeName` (L26). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyDefaults.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bd71b4a852ef9b27"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyErrorRegister.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `899880bb9b4a2a7863c1f3310b6a958cbe51845d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyErrorRegister.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8c6db28e2e59fbccb0.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyErrorRegister.scala, lines 2–13](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyErrorRegister.scala#L2-L13). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc6918b1fa5bedd81ada832cd126ce36da49041ffc80758be5927a2b9eb56401`.

> 
> object PropertyErrorRegister {
>   private var errorMap = Set.empty[(Class[?], String)]
>   private val logger   = org.slf4j.LoggerFactory.getLogger(getClass)
> 
>   def logPropertyErrorIfFirst(clazz: Class[?], propertyName: String): Unit = {
>     if (!errorMap.contains((clazz, propertyName))) {
>       logger.warn("Property " + propertyName + " is deprecated for " + clazz.getName + ".")
>       errorMap += ((clazz, propertyName))
>     }
>   }
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 3 lexically identified declarations, including `PropertyErrorRegister` (L3), `logger` (L5), `logPropertyErrorIfFirst` (L7). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyErrorRegister.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5360be26b918ad74"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyNames.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b7a45738fd5203a2acf187b1f29cc938c6b0666a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyNames.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8229e4d76f7596afdb.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyNames.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyNames.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9e178496fb2e22b0bb47caa4c45114c1a0aa74e8717e021ce827e12b4f3663c5`.

> 
> object PropertyNames {
> 
>   /** This property holds the fully qualified name of the type that the node is a type alias of.
>     */
>   val AliasTypeFullName: String = "ALIAS_TYPE_FULL_NAME"
> 
>   /** AST-children of CALL nodes have an argument index, that is used to match call-site arguments with callee
>     * parameters. Explicit parameters are numbered from 1 to N, while index 0 is reserved for implicit self / this
>     * parameter. CALLs without implicit parameter therefore have arguments starting with index 1. AST-children of BLOCK
>     * nodes may have an argument index as well; in this case, the last argument index determines the return expression
>     * of a BLOCK expression. If the `PARAMETER_NAME` field is set, then the `ARGUMENT_INDEX` field is ignored. It is
>     * suggested to set it to -1.
>     */
>   val ArgumentIndex: String = "ARGUMENT_INDEX"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 61 lexically identified declarations, including `PropertyNames` (L6), `AliasTypeFullName` (L10), `ArgumentIndex` (L19), `ArgumentLabel` (L28), `ArgumentName` (L34), `AstParentFullName` (L37), `AstParentType` (L42), `CanonicalName` (L50), `ClosureBindingId` (L55), `Code` (L58), `ColumnNumber` (L62), `ColumnNumberEnd` (L66), `ContainedRef` (L71), `Content` (L76), `ControlStructureType` (L82), `DependencyGroupId` (L85), `DispatchType` (L92), `DynamicTypeHintFullName` (L95), `EvaluationStrategy` (L103), `EvidenceDescription` (L108), `ExplicitAs` (L114), `Filename` (L121), `FullName` (L126), `GenericSignature` (L132). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/PropertyNames.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-14aa9e6b70cf80b3"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/language.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `ff660e0079e9da18f2dcc30996628fd252fe7377`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/language.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-55f0cf08a56a0d707c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/language.scala, lines 2–14](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/language.scala#L2-L14). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dd7268f55448e22081925c9c8a29326b2b47c91d7f71b28fc722c55fc7298abb`.

> 
> /** combining all implicits into one trait that can be mixed in further downstream */
> trait language
>     extends accessors.ConcreteStoredConversions
>     with traversals.ConcreteStoredConversions
>     with neighboraccessors.Conversions
>     with flatgraph.traversal.language
>     with flatgraph.help.language
>     with flatgraph.Implicits {
>   implicit def toGeneratedNodeStarters(domain: Cpg): CpgNodeStarters = CpgNodeStarters(domain)
> }
> 
> object language extends language


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 3 lexically identified declarations, including `language` (L4), `toGeneratedNodeStarters` (L11), `language` (L14). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/language.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d20c5249919e7370"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/package.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `aca3eb80998f11ac7c6c7f063016e8f66919fe89`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/package.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-3e9e9ff1ad216d49be.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/package.scala, lines 4–12](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/package.scala#L4-L12). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7416b4d4c34bee0f6db8b3ef078c7abcef94f0f70838f502de283ba6dbb5bbc7`.

>   // some type aliases so that the domain-specific code can avoid referencing the `flatgraph` namespace
>   type DiffGraphBuilder = _root_.flatgraph.DiffGraphBuilder
> 
>   object help {
>     type Doc             = _root_.flatgraph.help.Doc
>     type Traversal       = _root_.flatgraph.help.Traversal
>     type TraversalSource = _root_.flatgraph.help.TraversalSource
>   }
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 5 lexically identified declarations, including `DiffGraphBuilder` (L5), `help` (L7), `Doc` (L8), `Traversal` (L9), `TraversalSource` (L10). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/package.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/accessors` -> module `domainClasses::src::main::generated::io::shiftleft::codepropertygraph::generated::accessors`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c29272595b076bd119.** [cpg/README.md, lines 50–61](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L50-L61). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Domain class generation
> 
> The Scala domain classes under `domainClasses/src/main/generated/` are generated from the schema defined in `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/`. They are checked into the repository, so you only need to regenerate them when the schema changes.
> 
> Generation is handled by the `sbt-flatgraph` plugin via the `generateDomainClasses` task in the `schema` subproject. It reads `CpgSchema.instance`, which composes all schema modules, and writes Scala source files to `domainClasses/src/main/generated/`.
> 
> The task is wired as a dependency of `domainClasses/compile` and `stage`, so it runs automatically when you compile or stage:
> 
> ```
> sbt compile   # triggers generateDomainClasses, then compiles
> sbt stage     # triggers generateDomainClasses, then packages
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve CPG functional and interchange contracts and exact module/file spelling; implement initialization and visibility with native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic and file excerpts establish CPG functional obligations. Rust libraries provide native implementation; no Go runtime is required.


<a id="file-f7062cf8a66e6fac"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/accessors/Accessors.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f666c5cd0fc86da6d65b31715eb2e6b22e3afc87`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/accessors/Accessors.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-15d434ab27b1e75a18.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/accessors/Accessors.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/accessors/Accessors.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dd28ced5574b5ee82a059db0bfff6d4abf6eea01da549184b526d087a712fcc1`.

> 
> /** not supposed to be used directly by users, hence the `bootstrap` in the name */
> object languagebootstrap extends ConcreteStoredConversions
> 
> object Accessors {
>   /* accessors for concrete stored nodes start */
>   final class AccessPropertyAliasTypeFullName(val node: nodes.StoredNode) extends AnyVal {
>     def aliasTypeFullName: Option[String] = flatgraph.Accessors
>       .getNodePropertyOption[String](node.graph, nodeKind = node.nodeKind, propertyKind = 0, seq = node.seq)
>   }
>   final class AccessPropertyArgumentIndex(val node: nodes.StoredNode) extends AnyVal {
>     def argumentIndex: Int = flatgraph.Accessors.getNodePropertySingle(
>       node.graph,
>       nodeKind = node.nodeKind,
>       propertyKind = 1,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 397 lexically identified declarations, including `languagebootstrap` (L6), `Accessors` (L8), `AccessPropertyAliasTypeFullName` (L10), `aliasTypeFullName` (L11), `AccessPropertyArgumentIndex` (L14), `argumentIndex` (L15), `AccessPropertyArgumentLabel` (L23), `argumentLabel` (L24), `AccessPropertyArgumentName` (L27), `argumentName` (L28), `AccessPropertyAstParentFullName` (L31), `astParentFullName` (L32), `AccessPropertyAstParentType` (L40), `astParentType` (L41), `AccessPropertyCanonicalName` (L49), `canonicalName` (L50), `AccessPropertyClosureBindingId` (L58), `closureBindingId` (L59), `AccessPropertyCode` (L62), `code` (L63), `AccessPropertyColumnNumber` (L71), `columnNumber` (L72), `AccessPropertyColumnNumberEnd` (L75), `columnNumberEnd` (L76). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/accessors/Accessors.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/edges` -> module `domainClasses::src::main::generated::io::shiftleft::codepropertygraph::generated::edges`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c29272595b076bd119.** [cpg/README.md, lines 50–61](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L50-L61). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Domain class generation
> 
> The Scala domain classes under `domainClasses/src/main/generated/` are generated from the schema defined in `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/`. They are checked into the repository, so you only need to regenerate them when the schema changes.
> 
> Generation is handled by the `sbt-flatgraph` plugin via the `generateDomainClasses` task in the `schema` subproject. It reads `CpgSchema.instance`, which composes all schema modules, and writes Scala source files to `domainClasses/src/main/generated/`.
> 
> The task is wired as a dependency of `domainClasses/compile` and `stage`, so it runs automatically when you compile or stage:
> 
> ```
> sbt compile   # triggers generateDomainClasses, then compiles
> sbt stage     # triggers generateDomainClasses, then packages
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve CPG functional and interchange contracts and exact module/file spelling; implement initialization and visibility with native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic and file excerpts establish CPG functional obligations. Rust libraries provide native implementation; no Go runtime is required.


<a id="file-19c3b170eb36619d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/edges/EdgeTypes.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9fd7ea9e8d12c8f81f2215296087e2e0f8529e83`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/edges/EdgeTypes.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-477247ddfd4430679d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/edges/EdgeTypes.scala, lines 2–16](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/edges/EdgeTypes.scala#L2-L16). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `86a02ebb7c275103a1d22fd4e0f8984b070c9f7bb106eda14d451ee001b03a0c`.

> 
> object AliasOf {
>   val Label = "ALIAS_OF"
> 
> }
> 
> class AliasOf(src_4762: flatgraph.GNode, dst_4762: flatgraph.GNode, subSeq_4862: Int, property_4862: Any)
>     extends flatgraph.Edge(src_4762, dst_4762, 0, subSeq_4862, property_4862) {}
> 
> object Argument {
>   val Label = "ARGUMENT"
> 
> }
> 
> class Argument(src_4762: flatgraph.GNode, dst_4762: flatgraph.GNode, subSeq_4862: Int, property_4862: Any)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 104 lexically identified declarations, including `AliasOf` (L3), `Label` (L4), `AliasOf` (L8), `Argument` (L11), `Label` (L12), `Argument` (L16), `Ast` (L19), `Label` (L20), `Ast` (L24), `Binds` (L27), `Label` (L28), `Binds` (L32), `BindsTo` (L35), `Label` (L36), `BindsTo` (L40), `Call` (L43), `Label` (L44), `Call` (L48), `Capture` (L51), `Label` (L52), `Capture` (L56), `CapturedBy` (L59), `Label` (L60), `CapturedBy` (L64). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/edges/EdgeTypes.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors` -> module `domainClasses::src::main::generated::io::shiftleft::codepropertygraph::generated::neighboraccessors`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c29272595b076bd119.** [cpg/README.md, lines 50–61](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L50-L61). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Domain class generation
> 
> The Scala domain classes under `domainClasses/src/main/generated/` are generated from the schema defined in `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/`. They are checked into the repository, so you only need to regenerate them when the schema changes.
> 
> Generation is handled by the `sbt-flatgraph` plugin via the `generateDomainClasses` task in the `schema` subproject. It reads `CpgSchema.instance`, which composes all schema modules, and writes Scala source files to `domainClasses/src/main/generated/`.
> 
> The task is wired as a dependency of `domainClasses/compile` and `stage`, so it runs automatically when you compile or stage:
> 
> ```
> sbt compile   # triggers generateDomainClasses, then compiles
> sbt stage     # triggers generateDomainClasses, then packages
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve CPG functional and interchange contracts and exact module/file spelling; implement initialization and visibility with native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic and file excerpts establish CPG functional obligations. Rust libraries provide native implementation; no Go runtime is required.


<a id="file-00dd1e8f55dcc24a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Annotation.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1c807758311a4ae49fc9e0296637b6d9962cc82f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Annotation.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e3ac48e4bebf8ae748.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Annotation.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Annotation.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0d02a64cb67365567792e3d5f2393a3ba304d642c69aba6441cbfe78a24925cc`.

> 
> final class AccessNeighborsForAnnotation(val node: nodes.Annotation) extends AnyVal {
> 
>   /** Traverse to ANNOTATION_PARAMETER_ASSIGN via AST IN edge.
>     */
>   def _annotationParameterAssignViaAstIn: Iterator[nodes.AnnotationParameterAssign] =
>     astIn.collectAll[nodes.AnnotationParameterAssign]
> 
>   /** Traverse to ANNOTATION_PARAMETER_ASSIGN via AST OUT edge.
>     */
>   def _annotationParameterAssignViaAstOut: Iterator[nodes.AnnotationParameterAssign] =
>     astOut.collectAll[nodes.AnnotationParameterAssign]
> 
>   /** Traverse to IDENTIFIER via AST IN edge.
>     */


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 26 lexically identified declarations, including `AccessNeighborsForAnnotation` (L6), `_annotationParameterAssignViaAstIn` (L10), `_annotationParameterAssignViaAstOut` (L15), `_identifierViaAstIn` (L20), `_literalViaAstIn` (L24), `_memberViaAstIn` (L28), `_methodViaAstIn` (L32), `_methodParameterInViaAstIn` (L36), `_methodRefViaAstIn` (L40), `_typeDeclViaAstIn` (L44), `_unknownViaAstIn` (L48), `astIn` (L50), `astOut` (L52), `AccessNeighborsForAnnotationTraversal` (L55), `_annotationParameterAssignViaAstIn` (L59), `_annotationParameterAssignViaAstOut` (L64), `_identifierViaAstIn` (L69), `_literalViaAstIn` (L73), `_memberViaAstIn` (L77), `_methodViaAstIn` (L81), `_methodParameterInViaAstIn` (L85), `_methodRefViaAstIn` (L89), `_typeDeclViaAstIn` (L93), `_unknownViaAstIn` (L97). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Annotation.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5200355e2837027a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationLiteral.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `92f553e341cf498bc486be6e0a47145f56fa264e`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationLiteral.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4666571a352de1ac9b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationLiteral.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationLiteral.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dbc95ce4aa34a6fc8e868d4fe54c47cd03db3b8be84207da7cdbbaf2e2877920`.

> 
> final class AccessNeighborsForAnnotationLiteral(val node: nodes.AnnotationLiteral) extends AnyVal {
> 
>   /** Traverse to ANNOTATION_PARAMETER_ASSIGN via AST IN edge.
>     */
>   def _annotationParameterAssignViaAstIn: Iterator[nodes.AnnotationParameterAssign] =
>     astIn.collectAll[nodes.AnnotationParameterAssign]
> 
>   def astIn: Iterator[nodes.AnnotationParameterAssign] = node._astIn.cast[nodes.AnnotationParameterAssign]
> }
> 
> final class AccessNeighborsForAnnotationLiteralTraversal(val traversal: Iterator[nodes.AnnotationLiteral])
>     extends AnyVal {
> 
>   /** Traverse to ANNOTATION_PARAMETER_ASSIGN via AST IN edge.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 6 lexically identified declarations, including `AccessNeighborsForAnnotationLiteral` (L6), `_annotationParameterAssignViaAstIn` (L10), `astIn` (L13), `AccessNeighborsForAnnotationLiteralTraversal` (L16), `_annotationParameterAssignViaAstIn` (L21), `astIn` (L24). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationLiteral.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8d6ee96861dcd492"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameter.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0928fbb56df237044d07b36f8edaa2392a3c8cdc`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameter.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-63ead558007222527f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameter.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameter.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0ee06ab8f2976edd49fb4790ddecc7b712f0135d632143bd2094abb3139fcc4a`.

> 
> final class AccessNeighborsForAnnotationParameter(val node: nodes.AnnotationParameter) extends AnyVal {
> 
>   /** Traverse to ANNOTATION_PARAMETER_ASSIGN via AST IN edge.
>     */
>   def _annotationParameterAssignViaAstIn: Iterator[nodes.AnnotationParameterAssign] =
>     astIn.collectAll[nodes.AnnotationParameterAssign]
> 
>   def astIn: Iterator[nodes.AnnotationParameterAssign] = node._astIn.cast[nodes.AnnotationParameterAssign]
> }
> 
> final class AccessNeighborsForAnnotationParameterTraversal(val traversal: Iterator[nodes.AnnotationParameter])
>     extends AnyVal {
> 
>   /** Traverse to ANNOTATION_PARAMETER_ASSIGN via AST IN edge.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 6 lexically identified declarations, including `AccessNeighborsForAnnotationParameter` (L6), `_annotationParameterAssignViaAstIn` (L10), `astIn` (L13), `AccessNeighborsForAnnotationParameterTraversal` (L16), `_annotationParameterAssignViaAstIn` (L21), `astIn` (L24). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameter.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d643e746513ef86a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameterAssign.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4f3f0cef25a404258379342f335ce830842db150`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameterAssign.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-49c9a059d7a4d16ae7.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameterAssign.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameterAssign.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `14c9bee50e21e15d311c0544196705887cbe3c3eb204aa11d8736d7095cb5d8a`.

> 
> final class AccessNeighborsForAnnotationParameterAssign(val node: nodes.AnnotationParameterAssign) extends AnyVal {
> 
>   /** Traverse to ANNOTATION via AST IN edge.
>     */
>   def _annotationViaAstIn: Iterator[nodes.Annotation] = astIn.collectAll[nodes.Annotation]
> 
>   /** Traverse to ANNOTATION via AST OUT edge.
>     */
>   def _annotationViaAstOut: Iterator[nodes.Annotation] = astOut.collectAll[nodes.Annotation]
> 
>   /** Traverse to ANNOTATION_LITERAL via AST OUT edge.
>     */
>   def _annotationLiteralViaAstOut: Iterator[nodes.AnnotationLiteral] = astOut.collectAll[nodes.AnnotationLiteral]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 17 lexically identified declarations, including `AccessNeighborsForAnnotationParameterAssign` (L6), `_annotationViaAstIn` (L10), `_annotationViaAstOut` (L14), `_annotationLiteralViaAstOut` (L18), `_annotationParameterViaAstOut` (L22), `_arrayInitializerViaAstOut` (L26), `astIn` (L28), `astOut` (L30), `AccessNeighborsForAnnotationParameterAssignTraversal` (L33), `traversal` (L34), `_annotationViaAstIn` (L39), `_annotationViaAstOut` (L43), `_annotationLiteralViaAstOut` (L47), `_annotationParameterViaAstOut` (L51), `_arrayInitializerViaAstOut` (L56), `astIn` (L58), `astOut` (L60). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/AnnotationParameterAssign.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9634abe195c4b427"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ArrayInitializer.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e76849ff505bd1b3a67bfe91edf770321edd928d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ArrayInitializer.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3256d32562e4b6e0c7.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ArrayInitializer.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ArrayInitializer.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0ebf02b3c74d8c0158e77317120a64c50bdabdeb359093212bc4e102fa75abf4`.

> 
> final class AccessNeighborsForArrayInitializer(val node: nodes.ArrayInitializer) extends AnyVal {
> 
>   /** Traverse to ANNOTATION_PARAMETER_ASSIGN via AST IN edge.
>     */
>   def _annotationParameterAssignViaAstIn: Iterator[nodes.AnnotationParameterAssign] =
>     astIn.collectAll[nodes.AnnotationParameterAssign]
> 
>   /** Traverse to LITERAL via AST OUT edge.
>     */
>   def _literalViaAstOut: Iterator[nodes.Literal] = astOut.collectAll[nodes.Literal]
> 
>   /** Traverse to TYPE via EVAL_TYPE OUT edge.
>     */
>   def _typeViaEvalTypeOut: Iterator[nodes.Type] = evalTypeOut.collectAll[nodes.Type]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 14 lexically identified declarations, including `AccessNeighborsForArrayInitializer` (L6), `_annotationParameterAssignViaAstIn` (L10), `_literalViaAstOut` (L15), `_typeViaEvalTypeOut` (L19), `astIn` (L21), `astOut` (L23), `evalTypeOut` (L25), `AccessNeighborsForArrayInitializerTraversal` (L28), `_annotationParameterAssignViaAstIn` (L33), `_literalViaAstOut` (L38), `_typeViaEvalTypeOut` (L42), `astIn` (L44), `astOut` (L46), `evalTypeOut` (L48). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ArrayInitializer.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-138a9c5a5113ad0d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Binding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `710922305e9eb3f83ff3140685082d670b6ecaee`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Binding.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-97e4316b80acf65b34.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Binding.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Binding.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `831441d9f133ac2f041fe46b2eb5da6a0b4b05f1ca267cd535f5cd32823ece50`.

> 
> final class AccessNeighborsForBinding(val node: nodes.Binding) extends AnyVal {
> 
>   /** Traverse to METHOD via REF OUT edge.
>     */
>   @deprecated("please use boundMethod instead")
>   def _methodViaRefOut: nodes.Method = boundMethod
> 
>   /** Traverse to METHOD via REF OUT edge.
>     */
>   def boundMethod: nodes.Method = {
>     try { refOut.collectAll[nodes.Method].next() }
>     catch {
>       case e: java.util.NoSuchElementException =>
>         val nodeInfo = String.format("id=%d, (seq=%d)", node.id, node.seq)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `AccessNeighborsForBinding` (L6), `_methodViaRefOut` (L11), `boundMethod` (L15), `nodeInfo` (L19), `_typeDeclViaBindsIn` (L30), `bindingTypeDecl` (L34), `nodeInfo` (L38), `bindsIn` (L46), `refOut` (L48), `AccessNeighborsForBindingTraversal` (L51), `boundMethod` (L55), `_methodViaRefOut` (L60), `bindingTypeDecl` (L64), `_typeDeclViaBindsIn` (L69), `bindsIn` (L71), `refOut` (L73). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Binding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ecf164bce76eb66a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Block.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5ca91ea5acfb39faac47419082753736ebbf01cb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Block.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-24a9c499a60a095da4.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Block.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Block.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `77749cf6d28f0448beadf9844cb9c14a7bd262586ac3380172893e0f31bfd792`.

> 
> final class AccessNeighborsForBlock(val node: nodes.Block) extends AnyVal {
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: nodes.Block = {
>     try { astIn.collectAll[nodes.Block].next() }
>     catch {
>       case e: java.util.NoSuchElementException =>
>         val nodeInfo = String.format("id=%d, (seq=%d)", node.id, node.seq)
>         throw new flatgraph.SchemaViolationException(
>           "IN edge with label AST to an adjacent BLOCK is mandatory, but not defined for this BLOCK node with " + nodeInfo,
>           e
>         )
>     }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 286 lexically identified declarations, including `AccessNeighborsForBlock` (L6), `_blockViaAstIn` (L10), `nodeInfo` (L14), `_blockViaAstOut` (L24), `_blockViaCdgIn` (L28), `_blockViaCdgOut` (L32), `_blockViaDominateIn` (L36), `_blockViaDominateOut` (L40), `_blockViaPostDominateIn` (L44), `_blockViaPostDominateOut` (L48), `_blockViaReachingDefIn` (L52), `_blockViaReachingDefOut` (L56), `_callViaArgumentIn` (L60), `_callViaAstIn` (L64), `_callViaAstOut` (L68), `_callViaCdgIn` (L72), `_callViaCdgOut` (L76), `_callViaDominateIn` (L80), `_callViaDominateOut` (L84), `_callViaPostDominateIn` (L88), `_callViaPostDominateOut` (L92), `_callViaReachingDefOut` (L96), `_callViaReceiverIn` (L100), `_cfgNodeViaCfgOut` (L104). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Block.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-76850920bea5639b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Call.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f89e26380e2fab58c146a48713dcd996c02e28a9`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Call.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-eff099b2fd7a676179.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Call.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Call.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e33133960f9abbed3b76e069903b1573ea7363abf2bd340b0ad829173f7ab9a3`.

> 
> final class AccessNeighborsForCall(val node: nodes.Call) extends AnyVal {
> 
>   /** Traverse to BLOCK via ARGUMENT OUT edge.
>     */
>   def _blockViaArgumentOut: Iterator[nodes.Block] = argumentOut.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via AST OUT edge.
>     */
>   def _blockViaAstOut: Iterator[nodes.Block] = astOut.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 353 lexically identified declarations, including `AccessNeighborsForCall` (L6), `_blockViaArgumentOut` (L10), `_blockViaAstIn` (L14), `_blockViaAstOut` (L18), `_blockViaCdgIn` (L22), `_blockViaCdgOut` (L26), `_blockViaDominateIn` (L30), `_blockViaDominateOut` (L34), `_blockViaPostDominateIn` (L38), `_blockViaPostDominateOut` (L42), `_blockViaReachingDefIn` (L46), `_blockViaReceiverOut` (L50), `_callViaArgumentIn` (L54), `_callViaArgumentOut` (L58), `_callViaAstIn` (L62), `_callViaAstOut` (L66), `_callViaCdgIn` (L70), `_callViaCdgOut` (L74), `_callViaDominateIn` (L78), `_callViaDominateOut` (L82), `_callViaPostDominateIn` (L86), `_callViaPostDominateOut` (L90), `_callViaReachingDefIn` (L94), `_callViaReachingDefOut` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Call.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-eb52b078cf2160aa"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/CfgNode.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `30e555aa742a6a1255a74adadba7e1195fb7e5a4`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/CfgNode.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-26d25fc25b2f6d50c6.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/CfgNode.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/CfgNode.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `4f185271acb32ba457c89d95e5d73d50debc0b9eaf69cc42f35c5f6e27a50569`.

> 
> final class AccessNeighborsForCfgNode(val node: nodes.CfgNode) extends AnyVal {
> 
>   /** Traverse to BLOCK via CFG IN edge.
>     */
>   def _blockViaCfgIn: Iterator[nodes.Block] = cfgIn.collectAll[nodes.Block]
> 
>   /** Traverse to CALL via CFG IN edge.
>     */
>   def _callViaCfgIn: Iterator[nodes.Call] = cfgIn.collectAll[nodes.Call]
> 
>   /** Traverse to CONTROL_STRUCTURE via CFG IN edge.
>     */
>   def _controlStructureViaCfgIn: Iterator[nodes.ControlStructure] = cfgIn.collectAll[nodes.ControlStructure]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 26 lexically identified declarations, including `AccessNeighborsForCfgNode` (L6), `_blockViaCfgIn` (L10), `_callViaCfgIn` (L14), `_controlStructureViaCfgIn` (L18), `_fieldIdentifierViaCfgIn` (L22), `_identifierViaCfgIn` (L26), `_jumpTargetViaCfgIn` (L30), `_literalViaCfgIn` (L34), `_methodViaCfgIn` (L38), `_methodRefViaCfgIn` (L42), `_typeRefViaCfgIn` (L46), `_unknownViaCfgIn` (L50), `cfgIn` (L52), `AccessNeighborsForCfgNodeTraversal` (L55), `_blockViaCfgIn` (L59), `_callViaCfgIn` (L63), `_controlStructureViaCfgIn` (L67), `_fieldIdentifierViaCfgIn` (L71), `_identifierViaCfgIn` (L75), `_jumpTargetViaCfgIn` (L79), `_literalViaCfgIn` (L83), `_methodViaCfgIn` (L87), `_methodRefViaCfgIn` (L91), `_typeRefViaCfgIn` (L95). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/CfgNode.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7ec7874da2aeb09b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ClosureBinding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7a7cc3efed6372daa069635df810ba5f2b0baaf8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ClosureBinding.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-12cb5b4f7c3874f973.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ClosureBinding.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ClosureBinding.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b6014ad6b18cde19550542469f25d8f2140c3c1d901d80f8f001b379078c69a2`.

> 
> final class AccessNeighborsForClosureBinding(val node: nodes.ClosureBinding) extends AnyVal {
> 
>   /** Traverse to LOCAL via CAPTURED_BY IN edge.
>     */
>   def _localViaCapturedByIn: Iterator[nodes.Local] = capturedByIn.collectAll[nodes.Local]
> 
>   /** Traverse to LOCAL via REF OUT edge.
>     */
>   def _localViaRefOut: Option[nodes.Local] = refOut.collectAll[nodes.Local].nextOption()
> 
>   /** Traverse to METHOD_PARAMETER_IN via CAPTURED_BY IN edge.
>     */
>   def _methodParameterInViaCapturedByIn: Iterator[nodes.MethodParameterIn] =
>     capturedByIn.collectAll[nodes.MethodParameterIn]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `AccessNeighborsForClosureBinding` (L6), `_localViaCapturedByIn` (L10), `_localViaRefOut` (L14), `_methodParameterInViaCapturedByIn` (L18), `_methodParameterInViaRefOut` (L23), `_methodRefViaCaptureIn` (L28), `_typeRefViaCaptureIn` (L32), `captureIn` (L34), `capturedByIn` (L36), `refOut` (L38), `AccessNeighborsForClosureBindingTraversal` (L41), `_localViaCapturedByIn` (L45), `_localViaRefOut` (L49), `_methodParameterInViaCapturedByIn` (L53), `_methodParameterInViaRefOut` (L58), `_methodRefViaCaptureIn` (L62), `_typeRefViaCaptureIn` (L66), `captureIn` (L68), `capturedByIn` (L70), `refOut` (L72). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ClosureBinding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-edaac79c9325646f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Comment.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `13c263aec669f426b45a08e26c3250b863824680`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Comment.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fcc72defa7311bc60c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Comment.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Comment.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c62cdc6592981754dc06733e2652b6dd4ace966002dc270697d6d5563ac06334`.

> 
> final class AccessNeighborsForComment(val node: nodes.Comment) extends AnyVal {
> 
>   /** Traverse to COMMENT via SOURCE_FILE IN edge.
>     */
>   def _commentViaSourceFileIn: Iterator[nodes.Comment] = sourceFileIn.collectAll[nodes.Comment]
> 
>   /** Traverse to COMMENT via SOURCE_FILE OUT edge.
>     */
>   @deprecated("please use file instead")
>   def _commentViaSourceFileOut: Iterator[nodes.Comment] = file
> 
>   /** Traverse to COMMENT via SOURCE_FILE OUT edge.
>     */
>   def file: Iterator[nodes.Comment] = sourceFileOut.collectAll[nodes.Comment]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `AccessNeighborsForComment` (L6), `_commentViaSourceFileIn` (L10), `_commentViaSourceFileOut` (L15), `file` (L19), `_fileViaAstIn` (L23), `astIn` (L25), `sourceFileIn` (L27), `sourceFileOut` (L29), `AccessNeighborsForCommentTraversal` (L32), `_commentViaSourceFileIn` (L36), `file` (L40), `_commentViaSourceFileOut` (L45), `_fileViaAstIn` (L49), `astIn` (L51), `sourceFileIn` (L53), `sourceFileOut` (L55). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Comment.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ab88d9bdcb15eaa9"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ControlStructure.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `fd621e12da464a00a931bee2f14ceef375579863`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ControlStructure.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-61f233ee9031d63042.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ControlStructure.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ControlStructure.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `655330e22a6bfd40d5425823922416d39ddd38b9fe27aaa55d9cff583dd9a343`.

> 
> final class AccessNeighborsForControlStructure(val node: nodes.ControlStructure) extends AnyVal {
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via AST OUT edge.
>     */
>   def _blockViaAstOut: Iterator[nodes.Block] = astOut.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via CATCH_BODY OUT edge.
>     */
>   def _blockViaCatchBodyOut: Iterator[nodes.Block] = catchBodyOut.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 500 lexically identified declarations, including `AccessNeighborsForControlStructure` (L6), `_blockViaAstIn` (L10), `_blockViaAstOut` (L14), `_blockViaCatchBodyOut` (L18), `_blockViaCdgIn` (L22), `_blockViaCdgOut` (L26), `_blockViaConditionOut` (L30), `_blockViaDominateIn` (L34), `_blockViaDominateOut` (L38), `_blockViaDoBodyOut` (L42), `_blockViaFalseBodyOut` (L46), `_blockViaFinallyBodyOut` (L50), `_blockViaForBodyOut` (L54), `_blockViaForInitOut` (L58), `_blockViaForUpdateOut` (L62), `_blockViaPostDominateIn` (L66), `_blockViaPostDominateOut` (L70), `_blockViaTrueBodyOut` (L74), `_blockViaTryBodyOut` (L78), `_callViaArgumentIn` (L82), `_callViaAstIn` (L86), `_callViaAstOut` (L90), `_callViaCatchBodyOut` (L94), `_callViaCdgIn` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/ControlStructure.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-92cbff7d56e714b9"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Dependency.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c3a3da43394c5b3dfe55e3ae006084076514e13c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Dependency.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-35643b5f4933fbcf29.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Dependency.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Dependency.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dc2866fbeef9907e2442d46c0db7c6adcd438e18f294021b295165733494a62a`.

> 
> final class AccessNeighborsForDependency(val node: nodes.Dependency) extends AnyVal {
> 
>   /** Traverse to IMPORT via IMPORTS IN edge.
>     */
>   def _importViaImportsIn: Iterator[nodes.Import] = importsIn.collectAll[nodes.Import]
> 
>   def importsIn: Iterator[nodes.Import] = node._importsIn.cast[nodes.Import]
> }
> 
> final class AccessNeighborsForDependencyTraversal(val traversal: Iterator[nodes.Dependency]) extends AnyVal {
> 
>   /** Traverse to IMPORT via IMPORTS IN edge.
>     */
>   def _importViaImportsIn: Iterator[nodes.Import] = traversal.flatMap(_._importViaImportsIn)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 6 lexically identified declarations, including `AccessNeighborsForDependency` (L6), `_importViaImportsIn` (L10), `importsIn` (L12), `AccessNeighborsForDependencyTraversal` (L15), `_importViaImportsIn` (L19), `importsIn` (L21). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Dependency.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4342fead3f172142"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Expression.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `46fe6e9b81b001cb413e2c52cafd8dc1cb671a52`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Expression.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ef7d5fd7547bc5c29f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Expression.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Expression.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `53fe0b481cde96b02da79fedf3530a59a49ad5f94b2884527daa21b2b66e4308`.

> 
> final class AccessNeighborsForExpression(val node: nodes.Expression) extends AnyVal {
> 
>   /** Traverse to TEMPLATE_DOM via ARGUMENT OUT edge.
>     */
>   def _templateDomViaArgumentOut: Iterator[nodes.TemplateDom] = argumentOut.collectAll[nodes.TemplateDom]
> 
>   /** Traverse to TEMPLATE_DOM via AST IN edge.
>     */
>   def _templateDomViaAstIn: Iterator[nodes.TemplateDom] = astIn.collectAll[nodes.TemplateDom]
> 
>   /** Traverse to TEMPLATE_DOM via REACHING_DEF IN edge.
>     */
>   def _templateDomViaReachingDefIn: Iterator[nodes.TemplateDom] = reachingDefIn.collectAll[nodes.TemplateDom]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 14 lexically identified declarations, including `AccessNeighborsForExpression` (L6), `_templateDomViaArgumentOut` (L10), `_templateDomViaAstIn` (L14), `_templateDomViaReachingDefIn` (L18), `argumentOut` (L20), `astIn` (L22), `reachingDefIn` (L24), `AccessNeighborsForExpressionTraversal` (L27), `_templateDomViaArgumentOut` (L31), `_templateDomViaAstIn` (L35), `_templateDomViaReachingDefIn` (L39), `argumentOut` (L41), `astIn` (L43), `reachingDefIn` (L45). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Expression.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6bb2fdf64c4c0d0c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/FieldIdentifier.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5744978849eb72e4b71ac44a2c3bfa674da30d9c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/FieldIdentifier.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-256992003baa66eebb.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/FieldIdentifier.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/FieldIdentifier.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6d5d5de21fad9752f202cc6843f9603a812f8bfed4311e20a03db53ba6bdbdb0`.

> 
> final class AccessNeighborsForFieldIdentifier(val node: nodes.FieldIdentifier) extends AnyVal {
> 
>   /** Traverse to BLOCK via CDG IN edge.
>     */
>   def _blockViaCdgIn: Iterator[nodes.Block] = cdgIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via CDG OUT edge.
>     */
>   def _blockViaCdgOut: Iterator[nodes.Block] = cdgOut.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via DOMINATE IN edge.
>     */
>   def _blockViaDominateIn: Iterator[nodes.Block] = dominateIn.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 181 lexically identified declarations, including `AccessNeighborsForFieldIdentifier` (L6), `_blockViaCdgIn` (L10), `_blockViaCdgOut` (L14), `_blockViaDominateIn` (L18), `_blockViaDominateOut` (L22), `_blockViaPostDominateIn` (L26), `_blockViaPostDominateOut` (L30), `_callViaArgumentIn` (L34), `nodeInfo` (L38), `_callViaAstIn` (L48), `nodeInfo` (L52), `_callViaCdgIn` (L62), `_callViaCdgOut` (L66), `_callViaCfgOut` (L70), `nodeInfo` (L74), `_callViaDominateIn` (L84), `_callViaDominateOut` (L88), `_callViaPostDominateIn` (L92), `_callViaPostDominateOut` (L96), `_cfgNodeViaCfgOut` (L100), `_controlStructureViaCdgIn` (L104), `_controlStructureViaCdgOut` (L108), `_controlStructureViaDominateIn` (L112), `_controlStructureViaDominateOut` (L116). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/FieldIdentifier.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cfe3981a88f809b4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/File.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4c00c2fa8e8e38d3e33b480a5cd0e2c7f3f80f6c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/File.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4a059fa33c3328c1f7.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/File.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/File.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `bcb4f7b2cedbe7943c597e4ed79e3c698a8c6a1d7f483c25d9f18d8461bf2656`.

> 
> final class AccessNeighborsForFile(val node: nodes.File) extends AnyVal {
> 
>   /** Traverse to COMMENT via AST OUT edge.
>     */
>   @deprecated("please use comment instead")
>   def _commentViaAstOut: Iterator[nodes.Comment] = comment
> 
>   /** Traverse to COMMENT via AST OUT edge.
>     */
>   def comment: Iterator[nodes.Comment] = astOut.collectAll[nodes.Comment]
> 
>   /** Traverse to IMPORT via AST OUT edge.
>     */
>   def _importViaAstOut: Iterator[nodes.Import] = astOut.collectAll[nodes.Import]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 38 lexically identified declarations, including `AccessNeighborsForFile` (L6), `_commentViaAstOut` (L11), `comment` (L15), `_importViaAstOut` (L19), `_methodViaContainsOut` (L23), `_methodViaSourceFileIn` (L28), `method` (L32), `_namespaceBlockViaAstOut` (L36), `_namespaceBlockViaSourceFileIn` (L41), `namespaceBlock` (L45), `_tagViaTaggedByOut` (L49), `_templateDomViaContainsOut` (L53), `_typeDeclViaContainsOut` (L57), `_typeDeclViaSourceFileIn` (L62), `typeDecl` (L66), `astOut` (L68), `containsOut` (L70), `sourceFileIn` (L72), `taggedByOut` (L74), `AccessNeighborsForFileTraversal` (L77), `comment` (L81), `_commentViaAstOut` (L86), `_importViaAstOut` (L90), `_methodViaContainsOut` (L94). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/File.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c0e695db23be2187"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Identifier.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0edaf1c0051b8746f199dd6ae7d5b8fe5ea17c33`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Identifier.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c62990bc368d4b9508.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Identifier.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Identifier.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `460dd19530afe1aa928b369cc4715e0fef4b8b7b585a1e8cfe5242987da4091a`.

> 
> final class AccessNeighborsForIdentifier(val node: nodes.Identifier) extends AnyVal {
> 
>   /** Traverse to ANNOTATION via AST OUT edge.
>     */
>   def _annotationViaAstOut: Iterator[nodes.Annotation] = astOut.collectAll[nodes.Annotation]
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via CDG IN edge.
>     */
>   def _blockViaCdgIn: Iterator[nodes.Block] = cdgIn.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 284 lexically identified declarations, including `AccessNeighborsForIdentifier` (L6), `_annotationViaAstOut` (L10), `_blockViaAstIn` (L14), `_blockViaCdgIn` (L18), `_blockViaCdgOut` (L22), `_blockViaDominateIn` (L26), `_blockViaDominateOut` (L30), `_blockViaPostDominateIn` (L34), `_blockViaPostDominateOut` (L38), `_blockViaReachingDefIn` (L42), `_callViaArgumentIn` (L46), `_callViaAstIn` (L50), `_callViaCdgIn` (L54), `_callViaCdgOut` (L58), `_callViaDominateIn` (L62), `_callViaDominateOut` (L66), `_callViaPostDominateIn` (L70), `_callViaPostDominateOut` (L74), `_callViaReachingDefIn` (L78), `_callViaReachingDefOut` (L82), `_callViaReceiverIn` (L86), `_cfgNodeViaCfgOut` (L90), `_controlStructureViaAstIn` (L94), `_controlStructureViaCatchBodyIn` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Identifier.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7b6a3b61896ef915"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Import.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `2fa843e59fa6edd0554efeae3148c6ff8814fc5b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Import.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c3f20205966466255d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Import.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Import.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3925830a4dec1572ce7838abcd49ba3ee2867fa4247e8538517383bec483304e`.

> 
> final class AccessNeighborsForImport(val node: nodes.Import) extends AnyVal {
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]
> 
>   /** Traverse to CALL via IS_CALL_FOR_IMPORT IN edge.
>     */
>   def _callViaIsCallForImportIn: Iterator[nodes.Call] = isCallForImportIn.collectAll[nodes.Call]
> 
>   /** Traverse to DEPENDENCY via IMPORTS OUT edge.
>     */
>   def _dependencyViaImportsOut: Iterator[nodes.Dependency] = importsOut.collectAll[nodes.Dependency]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 22 lexically identified declarations, including `AccessNeighborsForImport` (L6), `_blockViaAstIn` (L10), `_callViaIsCallForImportIn` (L14), `_dependencyViaImportsOut` (L18), `_fileViaAstIn` (L22), `_tagViaTaggedByOut` (L26), `_typeDeclViaAstIn` (L30), `astIn` (L32), `importsOut` (L34), `isCallForImportIn` (L36), `taggedByOut` (L38), `AccessNeighborsForImportTraversal` (L41), `_blockViaAstIn` (L45), `_callViaIsCallForImportIn` (L49), `_dependencyViaImportsOut` (L53), `_fileViaAstIn` (L57), `_tagViaTaggedByOut` (L61), `_typeDeclViaAstIn` (L65), `astIn` (L67), `importsOut` (L69), `isCallForImportIn` (L71), `taggedByOut` (L73). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Import.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-92b1d2195f89f192"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpLabel.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `90b40000edb2a35c34d6895f9f4e7a235c793be4`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpLabel.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7a2bf46fba9625d111.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpLabel.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpLabel.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7ddb29b105a18c0024d554fb68e15b8608d845dd3516a1a140ce5879b7e20bb2`.

> 
> final class AccessNeighborsForJumpLabel(val node: nodes.JumpLabel) extends AnyVal {
> 
>   /** Traverse to CONTROL_STRUCTURE via AST IN edge.
>     */
>   def _controlStructureViaAstIn: Iterator[nodes.ControlStructure] = astIn.collectAll[nodes.ControlStructure]
> 
>   /** Traverse to CONTROL_STRUCTURE via JUMP_ARGUMENT IN edge.
>     */
>   def _controlStructureViaJumpArgumentIn: Iterator[nodes.ControlStructure] =
>     jumpArgumentIn.collectAll[nodes.ControlStructure]
> 
>   def astIn: Iterator[nodes.ControlStructure] = node._astIn.cast[nodes.ControlStructure]
> 
>   def jumpArgumentIn: Iterator[nodes.ControlStructure] = node._jumpArgumentIn.cast[nodes.ControlStructure]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 10 lexically identified declarations, including `AccessNeighborsForJumpLabel` (L6), `_controlStructureViaAstIn` (L10), `_controlStructureViaJumpArgumentIn` (L14), `astIn` (L17), `jumpArgumentIn` (L19), `AccessNeighborsForJumpLabelTraversal` (L22), `_controlStructureViaAstIn` (L26), `_controlStructureViaJumpArgumentIn` (L30), `astIn` (L33), `jumpArgumentIn` (L35). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpLabel.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-48498daebcb37371"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpTarget.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `82671dc78b0a3d6c11dfc8871354a6d70cdf0cd9`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpTarget.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a27551ce855586a045.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpTarget.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpTarget.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a8378609c55c7534c3ec656d14cb9e7899815e1a8ea747953b5d59aa07250dc2`.

> 
> final class AccessNeighborsForJumpTarget(val node: nodes.JumpTarget) extends AnyVal {
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via CDG IN edge.
>     */
>   def _blockViaCdgIn: Iterator[nodes.Block] = cdgIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via CDG OUT edge.
>     */
>   def _blockViaCdgOut: Iterator[nodes.Block] = cdgOut.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 216 lexically identified declarations, including `AccessNeighborsForJumpTarget` (L6), `_blockViaAstIn` (L10), `_blockViaCdgIn` (L14), `_blockViaCdgOut` (L18), `_blockViaDominateIn` (L22), `_blockViaDominateOut` (L26), `_blockViaPostDominateIn` (L30), `_blockViaPostDominateOut` (L34), `_callViaArgumentIn` (L38), `_callViaCdgIn` (L42), `_callViaCdgOut` (L46), `_callViaDominateIn` (L50), `_callViaDominateOut` (L54), `_callViaPostDominateIn` (L58), `_callViaPostDominateOut` (L62), `_cfgNodeViaCfgOut` (L66), `_controlStructureViaAstIn` (L70), `_controlStructureViaCatchBodyIn` (L74), `_controlStructureViaCdgIn` (L78), `_controlStructureViaCdgOut` (L82), `_controlStructureViaConditionIn` (L86), `_controlStructureViaDominateIn` (L90), `_controlStructureViaDominateOut` (L94), `_controlStructureViaDoBodyIn` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/JumpTarget.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-89802247e4f97af4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Literal.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b66cf4314dfd3a3a2d021ae37a3e2e3c907ae007`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Literal.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-57e674f865bd719e0e.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Literal.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Literal.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c274a552c31946ad6fbb215cc88e53ec283c943bf4ed564919e5cc5953d0c061`.

> 
> final class AccessNeighborsForLiteral(val node: nodes.Literal) extends AnyVal {
> 
>   /** Traverse to ANNOTATION via AST OUT edge.
>     */
>   def _annotationViaAstOut: Iterator[nodes.Annotation] = astOut.collectAll[nodes.Annotation]
> 
>   /** Traverse to ARRAY_INITIALIZER via AST IN edge.
>     */
>   def _arrayInitializerViaAstIn: Iterator[nodes.ArrayInitializer] = astIn.collectAll[nodes.ArrayInitializer]
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 283 lexically identified declarations, including `AccessNeighborsForLiteral` (L6), `_annotationViaAstOut` (L10), `_arrayInitializerViaAstIn` (L14), `_blockViaAstIn` (L18), `_blockViaCdgIn` (L22), `_blockViaCdgOut` (L26), `_blockViaDominateIn` (L30), `_blockViaDominateOut` (L34), `_blockViaPostDominateIn` (L38), `_blockViaPostDominateOut` (L42), `_blockViaReachingDefIn` (L46), `_callViaArgumentIn` (L50), `_callViaAstIn` (L54), `_callViaCdgIn` (L58), `_callViaCdgOut` (L62), `_callViaDominateIn` (L66), `_callViaDominateOut` (L70), `_callViaPostDominateIn` (L74), `_callViaPostDominateOut` (L78), `_callViaReachingDefIn` (L82), `_callViaReachingDefOut` (L86), `_callViaReceiverIn` (L90), `_cfgNodeViaCfgOut` (L94), `_controlStructureViaAstIn` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Literal.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-83e23b762fb08aea"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Local.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `da65efda9dede038aa8d5629cd7b339e33826ff3`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Local.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6f0cb1697ad4562165.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Local.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Local.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `34ad1fa8894a036e96d80f96dbf283281827e523918e6f0ee6bdbfc1142b14b0`.

> 
> final class AccessNeighborsForLocal(val node: nodes.Local) extends AnyVal {
> 
>   /** Traverse to CLOSURE_BINDING via CAPTURED_BY OUT edge.
>     */
>   def _closureBindingViaCapturedByOut: Iterator[nodes.ClosureBinding] = capturedByOut.collectAll[nodes.ClosureBinding]
> 
>   /** Traverse to CLOSURE_BINDING via REF IN edge.
>     */
>   def _closureBindingViaRefIn: Iterator[nodes.ClosureBinding] = refIn.collectAll[nodes.ClosureBinding]
> 
>   /** Traverse to CONTROL_STRUCTURE via AST IN edge.
>     */
>   def _controlStructureViaAstIn: Iterator[nodes.ControlStructure] = astIn.collectAll[nodes.ControlStructure]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 34 lexically identified declarations, including `AccessNeighborsForLocal` (L6), `_closureBindingViaCapturedByOut` (L10), `_closureBindingViaRefIn` (L14), `_controlStructureViaAstIn` (L18), `_tagViaTaggedByOut` (L22), `_unknownViaAstIn` (L26), `_identifierViaRefIn` (L31), `referencingIdentifiers` (L35), `_blockViaAstIn` (L40), `definingBlock` (L44), `_typeViaEvalTypeOut` (L49), `typ` (L53), `astIn` (L55), `capturedByOut` (L57), `evalTypeOut` (L59), `refIn` (L61), `taggedByOut` (L63), `AccessNeighborsForLocalTraversal` (L66), `_closureBindingViaCapturedByOut` (L70), `_closureBindingViaRefIn` (L75), `_controlStructureViaAstIn` (L79), `_tagViaTaggedByOut` (L83), `_unknownViaAstIn` (L87), `referencingIdentifiers` (L91). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Local.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d444c601e88d8898"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Member.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `2ae74df83417b04d5ec77ecc354fbead9651dbab`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Member.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-77adab08d48dbbed2d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Member.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Member.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `8aac695ed18a2448fc7a6e5eeb5a6a56418809168f34d1233e489d5b58219c51`.

> 
> final class AccessNeighborsForMember(val node: nodes.Member) extends AnyVal {
> 
>   /** Traverse to ANNOTATION via AST OUT edge.
>     */
>   def _annotationViaAstOut: Iterator[nodes.Annotation] = astOut.collectAll[nodes.Annotation]
> 
>   /** Traverse to CALL via REF IN edge.
>     */
>   def _callViaRefIn: Iterator[nodes.Call] = refIn.collectAll[nodes.Call]
> 
>   /** Traverse to MODIFIER via AST OUT edge.
>     */
>   def _modifierViaAstOut: Iterator[nodes.Modifier] = astOut.collectAll[nodes.Modifier]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 31 lexically identified declarations, including `AccessNeighborsForMember` (L6), `_annotationViaAstOut` (L10), `_callViaRefIn` (L14), `_modifierViaAstOut` (L18), `_tagViaTaggedByOut` (L22), `_unknownViaAstIn` (L26), `_typeDeclViaAstIn` (L31), `typeDecl` (L35), `nodeInfo` (L39), `_typeViaEvalTypeOut` (L50), `typ` (L54), `astIn` (L56), `astOut` (L58), `evalTypeOut` (L60), `refIn` (L62), `taggedByOut` (L64), `AccessNeighborsForMemberTraversal` (L67), `_annotationViaAstOut` (L71), `_callViaRefIn` (L75), `_modifierViaAstOut` (L79), `_tagViaTaggedByOut` (L83), `_unknownViaAstIn` (L87), `typeDecl` (L91), `_typeDeclViaAstIn` (L96). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Member.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6a019959a1b4bf78"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Method.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `59c2b773df6ef4528ff0832f7f84c27b37f26151`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Method.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-68f8d977d3f8b8c55f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Method.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Method.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7c9dde6dafa38b0de781ceca203f1da90fd1b10f52283eb7b74fc2a98ff3315f`.

> 
> final class AccessNeighborsForMethod(val node: nodes.Method) extends AnyVal {
> 
>   /** Traverse to ANNOTATION via AST OUT edge.
>     */
>   def _annotationViaAstOut: Iterator[nodes.Annotation] = astOut.collectAll[nodes.Annotation]
> 
>   /** Traverse to BINDING via REF IN edge.
>     */
>   def _bindingViaRefIn: Iterator[nodes.Binding] = refIn.collectAll[nodes.Binding]
> 
>   /** Traverse to BLOCK via CONTAINS OUT edge.
>     */
>   def _blockViaContainsOut: Iterator[nodes.Block] = containsOut.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 162 lexically identified declarations, including `AccessNeighborsForMethod` (L6), `_annotationViaAstOut` (L10), `_bindingViaRefIn` (L14), `_blockViaContainsOut` (L18), `_blockViaDominateOut` (L22), `_blockViaPostDominateIn` (L26), `_callViaCallIn` (L30), `_callViaContainsOut` (L34), `_callViaDominateOut` (L38), `_callViaPostDominateIn` (L42), `_callViaReachingDefOut` (L46), `_controlStructureViaContainsOut` (L50), `_controlStructureViaPostDominateIn` (L54), `_fieldIdentifierViaContainsOut` (L59), `_fieldIdentifierViaDominateOut` (L63), `_fieldIdentifierViaPostDominateIn` (L67), `_fileViaContainsIn` (L72), `_fileViaSourceFileOut` (L76), `_identifierViaContainsOut` (L80), `_identifierViaDominateOut` (L84), `_identifierViaPostDominateIn` (L88), `_identifierViaReachingDefOut` (L92), `_jumpTargetViaContainsOut` (L96), `_literalViaDominateOut` (L100). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Method.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c37e681ea957f4db"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterIn.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1cacc10c361fc66274afd3264f6c02f73d18b4a8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterIn.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bab566cd3ddcff24d3.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterIn.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterIn.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b0e17742e1ceddf6b56aa0a19b2cbf5d5c0d2ddd29e48f03cdefd5531b4af135`.

> 
> final class AccessNeighborsForMethodParameterIn(val node: nodes.MethodParameterIn) extends AnyVal {
> 
>   /** Traverse to ANNOTATION via AST OUT edge.
>     */
>   def _annotationViaAstOut: Iterator[nodes.Annotation] = astOut.collectAll[nodes.Annotation]
> 
>   /** Traverse to CALL via REACHING_DEF OUT edge.
>     */
>   def _callViaReachingDefOut: Iterator[nodes.Call] = reachingDefOut.collectAll[nodes.Call]
> 
>   /** Traverse to CLOSURE_BINDING via CAPTURED_BY OUT edge.
>     */
>   def _closureBindingViaCapturedByOut: Iterator[nodes.ClosureBinding] = capturedByOut.collectAll[nodes.ClosureBinding]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 64 lexically identified declarations, including `AccessNeighborsForMethodParameterIn` (L6), `_annotationViaAstOut` (L10), `_callViaReachingDefOut` (L14), `_closureBindingViaCapturedByOut` (L18), `_closureBindingViaRefIn` (L22), `_identifierViaReachingDefOut` (L26), `_literalViaReachingDefOut` (L30), `_methodViaReachingDefIn` (L34), `_methodParameterOutViaReachingDefOut` (L38), `_methodRefViaReachingDefOut` (L43), `_returnViaReachingDefOut` (L47), `_tagViaTaggedByOut` (L51), `_typeRefViaReachingDefOut` (L55), `_unknownViaAstOut` (L59), `_identifierViaRefIn` (L64), `referencingIdentifiers` (L68), `_methodParameterOutViaParameterLinkOut` (L73), `asOutput` (L77), `_methodViaAstIn` (L82), `method` (L86), `nodeInfo` (L90), `_typeViaEvalTypeOut` (L101), `typ` (L105), `nodeInfo` (L109). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterIn.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9d526574d87f02f3"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterOut.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `919ef30a1b3495c9bafdafefecfe9c3e97104f4f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterOut.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-862eae573898eb09ea.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterOut.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterOut.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `68e689e679fa40042e54f399a2530caf9bcb13eed3d3a5bb48cef5b8d2c616dc`.

> 
> final class AccessNeighborsForMethodParameterOut(val node: nodes.MethodParameterOut) extends AnyVal {
> 
>   /** Traverse to BLOCK via REACHING_DEF IN edge.
>     */
>   def _blockViaReachingDefIn: Iterator[nodes.Block] = reachingDefIn.collectAll[nodes.Block]
> 
>   /** Traverse to CALL via REACHING_DEF IN edge.
>     */
>   def _callViaReachingDefIn: Iterator[nodes.Call] = reachingDefIn.collectAll[nodes.Call]
> 
>   /** Traverse to CALL via REACHING_DEF OUT edge.
>     */
>   def _callViaReachingDefOut: Iterator[nodes.Call] = reachingDefOut.collectAll[nodes.Call]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 63 lexically identified declarations, including `AccessNeighborsForMethodParameterOut` (L6), `_blockViaReachingDefIn` (L10), `_callViaReachingDefIn` (L14), `_callViaReachingDefOut` (L18), `_controlStructureViaReachingDefIn` (L22), `_identifierViaReachingDefIn` (L27), `_identifierViaReachingDefOut` (L31), `_literalViaReachingDefIn` (L35), `_literalViaReachingDefOut` (L39), `_methodViaAstIn` (L44), `method` (L48), `nodeInfo` (L52), `_methodViaReachingDefIn` (L62), `_methodParameterInViaParameterLinkIn` (L67), `asInput` (L71), `_methodParameterInViaReachingDefIn` (L75), `_methodRefViaReachingDefIn` (L80), `_methodRefViaReachingDefOut` (L84), `_returnViaReachingDefIn` (L88), `_returnViaReachingDefOut` (L92), `_tagViaTaggedByOut` (L96), `_typeRefViaReachingDefIn` (L100), `_typeRefViaReachingDefOut` (L104), `_unknownViaReachingDefIn` (L108). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodParameterOut.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d577c9aadc72ad22"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodRef.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4643efd31eca754585ca838490ae95f65bd33a99`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodRef.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-42dfb7e6d481f35b3a.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodRef.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodRef.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0fe3c6e78f664a8997cf6ee70ec81ace289ecfff0c29635b2f68cbe8464021a1`.

> 
> final class AccessNeighborsForMethodRef(val node: nodes.MethodRef) extends AnyVal {
> 
>   /** Traverse to ANNOTATION via AST OUT edge.
>     */
>   def _annotationViaAstOut: Iterator[nodes.Annotation] = astOut.collectAll[nodes.Annotation]
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via CDG IN edge.
>     */
>   def _blockViaCdgIn: Iterator[nodes.Block] = cdgIn.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 290 lexically identified declarations, including `AccessNeighborsForMethodRef` (L6), `_annotationViaAstOut` (L10), `_blockViaAstIn` (L14), `_blockViaCdgIn` (L18), `_blockViaCdgOut` (L22), `_blockViaDominateIn` (L26), `_blockViaDominateOut` (L30), `_blockViaPostDominateIn` (L34), `_blockViaPostDominateOut` (L38), `_blockViaReachingDefIn` (L42), `_callViaArgumentIn` (L46), `_callViaAstIn` (L50), `_callViaCdgIn` (L54), `_callViaCdgOut` (L58), `_callViaDominateIn` (L62), `_callViaDominateOut` (L66), `_callViaPostDominateIn` (L70), `_callViaPostDominateOut` (L74), `_callViaReachingDefIn` (L78), `_callViaReachingDefOut` (L82), `_callViaReceiverIn` (L86), `_cfgNodeViaCfgOut` (L90), `_closureBindingViaCaptureOut` (L94), `_controlStructureViaAstIn` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodRef.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e9ca6669317c5b00"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodReturn.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `28476e63a6894e7e4f90c49b2599f75d5f3c0eaf`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodReturn.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ead1dcb322381a6b0f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodReturn.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodReturn.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e1932ec3f58e4bbde14490640b58459e16b98e45a1e14f72a56f52f5c78c6ead`.

> 
> final class AccessNeighborsForMethodReturn(val node: nodes.MethodReturn) extends AnyVal {
> 
>   /** Traverse to BLOCK via CDG IN edge.
>     */
>   def _blockViaCdgIn: Iterator[nodes.Block] = cdgIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via DOMINATE IN edge.
>     */
>   def _blockViaDominateIn: Iterator[nodes.Block] = dominateIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via POST_DOMINATE OUT edge.
>     */
>   def _blockViaPostDominateOut: Iterator[nodes.Block] = postDominateOut.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 105 lexically identified declarations, including `AccessNeighborsForMethodReturn` (L6), `_blockViaCdgIn` (L10), `_blockViaDominateIn` (L14), `_blockViaPostDominateOut` (L18), `_callViaCdgIn` (L22), `_callViaCfgIn` (L26), `_callViaDominateIn` (L30), `_callViaPostDominateOut` (L34), `_controlStructureViaCdgIn` (L38), `_controlStructureViaDominateIn` (L42), `_controlStructureViaPostDominateOut` (L46), `_fieldIdentifierViaCdgIn` (L51), `_fieldIdentifierViaDominateIn` (L55), `_fieldIdentifierViaPostDominateOut` (L59), `_identifierViaCdgIn` (L64), `_identifierViaDominateIn` (L68), `_identifierViaPostDominateOut` (L72), `_jumpTargetViaCdgIn` (L76), `_jumpTargetViaPostDominateOut` (L80), `_literalViaCdgIn` (L84), `_literalViaDominateIn` (L88), `_literalViaPostDominateOut` (L92), `_methodViaAstIn` (L96), `nodeInfo` (L100). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/MethodReturn.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-aa99143a1f89f7a6"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Modifier.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `926a6b3af29949d04cbdffb59f0445742bb6b2ba`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Modifier.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0a4a06e21fcddecaf3.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Modifier.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Modifier.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `bc10f5e7d6af3949a65fdefbc33b78ee7a0295e68502319435db865757f388e5`.

> 
> final class AccessNeighborsForModifier(val node: nodes.Modifier) extends AnyVal {
> 
>   /** Traverse to CONTROL_STRUCTURE via AST IN edge.
>     */
>   def _controlStructureViaAstIn: Iterator[nodes.ControlStructure] = astIn.collectAll[nodes.ControlStructure]
> 
>   /** Traverse to MEMBER via AST IN edge.
>     */
>   def _memberViaAstIn: Iterator[nodes.Member] = astIn.collectAll[nodes.Member]
> 
>   /** Traverse to METHOD via AST IN edge.
>     */
>   def _methodViaAstIn: nodes.Method = {
>     try { astIn.collectAll[nodes.Method].next() }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `AccessNeighborsForModifier` (L6), `_controlStructureViaAstIn` (L10), `_memberViaAstIn` (L14), `_methodViaAstIn` (L18), `nodeInfo` (L22), `_typeDeclViaAstIn` (L32), `nodeInfo` (L36), `_unknownViaAstIn` (L46), `astIn` (L48), `AccessNeighborsForModifierTraversal` (L51), `_controlStructureViaAstIn` (L55), `_memberViaAstIn` (L59), `_methodViaAstIn` (L63), `_typeDeclViaAstIn` (L67), `_unknownViaAstIn` (L71), `astIn` (L73). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Modifier.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1f8551acfdcb0f2b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Namespace.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `92ce10296dae66a2d5d4d80289b934b26cd74e8c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Namespace.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a34b373a3a1e835a45.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Namespace.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Namespace.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c4e31adddf52302c7b077bb23fa0d4f1344fc171bff001fbab62a8909d8d0ea5`.

> 
> final class AccessNeighborsForNamespace(val node: nodes.Namespace) extends AnyVal {
> 
>   /** Traverse to NAMESPACE_BLOCK via REF IN edge.
>     */
>   def _namespaceBlockViaRefIn: Iterator[nodes.NamespaceBlock] = refIn.collectAll[nodes.NamespaceBlock]
> 
>   def refIn: Iterator[nodes.NamespaceBlock] = node._refIn.cast[nodes.NamespaceBlock]
> }
> 
> final class AccessNeighborsForNamespaceTraversal(val traversal: Iterator[nodes.Namespace]) extends AnyVal {
> 
>   /** Traverse to NAMESPACE_BLOCK via REF IN edge.
>     */
>   def _namespaceBlockViaRefIn: Iterator[nodes.NamespaceBlock] = traversal.flatMap(_._namespaceBlockViaRefIn)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 6 lexically identified declarations, including `AccessNeighborsForNamespace` (L6), `_namespaceBlockViaRefIn` (L10), `refIn` (L12), `AccessNeighborsForNamespaceTraversal` (L15), `_namespaceBlockViaRefIn` (L19), `refIn` (L21). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Namespace.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fe8fde5f182396bd"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/NamespaceBlock.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `98967b1391c299fc680f102a08bc0ff589f8c0bf`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/NamespaceBlock.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c46cbf6f6924f0401a.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/NamespaceBlock.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/NamespaceBlock.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `ebbf3afdaf6d7e68cddcdcb4bf7b439dc87266ec8ef20eb633f3fadfd9107b1d`.

> 
> final class AccessNeighborsForNamespaceBlock(val node: nodes.NamespaceBlock) extends AnyVal {
> 
>   /** Traverse to FILE via AST IN edge.
>     */
>   def _fileViaAstIn: Option[nodes.File] = astIn.collectAll[nodes.File].nextOption()
> 
>   /** Traverse to FILE via SOURCE_FILE OUT edge.
>     */
>   def _fileViaSourceFileOut: Iterator[nodes.File] = sourceFileOut.collectAll[nodes.File]
> 
>   /** Traverse to METHOD via AST OUT edge.
>     */
>   def _methodViaAstOut: Iterator[nodes.Method] = astOut.collectAll[nodes.Method]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `AccessNeighborsForNamespaceBlock` (L6), `_fileViaAstIn` (L10), `_fileViaSourceFileOut` (L14), `_methodViaAstOut` (L18), `_namespaceViaRefOut` (L22), `_typeDeclViaAstOut` (L26), `astIn` (L28), `astOut` (L30), `refOut` (L32), `sourceFileOut` (L34), `AccessNeighborsForNamespaceBlockTraversal` (L37), `_fileViaAstIn` (L41), `_fileViaSourceFileOut` (L45), `_methodViaAstOut` (L49), `_namespaceViaRefOut` (L53), `_typeDeclViaAstOut` (L57), `astIn` (L59), `astOut` (L61), `refOut` (L63), `sourceFileOut` (L65). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/NamespaceBlock.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c2b103b125ea4369"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Return.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c515bb8980940449100581ea0920e24bc13d8ebf`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Return.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bb3514f6afadbf7f00.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Return.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Return.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f21d09f87b3cb063e95e07d7f7f2fef2b6e346198057f5ce547e5c9bdb1fbc9b`.

> 
> final class AccessNeighborsForReturn(val node: nodes.Return) extends AnyVal {
> 
>   /** Traverse to BLOCK via ARGUMENT OUT edge.
>     */
>   def _blockViaArgumentOut: Option[nodes.Block] = argumentOut.collectAll[nodes.Block].nextOption()
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via AST OUT edge.
>     */
>   def _blockViaAstOut: Iterator[nodes.Block] = astOut.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 277 lexically identified declarations, including `AccessNeighborsForReturn` (L6), `_blockViaArgumentOut` (L10), `_blockViaAstIn` (L14), `_blockViaAstOut` (L18), `_blockViaCdgIn` (L22), `_blockViaDominateIn` (L26), `_blockViaDominateOut` (L30), `_blockViaPostDominateIn` (L34), `_blockViaPostDominateOut` (L38), `_blockViaReachingDefIn` (L42), `_callViaArgumentOut` (L46), `_callViaAstIn` (L50), `_callViaAstOut` (L54), `_callViaCdgIn` (L58), `_callViaDominateIn` (L62), `_callViaDominateOut` (L66), `_callViaPostDominateIn` (L70), `_callViaPostDominateOut` (L74), `_callViaReachingDefIn` (L78), `_controlStructureViaArgumentOut` (L82), `_controlStructureViaAstIn` (L86), `_controlStructureViaAstOut` (L90), `_controlStructureViaCatchBodyIn` (L94), `_controlStructureViaCdgIn` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Return.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-712196c465967ac5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Tag.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `747874e1b1352a3edc577bd194b692ab1f57bba0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Tag.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d32bf79049e4048627.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Tag.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Tag.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `797342e481f84d70b42beaf69aa141832c1b1b2bab086a7bbece3dedff711c13`.

> 
> final class AccessNeighborsForTag(val node: nodes.Tag) extends AnyVal {
> 
>   /** Traverse to BLOCK via TAGGED_BY IN edge.
>     */
>   def _blockViaTaggedByIn: Iterator[nodes.Block] = taggedByIn.collectAll[nodes.Block]
> 
>   /** Traverse to CALL via TAGGED_BY IN edge.
>     */
>   def _callViaTaggedByIn: Iterator[nodes.Call] = taggedByIn.collectAll[nodes.Call]
> 
>   /** Traverse to CONTROL_STRUCTURE via TAGGED_BY IN edge.
>     */
>   def _controlStructureViaTaggedByIn: Iterator[nodes.ControlStructure] = taggedByIn.collectAll[nodes.ControlStructure]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 52 lexically identified declarations, including `AccessNeighborsForTag` (L6), `_blockViaTaggedByIn` (L10), `_callViaTaggedByIn` (L14), `_controlStructureViaTaggedByIn` (L18), `_fieldIdentifierViaTaggedByIn` (L22), `_fileViaTaggedByIn` (L26), `_identifierViaTaggedByIn` (L30), `_importViaTaggedByIn` (L34), `_jumpTargetViaTaggedByIn` (L38), `_literalViaTaggedByIn` (L42), `_localViaTaggedByIn` (L46), `_memberViaTaggedByIn` (L50), `_methodViaTaggedByIn` (L54), `_methodParameterInViaTaggedByIn` (L58), `_methodParameterOutViaTaggedByIn` (L63), `_methodRefViaTaggedByIn` (L68), `_methodReturnViaTaggedByIn` (L72), `_returnViaTaggedByIn` (L76), `_tagViaTaggedByIn` (L80), `_tagViaTaggedByOut` (L84), `_templateDomViaTaggedByIn` (L88), `_typeDeclViaTaggedByIn` (L92), `_typeRefViaTaggedByIn` (L96), `_unknownViaTaggedByIn` (L100). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Tag.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9b1f6c7837dfb266"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TemplateDom.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `76600ab2dbf243d3ea3d4d02d99c7fe3544fb289`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TemplateDom.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-dd81010011903fd9e8.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TemplateDom.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TemplateDom.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `49036d83bbe743fe4197166dc05a890a299656b295e0a8ebe636d59f898ef656`.

> 
> final class AccessNeighborsForTemplateDom(val node: nodes.TemplateDom) extends AnyVal {
> 
>   /** Traverse to EXPRESSION via ARGUMENT IN edge.
>     */
>   def _expressionViaArgumentIn: Iterator[nodes.Expression] = argumentIn.collectAll[nodes.Expression]
> 
>   /** Traverse to EXPRESSION via AST OUT edge.
>     */
>   def _expressionViaAstOut: Iterator[nodes.Expression] = astOut.collectAll[nodes.Expression]
> 
>   /** Traverse to EXPRESSION via REACHING_DEF OUT edge.
>     */
>   def _expressionViaReachingDefOut: Iterator[nodes.Expression] = reachingDefOut.collectAll[nodes.Expression]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 24 lexically identified declarations, including `AccessNeighborsForTemplateDom` (L6), `_expressionViaArgumentIn` (L10), `_expressionViaAstOut` (L14), `_expressionViaReachingDefOut` (L18), `_fileViaContainsIn` (L22), `_methodViaContainsIn` (L26), `_tagViaTaggedByOut` (L30), `argumentIn` (L32), `astOut` (L34), `containsIn` (L36), `reachingDefOut` (L38), `taggedByOut` (L40), `AccessNeighborsForTemplateDomTraversal` (L43), `_expressionViaArgumentIn` (L47), `_expressionViaAstOut` (L51), `_expressionViaReachingDefOut` (L55), `_fileViaContainsIn` (L59), `_methodViaContainsIn` (L63), `_tagViaTaggedByOut` (L67), `argumentIn` (L69), `astOut` (L71), `containsIn` (L73), `reachingDefOut` (L75), `taggedByOut` (L77). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TemplateDom.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fa8bddf27fb1d0fd"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Type.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `3da96504aa0d6a691a721bd5694be591843f642d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Type.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b2e73a4489ea44c586.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Type.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Type.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1aabe9047cb6ebeff4de39c266f5cd1ebb28ed4b9ec104239484c6c90f4f2a06`.

> 
> final class AccessNeighborsForType(val node: nodes.Type) extends AnyVal {
> 
>   /** Traverse to ARRAY_INITIALIZER via EVAL_TYPE IN edge.
>     */
>   def _arrayInitializerViaEvalTypeIn: Iterator[nodes.ArrayInitializer] = evalTypeIn.collectAll[nodes.ArrayInitializer]
> 
>   /** Traverse to BLOCK via EVAL_TYPE IN edge.
>     */
>   def _blockViaEvalTypeIn: Iterator[nodes.Block] = evalTypeIn.collectAll[nodes.Block]
> 
>   /** Traverse to CALL via EVAL_TYPE IN edge.
>     */
>   def _callViaEvalTypeIn: Iterator[nodes.Call] = evalTypeIn.collectAll[nodes.Call]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 56 lexically identified declarations, including `AccessNeighborsForType` (L6), `_arrayInitializerViaEvalTypeIn` (L10), `_blockViaEvalTypeIn` (L14), `_callViaEvalTypeIn` (L18), `_controlStructureViaEvalTypeIn` (L22), `_identifierViaEvalTypeIn` (L26), `_literalViaEvalTypeIn` (L30), `_localViaEvalTypeIn` (L34), `_memberViaEvalTypeIn` (L38), `_methodParameterInViaEvalTypeIn` (L42), `_methodParameterOutViaEvalTypeIn` (L47), `_methodRefViaEvalTypeIn` (L52), `_methodReturnViaEvalTypeIn` (L56), `_typeArgumentViaAstOut` (L60), `_typeArgumentViaRefIn` (L64), `_typeDeclViaInheritsFromIn` (L68), `_typeRefViaEvalTypeIn` (L72), `_unknownViaEvalTypeIn` (L76), `_typeDeclViaAliasOfIn` (L81), `aliasTypeDecl` (L85), `_typeDeclViaRefOut` (L90), `referencedTypeDecl` (L94), `aliasOfIn` (L96), `astOut` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Type.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f4b32ace2bca4241"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeArgument.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0bbc31743255d65a00272082cc14c28922c1f20d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeArgument.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3302b8fca62c39378d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeArgument.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeArgument.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `fd6b6411338c6441df565980a1b9da78942b628be430f68ba04d40ddade7ff90`.

> 
> final class AccessNeighborsForTypeArgument(val node: nodes.TypeArgument) extends AnyVal {
> 
>   /** Traverse to TYPE via AST IN edge.
>     */
>   def _typeViaAstIn: Iterator[nodes.Type] = astIn.collectAll[nodes.Type]
> 
>   /** Traverse to TYPE via REF OUT edge.
>     */
>   def _typeViaRefOut: Iterator[nodes.Type] = refOut.collectAll[nodes.Type]
> 
>   /** Traverse to TYPE_PARAMETER via BINDS_TO OUT edge.
>     */
>   def _typeParameterViaBindsToOut: Iterator[nodes.TypeParameter] = bindsToOut.collectAll[nodes.TypeParameter]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 14 lexically identified declarations, including `AccessNeighborsForTypeArgument` (L6), `_typeViaAstIn` (L10), `_typeViaRefOut` (L14), `_typeParameterViaBindsToOut` (L18), `astIn` (L20), `bindsToOut` (L22), `refOut` (L24), `AccessNeighborsForTypeArgumentTraversal` (L27), `_typeViaAstIn` (L31), `_typeViaRefOut` (L35), `_typeParameterViaBindsToOut` (L39), `astIn` (L41), `bindsToOut` (L43), `refOut` (L45). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeArgument.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-12f725901f79c557"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeDecl.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `778ba935ce0ae96bd32d15605b22d041bb38d6e7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeDecl.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-64fbffe9a6110527a3.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeDecl.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeDecl.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2bc6a0e954a65dbdfcae6843484822250a2a1b0cc3940a68af68a2b0968df9b4`.

> 
> final class AccessNeighborsForTypeDecl(val node: nodes.TypeDecl) extends AnyVal {
> 
>   /** Traverse to ANNOTATION via AST OUT edge.
>     */
>   def _annotationViaAstOut: Iterator[nodes.Annotation] = astOut.collectAll[nodes.Annotation]
> 
>   /** Traverse to BINDING via BINDS OUT edge.
>     */
>   def _bindingViaBindsOut: Iterator[nodes.Binding] = bindsOut.collectAll[nodes.Binding]
> 
>   /** Traverse to FILE via CONTAINS IN edge.
>     */
>   def _fileViaContainsIn: Iterator[nodes.File] = containsIn.collectAll[nodes.File]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 62 lexically identified declarations, including `AccessNeighborsForTypeDecl` (L6), `_annotationViaAstOut` (L10), `_bindingViaBindsOut` (L14), `_fileViaContainsIn` (L18), `_fileViaSourceFileOut` (L22), `_importViaAstOut` (L26), `_memberViaAstOut` (L30), `_methodViaAstIn` (L34), `_methodViaAstOut` (L38), `_methodViaContainsOut` (L42), `_modifierViaAstOut` (L46), `_namespaceBlockViaAstIn` (L51), `namespaceBlock` (L55), `_tagViaTaggedByOut` (L59), `_typeViaAliasOfOut` (L64), `aliasedType` (L68), `_typeViaInheritsFromOut` (L72), `_typeViaRefIn` (L76), `_typeDeclViaAstIn` (L80), `_typeDeclViaAstOut` (L84), `_typeParameterViaAstOut` (L88), `aliasOfOut` (L90), `astIn` (L92), `astOut` (L94). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeDecl.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b0da51bd102162bc"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeParameter.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8caffc569e1368ff7008b9afb033d1def92d59f3`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeParameter.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2a44ef3db3f3776982.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeParameter.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeParameter.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0cfeefa845290d31ff563b5538805f8df52baa8a1c53ae14d0ae58cfd3d7160a`.

> 
> final class AccessNeighborsForTypeParameter(val node: nodes.TypeParameter) extends AnyVal {
> 
>   /** Traverse to METHOD via AST IN edge.
>     */
>   def _methodViaAstIn: nodes.Method = {
>     try { astIn.collectAll[nodes.Method].next() }
>     catch {
>       case e: java.util.NoSuchElementException =>
>         val nodeInfo = String.format("id=%d, (seq=%d)", node.id, node.seq)
>         throw new flatgraph.SchemaViolationException(
>           "IN edge with label AST to an adjacent METHOD is mandatory, but not defined for this TYPE_PARAMETER node with " + nodeInfo,
>           e
>         )
>     }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `AccessNeighborsForTypeParameter` (L6), `_methodViaAstIn` (L10), `nodeInfo` (L14), `_typeArgumentViaBindsToIn` (L24), `_typeDeclViaAstIn` (L28), `astIn` (L30), `bindsToIn` (L32), `AccessNeighborsForTypeParameterTraversal` (L35), `_methodViaAstIn` (L39), `_typeArgumentViaBindsToIn` (L43), `_typeDeclViaAstIn` (L47), `astIn` (L49), `bindsToIn` (L51). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeParameter.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d66cbc5a1615cace"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeRef.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `a3b2be3cee52f0f578526d02308dcf66110e8b9f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeRef.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f57b70985fb2d375f5.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeRef.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeRef.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `05f0904233ab17d298b573c5b678e67073ad262b0f8fdcb1e38898adc8799cc2`.

> 
> final class AccessNeighborsForTypeRef(val node: nodes.TypeRef) extends AnyVal {
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via CDG IN edge.
>     */
>   def _blockViaCdgIn: Iterator[nodes.Block] = cdgIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via CDG OUT edge.
>     */
>   def _blockViaCdgOut: Iterator[nodes.Block] = cdgOut.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 278 lexically identified declarations, including `AccessNeighborsForTypeRef` (L6), `_blockViaAstIn` (L10), `_blockViaCdgIn` (L14), `_blockViaCdgOut` (L18), `_blockViaDominateIn` (L22), `_blockViaDominateOut` (L26), `_blockViaPostDominateIn` (L30), `_blockViaPostDominateOut` (L34), `_blockViaReachingDefIn` (L38), `_callViaArgumentIn` (L42), `_callViaAstIn` (L46), `_callViaCdgIn` (L50), `_callViaCdgOut` (L54), `_callViaDominateIn` (L58), `_callViaDominateOut` (L62), `_callViaPostDominateIn` (L66), `_callViaPostDominateOut` (L70), `_callViaReachingDefIn` (L74), `_callViaReachingDefOut` (L78), `_callViaReceiverIn` (L82), `_cfgNodeViaCfgOut` (L86), `_closureBindingViaCaptureOut` (L90), `_controlStructureViaAstIn` (L94), `_controlStructureViaCatchBodyIn` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/TypeRef.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0c55ae36aa4466d3"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Unknown.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5261e5546a330aa3658a919537a4b3309217009d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Unknown.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f4176619fe5c4a407f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Unknown.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Unknown.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3d72130ae6e328cbf2ebbe61523b56bb41378b5e16345cfe5382710087bba08b`.

> 
> final class AccessNeighborsForUnknown(val node: nodes.Unknown) extends AnyVal {
> 
>   /** Traverse to ANNOTATION via AST OUT edge.
>     */
>   def _annotationViaAstOut: Iterator[nodes.Annotation] = astOut.collectAll[nodes.Annotation]
> 
>   /** Traverse to BLOCK via AST IN edge.
>     */
>   def _blockViaAstIn: Iterator[nodes.Block] = astIn.collectAll[nodes.Block]
> 
>   /** Traverse to BLOCK via AST OUT edge.
>     */
>   def _blockViaAstOut: Iterator[nodes.Block] = astOut.collectAll[nodes.Block]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 276 lexically identified declarations, including `AccessNeighborsForUnknown` (L6), `_annotationViaAstOut` (L10), `_blockViaAstIn` (L14), `_blockViaAstOut` (L18), `_blockViaCdgIn` (L22), `_blockViaCdgOut` (L26), `_blockViaDominateIn` (L30), `_blockViaDominateOut` (L34), `_blockViaPostDominateIn` (L38), `_blockViaPostDominateOut` (L42), `_callViaArgumentIn` (L46), `_callViaAstOut` (L50), `_callViaCdgIn` (L54), `_callViaCdgOut` (L58), `_callViaDominateIn` (L62), `_callViaDominateOut` (L66), `_callViaPostDominateIn` (L70), `_callViaPostDominateOut` (L74), `_callViaReachingDefOut` (L78), `_callViaReceiverIn` (L82), `_cfgNodeViaCfgOut` (L86), `_controlStructureViaAstIn` (L90), `_controlStructureViaAstOut` (L94), `_controlStructureViaCatchBodyIn` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/Unknown.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f3ce4703d89f9435"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/package.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `95fb803d89e6891d05eb06ac194abc3e260eb71e`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/package.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e1fc0859acb656a2ad.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/package.scala, lines 7–21](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/package.scala#L7-L21). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `25e419186fd6bb1374db1277998fc5dcb510ca670c380c40620bd5e9a80ce23c`.

>   object Lang extends Conversions
> 
>   trait Conversions {
>     implicit def accessNeighborsForAnnotation(node: nodes.Annotation): AccessNeighborsForAnnotation =
>       new AccessNeighborsForAnnotation(node)
> 
>     implicit def accessNeighborsForAnnotationTraversal(
>       traversal: IterableOnce[nodes.Annotation]
>     ): AccessNeighborsForAnnotationTraversal =
>       new AccessNeighborsForAnnotationTraversal(traversal.iterator)
> 
>     implicit def accessNeighborsForAnnotationLiteral(
>       node: nodes.AnnotationLiteral
>     ): AccessNeighborsForAnnotationLiteral =
>       new AccessNeighborsForAnnotationLiteral(node)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 82 lexically identified declarations, including `Lang` (L7), `Conversions` (L9), `accessNeighborsForAnnotation` (L10), `accessNeighborsForAnnotationTraversal` (L13), `accessNeighborsForAnnotationLiteral` (L18), `accessNeighborsForAnnotationLiteralTraversal` (L23), `accessNeighborsForAnnotationParameter` (L28), `accessNeighborsForAnnotationParameterTraversal` (L33), `accessNeighborsForAnnotationParameterAssign` (L38), `accessNeighborsForAnnotationParameterAssignTraversal` (L43), `accessNeighborsForArrayInitializer` (L48), `accessNeighborsForArrayInitializerTraversal` (L51), `accessNeighborsForBinding` (L56), `accessNeighborsForBindingTraversal` (L59), `accessNeighborsForBlock` (L64), `accessNeighborsForBlockTraversal` (L67), `accessNeighborsForCall` (L72), `accessNeighborsForCallTraversal` (L75), `accessNeighborsForClosureBinding` (L78), `accessNeighborsForClosureBindingTraversal` (L81), `accessNeighborsForComment` (L86), `accessNeighborsForCommentTraversal` (L89), `accessNeighborsForControlStructure` (L94), `accessNeighborsForControlStructureTraversal` (L97). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/neighboraccessors/package.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes` -> module `domainClasses::src::main::generated::io::shiftleft::codepropertygraph::generated::nodes`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c29272595b076bd119.** [cpg/README.md, lines 50–61](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L50-L61). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Domain class generation
> 
> The Scala domain classes under `domainClasses/src/main/generated/` are generated from the schema defined in `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/`. They are checked into the repository, so you only need to regenerate them when the schema changes.
> 
> Generation is handled by the `sbt-flatgraph` plugin via the `generateDomainClasses` task in the `schema` subproject. It reads `CpgSchema.instance`, which composes all schema modules, and writes Scala source files to `domainClasses/src/main/generated/`.
> 
> The task is wired as a dependency of `domainClasses/compile` and `stage`, so it runs automatically when you compile or stage:
> 
> ```
> sbt compile   # triggers generateDomainClasses, then compiles
> sbt stage     # triggers generateDomainClasses, then packages
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve CPG functional and interchange contracts and exact module/file spelling; implement initialization and visibility with native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic and file excerpts establish CPG functional obligations. Rust libraries provide native implementation; no Go runtime is required.


<a id="file-a61afd8b823867d4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Annotation.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `29e0009f75d59c8c2115fcdb0cdb0e87780ca1f7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Annotation.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4e9f909fb37f8f3366.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Annotation.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Annotation.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `440f833636a55069784fc2e8dcf777bcda2eec90a762a0550a87b08de48ea81d`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait AnnotationEMT extends AnyRef with ExpressionEMT with HasFullNameEMT with HasNameEMT
> 
> trait AnnotationBase extends AbstractNode with ExpressionBase with StaticType[AnnotationEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if ((-1: Int) != this.argumentIndex) res.put("ARGUMENT_INDEX", this.argumentIndex)
>     this.argumentLabel.foreach { p => res.put("ARGUMENT_LABEL", p) }
>     this.argumentName.foreach { p => res.put("ARGUMENT_NAME", p) }
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `AnnotationEMT` (L10), `AnnotationBase` (L12), `propertiesMap` (L14), `res` (L16), `Annotation` (L32), `Label` (L33), `unapply` (L34), `Annotation` (L100), `productElementName` (L106), `productElement` (L122), `productPrefix` (L138), `productArity` (L139), `canEqual` (L141). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Annotation.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bb3d5a01e081a8c9"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationLiteral.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `3ca42f4b198288b7823c208855a9fc56e963afdd`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationLiteral.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d3b9350cb77e8bf723.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationLiteral.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationLiteral.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1d6bbf48adc754a798a6f1514d380b5c88a902bcfba5a472bba5824afeb1ac25`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait AnnotationLiteralEMT extends AnyRef with ExpressionEMT with HasNameEMT
> 
> trait AnnotationLiteralBase extends AbstractNode with ExpressionBase with StaticType[AnnotationLiteralEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if ((-1: Int) != this.argumentIndex) res.put("ARGUMENT_INDEX", this.argumentIndex)
>     this.argumentLabel.foreach { p => res.put("ARGUMENT_LABEL", p) }
>     this.argumentName.foreach { p => res.put("ARGUMENT_NAME", p) }
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `AnnotationLiteralEMT` (L10), `AnnotationLiteralBase` (L12), `propertiesMap` (L14), `res` (L16), `AnnotationLiteral` (L31), `Label` (L32), `unapply` (L33), `AnnotationLiteral` (L94), `productElementName` (L100), `productElement` (L115), `productPrefix` (L130), `productArity` (L131), `canEqual` (L133). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationLiteral.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7d51388123a29c1c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameter.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `01971461504037afe536747207755050671f1814`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameter.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e2587cb99e2a27fba7.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameter.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameter.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9c1e962391e2fa00c0981c926e421c95304b8e27d71caaf13826aad6aa6de82d`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait AnnotationParameterEMT extends AnyRef with AstNodeEMT
> 
> trait AnnotationParameterBase extends AbstractNode with AstNodeBase with StaticType[AnnotationParameterEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     this.lineNumber.foreach { p => res.put("LINE_NUMBER", p) }
>     this.offset.foreach { p => res.put("OFFSET", p) }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `AnnotationParameterEMT` (L10), `AnnotationParameterBase` (L12), `propertiesMap` (L14), `res` (L16), `AnnotationParameter` (L27), `Label` (L28), `unapply` (L29), `AnnotationParameter` (L63), `productElementName` (L69), `productElement` (L80), `productPrefix` (L91), `productArity` (L92), `canEqual` (L94). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameter.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b25af2791e36901d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameterAssign.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `47f0fc44b2bc8fbe57d3d7816457778b008a1e1d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameterAssign.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-003603fdf95c226983.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameterAssign.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameterAssign.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `02a5b57cad8e6767572d39863cea152a216eee300b9fe5f83b9a282f8faecbff`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait AnnotationParameterAssignEMT extends AnyRef with AstNodeEMT
> 
> trait AnnotationParameterAssignBase
>     extends AbstractNode
>     with AstNodeBase
>     with StaticType[AnnotationParameterAssignEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `AnnotationParameterAssignEMT` (L10), `AnnotationParameterAssignBase` (L12), `propertiesMap` (L17), `res` (L19), `AnnotationParameterAssign` (L30), `Label` (L31), `unapply` (L32), `AnnotationParameterAssign` (L66), `productElementName` (L72), `productElement` (L83), `productPrefix` (L94), `productArity` (L95), `canEqual` (L97). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/AnnotationParameterAssign.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-54ed47f0d84fc47c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ArrayInitializer.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `2018e25fec31d73072cd40fb7323b1c68f5b6872`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ArrayInitializer.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4b46f6e34c2a7fbfab.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ArrayInitializer.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ArrayInitializer.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `42821004187903f70a59d7917796272b30443462d0e8e4c658c937b51410dfc8`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait ArrayInitializerEMT extends AnyRef with ExpressionEMT
> 
> trait ArrayInitializerBase extends AbstractNode with ExpressionBase with StaticType[ArrayInitializerEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if ((-1: Int) != this.argumentIndex) res.put("ARGUMENT_INDEX", this.argumentIndex)
>     this.argumentLabel.foreach { p => res.put("ARGUMENT_LABEL", p) }
>     this.argumentName.foreach { p => res.put("ARGUMENT_NAME", p) }
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `ArrayInitializerEMT` (L10), `ArrayInitializerBase` (L12), `propertiesMap` (L14), `res` (L16), `ArrayInitializer` (L30), `Label` (L31), `unapply` (L32), `ArrayInitializer` (L89), `productElementName` (L95), `productElement` (L109), `productPrefix` (L123), `productArity` (L124), `canEqual` (L126). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ArrayInitializer.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a96378ca686e50d7"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/BaseTypes.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e8cf3b6c13ab1ff0c22f8531414acc91a70de2b0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/BaseTypes.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-16ad8be38786cdac54.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/BaseTypes.scala, lines 2–16](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/BaseTypes.scala#L2-L16). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `af35d193146d5b76e80576b9dd5ed7c467774b0edc3d2d705ddddc19b07392de`.

> 
> trait AstNodeEMT
>     extends AnyRef
>     with HasCodeEMT
>     with HasColumnNumberEMT
>     with HasLineNumberEMT
>     with HasOffsetEMT
>     with HasOffsetEndEMT
>     with HasOrderEMT
> 
> trait AstNodeBase extends AbstractNode with StaticType[AstNodeEMT]
> // new properties: CODE, COLUMN_NUMBER, LINE_NUMBER, OFFSET, OFFSET_END, ORDER
> // inherited properties:
> // inherited interfaces:
> // implementing nodes: ANNOTATION, ANNOTATION_LITERAL, ANNOTATION_PARAMETER, ANNOTATION_PARAMETER_ASSIGN, ARRAY_INITIALIZER, BLOCK, CALL, COMMENT, CONTROL_STRUCTURE, FIELD_IDENTIFIER, FILE, IDENTIFIER, IMPORT, JUMP_LABEL, JUMP_TARGET, LITERAL, LOCAL, MEMBER, METHOD, METHOD_PARAMETER_IN, METHOD_PARAMETER_OUT, METHOD_REF, METHOD_RETURN, MODIFIER, NAMESPACE, NAMESPACE_BLOCK, RETURN, TEMPLATE_DOM, TYPE_ARGUMENT, TYPE_DECL, TYPE_PARAMETER, TYPE_REF, UNKNOWN


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 115 lexically identified declarations, including `AstNodeEMT` (L3), `AstNodeBase` (L12), `AstNode` (L17), `AstNodeNew` (L19), `code` (L20), `code_` (L21), `code` (L22), `columnNumber` (L23), `columnNumber_` (L24), `columnNumber` (L25), `columnNumber` (L26), `lineNumber` (L27), `lineNumber_` (L28), `lineNumber` (L29), `lineNumber` (L30), `offset` (L31), `offset_` (L32), `offset` (L33), `offset` (L34), `offsetEnd` (L35), `offsetEnd_` (L36), `offsetEnd` (L37), `offsetEnd` (L38), `order` (L39). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/BaseTypes.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-05f1fb232b2c922e"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Binding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `061d634a7f077cfbc148399032b0389338ded46a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Binding.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-fc2bae403a5eecc6e0.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Binding.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Binding.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `4dc4a8fe7fea720a291fd40081dc6c841926616964f0018bb8211ebce0758f10`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait BindingEMT extends AnyRef with HasMethodFullNameEMT with HasNameEMT with HasSignatureEMT
> 
> trait BindingBase extends AbstractNode with StaticType[BindingEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.methodFullName) res.put("METHOD_FULL_NAME", this.methodFullName)
>     if (("<empty>": String) != this.name) res.put("NAME", this.name)
>     if (("": String) != this.signature) res.put("SIGNATURE", this.signature)
>     res


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `BindingEMT` (L10), `BindingBase` (L12), `propertiesMap` (L14), `res` (L16), `Binding` (L24), `Label` (L25), `unapply` (L26), `Binding` (L43), `productElementName` (L48), `productElement` (L56), `productPrefix` (L64), `productArity` (L65), `canEqual` (L67). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Binding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c4384b87abd745f7"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Block.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `430f7f3e9b137d80abcf87fac531937d193c2721`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Block.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-6b32b796c50f2a0f20.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Block.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Block.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1eb13a0a615a818028c2b20f5ad476c9c1fcac21109718d0b8b54a123941e1bd`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait BlockEMT
>     extends AnyRef
>     with ExpressionEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT
> 
> trait BlockBase extends AbstractNode with ExpressionBase with StaticType[BlockEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `BlockEMT` (L10), `BlockBase` (L17), `propertiesMap` (L19), `res` (L21), `tmpDynamicTypeHintFullName` (L27), `tmpPossibleTypes` (L33), `Block` (L40), `Label` (L41), `unapply` (L42), `Block` (L114), `productElementName` (L120), `productElement` (L137), `productPrefix` (L154), `productArity` (L155), `canEqual` (L157). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Block.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3b84c6b826fc37bf"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Call.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8e8e430ee5c2e6ae33b5d1c8b53feea84fdf5855`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Call.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-5778f62daa3d2cc537.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Call.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Call.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `582c05734e36b646b0ab343343c96ac198983b36b2835bb3e271f648ce946bfb`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait CallEMT
>     extends AnyRef
>     with CallReprEMT
>     with ExpressionEMT
>     with HasDispatchTypeEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasMethodFullNameEMT
>     with HasPossibleTypesEMT
>     with HasStaticReceiverEMT
>     with HasTypeFullNameEMT


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `CallEMT` (L10), `CallBase` (L21), `propertiesMap` (L23), `res` (L25), `tmpDynamicTypeHintFullName` (L32), `tmpPossibleTypes` (L40), `Call` (L49), `Label` (L50), `unapply` (L51), `Call` (L151), `productElementName` (L158), `productElement` (L180), `productPrefix` (L202), `productArity` (L203), `canEqual` (L205). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Call.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9f2dad560d3e65bb"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ClosureBinding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f3b5a3b577814d7674ad0633dd8cd73e55f1a1aa`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ClosureBinding.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-cfda2686161328749f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ClosureBinding.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ClosureBinding.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `4cc15a203cae4c233c254083e3ef065a8e539183746f2e3e69561c84385234e4`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait ClosureBindingEMT extends AnyRef with HasClosureBindingIdEMT with HasEvaluationStrategyEMT
> 
> trait ClosureBindingBase extends AbstractNode with StaticType[ClosureBindingEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     this.closureBindingId.foreach { p => res.put("CLOSURE_BINDING_ID", p) }
>     if (("<empty>": String) != this.evaluationStrategy) res.put("EVALUATION_STRATEGY", this.evaluationStrategy)
>     res
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `ClosureBindingEMT` (L10), `ClosureBindingBase` (L12), `propertiesMap` (L14), `res` (L16), `ClosureBinding` (L23), `Label` (L24), `unapply` (L25), `ClosureBinding` (L40), `productElementName` (L45), `productElement` (L52), `productPrefix` (L59), `productArity` (L60), `canEqual` (L62). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ClosureBinding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-405fe7b87752010c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Comment.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5307e1166f2f4b2e705b2e928ca23d1c0b57cebf`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Comment.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-1017be51ed0657a653.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Comment.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Comment.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `277b35b923bfb1a5781b207638d568852d3585fd1bcb787303c72066e66d68f9`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait CommentEMT extends AnyRef with AstNodeEMT with HasFilenameEMT
> 
> trait CommentBase extends AbstractNode with AstNodeBase with StaticType[CommentEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     if (("<empty>": String) != this.filename) res.put("FILENAME", this.filename)
>     this.lineNumber.foreach { p => res.put("LINE_NUMBER", p) }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `CommentEMT` (L10), `CommentBase` (L12), `propertiesMap` (L14), `res` (L16), `Comment` (L28), `Label` (L29), `unapply` (L30), `Comment` (L71), `productElementName` (L77), `productElement` (L89), `productPrefix` (L101), `productArity` (L102), `canEqual` (L104). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Comment.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e812e8ea4c5e5fba"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ConfigFile.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8e1453f7f874d1dc19678437897e5f28d3726abf`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ConfigFile.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-286588e4913ebb82fa.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ConfigFile.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ConfigFile.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dba31944e7c17411eb444379a7829f0cad7fe25500dc37df6b854a32cfab05be`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait ConfigFileEMT extends AnyRef with HasContentEMT with HasNameEMT
> 
> trait ConfigFileBase extends AbstractNode with StaticType[ConfigFileEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.content) res.put("CONTENT", this.content)
>     if (("<empty>": String) != this.name) res.put("NAME", this.name)
>     res
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `ConfigFileEMT` (L10), `ConfigFileBase` (L12), `propertiesMap` (L14), `res` (L16), `ConfigFile` (L23), `Label` (L24), `unapply` (L25), `ConfigFile` (L36), `productElementName` (L41), `productElement` (L48), `productPrefix` (L55), `productArity` (L56), `canEqual` (L58). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ConfigFile.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c4ca3f9bb1daac8b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ControlStructure.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9bf69b3045e630a054624b9acbc4cec3f16224dd`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ControlStructure.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-149ee170b67e024944.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ControlStructure.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ControlStructure.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7dbc86f165311cbc5e9d84033ec118c7b71d08aeb65f14e8c161a9351bfc0c96`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait ControlStructureEMT extends AnyRef with ExpressionEMT with HasControlStructureTypeEMT with HasParserTypeNameEMT
> 
> trait ControlStructureBase extends AbstractNode with ExpressionBase with StaticType[ControlStructureEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if ((-1: Int) != this.argumentIndex) res.put("ARGUMENT_INDEX", this.argumentIndex)
>     this.argumentLabel.foreach { p => res.put("ARGUMENT_LABEL", p) }
>     this.argumentName.foreach { p => res.put("ARGUMENT_NAME", p) }
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `ControlStructureEMT` (L10), `ControlStructureBase` (L12), `propertiesMap` (L14), `res` (L16), `ControlStructure` (L32), `Label` (L33), `unapply` (L34), `ControlStructure` (L100), `productElementName` (L106), `productElement` (L122), `productPrefix` (L138), `productArity` (L139), `canEqual` (L141). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/ControlStructure.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-18d4808e74af86f4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Dependency.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7c5061636ed78f21f9dc8f05298543dd9a9842f3`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Dependency.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-259407c2e30c2edcf5.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Dependency.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Dependency.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9f1612d9310105e7e857036f9f434ba8bc954afaffd378af10f279f4d396dae5`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait DependencyEMT extends AnyRef with HasDependencyGroupIdEMT with HasNameEMT with HasVersionEMT
> 
> trait DependencyBase extends AbstractNode with StaticType[DependencyEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     this.dependencyGroupId.foreach { p => res.put("DEPENDENCY_GROUP_ID", p) }
>     if (("<empty>": String) != this.name) res.put("NAME", this.name)
>     if (("<empty>": String) != this.version) res.put("VERSION", this.version)
>     res


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `DependencyEMT` (L10), `DependencyBase` (L12), `propertiesMap` (L14), `res` (L16), `Dependency` (L24), `Label` (L25), `unapply` (L26), `Dependency` (L40), `productElementName` (L45), `productElement` (L53), `productPrefix` (L61), `productArity` (L62), `canEqual` (L64). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Dependency.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fb114914e87fc457"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/FieldIdentifier.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7288596c143a8bc45b5b5f0ea0427b1eb403b8be`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/FieldIdentifier.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-743dbf87a4b0e40347.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/FieldIdentifier.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/FieldIdentifier.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `964690b90357eceecdfd7e7d72496ea18a71999c64f38c8d6d63488bd726c627`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait FieldIdentifierEMT extends AnyRef with ExpressionEMT with HasCanonicalNameEMT
> 
> trait FieldIdentifierBase extends AbstractNode with ExpressionBase with StaticType[FieldIdentifierEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if ((-1: Int) != this.argumentIndex) res.put("ARGUMENT_INDEX", this.argumentIndex)
>     this.argumentLabel.foreach { p => res.put("ARGUMENT_LABEL", p) }
>     this.argumentName.foreach { p => res.put("ARGUMENT_NAME", p) }
>     if (("<empty>": String) != this.canonicalName) res.put("CANONICAL_NAME", this.canonicalName)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `FieldIdentifierEMT` (L10), `FieldIdentifierBase` (L12), `propertiesMap` (L14), `res` (L16), `FieldIdentifier` (L31), `Label` (L32), `unapply` (L33), `FieldIdentifier` (L98), `productElementName` (L104), `productElement` (L119), `productPrefix` (L134), `productArity` (L135), `canEqual` (L137). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/FieldIdentifier.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-26f99f8d3fd92763"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/File.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f3cc7409cc10aea73a732baa6d572589a0a43ea9`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/File.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a1c50a7113ba89c223.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/File.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/File.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1c987a57c9deba3639807cee689104bdea97e22034784242762b15dabeb5deeb`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait FileEMT extends AnyRef with AstNodeEMT with HasContentEMT with HasHashEMT with HasNameEMT
> 
> trait FileBase extends AbstractNode with AstNodeBase with StaticType[FileEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     if (("<empty>": String) != this.content) res.put("CONTENT", this.content)
>     this.hash.foreach { p => res.put("HASH", p) }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `FileEMT` (L10), `FileBase` (L12), `propertiesMap` (L14), `res` (L16), `File` (L30), `Label` (L31), `unapply` (L32), `File` (L80), `productElementName` (L86), `productElement` (L100), `productPrefix` (L114), `productArity` (L115), `canEqual` (L117). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/File.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c2d065542a80ccb7"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Finding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `feca5eee800ad040c2086efc7969ed9bb635b092`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Finding.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0eade8922ba5c4241f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Finding.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Finding.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `4247d9f0bd37542c3ffe032f257c46c1acb4081a49b2a9017b713818a33f2cb8`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait FindingEMT extends AnyRef with HasEvidenceDescriptionEMT
> 
> trait FindingBase extends AbstractNode with StaticType[FindingEMT] {
>   def evidence: IndexedSeq[AbstractNode]
>   def keyValuePairs: IndexedSeq[KeyValuePairBase]
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res                    = new java.util.HashMap[String, Any]()
>     val tmpEvidenceDescription = this.evidenceDescription;
>     if (tmpEvidenceDescription.nonEmpty) res.put("EVIDENCE_DESCRIPTION", tmpEvidenceDescription)
>     val tmpevidence      = this.evidence; if (tmpevidence.nonEmpty) res.put("evidence", tmpevidence)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `FindingEMT` (L10), `FindingBase` (L12), `evidence` (L13), `keyValuePairs` (L14), `propertiesMap` (L15), `res` (L17), `tmpEvidenceDescription` (L18), `tmpevidence` (L20), `tmpkeyValuePairs` (L21), `Finding` (L26), `Label` (L27), `unapply` (L28), `Finding` (L44), `evidence` (L48), `keyValuePairs` (L50), `productElementName` (L53), `productElement` (L61), `productPrefix` (L69), `productArity` (L70), `canEqual` (L72). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Finding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fe010b7a838713a9"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Identifier.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `eba854d5bd8963318e6445fc62652ea65ea9c201`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Identifier.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b6332d45203c05b882.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Identifier.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Identifier.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `62e83777734e1b538bd4bbf93a2c850aebe788bb2f6ff88069340f82a8217214`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait IdentifierEMT
>     extends AnyRef
>     with ExpressionEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasNameEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT
> 
> trait IdentifierBase extends AbstractNode with ExpressionBase with StaticType[IdentifierEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `IdentifierEMT` (L10), `IdentifierBase` (L18), `propertiesMap` (L20), `res` (L22), `tmpDynamicTypeHintFullName` (L28), `tmpPossibleTypes` (L35), `Identifier` (L42), `Label` (L43), `unapply` (L44), `Identifier` (L120), `productElementName` (L126), `productElement` (L144), `productPrefix` (L162), `productArity` (L163), `canEqual` (L165). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Identifier.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-525027b8a6c207d4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Import.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `85af9ca3762cd71ad6feb4807ca0bba6e1def53c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Import.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-d3944aee3f3252d796.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Import.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Import.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6c5be19f2ba1a65fe1d0ea1f129d664df503b712c9a2d19317323b921c9690d8`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait ImportEMT
>     extends AnyRef
>     with AstNodeEMT
>     with HasExplicitAsEMT
>     with HasImportedAsEMT
>     with HasImportedEntityEMT
>     with HasIsExplicitEMT
>     with HasIsModuleImportEMT
>     with HasIsWildcardEMT
> 
> trait ImportBase extends AbstractNode with AstNodeBase with StaticType[ImportEMT] {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `ImportEMT` (L10), `ImportBase` (L20), `propertiesMap` (L22), `res` (L24), `Import` (L41), `Label` (L42), `unapply` (L43), `Import` (L107), `productElementName` (L113), `productElement` (L130), `productPrefix` (L147), `productArity` (L148), `canEqual` (L150). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Import.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e5370527e105a780"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpLabel.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `48017758aafb5eac0e956431b0512551eae4f472`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpLabel.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0335fde40d008f5411.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpLabel.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpLabel.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b6e8bbf432089b65450b3fd4a8f92410d48f089a8fa219fe0b68ebf9e4179163`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait JumpLabelEMT extends AnyRef with AstNodeEMT with HasNameEMT with HasParserTypeNameEMT
> 
> trait JumpLabelBase extends AbstractNode with AstNodeBase with StaticType[JumpLabelEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     this.lineNumber.foreach { p => res.put("LINE_NUMBER", p) }
>     if (("<empty>": String) != this.name) res.put("NAME", this.name)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `JumpLabelEMT` (L10), `JumpLabelBase` (L12), `propertiesMap` (L14), `res` (L16), `JumpLabel` (L29), `Label` (L30), `unapply` (L31), `JumpLabel` (L73), `productElementName` (L79), `productElement` (L92), `productPrefix` (L105), `productArity` (L106), `canEqual` (L108). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpLabel.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-39590600706147e5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpTarget.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7108e75704e6404deb9ccf6ff70f30f3c0d8c93a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpTarget.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e014f611296238ca5b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpTarget.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpTarget.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5678e1fbe2d15c706868713b9b44a71263af41f86ef166dc201a08602fb28a5d`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait JumpTargetEMT extends AnyRef with CfgNodeEMT with HasArgumentIndexEMT with HasNameEMT with HasParserTypeNameEMT
> 
> trait JumpTargetBase extends AbstractNode with CfgNodeBase with StaticType[JumpTargetEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if ((-1: Int) != this.argumentIndex) res.put("ARGUMENT_INDEX", this.argumentIndex)
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     this.lineNumber.foreach { p => res.put("LINE_NUMBER", p) }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `JumpTargetEMT` (L10), `JumpTargetBase` (L12), `propertiesMap` (L14), `res` (L16), `JumpTarget` (L30), `Label` (L31), `unapply` (L32), `JumpTarget` (L82), `productElementName` (L88), `productElement` (L102), `productPrefix` (L116), `productArity` (L117), `canEqual` (L119). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/JumpTarget.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-40ce52846ddf411f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/KeyValuePair.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `fdf527775625ca899dfd6139065043733e700e61`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/KeyValuePair.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-eb6f8377d6134bf808.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/KeyValuePair.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/KeyValuePair.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7b5d4e3dc4affa8c23f4b2e940928d50605e2d0574471fc9dd4eaf316e647018`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait KeyValuePairEMT extends AnyRef with HasKeyEMT with HasValueEMT
> 
> trait KeyValuePairBase extends AbstractNode with StaticType[KeyValuePairEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.key) res.put("KEY", this.key)
>     if (("": String) != this.value) res.put("VALUE", this.value)
>     res
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `KeyValuePairEMT` (L10), `KeyValuePairBase` (L12), `propertiesMap` (L14), `res` (L16), `KeyValuePair` (L23), `Label` (L24), `unapply` (L25), `KeyValuePair` (L36), `productElementName` (L41), `productElement` (L48), `productPrefix` (L55), `productArity` (L56), `canEqual` (L58). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/KeyValuePair.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d257727df0ba6975"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Literal.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `24254961e1bb189c32316ffb7470c7c3788f9e0e`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Literal.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4d47052d7411fd3373.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Literal.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Literal.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f54c44fb737364d11d6e67f1dab65732982e2d7eba06d9641e728200aba32242`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait LiteralEMT
>     extends AnyRef
>     with ExpressionEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT
> 
> trait LiteralBase extends AbstractNode with ExpressionBase with StaticType[LiteralEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `LiteralEMT` (L10), `LiteralBase` (L17), `propertiesMap` (L19), `res` (L21), `tmpDynamicTypeHintFullName` (L27), `tmpPossibleTypes` (L33), `Literal` (L40), `Label` (L41), `unapply` (L42), `Literal` (L114), `productElementName` (L120), `productElement` (L137), `productPrefix` (L154), `productArity` (L155), `canEqual` (L157). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Literal.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ea9a590edf603b55"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Local.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9161ac91f2227d37ff499cf47c561f233a288f7d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Local.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-3672d262b1b6bd0c4b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Local.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Local.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0b20c11bcf9022769ddddbd407330fcc92edd4cf60e68a7b4da9f277ddafd0b6`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait LocalEMT
>     extends AnyRef
>     with AstNodeEMT
>     with DeclarationEMT
>     with HasClosureBindingIdEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasGenericSignatureEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT
> 
> trait LocalBase extends AbstractNode with AstNodeBase with DeclarationBase with StaticType[LocalEMT] {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `LocalEMT` (L10), `LocalBase` (L20), `propertiesMap` (L22), `res` (L24), `tmpDynamicTypeHintFullName` (L28), `tmpPossibleTypes` (L36), `Local` (L43), `Label` (L44), `unapply` (L45), `Local` (L108), `productElementName` (L115), `productElement` (L132), `productPrefix` (L149), `productArity` (L150), `canEqual` (L152). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Local.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f2c77cf5d69defdf"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Member.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4b7ca79886993adfc952f49e68e74e1892050f39`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Member.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a1707cf66a766ac420.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Member.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Member.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `acbade94cdfb60da24c98bfdcbc23e0710ee5f77fb8e109fd3e56dee63fd4857`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait MemberEMT
>     extends AnyRef
>     with AstNodeEMT
>     with DeclarationEMT
>     with HasAstParentFullNameEMT
>     with HasAstParentTypeEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasGenericSignatureEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `MemberEMT` (L10), `MemberBase` (L21), `propertiesMap` (L23), `res` (L25), `tmpDynamicTypeHintFullName` (L30), `tmpPossibleTypes` (L38), `Member` (L45), `Label` (L46), `unapply` (L47), `Member` (L115), `productElementName` (L122), `productElement` (L140), `productPrefix` (L158), `productArity` (L159), `canEqual` (L161). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Member.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2f5ec1ce3c4826a4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MetaData.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `607f9a47fdd052111228b066d77d91d3b6215d57`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MetaData.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4897cd726bd9cb6c6b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MetaData.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MetaData.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `735ae7f34fabfbfb300dfdd511b946f8a6eb35831b569bc05b47fb887db88a50`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait MetaDataEMT
>     extends AnyRef
>     with HasHashEMT
>     with HasLanguageEMT
>     with HasOverlaysEMT
>     with HasRootEMT
>     with HasVersionEMT
> 
> trait MetaDataBase extends AbstractNode with StaticType[MetaDataEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 14 lexically identified declarations, including `MetaDataEMT` (L10), `MetaDataBase` (L18), `propertiesMap` (L20), `res` (L22), `tmpOverlays` (L25), `MetaData` (L32), `Label` (L33), `unapply` (L34), `MetaData` (L59), `productElementName` (L64), `productElement` (L74), `productPrefix` (L84), `productArity` (L85), `canEqual` (L87). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MetaData.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9564aca1fb243c2d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Method.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `cb834ae38f3efe96abd1b6d47ae91942a783c4e1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Method.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-4cd4552c4eabb50da7.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Method.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Method.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `59f028bf7744ef0afc20b6c8e0326babcb254e5139d3f489399551075144e3ed`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait MethodEMT
>     extends AnyRef
>     with CfgNodeEMT
>     with DeclarationEMT
>     with HasAstParentFullNameEMT
>     with HasAstParentTypeEMT
>     with HasColumnNumberEndEMT
>     with HasFilenameEMT
>     with HasFullNameEMT
>     with HasGenericSignatureEMT
>     with HasHashEMT


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `MethodEMT` (L10), `MethodBase` (L25), `propertiesMap` (L27), `res` (L29), `Method` (L51), `Label` (L52), `unapply` (L53), `Method` (L144), `productElementName` (L151), `productElement` (L173), `productPrefix` (L195), `productArity` (L196), `canEqual` (L198). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Method.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fe2e81f982975ca5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterIn.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `08b52b83da39278ea1912434a466d5964be97734`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterIn.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-02bd1450a634baec3d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterIn.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterIn.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `62b5b7b8cd1939777d1becb339b833ec8cd4c862ab1efdf77cfafa7c358f2836`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait MethodParameterInEMT
>     extends AnyRef
>     with CfgNodeEMT
>     with DeclarationEMT
>     with HasClosureBindingIdEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasEvaluationStrategyEMT
>     with HasIndexEMT
>     with HasIsVariadicEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `MethodParameterInEMT` (L10), `MethodParameterInBase` (L22), `propertiesMap` (L28), `res` (L30), `tmpDynamicTypeHintFullName` (L34), `tmpPossibleTypes` (L44), `MethodParameterIn` (L51), `Label` (L52), `unapply` (L53), `MethodParameterIn` (L127), `productElementName` (L134), `productElement` (L153), `productPrefix` (L172), `productArity` (L173), `canEqual` (L175). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterIn.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d0ca10e855c90425"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterOut.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c5f7d5da2179075f6593bc350a53e0001ba20f45`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterOut.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-30e3471784730f063d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterOut.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterOut.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `60292ae244f84ebf0c65fb1f16bd7459e520f81f60ba5c497f6d3204339c7430`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait MethodParameterOutEMT
>     extends AnyRef
>     with CfgNodeEMT
>     with DeclarationEMT
>     with HasEvaluationStrategyEMT
>     with HasIndexEMT
>     with HasIsVariadicEMT
>     with HasTypeFullNameEMT
> 
> trait MethodParameterOutBase
>     extends AbstractNode


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `MethodParameterOutEMT` (L10), `MethodParameterOutBase` (L19), `propertiesMap` (L25), `res` (L27), `MethodParameterOut` (L43), `Label` (L44), `unapply` (L45), `MethodParameterOut` (L106), `productElementName` (L113), `productElement` (L129), `productPrefix` (L145), `productArity` (L146), `canEqual` (L148). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodParameterOut.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6fca9570b55244ec"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodRef.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b86ac24153da984ab823543503c55baa78fa9bbb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodRef.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-1b1044fd7fa8624dc7.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodRef.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodRef.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `611890f10d7167ca5a02d27e9e3b6734aafc630229ecf09c451eb7ed212e2151`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait MethodRefEMT
>     extends AnyRef
>     with ExpressionEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasMethodFullNameEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT
> 
> trait MethodRefBase extends AbstractNode with ExpressionBase with StaticType[MethodRefEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `MethodRefEMT` (L10), `MethodRefBase` (L18), `propertiesMap` (L20), `res` (L22), `tmpDynamicTypeHintFullName` (L28), `tmpPossibleTypes` (L35), `MethodRef` (L42), `Label` (L43), `unapply` (L44), `MethodRef` (L120), `productElementName` (L126), `productElement` (L144), `productPrefix` (L162), `productArity` (L163), `canEqual` (L165). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodRef.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d0e1e66f9328a616"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodReturn.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `a6e37e2646e6fb7809720263540da903638700be`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodReturn.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-93978a6ccb596a259c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodReturn.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodReturn.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `00577854bfea44415b2cd201a9a5a04a911c229642793d884f2e6d3b48bc4108`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait MethodReturnEMT
>     extends AnyRef
>     with CfgNodeEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasEvaluationStrategyEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT
> 
> trait MethodReturnBase extends AbstractNode with CfgNodeBase with StaticType[MethodReturnEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `MethodReturnEMT` (L10), `MethodReturnBase` (L18), `propertiesMap` (L20), `res` (L22), `tmpDynamicTypeHintFullName` (L25), `tmpPossibleTypes` (L32), `MethodReturn` (L39), `Label` (L40), `unapply` (L41), `MethodReturn` (L97), `productElementName` (L103), `productElement` (L118), `productPrefix` (L133), `productArity` (L134), `canEqual` (L136). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/MethodReturn.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-70be5e7f77a792c4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Modifier.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `6ffb372339b0cfe85914c33bd5279a6aee8c5ff1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Modifier.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-7f0a701794558ad09f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Modifier.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Modifier.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `597ec1043bbc5c4c6b68d5c4dce6e821b2a421f8229ca0694a1d756cb65aeb61`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait ModifierEMT extends AnyRef with AstNodeEMT with HasModifierTypeEMT
> 
> trait ModifierBase extends AbstractNode with AstNodeBase with StaticType[ModifierEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     this.lineNumber.foreach { p => res.put("LINE_NUMBER", p) }
>     if (("<empty>": String) != this.modifierType) res.put("MODIFIER_TYPE", this.modifierType)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `ModifierEMT` (L10), `ModifierBase` (L12), `propertiesMap` (L14), `res` (L16), `Modifier` (L28), `Label` (L29), `unapply` (L30), `Modifier` (L69), `productElementName` (L75), `productElement` (L87), `productPrefix` (L99), `productArity` (L100), `canEqual` (L102). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Modifier.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-85ebe2f414f5775b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Namespace.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0fbae482f3432c4c30dcdd6a5da18635d5926c9d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Namespace.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-29f8e33379917ab042.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Namespace.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Namespace.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `539057c4e67772797ba1047df89dcd23a08c568e0fd4b3345ee8ecf5c77aa1a9`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait NamespaceEMT extends AnyRef with AstNodeEMT with HasNameEMT
> 
> trait NamespaceBase extends AbstractNode with AstNodeBase with StaticType[NamespaceEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     this.lineNumber.foreach { p => res.put("LINE_NUMBER", p) }
>     if (("<empty>": String) != this.name) res.put("NAME", this.name)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `NamespaceEMT` (L10), `NamespaceBase` (L12), `propertiesMap` (L14), `res` (L16), `Namespace` (L28), `Label` (L29), `unapply` (L30), `Namespace` (L68), `productElementName` (L74), `productElement` (L86), `productPrefix` (L98), `productArity` (L99), `canEqual` (L101). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Namespace.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bfb64145c75e3bd4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NamespaceBlock.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `75bb16cb9b7cfa3dba0adda857773a3eef32640b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NamespaceBlock.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-58842a81baec50823e.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NamespaceBlock.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NamespaceBlock.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a56325dafaa4494654e0e2bf7a62aefdef349a85941a8bc2b92dcec76d7ff02e`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait NamespaceBlockEMT extends AnyRef with AstNodeEMT with HasFilenameEMT with HasFullNameEMT with HasNameEMT
> 
> trait NamespaceBlockBase extends AbstractNode with AstNodeBase with StaticType[NamespaceBlockEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     if (("<empty>": String) != this.filename) res.put("FILENAME", this.filename)
>     if (("<empty>": String) != this.fullName) res.put("FULL_NAME", this.fullName)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `NamespaceBlockEMT` (L10), `NamespaceBlockBase` (L12), `propertiesMap` (L14), `res` (L16), `NamespaceBlock` (L30), `Label` (L31), `unapply` (L32), `NamespaceBlock` (L82), `productElementName` (L88), `productElement` (L102), `productPrefix` (L116), `productArity` (L117), `canEqual` (L119). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NamespaceBlock.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5f159daafbec38a5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotation.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1f5187e930b538b3ad941027a00f4d9fe33811ec`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotation.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1c12d1f424e3e2987a.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotation.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotation.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3664242811d6ccef98ca3b68feaecf0c0304903ba16ba6341030c34617439667`.

> 
> object NewAnnotation {
>   def apply(): NewAnnotation = new NewAnnotation
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 90 lexically identified declarations, including `NewAnnotation` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Annotation_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Annotation_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Annotation_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Annotation_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotation.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fb0a883a11c2774d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationLiteral.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `212efc5197719a85764069a42586223752069675`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationLiteral.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f4a74abf7cd2544d43.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationLiteral.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationLiteral.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c680c5d025d79778d87eaaf8882efbb2eb118eb1a9752effd53f738f3e7894c5`.

> 
> object NewAnnotationLiteral {
>   def apply(): NewAnnotationLiteral = new NewAnnotationLiteral
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 84 lexically identified declarations, including `NewAnnotationLiteral` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_AnnotationLiteral_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_AnnotationLiteral_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_AnnotationLiteral_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_AnnotationLiteral_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationLiteral.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7d87fcd84fc157ff"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameter.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `ac5a77a0cea08de0a8500d8c3ca05af0fb900f6f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameter.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-853fe3481725d69c5b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameter.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameter.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `129d49bd59f650db49717c6210775de4dd4ea3b53f2b906441cfd46bb4edd188`.

> 
> object NewAnnotationParameter {
>   def apply(): NewAnnotationParameter = new NewAnnotationParameter
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 58 lexically identified declarations, including `NewAnnotationParameter` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_AnnotationParameter_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_AnnotationParameter_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_AnnotationParameter_lineNumber` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_AnnotationParameter_offset` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameter.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-28150acb48dc75a6"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameterAssign.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c2d0d66afedbbf9d238348743bf42770b1a8d8b0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameterAssign.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0342d32fcc9ec12a92.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameterAssign.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameterAssign.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `101c98cc5977cb20494246db63f499bf6276dbe52c7626c96e95e27c4e0087cf`.

> 
> object NewAnnotationParameterAssign {
>   def apply(): NewAnnotationParameterAssign = new NewAnnotationParameterAssign
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 58 lexically identified declarations, including `NewAnnotationParameterAssign` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_AnnotationParameterAssign_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_AnnotationParameterAssign_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_AnnotationParameterAssign_lineNumber` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_AnnotationParameterAssign_offset` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewAnnotationParameterAssign.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-dd3c95d986aa4eda"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewArrayInitializer.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `abcdd4ff272546902609d741b3f3bfd7b38859ed`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewArrayInitializer.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6ab26514cd7032450a.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewArrayInitializer.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewArrayInitializer.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `69e874bf1b1f34aa07b98ea27acaace4bd83855c18bce0f4c12687f53a80d049`.

> 
> object NewArrayInitializer {
>   def apply(): NewArrayInitializer = new NewArrayInitializer
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 78 lexically identified declarations, including `NewArrayInitializer` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_ArrayInitializer_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_ArrayInitializer_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_ArrayInitializer_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_ArrayInitializer_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewArrayInitializer.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a983ec5c49d5cfb7"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBinding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `fa1e3ddcc8f6d6a4fb1048b16b261b99b98a9a79`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBinding.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bafab3535fbfc41646.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBinding.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBinding.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `67dd6caeebe6ef829eb3d168bc5f9aa528d94dda2fe6bb747bcf9f5c8379bf70`.

> 
> object NewBinding {
>   def apply(): NewBinding                            = new NewBinding
>   private val outNeighbors: Map[String, Set[String]] = Map("REF" -> Set("METHOD"))
>   private val inNeighbors: Map[String, Set[String]]  = Map("BINDS" -> Set("TYPE_DECL"))
> 
>   object InsertionHelpers {
>     object NewNodeInserter_Binding_methodFullName extends flatgraph.NewNodePropertyInsertionHelper {
>       override def insertNewNodeProperties(
>         newNodes: mutable.ArrayBuffer[flatgraph.DNode],
>         dst: AnyRef,
>         offsets: Array[Int]
>       ): Unit = {
>         if (newNodes.isEmpty) return
>         val dstCast = dst.asInstanceOf[Array[String]]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 36 lexically identified declarations, including `NewBinding` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L12), `NewNodeInserter_Binding_methodFullName` (L13), `insertNewNodeProperties` (L14), `dstCast` (L20), `seq` (L21), `nn` (L25), `NewNodeInserter_Binding_name` (L38), `insertNewNodeProperties` (L39), `dstCast` (L45), `seq` (L46), `nn` (L50), `NewNodeInserter_Binding_signature` (L63), `insertNewNodeProperties` (L64), `dstCast` (L70), `seq` (L71), `nn` (L75), `NewBinding` (L91), `StoredNodeType` (L92), `label` (L93), `isValidOutNeighbor` (L95). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBinding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0c5c326d9445b5fd"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBlock.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `2c846ca92fa4d5e53530374231a95642e915cab4`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBlock.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2c4b01b54fbcdf0eb4.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBlock.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBlock.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7ee9161829fd2d011e40161162ad565abe115e0f70fb777413a55241ebce1a08`.

> 
> object NewBlock {
>   def apply(): NewBlock = new NewBlock
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 96 lexically identified declarations, including `NewBlock` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Block_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Block_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Block_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Block_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewBlock.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ec2673be87f9c1a5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewCall.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f0c28e5a39ea6b3a08ba356eccc0554fffbdf381`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewCall.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-03a4794fc20c6f7b13.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewCall.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewCall.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `31b893263418458bfbbc62bae635c5061ca9a593fcb45f0c0611547a36c82989`.

> 
> object NewCall {
>   def apply(): NewCall = new NewCall
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 127 lexically identified declarations, including `NewCall` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Call_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Call_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Call_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Call_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewCall.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-45e0286e41f681f2"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewClosureBinding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b706cd30efd565147a04e8cd74d718e683285c61`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewClosureBinding.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-01491556da577bbc6b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewClosureBinding.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewClosureBinding.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `ea7e309aefdc4af74b4a95f7b75d44f83d8a4adb5856b1e8f645e2c13b3d72ee`.

> 
> object NewClosureBinding {
>   def apply(): NewClosureBinding                     = new NewClosureBinding
>   private val outNeighbors: Map[String, Set[String]] = Map("REF" -> Set("LOCAL", "METHOD_PARAMETER_IN"))
>   private val inNeighbors: Map[String, Set[String]] =
>     Map("CAPTURE" -> Set("METHOD_REF", "TYPE_REF"), "CAPTURED_BY" -> Set("LOCAL", "METHOD_PARAMETER_IN"))
> 
>   object InsertionHelpers {
>     object NewNodeInserter_ClosureBinding_closureBindingId extends flatgraph.NewNodePropertyInsertionHelper {
>       override def insertNewNodeProperties(
>         newNodes: mutable.ArrayBuffer[flatgraph.DNode],
>         dst: AnyRef,
>         offsets: Array[Int]
>       ): Unit = {
>         if (newNodes.isEmpty) return


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 31 lexically identified declarations, including `NewClosureBinding` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L13), `NewNodeInserter_ClosureBinding_closureBindingId` (L14), `insertNewNodeProperties` (L15), `dstCast` (L21), `seq` (L22), `nn` (L26), `NewNodeInserter_ClosureBinding_evaluationStrategy` (L43), `insertNewNodeProperties` (L44), `dstCast` (L50), `seq` (L51), `nn` (L55), `NewClosureBinding` (L71), `StoredNodeType` (L72), `label` (L73), `isValidOutNeighbor` (L75), `isValidInNeighbor` (L78), `closureBindingId` (L84), `closureBindingId` (L85), `evaluationStrategy` (L86), `countAndVisitProperties` (L87). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewClosureBinding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e5c772af7e57b93a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewComment.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `647e1707236394ccf4c9d14f30a374ea1a8b3bcd`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewComment.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-30d6182457284d6971.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewComment.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewComment.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d79797260a356febd603b9b8b00c7ff6818d2067b611cab4a35aa3b5d702b4ce`.

> 
> object NewComment {
>   def apply(): NewComment = new NewComment
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 64 lexically identified declarations, including `NewComment` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Comment_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Comment_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Comment_filename` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Comment_lineNumber` (L2083), `insertNewNodeProperties` (L2084), `dstCast` (L2090), `seq` (L2091). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewComment.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a3cd1bc5960089c9"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewConfigFile.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4194970339db7ecf214f29563e7558ae6158f821`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewConfigFile.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6b159ca65eaadfc338.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewConfigFile.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewConfigFile.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `08689cde574db361eddcc40c31093aa890b2aa55f2dbf08287d2f8e57fa1a4bd`.

> 
> object NewConfigFile {
>   def apply(): NewConfigFile                         = new NewConfigFile
>   private val outNeighbors: Map[String, Set[String]] = Map()
>   private val inNeighbors: Map[String, Set[String]]  = Map()
> 
>   object InsertionHelpers {
>     object NewNodeInserter_ConfigFile_content extends flatgraph.NewNodePropertyInsertionHelper {
>       override def insertNewNodeProperties(
>         newNodes: mutable.ArrayBuffer[flatgraph.DNode],
>         dst: AnyRef,
>         offsets: Array[Int]
>       ): Unit = {
>         if (newNodes.isEmpty) return
>         val dstCast = dst.asInstanceOf[Array[String]]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 30 lexically identified declarations, including `NewConfigFile` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L12), `NewNodeInserter_ConfigFile_content` (L13), `insertNewNodeProperties` (L14), `dstCast` (L20), `seq` (L21), `nn` (L25), `NewNodeInserter_ConfigFile_name` (L38), `insertNewNodeProperties` (L39), `dstCast` (L45), `seq` (L46), `nn` (L50), `NewConfigFile` (L66), `StoredNodeType` (L67), `label` (L68), `isValidOutNeighbor` (L70), `isValidInNeighbor` (L73), `content` (L79), `name` (L80), `countAndVisitProperties` (L81), `copy` (L86). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewConfigFile.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bb69b4445b818f13"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewControlStructure.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8498716f28f0577e30faaa52704130fd95131f77`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewControlStructure.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1c1625ff03234ed0e5.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewControlStructure.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewControlStructure.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cca42343574627dbdbcf9e004c5e1653aad6b92c420ad86a14835119ff757497`.

> 
> object NewControlStructure {
>   def apply(): NewControlStructure = new NewControlStructure
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 90 lexically identified declarations, including `NewControlStructure` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_ControlStructure_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_ControlStructure_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_ControlStructure_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_ControlStructure_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewControlStructure.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cfa3c3f7b72dc0aa"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewDependency.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1317a0a91a3de9ca59f4244447105fde30f00b3f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewDependency.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3093bd9099e3830bea.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewDependency.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewDependency.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b5e1efa7e0eb89ace83349be0b68c314a69f7881008466ec0eb0abb2b443dbd3`.

> 
> object NewDependency {
>   def apply(): NewDependency                         = new NewDependency
>   private val outNeighbors: Map[String, Set[String]] = Map()
>   private val inNeighbors: Map[String, Set[String]]  = Map("IMPORTS" -> Set("IMPORT"))
> 
>   object InsertionHelpers {
>     object NewNodeInserter_Dependency_dependencyGroupId extends flatgraph.NewNodePropertyInsertionHelper {
>       override def insertNewNodeProperties(
>         newNodes: mutable.ArrayBuffer[flatgraph.DNode],
>         dst: AnyRef,
>         offsets: Array[Int]
>       ): Unit = {
>         if (newNodes.isEmpty) return
>         val dstCast = dst.asInstanceOf[Array[String]]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 37 lexically identified declarations, including `NewDependency` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L12), `NewNodeInserter_Dependency_dependencyGroupId` (L13), `insertNewNodeProperties` (L14), `dstCast` (L20), `seq` (L21), `nn` (L25), `NewNodeInserter_Dependency_name` (L42), `insertNewNodeProperties` (L43), `dstCast` (L49), `seq` (L50), `nn` (L54), `NewNodeInserter_Dependency_version` (L67), `insertNewNodeProperties` (L68), `dstCast` (L74), `seq` (L75), `nn` (L79), `NewDependency` (L95), `StoredNodeType` (L96), `label` (L97), `isValidOutNeighbor` (L99). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewDependency.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-60c39bbcb605e4b8"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFieldIdentifier.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `a08cedf64de71c955a638132be7a69a9089fe391`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFieldIdentifier.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6a9104df17cb8a26dd.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFieldIdentifier.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFieldIdentifier.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `79f43948fa45269dea72894a2b6559e19805266acb1d538a2cd5c8ce956452e2`.

> 
> object NewFieldIdentifier {
>   def apply(): NewFieldIdentifier = new NewFieldIdentifier
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 84 lexically identified declarations, including `NewFieldIdentifier` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_FieldIdentifier_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_FieldIdentifier_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_FieldIdentifier_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_FieldIdentifier_canonicalName` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFieldIdentifier.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-acfa491b13ae3f89"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFile.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9e32bbaaae835f3aaf8272ae1412c95caeaf53b4`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFile.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-87b166dddca485d87a.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFile.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFile.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9556fa37e467f0f8cb081b1bf18bd22cec2d9d0ebd97c0f5e61f3f85b67c8789`.

> 
> object NewFile {
>   def apply(): NewFile = new NewFile
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 77 lexically identified declarations, including `NewFile` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_File_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_File_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_File_content` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_File_hash` (L2083), `insertNewNodeProperties` (L2084), `dstCast` (L2090), `seq` (L2091). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFile.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ea997089eafc0aab"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFinding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5ea9b4b0b7f38c145793c2049870ae0f63311845`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFinding.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a47738be8bee1b7cda.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFinding.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFinding.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `83b494b3a11fb3b0e859fb6a5f7ec81acb6b4503524617e9ce82d518d0117c3e`.

> 
> object NewFinding {
>   def apply(): NewFinding                            = new NewFinding
>   private val outNeighbors: Map[String, Set[String]] = Map()
>   private val inNeighbors: Map[String, Set[String]]  = Map()
> 
>   object InsertionHelpers {
>     object NewNodeInserter_Finding_evidenceDescription extends flatgraph.NewNodePropertyInsertionHelper {
>       override def insertNewNodeProperties(
>         newNodes: mutable.ArrayBuffer[flatgraph.DNode],
>         dst: AnyRef,
>         offsets: Array[Int]
>       ): Unit = {
>         if (newNodes.isEmpty) return
>         val dstCast = dst.asInstanceOf[Array[String]]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 36 lexically identified declarations, including `NewFinding` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L12), `NewNodeInserter_Finding_evidenceDescription` (L13), `insertNewNodeProperties` (L14), `dstCast` (L20), `seq` (L21), `nn` (L25), `NewNodeInserter_Finding_evidence` (L40), `insertNewNodeProperties` (L41), `dstCast` (L47), `seq` (L48), `nn` (L52), `NewNodeInserter_Finding_keyValuePairs` (L70), `insertNewNodeProperties` (L71), `dstCast` (L77), `seq` (L78), `nn` (L82), `NewFinding` (L103), `StoredNodeType` (L104), `label` (L105), `isValidOutNeighbor` (L107). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewFinding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e2229fe6d2563831"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewIdentifier.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `3afeb5b9a944fe3dd88f101f9bfa7bfcb84da818`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewIdentifier.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9621e54bff5b0d57d0.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewIdentifier.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewIdentifier.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `fe05dfa7c23c8249d04b9c5e36214b966fc130131d0a4f1afd7c1d2f6a3b2ea7`.

> 
> object NewIdentifier {
>   def apply(): NewIdentifier = new NewIdentifier
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 102 lexically identified declarations, including `NewIdentifier` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Identifier_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Identifier_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Identifier_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Identifier_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewIdentifier.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-272c435c9d3adfd1"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewImport.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `48e4848dc029bcfec15dcda2d172ac3e31b9adaf`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewImport.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3943b67d1d13bf3350.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewImport.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewImport.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `56621bdd32ac585bf96265d3908616c93cbd28a6d23fcb2df4aab8b6c1ba4308`.

> 
> object NewImport {
>   def apply(): NewImport = new NewImport
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 100 lexically identified declarations, including `NewImport` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Import_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Import_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Import_explicitAs` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Import_importedAs` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewImport.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5891beb23a0bdc22"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpLabel.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `52d40d7524e4c364bd830b0cc8304782d5f523e5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpLabel.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-84999f19b17b762628.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpLabel.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpLabel.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `8b5c5986db2c67d4f36fe9b831306baf20e5426bc7bdbcc3a72c7e1db8fc4c72`.

> 
> object NewJumpLabel {
>   def apply(): NewJumpLabel = new NewJumpLabel
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 70 lexically identified declarations, including `NewJumpLabel` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_JumpLabel_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_JumpLabel_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_JumpLabel_lineNumber` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_JumpLabel_name` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpLabel.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-15eae84c7f286d27"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpTarget.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `a69922392f78b81803befe4bd09bf5759f3dd320`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpTarget.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3d483d7853f159c780.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpTarget.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpTarget.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `4df7858b4d8522af14e96e13cbe74b460644a6e62dc20025ff3a6a9c588745ba`.

> 
> object NewJumpTarget {
>   def apply(): NewJumpTarget = new NewJumpTarget
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 76 lexically identified declarations, including `NewJumpTarget` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_JumpTarget_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_JumpTarget_code` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_JumpTarget_columnNumber` (L2054), `insertNewNodeProperties` (L2055), `dstCast` (L2061), `seq` (L2062), `nn` (L2066), `NewNodeInserter_JumpTarget_lineNumber` (L2083), `insertNewNodeProperties` (L2084), `dstCast` (L2090), `seq` (L2091). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewJumpTarget.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-55d5a215a859127b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewKeyValuePair.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `334f9d044f460184f07a1b42c32fbd6849d81fdb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewKeyValuePair.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1cea451de50965279d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewKeyValuePair.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewKeyValuePair.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f1237357a25878d7934eeea95de5a4fdc7443ee4c91218bc12c11b6af647cbb2`.

> 
> object NewKeyValuePair {
>   def apply(): NewKeyValuePair                       = new NewKeyValuePair
>   private val outNeighbors: Map[String, Set[String]] = Map()
>   private val inNeighbors: Map[String, Set[String]]  = Map()
> 
>   object InsertionHelpers {
>     object NewNodeInserter_KeyValuePair_key extends flatgraph.NewNodePropertyInsertionHelper {
>       override def insertNewNodeProperties(
>         newNodes: mutable.ArrayBuffer[flatgraph.DNode],
>         dst: AnyRef,
>         offsets: Array[Int]
>       ): Unit = {
>         if (newNodes.isEmpty) return
>         val dstCast = dst.asInstanceOf[Array[String]]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 30 lexically identified declarations, including `NewKeyValuePair` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L12), `NewNodeInserter_KeyValuePair_key` (L13), `insertNewNodeProperties` (L14), `dstCast` (L20), `seq` (L21), `nn` (L25), `NewNodeInserter_KeyValuePair_value` (L38), `insertNewNodeProperties` (L39), `dstCast` (L45), `seq` (L46), `nn` (L50), `NewKeyValuePair` (L66), `StoredNodeType` (L67), `label` (L68), `isValidOutNeighbor` (L70), `isValidInNeighbor` (L73), `key` (L79), `value` (L80), `countAndVisitProperties` (L81), `copy` (L86). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewKeyValuePair.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6979cbbbe5211450"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLiteral.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `fc7d9cedec0fd6d7acdfc5dcf08498b5f61613de`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLiteral.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9b9af416223ca02ea3.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLiteral.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLiteral.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c778eb8a009f7d835e61f9b447bd1de9d42a6dc89a67abc7b7ac8ed59385736c`.

> 
> object NewLiteral {
>   def apply(): NewLiteral = new NewLiteral
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 96 lexically identified declarations, including `NewLiteral` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Literal_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Literal_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Literal_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Literal_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLiteral.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3508f9fa4c565c2b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLocal.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `3deb74efbf1dc991ebf82b04064d049b57e2a776`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLocal.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4f0ea4af9e2481b2f9.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLocal.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLocal.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `737657f69a70819abddbc729f7bbd8cb330e30e47273595d8f4579b184578515`.

> 
> object NewLocal {
>   def apply(): NewLocal = new NewLocal
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 95 lexically identified declarations, including `NewLocal` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Local_closureBindingId` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Local_code` (L2033), `insertNewNodeProperties` (L2034), `dstCast` (L2040), `seq` (L2041), `nn` (L2045), `NewNodeInserter_Local_columnNumber` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Local_dynamicTypeHintFullName` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewLocal.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ca230cb0f5e5e0c8"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMember.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b790138294a36e1ba56d95acc641e3fabc480cb0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMember.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-be30f70a95f1367fa9.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMember.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMember.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `458a05b2140335f8de524d644a9d28824491d044a2d84fcf924c777604976e52`.

> 
> object NewMember {
>   def apply(): NewMember = new NewMember
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 100 lexically identified declarations, including `NewMember` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Member_astParentFullName` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Member_astParentType` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Member_code` (L2054), `insertNewNodeProperties` (L2055), `dstCast` (L2061), `seq` (L2062), `nn` (L2066), `NewNodeInserter_Member_columnNumber` (L2079), `insertNewNodeProperties` (L2080), `dstCast` (L2086), `seq` (L2087). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMember.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bcf03bfa58aa50b1"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMetaData.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0d7746c8b1db3676b5925e2f83a62fc987e795bc`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMetaData.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5add7edcb47c0a02c4.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMetaData.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMetaData.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5c97f98a953db269f7ef031aa815e0222677d61eca754dfb1d29e6522fc509a9`.

> 
> object NewMetaData {
>   def apply(): NewMetaData                           = new NewMetaData
>   private val outNeighbors: Map[String, Set[String]] = Map()
>   private val inNeighbors: Map[String, Set[String]]  = Map()
> 
>   object InsertionHelpers {
>     object NewNodeInserter_MetaData_hash extends flatgraph.NewNodePropertyInsertionHelper {
>       override def insertNewNodeProperties(
>         newNodes: mutable.ArrayBuffer[flatgraph.DNode],
>         dst: AnyRef,
>         offsets: Array[Int]
>       ): Unit = {
>         if (newNodes.isEmpty) return
>         val dstCast = dst.asInstanceOf[Array[String]]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 49 lexically identified declarations, including `NewMetaData` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L12), `NewNodeInserter_MetaData_hash` (L13), `insertNewNodeProperties` (L14), `dstCast` (L20), `seq` (L21), `nn` (L25), `NewNodeInserter_MetaData_language` (L42), `insertNewNodeProperties` (L43), `dstCast` (L49), `seq` (L50), `nn` (L54), `NewNodeInserter_MetaData_overlays` (L67), `insertNewNodeProperties` (L68), `dstCast` (L74), `seq` (L75), `nn` (L79), `NewNodeInserter_MetaData_root` (L94), `insertNewNodeProperties` (L95), `dstCast` (L101), `seq` (L102). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMetaData.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-926b700a7e09f32a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethod.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `d9248d49d1450a8c1f987fe02c855f2e28af87f5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethod.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bbae3fd00e4a0bbc19.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethod.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethod.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `61a85829d2d1b52b93d915255138ab21c1d8815f2ae65ebbfaa3ff7837d1db25`.

> 
> object NewMethod {
>   def apply(): NewMethod = new NewMethod
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 127 lexically identified declarations, including `NewMethod` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Method_astParentFullName` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Method_astParentType` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Method_code` (L2054), `insertNewNodeProperties` (L2055), `dstCast` (L2061), `seq` (L2062), `nn` (L2066), `NewNodeInserter_Method_columnNumber` (L2079), `insertNewNodeProperties` (L2080), `dstCast` (L2086), `seq` (L2087). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethod.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7e93c222ab11341d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterIn.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `da0656ee8a1114e3fe4d0e8ffbcd36d5f7542f1e`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterIn.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-cac0a7cfd953fe2609.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterIn.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterIn.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `560eb016712ee81ab8d86fd33a8d526bfc2f4340cee97be460f798e0c215495e`.

> 
> object NewMethodParameterIn {
>   def apply(): NewMethodParameterIn = new NewMethodParameterIn
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 107 lexically identified declarations, including `NewMethodParameterIn` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_MethodParameterIn_closureBindingId` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_MethodParameterIn_code` (L2033), `insertNewNodeProperties` (L2034), `dstCast` (L2040), `seq` (L2041), `nn` (L2045), `NewNodeInserter_MethodParameterIn_columnNumber` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_MethodParameterIn_dynamicTypeHintFullName` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterIn.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ecaa6d5d3479b9a9"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterOut.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `6a243666c82d965f773973d6283d918b2ff7cd24`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterOut.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-62408f10e0d5915ff4.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterOut.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterOut.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `17365a3f714e137e2b016acdb9d9e5ffce52301cbfea2d0e4aaeababdec28728`.

> 
> object NewMethodParameterOut {
>   def apply(): NewMethodParameterOut = new NewMethodParameterOut
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 88 lexically identified declarations, including `NewMethodParameterOut` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_MethodParameterOut_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_MethodParameterOut_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_MethodParameterOut_evaluationStrategy` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_MethodParameterOut_index` (L2083), `insertNewNodeProperties` (L2084), `dstCast` (L2090), `seq` (L2091). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodParameterOut.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3e30b7c4afcb0e1a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodRef.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c20774033e81de4f83e5dcb61a99bdcf92510f99`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodRef.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2186b5289b793f3091.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodRef.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodRef.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `26fbf24b9ca23b7dcab6e913c2704bebe7a9247ba5b8d4e2842a03e5c3b29d70`.

> 
> object NewMethodRef {
>   def apply(): NewMethodRef = new NewMethodRef
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 102 lexically identified declarations, including `NewMethodRef` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_MethodRef_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_MethodRef_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_MethodRef_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_MethodRef_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodRef.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5f85452edf77003f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodReturn.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7d7ccf9a1bbd3c9b5ac63749a24fe6590e1c53cd`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodReturn.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-54534321dcbfcdf32f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodReturn.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodReturn.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d252e28a5a29aac6c34917b3e0006b8eb6686117d0ad05ff24e95641b3de089f`.

> 
> object NewMethodReturn {
>   def apply(): NewMethodReturn = new NewMethodReturn
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 82 lexically identified declarations, including `NewMethodReturn` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_MethodReturn_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_MethodReturn_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_MethodReturn_dynamicTypeHintFullName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_MethodReturn_evaluationStrategy` (L2085), `insertNewNodeProperties` (L2086), `dstCast` (L2092), `seq` (L2093). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewMethodReturn.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-34a93b7faef7c1a1"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewModifier.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `db58d548cdc3eac1e4366966666f9f802fb770a5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewModifier.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6788abbf80f1cebc09.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewModifier.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewModifier.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `8ad9a99e1086d6da10e55ae9840ae1bcfd37fd37a261f4e676f6dba39f27e130`.

> 
> object NewModifier {
>   def apply(): NewModifier = new NewModifier
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 64 lexically identified declarations, including `NewModifier` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Modifier_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Modifier_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Modifier_lineNumber` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Modifier_modifierType` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewModifier.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0fd1b05ebceb5602"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespace.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b9461c74f5dbf9d33b8597611f6de165b5f33ffe`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespace.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-508303bba6d9bd1f0d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespace.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespace.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e6a4a91c702146d2e992b001753d432adf2bbfbc9c6e6ac33ea260d6a2e226a5`.

> 
> object NewNamespace {
>   def apply(): NewNamespace = new NewNamespace
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 64 lexically identified declarations, including `NewNamespace` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Namespace_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Namespace_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Namespace_lineNumber` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Namespace_name` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespace.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2039ff40fcf38e97"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespaceBlock.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `ba36aa385893e90e7e4b96af8faca9fe7a870711`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespaceBlock.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-122eb017ef6111fe64.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespaceBlock.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespaceBlock.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `39f03e37c439f270ae3636626bbf47a32d26f5a9fae15a099180e4fe54b52f7b`.

> 
> object NewNamespaceBlock {
>   def apply(): NewNamespaceBlock = new NewNamespaceBlock
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 76 lexically identified declarations, including `NewNamespaceBlock` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_NamespaceBlock_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_NamespaceBlock_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_NamespaceBlock_filename` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_NamespaceBlock_fullName` (L2083), `insertNewNodeProperties` (L2084), `dstCast` (L2090), `seq` (L2091). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewNamespaceBlock.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7e72dede86ce96d9"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewReturn.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `2b228734262422c04d17148892b769bcfc55dec6`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewReturn.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2e8c29de863a4d8ade.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewReturn.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewReturn.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b7878db284ec1419dc1aec54ee2bbc3353f1e49bc18bd77bfc083b64e58dc5ba`.

> 
> object NewReturn {
>   def apply(): NewReturn = new NewReturn
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 78 lexically identified declarations, including `NewReturn` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Return_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Return_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Return_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Return_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewReturn.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-dd37f7375647f42d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTag.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f860e61e6462540ecf2dc63f2d6e0a1794e2c44b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTag.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-014262477c4cd096eb.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTag.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTag.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dcbc3173da91240ea59a11a5eb099c93763fd34b511f1bd165660ca623c29344`.

> 
> object NewTag {
>   def apply(): NewTag                                = new NewTag
>   private val outNeighbors: Map[String, Set[String]] = Map("TAGGED_BY" -> Set("TAG"))
>   private val inNeighbors: Map[String, Set[String]] = Map(
>     "TAGGED_BY" -> Set(
>       "BLOCK",
>       "CALL",
>       "CONTROL_STRUCTURE",
>       "FIELD_IDENTIFIER",
>       "FILE",
>       "IDENTIFIER",
>       "IMPORT",
>       "JUMP_TARGET",
>       "LITERAL",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 30 lexically identified declarations, including `NewTag` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L37), `NewNodeInserter_Tag_name` (L38), `insertNewNodeProperties` (L39), `dstCast` (L45), `seq` (L46), `nn` (L50), `NewNodeInserter_Tag_value` (L63), `insertNewNodeProperties` (L64), `dstCast` (L70), `seq` (L71), `nn` (L75), `NewTag` (L91), `StoredNodeType` (L92), `label` (L93), `isValidOutNeighbor` (L95), `isValidInNeighbor` (L98), `name` (L104), `value` (L105), `countAndVisitProperties` (L106), `copy` (L111). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTag.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b715fb2c4432bbd0"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTagNodePair.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `ff47246f86fc7dd643441194afde1679c8168573`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTagNodePair.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-56d5eb729fd53b6295.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTagNodePair.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTagNodePair.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7620769a6bcbe087bcd651098acaead78bea21093048f8697855c1be324e9b59`.

> 
> object NewTagNodePair {
>   def apply(): NewTagNodePair                        = new NewTagNodePair
>   private val outNeighbors: Map[String, Set[String]] = Map()
>   private val inNeighbors: Map[String, Set[String]]  = Map()
> 
>   object InsertionHelpers {
>     object NewNodeInserter_TagNodePair_node extends flatgraph.NewNodePropertyInsertionHelper {
>       override def insertNewNodeProperties(
>         newNodes: mutable.ArrayBuffer[flatgraph.DNode],
>         dst: AnyRef,
>         offsets: Array[Int]
>       ): Unit = {
>         if (newNodes.isEmpty) return
>         val dstCast = dst.asInstanceOf[Array[flatgraph.GNode]]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 30 lexically identified declarations, including `NewTagNodePair` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L12), `NewNodeInserter_TagNodePair_node` (L13), `insertNewNodeProperties` (L14), `dstCast` (L20), `seq` (L21), `nn` (L25), `NewNodeInserter_TagNodePair_tag` (L40), `insertNewNodeProperties` (L41), `dstCast` (L47), `seq` (L48), `nn` (L52), `NewTagNodePair` (L70), `StoredNodeType` (L71), `label` (L72), `isValidOutNeighbor` (L74), `isValidInNeighbor` (L77), `node` (L83), `tag` (L84), `countAndVisitProperties` (L85), `copy` (L92). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTagNodePair.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6e7aee53f0248aed"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTemplateDom.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b1e35e92249ed3c751fc9d3e142203c27ed1770f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTemplateDom.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e102d5b5811d41cd0c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTemplateDom.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTemplateDom.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3cdd09ee4f2050198eb356ed60ed954344b79b16da74464f79c41034c983f5a8`.

> 
> object NewTemplateDom {
>   def apply(): NewTemplateDom = new NewTemplateDom
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 84 lexically identified declarations, including `NewTemplateDom` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_TemplateDom_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_TemplateDom_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_TemplateDom_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_TemplateDom_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTemplateDom.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6c2294be43fe20f7"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewType.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1b880e69ccfed5bdd7fc046b7f604178889a71f9`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewType.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-aef0d7b0c616f31b79.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewType.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewType.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5b0c2aaaed7c2041ca56a86fda0f5fa609f486f9ba1a53872ac4475bd77c56ba`.

> 
> object NewType {
>   def apply(): NewType                               = new NewType
>   private val outNeighbors: Map[String, Set[String]] = Map("AST" -> Set("TYPE_ARGUMENT"), "REF" -> Set("TYPE_DECL"))
>   private val inNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE_DECL"),
>     "EVAL_TYPE" -> Set(
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CONTROL_STRUCTURE",
>       "IDENTIFIER",
>       "LITERAL",
>       "LOCAL",
>       "MEMBER",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 36 lexically identified declarations, including `NewType` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L10), `InsertionHelpers` (L32), `NewNodeInserter_Type_fullName` (L33), `insertNewNodeProperties` (L34), `dstCast` (L40), `seq` (L41), `nn` (L45), `NewNodeInserter_Type_name` (L58), `insertNewNodeProperties` (L59), `dstCast` (L65), `seq` (L66), `nn` (L70), `NewNodeInserter_Type_typeDeclFullName` (L83), `insertNewNodeProperties` (L84), `dstCast` (L90), `seq` (L91), `nn` (L95), `NewType` (L111), `StoredNodeType` (L112), `label` (L113), `isValidOutNeighbor` (L115). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewType.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b69d94b68bcfb885"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeArgument.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `735755c47b86f222b548a584d7e9a0703be69439`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeArgument.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5cef71817247a7c64c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeArgument.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeArgument.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e667ab558be4cef6037fab262d91ef0891ad385b04c9ac90dc993fc2c6b22a83`.

> 
> object NewTypeArgument {
>   def apply(): NewTypeArgument = new NewTypeArgument
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 58 lexically identified declarations, including `NewTypeArgument` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_TypeArgument_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_TypeArgument_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_TypeArgument_lineNumber` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_TypeArgument_offset` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeArgument.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2160e461fb86cfcd"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeDecl.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `440619dc20d510d35900964ac2b606644c7b4220`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeDecl.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-97802c3d759270b57b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeDecl.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeDecl.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `65a44c97615dcb7b91005d22b4bc7ba0d5b53eda47789a548eef33fe6daa6b41`.

> 
> object NewTypeDecl {
>   def apply(): NewTypeDecl = new NewTypeDecl
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 113 lexically identified declarations, including `NewTypeDecl` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_TypeDecl_aliasTypeFullName` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_TypeDecl_astParentFullName` (L2033), `insertNewNodeProperties` (L2034), `dstCast` (L2040), `seq` (L2041), `nn` (L2045), `NewNodeInserter_TypeDecl_astParentType` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_TypeDecl_code` (L2083), `insertNewNodeProperties` (L2084), `dstCast` (L2090), `seq` (L2091). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeDecl.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1bfee12bad4639a2"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeParameter.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1a802973c5bf81f87bdc1d320ca1b1513b132369`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeParameter.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-960830d6e58c58c327.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeParameter.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeParameter.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a20d2fee58be9b33d72dae8319f20708bcf41d6dd49abb894427b8479ddaed25`.

> 
> object NewTypeParameter {
>   def apply(): NewTypeParameter = new NewTypeParameter
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 64 lexically identified declarations, including `NewTypeParameter` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_TypeParameter_code` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_TypeParameter_columnNumber` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_TypeParameter_lineNumber` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_TypeParameter_name` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeParameter.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-abce84c3d966a07c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeRef.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7b9aaa6e1b9b5639816045a13cfe356d5e78f8c8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeRef.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-69c8c0b6990b7d5108.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeRef.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeRef.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `4305216d2a630deba30066758b229378289f4ede6b4b3065fe18728f2f7a3093`.

> 
> object NewTypeRef {
>   def apply(): NewTypeRef = new NewTypeRef
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 96 lexically identified declarations, including `NewTypeRef` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_TypeRef_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_TypeRef_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_TypeRef_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_TypeRef_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewTypeRef.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bdd9f6e8b039e7b5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewUnknown.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c3bb541e8b57e471f6d6b26d55324f72d81f4cfb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewUnknown.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e40ec0d0a4247c8b31.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewUnknown.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewUnknown.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c6a88ae0905a044adabf33437c982dd039bed575018a5a66d6c9188a1345ddf7`.

> 
> object NewUnknown {
>   def apply(): NewUnknown = new NewUnknown
>   private val outNeighbors: Map[String, Set[String]] = Map(
>     "ALIAS_OF" -> Set("TYPE"),
>     "ARGUMENT" -> Set(
>       "ANNOTATION",
>       "ANNOTATION_LITERAL",
>       "ANNOTATION_PARAMETER",
>       "ANNOTATION_PARAMETER_ASSIGN",
>       "ARRAY_INITIALIZER",
>       "BLOCK",
>       "CALL",
>       "CALL_REPR",
>       "CFG_NODE",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 108 lexically identified declarations, including `NewUnknown` (L7), `apply` (L8), `outNeighbors` (L9), `inNeighbors` (L1008), `InsertionHelpers` (L2003), `NewNodeInserter_Unknown_argumentIndex` (L2004), `insertNewNodeProperties` (L2005), `dstCast` (L2011), `seq` (L2012), `nn` (L2016), `NewNodeInserter_Unknown_argumentLabel` (L2029), `insertNewNodeProperties` (L2030), `dstCast` (L2036), `seq` (L2037), `nn` (L2041), `NewNodeInserter_Unknown_argumentName` (L2058), `insertNewNodeProperties` (L2059), `dstCast` (L2065), `seq` (L2066), `nn` (L2070), `NewNodeInserter_Unknown_code` (L2087), `insertNewNodeProperties` (L2088), `dstCast` (L2094), `seq` (L2095). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/NewUnknown.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c21d860bc30d06ba"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Return.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5c32df5e5f88dea9d1d8a673a3c9841c961397ca`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Return.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-393c904546795e629a.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Return.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Return.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `070b3dcc1217745ecbc97a3dfe720b57d555354bd4cebd6f991922f29986daf7`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait ReturnEMT extends AnyRef with ExpressionEMT
> 
> trait ReturnBase extends AbstractNode with ExpressionBase with StaticType[ReturnEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if ((-1: Int) != this.argumentIndex) res.put("ARGUMENT_INDEX", this.argumentIndex)
>     this.argumentLabel.foreach { p => res.put("ARGUMENT_LABEL", p) }
>     this.argumentName.foreach { p => res.put("ARGUMENT_NAME", p) }
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `ReturnEMT` (L10), `ReturnBase` (L12), `propertiesMap` (L14), `res` (L16), `Return` (L30), `Label` (L31), `unapply` (L32), `Return` (L89), `productElementName` (L95), `productElement` (L109), `productPrefix` (L123), `productArity` (L124), `canEqual` (L126). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Return.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7ef7e7252496826b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypes.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `342d65faf296444115ee27948ce9be28a77c668d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypes.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4cdba27f7c0f3a1367.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypes.scala, lines 2–16](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypes.scala#L2-L16). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2e43b52766a7d57a15448c2ef42873798d525903cb01b3cb4e39ba26949c2ae4`.

> 
> trait StaticType[+T]
> 
> trait AbstractNode extends flatgraph.DNodeOrNode with StaticType[AnyRef] with Product {
>   def label: String
> 
>   def properties: Map[String, Any] = {
>     scala.jdk.CollectionConverters.MapHasAsScala(propertiesMap).asScala.toMap
>   }
> 
>   /** TODO deprecate and phase out */
>   def propertiesMap: java.util.Map[String, Any]
> }
> 
> abstract class StoredNode(graph_4762: flatgraph.Graph, kind_4762: Short, seq_4762: Int)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 80 lexically identified declarations, including `StaticType` (L3), `AbstractNode` (L5), `label` (L6), `properties` (L8), `propertiesMap` (L13), `StoredNode` (L16), `_aliasOfOut` (L20), `_aliasOfIn` (L25), `_argumentOut` (L31), `_argumentIn` (L36), `_astOut` (L42), `_astIn` (L47), `_bindsOut` (L53), `_bindsIn` (L58), `_bindsToOut` (L64), `_bindsToIn` (L69), `_callOut` (L75), `_callIn` (L80), `_captureOut` (L86), `_captureIn` (L91), `_capturedByOut` (L97), `_capturedByIn` (L102), `_catchBodyOut` (L108), `_catchBodyIn` (L113). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypes.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-476c1f017dce68ed"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypesTraversals.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4035e1e011d9ce37250fae302b4cabb3c90cc054`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypesTraversals.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-801cf252ce575a8654.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypesTraversals.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypesTraversals.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `8f0562a8d615ffebb23b0175860cd0958525a473ad070d42f372395e00262ba3`.

> 
>   final def _aliasOfOut: Iterator[StoredNode] = iterator.flatMap(_._aliasOfOut)
>   final def _aliasOfIn: Iterator[StoredNode]  = iterator.flatMap(_._aliasOfIn)
> 
>   final def _argumentOut: Iterator[StoredNode] = iterator.flatMap(_._argumentOut)
>   final def _argumentIn: Iterator[StoredNode]  = iterator.flatMap(_._argumentIn)
> 
>   final def _astOut: Iterator[StoredNode] = iterator.flatMap(_._astOut)
>   final def _astIn: Iterator[StoredNode]  = iterator.flatMap(_._astIn)
> 
>   final def _bindsOut: Iterator[StoredNode] = iterator.flatMap(_._bindsOut)
>   final def _bindsIn: Iterator[StoredNode]  = iterator.flatMap(_._bindsIn)
> 
>   final def _bindsToOut: Iterator[StoredNode] = iterator.flatMap(_._bindsToOut)
>   final def _bindsToIn: Iterator[StoredNode]  = iterator.flatMap(_._bindsToIn)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 68 lexically identified declarations, including `_aliasOfOut` (L5), `_aliasOfIn` (L6), `_argumentOut` (L8), `_argumentIn` (L9), `_astOut` (L11), `_astIn` (L12), `_bindsOut` (L14), `_bindsIn` (L15), `_bindsToOut` (L17), `_bindsToIn` (L18), `_callOut` (L20), `_callIn` (L21), `_captureOut` (L23), `_captureIn` (L24), `_capturedByOut` (L26), `_capturedByIn` (L27), `_catchBodyOut` (L29), `_catchBodyIn` (L30), `_cdgOut` (L32), `_cdgIn` (L33), `_cfgOut` (L35), `_cfgIn` (L36), `_conditionOut` (L38), `_conditionIn` (L39). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/RootTypesTraversals.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b8748cc0f60b739c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Tag.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `08ce23b2b1eaa5e78fec3c46b393884bdb81d0b9`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Tag.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c1203142f7577753dd.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Tag.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Tag.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `22478cdddc3a8312e20fad52ae9976ac52f1b10aa0c7c30eafb02aa3476c0929`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait TagEMT extends AnyRef with HasNameEMT with HasValueEMT
> 
> trait TagBase extends AbstractNode with StaticType[TagEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.name) res.put("NAME", this.name)
>     if (("": String) != this.value) res.put("VALUE", this.value)
>     res
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `TagEMT` (L10), `TagBase` (L12), `propertiesMap` (L14), `res` (L16), `Tag` (L23), `Label` (L24), `unapply` (L25), `Tag` (L36), `productElementName` (L41), `productElement` (L48), `productPrefix` (L55), `productArity` (L56), `canEqual` (L58). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Tag.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5be79a77eced591e"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TagNodePair.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e883068d0e679396ec87aa322070d7e36b1a6483`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TagNodePair.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-59f4fbe246934136e2.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TagNodePair.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TagNodePair.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f1d8a40b462aad6616f731d1941509088758f869f8c612e92771c6e2523cc4f1`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait TagNodePairEMT extends AnyRef
> 
> trait TagNodePairBase extends AbstractNode with StaticType[TagNodePairEMT] {
>   def node: AbstractNode
>   def tag: TagBase
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     res.put("node", this.node)
>     res.put("tag", this.tag)
>     res


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 17 lexically identified declarations, including `TagNodePairEMT` (L10), `TagNodePairBase` (L12), `node` (L13), `tag` (L14), `propertiesMap` (L15), `res` (L17), `TagNodePair` (L24), `Label` (L25), `unapply` (L26), `TagNodePair` (L35), `node` (L39), `tag` (L46), `productElementName` (L54), `productElement` (L61), `productPrefix` (L68), `productArity` (L69), `canEqual` (L71). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TagNodePair.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-056f9bf0915902f3"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TemplateDom.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e3782b4cf7fa38dfb9e7099e7b545875a83f8f88`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TemplateDom.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c8618d328e5e70e289.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TemplateDom.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TemplateDom.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cef60c46d2ba69554cc40c5f6e4c621912756f416ced799a59472b1656049e87`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait TemplateDomEMT extends AnyRef with ExpressionEMT with HasNameEMT
> 
> trait TemplateDomBase extends AbstractNode with ExpressionBase with StaticType[TemplateDomEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if ((-1: Int) != this.argumentIndex) res.put("ARGUMENT_INDEX", this.argumentIndex)
>     this.argumentLabel.foreach { p => res.put("ARGUMENT_LABEL", p) }
>     this.argumentName.foreach { p => res.put("ARGUMENT_NAME", p) }
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `TemplateDomEMT` (L10), `TemplateDomBase` (L12), `propertiesMap` (L14), `res` (L16), `TemplateDom` (L31), `Label` (L32), `unapply` (L33), `TemplateDom` (L94), `productElementName` (L100), `productElement` (L115), `productPrefix` (L130), `productArity` (L131), `canEqual` (L133). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TemplateDom.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9fee641be4e9df79"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Type.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4d4e821ed46ff9f909a7293334f01939b55a37a7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Type.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-65799df3b1bccd3cfd.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Type.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Type.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `68c0162b82cdd3c90f810d8c48a0ba7637c1c75b9fc85a5c6e43e514a1d76e74`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait TypeEMT extends AnyRef with HasFullNameEMT with HasNameEMT with HasTypeDeclFullNameEMT
> 
> trait TypeBase extends AbstractNode with StaticType[TypeEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.fullName) res.put("FULL_NAME", this.fullName)
>     if (("<empty>": String) != this.name) res.put("NAME", this.name)
>     if (("<empty>": String) != this.typeDeclFullName) res.put("TYPE_DECL_FULL_NAME", this.typeDeclFullName)
>     res


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `TypeEMT` (L10), `TypeBase` (L12), `propertiesMap` (L14), `res` (L16), `Type` (L24), `Label` (L25), `unapply` (L26), `Type` (L42), `productElementName` (L47), `productElement` (L55), `productPrefix` (L63), `productArity` (L64), `canEqual` (L66). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Type.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7fadded77dce0a2d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeArgument.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `47807f13af6c1681578aab50edad983d63b790f1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeArgument.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-68037b197166b7c301.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeArgument.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeArgument.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1add6fe7d1e61c3db75e8c88af3730785a82a8af0e39fb794b31220d4477867f`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait TypeArgumentEMT extends AnyRef with AstNodeEMT
> 
> trait TypeArgumentBase extends AbstractNode with AstNodeBase with StaticType[TypeArgumentEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     this.lineNumber.foreach { p => res.put("LINE_NUMBER", p) }
>     this.offset.foreach { p => res.put("OFFSET", p) }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `TypeArgumentEMT` (L10), `TypeArgumentBase` (L12), `propertiesMap` (L14), `res` (L16), `TypeArgument` (L27), `Label` (L28), `unapply` (L29), `TypeArgument` (L63), `productElementName` (L69), `productElement` (L80), `productPrefix` (L91), `productArity` (L92), `canEqual` (L94). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeArgument.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cab1ecc9411d3231"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeDecl.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `52c63bd489505e87d7174b238c18b0ab6214a3c2`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeDecl.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c4e8255d5315dc2fc6.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeDecl.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeDecl.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `93eedd6c3681a34fa1f7ce4c951f11cf8bf189af0bbfb2384b28b5db3d39c71f`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait TypeDeclEMT
>     extends AnyRef
>     with AstNodeEMT
>     with HasAliasTypeFullNameEMT
>     with HasAstParentFullNameEMT
>     with HasAstParentTypeEMT
>     with HasFilenameEMT
>     with HasFullNameEMT
>     with HasGenericSignatureEMT
>     with HasInheritsFromTypeFullNameEMT
>     with HasIsExternalEMT


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 14 lexically identified declarations, including `TypeDeclEMT` (L10), `TypeDeclBase` (L23), `propertiesMap` (L25), `res` (L27), `tmpInheritsFromTypeFullName` (L36), `TypeDecl` (L48), `Label` (L49), `unapply` (L50), `TypeDecl` (L129), `productElementName` (L135), `productElement` (L155), `productPrefix` (L175), `productArity` (L176), `canEqual` (L178). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeDecl.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-76cb18a2342f68f5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeParameter.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `cc1a6311229c077028a9ccc5985c69e4992799bb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeParameter.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-b0f3c8a2f302daf6be.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeParameter.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeParameter.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `807d881b2dcd28e521f29de27cd7f54bff87c8345aa90cad66c1cd727a538167`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait TypeParameterEMT extends AnyRef with AstNodeEMT with HasNameEMT
> 
> trait TypeParameterBase extends AbstractNode with AstNodeBase with StaticType[TypeParameterEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*
>     val res = new java.util.HashMap[String, Any]()
>     if (("<empty>": String) != this.code) res.put("CODE", this.code)
>     this.columnNumber.foreach { p => res.put("COLUMN_NUMBER", p) }
>     this.lineNumber.foreach { p => res.put("LINE_NUMBER", p) }
>     if (("<empty>": String) != this.name) res.put("NAME", this.name)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 13 lexically identified declarations, including `TypeParameterEMT` (L10), `TypeParameterBase` (L12), `propertiesMap` (L14), `res` (L16), `TypeParameter` (L28), `Label` (L29), `unapply` (L30), `TypeParameter` (L68), `productElementName` (L74), `productElement` (L86), `productPrefix` (L98), `productArity` (L99), `canEqual` (L101). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeParameter.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b444a0d18163170a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeRef.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `a82e10a07801c6db377a9bac621ebfe8f5f2b331`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeRef.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-6de3dc50809f756066.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeRef.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeRef.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `361e82e3e094987c745bcdafa89b5870f469d7eab711110c96eecce1f2e55f0d`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait TypeRefEMT
>     extends AnyRef
>     with ExpressionEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT
> 
> trait TypeRefBase extends AbstractNode with ExpressionBase with StaticType[TypeRefEMT] {
> 
>   override def propertiesMap: java.util.Map[String, Any] = {
>     import io.shiftleft.codepropertygraph.generated.accessors.languagebootstrap.*


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TypeRefEMT` (L10), `TypeRefBase` (L17), `propertiesMap` (L19), `res` (L21), `tmpDynamicTypeHintFullName` (L27), `tmpPossibleTypes` (L33), `TypeRef` (L40), `Label` (L41), `unapply` (L42), `TypeRef` (L114), `productElementName` (L120), `productElement` (L137), `productPrefix` (L154), `productArity` (L155), `canEqual` (L157). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/TypeRef.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8e1339b44501968c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Unknown.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4c7961edf4cb0c64d61335cfdcf3ce1a9071783c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Unknown.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0788a99930b046a603.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Unknown.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Unknown.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b376b82164baaa6de2adb4656f0cef8a019fce8f2af1099184b0c3cffbee10e0`.

> 
> /** Node base type for compiletime-only checks to improve type safety. EMT stands for: "erased marker trait", i.e. it is
>   * erased at runtime
>   */
> trait UnknownEMT
>     extends AnyRef
>     with ExpressionEMT
>     with HasContainedRefEMT
>     with HasDynamicTypeHintFullNameEMT
>     with HasParserTypeNameEMT
>     with HasPossibleTypesEMT
>     with HasTypeFullNameEMT
> 
> trait UnknownBase extends AbstractNode with ExpressionBase with StaticType[UnknownEMT] {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `UnknownEMT` (L10), `UnknownBase` (L19), `propertiesMap` (L21), `res` (L23), `tmpDynamicTypeHintFullName` (L30), `tmpPossibleTypes` (L37), `Unknown` (L44), `Label` (L45), `unapply` (L46), `Unknown` (L127), `productElementName` (L133), `productElement` (L152), `productPrefix` (L171), `productArity` (L172), `canEqual` (L174). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/nodes/Unknown.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Use native Rust collections while preserving required CPG graph key/edge contracts.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals` -> module `domainClasses::src::main::generated::io::shiftleft::codepropertygraph::generated::traversals`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c29272595b076bd119.** [cpg/README.md, lines 50–61](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L50-L61). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Domain class generation
> 
> The Scala domain classes under `domainClasses/src/main/generated/` are generated from the schema defined in `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/`. They are checked into the repository, so you only need to regenerate them when the schema changes.
> 
> Generation is handled by the `sbt-flatgraph` plugin via the `generateDomainClasses` task in the `schema` subproject. It reads `CpgSchema.instance`, which composes all schema modules, and writes Scala source files to `domainClasses/src/main/generated/`.
> 
> The task is wired as a dependency of `domainClasses/compile` and `stage`, so it runs automatically when you compile or stage:
> 
> ```
> sbt compile   # triggers generateDomainClasses, then compiles
> sbt stage     # triggers generateDomainClasses, then packages
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve CPG functional and interchange contracts and exact module/file spelling; implement initialization and visibility with native Rust. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic and file excerpts establish CPG functional obligations. Rust libraries provide native implementation; no Go runtime is required.


<a id="file-05dc5a892973ea49"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `13b1fb40cd3b550fdb6618f5ded25bfb349df3f1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-531cd2971e19c0a008.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `4c607c9dbde9c9be381c2a16b468a7894a80647546eb1f4f9d4f93ec7af5ac7a`.

> 
> final class TraversalAnnotationBase[NodeType <: nodes.AnnotationBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to fullName property */
>   def fullName: Iterator[String] =
>     traversal.map(_.fullName)
> 
>   /** Traverse to nodes where the fullName matches the regular expression `value`
>     */
>   def fullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       fullNameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 29 lexically identified declarations, including `TraversalAnnotationBase` (L6), `fullName` (L10), `fullName` (L15), `matcher` (L19), `fullName` (L25), `matchers` (L26), `fullNameExact` (L32), `someNode` (L34), `fullNameExact` (L43), `someNode` (L47), `valueSet` (L54), `fullNameNot` (L61), `matcher` (L65), `fullNameNot` (L72), `matchers` (L73), `name` (L78), `name` (L83), `matcher` (L87), `name` (L93), `matchers` (L94), `nameExact` (L100), `someNode` (L102), `nameExact` (L111), `someNode` (L115). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1f4faee410feb92f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationliteralBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9a5fe6e7242f769d13cbf25b0d134872e76387e6`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationliteralBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-97cd961401f925d907.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationliteralBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationliteralBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9018adbb975e3d747333e76870940a31b1772e28cf05d0ba587ee5ffe0127ea8`.

> 
> final class TraversalAnnotationliteralBase[NodeType <: nodes.AnnotationLiteralBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       nameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalAnnotationliteralBase` (L6), `name` (L10), `name` (L15), `matcher` (L19), `name` (L25), `matchers` (L26), `nameExact` (L32), `someNode` (L34), `nameExact` (L43), `someNode` (L47), `valueSet` (L54), `nameNot` (L61), `matcher` (L65), `nameNot` (L72), `matchers` (L73). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationliteralBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-81c4a5d45f395304"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `ece3d4da1e942a22abb599dbaf275dd7d12946db`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-aeb3dc76c963bb4957.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterBase.scala, lines 5–8](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterBase.scala#L5-L8). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c7986d1f488d6460c39187347137a01da41ebf6d483acfb125415ea8fad6b3b6`.

> 
> final class TraversalAnnotationparameterBase[NodeType <: nodes.AnnotationParameterBase](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 2 lexically identified declarations, including `TraversalAnnotationparameterBase` (L6), `traversal` (L7). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f3d4c38a856b71e0"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterassignBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `3aa196f315f7f17dba227547d48ad0c99d99f206`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterassignBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-05def4827f10162a06.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterassignBase.scala, lines 5–8](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterassignBase.scala#L5-L8). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d339e796492495f4bb5ea8d948c21fef606bc67a3bb3512f291328c60856eed7`.

> 
> final class TraversalAnnotationparameterassignBase[NodeType <: nodes.AnnotationParameterAssignBase](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 2 lexically identified declarations, including `TraversalAnnotationparameterassignBase` (L6), `traversal` (L7). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAnnotationparameterassignBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-50d690684cef7a5b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalArrayinitializerBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `55e1772ab83fc00d10670c02e977d2e12f44b8d5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalArrayinitializerBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-61ca7172599979ec1c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalArrayinitializerBase.scala, lines 5–7](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalArrayinitializerBase.scala#L5-L7). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c32298e7e2f4cf3414a2df63cc620c9841e9b755d1e91b761a3cb6245db6d0d8`.

> 
> final class TraversalArrayinitializerBase[NodeType <: nodes.ArrayInitializerBase](val traversal: Iterator[NodeType])
>     extends AnyVal {}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 1 lexically identified declarations, including `TraversalArrayinitializerBase` (L6). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalArrayinitializerBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6bb1088a159d6f27"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAstnodeBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `afbb4f7514f82e759f4cceb0880ba80c7c9b7133`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAstnodeBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5c2d6f7d1fd07ace9d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAstnodeBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAstnodeBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9d96caf165798b2a49cac071aeb283e625e09c8f1cabe1a9539dc10e4ce74823`.

> 
> final class TraversalAstnodeBase[NodeType <: nodes.AstNodeBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to code property */
>   def code: Iterator[String] =
>     traversal.map(_.code)
> 
>   /** Traverse to nodes where the code matches the regular expression `value`
>     */
>   def code(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       codeExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.code).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 111 lexically identified declarations, including `TraversalAstnodeBase` (L6), `code` (L9), `code` (L14), `matcher` (L18), `code` (L24), `matchers` (L25), `codeExact` (L31), `someNode` (L33), `codeExact` (L42), `someNode` (L46), `valueSet` (L53), `codeNot` (L60), `matcher` (L64), `codeNot` (L71), `matchers` (L72), `columnNumber` (L77), `columnNumber` (L82), `tmp` (L84), `columnNumber` (L90), `columnNumberIfPresent` (L97), `columnNumber` (L102), `vset` (L103), `tmp` (L105), `columnNumberNot` (L111). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalAstnodeBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5777c0f85c9404a2"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBindingBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1ca5170fca8eec1bfd62de53bdccb561ae02bb6b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBindingBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-06168b62b877b8261f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBindingBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBindingBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `67b5f2614541b24d69429ba47c196c29e7ee379cdb75f5ded0b6d5198003e9ca`.

> 
> final class TraversalBindingBase[NodeType <: nodes.BindingBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to methodFullName property */
>   def methodFullName: Iterator[String] =
>     traversal.map(_.methodFullName)
> 
>   /** Traverse to nodes where the methodFullName matches the regular expression `value`
>     */
>   def methodFullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       methodFullNameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.methodFullName).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 43 lexically identified declarations, including `TraversalBindingBase` (L6), `methodFullName` (L9), `methodFullName` (L14), `matcher` (L18), `methodFullName` (L24), `matchers` (L25), `methodFullNameExact` (L31), `someNode` (L33), `methodFullNameExact` (L42), `someNode` (L46), `valueSet` (L53), `methodFullNameNot` (L60), `matcher` (L64), `methodFullNameNot` (L71), `matchers` (L72), `name` (L77), `name` (L82), `matcher` (L86), `name` (L92), `matchers` (L93), `nameExact` (L99), `someNode` (L101), `nameExact` (L110), `someNode` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBindingBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b6c0807231048aa5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBlockBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `3ec9307e966110d12379501d5a1e152610184792`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBlockBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-105af0b9a1b921789d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBlockBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBlockBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `92ae9fb2fc2f559c1bf78f56701e7a521629503b5dffcb19bfd708469ea82cf4`.

> 
> final class TraversalBlockBase[NodeType <: nodes.BlockBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to dynamicTypeHintFullName property */
>   def dynamicTypeHintFullName: Iterator[String] =
>     traversal.flatMap(_.dynamicTypeHintFullName)
> 
>   /** Traverse to possibleTypes property */
>   def possibleTypes: Iterator[String] =
>     traversal.flatMap(_.possibleTypes)
> 
>   /** Traverse to typeFullName property */
>   def typeFullName: Iterator[String] =
>     traversal.map(_.typeFullName)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 17 lexically identified declarations, including `TraversalBlockBase` (L6), `dynamicTypeHintFullName` (L9), `possibleTypes` (L13), `typeFullName` (L17), `typeFullName` (L22), `matcher` (L26), `typeFullName` (L32), `matchers` (L33), `typeFullNameExact` (L39), `someNode` (L41), `typeFullNameExact` (L50), `someNode` (L54), `valueSet` (L61), `typeFullNameNot` (L68), `matcher` (L72), `typeFullNameNot` (L79), `matchers` (L80). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalBlockBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2716ae1ae9aa8e9c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1523a8bac62031f7b1bb930c331b3253030f940c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-70338cfd5c8d4d07ff.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f27a8acaecf6e1cb58da751eb4cf9f10e87546a86fbed8a8ff870c975937b889`.

> 
> final class TraversalCallBase[NodeType <: nodes.CallBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to dispatchType property */
>   def dispatchType: Iterator[String] =
>     traversal.map(_.dispatchType)
> 
>   /** Traverse to nodes where the dispatchType matches the regular expression `value`
>     */
>   def dispatchType(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       dispatchTypeExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.dispatchType).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 64 lexically identified declarations, including `TraversalCallBase` (L6), `dispatchType` (L9), `dispatchType` (L14), `matcher` (L18), `dispatchType` (L24), `matchers` (L25), `dispatchTypeExact` (L31), `someNode` (L33), `dispatchTypeExact` (L42), `someNode` (L46), `valueSet` (L53), `dispatchTypeNot` (L60), `matcher` (L64), `dispatchTypeNot` (L71), `matchers` (L72), `dynamicTypeHintFullName` (L77), `methodFullName` (L81), `methodFullName` (L86), `matcher` (L90), `methodFullName` (L96), `matchers` (L97), `methodFullNameExact` (L103), `someNode` (L105), `methodFullNameExact` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e355d175d7e4226c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallreprBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `71990616dcfe6c44d90effea9475392d876d8985`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallreprBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c0aa95ffef6e18180f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallreprBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallreprBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `91ac17aeab0cede3b832d7723289e5d9b9156a608fd266573859a7e174a091a9`.

> 
> final class TraversalCallreprBase[NodeType <: nodes.CallReprBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       nameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.name).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 29 lexically identified declarations, including `TraversalCallreprBase` (L6), `name` (L9), `name` (L14), `matcher` (L18), `name` (L24), `matchers` (L25), `nameExact` (L31), `someNode` (L33), `nameExact` (L42), `someNode` (L46), `valueSet` (L53), `nameNot` (L60), `matcher` (L64), `nameNot` (L71), `matchers` (L72), `signature` (L77), `signature` (L82), `matcher` (L86), `signature` (L92), `matchers` (L93), `signatureExact` (L99), `someNode` (L101), `signatureExact` (L110), `someNode` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCallreprBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b675a7b40310f7aa"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCfgnodeBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0172f5b59ec44bb35b7c463867cfd74e58c7868b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCfgnodeBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f94afa50935debd247.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCfgnodeBase.scala, lines 5–6](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCfgnodeBase.scala#L5-L6). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0fc7acc3ee46cf95372deae26261ed4bdcaac0238514b4d6631927c5cbd10a3a`.

> 
> final class TraversalCfgnodeBase[NodeType <: nodes.CfgNodeBase](val traversal: Iterator[NodeType]) extends AnyVal {}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 1 lexically identified declarations, including `TraversalCfgnodeBase` (L6). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCfgnodeBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2dc36fc21eefaa1d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalClosurebindingBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8dd1f2eebbff6ad589e77d55cea1f8aa4250edf4`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalClosurebindingBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-97c4036c22936efe0e.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalClosurebindingBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalClosurebindingBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f70c7081bcb31f8cd99fcbaae9a8e46a33fcf64bba787f2c7a456ca96ff6528b`.

> 
> final class TraversalClosurebindingBase[NodeType <: nodes.ClosureBindingBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to closureBindingId property */
>   def closureBindingId: Iterator[String] =
>     traversal.flatMap(_.closureBindingId)
> 
>   /** Traverse to nodes where the closureBindingId matches the regular expression `value`
>     */
>   def closureBindingId(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       closureBindingIdExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 34 lexically identified declarations, including `TraversalClosurebindingBase` (L6), `closureBindingId` (L10), `closureBindingId` (L15), `matcher` (L19), `tmp` (L21), `closureBindingId` (L28), `matchers` (L29), `tmp` (L31), `closureBindingIdExact` (L37), `someNode` (L39), `tmp` (L45), `closureBindingIdExact` (L51), `valueSet` (L54), `tmp` (L56), `closureBindingIdNot` (L62), `matcher` (L66), `tmp` (L68), `closureBindingIdNot` (L75), `matchers` (L76), `tmp` (L78), `evaluationStrategy` (L83), `evaluationStrategy` (L88), `matcher` (L92), `evaluationStrategy` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalClosurebindingBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cb848ef76562270b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCommentBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `2cfb1d32028e1c43c6354c6b6df0b4de87e40984`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCommentBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e023089d75fc72248d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCommentBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCommentBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0643e43ebb0354e35e617791db41247c0cd273ec11ee7efe1171ff7896e3bf42`.

> 
> final class TraversalCommentBase[NodeType <: nodes.CommentBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to filename property */
>   def filename: Iterator[String] =
>     traversal.map(_.filename)
> 
>   /** Traverse to nodes where the filename matches the regular expression `value`
>     */
>   def filename(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       filenameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.filename).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalCommentBase` (L6), `filename` (L9), `filename` (L14), `matcher` (L18), `filename` (L24), `matchers` (L25), `filenameExact` (L31), `someNode` (L33), `filenameExact` (L42), `someNode` (L46), `valueSet` (L53), `filenameNot` (L60), `matcher` (L64), `filenameNot` (L71), `matchers` (L72). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalCommentBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-530a69d1bf260faf"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalConfigfileBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `440de53ac622366437a4b49f161733a178c88e39`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalConfigfileBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-80107d2dcdaa340eb1.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalConfigfileBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalConfigfileBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3a3c9547862c54ac04ac563244f9f6d44063c9bf82df53c822a4f818e028c6ef`.

> 
> final class TraversalConfigfileBase[NodeType <: nodes.ConfigFileBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to content property */
>   def content: Iterator[String] =
>     traversal.map(_.content)
> 
>   /** Traverse to nodes where the content matches the regular expression `value`
>     */
>   def content(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       contentExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 29 lexically identified declarations, including `TraversalConfigfileBase` (L6), `content` (L10), `content` (L15), `matcher` (L19), `content` (L25), `matchers` (L26), `contentExact` (L32), `someNode` (L34), `contentExact` (L43), `someNode` (L47), `valueSet` (L54), `contentNot` (L61), `matcher` (L65), `contentNot` (L72), `matchers` (L73), `name` (L78), `name` (L83), `matcher` (L87), `name` (L93), `matchers` (L94), `nameExact` (L100), `someNode` (L102), `nameExact` (L111), `someNode` (L115). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalConfigfileBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-84846f1fdbcebe55"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalControlstructureBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c0911074cffbf733992105d9bef736fc205a6da7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalControlstructureBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-baf432a9cf2030c99f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalControlstructureBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalControlstructureBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `fbeca86ce1b82f549b2dd3ded858f194138c090696913bd919fad632975a0433`.

> 
> final class TraversalControlstructureBase[NodeType <: nodes.ControlStructureBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to controlStructureType property */
>   def controlStructureType: Iterator[String] =
>     traversal.map(_.controlStructureType)
> 
>   /** Traverse to nodes where the controlStructureType matches the regular expression `value`
>     */
>   def controlStructureType(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       controlStructureTypeExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 29 lexically identified declarations, including `TraversalControlstructureBase` (L6), `controlStructureType` (L10), `controlStructureType` (L15), `matcher` (L19), `controlStructureType` (L25), `matchers` (L26), `controlStructureTypeExact` (L32), `someNode` (L34), `controlStructureTypeExact` (L43), `someNode` (L47), `valueSet` (L54), `controlStructureTypeNot` (L61), `matcher` (L65), `controlStructureTypeNot` (L72), `matchers` (L73), `parserTypeName` (L78), `parserTypeName` (L83), `matcher` (L87), `parserTypeName` (L93), `matchers` (L94), `parserTypeNameExact` (L100), `someNode` (L102), `parserTypeNameExact` (L111), `someNode` (L115). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalControlstructureBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3c3fe3accf20f630"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDeclarationBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9f7e353085dd30a447866805675d498264f2f3b7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDeclarationBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-94f413236bb3fb3ef9.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDeclarationBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDeclarationBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `27364868908ae56dd73a68160cdce814392f1ca7c6dcee556d1cd73299d6e1d6`.

> 
> final class TraversalDeclarationBase[NodeType <: nodes.DeclarationBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       nameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalDeclarationBase` (L6), `name` (L10), `name` (L15), `matcher` (L19), `name` (L25), `matchers` (L26), `nameExact` (L32), `someNode` (L34), `nameExact` (L43), `someNode` (L47), `valueSet` (L54), `nameNot` (L61), `matcher` (L65), `nameNot` (L72), `matchers` (L73). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDeclarationBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-692321d13532adef"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDependencyBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `091260c687e355f79cb42db949cee4e0625be99a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDependencyBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-27d0f994c3566ce866.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDependencyBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDependencyBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dd94f574c15e3a59da5485e84a00704b625485cacd3b0ee76cc08edbcb717b9e`.

> 
> final class TraversalDependencyBase[NodeType <: nodes.DependencyBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to dependencyGroupId property */
>   def dependencyGroupId: Iterator[String] =
>     traversal.flatMap(_.dependencyGroupId)
> 
>   /** Traverse to nodes where the dependencyGroupId matches the regular expression `value`
>     */
>   def dependencyGroupId(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       dependencyGroupIdExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 48 lexically identified declarations, including `TraversalDependencyBase` (L6), `dependencyGroupId` (L10), `dependencyGroupId` (L15), `matcher` (L19), `tmp` (L21), `dependencyGroupId` (L28), `matchers` (L29), `tmp` (L31), `dependencyGroupIdExact` (L37), `someNode` (L39), `tmp` (L45), `dependencyGroupIdExact` (L51), `valueSet` (L54), `tmp` (L56), `dependencyGroupIdNot` (L62), `matcher` (L66), `tmp` (L68), `dependencyGroupIdNot` (L75), `matchers` (L76), `tmp` (L78), `name` (L83), `name` (L88), `matcher` (L92), `name` (L98). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalDependencyBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-783bb50ae5c3d07c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalExpressionBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `043acce79714d8b99b9f8c1a90fb7b919ba3baca`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalExpressionBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d545fed4ded619d2d6.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalExpressionBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalExpressionBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `69cd0e12927a1ad97d85c2f12378c1465b206f02cccecde53963a170130c8bae`.

> 
> final class TraversalExpressionBase[NodeType <: nodes.ExpressionBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to argumentIndex property */
>   def argumentIndex: Iterator[Int] =
>     traversal.map(_.argumentIndex)
> 
>   /** Traverse to nodes where the argumentIndex equals the given `value`
>     */
>   def argumentIndex(value: Int): Iterator[NodeType] =
>     traversal.filter { _.argumentIndex == value }
> 
>   /** Traverse to nodes where the argumentIndex equals the given `value`, or no results if `value` is None
>     */


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 51 lexically identified declarations, including `TraversalExpressionBase` (L6), `argumentIndex` (L10), `argumentIndex` (L15), `argumentIndex` (L20), `argumentIndex` (L25), `vset` (L26), `argumentIndexNot` (L32), `argumentIndexNot` (L37), `vset` (L38), `argumentIndexGt` (L44), `argumentIndexGte` (L49), `argumentIndexLt` (L54), `argumentIndexLte` (L59), `argumentLabel` (L63), `argumentLabel` (L68), `matcher` (L72), `tmp` (L74), `argumentLabel` (L81), `matchers` (L82), `tmp` (L84), `argumentLabelExact` (L90), `someNode` (L92), `tmp` (L98), `argumentLabelExact` (L104). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalExpressionBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4ec992a9a613fc78"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFieldidentifierBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `71949cef57a608c351ca1de1fc7c6db81cc8f80c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFieldidentifierBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ff6b1063942c672f62.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFieldidentifierBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFieldidentifierBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7a8dec1dbc46b9ab38aa7f85eeeb295a6e4567e905a28daf8612ef7181a5b32b`.

> 
> final class TraversalFieldidentifierBase[NodeType <: nodes.FieldIdentifierBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to canonicalName property */
>   def canonicalName: Iterator[String] =
>     traversal.map(_.canonicalName)
> 
>   /** Traverse to nodes where the canonicalName matches the regular expression `value`
>     */
>   def canonicalName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       canonicalNameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalFieldidentifierBase` (L6), `canonicalName` (L10), `canonicalName` (L15), `matcher` (L19), `canonicalName` (L25), `matchers` (L26), `canonicalNameExact` (L32), `someNode` (L34), `canonicalNameExact` (L43), `someNode` (L47), `valueSet` (L54), `canonicalNameNot` (L61), `matcher` (L65), `canonicalNameNot` (L72), `matchers` (L73). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFieldidentifierBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-406fe479e51deba4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFileBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `419275e7b7307131291add56c5349d3ff6ea4ce3`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFileBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-31058d3d42175700c5.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFileBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFileBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1632b6ec49a4d6600606fa443a323abd45f8852844aecceb531c3c69ebfcf853`.

> 
> final class TraversalFileBase[NodeType <: nodes.FileBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to content property */
>   def content: Iterator[String] =
>     traversal.map(_.content)
> 
>   /** Traverse to nodes where the content matches the regular expression `value`
>     */
>   def content(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       contentExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.content).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 48 lexically identified declarations, including `TraversalFileBase` (L6), `content` (L9), `content` (L14), `matcher` (L18), `content` (L24), `matchers` (L25), `contentExact` (L31), `someNode` (L33), `contentExact` (L42), `someNode` (L46), `valueSet` (L53), `contentNot` (L60), `matcher` (L64), `contentNot` (L71), `matchers` (L72), `hash` (L77), `hash` (L82), `matcher` (L86), `tmp` (L88), `hash` (L95), `matchers` (L96), `tmp` (L98), `hashExact` (L104), `someNode` (L106). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFileBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ef30604316386adf"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFindingBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `30f185e17f5cec606239e742624dac928241c7b9`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFindingBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-cf01ee034b5354aa71.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFindingBase.scala, lines 5–12](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFindingBase.scala#L5-L12). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `4917b439ec6f6876f80559b7023e340cc5ae73240485be300fff356ff9cdfebb`.

> 
> final class TraversalFindingBase[NodeType <: nodes.FindingBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to evidenceDescription property */
>   def evidenceDescription: Iterator[String] =
>     traversal.flatMap(_.evidenceDescription)
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 2 lexically identified declarations, including `TraversalFindingBase` (L6), `evidenceDescription` (L9). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalFindingBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c39710bb60f3e5c3"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalIdentifierBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `51e5221d01402d9e6ec9045e0bf61e1d9027239b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalIdentifierBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-456b19b005f49ef332.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalIdentifierBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalIdentifierBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `31ee6e0ba484f9e959b6565d9b882c19a02ad112c234422939bde31179ad4bbb`.

> 
> final class TraversalIdentifierBase[NodeType <: nodes.IdentifierBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to dynamicTypeHintFullName property */
>   def dynamicTypeHintFullName: Iterator[String] =
>     traversal.flatMap(_.dynamicTypeHintFullName)
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 31 lexically identified declarations, including `TraversalIdentifierBase` (L6), `dynamicTypeHintFullName` (L10), `name` (L14), `name` (L19), `matcher` (L23), `name` (L29), `matchers` (L30), `nameExact` (L36), `someNode` (L38), `nameExact` (L47), `someNode` (L51), `valueSet` (L58), `nameNot` (L65), `matcher` (L69), `nameNot` (L76), `matchers` (L77), `possibleTypes` (L82), `typeFullName` (L86), `typeFullName` (L91), `matcher` (L95), `typeFullName` (L101), `matchers` (L102), `typeFullNameExact` (L108), `someNode` (L110). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalIdentifierBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7f177c279d3cc28e"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalImportBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `98a8d2ae2109da7bcbe5c0d05322335712e51b7a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalImportBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4a291879d0326787c1.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalImportBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalImportBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `99d4b9ef6302214afbac4cf89002ba210ac16c5477971313ac5307c5ef595161`.

> 
> final class TraversalImportBase[NodeType <: nodes.ImportBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to explicitAs property */
>   def explicitAs: Iterator[Boolean] =
>     traversal.flatMap(_.explicitAs)
> 
>   /** Traverse to nodes where the explicitAs equals the given `value`
>     */
>   def explicitAs(value: Boolean): Iterator[NodeType] =
>     traversal.filter { node => node.explicitAs.isDefined && node.explicitAs.get == value }
> 
>   /** Traverse to importedAs property */
>   def importedAs: Iterator[String] =
>     traversal.flatMap(_.importedAs)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 47 lexically identified declarations, including `TraversalImportBase` (L6), `explicitAs` (L9), `explicitAs` (L14), `importedAs` (L18), `importedAs` (L23), `matcher` (L27), `tmp` (L29), `importedAs` (L36), `matchers` (L37), `tmp` (L39), `importedAsExact` (L45), `someNode` (L47), `tmp` (L53), `importedAsExact` (L59), `valueSet` (L62), `tmp` (L64), `importedAsNot` (L70), `matcher` (L74), `tmp` (L76), `importedAsNot` (L83), `matchers` (L84), `tmp` (L86), `importedEntity` (L91), `importedEntity` (L96). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalImportBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d1932ad3a206552a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumplabelBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `28af5a8242fa930a619ac0b0c65b666ba971c5f5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumplabelBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b6d95450b41a056ae7.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumplabelBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumplabelBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `26e527918baa6433d604d1e37fafc1b7f657002ba6906831e76e25189a4896a8`.

> 
> final class TraversalJumplabelBase[NodeType <: nodes.JumpLabelBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       nameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.name).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 29 lexically identified declarations, including `TraversalJumplabelBase` (L6), `name` (L9), `name` (L14), `matcher` (L18), `name` (L24), `matchers` (L25), `nameExact` (L31), `someNode` (L33), `nameExact` (L42), `someNode` (L46), `valueSet` (L53), `nameNot` (L60), `matcher` (L64), `nameNot` (L71), `matchers` (L72), `parserTypeName` (L77), `parserTypeName` (L82), `matcher` (L86), `parserTypeName` (L92), `matchers` (L93), `parserTypeNameExact` (L99), `someNode` (L101), `parserTypeNameExact` (L110), `someNode` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumplabelBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-901a39eb3ce012c1"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumptargetBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8c09b31a6cd3e952d9f3c2d04bc544fda51e19cb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumptargetBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-34be18c9b390ed86dc.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumptargetBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumptargetBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3eb95e381e3b340903ccb281a98ef9598cb3ff57d8b1421ceb6d0b5a00785c1f`.

> 
> final class TraversalJumptargetBase[NodeType <: nodes.JumpTargetBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to argumentIndex property */
>   def argumentIndex: Iterator[Int] =
>     traversal.map(_.argumentIndex)
> 
>   /** Traverse to nodes where the argumentIndex equals the given `value`
>     */
>   def argumentIndex(value: Int): Iterator[NodeType] =
>     traversal.filter { _.argumentIndex == value }
> 
>   /** Traverse to nodes where the argumentIndex equals the given `value`, or no results if `value` is None
>     */


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 41 lexically identified declarations, including `TraversalJumptargetBase` (L6), `argumentIndex` (L10), `argumentIndex` (L15), `argumentIndex` (L20), `argumentIndex` (L25), `vset` (L26), `argumentIndexNot` (L32), `argumentIndexNot` (L37), `vset` (L38), `argumentIndexGt` (L44), `argumentIndexGte` (L49), `argumentIndexLt` (L54), `argumentIndexLte` (L59), `name` (L63), `name` (L68), `matcher` (L72), `name` (L78), `matchers` (L79), `nameExact` (L85), `someNode` (L87), `nameExact` (L96), `someNode` (L100), `valueSet` (L107), `nameNot` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalJumptargetBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3eb3ff82b9a45440"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalKeyvaluepairBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9e22548220ddc6d739840e7f51ebf440fcd43356`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalKeyvaluepairBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1040019315bf85c92d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalKeyvaluepairBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalKeyvaluepairBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f7dbe0f644f7d8396503b45ba6c37cc5b26b5b9b361276b61ec011bd2c77fc51`.

> 
> final class TraversalKeyvaluepairBase[NodeType <: nodes.KeyValuePairBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to key property */
>   def key: Iterator[String] =
>     traversal.map(_.key)
> 
>   /** Traverse to nodes where the key matches the regular expression `value`
>     */
>   def key(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       keyExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 29 lexically identified declarations, including `TraversalKeyvaluepairBase` (L6), `key` (L10), `key` (L15), `matcher` (L19), `key` (L25), `matchers` (L26), `keyExact` (L32), `someNode` (L34), `keyExact` (L43), `someNode` (L47), `valueSet` (L54), `keyNot` (L61), `matcher` (L65), `keyNot` (L72), `matchers` (L73), `value` (L78), `value` (L83), `matcher` (L87), `value` (L93), `matchers` (L94), `valueExact` (L100), `someNode` (L102), `valueExact` (L111), `someNode` (L115). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalKeyvaluepairBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3fb2022285c70b41"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLiteralBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `602141e473a69d47721006af2babe257ae5ddd4c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLiteralBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9ec50bb34a13b5e769.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLiteralBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLiteralBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9ad563062882ae9edde818634a025c9d29e47791277fe4af1eab39ad04eb7972`.

> 
> final class TraversalLiteralBase[NodeType <: nodes.LiteralBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to dynamicTypeHintFullName property */
>   def dynamicTypeHintFullName: Iterator[String] =
>     traversal.flatMap(_.dynamicTypeHintFullName)
> 
>   /** Traverse to possibleTypes property */
>   def possibleTypes: Iterator[String] =
>     traversal.flatMap(_.possibleTypes)
> 
>   /** Traverse to typeFullName property */
>   def typeFullName: Iterator[String] =
>     traversal.map(_.typeFullName)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 17 lexically identified declarations, including `TraversalLiteralBase` (L6), `dynamicTypeHintFullName` (L9), `possibleTypes` (L13), `typeFullName` (L17), `typeFullName` (L22), `matcher` (L26), `typeFullName` (L32), `matchers` (L33), `typeFullNameExact` (L39), `someNode` (L41), `typeFullNameExact` (L50), `someNode` (L54), `valueSet` (L61), `typeFullNameNot` (L68), `matcher` (L72), `typeFullNameNot` (L79), `matchers` (L80). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLiteralBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c25c86ef2f44f9be"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLocalBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `ef5d50094d018e5e36865eeebe3faa297aa4b578`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLocalBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b38eb5aa64ad27d9bf.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLocalBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLocalBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `43f99323bd50eda196c57cc3c61480c6acd88e78c07b3aab9bfc1dcb213c03dc`.

> 
> final class TraversalLocalBase[NodeType <: nodes.LocalBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to closureBindingId property */
>   def closureBindingId: Iterator[String] =
>     traversal.flatMap(_.closureBindingId)
> 
>   /** Traverse to nodes where the closureBindingId matches the regular expression `value`
>     */
>   def closureBindingId(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       closureBindingIdExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item =>


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 50 lexically identified declarations, including `TraversalLocalBase` (L6), `closureBindingId` (L9), `closureBindingId` (L14), `matcher` (L18), `tmp` (L20), `closureBindingId` (L27), `matchers` (L28), `tmp` (L30), `closureBindingIdExact` (L36), `someNode` (L38), `tmp` (L44), `closureBindingIdExact` (L50), `valueSet` (L53), `tmp` (L55), `closureBindingIdNot` (L61), `matcher` (L65), `tmp` (L67), `closureBindingIdNot` (L74), `matchers` (L75), `tmp` (L77), `dynamicTypeHintFullName` (L82), `genericSignature` (L86), `genericSignature` (L91), `matcher` (L95). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalLocalBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2710d6718ed1051f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMemberBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e20a9e1310fd65350b6c56bfecbf349a346eca76`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMemberBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-593f804974a098ab03.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMemberBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMemberBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `668af7da83c7b901b50789e34991388db8bae2cc5e53e6a334846d5a0c56d3b5`.

> 
> final class TraversalMemberBase[NodeType <: nodes.MemberBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to astParentFullName property */
>   def astParentFullName: Iterator[String] =
>     traversal.map(_.astParentFullName)
> 
>   /** Traverse to nodes where the astParentFullName matches the regular expression `value`
>     */
>   def astParentFullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       astParentFullNameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.astParentFullName).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 59 lexically identified declarations, including `TraversalMemberBase` (L6), `astParentFullName` (L9), `astParentFullName` (L14), `matcher` (L18), `astParentFullName` (L24), `matchers` (L25), `astParentFullNameExact` (L31), `someNode` (L33), `astParentFullNameExact` (L42), `someNode` (L46), `valueSet` (L53), `astParentFullNameNot` (L60), `matcher` (L64), `astParentFullNameNot` (L71), `matchers` (L72), `astParentType` (L77), `astParentType` (L82), `matcher` (L86), `astParentType` (L92), `matchers` (L93), `astParentTypeExact` (L99), `someNode` (L101), `astParentTypeExact` (L110), `someNode` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMemberBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0e37025107c48409"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMetadataBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b318046cc11aded00d534a9e05d1d4a977ce9e76`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMetadataBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-20329e3cb86a17871e.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMetadataBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMetadataBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `307edd6d7f56a34c047c1437dd063e6edef21a1ed805f3a8bc818e8e72e1deb3`.

> 
> final class TraversalMetadataBase[NodeType <: nodes.MetaDataBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to hash property */
>   def hash: Iterator[String] =
>     traversal.flatMap(_.hash)
> 
>   /** Traverse to nodes where the hash matches the regular expression `value`
>     */
>   def hash(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       hashExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item =>


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 63 lexically identified declarations, including `TraversalMetadataBase` (L6), `hash` (L9), `hash` (L14), `matcher` (L18), `tmp` (L20), `hash` (L27), `matchers` (L28), `tmp` (L30), `hashExact` (L36), `someNode` (L38), `tmp` (L44), `hashExact` (L50), `valueSet` (L53), `tmp` (L55), `hashNot` (L61), `matcher` (L65), `tmp` (L67), `hashNot` (L74), `matchers` (L75), `tmp` (L77), `language` (L82), `language` (L87), `matcher` (L91), `language` (L97). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMetadataBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-92489597714c5f80"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b343bc128714b2a7ec812abc1582ca80cf3af640`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-62efadf1518a0514d1.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2a515eb29505ef26ec47916acfbdba7e8559d87dbb86fc063687f9b0656a57bb`.

> 
> final class TraversalMethodBase[NodeType <: nodes.MethodBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to astParentFullName property */
>   def astParentFullName: Iterator[String] =
>     traversal.map(_.astParentFullName)
> 
>   /** Traverse to nodes where the astParentFullName matches the regular expression `value`
>     */
>   def astParentFullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       astParentFullNameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.astParentFullName).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 148 lexically identified declarations, including `TraversalMethodBase` (L6), `astParentFullName` (L9), `astParentFullName` (L14), `matcher` (L18), `astParentFullName` (L24), `matchers` (L25), `astParentFullNameExact` (L31), `someNode` (L33), `astParentFullNameExact` (L42), `someNode` (L46), `valueSet` (L53), `astParentFullNameNot` (L60), `matcher` (L64), `astParentFullNameNot` (L71), `matchers` (L72), `astParentType` (L77), `astParentType` (L82), `matcher` (L86), `astParentType` (L92), `matchers` (L93), `astParentTypeExact` (L99), `someNode` (L101), `astParentTypeExact` (L110), `someNode` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e00c463fb46ef960"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameterinBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `262517978ee4e3d40fb6612702e2fabb88b4d732`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameterinBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-12bfcdcfa260c372ab.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameterinBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameterinBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3a18f07abd2b5a1c3edd2efac854009a10b5ea7e44f2c638b7a21e068a42e89b`.

> 
> final class TraversalMethodparameterinBase[NodeType <: nodes.MethodParameterInBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to closureBindingId property */
>   def closureBindingId: Iterator[String] =
>     traversal.flatMap(_.closureBindingId)
> 
>   /** Traverse to nodes where the closureBindingId matches the regular expression `value`
>     */
>   def closureBindingId(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       closureBindingIdExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 64 lexically identified declarations, including `TraversalMethodparameterinBase` (L6), `closureBindingId` (L10), `closureBindingId` (L15), `matcher` (L19), `tmp` (L21), `closureBindingId` (L28), `matchers` (L29), `tmp` (L31), `closureBindingIdExact` (L37), `someNode` (L39), `tmp` (L45), `closureBindingIdExact` (L51), `valueSet` (L54), `tmp` (L56), `closureBindingIdNot` (L62), `matcher` (L66), `tmp` (L68), `closureBindingIdNot` (L75), `matchers` (L76), `tmp` (L78), `dynamicTypeHintFullName` (L83), `evaluationStrategy` (L87), `evaluationStrategy` (L92), `matcher` (L96). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameterinBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8f7b9da0a88ca7b4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameteroutBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c38fa18a521d711333bd8b7a836e5fbd69991fe1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameteroutBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-95ba9c87ea00293719.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameteroutBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameteroutBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1146faa586d1446379a69aeeaac3140c4cc26a2edf90b886d5ee3e83e738b487`.

> 
> final class TraversalMethodparameteroutBase[NodeType <: nodes.MethodParameterOutBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to evaluationStrategy property */
>   def evaluationStrategy: Iterator[String] =
>     traversal.map(_.evaluationStrategy)
> 
>   /** Traverse to nodes where the evaluationStrategy matches the regular expression `value`
>     */
>   def evaluationStrategy(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       evaluationStrategyExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 43 lexically identified declarations, including `TraversalMethodparameteroutBase` (L6), `evaluationStrategy` (L10), `evaluationStrategy` (L15), `matcher` (L19), `evaluationStrategy` (L25), `matchers` (L26), `evaluationStrategyExact` (L32), `someNode` (L34), `evaluationStrategyExact` (L43), `someNode` (L47), `valueSet` (L54), `evaluationStrategyNot` (L61), `matcher` (L65), `evaluationStrategyNot` (L72), `matchers` (L73), `index` (L78), `index` (L83), `index` (L88), `index` (L93), `vset` (L94), `indexNot` (L100), `indexNot` (L105), `vset` (L106), `indexGt` (L112). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodparameteroutBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-abd817525014d20e"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodrefBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `6320a6896694a9178184c554d4902de8f55c993a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodrefBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5ac11a983fd1a52412.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodrefBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodrefBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `fbc1cf71ffc3ce55c80267180925c9b3fda8b9ca3e0f850fc58fbbd5e3ec9b61`.

> 
> final class TraversalMethodrefBase[NodeType <: nodes.MethodRefBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to dynamicTypeHintFullName property */
>   def dynamicTypeHintFullName: Iterator[String] =
>     traversal.flatMap(_.dynamicTypeHintFullName)
> 
>   /** Traverse to methodFullName property */
>   def methodFullName: Iterator[String] =
>     traversal.map(_.methodFullName)
> 
>   /** Traverse to nodes where the methodFullName matches the regular expression `value`
>     */
>   def methodFullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 31 lexically identified declarations, including `TraversalMethodrefBase` (L6), `dynamicTypeHintFullName` (L9), `methodFullName` (L13), `methodFullName` (L18), `matcher` (L22), `methodFullName` (L28), `matchers` (L29), `methodFullNameExact` (L35), `someNode` (L37), `methodFullNameExact` (L46), `someNode` (L50), `valueSet` (L57), `methodFullNameNot` (L64), `matcher` (L68), `methodFullNameNot` (L75), `matchers` (L76), `possibleTypes` (L81), `typeFullName` (L85), `typeFullName` (L90), `matcher` (L94), `typeFullName` (L100), `matchers` (L101), `typeFullNameExact` (L107), `someNode` (L109). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodrefBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f17fcc39c5b5f7d3"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodreturnBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `313d94be04de618e1a84eb76eb6706425e18b2bf`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodreturnBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ebf7a8a6bd12ce12b3.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodreturnBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodreturnBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1924962f24d1bded585356bae1068cc99facce35fc4aa19222b97722907130c5`.

> 
> final class TraversalMethodreturnBase[NodeType <: nodes.MethodReturnBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to dynamicTypeHintFullName property */
>   def dynamicTypeHintFullName: Iterator[String] =
>     traversal.flatMap(_.dynamicTypeHintFullName)
> 
>   /** Traverse to evaluationStrategy property */
>   def evaluationStrategy: Iterator[String] =
>     traversal.map(_.evaluationStrategy)
> 
>   /** Traverse to nodes where the evaluationStrategy matches the regular expression `value`
>     */
>   def evaluationStrategy(pattern: String): Iterator[NodeType] =


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 31 lexically identified declarations, including `TraversalMethodreturnBase` (L6), `dynamicTypeHintFullName` (L10), `evaluationStrategy` (L14), `evaluationStrategy` (L19), `matcher` (L23), `evaluationStrategy` (L29), `matchers` (L30), `evaluationStrategyExact` (L36), `someNode` (L38), `evaluationStrategyExact` (L47), `someNode` (L51), `valueSet` (L58), `evaluationStrategyNot` (L65), `matcher` (L69), `evaluationStrategyNot` (L76), `matchers` (L77), `possibleTypes` (L82), `typeFullName` (L86), `typeFullName` (L91), `matcher` (L95), `typeFullName` (L101), `matchers` (L102), `typeFullNameExact` (L108), `someNode` (L110). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalMethodreturnBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-eaf12ec4b2bac85d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalModifierBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0a720323e5b5355072ac995ee1d3c6e31f9252e5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalModifierBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bf0aadbe1dae77135a.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalModifierBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalModifierBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a81ef33dcbd9267d993481ad779113040a4a592d1f37bb6cd4e9e946f54e2438`.

> 
> final class TraversalModifierBase[NodeType <: nodes.ModifierBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to modifierType property */
>   def modifierType: Iterator[String] =
>     traversal.map(_.modifierType)
> 
>   /** Traverse to nodes where the modifierType matches the regular expression `value`
>     */
>   def modifierType(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       modifierTypeExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.modifierType).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalModifierBase` (L6), `modifierType` (L9), `modifierType` (L14), `matcher` (L18), `modifierType` (L24), `matchers` (L25), `modifierTypeExact` (L31), `someNode` (L33), `modifierTypeExact` (L42), `someNode` (L46), `valueSet` (L53), `modifierTypeNot` (L60), `matcher` (L64), `modifierTypeNot` (L71), `matchers` (L72). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalModifierBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c6546bea8f5f4f9b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0248be1bb71d773c1bc29efaf33a606bda852aa0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d75c0798dbad0a1be7.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `fe2b5f7b5d2160199371e5c3e388a9521016123d43b2501e229f333b6fa162b4`.

> 
> final class TraversalNamespaceBase[NodeType <: nodes.NamespaceBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       nameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.name).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalNamespaceBase` (L6), `name` (L9), `name` (L14), `matcher` (L18), `name` (L24), `matchers` (L25), `nameExact` (L31), `someNode` (L33), `nameExact` (L42), `someNode` (L46), `valueSet` (L53), `nameNot` (L60), `matcher` (L64), `nameNot` (L71), `matchers` (L72). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d7f1285171958d7f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceblockBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1dc0e9dea201eefc9ed9d2fed0e58b6b2e4bdc76`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceblockBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3be7d109b5250c5441.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceblockBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceblockBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `37345a2d2b48ea673217c376e457c13e5c24f177a080fe5c9d229f2d31615244`.

> 
> final class TraversalNamespaceblockBase[NodeType <: nodes.NamespaceBlockBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to filename property */
>   def filename: Iterator[String] =
>     traversal.map(_.filename)
> 
>   /** Traverse to nodes where the filename matches the regular expression `value`
>     */
>   def filename(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       filenameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 43 lexically identified declarations, including `TraversalNamespaceblockBase` (L6), `filename` (L10), `filename` (L15), `matcher` (L19), `filename` (L25), `matchers` (L26), `filenameExact` (L32), `someNode` (L34), `filenameExact` (L43), `someNode` (L47), `valueSet` (L54), `filenameNot` (L61), `matcher` (L65), `filenameNot` (L72), `matchers` (L73), `fullName` (L78), `fullName` (L83), `matcher` (L87), `fullName` (L93), `matchers` (L94), `fullNameExact` (L100), `someNode` (L102), `fullNameExact` (L111), `someNode` (L115). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalNamespaceblockBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-04b8f77e9dceb67b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAliasTypeFullName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `86471f4af447034adb4d88e7f7f7328b5c4a2ad8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAliasTypeFullName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c5c3ba0aa368412b20.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAliasTypeFullName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAliasTypeFullName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `600e6828c9bcd24eb040770516802e4c4d2865b2a380467a8b210f6329e39830`.

> 
> final class TraversalPropertyAliasTypeFullName[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasAliasTypeFullNameEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to aliasTypeFullName property */
>   def aliasTypeFullName: Iterator[String] =
>     traversal.flatMap(_.aliasTypeFullName)
> 
>   /** Traverse to nodes where the aliasTypeFullName matches the regular expression `value`
>     */
>   def aliasTypeFullName(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       aliasTypeFullNameExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `TraversalPropertyAliasTypeFullName` (L6), `aliasTypeFullName` (L12), `aliasTypeFullName` (L17), `matcher` (L21), `tmp` (L23), `aliasTypeFullName` (L30), `matchers` (L31), `tmp` (L33), `aliasTypeFullNameExact` (L39), `someNode` (L41), `tmp` (L47), `aliasTypeFullNameExact` (L53), `valueSet` (L56), `tmp` (L58), `aliasTypeFullNameNot` (L64), `matcher` (L68), `tmp` (L70), `aliasTypeFullNameNot` (L77), `matchers` (L78), `tmp` (L80). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAliasTypeFullName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e2951fb8b52cddf7"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentIndex.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9e26684f834d342d66b6a6caad600a48a857737a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentIndex.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-eb49fb9c6b11cb9b2a.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentIndex.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentIndex.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2d6b043b6e5df7c589cb3c4ac1d0155f2ca9dbfc2bbfc6dedf75ee2c585454e6`.

> 
> final class TraversalPropertyArgumentIndex[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasArgumentIndexEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to argumentIndex property */
>   def argumentIndex: Iterator[Int] =
>     traversal.map(_.argumentIndex)
> 
>   /** Traverse to nodes where the argumentIndex equals the given `value`
>     */
>   def argumentIndex(value: Int): Iterator[NodeType] =
>     traversal.filter { _.argumentIndex == value }
> 
>   /** Traverse to nodes where the argumentIndex equals the given `value`, or no results if `value` is None


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 14 lexically identified declarations, including `TraversalPropertyArgumentIndex` (L6), `traversal` (L7), `argumentIndex` (L11), `argumentIndex` (L16), `argumentIndex` (L21), `argumentIndex` (L26), `vset` (L27), `argumentIndexNot` (L33), `argumentIndexNot` (L38), `vset` (L39), `argumentIndexGt` (L45), `argumentIndexGte` (L50), `argumentIndexLt` (L55), `argumentIndexLte` (L60). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentIndex.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0834ec6e060866a9"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentLabel.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `643ac081e479c44f7ad2e3f919a48ab5743e5376`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentLabel.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-63358d99d5b8436a06.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentLabel.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentLabel.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0b4909a9951a3a39ced44a8ae06bdc7da52b6bf6725cdf7081e7af071117c3ae`.

> 
> final class TraversalPropertyArgumentLabel[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasArgumentLabelEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to argumentLabel property */
>   def argumentLabel: Iterator[String] =
>     traversal.flatMap(_.argumentLabel)
> 
>   /** Traverse to nodes where the argumentLabel matches the regular expression `value`
>     */
>   def argumentLabel(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       argumentLabelExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 21 lexically identified declarations, including `TraversalPropertyArgumentLabel` (L6), `traversal` (L7), `argumentLabel` (L11), `argumentLabel` (L16), `matcher` (L20), `tmp` (L22), `argumentLabel` (L29), `matchers` (L30), `tmp` (L32), `argumentLabelExact` (L38), `someNode` (L40), `tmp` (L46), `argumentLabelExact` (L52), `valueSet` (L55), `tmp` (L57), `argumentLabelNot` (L63), `matcher` (L67), `tmp` (L69), `argumentLabelNot` (L76), `matchers` (L77), `tmp` (L79). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentLabel.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-435325db9e65456f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `629210e3a1184a6f781918e215b6bf4adf354236`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-15db1ba7c8c873c432.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7053bb94a8f7b89d035f6b977359f1adfd2d9550307ec4dcdc441f1c3b71097f`.

> 
> final class TraversalPropertyArgumentName[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasArgumentNameEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to argumentName property */
>   def argumentName: Iterator[String] =
>     traversal.flatMap(_.argumentName)
> 
>   /** Traverse to nodes where the argumentName matches the regular expression `value`
>     */
>   def argumentName(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       argumentNameExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 21 lexically identified declarations, including `TraversalPropertyArgumentName` (L6), `traversal` (L7), `argumentName` (L11), `argumentName` (L16), `matcher` (L20), `tmp` (L22), `argumentName` (L29), `matchers` (L30), `tmp` (L32), `argumentNameExact` (L38), `someNode` (L40), `tmp` (L46), `argumentNameExact` (L52), `valueSet` (L55), `tmp` (L57), `argumentNameNot` (L63), `matcher` (L67), `tmp` (L69), `argumentNameNot` (L76), `matchers` (L77), `tmp` (L79). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyArgumentName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5472bb9e723f9d93"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentFullName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `683d4f9301f1b93515807a24ecc2a27429b45678`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentFullName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-278f3109b827b3915c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentFullName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentFullName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a7f906c15d77776ccde15537f5f67831ee29b7006dc9bb276aed8a87abd6adf0`.

> 
> final class TraversalPropertyAstParentFullName[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasAstParentFullNameEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to astParentFullName property */
>   def astParentFullName: Iterator[String] =
>     traversal.map(_.astParentFullName)
> 
>   /** Traverse to nodes where the astParentFullName matches the regular expression `value`
>     */
>   def astParentFullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       astParentFullNameExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalPropertyAstParentFullName` (L6), `astParentFullName` (L12), `astParentFullName` (L17), `matcher` (L21), `astParentFullName` (L27), `matchers` (L28), `astParentFullNameExact` (L34), `someNode` (L36), `astParentFullNameExact` (L45), `someNode` (L49), `valueSet` (L56), `astParentFullNameNot` (L63), `matcher` (L67), `astParentFullNameNot` (L74), `matchers` (L75). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentFullName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-eb5c4ccb56b2ffea"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentType.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `3739ea6933f1090d2b4d0d087f7dc05bb2a989d7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentType.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d80f009c2230504975.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentType.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentType.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cef4478a6a3d8349a66c012aa8f11bf9f4aebb373c74dfaae10aa7fc530097ca`.

> 
> final class TraversalPropertyAstParentType[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasAstParentTypeEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to astParentType property */
>   def astParentType: Iterator[String] =
>     traversal.map(_.astParentType)
> 
>   /** Traverse to nodes where the astParentType matches the regular expression `value`
>     */
>   def astParentType(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       astParentTypeExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyAstParentType` (L6), `traversal` (L7), `astParentType` (L11), `astParentType` (L16), `matcher` (L20), `astParentType` (L26), `matchers` (L27), `astParentTypeExact` (L33), `someNode` (L35), `astParentTypeExact` (L44), `someNode` (L48), `valueSet` (L55), `astParentTypeNot` (L62), `matcher` (L66), `astParentTypeNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyAstParentType.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c60bd524b50cd0c6"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCanonicalName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f5c065d84bddfabf68578994c2134e82bd16c79a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCanonicalName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6b10c30fab3ece5b3c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCanonicalName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCanonicalName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e0af79f9b07962acf1ea163cf3fb74d3a222b1c4ecb35aed93a3a96b1b3a762f`.

> 
> final class TraversalPropertyCanonicalName[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasCanonicalNameEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to canonicalName property */
>   def canonicalName: Iterator[String] =
>     traversal.map(_.canonicalName)
> 
>   /** Traverse to nodes where the canonicalName matches the regular expression `value`
>     */
>   def canonicalName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       canonicalNameExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyCanonicalName` (L6), `traversal` (L7), `canonicalName` (L11), `canonicalName` (L16), `matcher` (L20), `canonicalName` (L26), `matchers` (L27), `canonicalNameExact` (L33), `someNode` (L35), `canonicalNameExact` (L44), `someNode` (L48), `valueSet` (L55), `canonicalNameNot` (L62), `matcher` (L66), `canonicalNameNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCanonicalName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-82778b0cb2d55a2d"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyClosureBindingId.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `45ef12de4d9978d870ade5acc4dfdb7485070a66`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyClosureBindingId.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-686836672ef0db2ff4.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyClosureBindingId.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyClosureBindingId.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e6f0897c1c5a61fa276e17d24fa7d16fe4fc41a02e6ce6924bd41934a6955a09`.

> 
> final class TraversalPropertyClosureBindingId[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasClosureBindingIdEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to closureBindingId property */
>   def closureBindingId: Iterator[String] =
>     traversal.flatMap(_.closureBindingId)
> 
>   /** Traverse to nodes where the closureBindingId matches the regular expression `value`
>     */
>   def closureBindingId(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       closureBindingIdExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `TraversalPropertyClosureBindingId` (L6), `closureBindingId` (L12), `closureBindingId` (L17), `matcher` (L21), `tmp` (L23), `closureBindingId` (L30), `matchers` (L31), `tmp` (L33), `closureBindingIdExact` (L39), `someNode` (L41), `tmp` (L47), `closureBindingIdExact` (L53), `valueSet` (L56), `tmp` (L58), `closureBindingIdNot` (L64), `matcher` (L68), `tmp` (L70), `closureBindingIdNot` (L77), `matchers` (L78), `tmp` (L80). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyClosureBindingId.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3c63f7eba512a0b8"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCode.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `3180ae79027612a4b47153445e6d3eb139967927`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCode.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-adb9fe28515061c466.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCode.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCode.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `863fbcb31df5bba458ca632443f4b956db627161a445d32bf173d16632244e6e`.

> 
> final class TraversalPropertyCode[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasCodeEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to code property */
>   def code: Iterator[String] =
>     traversal.map(_.code)
> 
>   /** Traverse to nodes where the code matches the regular expression `value`
>     */
>   def code(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       codeExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyCode` (L6), `traversal` (L7), `code` (L11), `code` (L16), `matcher` (L20), `code` (L26), `matchers` (L27), `codeExact` (L33), `someNode` (L35), `codeExact` (L44), `someNode` (L48), `valueSet` (L55), `codeNot` (L62), `matcher` (L66), `codeNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyCode.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9687f0ee132e1c95"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumber.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `56e839f568df03d476f82f64be767ba609da9a14`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumber.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2569fc515e73a5522d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumber.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumber.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `ca27ba7196512dda470cda9a8bf9afb5e04fdb95473cf271615d64844303295e`.

> 
> final class TraversalPropertyColumnNumber[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasColumnNumberEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to columnNumber property */
>   def columnNumber: Iterator[Int] =
>     traversal.flatMap(_.columnNumber)
> 
>   /** Traverse to nodes where the columnNumber equals the given `value`
>     */
>   def columnNumber(value: Int): Iterator[NodeType] =
>     traversal.filter { node =>
>       val tmp = node.columnNumber; tmp.isDefined && tmp.get == value
>     }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 23 lexically identified declarations, including `TraversalPropertyColumnNumber` (L6), `traversal` (L7), `columnNumber` (L11), `columnNumber` (L16), `tmp` (L18), `columnNumber` (L24), `columnNumberIfPresent` (L31), `columnNumber` (L36), `vset` (L37), `tmp` (L39), `columnNumberNot` (L45), `tmp` (L47), `columnNumberNot` (L52), `vset` (L53), `tmp` (L55), `columnNumberGt` (L61), `tmp` (L63), `columnNumberGte` (L68), `tmp` (L70), `columnNumberLt` (L75), `tmp` (L77), `columnNumberLte` (L82), `tmp` (L84). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumber.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-336a8c88b2b26006"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumberEnd.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0657ee5bc6b93e9bd738f935666dff34bbe71dd8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumberEnd.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3a58b82c492d36e192.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumberEnd.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumberEnd.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `88e02264e17b5826d0f1bdf2e361a89d65c41e71a4b33bc6344e15d73560908e`.

> 
> final class TraversalPropertyColumnNumberEnd[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasColumnNumberEndEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to columnNumberEnd property */
>   def columnNumberEnd: Iterator[Int] =
>     traversal.flatMap(_.columnNumberEnd)
> 
>   /** Traverse to nodes where the columnNumberEnd equals the given `value`
>     */
>   def columnNumberEnd(value: Int): Iterator[NodeType] =
>     traversal.filter { node =>
>       val tmp = node.columnNumberEnd; tmp.isDefined && tmp.get == value


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 22 lexically identified declarations, including `TraversalPropertyColumnNumberEnd` (L6), `columnNumberEnd` (L12), `columnNumberEnd` (L17), `tmp` (L19), `columnNumberEnd` (L25), `columnNumberEndIfPresent` (L32), `columnNumberEnd` (L37), `vset` (L38), `tmp` (L40), `columnNumberEndNot` (L46), `tmp` (L48), `columnNumberEndNot` (L53), `vset` (L54), `tmp` (L56), `columnNumberEndGt` (L62), `tmp` (L64), `columnNumberEndGte` (L69), `tmp` (L71), `columnNumberEndLt` (L76), `tmp` (L78), `columnNumberEndLte` (L83), `tmp` (L85). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyColumnNumberEnd.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-688042d915e5512c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContainedRef.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8651daaab1ed6a422763a75e229e8b6e721a6cb0`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContainedRef.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-db2a7caabbccd74ae0.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContainedRef.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContainedRef.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7c7e9ea72aa9756a30a5d1b9e5c8ee4b296b6b32e932de177a81bc503bccabc3`.

> 
> final class TraversalPropertyContainedRef[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasContainedRefEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to containedRef property */
>   def containedRef: Iterator[String] =
>     traversal.map(_.containedRef)
> 
>   /** Traverse to nodes where the containedRef matches the regular expression `value`
>     */
>   def containedRef(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       containedRefExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyContainedRef` (L6), `traversal` (L7), `containedRef` (L11), `containedRef` (L16), `matcher` (L20), `containedRef` (L26), `matchers` (L27), `containedRefExact` (L33), `someNode` (L35), `containedRefExact` (L44), `someNode` (L48), `valueSet` (L55), `containedRefNot` (L62), `matcher` (L66), `containedRefNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContainedRef.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2e3f3b15eac947ca"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContent.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0ebda7c20fcd957ab047fceb3bdb4c28db3e024d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContent.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d99d3259d13ad56844.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContent.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContent.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d150d14a18a07466b2cebf556de83be36c6c65402b31e8a9223d26117ee20613`.

> 
> final class TraversalPropertyContent[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasContentEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to content property */
>   def content: Iterator[String] =
>     traversal.map(_.content)
> 
>   /** Traverse to nodes where the content matches the regular expression `value`
>     */
>   def content(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       contentExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyContent` (L6), `traversal` (L7), `content` (L11), `content` (L16), `matcher` (L20), `content` (L26), `matchers` (L27), `contentExact` (L33), `someNode` (L35), `contentExact` (L44), `someNode` (L48), `valueSet` (L55), `contentNot` (L62), `matcher` (L66), `contentNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyContent.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0b054c903d860689"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyControlStructureType.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `2b7c72110c5e37903520f4b4f3e99295e40cc25c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyControlStructureType.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1c892688c9aac39f4f.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyControlStructureType.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyControlStructureType.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `aa8f0768f46b0df8b88b555ae8411d03a1a4e76652c94fc43cadb284abd9a001`.

> 
> final class TraversalPropertyControlStructureType[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasControlStructureTypeEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to controlStructureType property */
>   def controlStructureType: Iterator[String] =
>     traversal.map(_.controlStructureType)
> 
>   /** Traverse to nodes where the controlStructureType matches the regular expression `value`
>     */
>   def controlStructureType(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       controlStructureTypeExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalPropertyControlStructureType` (L6), `controlStructureType` (L12), `controlStructureType` (L17), `matcher` (L21), `controlStructureType` (L27), `matchers` (L28), `controlStructureTypeExact` (L34), `someNode` (L36), `controlStructureTypeExact` (L45), `someNode` (L49), `valueSet` (L56), `controlStructureTypeNot` (L63), `matcher` (L67), `controlStructureTypeNot` (L74), `matchers` (L75). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyControlStructureType.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-72dad0262778cda3"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDependencyGroupId.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `606e5e949e72a9d38df816d0765fd460b7a4ed69`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDependencyGroupId.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-81f78dc126e716d253.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDependencyGroupId.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDependencyGroupId.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dcef9af8ae50c24c11807e44deb4ad6bf6e65301f1db1dfc00f2009ed6f27b35`.

> 
> final class TraversalPropertyDependencyGroupId[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasDependencyGroupIdEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to dependencyGroupId property */
>   def dependencyGroupId: Iterator[String] =
>     traversal.flatMap(_.dependencyGroupId)
> 
>   /** Traverse to nodes where the dependencyGroupId matches the regular expression `value`
>     */
>   def dependencyGroupId(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       dependencyGroupIdExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `TraversalPropertyDependencyGroupId` (L6), `dependencyGroupId` (L12), `dependencyGroupId` (L17), `matcher` (L21), `tmp` (L23), `dependencyGroupId` (L30), `matchers` (L31), `tmp` (L33), `dependencyGroupIdExact` (L39), `someNode` (L41), `tmp` (L47), `dependencyGroupIdExact` (L53), `valueSet` (L56), `tmp` (L58), `dependencyGroupIdNot` (L64), `matcher` (L68), `tmp` (L70), `dependencyGroupIdNot` (L77), `matchers` (L78), `tmp` (L80). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDependencyGroupId.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-64816e2444f181e8"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDispatchType.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f6f59123a25103f38ede5d6dc480c08c43a9bb46`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDispatchType.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-df05546ae925bac1be.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDispatchType.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDispatchType.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a4f688d152e1a62978eb5f9dcb897f9d57e2ad92bb43e13591a7122a26dd7229`.

> 
> final class TraversalPropertyDispatchType[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasDispatchTypeEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to dispatchType property */
>   def dispatchType: Iterator[String] =
>     traversal.map(_.dispatchType)
> 
>   /** Traverse to nodes where the dispatchType matches the regular expression `value`
>     */
>   def dispatchType(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       dispatchTypeExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyDispatchType` (L6), `traversal` (L7), `dispatchType` (L11), `dispatchType` (L16), `matcher` (L20), `dispatchType` (L26), `matchers` (L27), `dispatchTypeExact` (L33), `someNode` (L35), `dispatchTypeExact` (L44), `someNode` (L48), `valueSet` (L55), `dispatchTypeNot` (L62), `matcher` (L66), `dispatchTypeNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDispatchType.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ffa0cb896142a0f5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDynamicTypeHintFullName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `def758f34a16152a159abf2e666496e825598ba7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDynamicTypeHintFullName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4e14795964a0c6b21c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDynamicTypeHintFullName.scala, lines 5–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDynamicTypeHintFullName.scala#L5-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `90b97e694d24c8129a22031b4ee48a3701614646c892ed6f22b3d2d976801fef`.

> 
> final class TraversalPropertyDynamicTypeHintFullName[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasDynamicTypeHintFullNameEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to dynamicTypeHintFullName property */
>   def dynamicTypeHintFullName: Iterator[String] =
>     traversal.flatMap(_.dynamicTypeHintFullName)
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 2 lexically identified declarations, including `TraversalPropertyDynamicTypeHintFullName` (L6), `dynamicTypeHintFullName` (L12). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyDynamicTypeHintFullName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-541a3a07ba6ead76"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvaluationStrategy.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `65d10c3819680d5fc07d63d8f6f5c70909d5868d`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvaluationStrategy.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1831c042cff8c8c22c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvaluationStrategy.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvaluationStrategy.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0d89b1284606c0c405297dd14ca5dce81844c8757e0dd4feb55ceafa2660e7b0`.

> 
> final class TraversalPropertyEvaluationStrategy[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasEvaluationStrategyEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to evaluationStrategy property */
>   def evaluationStrategy: Iterator[String] =
>     traversal.map(_.evaluationStrategy)
> 
>   /** Traverse to nodes where the evaluationStrategy matches the regular expression `value`
>     */
>   def evaluationStrategy(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       evaluationStrategyExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalPropertyEvaluationStrategy` (L6), `evaluationStrategy` (L12), `evaluationStrategy` (L17), `matcher` (L21), `evaluationStrategy` (L27), `matchers` (L28), `evaluationStrategyExact` (L34), `someNode` (L36), `evaluationStrategyExact` (L45), `someNode` (L49), `valueSet` (L56), `evaluationStrategyNot` (L63), `matcher` (L67), `evaluationStrategyNot` (L74), `matchers` (L75). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvaluationStrategy.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cc5c101926fdf05f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvidenceDescription.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `cefd73ff1d8c46920b83c1e7ec8ac85caf35bce7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvidenceDescription.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c591ec324f09d0932e.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvidenceDescription.scala, lines 5–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvidenceDescription.scala#L5-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9cbfffd0158b76d0f0128092efe9bbe2c75dd3a357446f1f762bfe8452590a5c`.

> 
> final class TraversalPropertyEvidenceDescription[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasEvidenceDescriptionEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to evidenceDescription property */
>   def evidenceDescription: Iterator[String] =
>     traversal.flatMap(_.evidenceDescription)
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 2 lexically identified declarations, including `TraversalPropertyEvidenceDescription` (L6), `evidenceDescription` (L12). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyEvidenceDescription.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-92a9c7049a55622c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyExplicitAs.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `34df2f2b73b6686768643fe1f1e1b61399e73760`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyExplicitAs.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9ebe080b1005dcbb18.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyExplicitAs.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyExplicitAs.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `47e302688ffa9142e3944f14e697d12f9adbe4dfd2012b48dcc033103065bc42`.

> 
> final class TraversalPropertyExplicitAs[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasExplicitAsEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to explicitAs property */
>   def explicitAs: Iterator[Boolean] =
>     traversal.flatMap(_.explicitAs)
> 
>   /** Traverse to nodes where the explicitAs equals the given `value`
>     */
>   def explicitAs(value: Boolean): Iterator[NodeType] =
>     traversal.filter { node => node.explicitAs.isDefined && node.explicitAs.get == value }
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 4 lexically identified declarations, including `TraversalPropertyExplicitAs` (L6), `traversal` (L7), `explicitAs` (L11), `explicitAs` (L16). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyExplicitAs.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f94dce2f5b7c94ae"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFilename.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9936e9eed9bf4dd04ec6430fdf7c26253789de2b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFilename.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-46de965e51ce019071.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFilename.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFilename.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `724ddb0db0e7796b36f657d4357ce69bbdeaf77377e85d88fed8989370bbfb05`.

> 
> final class TraversalPropertyFilename[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasFilenameEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to filename property */
>   def filename: Iterator[String] =
>     traversal.map(_.filename)
> 
>   /** Traverse to nodes where the filename matches the regular expression `value`
>     */
>   def filename(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       filenameExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyFilename` (L6), `traversal` (L7), `filename` (L11), `filename` (L16), `matcher` (L20), `filename` (L26), `matchers` (L27), `filenameExact` (L33), `someNode` (L35), `filenameExact` (L44), `someNode` (L48), `valueSet` (L55), `filenameNot` (L62), `matcher` (L66), `filenameNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFilename.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e8b955f64566ab16"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFullName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f5adf4f499b7a5ec032514d6014e4c76b9d45cd3`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFullName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-cf58381117e963317d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFullName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFullName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2c749f2203f77f575ca8ca97e0e21daf7a4fe47a55d30deaa819ce83f79e1707`.

> 
> final class TraversalPropertyFullName[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasFullNameEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to fullName property */
>   def fullName: Iterator[String] =
>     traversal.map(_.fullName)
> 
>   /** Traverse to nodes where the fullName matches the regular expression `value`
>     */
>   def fullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       fullNameExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyFullName` (L6), `traversal` (L7), `fullName` (L11), `fullName` (L16), `matcher` (L20), `fullName` (L26), `matchers` (L27), `fullNameExact` (L33), `someNode` (L35), `fullNameExact` (L44), `someNode` (L48), `valueSet` (L55), `fullNameNot` (L62), `matcher` (L66), `fullNameNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyFullName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5b426db812f30d48"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyGenericSignature.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `d103485c048cf8ce2a7bb0a6f2e679a3519a3c3a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyGenericSignature.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-170fa6919b821e5e89.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyGenericSignature.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyGenericSignature.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6f2d0fd2254cbae2c57731cd225fd74d7fa6b419563b31e03d921c6c71c9c2db`.

> 
> final class TraversalPropertyGenericSignature[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasGenericSignatureEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to genericSignature property */
>   def genericSignature: Iterator[String] =
>     traversal.map(_.genericSignature)
> 
>   /** Traverse to nodes where the genericSignature matches the regular expression `value`
>     */
>   def genericSignature(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       genericSignatureExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalPropertyGenericSignature` (L6), `genericSignature` (L12), `genericSignature` (L17), `matcher` (L21), `genericSignature` (L27), `matchers` (L28), `genericSignatureExact` (L34), `someNode` (L36), `genericSignatureExact` (L45), `someNode` (L49), `valueSet` (L56), `genericSignatureNot` (L63), `matcher` (L67), `genericSignatureNot` (L74), `matchers` (L75). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyGenericSignature.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2509c71eb3975fdc"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyHash.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `cf74ce09e2114ea32a246bfebbd6029dd0119b2a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyHash.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-20d2bc542b03bb8862.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyHash.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyHash.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `324e8f3a73769facb6c78701bbf81f6d3e8b636782605a30f8ca5e81822fd304`.

> 
> final class TraversalPropertyHash[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasHashEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to hash property */
>   def hash: Iterator[String] =
>     traversal.flatMap(_.hash)
> 
>   /** Traverse to nodes where the hash matches the regular expression `value`
>     */
>   def hash(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       hashExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 21 lexically identified declarations, including `TraversalPropertyHash` (L6), `traversal` (L7), `hash` (L11), `hash` (L16), `matcher` (L20), `tmp` (L22), `hash` (L29), `matchers` (L30), `tmp` (L32), `hashExact` (L38), `someNode` (L40), `tmp` (L46), `hashExact` (L52), `valueSet` (L55), `tmp` (L57), `hashNot` (L63), `matcher` (L67), `tmp` (L69), `hashNot` (L76), `matchers` (L77), `tmp` (L79). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyHash.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-096df0de2cc2a0ed"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedAs.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `fe92ef1a1cb577aac1ac9242d5b44a8ace33237b`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedAs.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-963735cb5dcafcb094.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedAs.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedAs.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2395b983d3887721246333664d405b6fc8da3153eda19424fcd087521f8e6e59`.

> 
> final class TraversalPropertyImportedAs[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasImportedAsEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to importedAs property */
>   def importedAs: Iterator[String] =
>     traversal.flatMap(_.importedAs)
> 
>   /** Traverse to nodes where the importedAs matches the regular expression `value`
>     */
>   def importedAs(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       importedAsExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 21 lexically identified declarations, including `TraversalPropertyImportedAs` (L6), `traversal` (L7), `importedAs` (L11), `importedAs` (L16), `matcher` (L20), `tmp` (L22), `importedAs` (L29), `matchers` (L30), `tmp` (L32), `importedAsExact` (L38), `someNode` (L40), `tmp` (L46), `importedAsExact` (L52), `valueSet` (L55), `tmp` (L57), `importedAsNot` (L63), `matcher` (L67), `tmp` (L69), `importedAsNot` (L76), `matchers` (L77), `tmp` (L79). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedAs.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4eb5bb04d6da23aa"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedEntity.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5965065156618f0759d587e49465f7a269353e13`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedEntity.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-32ea4081defdc8473b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedEntity.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedEntity.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d18a7899b1704efd3a6804d7520257af1d5cdef47b5e379c49f9125c6632a65d`.

> 
> final class TraversalPropertyImportedEntity[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasImportedEntityEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to importedEntity property */
>   def importedEntity: Iterator[String] =
>     traversal.flatMap(_.importedEntity)
> 
>   /** Traverse to nodes where the importedEntity matches the regular expression `value`
>     */
>   def importedEntity(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       importedEntityExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `TraversalPropertyImportedEntity` (L6), `importedEntity` (L12), `importedEntity` (L17), `matcher` (L21), `tmp` (L23), `importedEntity` (L30), `matchers` (L31), `tmp` (L33), `importedEntityExact` (L39), `someNode` (L41), `tmp` (L47), `importedEntityExact` (L53), `valueSet` (L56), `tmp` (L58), `importedEntityNot` (L64), `matcher` (L68), `tmp` (L70), `importedEntityNot` (L77), `matchers` (L78), `tmp` (L80). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyImportedEntity.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8b1a0ad683bf3952"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIndex.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `18a4d16c17eccb6a9cf3ab59e3e07c6114407099`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIndex.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8498cf90b7292f0021.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIndex.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIndex.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2511b3d628219a7df019255a2442295913031b915c6cab6458435e1a70fe6007`.

> 
> final class TraversalPropertyIndex[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasIndexEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to index property */
>   def index: Iterator[Int] =
>     traversal.map(_.index)
> 
>   /** Traverse to nodes where the index equals the given `value`
>     */
>   def index(value: Int): Iterator[NodeType] =
>     traversal.filter { _.index == value }
> 
>   /** Traverse to nodes where the index equals the given `value`, or no results if `value` is None


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 14 lexically identified declarations, including `TraversalPropertyIndex` (L6), `traversal` (L7), `index` (L11), `index` (L16), `index` (L21), `index` (L26), `vset` (L27), `indexNot` (L33), `indexNot` (L38), `vset` (L39), `indexGt` (L45), `indexGte` (L50), `indexLt` (L55), `indexLte` (L60). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIndex.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cda20d3bead5337e"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyInheritsFromTypeFullName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `412e0a38d5dab4b2a3e4764a5dd2abec80c2872a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyInheritsFromTypeFullName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-82ea7e814c7801ac6d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyInheritsFromTypeFullName.scala, lines 5–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyInheritsFromTypeFullName.scala#L5-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3583da5611003dde63e1fd1e587228d85794ee668bac8abec6f264e3e100f0a1`.

> 
> final class TraversalPropertyInheritsFromTypeFullName[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasInheritsFromTypeFullNameEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to inheritsFromTypeFullName property */
>   def inheritsFromTypeFullName: Iterator[String] =
>     traversal.flatMap(_.inheritsFromTypeFullName)
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 2 lexically identified declarations, including `TraversalPropertyInheritsFromTypeFullName` (L6), `inheritsFromTypeFullName` (L12). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyInheritsFromTypeFullName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7a1a7da4e1ae44d9"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExplicit.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `01a44699207499f113287472682a85c996e64634`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExplicit.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0000a07f4a7d846202.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExplicit.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExplicit.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b0e9a3b737459f9e251f824aed7f5f95ec530875023cd44efd40fd571d0c216f`.

> 
> final class TraversalPropertyIsExplicit[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasIsExplicitEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to isExplicit property */
>   def isExplicit: Iterator[Boolean] =
>     traversal.flatMap(_.isExplicit)
> 
>   /** Traverse to nodes where the isExplicit equals the given `value`
>     */
>   def isExplicit(value: Boolean): Iterator[NodeType] =
>     traversal.filter { node => node.isExplicit.isDefined && node.isExplicit.get == value }
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 4 lexically identified declarations, including `TraversalPropertyIsExplicit` (L6), `traversal` (L7), `isExplicit` (L11), `isExplicit` (L16). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExplicit.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f9cd0ecf5fd5bbef"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExternal.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c38831182156c4b714ccc965be2b5691d7861206`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExternal.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f92bd4decea2ef8d39.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExternal.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExternal.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `db6024f3ec114dd32b45bbddb8d25065f25c0208908da2d81cc9c70d835e986a`.

> 
> final class TraversalPropertyIsExternal[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasIsExternalEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to isExternal property */
>   def isExternal: Iterator[Boolean] =
>     traversal.map(_.isExternal)
> 
>   /** Traverse to nodes where the isExternal equals the given `value`
>     */
>   def isExternal(value: Boolean): Iterator[NodeType] =
>     traversal.filter { _.isExternal == value }
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 4 lexically identified declarations, including `TraversalPropertyIsExternal` (L6), `traversal` (L7), `isExternal` (L11), `isExternal` (L16). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsExternal.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1f16fe5288f6dbc2"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsModuleImport.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c6a51403c6f2dda43fa7c5cbd7dd65150c739ef1`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsModuleImport.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c128634e05d94ae28a.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsModuleImport.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsModuleImport.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b5f88aa2dab924984b0b208685a2408abdcd11406f5ad5fff8fc1319310d5aea`.

> 
> final class TraversalPropertyIsModuleImport[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasIsModuleImportEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to isModuleImport property */
>   def isModuleImport: Iterator[Boolean] =
>     traversal.flatMap(_.isModuleImport)
> 
>   /** Traverse to nodes where the isModuleImport equals the given `value`
>     */
>   def isModuleImport(value: Boolean): Iterator[NodeType] =
>     traversal.filter { node => node.isModuleImport.isDefined && node.isModuleImport.get == value }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 3 lexically identified declarations, including `TraversalPropertyIsModuleImport` (L6), `isModuleImport` (L12), `isModuleImport` (L17). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsModuleImport.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d4c4e76f57773185"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsVariadic.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `57249f941ec940bbad7dc3a0341496b6a061e276`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsVariadic.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7a1d2b9261e04c4ad3.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsVariadic.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsVariadic.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2aed1f3fe96e4b5828dccdb05ca08b64b842ab2764f5cdc6f000ebf871369d39`.

> 
> final class TraversalPropertyIsVariadic[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasIsVariadicEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to isVariadic property */
>   def isVariadic: Iterator[Boolean] =
>     traversal.map(_.isVariadic)
> 
>   /** Traverse to nodes where the isVariadic equals the given `value`
>     */
>   def isVariadic(value: Boolean): Iterator[NodeType] =
>     traversal.filter { _.isVariadic == value }
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 4 lexically identified declarations, including `TraversalPropertyIsVariadic` (L6), `traversal` (L7), `isVariadic` (L11), `isVariadic` (L16). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsVariadic.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-85671c2344e37bb4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsWildcard.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `65a839cd264e95878cecf23e7d152a61f509fea9`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsWildcard.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-05f432f44ebe2d4903.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsWildcard.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsWildcard.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6fec4630d55b61b4558e2d51b3ece5021e8c82ab59a89a1109b8bc24648bc8ec`.

> 
> final class TraversalPropertyIsWildcard[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasIsWildcardEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to isWildcard property */
>   def isWildcard: Iterator[Boolean] =
>     traversal.flatMap(_.isWildcard)
> 
>   /** Traverse to nodes where the isWildcard equals the given `value`
>     */
>   def isWildcard(value: Boolean): Iterator[NodeType] =
>     traversal.filter { node => node.isWildcard.isDefined && node.isWildcard.get == value }
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 4 lexically identified declarations, including `TraversalPropertyIsWildcard` (L6), `traversal` (L7), `isWildcard` (L11), `isWildcard` (L16). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyIsWildcard.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3e39d3a6e12dbb51"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyKey.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e6c70ab9b5c44b1fbdbe133996fc2efdaab57e89`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyKey.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f495be882305cb891b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyKey.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyKey.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f0d3709ff0af0d32e56f956095f80ceb53938e04efaabde83ed8015a3006e627`.

> 
> final class TraversalPropertyKey[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasKeyEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to key property */
>   def key: Iterator[String] =
>     traversal.map(_.key)
> 
>   /** Traverse to nodes where the key matches the regular expression `value`
>     */
>   def key(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       keyExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyKey` (L6), `traversal` (L7), `key` (L11), `key` (L16), `matcher` (L20), `key` (L26), `matchers` (L27), `keyExact` (L33), `someNode` (L35), `keyExact` (L44), `someNode` (L48), `valueSet` (L55), `keyNot` (L62), `matcher` (L66), `keyNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyKey.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3ba2e3e56549e2b1"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLanguage.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `47e253b827bab43632a387451cf428d96bb45cc3`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLanguage.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4caa135e544309f5a2.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLanguage.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLanguage.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `efbd34d4b88b75f665df010f3ec31262b5a31e618cce67db8b95c904635ffcf9`.

> 
> final class TraversalPropertyLanguage[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasLanguageEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to language property */
>   def language: Iterator[String] =
>     traversal.map(_.language)
> 
>   /** Traverse to nodes where the language matches the regular expression `value`
>     */
>   def language(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       languageExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyLanguage` (L6), `traversal` (L7), `language` (L11), `language` (L16), `matcher` (L20), `language` (L26), `matchers` (L27), `languageExact` (L33), `someNode` (L35), `languageExact` (L44), `someNode` (L48), `valueSet` (L55), `languageNot` (L62), `matcher` (L66), `languageNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLanguage.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-addc7cadfee8e622"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumber.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `35ddc70394147d6ee5c2a2476a497c249e098673`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumber.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-137b07d9544efd4360.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumber.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumber.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e423c05a62d01b950c0fff73db0cbb2b2ebe81e5ee00425da37820cb81c73e66`.

> 
> final class TraversalPropertyLineNumber[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasLineNumberEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to lineNumber property */
>   def lineNumber: Iterator[Int] =
>     traversal.flatMap(_.lineNumber)
> 
>   /** Traverse to nodes where the lineNumber equals the given `value`
>     */
>   def lineNumber(value: Int): Iterator[NodeType] =
>     traversal.filter { node =>
>       val tmp = node.lineNumber; tmp.isDefined && tmp.get == value
>     }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 23 lexically identified declarations, including `TraversalPropertyLineNumber` (L6), `traversal` (L7), `lineNumber` (L11), `lineNumber` (L16), `tmp` (L18), `lineNumber` (L24), `lineNumberIfPresent` (L31), `lineNumber` (L36), `vset` (L37), `tmp` (L39), `lineNumberNot` (L45), `tmp` (L47), `lineNumberNot` (L52), `vset` (L53), `tmp` (L55), `lineNumberGt` (L61), `tmp` (L63), `lineNumberGte` (L68), `tmp` (L70), `lineNumberLt` (L75), `tmp` (L77), `lineNumberLte` (L82), `tmp` (L84). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumber.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bdafbd19c661b861"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumberEnd.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `2b75d18e69e98ce4d244db81931e73be3be8ba9f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumberEnd.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-170d445c1dff9a7dcd.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumberEnd.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumberEnd.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `994bb5e3d6a130729d8006727ca8fbf677a4b4dba9c6a55d6c5cb2e61a471843`.

> 
> final class TraversalPropertyLineNumberEnd[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasLineNumberEndEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to lineNumberEnd property */
>   def lineNumberEnd: Iterator[Int] =
>     traversal.flatMap(_.lineNumberEnd)
> 
>   /** Traverse to nodes where the lineNumberEnd equals the given `value`
>     */
>   def lineNumberEnd(value: Int): Iterator[NodeType] =
>     traversal.filter { node =>
>       val tmp = node.lineNumberEnd; tmp.isDefined && tmp.get == value
>     }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 23 lexically identified declarations, including `TraversalPropertyLineNumberEnd` (L6), `traversal` (L7), `lineNumberEnd` (L11), `lineNumberEnd` (L16), `tmp` (L18), `lineNumberEnd` (L24), `lineNumberEndIfPresent` (L31), `lineNumberEnd` (L36), `vset` (L37), `tmp` (L39), `lineNumberEndNot` (L45), `tmp` (L47), `lineNumberEndNot` (L52), `vset` (L53), `tmp` (L55), `lineNumberEndGt` (L61), `tmp` (L63), `lineNumberEndGte` (L68), `tmp` (L70), `lineNumberEndLt` (L75), `tmp` (L77), `lineNumberEndLte` (L82), `tmp` (L84). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyLineNumberEnd.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-04c58951665f1936"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyMethodFullName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `968a9161ef746543dc7a522b08feaaac22ff1a1a`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyMethodFullName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-59cd61e8d3bd5a5a45.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyMethodFullName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyMethodFullName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cd39e83129d0370c9b071439d947f1ad2f9506b44c3ed1fb8f9adfcfe8e475be`.

> 
> final class TraversalPropertyMethodFullName[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasMethodFullNameEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to methodFullName property */
>   def methodFullName: Iterator[String] =
>     traversal.map(_.methodFullName)
> 
>   /** Traverse to nodes where the methodFullName matches the regular expression `value`
>     */
>   def methodFullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       methodFullNameExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalPropertyMethodFullName` (L6), `methodFullName` (L12), `methodFullName` (L17), `matcher` (L21), `methodFullName` (L27), `matchers` (L28), `methodFullNameExact` (L34), `someNode` (L36), `methodFullNameExact` (L45), `someNode` (L49), `valueSet` (L56), `methodFullNameNot` (L63), `matcher` (L67), `methodFullNameNot` (L74), `matchers` (L75). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyMethodFullName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3f06eb0241488b81"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyModifierType.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4b4cb9c7674ce36e3bec73de9e6054ddc9629b46`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyModifierType.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4df181615837963c4d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyModifierType.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyModifierType.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0be5bbb2528ae94d3c59696939775df258b430ffac4615d3ef8769f0757477bc`.

> 
> final class TraversalPropertyModifierType[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasModifierTypeEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to modifierType property */
>   def modifierType: Iterator[String] =
>     traversal.map(_.modifierType)
> 
>   /** Traverse to nodes where the modifierType matches the regular expression `value`
>     */
>   def modifierType(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       modifierTypeExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyModifierType` (L6), `traversal` (L7), `modifierType` (L11), `modifierType` (L16), `matcher` (L20), `modifierType` (L26), `matchers` (L27), `modifierTypeExact` (L33), `someNode` (L35), `modifierTypeExact` (L44), `someNode` (L48), `valueSet` (L55), `modifierTypeNot` (L62), `matcher` (L66), `modifierTypeNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyModifierType.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bf23acdd6530d479"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c417528cd211d7f573cde9c11d100bcdf5f2ee83`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-acabdf4735d13e01e1.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `127426b202918617a7d0ce5e3501aa191319a05f3edb4193b551bee263f617c8`.

> 
> final class TraversalPropertyName[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasNameEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       nameExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyName` (L6), `traversal` (L7), `name` (L11), `name` (L16), `matcher` (L20), `name` (L26), `matchers` (L27), `nameExact` (L33), `someNode` (L35), `nameExact` (L44), `someNode` (L48), `valueSet` (L55), `nameNot` (L62), `matcher` (L66), `nameNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cc6d8e3bd3048959"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffset.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `a8a30d5c77411804681a6ade49f2402e0390dd7e`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffset.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-651d950db4aa6a6b75.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffset.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffset.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `34507f48307b1b3e374c0bb2ecf60322d180ed7c4a0ac0ec0df8568bbb06667f`.

> 
> final class TraversalPropertyOffset[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasOffsetEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to offset property */
>   def offset: Iterator[Int] =
>     traversal.flatMap(_.offset)
> 
>   /** Traverse to nodes where the offset equals the given `value`
>     */
>   def offset(value: Int): Iterator[NodeType] =
>     traversal.filter { node =>
>       val tmp = node.offset; tmp.isDefined && tmp.get == value
>     }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 23 lexically identified declarations, including `TraversalPropertyOffset` (L6), `traversal` (L7), `offset` (L11), `offset` (L16), `tmp` (L18), `offset` (L24), `offsetIfPresent` (L29), `offset` (L34), `vset` (L35), `tmp` (L37), `offsetNot` (L43), `tmp` (L45), `offsetNot` (L50), `vset` (L51), `tmp` (L53), `offsetGt` (L59), `tmp` (L61), `offsetGte` (L66), `tmp` (L68), `offsetLt` (L73), `tmp` (L75), `offsetLte` (L80), `tmp` (L82). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffset.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d559670aa3023a82"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffsetEnd.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `ef51197b39ed5f8c553b7559259715dba790ea58`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffsetEnd.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-58e53c245d81d446a8.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffsetEnd.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffsetEnd.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a6c1d41ea1670ff744fce4ed15e76fbd602e3cb7164c7865d2fee9ec4bdb9e49`.

> 
> final class TraversalPropertyOffsetEnd[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasOffsetEndEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to offsetEnd property */
>   def offsetEnd: Iterator[Int] =
>     traversal.flatMap(_.offsetEnd)
> 
>   /** Traverse to nodes where the offsetEnd equals the given `value`
>     */
>   def offsetEnd(value: Int): Iterator[NodeType] =
>     traversal.filter { node =>
>       val tmp = node.offsetEnd; tmp.isDefined && tmp.get == value
>     }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 23 lexically identified declarations, including `TraversalPropertyOffsetEnd` (L6), `traversal` (L7), `offsetEnd` (L11), `offsetEnd` (L16), `tmp` (L18), `offsetEnd` (L24), `offsetEndIfPresent` (L29), `offsetEnd` (L34), `vset` (L35), `tmp` (L37), `offsetEndNot` (L43), `tmp` (L45), `offsetEndNot` (L50), `vset` (L51), `tmp` (L53), `offsetEndGt` (L59), `tmp` (L61), `offsetEndGte` (L66), `tmp` (L68), `offsetEndLt` (L73), `tmp` (L75), `offsetEndLte` (L80), `tmp` (L82). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOffsetEnd.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bc354fe97cd1a89f"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOrder.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `6a8737f830f1badcb88c3a1dc6eaa9ac024f34f6`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOrder.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a89eac9154b5b2b64c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOrder.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOrder.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `26f54837527e686a0a4fe76125c58133ffbd199a401e8631e5801f3c424dec38`.

> 
> final class TraversalPropertyOrder[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasOrderEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to order property */
>   def order: Iterator[Int] =
>     traversal.map(_.order)
> 
>   /** Traverse to nodes where the order equals the given `value`
>     */
>   def order(value: Int): Iterator[NodeType] =
>     traversal.filter { _.order == value }
> 
>   /** Traverse to nodes where the order equals the given `value`, or no results if `value` is None


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 14 lexically identified declarations, including `TraversalPropertyOrder` (L6), `traversal` (L7), `order` (L11), `order` (L16), `order` (L21), `order` (L26), `vset` (L27), `orderNot` (L33), `orderNot` (L38), `vset` (L39), `orderGt` (L45), `orderGte` (L50), `orderLt` (L55), `orderLte` (L60). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOrder.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-630cfab2ada8c88e"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOverlays.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `388fe99b1f01a3af39115552f2643e94ffd9e588`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOverlays.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5d44a8e60b969a23cb.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOverlays.scala, lines 5–14](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOverlays.scala#L5-L14). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `86e2318581c5842fd1636f1bd5511634930191d96cff76b18686cb5d53774e5e`.

> 
> final class TraversalPropertyOverlays[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasOverlaysEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to overlays property */
>   def overlays: Iterator[String] =
>     traversal.flatMap(_.overlays)
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 3 lexically identified declarations, including `TraversalPropertyOverlays` (L6), `traversal` (L7), `overlays` (L11). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyOverlays.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0314fdd3b5398a55"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyParserTypeName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7f1f95c8ab5be0dcb3b17d13fe9bf0cf62591ab5`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyParserTypeName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-606e08fe1a18f6c76d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyParserTypeName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyParserTypeName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `8dcf238f5742db95811f1d82d2536b22f497e46409178721bc8d460aec4e62b0`.

> 
> final class TraversalPropertyParserTypeName[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasParserTypeNameEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to parserTypeName property */
>   def parserTypeName: Iterator[String] =
>     traversal.map(_.parserTypeName)
> 
>   /** Traverse to nodes where the parserTypeName matches the regular expression `value`
>     */
>   def parserTypeName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       parserTypeNameExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalPropertyParserTypeName` (L6), `parserTypeName` (L12), `parserTypeName` (L17), `matcher` (L21), `parserTypeName` (L27), `matchers` (L28), `parserTypeNameExact` (L34), `someNode` (L36), `parserTypeNameExact` (L45), `someNode` (L49), `valueSet` (L56), `parserTypeNameNot` (L63), `matcher` (L67), `parserTypeNameNot` (L74), `matchers` (L75). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyParserTypeName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4a3490995c0649b4"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyPossibleTypes.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `21fe4a1fa34702e6e33bd7914991fac0f40de529`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyPossibleTypes.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-db40d44c107844dff9.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyPossibleTypes.scala, lines 5–14](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyPossibleTypes.scala#L5-L14). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dea11670bf7b1d90d0ef7de7dcdfd00b6518a03363c6c3c2c468fa9d8299925a`.

> 
> final class TraversalPropertyPossibleTypes[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasPossibleTypesEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to possibleTypes property */
>   def possibleTypes: Iterator[String] =
>     traversal.flatMap(_.possibleTypes)
> 
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 3 lexically identified declarations, including `TraversalPropertyPossibleTypes` (L6), `traversal` (L7), `possibleTypes` (L11). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyPossibleTypes.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4c8c53ab98cc3c19"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyRoot.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `736a27ec6d399b19270b43cf13efb67e60de5def`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyRoot.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3d1768076d1f270a78.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyRoot.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyRoot.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2adfb3a521ed36233b9567d2f3612909caba5216ea4c3b148dfc721af7a1ed66`.

> 
> final class TraversalPropertyRoot[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasRootEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to root property */
>   def root: Iterator[String] =
>     traversal.map(_.root)
> 
>   /** Traverse to nodes where the root matches the regular expression `value`
>     */
>   def root(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       rootExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyRoot` (L6), `traversal` (L7), `root` (L11), `root` (L16), `matcher` (L20), `root` (L26), `matchers` (L27), `rootExact` (L33), `someNode` (L35), `rootExact` (L44), `someNode` (L48), `valueSet` (L55), `rootNot` (L62), `matcher` (L66), `rootNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyRoot.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c50cc540299fa202"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertySignature.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4e55734281c348eb03749b9f30d94eb43412860c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertySignature.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fe2c85aa7c2c2e3e35.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertySignature.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertySignature.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e38efb86bb4ede1e0a4ac3240b41ee16d599a32e4011e3edf57fe5c002686347`.

> 
> final class TraversalPropertySignature[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasSignatureEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to signature property */
>   def signature: Iterator[String] =
>     traversal.map(_.signature)
> 
>   /** Traverse to nodes where the signature matches the regular expression `value`
>     */
>   def signature(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       signatureExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertySignature` (L6), `traversal` (L7), `signature` (L11), `signature` (L16), `matcher` (L20), `signature` (L26), `matchers` (L27), `signatureExact` (L33), `someNode` (L35), `signatureExact` (L44), `someNode` (L48), `valueSet` (L55), `signatureNot` (L62), `matcher` (L66), `signatureNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertySignature.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-905309618057e498"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyStaticReceiver.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7278dc9762f4cf328c0b17a66c78d8f1a357e234`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyStaticReceiver.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-91e5afa9b806f66e02.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyStaticReceiver.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyStaticReceiver.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3b780978a37068c7a2efb8a6c484021646809035c76eb66428c48c5803bd1b40`.

> 
> final class TraversalPropertyStaticReceiver[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasStaticReceiverEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to staticReceiver property */
>   def staticReceiver: Iterator[String] =
>     traversal.flatMap(_.staticReceiver)
> 
>   /** Traverse to nodes where the staticReceiver matches the regular expression `value`
>     */
>   def staticReceiver(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       staticReceiverExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 20 lexically identified declarations, including `TraversalPropertyStaticReceiver` (L6), `staticReceiver` (L12), `staticReceiver` (L17), `matcher` (L21), `tmp` (L23), `staticReceiver` (L30), `matchers` (L31), `tmp` (L33), `staticReceiverExact` (L39), `someNode` (L41), `tmp` (L47), `staticReceiverExact` (L53), `valueSet` (L56), `tmp` (L58), `staticReceiverNot` (L64), `matcher` (L68), `tmp` (L70), `staticReceiverNot` (L77), `matchers` (L78), `tmp` (L80). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyStaticReceiver.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-98c3444297a415a2"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeDeclFullName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e973a1dd3c813aad4623df730389b88ae3e88717`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeDeclFullName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bbeea8e0be1412b98c.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeDeclFullName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeDeclFullName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `47276b6fafbca902be8a34bd6b439d8dc2f5e0f449215d10b0c274124efd092b`.

> 
> final class TraversalPropertyTypeDeclFullName[
>   NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasTypeDeclFullNameEMT]
> ](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to typeDeclFullName property */
>   def typeDeclFullName: Iterator[String] =
>     traversal.map(_.typeDeclFullName)
> 
>   /** Traverse to nodes where the typeDeclFullName matches the regular expression `value`
>     */
>   def typeDeclFullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       typeDeclFullNameExact(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalPropertyTypeDeclFullName` (L6), `typeDeclFullName` (L12), `typeDeclFullName` (L17), `matcher` (L21), `typeDeclFullName` (L27), `matchers` (L28), `typeDeclFullNameExact` (L34), `someNode` (L36), `typeDeclFullNameExact` (L45), `someNode` (L49), `valueSet` (L56), `typeDeclFullNameNot` (L63), `matcher` (L67), `typeDeclFullNameNot` (L74), `matchers` (L75). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeDeclFullName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c2a2fb360dab6e85"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeFullName.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7481fc204777d5b5c21394ddd4ce7a98bfdfc1e8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeFullName.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-63c6c564e10edafbe0.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeFullName.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeFullName.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e9c272a42e6501897a3165cc3d0a53236b63331268433844fd7e5e6942268931`.

> 
> final class TraversalPropertyTypeFullName[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasTypeFullNameEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to typeFullName property */
>   def typeFullName: Iterator[String] =
>     traversal.map(_.typeFullName)
> 
>   /** Traverse to nodes where the typeFullName matches the regular expression `value`
>     */
>   def typeFullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       typeFullNameExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyTypeFullName` (L6), `traversal` (L7), `typeFullName` (L11), `typeFullName` (L16), `matcher` (L20), `typeFullName` (L26), `matchers` (L27), `typeFullNameExact` (L33), `someNode` (L35), `typeFullNameExact` (L44), `someNode` (L48), `valueSet` (L55), `typeFullNameNot` (L62), `matcher` (L66), `typeFullNameNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyTypeFullName.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b438b9a5b94daeee"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyValue.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `de6e0c81604068f6c285427c801634f55a6a76c7`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyValue.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f2de5989045809c52e.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyValue.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyValue.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1455c4118a6595d20da8abbbb690dae600f80e0416faeb2af50db1ce9f6b7440`.

> 
> final class TraversalPropertyValue[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasValueEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to value property */
>   def value: Iterator[String] =
>     traversal.map(_.value)
> 
>   /** Traverse to nodes where the value matches the regular expression `value`
>     */
>   def value(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       valueExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyValue` (L6), `traversal` (L7), `value` (L11), `value` (L16), `matcher` (L20), `value` (L26), `matchers` (L27), `valueExact` (L33), `someNode` (L35), `valueExact` (L44), `someNode` (L48), `valueSet` (L55), `valueNot` (L62), `matcher` (L66), `valueNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyValue.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6938de2ab1cae15a"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyVersion.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `892b50385b518bbd9d93f1075eaf2147df69eb9e`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyVersion.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-deba5c7169d28bdb5b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyVersion.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyVersion.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e8ddbaa363a14fe77ba0ef6d906ef3d30592ecadf490c63f0003d3e6e589c8d4`.

> 
> final class TraversalPropertyVersion[NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasVersionEMT]](
>   val traversal: Iterator[NodeType]
> ) extends AnyVal {
> 
>   /** Traverse to version property */
>   def version: Iterator[String] =
>     traversal.map(_.version)
> 
>   /** Traverse to nodes where the version matches the regular expression `value`
>     */
>   def version(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       versionExact(pattern)
>     } else {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 16 lexically identified declarations, including `TraversalPropertyVersion` (L6), `traversal` (L7), `version` (L11), `version` (L16), `matcher` (L20), `version` (L26), `matchers` (L27), `versionExact` (L33), `someNode` (L35), `versionExact` (L44), `someNode` (L48), `valueSet` (L55), `versionNot` (L62), `matcher` (L66), `versionNot` (L73), `matchers` (L74). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalPropertyVersion.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8524d3b872f20ee7"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalReturnBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `3913eb79c40ab9ac5a66896af66ed7ba95d40d32`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalReturnBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bb389b53ac81542cc0.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalReturnBase.scala, lines 5–6](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalReturnBase.scala#L5-L6). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `849fd3c825f809244bf170f8f1f7287a891f6235bc8c331b665b4365794c87a7`.

> 
> final class TraversalReturnBase[NodeType <: nodes.ReturnBase](val traversal: Iterator[NodeType]) extends AnyVal {}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 1 lexically identified declarations, including `TraversalReturnBase` (L6). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalReturnBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ef9d3450d7c3e812"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `badae0107c71a94c850404481200a7ee79d412d8`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d294ad0acb2373f1ca.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1355a57d5941251c95dfa3ec097036eb624fa6f49a8a6a2d8334066f4a56829d`.

> 
> final class TraversalTagBase[NodeType <: nodes.TagBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       nameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.name).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 29 lexically identified declarations, including `TraversalTagBase` (L6), `name` (L9), `name` (L14), `matcher` (L18), `name` (L24), `matchers` (L25), `nameExact` (L31), `someNode` (L33), `nameExact` (L42), `someNode` (L46), `valueSet` (L53), `nameNot` (L60), `matcher` (L64), `nameNot` (L71), `matchers` (L72), `value` (L77), `value` (L82), `matcher` (L86), `value` (L92), `matchers` (L93), `valueExact` (L99), `someNode` (L101), `valueExact` (L110), `someNode` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-89b2a2d5948048e5"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagnodepairBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `6977bfb3e970a38bbeb0455fd4a84ec4ffa2b1d3`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagnodepairBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a84fe7e9b563cd815b.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagnodepairBase.scala, lines 5–7](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagnodepairBase.scala#L5-L7). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7d83268a799239b30a22db1c222838858acdbd8545201f6da8a0e70a550caf97`.

> 
> final class TraversalTagnodepairBase[NodeType <: nodes.TagNodePairBase](val traversal: Iterator[NodeType])
>     extends AnyVal {}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 1 lexically identified declarations, including `TraversalTagnodepairBase` (L6). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTagnodepairBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3a996e27dbc169fe"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTemplatedomBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `361c4519116162b2cdd8e9cb32c9a24537f010bb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTemplatedomBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c851f2510ddcdeed95.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTemplatedomBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTemplatedomBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3d427ce009776908e3d431feb7b15bf8651444c78ecb76acbf62efede11d40f5`.

> 
> final class TraversalTemplatedomBase[NodeType <: nodes.TemplateDomBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       nameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalTemplatedomBase` (L6), `name` (L10), `name` (L15), `matcher` (L19), `name` (L25), `matchers` (L26), `nameExact` (L32), `someNode` (L34), `nameExact` (L43), `someNode` (L47), `valueSet` (L54), `nameNot` (L61), `matcher` (L65), `nameNot` (L72), `matchers` (L73). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTemplatedomBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-16cdd9fa04c4f706"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `4cf0c702bbe1349f6809fdc3483862a2127c760c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d70c9c5d81ca1d4500.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b55b07ec0cb7b935d70acb34d46f37433d4b0be4f69a9c4c3063dd7a2bce7116`.

> 
> final class TraversalTypeBase[NodeType <: nodes.TypeBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to fullName property */
>   def fullName: Iterator[String] =
>     traversal.map(_.fullName)
> 
>   /** Traverse to nodes where the fullName matches the regular expression `value`
>     */
>   def fullName(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       fullNameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.fullName).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 43 lexically identified declarations, including `TraversalTypeBase` (L6), `fullName` (L9), `fullName` (L14), `matcher` (L18), `fullName` (L24), `matchers` (L25), `fullNameExact` (L31), `someNode` (L33), `fullNameExact` (L42), `someNode` (L46), `valueSet` (L53), `fullNameNot` (L60), `matcher` (L64), `fullNameNot` (L71), `matchers` (L72), `name` (L77), `name` (L82), `matcher` (L86), `name` (L92), `matchers` (L93), `nameExact` (L99), `someNode` (L101), `nameExact` (L110), `someNode` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5ae2b8dae00e2c97"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeargumentBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `88096e1ebae62bb91a35b0793064537eeb615282`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeargumentBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3afb41095e36164280.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeargumentBase.scala, lines 5–7](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeargumentBase.scala#L5-L7). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5dc008f94d86df19a17e26c2f7d760c2a7732c356981133ec914430346de519d`.

> 
> final class TraversalTypeargumentBase[NodeType <: nodes.TypeArgumentBase](val traversal: Iterator[NodeType])
>     extends AnyVal {}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 1 lexically identified declarations, including `TraversalTypeargumentBase` (L6). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeargumentBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fd3e1ddaec571f93"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypedeclBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c2f0979065b5547698066fd9c337f99a8039bc07`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypedeclBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f267e0e9dfb53362d1.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypedeclBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypedeclBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1e93d7f7e6c79ff0c9cd8a5219ff40d758ea1e63596bb1473af8fca3193e6046`.

> 
> final class TraversalTypedeclBase[NodeType <: nodes.TypeDeclBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to aliasTypeFullName property */
>   def aliasTypeFullName: Iterator[String] =
>     traversal.flatMap(_.aliasTypeFullName)
> 
>   /** Traverse to nodes where the aliasTypeFullName matches the regular expression `value`
>     */
>   def aliasTypeFullName(pattern: String): Iterator[NodeType] = {
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       aliasTypeFullNameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item =>


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 107 lexically identified declarations, including `TraversalTypedeclBase` (L6), `aliasTypeFullName` (L9), `aliasTypeFullName` (L14), `matcher` (L18), `tmp` (L20), `aliasTypeFullName` (L27), `matchers` (L28), `tmp` (L30), `aliasTypeFullNameExact` (L36), `someNode` (L38), `tmp` (L44), `aliasTypeFullNameExact` (L50), `valueSet` (L53), `tmp` (L55), `aliasTypeFullNameNot` (L61), `matcher` (L65), `tmp` (L67), `aliasTypeFullNameNot` (L74), `matchers` (L75), `tmp` (L77), `astParentFullName` (L82), `astParentFullName` (L87), `matcher` (L91), `astParentFullName` (L97). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypedeclBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c219082089243996"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeparameterBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `138aa48aafad454fa98dbd387d08f223b6e53d01`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeparameterBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5a8874cd110ba04073.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeparameterBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeparameterBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `fb4d85142f08966842b25e96030c6ce653e59f04b409a68f03102f0be50647f3`.

> 
> final class TraversalTypeparameterBase[NodeType <: nodes.TypeParameterBase](val traversal: Iterator[NodeType])
>     extends AnyVal {
> 
>   /** Traverse to name property */
>   def name: Iterator[String] =
>     traversal.map(_.name)
> 
>   /** Traverse to nodes where the name matches the regular expression `value`
>     */
>   def name(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       nameExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 15 lexically identified declarations, including `TraversalTypeparameterBase` (L6), `name` (L10), `name` (L15), `matcher` (L19), `name` (L25), `matchers` (L26), `nameExact` (L32), `someNode` (L34), `nameExact` (L43), `someNode` (L47), `valueSet` (L54), `nameNot` (L61), `matcher` (L65), `nameNot` (L72), `matchers` (L73). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTypeparameterBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5de84808858b28ba"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTyperefBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `df6e98f6a1e30896221c289b054c33dcf6fd07eb`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTyperefBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0f3e1c016f2c9bacf8.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTyperefBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTyperefBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc712f1210c258a5ca2d4edd28e198f631228baccf145b0b70aa32f8e796aa7c`.

> 
> final class TraversalTyperefBase[NodeType <: nodes.TypeRefBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to dynamicTypeHintFullName property */
>   def dynamicTypeHintFullName: Iterator[String] =
>     traversal.flatMap(_.dynamicTypeHintFullName)
> 
>   /** Traverse to possibleTypes property */
>   def possibleTypes: Iterator[String] =
>     traversal.flatMap(_.possibleTypes)
> 
>   /** Traverse to typeFullName property */
>   def typeFullName: Iterator[String] =
>     traversal.map(_.typeFullName)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 17 lexically identified declarations, including `TraversalTyperefBase` (L6), `dynamicTypeHintFullName` (L9), `possibleTypes` (L13), `typeFullName` (L17), `typeFullName` (L22), `matcher` (L26), `typeFullName` (L32), `matchers` (L33), `typeFullNameExact` (L39), `someNode` (L41), `typeFullNameExact` (L50), `someNode` (L54), `valueSet` (L61), `typeFullNameNot` (L68), `matcher` (L72), `typeFullNameNot` (L79), `matchers` (L80). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalTyperefBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0d43ba3eb791697c"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalUnknownBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `676205fabdfc003cbb59fa5cc0abf0118b394f2f`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalUnknownBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fdf13ca4d2c5ba103e.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalUnknownBase.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalUnknownBase.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5e5769e4f7f83d6c2eecb42501c4634dc75ed585fa56f4fd0493764671f6920f`.

> 
> final class TraversalUnknownBase[NodeType <: nodes.UnknownBase](val traversal: Iterator[NodeType]) extends AnyVal {
> 
>   /** Traverse to containedRef property */
>   def containedRef: Iterator[String] =
>     traversal.map(_.containedRef)
> 
>   /** Traverse to nodes where the containedRef matches the regular expression `value`
>     */
>   def containedRef(pattern: String): Iterator[NodeType] =
>     if (!flatgraph.misc.Regex.isRegex(pattern)) {
>       containedRefExact(pattern)
>     } else {
>       val matcher = flatgraph.misc.Regex.multilineMatcher(pattern)
>       traversal.filter { item => matcher.reset(item.containedRef).matches }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 45 lexically identified declarations, including `TraversalUnknownBase` (L6), `containedRef` (L9), `containedRef` (L14), `matcher` (L18), `containedRef` (L24), `matchers` (L25), `containedRefExact` (L31), `someNode` (L33), `containedRefExact` (L42), `someNode` (L46), `valueSet` (L53), `containedRefNot` (L60), `matcher` (L64), `containedRefNot` (L71), `matchers` (L72), `dynamicTypeHintFullName` (L77), `parserTypeName` (L81), `parserTypeName` (L86), `matcher` (L90), `parserTypeName` (L96), `matchers` (L97), `parserTypeNameExact` (L103), `someNode` (L105), `parserTypeNameExact` (L114). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/TraversalUnknownBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-aced16fd72ab2f4b"></a>


### File `domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/package.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `6c4d5e11a2c8026f942f1ab8f6bb53d54f28248c`. **Disposition:** `GENERATED_PRODUCTION`. **Destination:** `cpg-rs/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/package.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0965baa1ff6429437d.** [cpg/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/package.scala, lines 7–21](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/package.scala#L7-L21). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `44bae4bb943c301067a4dda77d80f47d19072bb0b190e1974b6899d0eb153bff`.

> 
>   /** not supposed to be used directly by users, hence the `bootstrap` in the name */
>   object languagebootstrap extends ConcreteStoredConversions
> 
>   trait ConcreteStoredConversions extends ConcreteBaseConversions {
>     implicit def accessPropertyAliasTypeFullNameTraversal[
>       NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasAliasTypeFullNameEMT]
>     ](traversal: IterableOnce[NodeType]): TraversalPropertyAliasTypeFullName[NodeType] =
>       new TraversalPropertyAliasTypeFullName(traversal.iterator)
>     implicit def accessPropertyArgumentIndexTraversal[
>       NodeType <: nodes.StoredNode & nodes.StaticType[nodes.HasArgumentIndexEMT]
>     ](traversal: IterableOnce[NodeType]): TraversalPropertyArgumentIndex[NodeType] = new TraversalPropertyArgumentIndex(
>       traversal.iterator
>     )
>     implicit def accessPropertyArgumentLabelTraversal[


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Regenerate this exact file from its canonical schema/protocol/typed declarations. Preserve all source-defined fields, methods, values and relationships; do not manually maintain the generated body or exclude it from coverage.

2. Account for 106 lexically identified declarations, including `languagebootstrap` (L9), `ConcreteStoredConversions` (L11), `accessPropertyAliasTypeFullNameTraversal` (L12), `accessPropertyArgumentIndexTraversal` (L16), `accessPropertyArgumentLabelTraversal` (L21), `accessPropertyArgumentNameTraversal` (L26), `accessPropertyAstParentFullNameTraversal` (L31), `accessPropertyAstParentTypeTraversal` (L35), `accessPropertyCanonicalNameTraversal` (L40), `accessPropertyClosureBindingIdTraversal` (L45), `accessPropertyCodeTraversal` (L49), `accessPropertyColumnNumberTraversal` (L52), `accessPropertyColumnNumberEndTraversal` (L57), `accessPropertyContainedRefTraversal` (L61), `accessPropertyContentTraversal` (L66), `accessPropertyControlStructureTypeTraversal` (L69), `accessPropertyDependencyGroupIdTraversal` (L73), `accessPropertyDispatchTypeTraversal` (L77), `accessPropertyDynamicTypeHintFullNameTraversal` (L82), `accessPropertyEvaluationStrategyTraversal` (L86), `accessPropertyEvidenceDescriptionTraversal` (L90), `accessPropertyExplicitAsTraversal` (L94), `accessPropertyFilenameTraversal` (L99), `accessPropertyFullNameTraversal` (L102). The full declaration list is recorded under `cpg:domainClasses/src/main/generated/io/shiftleft/codepropertygraph/generated/traversals/package.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Use native Rust ownership and errors while preserving required CPG operations and data ordering. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

