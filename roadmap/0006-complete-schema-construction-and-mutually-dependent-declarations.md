<a id="m0006"></a>


# M0006 — Complete schema construction and mutually dependent declarations


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0006-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0002](0002-pinned-cpg-source-build-machinery-and-rust-support-boundaries.md)


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


1. Preserve SchemaBase, Base and Operators spelling and paths. Establish typed schema property/node/edge/constant handles, shared builder ownership, declaration order and source equality/identity distinctions.

2. Reconcile reusable CPGSchema and cpg-rs schema implementations at the source-declaration level. The two recorded CPG revisions have identical schema/src/main/scala blobs; this removes that text-difference concern but proves no Rust behavior or builder compatibility.

3. Use CGP providers for schema construction, native value/graph ownership and effects; derive-deftly generates repeated declaration projections. Retain signed widths, raw floating bits, UTF-16 values, nulls and scalar-list/cardinality distinctions.

4. Construct FileSystem after Base, Namespace after Base/FileSystem, and Type after Base/FileSystem. Preserve independent Configuration and MetaData declarations and exact type/property/default/cardinality definitions.

5. Retain caller-supplied builder handles and aliases; later modules may mutate earlier definitions. Do not flatten the object model into a name-only JSON map.

6. Reuse existing native module implementations by exact source path and symbols, recording unverified reuse rather than subtracting old candidate counts as if proved.

7. Construct Method after type/file/base definitions, Ast after namespace/method/type/file definitions, and CallGraph after Method/Ast.

8. Retain AST ordering, argument/parameter indices, method/type references, inheritance, contained relationships and source defaults. Preserve source module and class spellings.

9. Express module composition through CGP and repeated declarations through derive-deftly. Keep individual module APIs available; a complete factory alone does not replace them.

10. Construct Cfg, Dominators and Pdg from their already implemented Method/Ast dependencies. Preserve edge direction, endpoints, ordering, property values and layer relationships.

11. Do not equate a named graph edge with a completed analysis algorithm. Implement the actual source-owned schema behavior and keep external analysis boundaries explicit.

12. Generate repetitive typed metadata/access paths from the canonical declarations under the joint CGP/derive-deftly architecture.

13. Follow exact constructor dependencies for Shortcuts, Comment, Tags, Binding, Annotation, Finding and Hidden. Hidden is composed after its CFG/PDG/tag/shortcut dependencies.

14. Retain hidden nodes/edges/properties, duplicate declarations, inherited fields, source-specific links and mutation order. Do not use the filtered public JSON export as the complete internal schema.

15. Reuse the pinned source facts once in canonical declarations; derive API structure without dropping modules unused by Agones.

16. Implement CpgSchema, ProtoIds, ProtoSerialize and Protogen with the original names and declaration order. Compose every layer and preserve protobuf identifiers, defaults, reserved fields and options.

17. Retain build-time factories, individual parameterized APIs and mutation/equality/identity behavior. Separate immutable resolved export models from mutable source builder handles.

18. Author complete native generation inputs and provider/template wiring. Execute emission only through already available permitted generators; record unavailable fresh builds as blockers.


## Architecture, readability and authored outputs


