<a id="m0008"></a>


# M0008 — Schema export, protobuf tool inputs and launcher behavior


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0008-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0006](0006-complete-schema-construction-and-mutually-dependent-declarations.md)


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


1. Preserve standard Schema2Json export behavior, including hidden-element filtering, while defining a separate complete internal schema representation for translation.

2. Account for schema2json.sh, proto-bindings build behavior and source resource placeholders. Retain launch arguments, errors, output names and declared external-tool boundaries.

3. Use canonical typed declarations, CGP lifecycle/effect providers and derive-deftly structure. Do not make a JVM service the permanent implementation of native schema operations.


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


## Package `cpg:.` -> module `<crate root>`


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


<a id="file-cf5812ddd2cb6853"></a>


### File `schema2json.sh`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `675ff64dc7fa4528a3e2d509a4d53d675e10a7c1`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema2json.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8b884183044a4fde19.** [cpg/schema2json.sh, lines 1–1](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema2json.sh#L1-L1). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5e4280f651bd496fecc801b9eb9f3735aaffcf9d8e1fee31d5e4a09ea9895586`.

> ./schema2json/target/universal/stage/bin/schema2json


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:proto-bindings` -> module `proto-bindings`


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


<a id="file-7f06c47b5ddb4ece"></a>


### File `proto-bindings/build.sbt`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9740f2d36622fcba1ab77fcb87af0ed564c088a5`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/proto-bindings/build.sbt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4bcd97e7b87af04d71.** [cpg/proto-bindings/build.sbt, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/proto-bindings/build.sbt#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6e1a5a230d3ac1ec66cc1040d6f096aa77fd020df521ab6090ceb74b3085ca3f`.

> name := "codepropertygraph-protos"
> 
> lazy val generateProtobuf = taskKey[File]("generate protobuf definitions: cpg.proto")
> 
> enablePlugins(ProtobufPlugin)
> val protocLocalDir   = "protoc"
> val protocBinaryPath = s"$protocLocalDir/bin/protoc"
> ProtobufConfig / protobufProtoc := protocBinaryPath
> ProtobufConfig / version        := "3.20.1"
> ProtobufConfig / sourceDirectories += (ProtobufConfig / protobufExternalIncludePath).value
> ProtobufConfig / protobufGenerate := (ProtobufConfig / protobufGenerate)
>   .dependsOn(copyLatestCpgProto)
>   .dependsOn(installProtoc)
>   .value


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:proto-bindings/src/main/protobuf` -> module `proto-bindings::src::main::protobuf`


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


<a id="file-c633dab209e2f4a8"></a>


### File `proto-bindings/src/main/protobuf/empty-to-avoid-warnings`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e69de29bb2d1d6434b8b29ae775ad8c2e48c5391`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `cpg-rs/proto-bindings/src/main/protobuf/empty-to-avoid-warnings`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-037003a6adf05acaf4.** [cpg/README.md, lines 6–17](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L6-L17). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> You can find a clickable specification at:
> 
> https://cpg.joern.io
> 
> **Note: for first-time users, we recommend building "joern" at https://github.com/joernio/joern/ instead. It combines this repo with a C/C++ language frontend to construct a complete code analysis platform.**
> 
> A Code Property Graph (CPG) is an extensible and language-agnostic
> representation of program code designed for incremental and
> distributed code analysis. This repository hosts the base
> specification together with a build process that generates data
> structure definitions for accessing the graph with different
> programming languages.


**Reference boundary.** Binary/empty resource has no prose to quote; quote the upstream loader/test contract and retain exact blob/hash. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:schema2json` -> module `schema2json`


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


<a id="file-63b22e29a94a4e74"></a>


### File `schema2json/BUILD`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `914443022b5cecc62b61f14d8904d9fd0ec68cda`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:schema2json/BUILD`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8ffaa94a41c1cda577.** [cpg/schema2json/BUILD, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema2json/BUILD#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `0d77b62bac063d5b466ee694daa066914981313741b94e402a83613c0e235caf`.

> load("//bazel:scala.bzl", "scala_binary")
> 
> scala_binary(
>     name = "schema2json",
>     srcs = glob([
>         "src/main/scala/**/*.scala",
>     ]),
>     main_class = "Schema2Json",
>     runtime_deps = [
>         "//schema:domainClassesLinked",
>     ],
>     deps = [
>         "//schema",
>         "@maven//:com_lihaoyi_os_lib_3",
>         "@maven//:com_lihaoyi_ujson_3",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f73fdf30bf295578"></a>


### File `schema2json/build.sbt`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7e436d9f619502ae0feae74f8109f97b73e17c0e`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema2json/build.sbt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-aa8fb8da4f1d1b7072.** [cpg/schema2json/build.sbt, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema2json/build.sbt#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a0dfabf15e901bc406818fa155e188a9f46bf75e4cfeaaba13f90cb05514f667`.

> name := "schema2json"
> 
> dependsOn(Projects.schema)
> 
> libraryDependencies ++= Seq(
>   "com.lihaoyi"           %% "ujson"         % Versions.ujson,
>   "org.scalatest"         %% "scalatest"     % Versions.scalatest % Test
> )
> 
> scalacOptions -= "-Xfatal-warnings" // some antl-generated sources prompt compiler warnings :(
> 
> compile / javacOptions ++= Seq("-Xlint:all", "-Xlint:-cast", "-g")
> Test / fork := true
> testOptions += Tests.Argument(TestFrameworks.JUnit, "-a", "-v")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:schema2json/src/main/scala` -> module `schema2json::src::main::scala`


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


<a id="file-9f4c5ba1537baff5"></a>


### File `schema2json/src/main/scala/Schema2Json.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `97f4293ebafa9c34e9b31ddaad4b06c0a0ed7019`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema2json/src/main/scala/Schema2Json.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d5da48a65ddf19191d.** [cpg/schema2json/src/main/scala/Schema2Json.scala, lines 4–18](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema2json/src/main/scala/Schema2Json.scala#L4-L18). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `c1de15147a0cd742815450fdf7a0f1c3c47d90ed9e542d2c51d347c300fa76eb`.

> 
> object Schema2Json {
> 
>   val schema = CpgSchema.instance
> 
>   def main(args: Array[String]): Unit = {
>     val json = ujson.Obj(
>       "schemas"    -> schemaSummary,
>       "nodes"      -> nodeTypesAsJson,
>       "edges"      -> edgeTypesAsJson,
>       "properties" -> propertiesAsJson
>     )
> 
>     val outFileName = "/tmp/schema.json"
>     os.write(os.Path(outFileName), ujson.write(json))


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 31 lexically identified declarations, including `Schema2Json` (L5), `schema` (L7), `main` (L9), `json` (L10), `outFileName` (L17), `schemaName` (L23), `schemaName` (L26), `schemaIndex` (L29), `schemaIndex` (L32), `isSchemaHidden` (L37), `schemaSummary` (L40), `name` (L47), `description` (L48), `providedByFrontend` (L49), `nodeTypesAsJson` (L55), `baseTypeNames` (L60), `allPropertyNames` (L61), `cardinalities` (L62), `schName` (L64), `inheritedProperties` (L66), `inheritedPropertyNames` (L73), `nonInheritedProperties` (L74), `containedNodes` (L76), `json` (L83). The full declaration list is recorded under `cpg:schema2json/src/main/scala/Schema2Json.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
