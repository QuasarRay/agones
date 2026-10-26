<a id="m0002"></a>


# M0002 — Pinned CPG source, build machinery and Rust support boundaries


**Route:** `CPG`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0002-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** The shared contract and pinned source inventory.


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Preserve the complete pinned CPG tree and source-relative naming. Account for Bazel helpers, sbt definitions, project helpers, launcher and protobuf/domain generation inputs; do not exclude build-only handwritten behavior.

2. Establish Rust-only Cargo/module glue outside the mirror and reuse the existing cpg-rs Grafeo-backed graph/persistence support. Record source/configuration identities and separate pristine upstream from guidance/patch overlays.

3. Author concrete Rust equivalents for required source build/tool behavior. Inspect available generator receipts without running executables. Missing runnable emission tools are explicit blockers under the strict build prohibition.


## Architecture, readability and authored outputs


1. Own 49 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-c2192832ff053b7d"></a>


### File `.bazelrc`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5b7bd5de9977bd878856b6f5914757aca632eabe`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:.bazelrc`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5a6fd5bc4126dc5ca8.** [cpg/.bazelrc, lines 1–4](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/.bazelrc#L1-L4). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3ec7099aba9da5cad13fe9fb255b5d5f09da95bd094d9823a070fd05aa0b06d9`.

> common --tool_java_language_version=21
> common --tool_java_runtime_version=remotejdk_21
> common --java_runtime_version=remotejdk_21
> try-import user.bazelrc


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-09e3091121799b2f"></a>


### File `.bazelversion`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `47da986f86fdaed33e15cdc6431a1abf742e8398`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:.bazelversion`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-cbc294cc661b3f739a.** [cpg/.bazelversion, lines 1–1](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/.bazelversion#L1-L1). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `077ee5b3a3a7fc2622ceff6466fea0c28f3fb08b24653dcc7f4baab29b9aef36`.

> 9.1.0


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-45f8741c3eb877be"></a>


### File `.gitattributes`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `56204512e1d95307e28f992e8da281cbb887fbfa`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:.gitattributes`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-042dc57ec45d867c6b.** [cpg/.gitattributes, lines 1–1](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/.gitattributes#L1-L1). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `f3e10a699c2f80ab395aede3bdca2c7ece8baf157b3fc7384e1db8bbc1805de4`.