1. Own 25 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema` -> module `schema::src::main::scala::io::shiftleft::codepropertygraph::schema`


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


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-1c7edc263e41edff"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Annotation.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `488bd9059f71ace5e0fac0e09ed59b3133ae620e`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Annotation.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fb13cc927a5036b007.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Annotation.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Annotation.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `94c0771278a0e942ade8c3eaed04d2a27daa9d46f4594977a49168cb22bd800c`.

> 
> object Annotation extends SchemaBase {
>   def docIndex: Int = 20
> 
>   override def providedByFrontend: Boolean = true
> 
>   override def description: String =
>     """
>       |Java Annotation related CPG definitions.
>       |""".stripMargin
> 
>   def apply(
>     builder: SchemaBuilder,
>     base: Base.Schema,
>     methodSchema: Method.Schema,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 12 lexically identified declarations, including `Annotation` (L5), `docIndex` (L6), `providedByFrontend` (L8), `description` (L10), `apply` (L15), `Schema` (L25), `schemaInfo` (L40), `annotation` (L41), `annotationParameterAssign` (L54), `annotationParameter` (L62), `annotationLiteral` (L67), `arrayInitializer` (L73). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Annotation.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Annotation/mod.rs` (SHA-256 `f39a4d41d76726c32516b5d27832e5e5f214df308c96dae1b439e2342f2b4f18`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b0b149b8f113de53"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f29f880f4baa0b56717b644fb7495c6a9c9bd451`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5c13aaac78200bca19.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.scala, lines 7–21](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.scala#L7-L21). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dd3fe0ad05f427e94fedcaff66234a64014f868e1fd203fafd9519fe4886fb93`.

> 
> object Ast extends SchemaBase {
> 
>   def docIndex: Int                        = 7
>   override def providedByFrontend: Boolean = true
> 
>   override def description: String =
>     """
>       |The Abstract Syntax Tree (AST) Layer provides syntax trees for all compilation units.
>       |All nodes of the tree inherit from the same base class (`AST_NODE`) and are connected
>       |to their child nodes via outgoing `AST` edges.
>       |
>       |Syntax trees are typed, that is, when possible, types for all expressions are stored
>       |in the tree. Moreover, common control structure types are defined in the specification,
>       |making it possible to translate trees into corresponding control flow graphs if only


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 42 lexically identified declarations, including `Ast` (L8), `docIndex` (L10), `providedByFrontend` (L11), `description` (L13), `apply` (L31), `Schema` (L41), `schemaInfo` (L49), `order` (L58), `astNode` (L70), `block` (L109), `literal` (L126), `local` (L139), `identifier` (L153), `canonicalName` (L165), `fieldIdentifier` (L181), `modifierType` (L199), `modifierTypes` (L210), `modifier` (L259), `jumpTarget` (L271), `jumpLabel` (L284), `methodRef` (L297), `typeRef` (L309), `ret` (L314), `controlStructureType` (L326). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Ast.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Ast/mod.rs` (SHA-256 `ce94595c3cc82f9cc65b5c6b6b31dcaf1c5165a1d98aa3253538c97df213e2db`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-55f54031d05519f0"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Base.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `98d52580a7f338b2e9bfec24fffe5a846e18fb7f`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Base.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-187f34d3acd0414fee.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Base.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Base.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1553bf86adb331409c8f7123f3a91320c854aabe2990203d17d38d32ea5607ab`.

> 
> object Base extends SchemaBase {
> 
>   def docIndex: Int                        = Int.MaxValue
>   override def providedByFrontend: Boolean = true
>   override def description: String =
>     """
>       |""".stripMargin
> 
>   def apply(builder: SchemaBuilder) = new Schema(builder)
> 
>   class Schema(builder: SchemaBuilder) {
>     implicit private val schemaInfo: SchemaInfo = SchemaInfo.forClass(getClass)
> 
>     val version = builder


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 21 lexically identified declarations, including `Base` (L7), `docIndex` (L9), `providedByFrontend` (L10), `description` (L11), `apply` (L15), `Schema` (L17), `schemaInfo` (L18), `version` (L20), `hash` (L31), `code` (L45), `isExternal` (L54), `index` (L66), `name` (L82), `fullName` (L91), `parserTypeName` (L103), `value` (L112), `content` (L121), `astParentType` (L134), `astParentFullName` (L146), `declaration` (L156), `ref` (L162). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Base.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Base/mod.rs` (SHA-256 `91ba76417869b84f2ef2ecbc47bffdbbda02139369a581d2f99672c559d94e3f`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1307dfc874358b60"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Binding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b55f95065dc72c002fe4393ef19348d62e8a876e`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Binding.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bf58202e12b68024fb.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Binding.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Binding.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b57e1de1f66f1789e17bb15d6fca1d75295831ddbdfc4e9735e3f0abb4faea3e`.

> 
> object Binding extends SchemaBase {
> 
>   def apply(
>     builder: SchemaBuilder,
>     base: Base.Schema,
>     typeSchema: Type.Schema,
>     methodSchema: Method.Schema,
>     callGraphSchema: CallGraph.Schema
>   ) =
>     new Schema(builder, base, typeSchema, methodSchema, callGraphSchema)
> 
>   override def docIndex: Int = 19
> 
>   override def description: String =


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `Binding` (L5), `apply` (L7), `docIndex` (L16), `description` (L18), `Schema` (L27), `schemaInfo` (L39), `binding` (L41), `binds` (L54). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Binding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Binding/mod.rs` (SHA-256 `c947c6e22c282bcc7fc92a6b50a9cb746e65de7afcb8a71a7891b98df5656557`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-021c551c0c79439e"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CallGraph.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `2adade14b90866a4a397775a496f2af603074c67`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CallGraph.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-22d3328b47c1c2ce91.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CallGraph.scala, lines 7–21](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CallGraph.scala#L7-L21). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `897c6a46ae6861a9d15df61953e63b35d24bdba03e73ff1322abb258a15d0c9a`.

> 
> object CallGraph extends SchemaBase {
> 
>   def docIndex: Int = 8
> 
>   override def description: String =
>     """
>       |The Call Graph Layer represents call relations between methods.
>       |""".stripMargin
> 
>   def apply(builder: SchemaBuilder, methodSchema: Method.Schema, astSchema: Ast.Schema) =
>     new Schema(builder, methodSchema, astSchema: Ast.Schema)
> 
>   class Schema(builder: SchemaBuilder, methodSchema: Method.Schema, astSchema: Ast.Schema) {
>     implicit private val schemaInfo: SchemaInfo = SchemaInfo.forClass(getClass)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 18 lexically identified declarations, including `CallGraph` (L8), `docIndex` (L10), `description` (L12), `apply` (L17), `Schema` (L20), `schemaInfo` (L21), `argumentIndex` (L25), `argumentName` (L42), `argumentLabel` (L56), `methodFullName` (L72), `evaluationStrategy` (L84), `evaluationStrategies` (L100), `dispatchType` (L128), `call` (L142), `argument` (L155), `receiver` (L165), `dispatchTypes` (L176), `staticReceiver` (L198). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CallGraph.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/CallGraph/mod.rs` (SHA-256 `19c525f1a211ed49584572c6b324dd2593fdc40df14c5bcf4c3d5f49e5a11633`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-514077b89e106019"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Cfg.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `76b1a527a8c3e2a75c0035bdfc47b40eb0142819`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Cfg.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-04aedfe93414bf11bb.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Cfg.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Cfg.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6425a2578b20fc8c0c9283fc74e47f3aac22b471fff2904b710c51998d2be7b6`.

> 
> object Cfg extends SchemaBase {
> 
>   def docIndex: Int = 9
> 
>   override def description: String =
>     """
>       |The Control Flow Graph Layer provides control flow graphs for each method. Control
>       |flow graphs are constructed by marking a sub set of the abstract syntax tree nodes
>       |as control flow nodes (`CFG_NODE`) and connecting these nodes via `CFG` edges.
>       |The control flow graph models both the control flow within the calculation of an
>       |expression as well as from expression to expression. The layer can be automatically
>       |generated from the syntax tree layer if only control structure types supported by
>       |this specification are employed.
>       |""".stripMargin


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `Cfg` (L6), `docIndex` (L8), `description` (L10), `apply` (L21), `Schema` (L24), `schemaInfo` (L25), `cfgNode` (L29), `cfg` (L54). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Cfg.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Cfg/mod.rs` (SHA-256 `82e2690acfda7e7e01c2699757c478bb8ce0ecdc6aa4bb9b19fa8f95c98feb9d`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-930b06c0f4a8eac3"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Comment.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `6d620e8aa35e2f61aabb3ac233e47e2fa750021a`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Comment.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d92f2ab313fc557e3a.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Comment.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Comment.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b556ac97cabc9b3360b196dcb2354c193ac37f2d1369813da047e0cee81c6505`.

> 
> object Comment extends SchemaBase {
>   def docIndex                    = 12
>   override def providedByFrontend = true
> 
>   override def description = ""
> 
>   def apply(builder: SchemaBuilder, base: Base.Schema, ast: Ast.Schema, fs: FileSystem.Schema) =
>     new Schema(builder, base, ast, fs)
> 
>   class Schema(builder: SchemaBuilder, baseSchema: Base.Schema, astSchema: Ast.Schema, fs: FileSystem.Schema) {
>     import astSchema._
>     import fs._
>     import baseSchema._
>     implicit private val schemaInfo: SchemaInfo = SchemaInfo.forClass(getClass)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `Comment` (L5), `docIndex` (L6), `providedByFrontend` (L7), `description` (L9), `apply` (L11), `Schema` (L14), `schemaInfo` (L18), `comment` (L20). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Comment.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Comment/mod.rs` (SHA-256 `7f9d0ec5b8a83a692721e0119a6b05141daec1b46d3bda3c119ed214a2895eaa`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cda2af3fd097064c"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Configuration.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `605f19da34081c4ad8d8b43278b5e66e0fad1c92`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Configuration.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-898fcc8f83cd4f3b9a.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Configuration.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Configuration.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `45c5b4773db3afcf1ad7d23d2f78f174802f88f87a7e704ba3a236588af2889a`.

> 
> object Configuration extends SchemaBase {
> 
>   override def docIndex: Int = 18
> 
>   override def description: String =
>     """
>       |The code property graph specification currently does not contain
>       |schema elements for the representation of configuration files in
>       |a structured format, however, it does allow configuration files
>       |to be included verbatim in the graph to enable language-/framework-
>       |specific passes to access them. This layer provides the necessary
>       |schema elements for this basic support of configuration files.
>       |""".stripMargin


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 7 lexically identified declarations, including `Configuration` (L5), `docIndex` (L7), `description` (L9), `apply` (L19), `Schema` (L21), `schemaInfo` (L24), `configFile` (L26). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Configuration.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Configuration/mod.rs` (SHA-256 `987272f59e11a11e08e210381f3248f662f6f6143b6e988fc854fd441272b727`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fa4b4e319fda62da"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CpgSchema.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5d8a083628c17691782a3eb6e24f6f7bc1cd4f20`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CpgSchema.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e3b4c6b2469cee4bdb.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CpgSchema.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CpgSchema.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `92a8940dc1aee7df51aa23acb818446a612c4f59bb6f72ee981342315c5743bd`.

> 
> class CpgSchema(builder: SchemaBuilder) {
> 
>   val base       = Base(builder)
>   val fs         = FileSystem(builder, base)
>   val namespaces = Namespace(builder, base, fs)
> 
>   val operators     = Operators(builder)
>   val configuration = Configuration(builder, base)
>   val metaData      = MetaData(builder, base)
> 
>   val typeSchema = Type(builder, base, fs)
>   val method     = Method(builder, base, typeSchema, fs)
>   val ast        = Ast(builder, base, namespaces, method, typeSchema, fs)
>   val callGraph  = CallGraph(builder, method, ast)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 35 lexically identified declarations, including `CpgSchema` (L5), `base` (L7), `fs` (L8), `namespaces` (L9), `operators` (L11), `configuration` (L12), `metaData` (L13), `typeSchema` (L15), `method` (L16), `ast` (L17), `callGraph` (L18), `cfg` (L20), `dominators` (L21), `pdg` (L22), `shortcuts` (L24), `sourceSpecific` (L26), `tags` (L27), `binding` (L28), `annotation` (L29), `finding` (L30), `hidden` (L31), `protoSerialize` (L32), `CpgSchema` (L35), `instance` (L36). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CpgSchema.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve source floating width, narrowing, exceptional values and relevant bits.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/CpgSchema/mod.rs` (SHA-256 `09ebc3afcb29d2b2df0fed8e4f54f929526e3f7e786911a6aa5cf5bb2306e67d`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-59843c4a0686fe68"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Dominators.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `26e71e8a60962d2b0cec5537b05af4439f421c01`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Dominators.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f490f56446dff434e4.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Dominators.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Dominators.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1e1c134a2f62e4b6c991aea5ef6088ebc6d4b0cfb696d984171d328989c84d02`.

> 
> object Dominators extends SchemaBase {
> 
>   def docIndex: Int = 10
> 
>   override def description: String =
>     """
>       |The Dominators Layer provides dominator- and post-dominator trees for all methods.
>       |It is constructed automatically from the control flow graph layer and is in turn
>       |used to automatically construct control dependence relations of the PDG layer.
>       |""".stripMargin
> 
>   def apply(builder: SchemaBuilder, methodSchema: Method.Schema, ast: Ast.Schema) =
>     new Schema(builder, methodSchema, ast)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `Dominators` (L5), `docIndex` (L7), `description` (L9), `apply` (L16), `Schema` (L19), `schemaInfo` (L20), `dominate` (L24), `postDominate` (L31). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Dominators.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Dominators/mod.rs` (SHA-256 `c195d6d74adac50cf8503a1e4690d9ae8f89370507775da2322e3f42e5e78628`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a20b3bd177debf0e"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/FileSystem.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `bf20a1ad46e7b4758e2bf72d06609a58bae6d36b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/FileSystem.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5f2c5dbaa40014d827.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/FileSystem.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/FileSystem.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0eb2998831e1cf4bfc51b29581efdf9e2a6f0ce7328cbc72c68a633d45181ded`.

> 
> object FileSystem extends SchemaBase {
> 
>   def docIndex: Int = 3
>   override def description: String =
>     """
>       |CPGs are created from sets of files and the File System Layer describes the layout of
>       |these files, that is, it provides information about source files
>       |and shared objects for source-based and machine-code-based frontends
>       |respectively. The purpose of including this information in the CPG
>       |is to allow nodes of the graph to be mapped back to file system locations.
>       |""".stripMargin
> 
>   def apply(builder: SchemaBuilder, base: Base.Schema) =
>     new Schema(builder, base)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 15 lexically identified declarations, including `FileSystem` (L7), `docIndex` (L9), `description` (L10), `apply` (L19), `Schema` (L22), `schemaInfo` (L23), `filename` (L26), `lineNumber` (L40), `columnNumber` (L50), `lineNumberEnd` (L61), `columnNumberEnd` (L72), `sourceFile` (L83), `file` (L93), `offset` (L111), `offsetEnd` (L127). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/FileSystem.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/FileSystem/mod.rs` (SHA-256 `3a61a722188284714e0a39e13b3c70f575530913796c2415dca8cb8cf8c6ec61`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e8b198ae402c8f92"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Finding.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `aa324bf81c06d232a02299535a22670b70453eb7`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Finding.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b3482bd53dc35aae26.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Finding.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Finding.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cbef629f08ebd07dd16354e11038481b62c4017b09ca498d01b51b75d98f8f92`.

> 
> object Finding extends SchemaBase {
> 
>   def docIndex: Int = 15
> 
>   override def description: String =
>     """
>       |We allow findings (e.g., potential vulnerabilities, notes on dangerous practices) to
>       |be stored in the Findings Layer.
>       |""".stripMargin
> 
>   def apply(builder: SchemaBuilder, base: Base.Schema) =
>     new Schema(builder, base)
> 
>   class Schema(builder: SchemaBuilder, base: Base.Schema) {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 10 lexically identified declarations, including `Finding` (L7), `docIndex` (L9), `description` (L11), `apply` (L17), `Schema` (L20), `schemaInfo` (L21), `key` (L25), `finding` (L35), `keyValuePair` (L49), `evidenceDescription` (L57). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Finding.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Finding/mod.rs` (SHA-256 `ebe75b95540783fb225bd241821d9cb1c4c88e92ee67b4993c489343bddd7f4e`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-06ef477bd855bcb8"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Hidden.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e2dc065ae50671169e626ef7cc344a0465da0241`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Hidden.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1550e20a97a1f62ba0.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Hidden.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Hidden.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0ce3c8b931302e730eb5d998b1242b922cd3c7716d19c182046fe4315db014a9`.

> 
> object Hidden extends SchemaBase {
>   override def docIndex: Int = -1
> 
>   override def description: String =
>     """
>       |The schema elements defined here are NOT included on the schema website.
>       |These are schema elements that we are using but would like to rework
>       |in the future and not make part of the standard.
>       |""".stripMargin
> 
>   def apply(
>     builder: SchemaBuilder,
>     base: Base.Schema,
>     methodSchema: Method.Schema,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 26 lexically identified declarations, including `Hidden` (L7), `docIndex` (L8), `description` (L10), `apply` (L17), `Schema` (L31), `schemaInfo` (L55), `closureBindingId` (L61), `capture` (L71), `closureBinding` (L79), `capturedBy` (L87), `templateDOM` (L110), `dependencyGroupId` (L131), `dependency` (L140), `dynamicTypeHintFullName` (L150), `possibleTypes` (L159), `importNode` (L185), `importedEntity` (L196), `importedAs` (L205), `explicitAs` (L216), `isWildcard` (L226), `isExplicit` (L237), `isModuleImport` (L248), `imports` (L265), `isCallForImport` (L269). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Hidden.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

5. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Hidden/mod.rs` (SHA-256 `db9d855bd7a06ede00d96f2ea52590f110c9a5307b1967feff97a2ae9cd208df`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-57ff16f911130d7e"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/MetaData.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `71160001a3ce3a3a971d4e6be37e94b35b60f8ba`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/MetaData.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d7305da5fdb60f4a16.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/MetaData.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/MetaData.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b393a9ae6de5dc5f730be1078bbf950f4673ed796eba2e3d111c0c3c7aceefda`.

> 
> object MetaData extends SchemaBase {
> 
>   def docIndex: Int                        = 2
>   override def providedByFrontend: Boolean = true
> 
>   override def description: String =
>     """
>       |The Meta Data Layer contains information about CPG creation. In particular,
>       |it indicates which language frontend generated the CPG and which overlays
>       |have been applied. The layer consists of a single node - the Meta Data node -
>       |and language frontends MUST create this node. Overlay creators MUST edit
>       |this node to indicate that a layer has been successfully applied in all
>       |cases where applying the layer more than once is prohibitive.
>       |""".stripMargin


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 12 lexically identified declarations, including `MetaData` (L7), `docIndex` (L9), `providedByFrontend` (L10), `description` (L12), `apply` (L22), `Schema` (L24), `schemaInfo` (L26), `overlays` (L28), `language` (L41), `root` (L54), `metaData` (L63), `languages` (L78). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/MetaData.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/MetaData/mod.rs` (SHA-256 `92f843ecdb812c964e16c9a37ffd317aff8da404e857c29ed87029fa9185c341`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a7f6cc1b7a4d2f83"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Method.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `a48e410abce73a9a0e97ffd80051b093404ec1db`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Method.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bbe31051025f8c488f.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Method.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Method.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7643ca415b9218aa0d30d49dca34ee9613ef71c755f5f607994a482d26bf90b6`.

> 
> object Method extends SchemaBase {
> 
>   def apply(builder: SchemaBuilder, base: Base.Schema, typeSchema: Type.Schema, fs: FileSystem.Schema) =
>     new Schema(builder, base, typeSchema, fs)
> 
>   def docIndex: Int                        = 5
>   override def providedByFrontend: Boolean = true
> 
>   override def description: String =
>     """
>       |The Method Layer contains declarations of methods, functions, and procedures.
>       |Input parameters and output parameters (including return parameters) are
>       |represented, however, method contents is not present in this layer.
>       |""".stripMargin


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 13 lexically identified declarations, including `Method` (L6), `apply` (L8), `docIndex` (L11), `providedByFrontend` (L12), `description` (L14), `Schema` (L21), `schemaInfo` (L25), `signature` (L27), `method` (L43), `isVariadic` (L76), `methodParameterIn` (L89), `methodParameterOut` (L102), `methodReturn` (L114). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Method.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Method/mod.rs` (SHA-256 `9b87e01333e6be23190a2cc5c62a99862151ad4ca73a55f5609fb654cf1d37dd`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d96284909597a3c2"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Namespace.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `fd5367835743cfb0a631863d95c9d991471ab8ad`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Namespace.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-289e9dbacd4716cc30.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Namespace.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Namespace.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3dcceb8a131da28890c34077788fd27a841b11fb29130027539846c897698444`.

> 
> object Namespace extends SchemaBase {
> 
>   def docIndex: Int = 4
> 
>   override def description: String =
>     """
>       |Many programming languages allow code to be structured into namespaces. The Namespace
>       |Layer makes these namespaces explicit and associates program constructs with the
>       |namespaces they are defined in.
>       |""".stripMargin
> 
>   def apply(builder: SchemaBuilder, base: Base.Schema, fs: FileSystem.Schema) =
>     new Schema(builder, base, fs)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `Namespace` (L5), `docIndex` (L7), `description` (L9), `apply` (L16), `Schema` (L19), `schemaInfo` (L20), `namespaceBlock` (L24), `namespace` (L48). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Namespace.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Namespace/mod.rs` (SHA-256 `f0ea1cfabcd92b7b90a75086f0dbe0900a9d17131243f1fb675f3b4b80f79b9a`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3261c3459c6c0c53"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Operators.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c280e798b3b2ba4ae0f4630ff8ade15145915ee3`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Operators.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-377c2b30fe07101ba6.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Operators.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Operators.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5a5f67dec4cdc847939c1f083e5b64e4eb053e6b4f4dc153b5e7ee00009b5511`.

> 
> object Operators extends SchemaBase {
> 
>   override def docIndex: Int = 17
> 
>   override def description: String =
>     """
>       |
>       |""".stripMargin
> 
>   def apply(builder: SchemaBuilder) = new Schema(builder)
> 
>   class Schema(builder: SchemaBuilder) {
>     implicit private val schemaInfo: SchemaInfo = SchemaInfo.forClass(getClass)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 7 lexically identified declarations, including `Operators` (L6), `docIndex` (L8), `description` (L10), `apply` (L15), `Schema` (L17), `schemaInfo` (L18), `operators` (L25). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Operators.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Operators/mod.rs` (SHA-256 `04312d3ac89e71e098ba6646de164817b035ce4c2027123df9c9e58ba670447b`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b07735704655273d"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Pdg.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `74dcf96f9643b999397cce95568ea79cf26aa2a1`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Pdg.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-81b9cf521ae9ffd208.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Pdg.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Pdg.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `43c6e82d23f36c029e77f5d6a4f9fe977f8f8df10356eb3a6ca99dab159401e8`.

> 
> object Pdg extends SchemaBase {
> 
>   def docIndex: Int = 11
>   override def description: String =
>     """
>       The Program Dependence Graph Layer contains a program dependence graph for
>       |each method of the source program. A program dependence graph consists
>       |of a data dependence graph (DDG) and a control dependence graph (CDG),
>       |created by connecting nodes of the control flow graph via `REACHING_DEF`
>       |and `CDG` edges respectively.
>       |""".stripMargin
> 
>   def apply(builder: SchemaBuilder, methodSchema: Method.Schema, ast: Ast.Schema) =
>     new Schema(builder, methodSchema, ast)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 9 lexically identified declarations, including `Pdg` (L7), `docIndex` (L9), `description` (L10), `apply` (L19), `Schema` (L22), `schemaInfo` (L23), `variable` (L27), `cdg` (L36), `reachingDef` (L43). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Pdg.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Pdg/mod.rs` (SHA-256 `36627caa7389f557ef90387d47b8664886bfedb8197f5a133f14f81f6e3487b7`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-90b9aad918e86f03"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoIds.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `138fdff7dd5dc53f4e341b6fcfc72a9c28bdf0a4`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoIds.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6cbd510e1e9515fd8e.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoIds.scala, lines 2–16](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoIds.scala#L2-L16). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0251b6a362c4ea92e37e94280c1f67c5bc19e343ce77c903d27fa784177ad8fe`.

> 
> object ProtoIds {
>   val Break                     = 1
>   val Static                    = 1
>   val ByReference               = 1
>   val StaticDispatch            = 1
>   val Java                      = 1
>   val Method                    = 1
>   val Public                    = 2
>   val BySharing                 = 2
>   val Continue                  = 2
>   val DynamicDispatch           = 2
>   val LineNumber                = 2
>   val JavaScript                = 2
>   val Ast                       = 3


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 186 lexically identified declarations, including `ProtoIds` (L3), `Break` (L4), `Static` (L5), `ByReference` (L6), `StaticDispatch` (L7), `Java` (L8), `Method` (L9), `Public` (L10), `BySharing` (L11), `Continue` (L12), `DynamicDispatch` (L13), `LineNumber` (L14), `JavaScript` (L15), `Ast` (L16), `Protected` (L17), `While` (L18), `ParserTypeName` (L19), `ByValue` (L20), `Inlined` (L21), `Golang` (L22), `MethodReturn` (L23), `Private` (L24), `Order` (L25), `Do` (L26). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoIds.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/ProtoIds/mod.rs` (SHA-256 `4f0bf4aba1dec9248a25f6541df41c196ccff0fea8e80aa8d22590bc7fe2a3e2`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6db413bf98a6de9d"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoSerialize.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1b6cc654628cae0d6354429d628bbcddf91f03a6`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoSerialize.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e58f2b5a2cd962ad9b.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoSerialize.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoSerialize.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cb820e96a6a03c0ec82747b7fc18e216f36ec40a9726cf03aba57334612cd94a`.

> 
> object ProtoSerialize extends SchemaBase {
> 
>   override def docIndex: Int = Int.MaxValue
> 
>   override def description: String =
>     """
>       |
>       |""".stripMargin
> 
>   def apply(builder: SchemaBuilder, ast: Ast.Schema) = new Schema(builder, ast)
> 
>   class Schema(builder: SchemaBuilder, ast: Ast.Schema) {
> 
>     import ast._


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 7 lexically identified declarations, including `ProtoSerialize` (L7), `docIndex` (L9), `description` (L11), `apply` (L16), `Schema` (L18), `schemaInfo` (L21), `containedRef` (L23). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/ProtoSerialize.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/ProtoSerialize/mod.rs` (SHA-256 `97925c5099b79d1736e1a92db1e15aa4904b2399458a7547a67da579480844b6`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-57d462d92255e896"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Protogen.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `02b60bd15a535148124a912996c89a088e1bafde`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Protogen.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c4b3efae5a74396d2f.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Protogen.scala, lines 7–21](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Protogen.scala#L7-L21). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f1e5763c43c7a96e6fe28c744afaae03c78bfc22ef52079da9979360e7753363`.

> 
> object Protogen {
>   def main(args: Array[String]): Unit = {
>     val outputDir =
>       args.headOption.map(Paths.get(_)).getOrElse(throw new AssertionError("please pass outputDir as first parameter"))
> 
>     val builder = new SchemaBuilder(domainShortName = "Cpg", basePackage = "io.shiftleft.codepropertygraph.generated")
> 
>     builder.protoOptions(
>       ProtoOptions(
>         pkg = "cpg",
>         javaOuterClassname = "Cpg",
>         javaPackage = "io.shiftleft.proto.cpg",
>         goPackage = "github.com/ShiftLeftSecurity/proto/cpg",
>         csharpNamespace = "io.shiftleft.proto.cpg",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 4 lexically identified declarations, including `Protogen` (L8), `main` (L9), `outputDir` (L10), `builder` (L13). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Protogen.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Protogen/mod.rs` (SHA-256 `8f18059f7dda72dc18a7fc68b481ea5ea93fc023c07aa9702e7349bb25d00695`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-724732f0466bc14b"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/SchemaBase.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0446c74fbd7d6bda0fffd6a131d6d947749159a4`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/SchemaBase.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8577e0ff775bcda4d8.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/SchemaBase.scala, lines 2–8](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/SchemaBase.scala#L2-L8). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `8c14ae004fd3baba83259b4dc261956a3215f629fc064615e0c492278ce4425e`.

> 
> trait SchemaBase {
>   // Used for ordering in the documentation. Smallest index shows up first.
>   def docIndex: Int
>   def description: String
>   def providedByFrontend: Boolean = false
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 4 lexically identified declarations, including `SchemaBase` (L3), `docIndex` (L5), `description` (L6), `providedByFrontend` (L7). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/SchemaBase.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/SchemaBase/mod.rs` (SHA-256 `c7d87a5e44093cb6d280094e0326b3aa5bf3f25d4e99a39f81c188232f15eec6`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-76a84ae88e1d2aa9"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Shortcuts.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `cfa0d7296c5c900816bdd7a9da77ee231696f32d`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Shortcuts.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-51f0a1b1d15f3cb963.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Shortcuts.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Shortcuts.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9cdda8755ede9c1862f47009e6aa5fd4be4153a378742ea18c115e81cd5bcb8a`.

> 
> object Shortcuts extends SchemaBase {
> 
>   def docIndex: Int = 16
> 
>   override def description: String =
>     """
>       |The Shortcuts Layer provides shortcut edges calculated to speed up
>       |subsequent queries. Language frontends MUST NOT create shortcut edges.
>       |""".stripMargin
> 
>   def apply(
>     builder: SchemaBuilder,
>     base: Base.Schema,
>     methodSchema: Method.Schema,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 9 lexically identified declarations, including `Shortcuts` (L6), `docIndex` (L8), `description` (L10), `apply` (L16), `Schema` (L26), `schemaInfo` (L41), `evalType` (L43), `contains` (L47), `parameterLink` (L51). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Shortcuts.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Shortcuts/mod.rs` (SHA-256 `c2c135fce57b350be7f36adab331f0eeaef5b61a4f14ec9b5a027a05e08b7c42`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8836d003eb95fd5a"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Tags.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `824506370a726f4bc5dd57b176d33fc9f295d009`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Tags.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9a8cb12e8002fc6fb0.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Tags.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Tags.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `226143a2fc86d2b10fea5690529cfba4d35bc637a34965f3aaa5509f35bb8c65`.

> 
> object Tags extends SchemaBase {
> 
>   override def docIndex: Int = 17
> 
>   override def description: String =
>     """
>       |The Code Property Graph specification allows for tags to be attached
>       |to arbitrary nodes. Conceptually, this is similar to the creation of
>       |Finding nodes, however, tags are to be used for intermediate results
>       |rather than end-results that are to be reported to the user.
>       |""".stripMargin
> 
>   def apply(
>     builder: SchemaBuilder,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 9 lexically identified declarations, including `Tags` (L5), `docIndex` (L7), `description` (L9), `apply` (L17), `Schema` (L28), `schemaInfo` (L43), `taggedBy` (L47), `tag` (L53), `tagNodePair` (L59). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Tags.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Tags/mod.rs` (SHA-256 `becd310359cd45c2e4d1f65d3f9422beb18bbf92d51287484e4a3a8f66ce169b`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-02b451ee6388b2d2"></a>


### File `schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Type.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8afec07a721541a134d4760f08d1310a067fc983`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Type.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-563f7e9806cf7945fb.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Type.scala, lines 6–20](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Type.scala#L6-L20). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `30f4930c1fff1f3031441ff7a0c506bae4ced145321ce4f3eb1001dcc1e518fe`.

> 
> object Type extends SchemaBase {
> 
>   def apply(builder: SchemaBuilder, base: Base.Schema, fs: FileSystem.Schema) =
>     new Schema(builder, base, fs)
> 
>   def docIndex: Int                        = 6
>   override def providedByFrontend: Boolean = true
> 
>   override def description: String =
>     """
>       |The Type Layer contains information about type declarations, relations between
>       |types, and type instantiation and usage. In its current form, it allows modelling of
>       |parametrized types, type hierarchies and aliases.
>       |""".stripMargin


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 19 lexically identified declarations, including `Type` (L7), `apply` (L9), `docIndex` (L12), `providedByFrontend` (L13), `description` (L15), `Schema` (L22), `schemaInfo` (L25), `typeFullName` (L29), `aliasTypeFullName` (L42), `inheritsFromTypeFullName` (L52), `typeDeclFullName` (L63), `typeDecl` (L76), `typeParameter` (L112), `typeArgument` (L125), `member` (L137), `tpe` (L151), `bindsTo` (L164), `aliasOf` (L173), `inheritsFrom` (L183). The full declaration list is recorded under `cpg:schema/src/main/scala/io/shiftleft/codepropertygraph/schema/Type.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain declaration names, defaults, cardinality, identifiers, relationships and mutable handle identity.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

12. Reuse candidate: `CPGSchema/src/Type/mod.rs` (SHA-256 `e656a68816837a2bfe137f9e806b6852f9a7e7e5ece700b16b174e9e240a02ed`). This file exists and the source schema revisions match textually; behavior equivalence and complete constructor/body coverage remain unverified. Relocate/adapt through an explicit mapping; keep the original candidate intact until adoption.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.

## Constructor order, the source cycle and identity

**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-afcdbee03e94e5224c.** [cpg/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CpgSchema.scala, lines 7–32](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/src/main/scala/io/shiftleft/codepropertygraph/schema/CpgSchema.scala#L7-L32). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `92a8940dc1aee7df51aa23acb818446a612c4f59bb6f72ee981342315c5743bd`.

>   val base       = Base(builder)
>   val fs         = FileSystem(builder, base)
>   val namespaces = Namespace(builder, base, fs)
> 
>   val operators     = Operators(builder)
>   val configuration = Configuration(builder, base)
>   val metaData      = MetaData(builder, base)
> 
>   val typeSchema = Type(builder, base, fs)
>   val method     = Method(builder, base, typeSchema, fs)
>   val ast        = Ast(builder, base, namespaces, method, typeSchema, fs)
>   val callGraph  = CallGraph(builder, method, ast)
> 
>   val cfg        = Cfg(builder, method, ast)
>   val dominators = Dominators(builder, method, ast)
>   val pdg        = Pdg(builder, method, ast)
> 
>   val shortcuts = Shortcuts(builder, base, method, ast, typeSchema, fs)
> 
>   val sourceSpecific = Comment(builder, base, ast, fs)
>   val tags           = Tags(builder, base, typeSchema, method, ast, fs, callGraph)
>   val binding        = Binding(builder, base, typeSchema, method, callGraph)
>   val annotation     = Annotation(builder, base, method, typeSchema, ast, shortcuts)
>   val finding        = Finding(builder, base)
>   val hidden         = Hidden(builder, base, method, typeSchema, ast, cfg, fs, callGraph, pdg, tags, shortcuts)
>   val protoSerialize = ProtoSerialize(builder, ast)

**Derived requirement.** Preserve this exact constructor order and the shared SchemaBuilder/handle identities. CpgSchema constructs the modules, while several modules import CpgSchema.PropertyDefaults. The schema is therefore one cohesive milestone, including defaults, SchemaBase, ProtoIds, ProtoSerialize and Protogen. Establish declarations/constants first inside this milestone, then constructors in the quoted order and finally instance.build. File-entry ordering is an inventory order, not permission to reorder construction or create a second owner of constants.

## Native schema reuse boundary

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d1596e6d3150ce4cb8.** [CPGSchema/README.md, lines 3–17](https://github.com/QuasarRay/CPGSchema/blob/18bdf6122a887bf3dd8afa196235f3cf378a14e7/README.md#L3-L17). Revision `18bdf6122a887bf3dd8afa196235f3cf378a14e7`; source SHA-256 `9f245bb40eb7ff9ac647c35cbe11f279fd76b7ba07251f58fd6023f471d7da68`.

> Implementation milestone: **UNVERIFIED**. Compilation is used during implementation; this milestone includes no tests, proofs, source-comparison runs, or correctness reports.
> 
> The root `CPGSchema` package implements the Code Property Graph schema in native Rust. The independent `cpg-schema-json` package in `JsonSchema/` implements its own JSON Schema dialect, schema interpretation, and graph validation. Neither production implementation depends on the other. Their initial reference is codepropertygraph commit `d02d76ea5c9e3b398317d334228d7e8e78ab45aa` with Flatgraph `0.1.36`.
> 
> The source-compatible modules retain their Scala spelling. `CpgSchema::CpgSchema::new` composes all modules over a shared native `SchemaBuilder`; each module exposes its own `Schema` with typed handles for the definitions it owns. Later modules can modify earlier definitions. Builder storage retains declaration order and handle identity; resolved property and concrete-node accessors use the source's sorted presentation order. Original Scala references and earlier experimental bundles remain available alongside the integrated modules.
> 
> ```rust
> use CPGSchema::{CpgSchema, graph::{Graph, ValidationOptions, ConstructionStage}};
> use CPGSchema::components::{NativeContext, CanProvideSchema, CanValidateGraph};
> 
> let schema = NativeContext.provide_schema()?;
> let graph = Graph::default();
> let options = ValidationOptions::all(&schema, ConstructionStage::Frontend);
> let diagnostics = NativeContext.validate_graph(&schema, &graph, &options);
> # Ok::<(), CPGSchema::Error>(())

**Derived requirement.** Reconcile source blobs and each Rust constructor/body mapping using schema-reconciliation.json. Reuse compatible implementation, preserve original files until references are accounted for, and leave behavior status UNVERIFIED until the global gate.
