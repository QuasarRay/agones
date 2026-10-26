<a id="m0001"></a>


# M0001 — Source mirror, version and shared contracts


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0001-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** The shared contract and pinned source inventory.


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-4805ffe6a2e496f31d.** [agones/site/content/en/docs/Overview/_index.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Overview/_index.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0bc9c38c2a79249df54dff3a0e11ba34db407b100cd04e7ab7d2797721f46b8a`.

> 
> _Agones, is derived from the Greek word agōn which roughly translates to “contest”, “competition at games” and “gathering”
> ([source](https://www.merriam-webster.com/dictionary/agones))._
> 
> ## What is Agones?
> 
> Agones is an open source platform, for deploying, hosting, scaling, and orchestrating dedicated game servers for
> large scale multiplayer games, built on top of the industry standard, distributed system platform [Kubernetes](https://kubernetes.io).
> 
> Agones replaces bespoke or proprietary cluster management and game server scaling solutions with an open source solution that
> can be utilised and communally developed - so that you can focus on the important aspects of building a multiplayer game,
> rather than developing the infrastructure to support it. 


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Establish the literal agones-rs source mirror without moving or deleting the original Go reference or existing crates. Put Cargo/module registration in rust-support/agones and use explicit path attributes for source files, including dotted filenames.

2. Carry pkg version defaults, feature-gate declarations, build/environment precedence and shared API-group constants from their exact source definitions. Connect them to explicit CGP configuration capabilities; derive-deftly owns repeated declarations and projections.

3. Record current crates as reuse candidates, with source-file and production-call-path ownership. Do not count a Rust file or previous passing test as complete parity. Keep server state and configuration owners unique.


## Architecture, readability and authored outputs


1. Own 13 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:.` -> module `<crate root>`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-1f2e9bbc9583192f53.** [agones/doc.go, lines 17–18](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/doc.go#L17-L18). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `229d717f33b79eaeb525284403fbbe77e3885c505e4d82fe36539bbf915f6739`.

> // Package agones is a library for hosting, running and scaling dedicated game servers on Kubernetes.
> package agones // import "agones.dev/agones"


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-b447e2f69d4058ed"></a>


### File `.dockerignore`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6425692cdcc343e7875dd51e72ba6d726671cc97`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.dockerignore`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2a0fc6eaf8be060f1e.** [agones/.dockerignore, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.dockerignore#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0260e96bb8c3493a2fd601dbe14c311177e8816888e1f28005102cab426934ec`.

> .*
> /build
> /cmd
> /sdks/cpp/.build
> /sdks/cpp/.archives
> .idea
> *.zip
> /release
> bin
> /docs
> *.md
> *.amd64
> /site/public
> /test


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-11013c389531dea0"></a>


### File `.gcloudignore`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5b7d71875ab296af45b65c2ff8da127d4b62ccdd`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.gcloudignore`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-770576e2649ff60b4d.** [agones/.gcloudignore, lines 16–29](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.gcloudignore#L16-L29). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fb60eb6eaafc6a96f11314955c7adec949e9c230f6db8a7f296bbf8d5a78decc`.

> !.git
> .cargo
> .cargo-targets
> .config
> .gcloudignore
> .gitignore
> .gocache
> .gomod
> .terraform
> bin
> cpp*tgz
> rust*tgz
> target
> tmp


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-15fc590cc52ccdfc"></a>


### File `.gitattributes`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `17ca80dab7d3fdfaecb954c867543a07b9004358`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.gitattributes`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ed15fb1ca6afbcbbe3.** [agones/.gitattributes, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.gitattributes#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b304d428389464393146d710abcc414301b2d1ce42257bb92515a74ba574e47c`.

> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> 
> # See https://github.com/golang/go/issues/9281


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3e9ceab35cfd70c3"></a>


### File `.gitignore`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c41f44426266513edc2b868f5af0f260ddaf2a66`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.gitignore`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-35240fe62c11248c2c.** [agones/.gitignore, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.gitignore#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5844af2b0a94dcc1dcd01da2c7ce973618b85b46632a1ad94093fdf21fbebdac`.

> 
> .*
> !/.yamllint
> !.gitignore
> !.helmignore
> !.gitattributes
> !.dockerignore
> !.golangci.yml
> !.gcloudignore
> !.eslintrc.*
> !.nycrc
> !.clang-format
> !.github
> *.iml
> bin


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9742c31b42d64496"></a>


### File `.golangci.yml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `70f52c49bd881eac7ceca23d793e35738ed6f7e1`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.golangci.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e240e19171408d2625.** [agones/.golangci.yml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.golangci.yml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7f287008b6b8e59f78ce17c9d8b5944d5324ec12d886941daa78101e7773bbad`.

> ---
> # Configuration for golangci-lint. See https://golangci-lint.run/docs/configuration/file/
> # for all available options and their defaults.
> version: "2"
> # options for analysis running
> run:
>   # analyze dependencies from the vendor/ directory rather than downloading them
>   modules-download-mode: vendor
> 
>   # exit code when at least one issue was found, default is 1
>   issues-exit-code: 1
> 
>   # include test files or not, default is true
>   tests: true
> # output configuration options


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4b229c9dafbcb761"></a>


### File `.yamllint`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2a378735be45893f9fb6d04742de428019ccbc0e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.yamllint`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e9c762440a88cfbd50.** [agones/.yamllint, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.yamllint#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c7d7dbfc231e8aeb5a4f57a5eb41210744c0a3ba2f74f08de8b48a5bf7f890df`.

> ---
> extends: default
> 
> rules:
>   truthy: disable
>   line-length:
>     max: 1500
>     level: error
>   trailing-spaces:
>     level: error
>   comments-indentation:
>     level: error
>   comments:
>     level: error
>   braces:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d2af80f29ae084e1"></a>


### File `cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `43266340256faa1e496bb9f07262d033498b0678`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-02ffdd0fc40fba8282.** [agones/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4e96524a76e676fac09067afc2887101c3e90349a22c58bba82a5dde55a4cf43`.

> ---
> # Copyright Contributors to Agones a Series of LF Projects, LLC.
> #
> # Licensed under the Apache License, Version 2.0 (the "License");
> # you may not use this file except in compliance with the License.
> # You may obtain a copy of the License at
> #
> #     http://www.apache.org/licenses/LICENSE-2.0
> #
> # Unless required by applicable law or agreed to in writing, software
> # distributed under the License is distributed on an "AS IS" BASIS,
> # WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> # See the License for the specific language governing permissions and
> # limitations under the License.
> #


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0750614801ebb57f"></a>


### File `doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c484855dcec6d49a015e83d2368f7fb741daa42f`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-1caf1878370ab197b4.** [agones/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `229d717f33b79eaeb525284403fbbe77e3885c505e4d82fe36539bbf915f6739`.

> /*
>  * Copyright Contributors to Agones a Series of LF Projects, LLC.
>  *
>  * Licensed under the Apache License, Version 2.0 (the "License");
>  * you may not use this file except in compliance with the License.
>  * You may obtain a copy of the License at
>  *
>  *     http://www.apache.org/licenses/LICENSE-2.0
>  *
>  * Unless required by applicable law or agreed to in writing, software
>  * distributed under the License is distributed on an "AS IS" BASIS,
>  * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
>  * See the License for the specific language governing permissions and
>  * limitations under the License.
>  */


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-440e7ef9f3b71cc2"></a>


### File `go.mod`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `961c851d1a7a08fe5d513ff2cc8390a38fa03f0e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/go.mod`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ed1fd336315db39a9d.** [agones/go.mod, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/go.mod#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `01df333891a331341e77aab737c24dc722fab26abc8439b9c2036c71a477215a`.

> module agones.dev/agones
> 
> go 1.26.5
> 
> // Hotfix to address websocket connection issues with github.com/grpc-ecosystem/grpc-gateway/v2 v2.26.2 and above.
> // See: https://github.com/grpc-ecosystem/grpc-gateway/issues/5326, https://github.com/agones-dev/agones/issues/4248
> replace github.com/tmc/grpc-websocket-proxy v0.0.0-20220101234140-673ab2c3ae75 => github.com/swermin/grpc-websocket-proxy v0.1.0
> 
> require (
> 	cloud.google.com/go/cloudbuild v1.25.0
> 	cloud.google.com/go/compute/metadata v0.9.0
> 	contrib.go.opencensus.io/exporter/prometheus v0.1.0
> 	contrib.go.opencensus.io/exporter/stackdriver v0.8.0
> 	fortio.org/fortio v1.3.1
> 	github.com/ahmetb/gen-crd-api-reference-docs v0.3.0


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-586ae5eb6f85c36b"></a>


### File `go.sum`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b4e647fc9baa68d85a935cc6f3a0d036c4b8c7c6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/go.sum`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c90f1b4d6d52080db7.** [agones/go.sum, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/go.sum#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d890978bc12d9c9958b2a87f99a87d8478b007208e44c5d859e973ba89adcbf7`.

> cloud.google.com/go v0.23.0/go.mod h1:aQUYkXzVsufM+DwF1aE+0xfcU+56JwCaLick0ClmMTw=
> cloud.google.com/go v0.26.0/go.mod h1:aQUYkXzVsufM+DwF1aE+0xfcU+56JwCaLick0ClmMTw=
> cloud.google.com/go v0.34.0/go.mod h1:aQUYkXzVsufM+DwF1aE+0xfcU+56JwCaLick0ClmMTw=
> cloud.google.com/go v0.38.0/go.mod h1:990N+gfupTy94rShfmMCWGDn0LpTmnzTp2qbd1dvSRU=
> cloud.google.com/go v0.44.1/go.mod h1:iSa0KzasP4Uvy3f1mN/7PiObzGgflwredwwASm/v6AU=
> cloud.google.com/go v0.44.2/go.mod h1:60680Gw3Yr4ikxnPRS/oxxkBccT6SA1yMk63TGekxKY=
> cloud.google.com/go v0.44.3/go.mod h1:60680Gw3Yr4ikxnPRS/oxxkBccT6SA1yMk63TGekxKY=
> cloud.google.com/go v0.45.1/go.mod h1:RpBamKRgapWJb87xiFSdk4g1CME7QZg3uwTez+TSTjc=
> cloud.google.com/go v0.46.3/go.mod h1:a6bKKbmY7er1mI7TEI4lsAkts/mkhTSZK8w33B4RAg0=
> cloud.google.com/go v0.50.0/go.mod h1:r9sluTvynVuxRIOHXQEHMFffphuXHOMZMycpNR5e6To=
> cloud.google.com/go v0.52.0/go.mod h1:pXajvRH/6o3+F9jDHZWQ5PbGhn+o8w9qiu/CffaVdO4=
> cloud.google.com/go v0.53.0/go.mod h1:fp/UouUEsRkN6ryDKNW/Upv/JBKnv6WDthjR6+vze6M=
> cloud.google.com/go v0.54.0/go.mod h1:1rq2OEkV3YMf6n/9ZvGWI3GWw0VoqH/1x2nd8Is/bPc=
> cloud.google.com/go v0.56.0/go.mod h1:jr7tqZxxKOVYizybht9+26Z/gUq7tiRzu+ACVAMbKVk=
> cloud.google.com/go v0.57.0/go.mod h1:oXiQ6Rzq3RAkkY7N6t3TcE6jE+CIBBbA36lwQ1JyzZs=


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-69c758bfe3789c05"></a>


### File `tools.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `58453f115b9c70607738182386b0216f1381bfd1`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/tools.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-981c878b0580bb65a8.** [agones/tools.go, lines 19–33](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/tools.go#L19-L33). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c916e834507e2e31ca61016345f25d89980191b0f27ce3846e616de705e9f600`.

> 
> /*
> 	github.com/ahmetb/gen-crd-api-reference-docs                        site/gen-api-docs.sh
> 	github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway   build-sdk-images/go/gen.sh
> 	github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2      build-sdk-images/go/gen.sh
> 	golang.org/x/tools/cmd/goimports                                    build-image/Dockerfile
> 	google.golang.org/protobuf/cmd/protoc-gen-go                        build-sdk-images/go/Dockerfile
> */
> 
> import (
> 	_ "github.com/ahmetb/gen-crd-api-reference-docs"
> 	_ "github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway"
> 	_ "github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2"
> 	_ "golang.org/x/tools/cmd/goimports"
> 	_ "google.golang.org/protobuf/cmd/protoc-gen-go"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg` -> module `pkg`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-a499c55f3064a095b4.** [agones/pkg/doc.go, lines 15–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/doc.go#L15-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `69d341f39551d06ea8d60501b9ff57aeb1303abfe331df7c22cf650acf0f9901`.

> // Package pkg is all Agones libraries.
> package pkg


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-a80935204191be9c"></a>


### File `pkg/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4eba8a4995d61f218bc1b3bff4da945ab9fb9cbb`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-aaf6565f45ddef8b6e.** [agones/pkg/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `69d341f39551d06ea8d60501b9ff57aeb1303abfe331df7c22cf650acf0f9901`.

> // Copyright Contributors to Agones a Series of LF Projects, LLC.
> //
> // Licensed under the Apache License, Version 2.0 (the "License");
> // you may not use this file except in compliance with the License.
> // You may obtain a copy of the License at
> //
> //     http://www.apache.org/licenses/LICENSE-2.0
> //
> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> // Package pkg is all Agones libraries.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-232b7b211c41e54a"></a>


### File `pkg/version.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4d9e7bc4d0d3271e06c80734f77690bdd8f22002`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/version.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-51e52d896afb2fdbf7.** [agones/pkg/version.go, lines 16–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/version.go#L16-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `18a321dd868bdf897742490dde06040a35be996dbef93f4aae7ebd855c85842d`.

> 
> var (
> 	// Version is the global version for all binaries
> 	// This is set at compile time by the build process
> 	Version = "dev"
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Explicit new support/output ownership

These are planned Rust-only support or evidence files, not original-source mirror renames. Reuse an existing equivalent through a recorded path mapping; do not create a competing implementation merely to match this suggested support filename.

### New support/output `rust-support/agones/Cargo.toml`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-4805ffe6a2e496f31d.** [agones/site/content/en/docs/Overview/_index.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Overview/_index.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0bc9c38c2a79249df54dff3a0e11ba34db407b100cd04e7ab7d2797721f46b8a`.

> 
> _Agones, is derived from the Greek word agōn which roughly translates to “contest”, “competition at games” and “gathering”
> ([source](https://www.merriam-webster.com/dictionary/agones))._
> 
> ## What is Agones?
> 
> Agones is an open source platform, for deploying, hosting, scaling, and orchestrating dedicated game servers for
> large scale multiplayer games, built on top of the industry standard, distributed system platform [Kubernetes](https://kubernetes.io).
> 
> Agones replaces bespoke or proprietary cluster management and game server scaling solutions with an open source solution that
> can be utilised and communally developed - so that you can focus on the important aspects of building a multiplayer game,
> rather than developing the infrastructure to support it. 

**Derived file obligation.** Cargo package and pinned dependency ownership outside the source mirror. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/agones/src/lib.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-4805ffe6a2e496f31d.** [agones/site/content/en/docs/Overview/_index.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Overview/_index.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0bc9c38c2a79249df54dff3a0e11ba34db407b100cd04e7ab7d2797721f46b8a`.

> 
> _Agones, is derived from the Greek word agōn which roughly translates to “contest”, “competition at games” and “gathering”
> ([source](https://www.merriam-webster.com/dictionary/agones))._
> 
> ## What is Agones?
> 
> Agones is an open source platform, for deploying, hosting, scaling, and orchestrating dedicated game servers for
> large scale multiplayer games, built on top of the industry standard, distributed system platform [Kubernetes](https://kubernetes.io).
> 
> Agones replaces bespoke or proprietary cluster management and game server scaling solutions with an open source solution that
> can be utilised and communally developed - so that you can focus on the important aspects of building a multiplayer game,
> rather than developing the infrastructure to support it. 

**Derived file obligation.** Explicit source-module registration and public contract wiring. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/agones/src/lifecycle.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-4805ffe6a2e496f31d.** [agones/site/content/en/docs/Overview/_index.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Overview/_index.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0bc9c38c2a79249df54dff3a0e11ba34db407b100cd04e7ab7d2797721f46b8a`.

> 
> _Agones, is derived from the Greek word agōn which roughly translates to “contest”, “competition at games” and “gathering”
> ([source](https://www.merriam-webster.com/dictionary/agones))._
> 
> ## What is Agones?
> 
> Agones is an open source platform, for deploying, hosting, scaling, and orchestrating dedicated game servers for
> large scale multiplayer games, built on top of the industry standard, distributed system platform [Kubernetes](https://kubernetes.io).
> 
> Agones replaces bespoke or proprietary cluster management and game server scaling solutions with an open source solution that
> can be utilised and communally developed - so that you can focus on the important aspects of building a multiplayer game,
> rather than developing the infrastructure to support it. 

**Derived file obligation.** CGP startup, cancellation, supervision, drain and cleanup capabilities. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.

### New support/output `rust-support/agones/src/declarations.rs`

**Direct quotation — UPSTREAM_DOCUMENTATION; Q-4805ffe6a2e496f31d.** [agones/site/content/en/docs/Overview/_index.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Overview/_index.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0bc9c38c2a79249df54dff3a0e11ba34db407b100cd04e7ab7d2797721f46b8a`.

> 
> _Agones, is derived from the Greek word agōn which roughly translates to “contest”, “competition at games” and “gathering”
> ([source](https://www.merriam-webster.com/dictionary/agones))._
> 
> ## What is Agones?
> 
> Agones is an open source platform, for deploying, hosting, scaling, and orchestrating dedicated game servers for
> large scale multiplayer games, built on top of the industry standard, distributed system platform [Kubernetes](https://kubernetes.io).
> 
> Agones replaces bespoke or proprietary cluster management and game server scaling solutions with an open source solution that
> can be utilised and communally developed - so that you can focus on the important aspects of building a multiplayer game,
> rather than developing the infrastructure to support it. 

**Derived file obligation.** derive-deftly ownership and repeated wiring declarations. Record the concrete input, output, error and lifecycle contract, then implement all branches. CGP owns runtime policy and resources; derive-deftly owns repeated structural implementation. Genco receives explicit semantic inputs. Explain the file as short Python-familiar transformations in valid Rust. Keep one implementation owner, preserve original source identity, and defer execution/assurance to the whole-project gate.



## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