> resources/testcode/** filter=lfs diff=lfs merge=lfs -text


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-003511c394dc584f"></a>


### File `.gitignore`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `d4fb1f945ff9880e2d4599626d2b9f6b31cc5b9f`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:.gitignore`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c7a14ae91abd2aebae.** [cpg/.gitignore, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/.gitignore#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `9e091a95ae7edc12c30803b4aef85e1f51f24d230d4f7c98cf3f80f4280a4f52`.

> /build/*
> .idea/
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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-87478e00e6b8f809"></a>


### File `.gitmodules`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e69de29bb2d1d6434b8b29ae775ad8c2e48c5391`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:.gitmodules`.


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


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f99756eef59b13f8"></a>


### File `.installation_root`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e69de29bb2d1d6434b8b29ae775ad8c2e48c5391`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:.installation_root`.


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


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d6b09440c741f12e"></a>


### File `.scalafmt.conf`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `24691b3dbfdd5bb8fd106622b30ba1fc7341e4fd`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:.scalafmt.conf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-34422395676a7e1420.** [cpg/.scalafmt.conf, lines 1–6](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/.scalafmt.conf#L1-L6). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7638763ddf518ed7ad58fe359e717675dbe8c2f503f28f0d576ab0e82fbc6133`.

> version = 3.9.6
> runner.dialect = scala3
> preset = IntelliJ
> maxColumn = 120
> align.preset = true
> project.excludeFilters = [".*\\.sbt$", ".*\\.sc$", "project/.*\\.scala"]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b2e1f540529cee8e"></a>


### File `BUILD`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b811fe5c90c81ff526d78f14637ab874e8ac48d3`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:BUILD`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-644089fc64d402cd4b.** [cpg/BUILD, lines 1–11](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/BUILD#L1-L11). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d331502f744330fa6c5f1e1dff23186ef98595e29b77b46796ab51482004c64a`.

> exports_files([".scalafmt.conf"])
> 
> alias(
>     name = "format",
>     actual = "//bazel/format",
> )
> 
> alias(
>     name = "formatCheck",
>     actual = "//bazel/format:format.check",
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2a736e66a33a707d"></a>


### File `LICENSE`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `0bfde9ab43afb67241273d83c7f4c6953485d60a`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:LICENSE`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fee6c168182a673c63.** [cpg/LICENSE, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/LICENSE#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `10907e9c71b78937d48915c5d78cb3bf18e7a37f9fe5c38eac7388ebf13a2491`.

> 
>                                  Apache License
>                            Version 2.0, January 2004
>                         http://www.apache.org/licenses/
> 
>    TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION
> 
>    1. Definitions.
> 
>       "License" shall mean the terms and conditions for use, reproduction,
>       and distribution as defined by Sections 1 through 9 of this document.
> 
>       "Licensor" shall mean the copyright owner or entity authorized by
>       the copyright owner that is granting the License.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1defa512c4e0cd31"></a>


### File `MODULE.bazel`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `39339e7a9d2812c556ce9643c659c45c667ebfb5`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:MODULE.bazel`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d24575693cfc8562b6.** [cpg/MODULE.bazel, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/MODULE.bazel#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6349b2a361cada1ac1a8e720d5f529f342ee1de0977f90f16c58dbae7ae433a9`.

> module(name = "codepropertygraph")
> 
> bazel_dep(name = "protobuf", version = "34.1")
> bazel_dep(name = "bazel_tooling")
> bazel_dep(name = "rules_jvm_external", version = "7.0")
> bazel_dep(name = "aspect_rules_lint", version = "2.5.2")
> bazel_dep(name = "buildifier_prebuilt", version = "8.5.1.2")
> 
> BAZEL_TOOLING_VERSION = "86bbf2be26dc7a2517a56c90e12fd9a6ed20dd25"
> 
> git_override(
>     module_name = "bazel_tooling",
>     commit = "{}".format(BAZEL_TOOLING_VERSION),
>     remote = "git@github.com:joernio/bazel_tooling.git",
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f863879cf09f82ce"></a>


### File `MODULE.bazel.lock`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `a9fff93ca84030fae76913d0bd700e7b0f854028`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:MODULE.bazel.lock`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f7d9eabbc38d8bca6c.** [cpg/MODULE.bazel.lock, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/MODULE.bazel.lock#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e8e53205ae6f5c0e2c20511a158e405245566edef8f420bfe1c2e90809de5f12`.

> {
>   "lockFileVersion": 26,
>   "registryFileHashes": {
>     "https://bcr.bazel.build/bazel_registry.json": "8a28e4aff06ee60aed2a8c281907fb8bcbf3b753c91fb5a5c57da3215d5b3497",
>     "https://bcr.bazel.build/modules/abseil-cpp/20210324.2/MODULE.bazel": "7cd0312e064fde87c8d1cd79ba06c876bd23630c83466e9500321be55c96ace2",
>     "https://bcr.bazel.build/modules/abseil-cpp/20211102.0/MODULE.bazel": "70390338f7a5106231d20620712f7cccb659cd0e9d073d1991c038eb9fc57589",
>     "https://bcr.bazel.build/modules/abseil-cpp/20230125.1/MODULE.bazel": "89047429cb0207707b2dface14ba7f8df85273d484c2572755be4bab7ce9c3a0",
>     "https://bcr.bazel.build/modules/abseil-cpp/20230802.0.bcr.1/MODULE.bazel": "1c8cec495288dccd14fdae6e3f95f772c1c91857047a098fad772034264cc8cb",
>     "https://bcr.bazel.build/modules/abseil-cpp/20230802.0/MODULE.bazel": "d253ae36a8bd9ee3c5955384096ccb6baf16a1b1e93e858370da0a3b94f77c16",
>     "https://bcr.bazel.build/modules/abseil-cpp/20230802.1/MODULE.bazel": "fa92e2eb41a04df73cdabeec37107316f7e5272650f81d6cc096418fe647b915",
>     "https://bcr.bazel.build/modules/abseil-cpp/20240116.1/MODULE.bazel": "37bcdb4440fbb61df6a1c296ae01b327f19e9bb521f9b8e26ec854b6f97309ed",
>     "https://bcr.bazel.build/modules/abseil-cpp/20240116.2/MODULE.bazel": "73939767a4686cd9a520d16af5ab440071ed75cec1a876bf2fcfaf1f71987a16",
>     "https://bcr.bazel.build/modules/abseil-cpp/20250127.0/MODULE.bazel": "d1086e248cda6576862b4b3fe9ad76a214e08c189af5b42557a6e1888812c5d5",
>     "https://bcr.bazel.build/modules/abseil-cpp/20250127.1/MODULE.bazel": "c4a89e7ceb9bf1e25cf84a9f830ff6b817b72874088bf5141b314726e46a57c1",
>     "https://bcr.bazel.build/modules/abseil-cpp/20250512.1/MODULE.bazel": "d209fdb6f36ffaf61c509fcc81b19e81b411a999a934a032e10cd009a0226215",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-dd15167c00051b7f"></a>


### File `README.md`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `67cf2a36f54378d8e57a8dc2299ddc63004c442e`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-445eff2bec7f856712.** [cpg/README.md, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> [![CI pipeline](https://github.com/ShiftLeftSecurity/codepropertygraph/actions/workflows/release.yml/badge.svg)](https://github.com/ShiftLeftSecurity/codepropertygraph/actions/workflows/release.yml)
> [![Maven Central](https://maven-badges.herokuapp.com/maven-central/io.shiftleft/codepropertygraph_3/badge.svg)](https://maven-badges.herokuapp.com/maven-central/io.shiftleft/codepropertygraph_3)
> 
> # Code Property Graph - Specification and Tooling
> 
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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e7fbc49616ad2777"></a>


### File `build.sbt`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `732d1c48d1ea2d9a0b6a24b3f5b12100ce85f610`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/build.sbt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0f3dad85d955d9b5b2.** [cpg/build.sbt, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/build.sbt#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d49e3c43e754f0654869585324f26046db57cc38a4d6084fb2b07d0e20f3914e`.

> name := "codepropertygraph"
> 
> // parsed by project/Versions.scala, updated by updateDependencies.sh
> val flatgraphVersion = "0.1.36"
> 
> inThisBuild(
>   List(
>     organization       := "io.shiftleft",
>     scalaVersion       := "3.9.0",
>     resolvers ++= Seq(Resolver.mavenLocal, "Sonatype OSS" at "https://oss.sonatype.org/content/repositories/public"),
>     packageDoc / publishArtifact := true,
>     packageSrc / publishArtifact := true,
>     scmInfo := Some(
>       ScmInfo(
>         url("https://github.com/ShiftLeftSecurity/codepropertygraph"),


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f7fd23a75318d37e"></a>


### File `format_maven_install.json`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `062ae59f4ac4a65a506dcbda339ba796daa69cb5`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:format_maven_install.json`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bab80a86da7be2a298.** [cpg/format_maven_install.json, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/format_maven_install.json#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `5eeefbd9be48a625af3b1c8b6abe9eb7dc39d48d2176c537dc36042a4ef23339`.

> {
>   "__AUTOGENERATED_FILE_DO_NOT_MODIFY_THIS_FILE_MANUALLY": "THERE_IS_NO_DATA_ONLY_ZUUL",
>   "__INPUT_ARTIFACTS_HASH": {
>     "org.scalameta:scalafmt-cli_2.13": -1077369318,
>     "repositories": -1949687017
>   },
>   "__RESOLVED_ARTIFACTS_HASH": {
>     "com.facebook:nailgun-server": -1312000310,
>     "com.github.scopt:scopt_2.13": -1791766764,
>     "com.lihaoyi:fansi_2.13": -1754008317,
>     "com.lihaoyi:sourcecode_2.13": 2142439176,
>     "com.typesafe:config": 1422037420,
>     "io.get-coursier:interface": -1431594519,
>     "io.github.java-diff-utils:java-diff-utils": -2129961455,
>     "net.java.dev.jna:jna": -152040143,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-688fbfb0b478771a"></a>


### File `maven_install.json`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `75343a6d3a1e3f426310cdd6b5be6cc2d465859c`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:maven_install.json`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e673e5330d5defa5d9.** [cpg/maven_install.json, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/maven_install.json#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `91a48f70ff111d9e8bad372efe6412b7ede70c8d6456926c7fa0c04547699804`.

> {
>   "__AUTOGENERATED_FILE_DO_NOT_MODIFY_THIS_FILE_MANUALLY": "THERE_IS_NO_DATA_ONLY_ZUUL",
>   "__INPUT_ARTIFACTS_HASH": {
>     "com.google.code.findbugs:jsr305": 495355163,
>     "com.google.code.gson:gson": 597770368,
>     "com.google.errorprone:error_prone_annotations": -1035138750,
>     "com.google.guava:guava": 1943808618,
>     "com.google.j2objc:j2objc-annotations": 2003271689,
>     "com.google.protobuf:protobuf-java": 1237026331,
>     "com.lihaoyi:os-lib_3": 899915638,
>     "com.lihaoyi:ujson_3": 1182647324,
>     "io.joern:flatgraph-core_3": -747314055,
>     "io.joern:flatgraph-domain-classes-generator_3": 593398397,
>     "io.joern:flatgraph-help_3": -1945683429,
>     "io.joern:flatgraph-odb-convert_3": 152328455,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-74411b08f6fdf0a9"></a>


### File `private-key.pem.enc`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `fa949ebced13a6e664d2470bd301108fbf50ae11`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:private-key.pem.enc`.


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


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-aa5604bbe226081a"></a>


### File `updateDependencies.sh`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `67a873b93fff2a6fb5a3481734a4041871253c2b`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:updateDependencies.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6e13657d9d34932f10.** [cpg/updateDependencies.sh, lines 2–16](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/updateDependencies.sh#L2-L16). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `97440df9a38072a0763a9b44274a71fd3c008c5800cb24e8e70e8df345363c7c`.

> NON_INTERACTIVE_OPTION=$1
> DEPENDENCY=$2
> 
> # Check if Bash version is 4 or higher
> if (( BASH_VERSINFO[0] < 4 )); then
>   echo "Error: Bash version 4 or higher is required." >&2
>   echo "If you are on a mac, you can switch to a modern bash e.g. with 'brew install bash' and ensure it's in your PATH." >&2
>   # n.b. if it's not in your PATH, you can do so e.g. by defining "export PATH="/opt/homebrew/bin:$PATH" in your ~/.zshrc
>   exit 1
> fi
> 
> check_installed() {
>   if ! type "$1" > /dev/null; then
>     echo "Please ensure you have $1 installed."
>     exit 1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:.github/workflows` -> module `.github::workflows`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-e6cf0cdb4fbe5768"></a>


### File `.github/workflows/pr.yml`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `d473739a11aa2412ce05a76627e694ec1a777b04`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:.github/workflows/pr.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ac077f7a7decc908df.** [cpg/.github/workflows/pr.yml, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/.github/workflows/pr.yml#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `62c40fea699b71a31b9ef71be3a7f9b7e4a8534770a09f89dd45a0c03aaee4da`.

> name: PR build
> 
> on:
>   pull_request:
>     branches: [ master ]
> 
> jobs:
>   test-codepropertygraph:
>     runs-on: ubuntu-latest
> 
>     steps:
>     - uses: actions/checkout@v4
>       with:
>         lfs: true
>     - name: Clone submodules


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c537e8a04784268a"></a>


### File `.github/workflows/release.yml`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c0dd93d5c39ae16ab375cc8e3b6bb0c1ebb615e3`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:.github/workflows/release.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8077b0c0c1acd749e5.** [cpg/.github/workflows/release.yml, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/.github/workflows/release.yml#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `edba9a11be5e6a909cd2d9eadd7c8638dec4519b322f1e7dac27b83dca23968f`.

> name: master & release build
> 
> on:
>   push:
>     branches: [ master ]
>   workflow_dispatch:    # allow to manually trigger this workflow
> 
> jobs:
>   test-codepropertygraph:
>     runs-on: ubuntu-latest
>     steps:
>     - uses: actions/checkout@v4
>       with:
>         lfs: true
>         fetch-depth: 0


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:bazel` -> module `bazel`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-317a7d2276056115"></a>


### File `bazel/BUILD`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e69de29bb2d1d6434b8b29ae775ad8c2e48c5391`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:bazel/BUILD`.


**Direct quotation — UPSTREAM_RESOURCE_CONSUMER_EXCERPT; Q-9ccdcb6b0d0271cb4e.** [cpg/MODULE.bazel.lock, lines 467–480](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/MODULE.bazel.lock#L467-L480). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e8e53205ae6f5c0e2c20511a158e405245566edef8f420bfe1c2e90809de5f12`.

>           "apksig": {
>             "repoRuleId": "@@bazel_tools//tools/build_defs/repo:http.bzl%http_archive",
>             "attributes": {
>               "url": "https://android.googlesource.com/platform/tools/apksig/+archive/24e3075e68ebe17c0b529bb24bfda819db5e2f3b.tar.gz",
>               "build_file": "@@rules_android+//bzlmod_extensions:apksig.BUILD"
>             }
>           }
>         }
>       }
>     },
>     "@@rules_android+//bzlmod_extensions:com_android_dex.bzl%com_android_dex_extension": {
>       "general": {
>         "bzlTransitiveDigest": "toD4gOZwGNfBEM+xec6eRi3BM2Z9EZ4oQrORZQH3sNY=",
>         "usagesDigest": "c1Y/KGGjUYCyd8zNIVTUh1bynVXRFz6xGKaSCBpQANM=",


**Reference boundary.** Binary/empty resource has no prose to quote; quote the upstream loader/test contract and retain exact blob/hash. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5c487d3241ae4b1b"></a>


### File `bazel/java.bzl`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `ab4ecf0b429f2d6e5609beba636b17261849a9f7`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/bazel/java.bzl`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b1a19d4808e91bafb7.** [cpg/bazel/java.bzl, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/bazel/java.bzl#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e85315666436e96c1a21d293615cf3fb3bbd8f1b7dba20d3c012cbd5ea6619f0`.

> load("@bazel_tooling//:java_rule_factory.bzl", "make_java_rules")
> 
> _rules = make_java_rules(
>     java_common_opts = [
>         "-g",
>         "--release",
>         "17",
>         "-Xlint:all",
>         "-Xlint:-cast",
>         "-XepDisableAllChecks",
>     ],
>     common_java_binary_runtime_deps = [],
>     common_java_test_runtime_deps = [],
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-077cd082f6086ac0"></a>


### File `bazel/scala.bzl`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `5b8a4e99e0ec4b1cf299a7d974f3fae1aedf9353`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/bazel/scala.bzl`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-10e6bbefad4d813d4f.** [cpg/bazel/scala.bzl, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/bazel/scala.bzl#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `a39ce37d0ec68e2fa26a5f9a53646a959bafdac8ff1f724286e7af89a8a0dce2`.

> load("@bazel_tooling//:scala_rule_factory.bzl", "make_scala_rules")
> 
> _rules = make_scala_rules(
>     scala_version = "3.9.0",
>     common_scalacopts = [
>         "-java-output-version",
>         "17",
>         "-no-indent",
>         "-old-syntax",
>         "-Wshadow:type-parameter-shadow",
>         "-deprecation",
>         "-Wconf:msg=Implicit parameters should be provided with a `using` clause:s",
>     ],
>     # Applies to the Java sources of mixed Java/Scala targets which rules_scala
>     # compiles in a separate Javac action that does not see the javacopts from


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:bazel/format` -> module `bazel::format`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-1747914e083eefca"></a>


### File `bazel/format/BUILD`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `63b02cf1429256a4094bf188f3c397dc959a6ec0`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:bazel/format/BUILD`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e8e4bf3b3586b31c1e.** [cpg/bazel/format/BUILD, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/bazel/format/BUILD#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `3b590afde4eff4996aabe75df782873cc170b44bcfcf3bbc0f3ef4dc1dcdd6a4`.

> load("@aspect_rules_lint//format:defs.bzl", "format_multirun")
> load("//bazel:java.bzl", "java_binary")
> 
> java_binary(
>     name = "scalafmt",
>     data = ["//:.scalafmt.conf"],
>     main_class = "org.scalafmt.cli.Cli",
>     runtime_deps = ["@format_maven//:org_scalameta_scalafmt_cli_2_13"],
> )
> 
> format_multirun(
>     name = "format",
>     scala = ":scalafmt",
>     starlark = "@buildifier_prebuilt//:buildifier",
>     starlark_check_args = ["-mode=diff"],


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:codepropertygraph` -> module `codepropertygraph`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-59a9f7ec369a0ea7"></a>


### File `codepropertygraph/BUILD`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `88a5064cc8950f10e121e9566823e2535e240478`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:codepropertygraph/BUILD`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6ff28b9ca2f838898a.** [cpg/codepropertygraph/BUILD, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/BUILD#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `16da69bdbfbafa74b53d77f915f25b53b6e1a3874dab72e9cc25a07a47cc0a94`.

> load("//bazel:scala.bzl", "scala_library", "scala_test")
> 
> scala_library(
>     name = "codepropertygraph",
>     srcs = glob([
>         "src/main/scala/**/*.java",
>         "src/main/scala/**/*.scala",
>     ]),
>     visibility = ["//visibility:public"],
>     deps = [
>         "//schema:domainClassesNeverLinked",
>         "//schema:java_proto",
>         "@maven//:com_google_protobuf_protobuf_java",
>         "@maven//:io_joern_flatgraph_core_3",
>         "@maven//:io_joern_flatgraph_help_3",


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-45a50e275d82a633"></a>


### File `codepropertygraph/build.sbt`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `f335e113831b23c7974f1370745c35e3478f5f3a`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/codepropertygraph/build.sbt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-abc5130ad3d0e2eecf.** [cpg/codepropertygraph/build.sbt, lines 1–13](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/codepropertygraph/build.sbt#L1-L13). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `56d098b9e21a5362c692086639b27bcfa04182a3c20b098a5030b021a2009c99`.

> name := "codepropertygraph"
> 
> dependsOn(Projects.protoBindings, Projects.domainClasses)
> 
> libraryDependencies ++= Seq(
>   "io.joern"              %% "flatgraph-formats"     % Versions.flatgraph,
>   "io.joern"              %% "flatgraph-help"        % Versions.flatgraph,
>   "io.joern"              %% "flatgraph-odb-convert" % Versions.flatgraph,
>   "com.github.scopt"      %% "scopt"                % "4.0.1",
>   "com.github.pathikrit"  %% "better-files"         % "3.9.2",
>   "org.slf4j"              % "slf4j-api"            % "2.0.6",
>   "org.scalatest"         %% "scalatest"            % Versions.scalatest % Test
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:img` -> module `img`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-3a5ff433a69aebce"></a>


### File `img/call-site.jpg`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `80752aae5a66db161b92b20c04869f7c0d39845b`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:img/call-site.jpg`.


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


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-63202821580349a7"></a>


### File `img/call-site.odg`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `d4150663b004b518e273151554f83d3acccdd313`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:img/call-site.odg`.


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


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d48c04f4e4fa755a"></a>


### File `img/program-structure.jpg`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `10232fc551ff484f2c4903186be714a12a211361`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:img/program-structure.jpg`.


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


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ee3f2fc870219cac"></a>


### File `img/program-structure.odg`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `840ce066e311b4549bbd43f50bea2b73742bfe1e`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:img/program-structure.odg`.


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


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5550de2e35c0484b"></a>


### File `img/type-declaration.jpg`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `c34e52a1cea5accb2949e2a4268a0de5cc8c2f61`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:img/type-declaration.jpg`.


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


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-016f3c01937952f6"></a>


### File `img/type-declaration.odg`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `331b9d580dca65a4da08fd782c7629c38f10162c`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:img/type-declaration.odg`.


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


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:project` -> module `project`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-446d49224fc4371c"></a>


### File `project/FileUtils.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `8d8e5879f7ec1a5af0114e60e4e42c9d2a327fd0`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/project/FileUtils.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f072e4116234d97ad7.** [cpg/project/FileUtils.scala, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/project/FileUtils.scala#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `082131c77b6771cca6c1661275e21bc4272b73d5f97869fa3025312445fa187d`.

> 
> object FileUtils {
> 
>   def listFilesRecursively(roots: File*): Seq[File] = {
>     roots.flatMap { root =>
>       Files.walk(root.toPath).iterator.asScala.map(_.toFile).filter(!_.isDirectory)
>     }
>   }
> 
>   def deleteRecursively(root: File): Unit = {
>     if (root.exists) {
>       Files.walk(root.toPath).iterator.asScala.map(_.toFile).collect {
>         case file if (file.isDirectory) => deleteRecursively(file)
>         case file                       => file.delete()
>       }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 6 lexically identified declarations, including `FileUtils` (L6), `listFilesRecursively` (L8), `deleteRecursively` (L14), `md5` (L23), `md` (L24), `dis` (L27). The full declaration list is recorded under `cpg:project/FileUtils.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f1cd21d68f4c8d9c"></a>


### File `project/Projects.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `53d46b647a32bebfe80718129f64b30dcd61183b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/project/Projects.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-22e2951ecfd7f524c4.** [cpg/project/Projects.scala, lines 2–9](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/project/Projects.scala#L2-L9). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `b661b8be7db0cfe87b8f23f65f1b2fb8d0848e62801d04ad73c40936d8cf10a3`.

> 
> object Projects {
>   lazy val codepropertygraph = project.in(file("codepropertygraph"))
>   lazy val schema            = project.in(file("schema"))
>   lazy val domainClasses     = project.in(file("domainClasses"))
>   lazy val protoBindings     = project.in(file("proto-bindings"))
>   lazy val schema2json       = project.in(file("schema2json"))
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 6 lexically identified declarations, including `Projects` (L3), `codepropertygraph` (L4), `schema` (L5), `domainClasses` (L6), `protoBindings` (L7), `schema2json` (L8). The full declaration list is recorded under `cpg:project/Projects.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-14acca7cf94fe49b"></a>


### File `project/Utils.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `059dccc14c25be974147f5e13e9cef93a28048f3`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/project/Utils.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ff5b2ec79e31785537.** [cpg/project/Utils.scala, lines 1–7](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/project/Utils.scala#L1-L7). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `7a71074e1c14695ef1f46061d929465bae9dc842e0483eb42aee130c418d8b21`.

> object GitLFSUtils {
>   def isGitLFSEnabled(): Boolean = {
>     import scala.sys.process._
>     val exitCode = "git lfs ls-files" ! ProcessLogger(s => ())
>     exitCode == 0
>   }
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 3 lexically identified declarations, including `GitLFSUtils` (L1), `isGitLFSEnabled` (L2), `exitCode` (L4). The full declaration list is recorded under `cpg:project/Utils.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f749774116216d0c"></a>


### File `project/Versions.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `53d9f2affafc85439de050caffe6b8053e293a25`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/project/Versions.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1b7c3b1a9c61710a38.** [cpg/project/Versions.scala, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/project/Versions.scala#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `916cb600759bfc824c99c566d18394adfb6f201621575890ef264cef7941ce49`.

> /* reads version declarations from /build.sbt so that we can declare them in one place */
> object Versions {
>   val flatgraph = parseVersion("flatgraphVersion")
>   val scalatest = "3.2.10"
>   val ujson     = "4.4.3"
> 
>   private def parseVersion(key: String): String = {
>     val versionRegexp = s""".*val $key[ ]+=[ ]?"(.*?)"""".r
>     val versions: List[String] = scala.io.Source
>       .fromFile("build.sbt")
>       .getLines
>       .filter(_.contains(s"val $key"))
>       .collect { case versionRegexp(version) => version }
>       .toList
>     assert(


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 7 lexically identified declarations, including `Versions` (L2), `flatgraph` (L3), `scalatest` (L4), `ujson` (L5), `parseVersion` (L7), `versionRegexp` (L8), `versions` (L9). The full declaration list is recorded under `cpg:project/Versions.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f4d2007d90725f7f"></a>


### File `project/build.properties`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `b49295c71a981fa966b66b1470357065037ea7e4`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:project/build.properties`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-86aea656aedfa3d8e5.** [cpg/project/build.properties, lines 1–1](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/project/build.properties#L1-L1). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `e72872dd14ba73acb3ffb113fb4bbf10ea4b150f31297437a0b76454ca75bcf6`.

> sbt.version=1.12.5


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fa1de58111470599"></a>


### File `project/plugins.sbt`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `495815910b524d9e504a81e5dfeaca4c2331ab68`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/project/plugins.sbt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fbaa5412e21b1ad325.** [cpg/project/plugins.sbt, lines 1–6](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/project/plugins.sbt#L1-L6). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `85db9e5c3baa5b4f89a2bbacc7006121e9bee6c27880d5424d3276ea857aed00`.

> addSbtPlugin("com.github.sbt"    % "sbt-protobuf"         % "0.8.2")
> addSbtPlugin("org.scalameta"     % "sbt-scalafmt"         % "2.4.6")
> addSbtPlugin("io.shiftleft"      % "sbt-ci-release-early" % "2.1.2")
> addSbtPlugin("com.github.sbt"    % "sbt-dynver"           % "5.1.0")
> addSbtPlugin("com.github.sbt"    % "sbt-native-packager"  % "1.10.4")
> addSbtPlugin("io.joern"          % "sbt-flatgraph"        % Versions.flatgraph)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:project/project` -> module `project::project`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ad8a058c10eec11d"></a>


### File `project/project/Versions.scala`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7c7ddb3d2e4ee3e0e6383d65b57acbdd23f49df6`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/project/project/Versions.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bdaaaa33d5b65ab792.** [cpg/project/project/Versions.scala, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/project/project/Versions.scala#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `edcd3da5305a13edbf4ce682003c5b5f62245e84ce410fe50c0027f3da5296c3`.

> /* reads version declarations from /build.sbt so that we can declare them in one place */
> object Versions {
>   val flatgraph = parseVersion("flatgraphVersion")
> 
>   private def parseVersion(key: String): String = {
>     val versionRegexp = s""".*val $key[ ]+=[ ]?"(.*?)"""".r
>     val versions: List[String] = scala.io.Source
>       .fromFile("build.sbt")
>       .getLines
>       .filter(_.contains(s"val $key"))
>       .collect { case versionRegexp(version) => version }
>       .toList
>     assert(
>       versions.size == 1,
>       s"""unable to extract $key from build.sbt, expected exactly one line like `val $key= "0.0.0-SNAPSHOT"`."""


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 5 lexically identified declarations, including `Versions` (L2), `flatgraph` (L3), `parseVersion` (L5), `versionRegexp` (L6), `versions` (L7). The full declaration list is recorded under `cpg:project/project/Versions.scala` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:resources` -> module `resources`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-9f29c89261bf8bd0"></a>


### File `resources/log4j2-test.xml`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `92b517598eb1d467cb8be82eacfbcd6ef420a896`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:resources/log4j2-test.xml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-588fac018ea7faeadf.** [cpg/resources/log4j2-test.xml, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/resources/log4j2-test.xml#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `6d56d5acfc914cdc5596b75b094ade7bbbfe64edcb474962a06a54c89f659ca2`.

> <?xml version="1.0" encoding="UTF-8"?>
> <Configuration status="warn" strict="true">
>     <Appenders>
>         <Console name="Console" target="SYSTEM_OUT">
>             <PatternLayout pattern="[%-5p] %m%n" />
>         </Console>
>         <File name="FileAppender" fileName="${sys:user.dir}/logs/cpg.log">
>             <PatternLayout pattern="[%-5p] %m%n" />
>         </File>
>     </Appenders>
>     <Loggers>
>         <Logger name="org.reflections8.Reflections" level="error"/>
>         <Logger name="org.xnio.nio" level="error"/>
>         <Logger name="overflowdb" level="error"/>
>         <Logger name="io.shiftleft.codepropertygraph.cpgloading" level="error"/>


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:samples/customcpg` -> module `samples::customcpg`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ef5f1b22e8d2d592"></a>


### File `samples/customcpg/README.md`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `77bfec7e767aef8c15b56c921d05010e4cbb01e6`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:samples/customcpg/README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-8c284eb72215ff33e7.** [cpg/samples/customcpg/README.md, lines 1–1](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/samples/customcpg/README.md#L1-L1). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `06aa2f53fbe2a2bba3ef707e98070da98c4b8e435940f57095beb198ba58e7dc`.

> # Example of how to add node and edge types to the CPG


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-dacd15feeb574cfd"></a>


### File `samples/customcpg/local-m2-publish.sh`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e4c68ce7e34ffa57806dacfa2c1cf47788ed1e2f`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:samples/customcpg/local-m2-publish.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6055baa8eef19c3d72.** [cpg/samples/customcpg/local-m2-publish.sh, lines 5–19](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/samples/customcpg/local-m2-publish.sh#L5-L19). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `21faeac266c4f37060a253c6f77b99f867df372ad719659c6b6886c6584dedde`.

> 
> CODEPROPERTYGRAPH_ROOT=../../
> 
> # stop on errors
> set -e
> set -o pipefail
> 
> BASE_DIR=`pwd`
> CSHARP_FLAG=$1
> 
> # checkout the latest released (tagged) version of codepropertygraph (public)
> rm -rf build
> mkdir build
> rsync -av --exclude 'samples' $CODEPROPERTYGRAPH_ROOT build/codepropertygraph
> cd build/codepropertygraph


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:samples/customcpg/schemas` -> module `samples::customcpg::schemas`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-d6ba1c2cd665adcc"></a>


### File `samples/customcpg/schemas/myschema.json`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `eebc6de2c364e09efdd51489daaa26239800c169`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:samples/customcpg/schemas/myschema.json`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9724636750d9f5a26c.** [cpg/samples/customcpg/schemas/myschema.json, lines 1–9](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/samples/customcpg/schemas/myschema.json#L1-L9). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `555f48a5a1245b649c18030877bdf43ac86c444cf63d2b8db4028ad974301a19`.

> {
>     "nodeTypes" : [
>         {"id" : 35, "name" : "MY_CUSTOM_NODE_TYPE",
>          "keys": ["CODE"],
>          "comment" : "This is my custom node type",
>          "outEdges": []
>         }
>     ]
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:samples/pass` -> module `samples::pass`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-e2c7d98556f1c264"></a>


### File `samples/pass/README.md`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `63ea8abe49dc099176061a29793d1d2d9952f3b2`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:samples/pass/README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-fc7f712c7f53d32486.** [cpg/samples/pass/README.md, lines 1–2](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/samples/pass/README.md#L1-L2). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `1d860f5d3b0a131f979b2108a5b844e43278948f4eed2ebe1821de9af0da34bd`.

> # A sample CPG pass for Ocular/Joern


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-52d2f6ec60e9e5f5"></a>


### File `samples/pass/build.sbt`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `1734880f8cec3fc5d3d500c57c6a9a104fea8612`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:samples/pass/build.sbt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-414e855e12e8fadcac.** [cpg/samples/pass/build.sbt, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/samples/pass/build.sbt#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `64892a83c1925d18c82d9b0b7a270b51e0d84fc6383a763045bbd13acc63673b`.

> name := "samplepass"
> organization := "io.shiftleft"
> 
> val cpgVersion = "0.11.1"
> 
> libraryDependencies ++= Seq(
>   "io.shiftleft"  %% "codepropertygraph" % cpgVersion,
>   "io.shiftleft"  %% "enhancements"      % cpgVersion,
>   "org.scalatest" %% "scalatest"         % Versions.scalatest % Test
> )
> 
> ThisBuild / resolvers ++= Seq(
>   Resolver.mavenLocal,
>   "Sonatype OSS" at "https://oss.sonatype.org/content/repositories/public",
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2e5ff9938700c417"></a>


### File `samples/pass/samplepass.sh`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `72a503cdec3abf0c0de8013ce0139542de8119bb`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:samples/pass/samplepass.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e12e6d89dcedf38057.** [cpg/samples/pass/samplepass.sh, lines 1–1](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/samples/pass/samplepass.sh#L1-L1). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `116d2b51c486ec4f6ec7011d31aa491e4082af98ac83140672e3293e1c811e07`.

> ./target/universal/stage/bin/samplepass


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-83518f2773974540"></a>


### File `samples/pass/src_cpg.bin.zip`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `58daf00e937521567403793fcf7d49e391274715`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:samples/pass/src_cpg.bin.zip`.


**Direct quotation — UPSTREAM_RESOURCE_CONSUMER_EXCERPT; Q-14f88b5278acd9688b.** [cpg/samples/pass/src/main/scala/io/shiftleft/passes/mypass/SamplePass.scala, lines 13–24](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/samples/pass/src/main/scala/io/shiftleft/passes/mypass/SamplePass.scala#L13-L24). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `665808a4161e8a4149e78b49fb9a365e7a4f67c59841083b3b162c61fc2e4522`.

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
>   pass.createApplySerializeAndStore(serializedCpg)
> }


**Reference boundary.** Binary/empty resource has no prose to quote; quote the upstream loader/test contract and retain exact blob/hash. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:samples/pass/project` -> module `samples::pass::project`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-73ec22a3d826db4a"></a>


### File `samples/pass/project/plugins.sbt`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `9dbfce148ecf83b1035a6161263c325eb8b566ba`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:samples/pass/project/plugins.sbt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6bec9d3afec348ee65.** [cpg/samples/pass/project/plugins.sbt, lines 1–1](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/samples/pass/project/plugins.sbt#L1-L1). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `d83a16e5dbc04dc58942017f08616b9b7d84eb90dfc65e254162b401d913e039`.

> addSbtPlugin("com.typesafe.sbt"  % "sbt-native-packager"   % "1.3.5")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `cpg:schema` -> module `schema`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-b85d8a5e097cc440"></a>


### File `schema/BUILD`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `7ca917ae8ff3478c2969434aac813f4904c3dbc2`. **Disposition:** `RETAINED_REFERENCE`. **Destination:** `RETAIN_SOURCE_REFERENCE:schema/BUILD`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a7ce09feae2e3e98bf.** [cpg/schema/BUILD, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/BUILD#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `dc73e6a5eb0def31449846c80146696965f002c305240415b8e460a798407a5f`.

> load("@protobuf//bazel:java_proto_library.bzl", "java_proto_library")
> load("@protobuf//bazel:proto_library.bzl", "proto_library")
> load("//bazel:java.bzl", "java_import")
> load("//bazel:scala.bzl", "scala_binary", "scala_library")
> 
> scala_library(
>     name = "schema",
>     srcs = glob(["src/main/scala/**/*.scala"]),
>     visibility = ["//visibility:public"],
>     deps = [
>         "@maven//:io_joern_flatgraph_domain_classes_generator_3",
>     ],
> )
> 
> scala_binary(


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Retain this exact source path as a reference/build/documentation input with its content identity and role. It is not credited as a translated production body. Preserve applicable attribution and notices.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-41325c3764c64e44"></a>


### File `schema/build.sbt`


**Source:** `cpg` at `301c4010d28ecdac05bc2ed44310f56446459465`; Git blob `e6221c6581f7e67453c8b91934b86774b58698fb`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `cpg-rs/schema/build.sbt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7ab417f8247b005ab2.** [cpg/schema/build.sbt, lines 1–15](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/schema/build.sbt#L1-L15). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `2b4e293e99233a4adf64538bac5a2480db47dea3be759fbce6dc5f29ed8adc4d`.

> name := "codepropertygraph-schema"
> 
> libraryDependencies += "io.joern" %% "flatgraph-domain-classes-generator" % Versions.flatgraph
> 
> lazy val generatedSrcDir = settingKey[File]("root for generated sources - we want to check those in")
> enablePlugins(FlatgraphCodegenSbtPlugin)
> generateDomainClasses/classWithSchema := "io.shiftleft.codepropertygraph.schema.CpgSchema$"
> generateDomainClasses/fieldName       := "instance"
> generateDomainClasses/outputDir       := (Projects.domainClasses / generatedSrcDir).value
> generateDomainClasses/scala3          := true
> generateDomainClassesCheck/scala3     := true
> 
> val generateProtobuf = taskKey[File]("generate protobuf definitions: cpg.proto")
> generateProtobuf := Def.taskDyn {
>   val outputRoot       = target.value / "protos"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

9. Preserve this original-named build/launcher file as the source/build contract. Implement its Rust-toolchain equivalent in rust-support/cpg/src/build_contracts.rs (or an explicitly mapped existing owner), accounting for each task, argument, generated input/output and error. Do not place Rust syntax in a .sbt/.bzl file, silently discard build semantics, or retain a JVM dependency for core native behavior.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Explicit new support/output ownership

These are planned Rust-only support or evidence files, not original-source mirror renames. Reuse an existing equivalent through a recorded path mapping; do not create a competing implementation merely to match this suggested support filename.

### New support/output `rust-support/cpg/Cargo.toml`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```

**Derived file obligation.** Pinned native CPG dependencies and external source-module paths. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/cpg/src/lib.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```

**Derived file obligation.** Exact-case Scala module registration without moving src/main/scala. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/cpg/src/build_contracts.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```

**Derived file obligation.** Native Rust equivalents of SBT/Bazel/launcher behavior with exact task/source mappings. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/cpg/src/declarations.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```

**Derived file obligation.** Native builder identity, typed metadata and derive-deftly wiring. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/cpg/src/lifecycle.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-2679aa3a96f8c3ceb4.** [cpg/README.md, lines 141–152](https://github.com/ShiftLeftSecurity/codepropertygraph/blob/301c4010d28ecdac05bc2ed44310f56446459465/README.md#L141-L152). Revision `301c4010d28ecdac05bc2ed44310f56446459465`; source SHA-256 `cc3385daade4ca02a306944b7055d3c8219ed8b1a7b25f895c816647523b3d6e`.

> # Scripts
> 
> ## Dump the CPG schema
> 
> You can dump the current Code Property Graph schema using the `schema2json.sh`
> bash script.
> 
> ```bash
> ./schema2json.sh
> Schema written to: /tmp/schema.json
> ```

**Derived file obligation.** CGP stage resources, failure, cancellation and cleanup ownership. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.



## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
