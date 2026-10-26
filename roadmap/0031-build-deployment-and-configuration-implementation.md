<a id="m0031"></a>


# M0031 — Build, deployment and configuration implementation


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0031-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** The shared contract and pinned source inventory.


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Preserve every required deployment, packaging, build and configuration input and its literal path. Translate behavior where the Rust toolchain requires a concrete equivalent; keep original names as compatibility inputs.

2. Separate source Kubernetes deployment references from permitted Rusternetes integration. Retain values, ports, resources, certificates, feature gates, service contracts and user-visible configuration semantics without adding another orchestrator.

3. Author CI and packaging changes without executing them or pushing events that trigger project verification. Do not weaken protections or claim skipped checks are passing.


## Architecture, readability and authored outputs


1. Own 219 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:.github` -> module `.github`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-62589e534562afc5"></a>


### File `.github/SECURITY.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `29a47ca152c9963b870c3b06d6efe8268c4099f3`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/SECURITY.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-262c7ce80035c4b1e2.** [agones/.github/SECURITY.md, lines 3–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/SECURITY.md#L3-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5857df69ee69a68441440fb07524eac58bb947a77967c77cffc1dec02e82460f`.

> Agones runs on Kubernetes and extends the API with custom resources and controllers. The security of a deployment depends on correct cluster configuration (for example RBAC, network policies, TLS for the allocator service, and node isolation). Operational guidance appears in the [Agones documentation](https://agones.dev/site/docs/), including [best practices](https://agones.dev/site/docs/guides/best-practices/) and topics such as [service accounts](https://agones.dev/site/docs/advanced/service-accounts/) and the [allocator service](https://agones.dev/site/docs/advanced/allocator-service/).
> 
> ## Security contacts
> Security reports are triaged by the [Agones maintainers](https://github.com/agones-dev/agones/blob/main/docs/governance/community_membership.md#current-members). Coordinated fixes and public advisories are published through [GitHub Security Advisories](https://github.com/agones-dev/agones/security/advisories) when appropriate.
> 
> ## Reporting a vulnerability
> Please **do not report security vulnerabilities through public GitHub issues, discussions, or pull requests**.
> 
> ### Preferred: GitHub
> If you believe you have found a security vulnerability in Agones, please report it privately through the following steps:
> 
> 1. Open a [private vulnerability report](https://github.com/agones-dev/agones/security/advisories/new) for this repository (also reachable from the **Security** tab).
> 2. Include a clear description, affected version(s) if known, steps to reproduce, and any suspected impact or mitigations.
> 3. If available, please also include a Proof-of-concept, logs, or other supporting information.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8fc08261577545b2"></a>


### File `.github/dependabot.yml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fc1a63e24c780f1090777ed09fd9556ae7d07430`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/dependabot.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-84ae0831d18361dccf.** [agones/.github/dependabot.yml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/dependabot.yml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7fbe43f501ef4189ec043c195f4a882e30a753bcd6f936081dbcf158e65fe2ff`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8ecb796a0b5e411f"></a>


### File `.github/pull_request_template.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `928d48fe7d03db92846a13a4ed2d3b7fa8a927b4`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/pull_request_template.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-681269a7c9ce045cee.** [agones/.github/pull_request_template.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/pull_request_template.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7eb8f7020a38f61efb57c72d184ef6c6e7053800ff23fb0064a6ba9bab8a7094`.

> 
> 1. If this is your first time, please read our contributor guidelines: https://github.com/agones-dev/agones/blob/main/CONTRIBUTING.md and developer guide https://github.com/agones-dev/agones/blob/main/build/README.md
> 2. Please label this pull request according to what type of issue you are addressing.
> 3. Ensure you have added or ran the appropriate tests for your PR: https://github.com/agones-dev/agones/blob/main/build/README.md#testing-and-building
> -->
> 
> **What type of PR is this?**
> > Uncomment only one ` /kind <>` line, press enter to put that in a new line, and remove leading whitespace from that line:
> >
> > /kind breaking
> > /kind bug
> > /kind cleanup
> > /kind documentation
> > /kind feature
> > /kind hotfix


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-efdffa83bd06ff58"></a>


### File `.github/release.yml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `52cbc9657336ff2047b078e0567ed500f8a507fa`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/release.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a611b6901a24345129.** [agones/.github/release.yml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/release.yml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1c112011da8c8675a50040dde64aa5a8c1d80fbb19f7fed49bc4b09426a1e4dd`.

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
> changelog:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:.github/ISSUE_TEMPLATE` -> module `.github::ISSUE_TEMPLATE`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-4bcaf3a4dcd82127"></a>


### File `.github/ISSUE_TEMPLATE/become-a-repo-collaborator.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `761c47b84bef0a147c310f277487cc36fe7ba7ae`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/ISSUE_TEMPLATE/become-a-repo-collaborator.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-a73df2e9a3ae3834e7.** [agones/.github/ISSUE_TEMPLATE/become-a-repo-collaborator.md, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/ISSUE_TEMPLATE/become-a-repo-collaborator.md#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `611a3d3bcdbd9d3abc3946382e93a7f448f39ca1ffc932dea59f8f2a359cbb98`.

> ---
> name: Become a repo collaborator
> about: Request collaborator status for the Agones repo
> title: Collaborator Request
> labels: area/community
> assignees: thisisnotapril
> 
> ---
> 
> If you'd like to become a collaborator of the Agones repo on GitHub, please fill out this form and submit it.
> Give us a few days to review and you should receive an invitation to join. Once accepted you'll be able to have issues assigned to you! 
> 
> ### GitHub user id
> - List your GitHub user id


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4070aa8219d1c36d"></a>


### File `.github/ISSUE_TEMPLATE/bug_report.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a85baaf59936b91685e7bb49ab0f1554a7384494`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/ISSUE_TEMPLATE/bug_report.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-81cc8c4040df1ae523.** [agones/.github/ISSUE_TEMPLATE/bug_report.md, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/ISSUE_TEMPLATE/bug_report.md#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a368e9a748da8742910e3ee3ee30c21cc0a285d1987cbc180d07f3d4e2e4af5b`.

> ---
> name: Bug report
> about: Create a report to help us improve
> title: ''
> labels: kind/bug
> assignees: ''
> 
> ---
> 
> <!-- Please use this template while reporting a bug and provide as much info as possible. Not doing so may result in your bug not being addressed in a timely manner. Thanks!
> 
> If the matter is security related, please disclose it privately via 
> -->
> 


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-871ddee7eec65c16"></a>


### File `.github/ISSUE_TEMPLATE/feature_request.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d4e25a3143a00e8aedb9bb552f2557d3edfd9c9d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/ISSUE_TEMPLATE/feature_request.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-35f8bacd6a11cfa355.** [agones/.github/ISSUE_TEMPLATE/feature_request.md, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/ISSUE_TEMPLATE/feature_request.md#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `390438bb1c79d704e529dacaa52e73b992afd98a34e209f3138016e1073f30ff`.

> ---
> name: Feature Request
> about: Suggest a new feature or enhancement for the Agones project.
> title: '[Feature Proposal] - <short, descriptive title>'
> labels: kind/feature, kind/operations
> assignees: ''
> 
> ---
> 
> **Is your feature request related to a problem? Please describe.**
> A clear and concise description of what the problem is. Ex. I'm always frustrated when [...]
> 
> **Describe the solution you'd like**
> A clear and concise description of what you want to happen.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c1fa0f3111e2a1dc"></a>


### File `.github/ISSUE_TEMPLATE/golang_version_upgrade.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `58659240acccfe11849113ff0111e3611b61deb1`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/ISSUE_TEMPLATE/golang_version_upgrade.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-b456e390dbfbef3fe3.** [agones/.github/ISSUE_TEMPLATE/golang_version_upgrade.md, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/ISSUE_TEMPLATE/golang_version_upgrade.md#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e4f816ed92ca0ccbb8213e2f5afdd34e2c40bae1fb2d5f15d1e2aa2ba264afb4`.

> ---
> name: Golang Version and Dependency Upgrades
> about: Steps to upgrade Golang version and dependencies
> title: "Golang MAJOR.MINOR.PATCH Version and Dependency Upgrades"
> labels: kind/cleanup
> assignees: ""
> ---
> 
> Steps to determine the Golang version to upgrade to:
> 
> - [ ] Go to the Kubernetes repository and find the release branch for the most recent version of
>       Kuberntes supported by Agones. Find the `go.mod` file in that branch, and look for the go
>       version. This is the minor version that Agones should use. For instance, if the most recent
>       Kubernetes version supported by Agones is 1.32, then the minor version of Golang on Agones
>       should be 1.23 as is determined by the `go.mod` file in the [release branch of Kubernetes


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-16722fa79544a263"></a>


### File `.github/ISSUE_TEMPLATE/kubernetes_update.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `636b0fe047f5516449e46e20d3e8d0ca8c44108f`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/ISSUE_TEMPLATE/kubernetes_update.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-030758c843803da354.** [agones/.github/ISSUE_TEMPLATE/kubernetes_update.md, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/ISSUE_TEMPLATE/kubernetes_update.md#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2aae5509394eea0102667debf86bc3c79eb3a63160530c42ce71f160002bd965`.

> ---
> name: Upgrade Supported Kubernetes Versions
> about: Issue for updating the supported Kubernetes versions.
> title: 'Update Supported Kubernetes to {version_1} {version_2} {version_3}'
> labels: kind/operations, kind/breaking
> assignees: ''
> 
> ---
> 
> Agones supports and is tested against 3 releases of Kubernetes, targeting the newest version as
> being the [default version in the GKE Rapid channel](https://cloud.google.com/kubernetes-engine/docs/release-notes#current_versions).
> The vendored version of client-go will be aligned with the middle of the three supported Kubernetes
> versions ({version_2}). All the example clusters will use the middle of the three supported
> Kubernetes versions ({version_2}).


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:.github/workflows` -> module `.github::workflows`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-70bb294a73eb7575"></a>


### File `.github/workflows/close.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8f295c8eb4d9f5c956692d500c40a7919b714b1d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/workflows/close.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-be0965bce20b3af2eb.** [agones/.github/workflows/close.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/workflows/close.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `eb892a933928cd10b483a1d310ce3a378d9549434d4ca78ab63845adcccb7ba4`.

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

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-562fe5c1143bfc85"></a>


### File `.github/workflows/fossa.yml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e29f049a2d98975113a0bff582e9d04d891c2d59`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/workflows/fossa.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c7abea30c8d7ace1e4.** [agones/.github/workflows/fossa.yml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/workflows/fossa.yml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `91547c5696daf78c65e1916d92e7560a9d272445dbf847cc598b3703b3e04bd3`.

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

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f67d7507ffbf6a2e"></a>


### File `.github/workflows/label-pr.yml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4c3e5a51e23410c60d8ccb53369d12373d898b5b`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/workflows/label-pr.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3a3798a5cb601e6ecb.** [agones/.github/workflows/label-pr.yml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/workflows/label-pr.yml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c2482d702f48c9707bf74dfd47d9b065b4c016b26c3d716d3ea83e4deaa2be47`.

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


<a id="file-ac6d24eb30253a09"></a>


### File `.github/workflows/labeler.yml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5a7a580bf98d2848f80c3bfaf30fcdc865244f0e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/workflows/labeler.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8abe15f99ccf0794dc.** [agones/.github/workflows/labeler.yml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/workflows/labeler.yml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `542fde968231ad1bcc6b1eada3f8707a0f7a25685951131904bda7454a30166f`.

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

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5ba3d1d91ec20df2"></a>


### File `.github/workflows/obsolete.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ddf8c2222508c2ed72ad5c22bfb4bb49bc9fe8a4`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/workflows/obsolete.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a415411247eb849e6b.** [agones/.github/workflows/obsolete.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/workflows/obsolete.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bbde1641a9ddb0c9e80b7357113da0bc34ee75f291b37d0b672ef0f78d32042c`.

> ---
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
> # This workflow applies 'obsolete' label to issues that have had no activity for 60 days.
> #


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-cd21c811db32e39c"></a>


### File `.github/workflows/pr_update.yml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a303708c8cd9f577bb2a5270f4b5064907bf7204`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/workflows/pr_update.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-895b8adf35eb9bb923.** [agones/.github/workflows/pr_update.yml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/workflows/pr_update.yml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7be24a408b6032b58b1d96f3332949578c8822f5efcd456a1d5eb7508b2c9417`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-25d5dcc23bf505e8"></a>


### File `.github/workflows/rust.yml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0345b3fca3dea292c91645fa3b6a44248c4f7d59`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/workflows/rust.yml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0470bb1b86cbe7cb36.** [agones/.github/workflows/rust.yml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/workflows/rust.yml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `86afb37d7c14d27705fc92b7369da25205245cbc0a2948236bd8951982bbe630`.

> name: Rust reimplementation
> 
> on:
>   push:
>     branches:
>       - agones-rs-dev
>     paths:
>       - "Cargo.toml"
>       - "Cargo.lock"
>       - "crates/**"
>       - ".github/workflows/rust.yml"
>   pull_request:
>     paths:
>       - "Cargo.toml"
>       - "Cargo.lock"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-901d1feba88d2396"></a>


### File `.github/workflows/stale.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `56d5a02506af33522d10781b88ce15d616d3b28e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/workflows/stale.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-002aaeba6ad0c45bda.** [agones/.github/workflows/stale.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/workflows/stale.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5122827897c2136e500ead0b776109bcb402dcb298bc0a045e94dcb82ca45904`.

> ---
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
> # This workflow applies stale label to issues that have had no activity for atleast 30 days.
> #


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-47f99bbb5d963b20"></a>


### File `.github/workflows/update_dependencies.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0da4da3030ed4aafd8e8b3005e65d2f82ffaaca9`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/.github/workflows/update_dependencies.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a26ef59ab4f0979274.** [agones/.github/workflows/update_dependencies.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/.github/workflows/update_dependencies.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `73eae6f6484f7dea8c4f105f3f2d6542d0d340daeac4c474df5f41316b52d007`.

> ---
> name: Update Dependencies
> on:
>   schedule:
>     - cron: 0 0 * * 1
> permissions: {}
> jobs:
>   create_issue:
>     name: Create update dependency issue once every six weeks
>     runs-on: ubuntu-latest
>     permissions:
>       contents: read
>       issues: write
>     steps:
>       - name: Run this workflow only every six weeks (fail this step the other five weeks)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build` -> module `build`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-a55d61f22a4ce4e7"></a>


### File `build/Makefile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0d44901b97c42feae2b0270bb8ed21a62150fe31`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/Makefile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-31e3c842ea5ca28d80.** [agones/build/Makefile, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/Makefile#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c2f2914aa177347650170f98d761fe77a6a4ada356bb2634b56b1f840fdf3907`.

> base_version = 1.62.0
> 
> #
> # All of the following can be overwritten with environment variables
> # or passed through directly when invoking the relevent Make targets
> #
> 
> # Version defaults to the short hash of the latest commit
> VERSION ?= $(base_version)-dev-$(shell git rev-parse --short=7 HEAD)
> 
> # kubectl configuration to use
> KUBECONFIG ?= ~/.kube/config
> # The (gcloud) test cluster that is being worked against
> GCP_CLUSTER_NAME ?= test-cluster
> GCP_CLUSTER_ZONE ?= us-west1-c


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0352bc646de56f92"></a>


### File `build/README.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d3d37b4d737d7793327059ad22e4ca8d16535c1a`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-034bf88823360f2aff.** [agones/build/README.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/README.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d49829af9ec9e38eaefae1c26489cee0240ec3bd99fa7e4c0686ae0f2802d1a5`.

> 
> Tooling for building and developing against Agones, with only dependencies being
> [Make](https://www.gnu.org/software/make/) and [Docker](https://www.docker.com)
> 
> Rather than installing all the dependencies locally, you can test and build Agones using the Docker image that is
> built from the Dockerfile in this directory. There is an accompanying Makefile for all the common
> tasks you may wish to accomplish.
> 
> ## Quick Start
> 
> 1. **Platform Setup**: Set up your development environment → [Platform Setup Guide](docs/platform-setup.md)
> 2. **Build & Test**: Learn basic build workflows → [Building and Testing Guide](docs/building-testing.md)
> 3. **Cluster Setup**: Set up a development cluster → [Cluster Setup Guide](docs/cluster-setup.md)
> 
> ## Documentation


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a12db5c68e463a59"></a>


### File `build/boilerplate.go.txt`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `67105532cde53e52e381db3550a9024c7ed88328`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/boilerplate.go.txt`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-54f21923003b409191.** [agones/build/boilerplate.go.txt, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/boilerplate.go.txt#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3084d4971d3f03c5ff804bf45f8cbe315af5171ce5541055976b0a7f567bbd5b`.

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
> // This code was autogenerated. Do not edit directly.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ddfad9493a15dcf3"></a>


### File `build/boilerplate.yaml.txt`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `39f12e79fb59b67477ca2a24930e83d5b0209927`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/boilerplate.yaml.txt`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c2576e391d89ce14b9.** [agones/build/boilerplate.yaml.txt, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/boilerplate.yaml.txt#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a482c8fad85ae7ec75e6054f5103cac55576118a8723f6bc49833f063ece16cf`.

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
> # This code was autogenerated. Do not edit directly.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7e4713373246817e"></a>


### File `build/build-required-src-dist.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `049dda7c668f9ec468f9c0825b9823f003f93826`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-required-src-dist.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-27437921f1a9b13a09.** [agones/build/build-required-src-dist.sh, lines 15–29](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-required-src-dist.sh#L15-L29). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `463a385b223f940b30f7ada0077bbbfae0b041078e34c27f7dda351bf98d826c`.

> 
> set -o errexit
> set -o nounset
> set -o pipefail
> 
> SRC_ROOT=$(dirname "${BASH_SOURCE}")/..
> 
> TMP_DEPS_SRC=/tmp/dependencies-src.tgz
> 
> # Pack the source code of dependencies that are MPL
> tar -zcf ${TMP_DEPS_SRC} -C ${SRC_ROOT}/vendor/ \
>   github.com/hashicorp/golang-lru \
>   github.com/hashicorp/hcl
> 
> for ddir in ${SRC_ROOT}/cmd/controller/bin/ ${SRC_ROOT}/cmd/extensions/bin/ ${SRC_ROOT}/cmd/ping/bin/ ${SRC_ROOT}/cmd/sdk-server/bin/ ${SRC_ROOT}/cmd/allocator/bin/ ${SRC_ROOT}/cmd/processor/bin/  ; do


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-97b5ab0cfc9dac06"></a>


### File `build/e2e_upgrade_test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a9670ae3bc7b79b4ad3ffe2466681acbbaba1482`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/e2e_upgrade_test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-dc0df9186b00e63809.** [agones/build/e2e_upgrade_test.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/e2e_upgrade_test.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3e35c23d7dbe13c446da374fa2d5ebd3bace739f586d6306de8a7af006cbfb94`.

> 
> set -e
> set -o pipefail
> 
> BASE_VERSION=$1
> PROJECT_ID=$2
> BUCKET_NAME="upgrade-test-container-logs"
> 
> apt-get update && apt-get install -y jq
> export SHELL="/bin/bash"
> export KUBECONFIG="/root/.kube/config"
> mkdir -p /go/src/agones.dev/ /root/.kube/
> ln -s /workspace /go/src/agones.dev/agones
> cd /go/src/agones.dev/agones/test/upgrade


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b3abe6f232bd6789"></a>


### File `build/extract-licenses.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4261e728204a25dcc22cedbbf29d6740aaefe951`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/extract-licenses.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2d660d8c7d0ee6cfa5.** [agones/build/extract-licenses.sh, lines 15–29](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/extract-licenses.sh#L15-L29). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c9d5e39b64f0a261d61a575254e40cd5dbf8ff4fe2b84e4600ecb2ebb1ffea3a`.

> 
> set -o errexit
> set -o nounset
> set -o pipefail
> 
> append_license() {
>   lib=$1
>   path=$2
>   echo "================================================================================" >> ${TMP_LICENSES}
>   echo "= ${lib} licensed under: =" >> ${TMP_LICENSES}
>   echo >> ${TMP_LICENSES}
>   cat "$path" >> ${TMP_LICENSES}
>   echo >> ${TMP_LICENSES}
>   echo "= ${path} MD5 $(cat "${path}" | md5sum | awk '{print $1}')" >> ${TMP_LICENSES}
>   echo "================================================================================" >> ${TMP_LICENSES}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6f7bfc3421bfb4b5"></a>


### File `build/grafana-frontend.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `eb85ffe294398fec3cb57304c2cf02a2c01d2dd6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana-frontend.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b4361f2f32907eb661.** [agones/build/grafana-frontend.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana-frontend.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ede67d0cedc825eb0f00a9ce6d234929f3333dc039849584508adf37834fa0b1`.

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
> # Connects to Prometheus frontend UI: https://cloud.google.com/stackdriver/docs/managed-prometheus/query#ui-prometheus


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e14d3b77562b70f2"></a>


### File `build/grafana.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5c662e92de344dbbd6e5b378e5774cedd7436069`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-dbb2371f1c46cc0ad5.** [agones/build/grafana.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d30f7ecb540b04723fa7f73f488985b8f84c8d6be33ac06b507c66c4e6fcf148`.

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
> service:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-0bae6a11ed676eee"></a>


### File `build/install-release.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b3fc6fb5c1cf69f883cee74c6d3e348f008e515d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/install-release.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-85a19e4d1145c74401.** [agones/build/install-release.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/install-release.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `73b6cdb65449289026c69ba61332f720b74175ff8ac6e6c5ffd46bfe23b3807c`.

> 
> echo "Listing Helm releases in agones-system namespace..."
> helm ls -n agones-system
> 
> echo "Removing Agones deployment from agones-system namespace..."
> helm uninstall -n agones-system agones || echo "Failed to uninstall. Consider deleting the current cluster and setting up a new one. Refer to https://agones.dev/site/docs/installation/creating-cluster/gke/#create-a-standard-mode-cluster-for-agones"
> 
> echo "Listing pods in the agones-system namespace..."
> kubectl get pods -n agones-system
> 
> echo "Deleting agones-system namespace..."
> kubectl delete ns agones-system
> echo "Agones system namespace deleted."
> 
> echo "Helm repo update to fetch the latest version of Agones..."


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-eec4b741b2a4a2c6"></a>


### File `build/metallb-config.yaml.tpl`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `cc75154d768416042514f42df9857b88a6645801`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/metallb-config.yaml.tpl`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0a8c4b3d59d77d7722.** [agones/build/metallb-config.yaml.tpl, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/metallb-config.yaml.tpl#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bba1c80e2bdaa9f69966f34f3acf3506693a33dd97af5ee0afa70aefb48590fa`.

> 
> apiVersion: metallb.io/v1beta1
> kind: IPAddressPool
> metadata:
>   name: default-pool
>   namespace: metallb-system
> spec:
>   addresses:
>   - __RANGE__
> ---
> apiVersion: metallb.io/v1beta1
> kind: L2Advertisement
> metadata:
>   name: default-advertisement
>   namespace: metallb-system


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3befcf838700a390"></a>


### File `build/performance-test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b674e5f18479891f0ae131165750c86e27c76b78`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/performance-test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-35c7dea4b6f897d59d.** [agones/build/performance-test.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/performance-test.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8a13ab0be5dc1161033021f67c70d702ee5d25180eb1c543b42d485faff1a109`.

> set -e
> CLUSTER_NAME=$1
> CLUSTER_LOCATION=$2
> REGISTRY=$3
> PROJECT=$4
> REPLICAS=$5
> AUTO_SHUTDOWN_DELAY=$6
> BUFFER_SIZE=$7
> MAX_REPLICAS=$8
> DURATION=$9
> CLIENTS=${10}
> INTERVAL=${11}
> 
> export SHELL="/bin/bash"
> mkdir -p /go/src/agones.dev/


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-569ec7cd0f464f3e"></a>


### File `build/prometheus-google-managed.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4fc1267aa2a8e1352d28b7a5831c8df66632b7c0`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/prometheus-google-managed.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2d4123fb82294b9cf5.** [agones/build/prometheus-google-managed.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/prometheus-google-managed.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `01368eb456358403fe427805973ac20e62f66552085ec91ebb5e5e66642dcbcf`.

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
> apiVersion: monitoring.googleapis.com/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d617bb40da8eb065"></a>


### File `build/prometheus-stack.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ce9b16ec809d7d672d0a06ef8285683ad1018b2b`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/prometheus-stack.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-58eba4005a26a1f0ff.** [agones/build/prometheus-stack.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/prometheus-stack.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `10fc8b231a070a4e242ca7c3e95e4eb78978c7f8671f3f28811b2da177cfb831`.

> ---
> # disabled all not necessary chart
> defaultRules:
>   enabled: false
> alertmanager:
>   enabled: false
> kubeApiServer:
>   enabled: false
> kubelet:
>   enabled: false
> kubeControllerManager:
>   enabled: false
> coreDns:
>   enabled: false
> kubeDns:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-be6a54ac07b905e0"></a>


### File `build/prometheus.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e16c0cc4f8f44aee0c9f81551de253e0284a4b2c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/prometheus.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c2f5b9e83e11a3e15b.** [agones/build/prometheus.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/prometheus.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5ddbdcc1ee752564570d3a46d0032115bc692ee4a63da2a3d072fb21d0f471cd`.

> ---
> alertmanager:
>   enabled: false
> nodeExporter:
>   enabled: false
> kubeStateMetrics:
>   enabled: false
> pushgateway:
>   enabled: false
> server:
>   resources:
>     requests:
>       memory: 4Gi
>       cpu: 2
>   tolerations:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/agones-bot` -> module `build::agones-bot`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ba954877d14abcb8"></a>


### File `build/agones-bot/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f18221d24d8c4ac75fecb1413fe679679cca9592`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/agones-bot/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e394b1b6bd09da7da1.** [agones/build/agones-bot/Dockerfile, lines 14–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/agones-bot/Dockerfile#L14-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `faa16bec0f4a060b63c1a4b314aa9e7bd5ce3137cfe1dbcca8cfcfcc285e4f2e`.

> 
> FROM golang:1.26 AS build-env
> COPY . /go-src
> WORKDIR /go-src
> RUN CGO_ENABLED=0 go build -o /go-app .
> 
> FROM gcr.io/distroless/static-debian13
> COPY --from=build-env /go-app /
> ENTRYPOINT ["/go-app", "--alsologtostderr", "--v=0"]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8c9cd8625edda86f"></a>


### File `build/agones-bot/Makefile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6b8e6e60fe96b45c65694a462cc90c6956120239`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/agones-bot/Makefile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b329bc729f0cb1369b.** [agones/build/agones-bot/Makefile, lines 17–31](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/agones-bot/Makefile#L17-L31). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3255747906f9876646bf4cfcc3da41ac377b3d882c4040ccbdca5f889bbe4011`.

> 
> STORAGE ?= gs://agones-bot-pr-commenter-config
> 
> current_dir := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
> 
> all: build deploy
> 
> # build: creates the project Cloud Build custom build step and hosts it on gcr.io
> build:
> 	gcloud builds submit . --project=agones-images --config=cloudbuild.yaml $(ARGS)
> 
> # deploy pushes a new version to Cloud Run
> deploy:
> 	gsutil cp $(current_dir)/agones-bot-pr-commenter.yaml $(STORAGE)
> 	gcloud run deploy agones-bot-pr-commenter \


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-07f4925b55fab0ad"></a>


### File `build/agones-bot/README.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b275386270e4b3173db19cd560387c0e7ab1a042`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/agones-bot/README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-cca166571f60a64aaf.** [agones/build/agones-bot/README.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/agones-bot/README.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fa660a3ac327386d6c8935d0fa0e3d4ec1a7000b1a97c990983130717fb231b9`.

> 
> This is the bot that created comments on GitHub pull requests whenever a
> [Google Cloud Build](https://cloud.google.com/build) build passes or fails.
> 
> ## Setup
> 
> ### Image
> 
> Run `make build` to submit the cloud build to create the image that will be hosted on
> [Cloud Run](https://cloud.google.com/run).
> 
> ### Secrets
> 
> It is expected that there will be a secret named `agones-bot-pr-commenter` with a 
> [Github auth token](https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token) stored


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3ad595b4aea885dc"></a>


### File `build/agones-bot/agones-bot-pr-commenter.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fb95faa2443511ef58d64020b097ecfb0afc1a1b`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/agones-bot/agones-bot-pr-commenter.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7749726b97e3947618.** [agones/build/agones-bot/agones-bot-pr-commenter.yaml, lines 14–25](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/agones-bot/agones-bot-pr-commenter.yaml#L14-L25). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7bfa62478d21bcb0c038ad890cf662d437f5211222a920b3afbd92a7c774d4f0`.

> ---
> apiVersion: cloud-build-notifiers/v1
> kind: GithubNotifier
> metadata:
>   name: github-notifier
> spec:
>   notification:
>     filter: build.status == Build.Status.SUCCESS
>     delivery:
>   secrets:
>     - name: agones-bot-pr-commenter
>       value: projects/agones-images/secrets/agones-bot-pr-commenter/versions/latest


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4ed8c602543563b3"></a>


### File `build/agones-bot/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c47e367f5675e63a3567e71bb9290f4407a9623d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/agones-bot/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-89b63f961c57a8df0b.** [agones/build/agones-bot/cloudbuild.yaml, lines 14–26](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/agones-bot/cloudbuild.yaml#L14-L26). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `97c9c94aede3ffe4eab00403ac5bc62d4d1cd4b15997ff3e5f930853f757b70a`.

> ---
> steps:
>   - name: gcr.io/cloud-builders/docker
>     args:
>       - build
>       - --tag=${_REGISTRY}/agones-bot:latest
>       - '.'
> images:
>   - ${_REGISTRY}/agones-bot
> options:
>   dynamic_substitutions: true
> substitutions:
>   _REGISTRY: "us-docker.pkg.dev/${PROJECT_ID}/build"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e8ef2d33010dd63d"></a>


### File `build/agones-bot/go.mod`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b4bbd832e26df1baf0766b3644925ec202cab4b7`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/agones-bot/go.mod`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-778f5dd482bbad1008.** [agones/build/agones-bot/go.mod, lines 3–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/agones-bot/go.mod#L3-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `40813ce5c61ed169d3ac3d0544ba4799d11a1d76694e13cd4d5e560e91226ad6`.

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
> module github-bot
> 
> go 1.26.5
> 
> require (
> 	github.com/GoogleCloudPlatform/cloud-build-notifiers v0.0.0-20260318173334-9bb54edb0b8b
> 	github.com/Masterminds/sprig/v3 v3.3.0
> 	github.com/golang/glog v1.2.5


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c1946ee32128a93b"></a>


### File `build/agones-bot/go.sum`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1ebc91fa175cf565d99f3f8771091ed8bb9bcf75`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/agones-bot/go.sum`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e016ee8fef436cec44.** [agones/build/agones-bot/go.sum, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/agones-bot/go.sum#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0923c20e7312ad284c122b6577af716c3bb4ff217afa81da3e318b3e4388bf2a`.

> cel.dev/expr v0.25.2 h1:K6j46C81hXtZQfuX60cVWQFBJahKSE2gfRbNuvr5bFs=
> cel.dev/expr v0.25.2/go.mod h1:hrXvqGP6G6gyx8UAHSHJ5RGk//1Oj5nXQ2NI02Nrsg4=
> cloud.google.com/go v0.123.0 h1:2NAUJwPR47q+E35uaJeYoNhuNEM9kM8SjgRgdeOJUSE=
> cloud.google.com/go v0.123.0/go.mod h1:xBoMV08QcqUGuPW65Qfm1o9Y4zKZBpGS+7bImXLTAZU=
> cloud.google.com/go/auth v0.19.0 h1:DGYwtbcsGsT1ywuxsIoWi1u/vlks0moIblQHgSDgQkQ=
> cloud.google.com/go/auth v0.19.0/go.mod h1:2Aph7BT2KnaSFOM0JDPyiYgNh6PL9vGMiP8CUIXZ+IY=
> cloud.google.com/go/auth/oauth2adapt v0.2.8 h1:keo8NaayQZ6wimpNSmW5OPc283g65QNIiLpZnkHRbnc=
> cloud.google.com/go/auth/oauth2adapt v0.2.8/go.mod h1:XQ9y31RkqZCcwJWNSx2Xvric3RrU88hAYYbjDWYDL+c=
> cloud.google.com/go/cloudbuild v1.25.0 h1:Fkg+iJdN7bfICZJzLr/XV+k9aVxXS/hakIlhjDIRIDw=
> cloud.google.com/go/cloudbuild v1.25.0/go.mod h1:lCu+T6IPkobPo2Nw+vCE7wuaAl9HbXLzdPx/tcF+oWo=
> cloud.google.com/go/compute/metadata v0.9.0 h1:pDUj4QMoPejqq20dK0Pg2N4yG9zIkYGdBtwLoEkH9Zs=
> cloud.google.com/go/compute/metadata v0.9.0/go.mod h1:E0bWwX5wTnLPedCKqk3pJmVgCBSM6qQI1yTBdEb3C10=
> cloud.google.com/go/iam v1.6.0 h1:JiSIcEi38dWBKhB3BtfKCW+dMvCZJEhBA2BsaGJgoxs=
> cloud.google.com/go/iam v1.6.0/go.mod h1:ZS6zEy7QHmcNO18mjO2viYv/n+wOUkhJqGNkPPGueGU=
> cloud.google.com/go/logging v1.13.2 h1:qqlHCBvieJT9Cdq4QqYx1KPadCQ2noD4FK02eNqHAjA=


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/always` -> module `build::always`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-6a2f0880c4bb4e97"></a>


### File `build/always/always-be-building.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `02f8f55932bf8255c298a6c30ecf60f397ac3882`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/always/always-be-building.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2f43079da7f4b1d326.** [agones/build/always/always-be-building.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/always/always-be-building.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b8d3931172dedc55422537cf2582bf61ea5ce53ac42650e2127b150e1b21ae8f`.

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
> # Always Be Building


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-allocation-images/go` -> module `build::build-allocation-images::go`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-e410e445e3d38bcd"></a>


### File `build/build-allocation-images/go/gen.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ce1dab952683d7d5132a4fdd47c0665a15e14177`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-allocation-images/go/gen.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7837c78fb2e6ec3c46.** [agones/build/build-allocation-images/go/gen.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-allocation-images/go/gen.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4ebdb7052de43033b64508837df4a348526a7f03823cab4743944ee5c0a7df4c`.

> 
> set -ex
> 
> header() {
>     cat /go/src/agones.dev/agones/build/boilerplate.go.txt "$1" | sponge "$1"
> }
> 
> export GO111MODULE=on
> 
> mkdir -p /go/src/
> cp -r /go/src/agones.dev/agones/vendor/* /go/src/
> 
> cd /go/src/agones.dev/agones
> go install -mod=vendor github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway
> go install -mod=vendor github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-image` -> module `build::build-image`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-d3c4de60fa320c14"></a>


### File `build/build-image/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0ec583af20ccce771f58ab38d4a077e65df17a25`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6772b5e95ade23b071.** [agones/build/build-image/Dockerfile, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/Dockerfile#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `01b27ed8ce1b2e8b06e272186a984f18532ad7b76aba23b7c1ece428b0978026`.

> 
> FROM debian:trixie
> 
> RUN apt-get update && \
>     apt-get install -y build-essential gnupg curl git wget psmisc rsync make python3 bash-completion ruby\
>     zip nano jq graphviz gettext-base plantuml && \
>     apt-get clean
> 
> RUN gem install fpm && \
>     fpm -s empty -t deb -p python2.7-0.0.0-0-any.deb --name python2.7 --license MIT --version 0.0.0 --architecture all --description 'Dummy package to satisfy google-cloud-cli dependencies.' --maintainer 'Yourself' && \
>     dpkg -i python2.7-0.0.0-0-any.deb && \
>     rm python2.7-0.0.0-0-any.deb
> 
> # install go
> WORKDIR /usr/local


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-42180c4304e4fd03"></a>


### File `build/build-image/gen-crd-code.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `08a053ffff32ccd49cbc0a687b02f97ed02b7c51`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/gen-crd-code.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4a460f903cb4831c47.** [agones/build/build-image/gen-crd-code.sh, lines 18–32](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/gen-crd-code.sh#L18-L32). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e4a5226f10d87af45fa9b3f701b35ec66b6b30d0e151beac3991ceae41e5d361`.

> 
> set -x
> set -o errexit
> set -o nounset
> set -o pipefail
> 
> CODEGEN_SCRIPT="/go/src/k8s.io/code-generator/kube_codegen.sh"
> 
> source "${CODEGEN_SCRIPT}"
> 
> echo "Generating CRD client code..."
> OUTPUT_DIR="/go/src/agones.dev/agones/pkg/client"
> OUTPUT_PKG="agones.dev/agones/pkg/client"
> 
> kube::codegen::gen_client \


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-image/cache` -> module `build::build-image::cache`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-e74bbf9a3f539904"></a>


### File `build/build-image/cache/Dockerfile-base`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3669b4c9120048cebf46d6f80f5d8ab741803846`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/cache/Dockerfile-base`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-cabd564fe67a4f7bea.** [agones/build/build-image/cache/Dockerfile-base, lines 14–27](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/cache/Dockerfile-base#L14-L27). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c6b4057dafe74dc981a87021e621d099e180401a22e524f11d279eda025d235d`.

> 
> FROM gcr.io/cloud-builders/gcloud-slim
> 
> RUN    apt-get update \
>     && apt-get install -y gcc python3-dev python3-setuptools python3-pip \
>     && apt-get clean \
>     && rm -rf /var/lib/apt/lists/* \
>     && pip3 install --no-cache-dir -U crcmod
> 
> COPY checksum /bin
> COPY save_cache /bin
> COPY restore_cache /bin
> 
> RUN chmod +x /bin/checksum /bin/save_cache /bin/restore_cache


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4c71f03593d33199"></a>


### File `build/build-image/cache/Dockerfile-restore`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fbad1368a1129187c99aee28ce9097e8f6d6b814`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/cache/Dockerfile-restore`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-622092640f93e33846.** [agones/build/build-image/cache/Dockerfile-restore, lines 14–18](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/cache/Dockerfile-restore#L14-L18). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3dd569212935904d3991fc001f6b98c84b8d9a5f0900a2e90bc42d6b1442f6e6`.

> 
> ARG project_id
> FROM us-docker.pkg.dev/${project_id}/ci/cache
> 
> ENTRYPOINT ["restore_cache"]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-00b318201e5386cb"></a>


### File `build/build-image/cache/Dockerfile-save`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `82d6a0007c494fa44ea4b2e47c8fb1289f761015`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/cache/Dockerfile-save`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7eee13312e2a6c5b2f.** [agones/build/build-image/cache/Dockerfile-save, lines 14–18](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/cache/Dockerfile-save#L14-L18). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f857378ac3fd798a97877db9aa078f433ea9cde28c28a51b38c254325af511f1`.

> 
> ARG project_id
> FROM us-docker.pkg.dev/${project_id}/ci/cache
> 
> ENTRYPOINT ["save_cache"]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5b6c9c4fecb28ea7"></a>


### File `build/build-image/cache/README.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d2590a76f8e90572efc91c8decea22511a3c3654`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/cache/README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-158108c706e032792f.** [agones/build/build-image/cache/README.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/cache/README.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8a0e616d14adb5c8b5efe64f11ecaccdf5aba291606d71df5f7427996b11522c`.

> 
> This `README.md` was originally based on [cloud-builders-community](https://github.com/GoogleCloudPlatform/cloud-builders-community), which contains source code for community-contributed Docker images. You can use these images as build steps for [Google Cloud Build](https://cloud.google.com/build/docs). 
> 
> The key change made upon importing the [cache builder's](https://github.com/GoogleCloudPlatform/cloud-builders-community/tree/master/cache) code into Agones was the update of image tags to use Google Artifact Registry instead of Google Container Registry.
> 
> # Cache builders
> 
> This includes a pair of builders, `save_cache` and `restore_cache`, that work together to cache files between builds to a GCS bucket (or local file).
> 
> ## Using the `save_cache` builder
> 
> All options that require a value use the form `--option=value` or `-o=value` so that they look nice in Yaml files.
> 
> | Option           | Description                                                 |
> | ---------------- | ----------------------------------------------------------- |


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2e7ca8111fff9867"></a>


### File `build/build-image/cache/checksum`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0245eb546c578e0cfb53a2a73c98ac64e5bf52e6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/cache/checksum`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-eaf1e88f330b441c6e.** [agones/build/build-image/cache/checksum, lines 16–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/cache/checksum#L16-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `722197b3665c55103a10cf19ffe17f180632993aafe79973885007a640ae48d1`.

> 
> echo $(cksum $@ | cut -d' ' -f1) | tr ' ' -


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e99d4c4d3919bec9"></a>


### File `build/build-image/cache/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d06d7a6f1509f9e1284aedba83f1f42c302915a6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/cache/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-304c21c850a29c732e.** [agones/build/build-image/cache/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/cache/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `05d7016c1d232ffd7fa623b29a86a1a041f1985055d5e89264231cdaade9d08c`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2ca7153d948bd7ba"></a>


### File `build/build-image/cache/restore_cache`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d55363e4053f09b80f2327d290d9838ffa51f895`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/cache/restore_cache`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-86afceb5ae455d1988.** [agones/build/build-image/cache/restore_cache, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/cache/restore_cache#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `88daa24bfba526b790bab22ddfae3d43745cbe954ddaa61855c0ac4c835452d2`.

> 
> BUCKET=""
> SRC_DIR="."
> KEY="cache"
> KEY_FALLBACK_PATTERN=""
> PATHS=()
> 
> function print_usage {
>   echo "Usage: $0 --bucket=gs://my-cache-bucket --key=cache-key"
>   echo
>   echo "Saves the specified paths to a cache file located in the out directory."
>   echo
>   echo "  -b,  --bucket       The cloud storage bucket to download the cache from. [optional]"
>   echo "  -s,  --src          The local directory in which the cache is stored. [optional]"
>   echo "  -k,  --key          The cache key used for this cache file. [optional]"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3912f63e8dbb3023"></a>


### File `build/build-image/cache/save_cache`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6404f0436eb71e1f1a65c878406883c01f14ada7`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-image/cache/save_cache`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e814c6a08c4f018ae0.** [agones/build/build-image/cache/save_cache, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-image/cache/save_cache#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `20ed5c079969b98c1dba6f14a3014455191d48073cc497b9dd657efed9f90dc0`.

> 
> BUCKET=""
> OUT_DIR="."
> KEY="cache"
> THRESHOLD="50M"
> PATHS=()
> 
> function print_usage {
>   echo "Usage: $0 --out=/cache_dir --key=cache-key --path=/path/to/files/1 --path=/path/to/files/2"
>   echo
>   echo "Saves the specified paths to a cache file located in the out directory."
>   echo
>   echo "  -b, --bucket      The cloud storage bucket to upload the cache to. [optional]"
>   echo "  -o, --out         The output directory to write the cache to. [optional]"
>   echo "  -k, --key         The cache key used for this cache file."


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images` -> module `build::build-sdk-images`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-06c8a35193352e95"></a>


### File `build/build-sdk-images/README.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9f554f605d8369f82386befc1149bdfb5917626c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7ea6ea65c1a1e0cb52.** [agones/build/build-sdk-images/README.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/README.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `252c45404538045e58d3cf1770d533afb979d8457bf2d912d3e6b0c21d7de15f`.

> 
> As we seen before GameServers can communicate their current state back to Agones controller using a side car. This sidecar runs a GRPC server handling the communication to Agones, the GameServer connects using a SDK which a thin wrapper around GRPC client code.
> 
> By using GRPC, adding more languages should be pretty straightforward as it supports code generation for many of them.
> 
> > if your language doesn't support GRPC you can still create a SDK by connecting to the HTTP GRPC gateway.
> 
> This guide explains how to build, test and generate GRPC code for our SDKs but also how to add a new language.
> 
> ## How it works
> 
> Our build system heavily rely on Docker and make, but that's all you need. The rest of the GRPC tooling and language specific dependencies are installed inside Docker images via Dockerfile.
> 
> We separated each language specific tooling in their on Docker image, this way building them will be faster and easier to maintain.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images/cpp` -> module `build::build-sdk-images::cpp`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-d58a55ef4f836503"></a>


### File `build/build-sdk-images/cpp/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7ed1b95d3e07eef920fda58fc5dff2b5be1440b7`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/cpp/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c8ac1f664dd771b70c.** [agones/build/build-sdk-images/cpp/Dockerfile, lines 14–25](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/cpp/Dockerfile#L14-L25). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `87726a7128f196a6487080eb5677479b7064eb695053394f054133a8c9b19ac2`.

> ARG BASE_IMAGE=agones-build-sdk-base:latest
> FROM $BASE_IMAGE
> 
> RUN apt-get --allow-releaseinfo-change update && \
>     apt-get install -y zip wget clang-format && \
>     apt-get clean
> 
> WORKDIR /go/src/agones.dev/agones
> 
> # code generation scripts
> COPY *.sh /root/
> RUN chmod +x /root/*.sh


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ab7df4e19417b889"></a>


### File `build/build-sdk-images/cpp/build-sdk-test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8004149e60e409fb3d3ffa23fd3d1cf8f2542aae`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/cpp/build-sdk-test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-34912d71b216465b84.** [agones/build/build-sdk-images/cpp/build-sdk-test.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/cpp/build-sdk-test.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c36208c88a8e2c5798b0cf0fcb9fd76d54ea88156e1315ae9475d3b775b5b153`.

> 
> set -ex
> GO111MODULE=off
> DIR=/go/src/agones.dev/agones/test/sdk/cpp/
> echo "$DIR"/sdk
> 
> # Copy all CPP SDK files into a new directory
> if [[ ! -d $DIR/sdk ]]
> then
>     mkdir -p "$DIR"/sdk/.build
>     cp -r /go/src/agones.dev/agones/sdks/cpp/* $DIR/sdk
>     cd $DIR/sdk/.build
>     cmake .. -DCMAKE_BUILD_TYPE=Release -DAGONES_SILENT_OUTPUT=OFF \
>       -DCMAKE_INSTALL_PREFIX=$DIR/sdk/.build -G "Unix Makefiles" -Wno-dev
> else


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2eb3a633c1dd59f3"></a>


### File `build/build-sdk-images/cpp/build.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d107c666ea546b9aa96d37d36728ee7533707d26`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/cpp/build.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-01766086f8519595d9.** [agones/build/build-sdk-images/cpp/build.sh, lines 16–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/cpp/build.sh#L16-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b89591150f6ebe5a92b594e7ccd1ff5adc6024c3df675c37a39275cf969aa37b`.

> 
> set -ex
> 
> cd ./sdks/cpp
> make build verify install archive VERSION=$VERSION


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-047fea6e2f3ceeed"></a>


### File `build/build-sdk-images/cpp/clean.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3759cc3dcaa6cf074e71c8413f06391b2498308c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/cpp/clean.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2c7c4c67666174b0c8.** [agones/build/build-sdk-images/cpp/clean.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/cpp/clean.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `65a17cd67bab81bef1969b7c343fd75b76eef2d8f4ea122c67aa46787260c71b`.

> 
> set -ex
> rm -rf /go/src/agones.dev/agones/test/sdk/cpp/.build
> rm -rf /go/src/agones.dev/agones/test/sdk/cpp/sdk


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4841f60eeabf223b"></a>


### File `build/build-sdk-images/cpp/gen.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e36d6c887cff98c5b3f733265791a8860b41625e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/cpp/gen.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-25c3f3fc568fcc6f8c.** [agones/build/build-sdk-images/cpp/gen.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/cpp/gen.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5bc5a87bda77370e2a1842844e763bcefbf7c503007a79cf91da2fb541bd0f45`.

> 
> set -ex
> 
> header() {
>     cat /go/src/agones.dev/agones/build/boilerplate.go.txt ./$1 >> $2/$1
> }
> 
> sdk=/go/src/agones.dev/agones/proto/sdk
> googleapis=/go/src/agones.dev/agones/proto/googleapis
> gatewaygrpc=/go/src/agones.dev/agones/proto/grpc-gateway
> protoc_intermediate=/go/src/agones.dev/agones/sdks/cpp/.generated
> protoc_destination=/go/src/agones.dev/agones/sdks/cpp
> 
> mkdir -p ${protoc_intermediate}
> mkdir -p ${protoc_destination}/src/agones


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4ed5f64facf6638f"></a>


### File `build/build-sdk-images/cpp/sdktest.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `97edde1fc8ea8d1d5e1d805996534f24b8126865`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/cpp/sdktest.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c815d13270a86442ae.** [agones/build/build-sdk-images/cpp/sdktest.sh, lines 16–18](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/cpp/sdktest.sh#L16-L18). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `24ac31ba9bfae15d3883cb06a52569d20010b1ef91873987544a65aaf3677dfa`.

> 
> set -ex
> /go/src/agones.dev/agones/test/sdk/cpp/.build/.bin/cpp-simple


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6ee23c95bc4b54fb"></a>


### File `build/build-sdk-images/cpp/shell.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f8483626aae4a9752da793932652618eae1b82b3`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/cpp/shell.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7a701f016735565eb6.** [agones/build/build-sdk-images/cpp/shell.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/cpp/shell.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ffa78d3e5f7d77f4cfc25c9ccb10f6dbe1dbb9ce5f882d5e5f9af38c53ca3cc2`.

> 
> set -ex
> cd ./sdks/cpp
> bash


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images/csharp` -> module `build::build-sdk-images::csharp`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-796e202fa375194d"></a>


### File `build/build-sdk-images/csharp/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b6907fef1da5936dd1faeb21a55791b70be33943`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/csharp/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c639ae6220c5a9bfcd.** [agones/build/build-sdk-images/csharp/Dockerfile, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/csharp/Dockerfile#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `29f810e5d668049fe46688274fa84d91de1201b3e51711248c024e6c37db07fc`.

> ARG BASE_IMAGE=agones-build-sdk-base:latest
> FROM $BASE_IMAGE
> 
> # Add installation step required to generate your SDK.
> # This should contains tooling required to generate GRPC code, build (if distribution is required) 
> # and test your integration.
> 
> RUN apt-get --allow-releaseinfo-change update && \
>     apt-get install -y wget
> 
> RUN apt-get update \
>     && apt-get install -y apt-transport-https 
> 
> RUN wget -q https://packages.microsoft.com/config/debian/13/packages-microsoft-prod.deb -O packages-microsoft-prod.deb \
>     && dpkg -i packages-microsoft-prod.deb \


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2922a00d10787be4"></a>


### File `build/build-sdk-images/csharp/build.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `93dd01c8e776a411cd1287da8b7fc54d7647a103`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/csharp/build.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0c7335e6af0eae0f6a.** [agones/build/build-sdk-images/csharp/build.sh, lines 16–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/csharp/build.sh#L16-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7993e4929a51daf40a712408b1200fc107f2cef4b3dc1e8dccefeaf281f35bb9`.

> 
> set -ex
> 
> #Is obsolete if artifacts are not needed when NuGet publish is written.
> #I'm still not sure how the artifacts are used, using `publish` for self-contained release packages.
> cd ./sdks/csharp/sdk
> dotnet publish -c Release


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a860207a77f4bb70"></a>


### File `build/build-sdk-images/csharp/gen.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ddd6fe6dcd5696bbb944100df9d4de8da968dab3`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/csharp/gen.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-52be071a12cc97c4de.** [agones/build/build-sdk-images/csharp/gen.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/csharp/gen.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `463d545f65bf885a47be2c9931f0d3219a327f03043476dfc65fedb46d79d607`.

> 
> set -ex
> 
> proto=/go/src/agones.dev/agones/proto
> sdk=${proto}/sdk
> googleapis=${proto}/googleapis
> csharp_proto_file_output_dir=/go/src/agones.dev/agones/sdks/csharp/proto
> 
> echo "Copying protobuffers to csharp sdk"
> mkdir -p ${csharp_proto_file_output_dir}
> cp -r ${sdk} ${csharp_proto_file_output_dir}
> cp -r ${googleapis} ${csharp_proto_file_output_dir}
> 
> # Remove protoc-gen-openapiv2 definitions because C# package doesn't support grpc-gateway
> sed -i -e 's/import "protoc-gen-openapiv2\/options\/annotations.proto";//' ${csharp_proto_file_output_dir}/sdk/sdk.proto


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9c3ccb8d003d573f"></a>


### File `build/build-sdk-images/csharp/publish.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fef6237d71c8ed32bb82e4adfacc54aac6e032d4`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/csharp/publish.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4ae1a570c579625b75.** [agones/build/build-sdk-images/csharp/publish.sh, lines 16–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/csharp/publish.sh#L16-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ebf75d07528e3ae45445e2be422a8b0f8feba8742bff823a8e971c2679c668b5`.

> 
> set -ex
> cd ./sdks/csharp/sdk
> 
> dotnet pack csharp-sdk.csproj --configuration Release --include-source --output ./pkgs
> read -p 'NuGet API Key: ' TOKEN && \
> dotnet nuget push ./pkgs/AgonesSDK.$VERSION.nupkg -k $TOKEN -s https://api.nuget.org/v3/index.json


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a8fdc8106b26f0e7"></a>


### File `build/build-sdk-images/csharp/sdktest.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d27cb4bb11672b56d43ba4ad1f1bafd0b31c51cd`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/csharp/sdktest.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b4c17e613802e759b2.** [agones/build/build-sdk-images/csharp/sdktest.sh, lines 16–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/csharp/sdktest.sh#L16-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2184143d8259ce8af3f68376b8909964893667e27e0ade62fe0c9b1fe6bd24c4`.

> 
> set -ex
> 
> cd /go/src/agones.dev/agones/test/sdk/csharp
> 
> dotnet run


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a761c271c6311058"></a>


### File `build/build-sdk-images/csharp/shell.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d4316564ffd20addca0874e752e30f2086d54ca8`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/csharp/shell.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-97a6b2f5b31a0daa84.** [agones/build/build-sdk-images/csharp/shell.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/csharp/shell.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `599a568492f5a3ace745500a7bebc0593fead48447058e564fcd2dc24d785f73`.

> 
> set -ex
> cd ./sdks/csharp/sdk
> bash


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-013573eb7743d7b4"></a>


### File `build/build-sdk-images/csharp/test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `376f9d3d9e3af9db80971dbd3631541c9b21929c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/csharp/test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d10c45009d1be7ed6d.** [agones/build/build-sdk-images/csharp/test.sh, lines 16–23](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/csharp/test.sh#L16-L23). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `55a1afa81e4638c46c68dc51bbd79040dbf4aa926118c943cb94329df8a128c2`.

> 
> set -ex
> 
> # build the SDK project
> # Run Code Integirty tests
> cd ./sdks/csharp/test
> 
> dotnet test -c Release


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images/go` -> module `build::build-sdk-images::go`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-e9fe0d7d87ecf61d"></a>


### File `build/build-sdk-images/go/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d1585c8029054ea15fbd520aa6e8e64cf618173d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/go/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-eac18f06517e332c91.** [agones/build/build-sdk-images/go/Dockerfile, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/go/Dockerfile#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5af78e4987bfd289496053e81eb70b728955985f05c351f84033dadc81e7faba`.

> ARG BASE_IMAGE=agones-build-sdk-base:latest
> FROM $BASE_IMAGE
> 
> RUN apt-get --allow-releaseinfo-change update && \
>     apt-get install -y wget jq && \
>     apt-get clean
> 
> # install go
> WORKDIR /usr/local
> ENV GO_VERSION=1.26.5
> ENV GO111MODULE=on
> ENV GOPATH /go
> RUN ARCH=$(dpkg --print-architecture) && \
>     wget -q https://dl.google.com/go/go${GO_VERSION}.linux-${ARCH}.tar.gz && \
>     tar -xzf go${GO_VERSION}.linux-${ARCH}.tar.gz && rm go${GO_VERSION}.linux-${ARCH}.tar.gz && mkdir -p ${GOPATH}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1102f9f89c68dc5c"></a>


### File `build/build-sdk-images/go/gen.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `18d4281a2090f838d187a698ddd4ceb6e60ec169`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/go/gen.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f520e246f91a31113b.** [agones/build/build-sdk-images/go/gen.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/go/gen.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a57231a1ad6c10a1508d844a22aa8575553257e9af680189f857d4e31bbcaf99`.

> 
> set -ex
> 
> header() {
>     cat /go/src/agones.dev/agones/build/boilerplate.go.txt "$1" | sponge "$1"
> }
> 
> sdk=/go/src/agones.dev/agones/proto/sdk
> googleapis=/go/src/agones.dev/agones/proto/googleapis
> gatewaygrpc=/go/src/agones.dev/agones/proto/grpc-gateway
> 
> export GO111MODULE=on
> 
> mkdir -p /go/src/
> cp -r /go/src/agones.dev/agones/vendor/* /go/src/


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-efc5311fc3f46215"></a>


### File `build/build-sdk-images/go/sdktest.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0164c50454885579424d405b43527ebe75d88dfc`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/go/sdktest.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9e864f24f8b019a723.** [agones/build/build-sdk-images/go/sdktest.sh, lines 16–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/go/sdktest.sh#L16-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `493f36780da44b30004fb2edfa4bf33e2011be9eb89b89273a9e1c46424fcd47`.

> 
> set -ex
> GO111MODULE=off
> cd /go/src/agones.dev/agones/test/sdk/go
> go run sdk-client-test.go


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-86abf1c8b717fec8"></a>


### File `build/build-sdk-images/go/test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a6b7a8e4cdd238be94cf1f468127e0d1dd4045e8`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/go/test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-18d3c2b792dc196ca9.** [agones/build/build-sdk-images/go/test.sh, lines 16–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/go/test.sh#L16-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `30d63c44fe0c8aba7c04aa23c969003e867ac0cbe824c7496fa442b7b80ffee3`.

> 
> set -ex
> GO111MODULE=off
> cd /go/src/agones.dev/agones/
> go test -v -race ./sdks/go/


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images/node` -> module `build::build-sdk-images::node`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-86cbbe1f2f153cbb"></a>


### File `build/build-sdk-images/node/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a206cbd8eeaf83917577da29aea68a82f5996fd7`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/node/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ba6bc788207ee97a0a.** [agones/build/build-sdk-images/node/Dockerfile, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/node/Dockerfile#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `31604eb9e87292dce3d9ed1cf0b8809e3975321ef16c7cc862dfd7f7cda43f6c`.

> ARG BASE_IMAGE=agones-build-sdk-base:latest
> FROM $BASE_IMAGE
> 
> ## --allow-releaseinfo-change is a workaround for the new debian release
> ## Can be removed when upgrading to bullseye
> RUN apt-get --allow-releaseinfo-change update && apt-get install -y curl && apt-get install -y && \
>     apt-get install -y gnupg && apt-get clean
> 
> # Import the Nodesource GPG key
> RUN apt-get update && \
>     apt-get install -y ca-certificates curl gnupg && \
>     mkdir -p /etc/apt/keyrings && \
>     curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
> 
> # Create deb repository for a specific Node major version


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-df09eeb8a4e7582a"></a>


### File `build/build-sdk-images/node/build-sdk-test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `662d72675a5d2de5d4bb174b9caab85e91a4a9e7`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/node/build-sdk-test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-117f500e1a8a346629.** [agones/build/build-sdk-images/node/build-sdk-test.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/node/build-sdk-test.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b5362cc497d1c3e00c4fb461fdade60a8ea4611fa0e93784b646a14efb7a6edb`.

> 
> cd /go/src/agones.dev/agones/test/sdk/nodejs
> npm install --quiet
> npm rebuild --quiet
> 
> # If first 'npm install' attempt fails, which could occur for a variety of reasons,
> # do one more attempt
> if [ $? -gt 0 ]
> then
>     echo "Run npm install one more time"
>     rm -rf /go/src/agones.dev/agones/sdks/nodejs/node_modules
>     rm -rf /go/src/agones.dev/agones/test/sdk/nodejs/node_modules
>     rm /go/src/agones.dev/agones/test/sdk/nodejs/package-lock.json
>     npm cache clean
>     npm rebuild --quiet


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b6df81686c2ff459"></a>


### File `build/build-sdk-images/node/clean.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `21ce042a938751c921d69f47ce721e2d72207f80`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/node/clean.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-28ab447205c029f471.** [agones/build/build-sdk-images/node/clean.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/node/clean.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `069e26c9445d84d2922e28964c537871b0fcf12563526c2cc0a19ee68d417a83`.

> 
> set -ex
> rm -rf /go/src/agones.dev/agones/test/sdk/nodejs/node_modules
> rm -f /go/src/agones.dev/agones/test/sdk/nodejs/package-lock.json


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8c63b937ed35fb54"></a>


### File `build/build-sdk-images/node/gen.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `67f2948b84ee7999f04f30163c50deb73d54373a`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/node/gen.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5ed2a1292022b0b9e8.** [agones/build/build-sdk-images/node/gen.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/node/gen.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c0c76ba66e2d420d9e8eade88902980f8eab5dcee49b4c3bda4aba13a7b79a05`.

> 
> set -ex
> 
> header() {
>     cat ./build/boilerplate.go.txt $1 >> ./tmp.js && mv ./tmp.js $1
> }
> 
> sdk=/go/src/agones.dev/agones/proto/sdk
> googleapis=/go/src/agones.dev/agones/proto/googleapis
> gatewaygrpc=/go/src/agones.dev/agones/proto/grpc-gateway
> 
> cd /go/src/agones.dev/agones
> 
> mkdir -p ./sdks/nodejs/lib/alpha
> mkdir -p ./sdks/nodejs/lib/beta


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-07fcb239089501aa"></a>


### File `build/build-sdk-images/node/sdktest.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `82b25c6c6868eb1e88dbbc45bb8fdb1bf125449b`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/node/sdktest.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ef96485b3e77575d2a.** [agones/build/build-sdk-images/node/sdktest.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/node/sdktest.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `13e4e28f27d8456987f0caad24fd5f81fe0e4d3658f7b292b3b468f185368a38`.

> 
> set -ex
> cd /go/src/agones.dev/agones/test/sdk/nodejs
> npm run testSDK


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e1d4b788f3d2762e"></a>


### File `build/build-sdk-images/node/shell.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `eb5223f63640eb934e0034f7ebe0803d3e59b5d8`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/node/shell.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-28d23e937255d57245.** [agones/build/build-sdk-images/node/shell.sh, lines 16–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/node/shell.sh#L16-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5a09cda1aa0e3bd3aa7df95d91ebb5c8a486b91b757f490c22827442719fcf7e`.

> 
> set -ex
> mkdir -p /root/.ssh || true
> cp /tmp/.ssh/* /root/.ssh/
> cd ./sdks/nodejs
> bash


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-54357a9a454c08e1"></a>


### File `build/build-sdk-images/node/test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f5cc1a4c952b6c8d43d10a4374bc5ffcaecfb578`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/node/test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5879dee57d0f7be96a.** [agones/build/build-sdk-images/node/test.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/node/test.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `332b612049d29744595a2ac4ef26b7d7b8441f6671a6f0626260970ce92e1562`.

> 
> set -ex
> cd ./sdks/nodejs
> npm install-test


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images/python` -> module `build::build-sdk-images::python`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-3e91166d45be17b8"></a>


### File `build/build-sdk-images/python/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `79fc3fb3ab66e619548b48bcda265dc6f17ff85a`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/python/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2fe614c3bae907bade.** [agones/build/build-sdk-images/python/Dockerfile, lines 14–23](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/python/Dockerfile#L14-L23). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `434ef03b79f1d7ad07c12bc873e7934e321688cb3996b2b6c0f0d2752cef0b56`.

> ARG BASE_IMAGE=agones-build-sdk-base:latest
> FROM $BASE_IMAGE
> 
> RUN apt-get --allow-releaseinfo-change update && \
>     apt-get install -y python3 python3-pip python3-venv && \
>     apt-get clean
> 
> # code generation scripts
> COPY *.sh /root/
> RUN chmod +x /root/*.sh


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-eaf14a6aee82d172"></a>


### File `build/build-sdk-images/python/gen.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f3700f57da4f0b69cdb9c320e6c2cc8d6cf637e5`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/python/gen.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-57a3cf1de06dae6394.** [agones/build/build-sdk-images/python/gen.sh, lines 16–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/python/gen.sh#L16-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `06b0db942242b70cf5d834383a703906ba973f53d0d034062d8c41aabea6ff4e`.

> 
> set -ex
> cd /go/src/agones.dev/agones/sdks/python
> python3 -m venv .venv
> .venv/bin/pip install grpcio-tools==1.80.0
> PATH="$(pwd)/.venv/bin:$PATH" bash generate.sh


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f51887f2f6129d88"></a>


### File `build/build-sdk-images/python/sdktest.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ce68143f51ebe74f4b5e3c1067fe4e5f1c1fc122`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/python/sdktest.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bac422ff4dd27c12f7.** [agones/build/build-sdk-images/python/sdktest.sh, lines 16–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/python/sdktest.sh#L16-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `776d06069cd85f158dca500101069088638370d44e2f0cd28bc40a89ecc6b929`.

> 
> set -ex
> cd /go/src/agones.dev/agones/sdks/python
> python3 -m venv .venv
> .venv/bin/pip install grpcio protobuf
> .venv/bin/python /go/src/agones.dev/agones/test/sdk/python/testSDKClient.py


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-86d6bb549a1aa77f"></a>


### File `build/build-sdk-images/python/test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3e8805096ad875cf8e564e2d81fe3365cd5d0ecc`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/python/test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d1b307524866b19275.** [agones/build/build-sdk-images/python/test.sh, lines 16–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/python/test.sh#L16-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f94c3a49811cb7df0eb3ee84c1ba96fd68c95de2c7333d92eec59a65649c0921`.

> 
> set -ex
> cd /go/src/agones.dev/agones/sdks/python
> python3 -m venv .venv
> .venv/bin/pip install grpcio protobuf pytest
> .venv/bin/python -m pytest tests/


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images/restapi` -> module `build::build-sdk-images::restapi`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-f1551a9fa1395937"></a>


### File `build/build-sdk-images/restapi/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d3f0d4a6c07057aae1fa99956246a9f7b52d8c2e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/restapi/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c67f0f10b0e13c7ddb.** [agones/build/build-sdk-images/restapi/Dockerfile, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/restapi/Dockerfile#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3c946423d5d4490c94a8a5f9fb31332eaafcf9c1464fb3e29b00b95d0081986c`.

> ARG BASE_IMAGE=agones-build-sdk-base:latest
> FROM $BASE_IMAGE
> 
> RUN apt-get --allow-releaseinfo-change update && \
>     apt-get install -y wget jq gnupg && \
>     apt-get clean
> 
> # install go
> WORKDIR /usr/local
> ENV GO_VERSION=1.26.5
> ENV GO111MODULE=on
> ENV GOPATH /go
> RUN ARCH=$(dpkg --print-architecture) && \
>     wget -q https://dl.google.com/go/go${GO_VERSION}.linux-${ARCH}.tar.gz && \
>     tar -xzf go${GO_VERSION}.linux-${ARCH}.tar.gz && rm go${GO_VERSION}.linux-${ARCH}.tar.gz && mkdir -p ${GOPATH}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6a8ed63ddbf5bdf7"></a>


### File `build/build-sdk-images/restapi/gen.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e42d764db80a55b13124b03fcff8be37362e6915`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/restapi/gen.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-eb0d7626b9cd94a6b6.** [agones/build/build-sdk-images/restapi/gen.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/restapi/gen.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f7d8d93665f32c8da424298b62a574c2be83fbf8ac3bf439ec7624f6b237f252`.

> 
> header() {
>     cat /go/src/agones.dev/agones/build/boilerplate.go.txt "$1" | sponge "$1"
> }
> 
> wget -q https://repo1.maven.org/maven2/io/swagger/codegen/v3/swagger-codegen-cli/3.0.51/swagger-codegen-cli-3.0.51.jar -O /tmp/swagger-codegen-cli.jar
> 
> rm -rf /go/src/agones.dev/agones/test/sdk/restapi/swagger /go/src/agones.dev/agones/test/sdk/restapi/alpha/swagger /go/src/agones.dev/agones/test/sdk/restapi/beta/swagger
> java -jar /tmp/swagger-codegen-cli.jar generate -i /go/src/agones.dev/agones/sdks/swagger/sdk.swagger.json  -l go -o /go/src/agones.dev/agones/test/sdk/restapi/swagger
> java -jar /tmp/swagger-codegen-cli.jar generate -i /go/src/agones.dev/agones/sdks/swagger/alpha.swagger.json  -l go -o /go/src/agones.dev/agones/test/sdk/restapi/alpha/swagger
> java -jar /tmp/swagger-codegen-cli.jar generate -i /go/src/agones.dev/agones/sdks/swagger/beta.swagger.json  -l go -o /go/src/agones.dev/agones/test/sdk/restapi/beta/swagger
> 
> 
> # remove un-used files
> rm -rf /go/src/agones.dev/agones/test/sdk/restapi/swagger/.*


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2e54bc2242735209"></a>


### File `build/build-sdk-images/restapi/sdktest.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f652e25c105e47c022d2baef5cb13dad36b70143`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/restapi/sdktest.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e6cb8a53da1d225f86.** [agones/build/build-sdk-images/restapi/sdktest.sh, lines 16–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/restapi/sdktest.sh#L16-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c307f9745c33918d618009a4b3ce919d286b7cf59a186d01edbc8cebdc852c73`.

> 
> set -ex
> GO111MODULE=on
> cd /go/src/agones.dev/agones/test/sdk/restapi
> go run http-api-test.go
> cd /go/src/agones.dev/agones/test/sdk/websocket-watch
> go run ws-watch-test.go


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images/rust` -> module `build::build-sdk-images::rust`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-6c408b400d9c9cf2"></a>


### File `build/build-sdk-images/rust/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1852df2b96a89f2d6220b9dbcce7b22efc0c553b`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/rust/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c450d6c3c97ff9cdd4.** [agones/build/build-sdk-images/rust/Dockerfile, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/rust/Dockerfile#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `01a6e55a9d6930f8d9dbe131d8231f0a0a92f7ebc078a86288eb6ea7c3b957a1`.

> ARG BASE_IMAGE=agones-build-sdk-base:latest
> FROM $BASE_IMAGE
> 
> RUN apt-get --allow-releaseinfo-change update && \
>     apt-get install -y wget && \
>     apt-get clean
> 
> # install rust
> ENV RUSTUP_HOME=/usr/local/rustup \
>     CARGO_HOME=/usr/local/cargo \
>     PATH=/usr/local/cargo/bin:$PATH \
>     RUST_VERSION=1.90.0
> RUN ARCH=$(dpkg --print-architecture) && \
>     case "${ARCH}" in \
>         amd64) RUSTUP_ARCH=x86_64-unknown-linux-gnu ;; \


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b9e758106f82a475"></a>


### File `build/build-sdk-images/rust/build-sdk-test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7893ec06196f1a04f8152bb6602463d50b243ef4`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/rust/build-sdk-test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-73afc003c979e6394d.** [agones/build/build-sdk-images/rust/build-sdk-test.sh, lines 16–25](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/rust/build-sdk-test.sh#L16-L25). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `dd246b89252ba2b8af4cec25aac95c655363eecd3580d15fefabb660900b751b`.

> 
> set -ex
> 
> mkdir -p /go/src/agones.dev/agones/test/sdk/rust/.cargo
> mkdir -p /go/src/agones.dev/agones/test/sdk/rust/.cargo-targets
> cd /go/src/agones.dev/agones/test/sdk/rust
> export CARGO_HOME=/go/src/agones.dev/agones/test/sdk/rust/.cargo
> export CARGO_TARGET_DIR=/go/src/agones.dev/agones/test/sdk/rust/.cargo-targets
> cargo fetch
> cargo build


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-83839f55e1b0b154"></a>


### File `build/build-sdk-images/rust/clean.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `41d90529a88c96249f53a69f09bef5197a643d90`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/rust/clean.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-16ea02ef41c161b9a9.** [agones/build/build-sdk-images/rust/clean.sh, lines 16–22](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/rust/clean.sh#L16-L22). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `705b458fed21139997f825bb09d45347948ce8017c1b99a02526ee8c3484cd32`.

> 
> set -ex
> cd /go/src/agones.dev/agones/test/sdk/rust
> cargo clean
> rm -f /go/src/agones.dev/agones/test/sdk/rust/Cargo.lock
> rm -rf /go/src/agones.dev/agones/test/sdk/rust/.cargo-targets
> rm -rf /go/src/agones.dev/agones/test/sdk/rust/.cargo


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-c876baed41564a04"></a>


### File `build/build-sdk-images/rust/gen.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `fa08ce118a73149f67bc5812fc43de40138bcba5`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/rust/gen.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-76dbe5973831b8141d.** [agones/build/build-sdk-images/rust/gen.sh, lines 16–27](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/rust/gen.sh#L16-L27). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e1cd058bd76198ab38356b8eaf13c6b229850ced5815a63cd9023e75e5197e4f`.

> 
> set -ex
> 
> protos=/go/src/agones.dev/agones/proto
> dest=/go/src/agones.dev/agones/sdks/rust
> 
> rm -rf ${dest}/proto
> 
> echo "Copying protobuffers to rust sdk"
> cp -r ${protos} ${dest}
> 
> echo "Rust code is generated at build time"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-79cfa97973b85f37"></a>


### File `build/build-sdk-images/rust/publish.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0bfd7a795e1f2a946f16f716d61d06235722a478`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/rust/publish.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ef2e75c1bb275b6673.** [agones/build/build-sdk-images/rust/publish.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/rust/publish.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9424db5416ff2825379639a5288a667719f9708cd40ed8d69c1d5d7b17500586`.

> 
> set -ex
> 
> cd ./sdks/rust/proto/sdk
> 
> # Authenticate with crates.io
> set +x
> read -rsp 'Crates.io API Token: ' CARGO_REGISTRY_TOKEN
> printf '\n'
> export CARGO_REGISTRY_TOKEN
> set -x
> 
> # Perform a dry run of cargo publish
> dry_run_output=$(cargo publish --dry-run 2>&1)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-2a26a8267c2192e8"></a>


### File `build/build-sdk-images/rust/sdktest.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `802ff4fe377c0c6255653d82a6cb355ad3d5f92b`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/rust/sdktest.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bae46e595c58efa084.** [agones/build/build-sdk-images/rust/sdktest.sh, lines 16–18](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/rust/sdktest.sh#L16-L18). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c24eae0f240a8b7df3998ecbc02d71b205e33a4332f7ecab4f95198351d2604f`.

> 
> set -ex
> /go/src/agones.dev/agones/test/sdk/rust/.cargo-targets/debug/rust-simple


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-057cbd1436910e5e"></a>


### File `build/build-sdk-images/rust/test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ae8df2e1a3506936ce2e8c05c60d1206b53f8b3a`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/rust/test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d9f4169ad78924218a.** [agones/build/build-sdk-images/rust/test.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/rust/test.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b5e3223259b597eb6f2b6add19c859740d0eedd67114abf6949c4dfc919ed3e1`.

> 
> set -ex
> cd ./sdks/rust
> cargo test


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images/tool/base` -> module `build::build-sdk-images::tool::base`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-d82843f0682b22d9"></a>


### File `build/build-sdk-images/tool/base/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `604e8995c4b3aab1c87b72292d0f4e68a32ed564`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/tool/base/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-29c3d2add3db9fc60e.** [agones/build/build-sdk-images/tool/base/Dockerfile, lines 19–33](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/tool/base/Dockerfile#L19-L33). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `61f544dc50fb31d3f5e6eebfeb0e947631836790161b4e978be516f3674d091e`.

> 
> FROM debian:trixie
> 
> RUN apt-get update && apt-get install -y \
>   build-essential autoconf libtool git pkg-config curl \
>   automake libtool curl make g++ unzip moreutils cmake \
>   && apt-get clean
> 
> ARG GRPC_RELEASE_TAG
> # Install grpc locally for reuse across builds.
> RUN git clone --recurse-submodules -b $GRPC_RELEASE_TAG --depth 1 --shallow-submodules https://github.com/grpc/grpc /var/local/git/grpc && \
>     cd /var/local/git/grpc && \
>     mkdir -p cmake/build && \
>     cd cmake/build && \
>     export CMAKE_BUILD_PARALLEL_LEVEL=$(nproc) && \


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b090771ed35a575f"></a>


### File `build/build-sdk-images/tool/base/entrypoint.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `440edd8f784f35329e15624cc82d4c9676ce010e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/tool/base/entrypoint.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-691a2498c352df68cf.** [agones/build/build-sdk-images/tool/base/entrypoint.sh, lines 16–18](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/tool/base/entrypoint.sh#L16-L18). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `92813f64408a8a607628868dea65fb717d0b5a2052a8bcee17f42061bada28bd`.

> 
> set -ex
> /root/$1.sh


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/build-sdk-images/tool/template` -> module `build::build-sdk-images::tool::template`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-43af6ad1f10cdc33"></a>


### File `build/build-sdk-images/tool/template/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5db5be2276464912e8cc4f1ad4762a92342c9e49`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/tool/template/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-64ac519cfcbff3b5a7.** [agones/build/build-sdk-images/tool/template/Dockerfile, lines 14–23](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/tool/template/Dockerfile#L14-L23). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bc892c8161182b771e71bcc3c2055eef07b3efcb4ed73b56089b416e376f575e`.

> ARG BASE_IMAGE=agones-build-sdk-base:latest
> FROM $BASE_IMAGE
> 
> # Add installation step required to generate your SDK.
> # This should contains tooling required to generate GRPC code, build (if distribution is required) 
> # and test your integration.
> 
> # code generation scripts
> COPY *.sh /root/
> RUN chmod +x /root/*.sh


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-316455ee6891798b"></a>


### File `build/build-sdk-images/tool/template/build.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `168293d968b457d5ddb77a1157ff9d66195db99d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/tool/template/build.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0577cf121fab0dba2f.** [agones/build/build-sdk-images/tool/template/build.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/tool/template/build.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `de1a30ef738eb9726babee053715127898960a87e6ae899df667abf021a97956`.

> 
> set -ex
> 
> # Add steps required to build artifacts for you Agones SDK.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-73b7dda5ee65d55c"></a>


### File `build/build-sdk-images/tool/template/gen.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `992756803adfeed99378713de58fb9d5fc9b0bc8`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/tool/template/gen.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-914b6147ab8b2a4b19.** [agones/build/build-sdk-images/tool/template/gen.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/tool/template/gen.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b2284a8aa9962c4aed89f2fe179a8b0d8375890693bcfcc71093936a0fabfd8c`.

> 
> set -ex
> 
> # Add steps required to generate your language specific SDK GRPC code.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-69b269985d63f6d8"></a>


### File `build/build-sdk-images/tool/template/test.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c6fbd2664e728cf8b54bab0e6219dc136c08fd5e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/build-sdk-images/tool/template/test.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8a0a53a6272388efe9.** [agones/build/build-sdk-images/tool/template/test.sh, lines 16–19](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/build-sdk-images/tool/template/test.sh#L16-L19). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `771f50f7fa1107caa83bbca961664a473a25fe2fc71ce712e05053b3e5a6183e`.

> 
> set -ex
> 
> # Add steps required to test your language specific SDK Agones integration.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/docs` -> module `build::docs`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-003a2e69247620a3"></a>


### File `build/docs/building-testing.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8b5a66e1c055b834a528b2082eac36b7bffedafd`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/docs/building-testing.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-341d5904036a338c09.** [agones/build/docs/building-testing.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/docs/building-testing.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `963f6e70efb5f7c99515a769118e3866177e4eaeaf51fcc4365debb2c529a1eb`.

> 
> This guide covers the core workflows for building and testing Agones.
> 
> ## Getting Started
> 
> Make sure you are in the `build` directory to start.
> 
> First, let's test the Agones system code. To do this, run `make test-go`, which will execute all the unit tests for
> the Go codebase (there are other tests, but they can take a long time to run).
> 
> If you haven't run any of Make targets before then this will also create the Docker based build
> image, and then run the tests.
> 
> Building the build image may take a few minutes to download all the dependencies, so feel
> free to make cup of tea or coffee at this point. ☕️


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6134204623382c04"></a>


### File `build/docs/cluster-setup.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c95b6111eddfce190f45b6686ac0780eb98772b9`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/docs/cluster-setup.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-43101c5d8db6980363.** [agones/build/docs/cluster-setup.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/docs/cluster-setup.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0aaa09b7aa9b07eb674dae0f008c0dce0781feeb231aa7ca76984f6b11fc5e0e`.

> 
> This guide covers setting up different types of Kubernetes clusters for Agones development.
> 
> ## Running a Test Google Kubernetes Engine Cluster
> 
> This will setup a test GKE cluster on Google Cloud, with firewall rules set for each of the nodes for ports 7000-8000
> to be open to UDP traffic.
> 
> First step is to create a Google Cloud Project at https://console.cloud.google.com or reuse an existing one.
> 
> The build tools (by default) maintain configuration for gcloud within the `build` folder, to keep
> everything separate (see below for overwriting these config locations). Therefore, once the project has been created,
> we will need to authenticate our gcloud tooling against it. To do that run `make gcloud-init` and fill in the
> prompts as directed.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-822a249bde1599e1"></a>


### File `build/docs/dependencies.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `26df21b40aac8031fb9a2846ea2aaf5a7fe8e260`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/docs/dependencies.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-76f64881ce97bc6925.** [agones/build/docs/dependencies.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/docs/dependencies.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0101bdcd974ca732e6ab5e21b6e9f283e05fc4bd2e7913a1e9dad512eadc6d28`.

> 
> This guide covers dependency management for Agones development.
> 
> ## Dependencies
> 
> This project uses the [go modules](https://github.com/golang/go/wiki/Modules) as its manager. You can see the list of dependencies [here](https://github.com/agones-dev/agones/blob/main/go.mod).
> 
> ### Vendoring
> 
> Agones uses [module vendoring](https://tip.golang.org/cmd/go/#hdr-Modules_and_vendoring) to reliably produce versioned builds with consistent behavior.
> 
> Adding a new dependency to Agones:
> 
> *  `go mod tidy` This will import your new deps into the go.mod file and trim out any removed dependencies.
> *  `go mod vendor` Pulls module code into the vendor directory.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5a2de6d9dbdd7a06"></a>


### File `build/docs/development-workflow.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8747fcb7077df2d70148a7d20bf9f2b404f64b82`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/docs/development-workflow.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c17fd38f178acd2d29.** [agones/build/docs/development-workflow.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/docs/development-workflow.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `abaef57f0d49ea047a285a9d6f73a188178910c56e43dbe3e0037791dfce02bb`.

> 
> This guide covers advanced development patterns, debugging, and workflow optimization for Agones development.
> 
> ## Performance and Profiling Issues
> 
> ### I want to use pprof to profile the controller.
> 
> Run `make build-images GO_BUILD_TAGS=profile` and this will build images with [pprof](https://golang.org/pkg/net/http/pprof/)
> enabled in the controller, which you can then push and install on your cluster.
> 
> To get the pprof ui working, run `make controller-portforward PORT=6060` (or `minikube-controller-portforward PORT=6060` if you are on minikube),
> which will setup the port forwarding to the pprof http endpoint.
> 
> To view CPU profiling, run `make pprof-cpu-web`, which will start the web interface with a CPU usage graph
> on [http://localhost:6061](http://localhost:6061).


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1f33c6ec4975d588"></a>


### File `build/docs/make-reference.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4cc6f7f3efe8d74ceb88026467441aa57597a7d0`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/docs/make-reference.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-6a9514a44e36479bd6.** [agones/build/docs/make-reference.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/docs/make-reference.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fd7fcf83b5ee349d888175a82bc8e120345259b5a3c292b97d442496a051123f`.

> 
> This guide provides comprehensive documentation for all make variables and targets in the Agones build system.
> 
> ## Make Variable Reference
> 
> ### VERSION
> The version of this build. Version defaults to the short hash of the latest commit.
> 
> ### REGISTRY
> The registry that is being used to store docker images. It doesn't have default value and has to be set explicitly.
> 
> ### CHARTS_REGISTRY
> The chart registry that is being used to store helm charts. It doesn't have default value and has to be set explicitly.
> If not set it would use the `GCP_BUCKET_CHARTS`.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bc6c6468485c2cb2"></a>


### File `build/docs/performance-testing.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4f75addf5d1d061c7013f67ceff38cd574d6fca8`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/docs/performance-testing.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-9faea9bd9af2969bc1.** [agones/build/docs/performance-testing.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/docs/performance-testing.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `19effdbffa95772fc9c32ccb490a76eb937f0b59d5ff2fc0eca6381a05cd570c`.

> 
> This guide covers how to set up and run performance tests for Agones.
> 
> ## Running Performance Test
> 
> To be able to run the performance script located in the following path: `agones/build/performance-test.sh` you need to have the following setup:
> 
> ### Install a standard GKE cluster 
> 
> Follow for more details here: https://agones.dev/site/docs/installation/creating-cluster/gke/
> 
> ### Install Agones
> 
> Follow for more details here: https://agones.dev/site/docs/installation/install-agones/helm/


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a5890ea38a9f258a"></a>


### File `build/docs/platform-setup.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d9c5d7fb1f2a410749a73fb982461a7f7b827383`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/docs/platform-setup.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-4d358e76e50235e805.** [agones/build/docs/platform-setup.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/docs/platform-setup.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `050e21d6da09c54df96e68fac20364d1086975d97856cd43d1f116178364cf14`.

> 
> This guide covers setting up your development environment for building Agones on different platforms.
> 
> ## Building on Different Platforms
> 
> ### Linux
> - Install Make, either via `apt install make` or `yum install make` depending on platform.
> - [Install Docker](https://docs.docker.com/engine/installation/) for your Linux platform.
> 
> ### Windows
> Building and developing Agones requires you to use the
> [Windows Subsystem for Linux](https://blogs.msdn.microsoft.com/wsl/)(WSL),
> as this makes it easy to create a (relatively) cross platform development and build system.
> 
> - [Install WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-532cad9c6681cf92"></a>


### File `build/docs/troubleshooting.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e1ecc5a5057b0cc31657de13abaa25110a9a4ee0`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/docs/troubleshooting.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-89b24cdfb5b4ba5ffa.** [agones/build/docs/troubleshooting.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/docs/troubleshooting.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `15615952c1163810838bbbfe0b29c9af341a7ce8536ec9a26937978b75aa8285`.

> 
> This guide covers frequent issues and possible solutions when developing with Agones.
> 
> ## Common Development Issues
> 
> ### $GOPATH/$GOROOT error when building in WSL
> 
> If you get this error when building Agones in WSL (`make build`, `make test` or any other related target):
> 
> ```
> can't load package: package agones.dev/agones/cmd/controller: cannot find package "agones.dev/agones/cmd/controller" in any of:
>        /usr/local/go/src/agones.dev/agones/cmd/controller (from $GOROOT)
>        /go/src/agones.dev/agones/cmd/controller (from $GOPATH)
> ```


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/e2e-image` -> module `build::e2e-image`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-65efd807edd42f17"></a>


### File `build/e2e-image/Dockerfile`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `85a3399470ce436dca2f303093b63374161c5a80`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/e2e-image/Dockerfile`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-43d2e7288c9532c6a8.** [agones/build/e2e-image/Dockerfile, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/e2e-image/Dockerfile#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `71c22c843392da56e130f3ddfa28bc481539c425967e4f5277746e39f8619d42`.

> FROM gcr.io/cloud-builders/gcloud
> 
> RUN apt-get update && \
>     apt-get install -y --no-install-recommends \
>     wget \
>     psmisc \
>     make \
>     gcc \
>     jq \
>     zip && \
>     apt-get clean && \
>     rm -rf /var/lib/apt/lists/*
> 
> # install go
> WORKDIR /usr/local


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e1b426aa47235306"></a>


### File `build/e2e-image/e2e.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8ffdd4c9e47034a3864e399fe96fc62b5506e202`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/e2e-image/e2e.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-dd624469e10037e8a4.** [agones/build/e2e-image/e2e.sh, lines 16–29](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/e2e-image/e2e.sh#L16-L29). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1d66afd1ec6f13ccad74d3eb49cd13f0b62d0f5a3ed185b684f40aed0663a324`.

> 
> FEATURES=$1
> CLOUD_PRODUCT=$2
> REGISTRY=$3
> GS_TEST_IMAGE=$4
> 
> echo $FEATURES
> echo $REGISTRY
> set -e
> echo "installing current release"
> DOCKER_RUN= make install FEATURE_GATES='"'$FEATURES'"' REGISTRY='"'$REGISTRY'"' 
> echo "starting e2e test"
> DOCKER_RUN= make test-e2e ARGS=-parallel=16 E2E_USE_GOTESTSUM=true GOTESTSUM_VERBOSE=true FEATURE_GATES='"'$FEATURES'"' CLOUD_PRODUCT='"'$CLOUD_PRODUCT'"' GS_TEST_IMAGE='"'$GS_TEST_IMAGE'"'
> echo "completed e2e test"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d3b02f0759269074"></a>


### File `build/e2e-image/entrypoint.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9ea638832e0c97f394b9d7183ca3e8c7e0415347`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/e2e-image/entrypoint.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e490498c9ef7b17ef5.** [agones/build/e2e-image/entrypoint.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/e2e-image/entrypoint.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1932ca3c3b835e889609942d1075e0dd10f65d5357d521d68ee43efdaab5950e`.

> set -e
> FEATURES=$1
> CLOUD_PRODUCT=$2
> TEST_CLUSTER_NAME=$3
> TEST_CLUSTER_LOCATION=$4
> REGISTRY=$5
> GS_TEST_IMAGE=$6
> 
> echo $FEATURES
> export SHELL="/bin/bash"
> export KUBECONFIG="/root/.kube/config"
> mkdir -p /go/src/agones.dev/ /root/.kube/
> ln -s /workspace /go/src/agones.dev/agones
> cd /go/src/agones.dev/agones/build
> if [ "$1" = 'local' ]


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/grafana` -> module `build::grafana`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-fe3979954c2efb78"></a>


### File `build/grafana/dashboard-allocations.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `20a777a20663adf610ae87e2957f358337874259`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-allocations.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e49b85c963a9735b9c.** [agones/build/grafana/dashboard-allocations.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-allocations.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a41b540139d07fc108d71ed4381c3df1ec75aa7f2a0452531aaff4d06b03aa05`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-602ab07d46937172"></a>


### File `build/grafana/dashboard-allocator-usage.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a51949cd54e10a6c3c73d9f00d9a1294e0886617`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-allocator-usage.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-32346b52d4257b90ee.** [agones/build/grafana/dashboard-allocator-usage.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-allocator-usage.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `009e0307d322784b4334bf229d68301ad42ea8aaad82775bab2c51ded8dabb11`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6dd1ab449c8b4f58"></a>


### File `build/grafana/dashboard-apiserver-requests.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8564cf6ba7b9eee372d8fd470185c54e399ada3c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-apiserver-requests.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5ce7fd34c1cf3f7c57.** [agones/build/grafana/dashboard-apiserver-requests.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-apiserver-requests.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `432e053e5db16a3e29343778e20d7e93bbfa37043e327b1c5eb5121caa859284`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ac12ec6e33c864d4"></a>


### File `build/grafana/dashboard-autoscalers.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `249bfa5741c2046a140fe0eea800be297db10ff5`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-autoscalers.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6c1aabf69efdebf6de.** [agones/build/grafana/dashboard-autoscalers.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-autoscalers.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `92f703a955d7c87da789659b473810872a6681caa646cc16ad25e839f4338e2f`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-62817596229f8a1f"></a>


### File `build/grafana/dashboard-controller-usage.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f2e90e0dbd5105f85260c4eabea880017c22be61`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-controller-usage.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d725ac6623d7a60c0b.** [agones/build/grafana/dashboard-controller-usage.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-controller-usage.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a288d80d80bb827351a7d5b67f8135280936c8ab3ecf02c177b1c77293dbcefa`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-44049c004e1ea49e"></a>


### File `build/grafana/dashboard-extensions-status.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2b9ef6c16cb1219ffdc08320963d097f91e1fbab`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-extensions-status.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-98ac905d58641ef4ac.** [agones/build/grafana/dashboard-extensions-status.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-extensions-status.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1be696a5fe51542bcc14bcd27c2ee3e1ef64e02063a6bb75c60bedd2c197c428`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6d686c4aef2fa572"></a>


### File `build/grafana/dashboard-extensions-usage.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `96de405448c38cb22ec6eb4ab611acee680aeb17`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-extensions-usage.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9204b2133201be3718.** [agones/build/grafana/dashboard-extensions-usage.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-extensions-usage.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0a0109c902cc123fe62702944b9cf30483586b8f00889912eb267f6c2e51710d`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-378cb7c61000e8ed"></a>


### File `build/grafana/dashboard-gameservers.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b1319b404bb65cef92c7aaec40849ce7e96ccd8b`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-gameservers.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f4202999fd8476d716.** [agones/build/grafana/dashboard-gameservers.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-gameservers.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `69755a501df400042bac2b5e50a0f37bdef72c7fb5216a62fc27823cbd11d87b`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-df44e6b2cb47cdaa"></a>


### File `build/grafana/dashboard-goclient-requests.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8f8a4f56c222b8e405f3bed5462d1271560c0a51`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-goclient-requests.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2efdf1d529f7a85000.** [agones/build/grafana/dashboard-goclient-requests.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-goclient-requests.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f64c62c7ce805581b4fa853784555b25e1fa54221a7a6a0f20944450b2893507`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-98ef10f9cccb3487"></a>


### File `build/grafana/dashboard-goclient-workqueues.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `679aab02c6a21a28569a57447d9d956b6f0dd5eb`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-goclient-workqueues.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c6eea91db08dee0256.** [agones/build/grafana/dashboard-goclient-workqueues.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-goclient-workqueues.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `59bc137662eb2a7f7380ca946f2d473b20e8b9fbfd071635914efa960c56a0a3`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-baeb269ee23f278d"></a>


### File `build/grafana/dashboard-status.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `050f2f3f9de667664bcc5a6f64ecf6b320ba6e3f`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/grafana/dashboard-status.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-343dd88974805127a9.** [agones/build/grafana/dashboard-status.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/grafana/dashboard-status.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0770d1527d9f3336e2b9807fe04030daf80225717769ca47244059e6397d09ba`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/includes` -> module `build::includes`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-dd8d1c0475512593"></a>


### File `build/includes/allocation.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ad67a991ef9a5bf5f251c7dc9b0737f56fbe650c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/allocation.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9a5ba7f7d50126ff8c.** [agones/build/includes/allocation.mk, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/allocation.mk#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fe7ebeaa8b6801329411d822be2b8d4168fb411e7e4262327678ba24a4aea13f`.

> allocation_build_folder = build-allocation-images/
> sdk_build_folder = build-sdk-images
> build_allocation_version = $(call sha_dir,$(build_path)/$(allocation_build_folder)/$(ALLOCATION_FOLDER)/*)
> build_allocation_prefix = agones-build-allocation-
> ALLOCATION_FOLDER ?= go
> ALLOCATION_IMAGE_TAG=$(build_allocation_prefix)$(ALLOCATION_FOLDER):$(build_allocation_version)
> 
> .PHONY: gen-allocation-grpc 
> 
> # create the build image allocation if it doesn't exist
> ensure-build-allocation-image:
> 	$(MAKE) ensure-image IMAGE_TAG=$(ALLOCATION_IMAGE_TAG) BUILD_TARGET=build-build-allocation-image ALLOCATION_FOLDER=$(ALLOCATION_FOLDER)
> 
> # Builds the docker image
> # Note: allocation and sdk use the same dockerfile


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b090705e2dbeb5be"></a>


### File `build/includes/build-image.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a47cc25ab6f98132adb33ddd1651d0b05c45d931`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/build-image.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-da627e52c1733a2040.** [agones/build/includes/build-image.mk, lines 21–35](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/build-image.mk#L21-L35). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `36434e04854fdcdffd7353a1de591d049daabea1c07f4ff09cd3c2697884e76c`.

> 
> build_remote_tag = $(REGISTRY)/$(build_tag)
> 
> # Creates the build docker image
> build-build-image:
> 	docker build --tag=$(build_tag) $(build_path)/build-image $(DOCKER_BUILD_ARGS)
> 
> # Deletes the local build docker image
> clean-build-image:
> 	docker rmi $(build_tag)
> 
> ensure-arm-builder:
> 	 docker run --privileged --rm tonistiigi/binfmt:qemu-v6.2.0 --install arm64
> 	 
> ensure-build-config:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-644dc95f68d9a87c"></a>


### File `build/includes/examples.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5bb2b5a7d52d0188efde60c8e320691b839627ae`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/examples.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e84bb37e9aa7d72f4a.** [agones/build/includes/examples.mk, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/examples.mk#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9e121a807956e95cf46eb8a0144f6e04c9fae65c7539a4a96569a94ffda1d12e`.

> test-examples-on-gar: example-image-test.allocation-endpoint
> test-examples-on-gar: example-image-test.autoscaler-webhook
> test-examples-on-gar: example-image-test.cpp-simple
> test-examples-on-gar: example-image-test.nodejs-simple
> test-examples-on-gar: example-image-test.rust-simple
> test-examples-on-gar: example-image-test.unity-simple
> test-examples-on-gar: example-image-test.xonotic
> test-examples-on-gar: example-image-test.crd-client
> test-examples-on-gar: example-image-test.supertuxkart
> test-examples-on-gar: example-image-test.simple-game-server
> test-examples-on-gar: example-image-test.simple-genai-server
> test-examples-on-gar: example-image-test.custom-controller
> 
> push-example-golang-images: example-image-push.allocation-endpoint
> push-example-golang-images: example-image-push.autoscaler-webhook


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b430704fbbb02049"></a>


### File `build/includes/google-cloud.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `dac8684ab2edbddf762dfd87b5e6f12813dae591`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/google-cloud.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-72f9044a467af7a02d.** [agones/build/includes/google-cloud.mk, lines 23–37](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/google-cloud.mk#L23-L37). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `cfea19279f58dc6bc4824d57921c021cf547f2398c76cafc2588cf1fcfd520e5`.

> gcloud-init: ensure-build-config
> 	docker run --rm -it $(common_mounts) $(build_tag) gcloud init
> 
> # Creates and authenticates a small, 6 node GKE cluster to work against (2 nodes are used for agones-metrics and agones-system)
> gcloud-test-cluster: GCP_CLUSTER_NODEPOOL_INITIALNODECOUNT ?= 4
> gcloud-test-cluster: GCP_CLUSTER_NODEPOOL_MACHINETYPE ?= e2-standard-4
> gcloud-test-cluster: GCP_CLUSTER_NODEPOOL_ENABLEIMAGESTREAMING ?= true
> gcloud-test-cluster: GCP_CLUSTER_NODEPOOL_AUTOSCALE ?= false
> gcloud-test-cluster: GCP_CLUSTER_NODEPOOL_MIN_NODECOUNT ?= 1
> gcloud-test-cluster: GCP_CLUSTER_NODEPOOL_MAX_NODECOUNT ?= 5
> gcloud-test-cluster: GCP_CLUSTER_NODEPOOL_WINDOWSINITIALNODECOUNT ?= 0
> gcloud-test-cluster: GCP_CLUSTER_NODEPOOL_WINDOWSMACHINETYPE ?= e2-standard-4
> gcloud-test-cluster: GCP_CLUSTER_NODEPOOL_ENABLE_AGONES_METRICS_NODEPOOL ?= true
> gcloud-test-cluster: $(ensure-build-image)
> 	$(MAKE) gcloud-terraform-cluster GCP_TF_CLUSTER_NAME="$(GCP_CLUSTER_NAME)" \


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9377e62ee0e8a062"></a>


### File `build/includes/kind.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `253f6472f235d1b5ed4d27a749deef73ee19ab6c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/kind.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-083679a84d573b349d.** [agones/build/includes/kind.mk, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/kind.mk#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `1b810c0eb639e767bb9ef33f11d2d7c4845983cf3dc6a70e353690bebbe971ee`.

> kind-test-cluster: DOCKER_RUN_ARGS+=--network=host
> kind-test-cluster: $(ensure-build-image)
> 	@if [ -z $$(kind get clusters | grep $(KIND_PROFILE)) ]; then\
> 		echo "Could not find $(KIND_PROFILE) cluster. Creating...";\
> 		kind create cluster --name $(KIND_PROFILE) --image kindest/node:v1.35.6 --wait 5m;\
> 	fi
> 
> # kind-metallb-helm-install installs metallb via helm for kind
> kind-metallb-helm-install:
> 	helm repo add metallb https://metallb.github.io/metallb
> 	helm repo update
> 	helm upgrade metallb metallb/metallb --install --namespace metallb-system --create-namespace --version 0.15.2 --wait --timeout 180s
> 
> # kind-metallb-configure configures metallb with an ip address range based on the kind node IP
> kind-metallb-configure:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1f3f74ede5079cd4"></a>


### File `build/includes/linux.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6a355027e7fc794d3cb886001b4a99378bf1c14c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/linux.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7fe311831fe79e502e.** [agones/build/includes/linux.mk, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/linux.mk#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d2254236910e3d2b84cf285bc4ec2e5d43692d3a8967e285fe96079416a35edd`.

> sha = $(shell sha256sum $(1) | head -c 10)
> 
> # Get the sha of all files in a directory using wildcard in $(1)
> sha_dir = $(shell sha256sum  $(1) | cut -d' ' -f1 | sha256sum | head -c 10 )
> 
> # Minikube executable
> MINIKUBE ?= minikube
> 
> #   _____                    _
> #  |_   _|_ _ _ __ __ _  ___| |_ ___
> #    | |/ _` | '__/ _` |/ _ \ __/ __|
> #    | | (_| | | | (_| |  __/ |_\__ \
> #    |_|\__,_|_|  \__, |\___|\__|___/
> #                 |___/


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d1e0af18290e8e05"></a>


### File `build/includes/macos.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0d9a8504092aa733abe646171c945a2043e7dfee`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/macos.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-620c263c3d4a0c683b.** [agones/build/includes/macos.mk, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/macos.mk#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f18e50cae3e841f88be70944aa183cec31afc821f7282420457a24353ebae461`.

> sha = $(shell shasum -a 256 $(1) | head -c 10)
> 
> # Get the sha of all files in a directory using wildcard in $(1)
> sha_dir = $(shell shasum -a 256 $(1) | cut -d' ' -f1 | shasum -a 256 | head -c 10 )
> 
> # Minikube executable
> MINIKUBE ?= minikube
> 
> #   _____                    _
> #  |_   _|_ _ _ __ __ _  ___| |_ ___
> #    | |/ _` | '__/ _` |/ _ \ __/ __|
> #    | | (_| | | | (_| |  __/ |_\__ \
> #    |_|\__,_|_|  \__, |\___|\__|___/
> #                 |___/


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-e9da013dc95bc08d"></a>


### File `build/includes/minikube.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `77ebb8e89947fe90e427746d9b107dd48b636705`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/minikube.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b050ff57725be229fa.** [agones/build/includes/minikube.mk, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/minikube.mk#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9b316946a18a5d42b3d8f3181aeccc18f85404427559b32234b563721d3609e5`.

> 
> MINIKUBE_DRIVER ?= docker
> MINIKUBE_NODES ?= 1
> 
> MINIKUBE_DEBUG_SDK_NAMESPACE ?= default
> MINIKUBE_DEBUG_CONTROLLER_PORT ?= 2346
> MINIKUBE_DEBUG_EXTENSIONS_PORT ?= 2347
> MINIKUBE_DEBUG_PING_PORT ?= 2348
> MINIKUBE_DEBUG_ALLOCATOR_PORT ?= 2349
> MINIKUBE_DEBUG_PROCESSOR_PORT ?= 2350
> MINIKUBE_DEBUG_SDK_PORT ?= 2351
> 
> # minikube shell mount for certificates
> minikube_cert_mount := ~/.minikube:$(HOME)/.minikube


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-41347d943480085e"></a>


### File `build/includes/release.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2af98c9d3618c5b5ae25a268286e742c09a07b1e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/release.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-de268bf98cabfebf13.** [agones/build/includes/release.mk, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/release.mk#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `686af4de3df4debb6eeb6e8ffdb04543aebaa9167bb532bcba2da654e7c14633`.

> release_registry = us-docker.pkg.dev/agones-images/release
> 
> # outputs the markdown for the example images section of the release template
> release-example-image-markdown: example-image-markdown.allocation-endpoint
> release-example-image-markdown: example-image-markdown.autoscaler-webhook
> release-example-image-markdown: example-image-markdown.cpp-simple
> release-example-image-markdown: example-image-markdown.crd-client
> release-example-image-markdown: example-image-markdown.nodejs-simple
> release-example-image-markdown: example-image-markdown.rust-simple
> release-example-image-markdown: example-image-markdown.simple-game-server
> release-example-image-markdown: example-image-markdown.supertuxkart
> release-example-image-markdown: example-image-markdown.unity-simple
> release-example-image-markdown: example-image-markdown.xonotic
> 
> example-image-markdown.%:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-42a88615b0f60ced"></a>


### File `build/includes/sdk.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0f67ec8a76de43aaeedc2052ccc868c94d075896`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/sdk.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-17121039629563a37a.** [agones/build/includes/sdk.mk, lines 22–36](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/sdk.mk#L22-L36). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `51ede223a67445f461df1cd92a96ab2577e72456a96645f687b66f09dd99f05d`.

> 
> grpc_release_tag = v1.83.0
> 
> build_sdk_base_version = $(call sha,$(build_path)/build-sdk-images/tool/base/Dockerfile)_$(grpc_release_tag)
> build_sdk_base_tag = agones-build-sdk-base:$(build_sdk_base_version)
> 
> # Calculate sha hash of sha hashes of all files in a specified SDK_FOLDER
> build_sdk_version = $(call sha_dir,$(build_path)/build-sdk-images/$(SDK_FOLDER)/*)
> build_sdk_base_remote_tag = $(REGISTRY)/$(build_sdk_base_tag)
> build_sdk_prefix = agones-build-sdk-
> sdk_build_folder = build-sdk-images/
> examples_folder = ../examples/
> SDK_FOLDER ?= go
> COMMAND ?= gen
> SDK_IMAGE_TAG=$(build_sdk_prefix)$(SDK_FOLDER):$(build_sdk_version)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d15b034f39876904"></a>


### File `build/includes/website.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d1040d3e984c037b4d23c4b8e82c6dbc53f9428a`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/website.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-87cf797f2bce2a10e6.** [agones/build/includes/website.mk, lines 29–43](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/website.mk#L29-L43). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6a9e279006f2813e9b6a6b0a154b859d259379592bada960bf9a48f10cc96466`.

> site-deps: ensure-build-image
> 	docker run --rm $(common_mounts) --workdir=$(mount_path)/site $(DOCKER_RUN_ARGS) $(build_tag) bash -c \
> 	"$(git_safe) && hugo mod npm pack && npm install"
> 
> # generate the latest website
> site-server: ARGS ?=-F
> site-server: ENV ?= RELEASE_VERSION="$(base_version)" RELEASE_BRANCH=main
> site-server: site-deps
> 	docker run --rm $(common_mounts) --workdir=$(mount_path)/site $(DOCKER_RUN_ARGS) -p 1313:1313 $(build_tag) bash -c \
> 	"$(git_safe) && $(ENV) hugo server --watch --baseURL=http://localhost:1313/ --bind=0.0.0.0 $(ARGS)"
> 
> site-static: site-deps
> 	-docker run --rm $(common_mounts) --workdir=$(mount_path)/site $(DOCKER_RUN_ARGS) $(build_tag) rm -r ./public
> 	-mkdir $(agones_path)/site/public
> 	docker run --rm $(common_mounts) --workdir=$(mount_path)/site $(DOCKER_RUN_ARGS) $(build_tag) bash -c \


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d2ae75155a8b9b85"></a>


### File `build/includes/windows.mk`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `129ce0a5ba8b9b5f62ec3db81999028fc4cd44c6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/includes/windows.mk`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-419f2b0d391c502f85.** [agones/build/includes/windows.mk, lines 27–41](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/includes/windows.mk#L27-L41). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `81f2bb2fa5817478359acd673f3e9614974bf53e9f7ecdfc649d8ed3f73f6583`.

> sha = $(shell sha256sum $(1) | head -c 10)
> 
> # Get the sha of all files in a directory using wildcard in $(1)
> sha_dir = $(shell sha256sum  $(1) | cut -d' ' -f1 | sha256sum | head -c 10 )
> 
> # Minikube executable
> MINIKUBE ?= minikube.exe
> 
> #   _____                    _
> #  |_   _|_ _ _ __ __ _  ___| |_ ___
> #    | |/ _` | '__/ _` |/ _ \ __/ __|
> #    | | (_| | | | (_| |  __/ |_\__ \
> #    |_|\__,_|_|  \__, |\___|\__|___/
> #                 |___/


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/local-includes` -> module `build::local-includes`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-fccac5f2223fa577"></a>


### File `build/local-includes/README.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7a4bd65b56f4ad3756527e47ac8ab3109caa8022`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/local-includes/README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-d0e88bef115535ccb2.** [agones/build/local-includes/README.md, lines 2–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/local-includes/README.md#L2-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c111b473a7f5c64200ff8dd91138f8051a2986ef2e95c66d9a395bbc9a9a16b7`.

> 
> You can drop files in this folder to override default build variables.
> 
> For instance if you're behind a corporate proxy you could do this :
> 
> ```
> cat << 'EOF' > docker-build-args.mk 
> DOCKER_BUILD_ARGS = --build-arg HTTP_PROXY=$(HTTP_PROXY) --build-arg HTTPS_PROXY=$(HTTPS_PROXY) \
>                     --build-arg NO_PROXY=$(NO_PROXY) --build-arg http_proxy=$(HTTP_PROXY) \
>                     --build-arg https_proxy=$(HTTPS_PROXY) --build-arg no_proxy=$(NO_PROXY)
> EOF
> ```
> 
> And all `docker build` commands in the makefile will use your proxy.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/release` -> module `build::release`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-03a5480d032052de"></a>


### File `build/release/post_cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `bd2188cb4e674b81cad16966f5f4334ad4109ac9`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/release/post_cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4d5744816433cf8f56.** [agones/build/release/post_cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/release/post_cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `84d75827d91616d875f0c56db7349f3759b3b63586df3ffd7f476c09fc6523d7`.

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
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-89595a2000aef872"></a>


### File `build/release/pre_cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `10bfe56b0992fd4284b94880c8cb6e44d34c00ea`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/release/pre_cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-67d1a09dbf85fb8717.** [agones/build/release/pre_cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/release/pre_cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7ec026a84c02249bf4e5d4ffe6ac75140854d71a7ffa3dc45a564200403006d6`.

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
> steps:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/report` -> module `build::report`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ea52f5156048413f"></a>


### File `build/report/cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `dad48c2ca8fc59c9e6730c6bce553cfe6eeaf6ce`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/report/cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-bfdab84605c31c68e8.** [agones/build/report/cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/report/cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `72a45c370fd7c1d732b4e7657182e03e06b01cef3625699e9497703ac732e602`.

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

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/scripts/example-version-checker` -> module `build::scripts::example-version-checker`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-2990fc02e66b51f3"></a>


### File `build/scripts/example-version-checker/go.mod`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `70e5c9133c44dbe65be0ab58078daf5d68dfbd0e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/scripts/example-version-checker/go.mod`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6e1a09d0c91cd7b6e2.** [agones/build/scripts/example-version-checker/go.mod, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/example-version-checker/go.mod#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4b9cb3ac40a70bc6c6f46d1abf1936e23014cbc9dfc5a826da494c7bc00fab6a`.

> module github.com/agones/agones/build/scripts/example-version-checker
> 
> go 1.26.5
> 
> require github.com/go-git/go-git/v5 v5.19.2
> 
> require (
> 	dario.cat/mergo v1.0.2 // indirect
> 	github.com/Microsoft/go-winio v0.6.2 // indirect
> 	github.com/ProtonMail/go-crypto v1.4.1 // indirect
> 	github.com/cloudflare/circl v1.6.3 // indirect
> 	github.com/cyphar/filepath-securejoin v0.6.1 // indirect
> 	github.com/emirpasic/gods v1.18.1 // indirect
> 	github.com/go-git/gcfg v1.5.1-0.20230307220236-3a3c6141e376 // indirect
> 	github.com/go-git/go-billy/v5 v5.9.0 // indirect


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-250accdaea8fdd18"></a>


### File `build/scripts/example-version-checker/go.sum`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6be2062345eb40b916895c831fb09fc01574d53e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/scripts/example-version-checker/go.sum`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b4cfd51e794e0aee28.** [agones/build/scripts/example-version-checker/go.sum, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/example-version-checker/go.sum#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `40c14b50eb8ea68b7f97865b366e8cb6dfc431ab685270b904424feddb3a3b3e`.

> dario.cat/mergo v1.0.2 h1:85+piFYR1tMbRrLcDwR18y4UKJ3aH1Tbzi24VRW1TK8=
> dario.cat/mergo v1.0.2/go.mod h1:E/hbnu0NxMFBjpMIE34DRGLWqDy0g5FuKDhCb31ngxA=
> github.com/Microsoft/go-winio v0.5.2/go.mod h1:WpS1mjBmmwHBEWmogvA2mj8546UReBk4v8QkMxJ6pZY=
> github.com/Microsoft/go-winio v0.6.2 h1:F2VQgta7ecxGYO8k3ZZz3RS8fVIXVxONVUPlNERoyfY=
> github.com/Microsoft/go-winio v0.6.2/go.mod h1:yd8OoFMLzJbo9gZq8j5qaps8bJ9aShtEA8Ipt1oGCvU=
> github.com/ProtonMail/go-crypto v1.4.1 h1:9RfcZHqEQUvP8RzecWEUafnZVtEvrBVL9BiF67IQOfM=
> github.com/ProtonMail/go-crypto v1.4.1/go.mod h1:e1OaTyu5SYVrO9gKOEhTc+5UcXtTUa+P3uLudwcgPqo=
> github.com/anmitsu/go-shlex v0.0.0-20200514113438-38f4b401e2be h1:9AeTilPcZAjCFIImctFaOjnTIavg87rW78vTPkQqLI8=
> github.com/anmitsu/go-shlex v0.0.0-20200514113438-38f4b401e2be/go.mod h1:ySMOLuWl6zY27l47sB3qLNK6tF2fkHG55UZxx8oIVo4=
> github.com/armon/go-socks5 v0.0.0-20160902184237-e75332964ef5 h1:0CwZNZbxp69SHPdPJAN/hZIm0C4OItdklCFmMRWYpio=
> github.com/armon/go-socks5 v0.0.0-20160902184237-e75332964ef5/go.mod h1:wHh0iHkYZB8zMSxRWpUBQtwG5a7fFgvEO+odwuTv2gs=
> github.com/cloudflare/circl v1.6.3 h1:9GPOhQGF9MCYUeXyMYlqTR6a5gTrgR/fBLXvUgtVcg8=
> github.com/cloudflare/circl v1.6.3/go.mod h1:2eXP6Qfat4O/Yhh8BznvKnJ+uzEoTQ6jVKJRn81BiS4=
> github.com/cyphar/filepath-securejoin v0.6.1 h1:5CeZ1jPXEiYt3+Z6zqprSAgSWiggmpVyciv8syjIpVE=
> github.com/cyphar/filepath-securejoin v0.6.1/go.mod h1:A8hd4EnAeyujCJRrICiOWqjS1AX0a9kM5XL+NwKoYSc=


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/scripts/k8s-export-openapi` -> module `build::scripts::k8s-export-openapi`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-86f14cfe7166c8de"></a>


### File `build/scripts/k8s-export-openapi/go.mod`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1c79105908833d650ed02e8e2458a26e71e114e6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/scripts/k8s-export-openapi/go.mod`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-61ad27ae8182765508.** [agones/build/scripts/k8s-export-openapi/go.mod, lines 1–5](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/k8s-export-openapi/go.mod#L1-L5). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `96128affab8763e1a34ab5cc4fdabd9db99f9c35c848a6b13639656fc75e0e71`.

> module github.com/agones/agones/build/scripts/k8s-export-openapi
> 
> go 1.26.5
> 
> require github.com/itchyny/json2yaml v0.1.5


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d5b3ec8e4ce1b07d"></a>


### File `build/scripts/k8s-export-openapi/go.sum`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ddf81dd5b371a2f99a42f0ecac3e347cd7640255`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/scripts/k8s-export-openapi/go.sum`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6e7580ca07e2239f05.** [agones/build/scripts/k8s-export-openapi/go.sum, lines 1–2](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/k8s-export-openapi/go.sum#L1-L2). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8a0c3aa93588fca363cd7360dceececba2486cd872392351bbc2d5893b0cc258`.

> github.com/itchyny/json2yaml v0.1.5 h1:wKaepxwivjcn2ssiLu7v4KrLKSIILWtwD/8L054tTdI=
> github.com/itchyny/json2yaml v0.1.5/go.mod h1:lBQ0JI7HugKRQy/c5/U3KRymvBNub/HTbSXHS9/4018=


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/scripts/previousversion` -> module `build::scripts::previousversion`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-55e76aec00a446b4"></a>


### File `build/scripts/previousversion/go.mod`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c1ebd4eac7f677877f55521ee22253937d280c05`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/scripts/previousversion/go.mod`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5e86085c4b729a8a49.** [agones/build/scripts/previousversion/go.mod, lines 1–5](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/previousversion/go.mod#L1-L5). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ffeb1aecd94c715d5504e7c5f001cad4fd1d64d29590b8bfd6b2e228add45dbd`.

> module github.com/agones/agones/build/scripts/previousversion
> 
> go 1.26.5
> 
> require github.com/blang/semver/v4 v4.0.0


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b1e60a3afdc9cdce"></a>


### File `build/scripts/previousversion/go.sum`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `eae081784acc9a27c0ef0daa0170a2d1400cbaa6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/scripts/previousversion/go.sum`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f5001022d1b0546a90.** [agones/build/scripts/previousversion/go.sum, lines 1–2](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/previousversion/go.sum#L1-L2). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d98bfab480bd64e18afe1897ae5338133f4cde7e7c46f7797751b36cc547443b`.

> github.com/blang/semver/v4 v4.0.0 h1:1PFHFE6yCCTv8C1TeyNNarDzntLi7wMI5i/pzqYIsAM=
> github.com/blang/semver/v4 v4.0.0/go.mod h1:IbckMUScFkM3pff0VJDNKRiT6TG/YpiHIM2yvyW5YoQ=


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/scripts/site-config-update-version` -> module `build::scripts::site-config-update-version`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-1eda326593755694"></a>


### File `build/scripts/site-config-update-version/go.mod`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `576227702074af1f131e22429546ed7c9d997e46`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/scripts/site-config-update-version/go.mod`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fc966e00b04b286866.** [agones/build/scripts/site-config-update-version/go.mod, lines 1–5](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/site-config-update-version/go.mod#L1-L5). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a70821a6885c5e97f86524ebd331c200d7750127e65b06dafd6486bca99f53b1`.

> module github.com/agones/agones/build/scripts/site-config-update-version
> 
> go 1.26.5
> 
> require github.com/pelletier/go-toml v1.9.5


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ebac077d2924a953"></a>


### File `build/scripts/site-config-update-version/go.sum`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `02b71dfc0dcd8cff1e77b238420ccb9b0d155917`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/scripts/site-config-update-version/go.sum`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6c6cbd4738b7dddc45.** [agones/build/scripts/site-config-update-version/go.sum, lines 1–2](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/scripts/site-config-update-version/go.sum#L1-L2). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c79e16a646550bb1ac95c6e4311dd9a50aeb4a2799bbd0a95a5cdc1930ac90a1`.

> github.com/pelletier/go-toml v1.9.5 h1:4yBQzkHv+7BHq2PQUZF3Mx0IYxG7LsP222s7Agd3ve8=
> github.com/pelletier/go-toml v1.9.5/go.mod h1:u1nR/EPcESfeI/szUZKdtJ0xRNbUoANCkoOuaOx1Y+c=


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/e2e` -> module `build::terraform::e2e`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ee6c8ae1c4e1f17c"></a>


### File `build/terraform/e2e/module.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ad28506a47afa630db7e28ca38f9b5e9d08f3f64`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/e2e/module.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-6e448956553375e949.** [agones/build/terraform/e2e/module.tf, lines 8–27](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/e2e/module.tf#L8-L27). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `af3b5aae098c0203bc2777ad61545efd26a61ac61aa4b01912771df92d79f566`.

> //
> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform init -backend-config="bucket=<YOUR_GCP_ProjectID>-e2e-infra-bucket-tfstate" -backend-config="prefix=terraform/state"
> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>     helm = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/e2e/gke-autopilot` -> module `build::terraform::e2e::gke-autopilot`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-d64141ebb65741a8"></a>


### File `build/terraform/e2e/gke-autopilot/module.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4f1ef4a1ed528af983fbe435bac800c304705abe`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/e2e/gke-autopilot/module.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-3bd4ccfa64d6ec8541.** [agones/build/terraform/e2e/gke-autopilot/module.tf, lines 7–26](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/e2e/gke-autopilot/module.tf#L7-L26). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `79090bd0d1bb490c9cebc3b1ead5cebbd9ca358e5e684b35d36b36fdc75ace04`.

> //     http://www.apache.org/licenses/LICENSE-2.0
> //
> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source  = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>     helm = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/e2e/gke-standard` -> module `build::terraform::e2e::gke-standard`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-15c25e48e25fd0f7"></a>


### File `build/terraform/e2e/gke-standard/module.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `67ce4ce63048f79ac9f77d6d4409e81ec87ea936`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/e2e/gke-standard/module.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-bd50e43a72f86e964b.** [agones/build/terraform/e2e/gke-standard/module.tf, lines 7–26](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/e2e/gke-standard/module.tf#L7-L26). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `efa53f33c1f8cc59bceddb0d35d9360aecbdaa5ff16284b7fcfd5e6ace3f638f`.

> //     http://www.apache.org/licenses/LICENSE-2.0
> //
> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source  = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>     helm = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/e2e/state-bucket` -> module `build::terraform::e2e::state-bucket`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-e5a75d0dd46d40c0"></a>


### File `build/terraform/e2e/state-bucket/main.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b311a809530ae57b39069f68e695434968604e16`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/e2e/state-bucket/main.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a8eead7a0929701548.** [agones/build/terraform/e2e/state-bucket/main.tf, lines 17–36](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/e2e/state-bucket/main.tf#L17-L36). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c3fef3b66679c076982e1eefe2e34679ad25c2a94fafb3250e27cde668df2992`.

> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> 
> // If you are getting `Error: googleapi: Error 409: Your previous request to create the named bucket
> // succeeded and you already own it., conflict` this means that your local tfstate file has
> // divereged from the tfstate file in Google Cloud Storage (GCS). To use the GCS version of the
> // tfstate, delete your local .terraform and .tfstate files. You may need to run
> // `sudo chown -R yourusername .` to be able to delete them. Then navigate to this directory and run
> // `terraform init`. Pull in the tfstate file from gcloud with
> // `terraform import google_storage_bucket.default "<YOUR_GCP_ProjectID>"-e2e-infra-bucket-tfstate`.
> 
> // # GCS bucket for holding the Terraform state of the e2e Terraform config.
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source  = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/gke` -> module `build::terraform::gke`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-db884305b6a99ead"></a>


### File `build/terraform/gke/module.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `68eb9bd4aa5f7863a3796f8167851a9ce5438933`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/gke/module.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f9473bc6d9a6f38c9b.** [agones/build/terraform/gke/module.tf, lines 7–26](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/gke/module.tf#L7-L26). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `67df16c6b9d309fb04b3e8b9aceaa3793f51418cb7ca96aac620a42368457eab`.

> //     http://www.apache.org/licenses/LICENSE-2.0
> //
> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform apply -var project="<YOUR_GCP_ProjectID>" [-var agones_version="1.17.0"]
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source  = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/gke-autopilot` -> module `build::terraform::gke-autopilot`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-f0d8dd63042f6d4d"></a>


### File `build/terraform/gke-autopilot/module.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0130690344f9d00ec3ce4c886ca9ff2c5d834769`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/gke-autopilot/module.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-9daca186d1bc0e04ba.** [agones/build/terraform/gke-autopilot/module.tf, lines 7–26](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/gke-autopilot/module.tf#L7-L26). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `99d5bf246db1cab37a43ef38605bea949a5da13e330275422d9d61d6ee8b4756`.

> //     http://www.apache.org/licenses/LICENSE-2.0
> //
> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform apply -var project="<YOUR_GCP_ProjectID>" [-var agones_version="1.17.0"]
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source  = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/performance` -> module `build::terraform::performance`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ae5a92f728a8aaf1"></a>


### File `build/terraform/performance/module.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `71316d523f4e2e5697e217e5115756077b731d6d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/performance/module.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-1d5d7060e694412ecc.** [agones/build/terraform/performance/module.tf, lines 11–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/performance/module.tf#L11-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2ae3c267d7486b1353e7748a256368a2bc0e540f34c16fb05e9ccdd1b4f00dec`.

> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform init -backend-config="bucket=<YOUR_GCP_ProjectID>-performance-infra-bucket-tfstate" -backend-config="prefix=terraform/state"
> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> // The performance test cluster is hosted in project `agones-images`, i.e:
> //  terraform init -backend-config="bucket=agones-images-performance-infra-bucket-tfstate" -backend-config="prefix=terraform/state"
> //  terraform apply -var project="agones-images"
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>     helm = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/performance/state-bucket` -> module `build::terraform::performance::state-bucket`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-b1cb85acf308d405"></a>


### File `build/terraform/performance/state-bucket/main.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `14645f34ce20dcc03b06b46be6dc8e28282d6dd9`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/performance/state-bucket/main.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-7806f84946cf6551ad.** [agones/build/terraform/performance/state-bucket/main.tf, lines 12–31](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/performance/state-bucket/main.tf#L12-L31). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `79bdc692f08c7a2b4897ed2307afbb1b789ed952a778f4d33284c6b11e288e76`.

> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> 
> // The performance test cluster is hosted in project `agones-e2e-1`, i.e:
> //  terraform apply -var project="agones-e2e-1"
> 
> // # GCS bucket for holding the Terraform state of the performance test Terraform config.
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source  = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/upgrade` -> module `build::terraform::upgrade`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-554c065b68218a70"></a>


### File `build/terraform/upgrade/module.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b4a086763ad963c6fa127dea86ebd5aa1079518c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/upgrade/module.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-911bf5a1c59b590fe0.** [agones/build/terraform/upgrade/module.tf, lines 8–27](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/upgrade/module.tf#L8-L27). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e2f64aa60ff2b10ea7414cd952bb016be6acfc184d238f1fc01a0c5263fa96cd`.

> //
> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform init -backend-config="bucket=<YOUR_GCP_ProjectID>-upgrade-infra-bucket-tfstate" -backend-config="prefix=terraform/state"
> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>     helm = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/upgrade/gke-autopilot` -> module `build::terraform::upgrade::gke-autopilot`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-9d82e4282c649283"></a>


### File `build/terraform/upgrade/gke-autopilot/module.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4f063c95a484477945b2ffa20faadb26f25778d6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/upgrade/gke-autopilot/module.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-189a125134cc299869.** [agones/build/terraform/upgrade/gke-autopilot/module.tf, lines 7–26](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/upgrade/gke-autopilot/module.tf#L7-L26). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fc9ad5c4b0dccb16ee31816bc1df28fa9d33e458e7298f1208c48e030b6d5c98`.

> //     http://www.apache.org/licenses/LICENSE-2.0
> //
> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source  = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>     helm = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/upgrade/gke-standard` -> module `build::terraform::upgrade::gke-standard`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-093c4741dd08426b"></a>


### File `build/terraform/upgrade/gke-standard/module.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `52f3756cb73154e330fa8c150aff474d1a1d80a0`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/upgrade/gke-standard/module.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e070a14ae737295108.** [agones/build/terraform/upgrade/gke-standard/module.tf, lines 7–26](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/upgrade/gke-standard/module.tf#L7-L26). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3d521f3b3feb3ea90493d051dadd22445e51f1b27af23fb92aeb5d0f02256954`.

> //     http://www.apache.org/licenses/LICENSE-2.0
> //
> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source  = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>     helm = {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:build/terraform/upgrade/state-bucket` -> module `build::terraform::upgrade::state-bucket`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-c2375907fcb58e09"></a>


### File `build/terraform/upgrade/state-bucket/main.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1ce48afdf06092c4f1c6a48b95c114e1c3332cb6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/build/terraform/upgrade/state-bucket/main.tf`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-299f1be5bfc7b46d9c.** [agones/build/terraform/upgrade/state-bucket/main.tf, lines 9–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/build/terraform/upgrade/state-bucket/main.tf#L9-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d46f6a660cc9ab5054ef73fe1d7bbbf24f0d7b84dfdc62047f529312b8e23935`.

> // Unless required by applicable law or agreed to in writing, software
> // distributed under the License is distributed on an "AS IS" BASIS,
> // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> // See the License for the specific language governing permissions and
> // limitations under the License.
> 
> 
> // Run:
> //  terraform apply -var project="<YOUR_GCP_ProjectID>"
> 
> // GCS bucket for holding the Terraform state of the upgrade test Terraform config.
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     google = {
>       source  = "hashicorp/google"
>       version = "~> 4.25.0"
>     }
>   }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:ci` -> module `ci`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-c6323310266081e7"></a>


### File `ci/cancelot.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `87717fb7ed5ceada41a308caf78f8701e5cee253`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/ci/cancelot.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d6e735873db88ba27a.** [agones/ci/cancelot.sh, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/ci/cancelot.sh#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ecf55e36d201a71acf2ebd2cb44f4449056e98a56909d51d936eef573828ea23`.

> set -o errexit
> # Return value of a pipeline is the value of the last (rightmost) command to exit with a non-zero status
> set -o pipefail
> 
> CMDNAME=${0##*/}
> echoerr() { echo "$@" 1>&2; }
> 
> usage() {
>     cat <<USAGE >&2
> Usage:
>     $CMDNAME --current_build_id \$BUILD_ID
>     --current_build_id \$BUILD_ID  Current Build Id
> USAGE
>     exit 1
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7e00af14ed17ac02"></a>


### File `ci/e2e-test-cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `dc578fc54765130cf585371822781f2494e9d704`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/ci/e2e-test-cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-59d1ed42062362e9b3.** [agones/ci/e2e-test-cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/ci/e2e-test-cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `65d3a20f3abaef5edba1393bd4c396e6390dec79c385f591acdba196854d3a2b`.

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

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3c8157d07923db86"></a>


### File `ci/perf-test-cloudbuild.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `02e76f945c49fc380e60b9394225bd379f6728a7`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/ci/perf-test-cloudbuild.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d4a619d1f27ae7bb98.** [agones/ci/perf-test-cloudbuild.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/ci/perf-test-cloudbuild.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `69bfbc7b3fc7dc06f69da9f43bd3ad36330239d0ab2cd1b1dd9d7e76dce03289`.

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

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones` -> module `install::helm::agones`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-c5a4586fb927aa3b"></a>


### File `install/helm/agones/.helmignore`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `542b3390d86fdf1b3718b3dc564fcaeb93032643`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/.helmignore`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b193a278497619bf09.** [agones/install/helm/agones/.helmignore, lines 4–18](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/.helmignore#L4-L18). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d8cfdede78801560685206f3a77d54b3e9e1fcbc1a8d340b341f05ca0dc635e1`.

> .DS_Store
> # Common VCS dirs
> .git/
> .gitignore
> .bzr/
> .bzrignore
> .hg/
> .hgignore
> .svn/
> # Common backup files
> *.swp
> *.bak
> *.tmp
> *~
> # Various IDEs


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-31c20b9aa1e3d229"></a>


### File `install/helm/agones/Chart.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f1c7c92c003251ae0da90477e66afca4fc3c5ec0`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/Chart.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4a8628d6b5c8f38cde.** [agones/install/helm/agones/Chart.yaml, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/Chart.yaml#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `22a0566be83297cd1f3bd98e3883bacfa12e0d5653f88c71f3c724522179abf3`.

> 
> apiVersion: v1
> appVersion: "1.62.0-dev"
> version: 1.62.0-dev
> name: agones
> description: a library for hosting, running and scaling dedicated game servers on Kubernetes.
> keywords:
>   - go
>   - golang
>   - dedicated-gameservers
>   - multiplayer
>   - game-development
> home: https://agones.dev
> sources:
>   - https://github.com/agones-dev/agones


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-1cc29f94ce15da63"></a>


### File `install/helm/agones/README.md`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c86b5720c2aeebe46f877e6e42332240e90d8928`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/README.md`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-12b0c99a1fedc97ae6.** [agones/install/helm/agones/README.md, lines 2–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/README.md#L2-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `dbd282ff58e763a789302c8f5143a9322a3bffdef26684c27f3d0a6b9bce85d9`.

> 
> This chart installs the Agones application and defines deployment on a [Kubernetes](http://kubernetes.io)
> cluster using the [Helm](https://helm.sh) package manager.
> 
> See [Install Agones using Helm](https://agones.dev/site/docs/installation/install-agones/helm/) for
> installation and configuration instructions.
> 
> ## Development Work on Agones Custom Resource Definitions (CRDs)
> 
> Agones [extends the Kubernetes API with CustomResourceDefinitions](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/)
> for the kinds `Fleet`, `GameServerSet`, `GameServer`, `FleetAutoscaler`.
> (`GameServerAllocation` does not a have CRD.) Regardless of installation method, the definitions for
> the Agones Custom Resources are in the [agones/install/helm/agones/templates/crds](./templates/crds/)
> directory. The double braces `{{ }}` in the CRDs and elsewhere are
> [Helm chart template](https://helm.sh/docs/chart_template_guide/getting_started/) features.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-fa41cccb5e463b0c"></a>


### File `install/helm/agones/defaultfeaturegates.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d263b61e047f9643b672451d6ec4033fb2136246`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/defaultfeaturegates.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-4c273633f84978f1f3.** [agones/install/helm/agones/defaultfeaturegates.yaml, lines 20–34](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/defaultfeaturegates.yaml#L20-L34). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c00d3958bb2d77ff801657fdd5d6e1f5d247e0b11a5ee0431370743b1a81992e`.

> DisableResyncOnSDKServer: true
> AutopilotPassthroughPort: true
> PortPolicyNone: true
> PortRanges: true
> RollingUpdateFix: true
> 
> # Beta features
> 
> CountsAndLists: true
> GKEAutopilotExtendedDurationPods: true
> ScheduledAutoscaler: true
> SidecarContainers: true
> FleetAutoscaleRequestMetaData: true
> 
> # Alpha features


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-12015178ab50983a"></a>


### File `install/helm/agones/values.schema.json`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `714c1c57beeea2b56c1d35fb97c9750581c73608`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/values.schema.json`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-18578430e8cc2b13ed.** [agones/install/helm/agones/values.schema.json, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/values.schema.json#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `79eb615ce4f0d2e50cac007a10e92b9cec0aac346d006f60614fef8882b45883`.

> {
>   "$schema": "http://json-schema.org/draft-07/schema#",
>   "type": "object",
>   "properties": {
>     "agones": {
>       "type": "object",
>       "properties": {
>         "featureGates": {
>           "type": "string"
>         },
>         "rbacEnabled": {
>           "type": "boolean"
>         },
>         "registerApiService": {
>           "type": "boolean"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-98f8d5f26c907cb9"></a>


### File `install/helm/agones/values.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9cd6b8234bcc662ad33e4f7e2477b0e3083c2a83`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/values.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5f6755026ee5abeb22.** [agones/install/helm/agones/values.yaml, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/values.yaml#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a54f957723b402c69f6d0c6c094ec3006b415553de333d3da4dfd9a41e23d43c`.

> 
> agones:
>   featureGates: ""
>   metrics:
>     prometheusEnabled: true
>     prometheusServiceDiscovery: true
>     stackdriverEnabled: false
>     stackdriverProjectID: ""
>     stackdriverLabels: ""
>     serviceMonitor:
>       enabled: false
>       interval: 30s
>       additionalLabels: {}
>   rbacEnabled: true
>   registerServiceAccounts: true


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/certs` -> module `install::helm::agones::certs`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-51324847da5a2c76"></a>


### File `install/helm/agones/certs/cert.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `470978f95e387a4eae4811bdeb5b1d99ff0c9b8e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/certs/cert.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-610d9d1c8b87351a70.** [agones/install/helm/agones/certs/cert.sh, lines 2–12](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/certs/cert.sh#L2-L12). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4fc74f3a11742dcf2bdd1caa74d95c15f0ffe50d8a651013c38e33e8a66fab0e`.

> 
> echo "Generating certs..."
> echo "Email should be: emailAddress=agones-discuss@googlegroups.com"
> echo "Common Name should be: agones-controller-service.agones-system.svc"
> openssl genrsa -out server.key 2048
> openssl req -new -x509 -sha256 -key server.key -out server.crt -days 3650 -addext 'subjectAltName=DNS:agones-controller-service.agones-system.svc'
> 
> echo "caBundle:"
> base64 -w 0 server.crt
> 
> echo "done"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-de03e52225f178ab"></a>


### File `install/helm/agones/certs/server.crt`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `152430b94740698675928711a2db5c76986d2cdc`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/certs/server.crt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-33101925e31cdb5968.** [agones/install/helm/agones/certs/server.crt, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/certs/server.crt#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `249fd5c4480298d784a0824e2dc7109bab67e23abb94d0835da678048ab7306f`.

> -----BEGIN CERTIFICATE-----
> MIIEcTCCA1mgAwIBAgIUFnC9K1OY3FqMihj7tVmxyGxpQWswDQYJKoZIhvcNAQEL
> BQAwgaoxCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApTb21lLVN0YXRlMQ8wDQYDVQQK
> DAZBZ29uZXMxDzANBgNVBAsMBkFnb25lczE0MDIGA1UEAwwrYWdvbmVzLWNvbnRy
> b2xsZXItc2VydmljZS5hZ29uZXMtc3lzdGVtLnN2YzEuMCwGCSqGSIb3DQEJARYf
> YWdvbmVzLWRpc2N1c3NAZ29vZ2xlZ3JvdXBzLmNvbTAeFw0yMTA2MzAxOTU2MjFa
> Fw0zMTA2MjgxOTU2MjFaMIGqMQswCQYDVQQGEwJVUzETMBEGA1UECAwKU29tZS1T
> dGF0ZTEPMA0GA1UECgwGQWdvbmVzMQ8wDQYDVQQLDAZBZ29uZXMxNDAyBgNVBAMM
> K2Fnb25lcy1jb250cm9sbGVyLXNlcnZpY2UuYWdvbmVzLXN5c3RlbS5zdmMxLjAs
> BgkqhkiG9w0BCQEWH2Fnb25lcy1kaXNjdXNzQGdvb2dsZWdyb3Vwcy5jb20wggEi
> MA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQCgdkLOKCH58KJBitBjyYrL0+E9
> tItLXFTgqAOS0gAB+RUsY0hbreVDwtHLJapg0ny6/Ta7/0G5ZodhdxFQmKbV1LfY
> fddtBn/8gxZ/ICgQnU77tjcZKWrqin/gxweBnkxcXKkOvu2WhDwYeQK7vG4Ic8hs
> VGoXSej8Q/xwc8kABDm8aTRSTTbk2Z/dzofRk0SlksPkUeyoCpDeFlDjcKSp03Zu
> WgAQ3iW/7sP1TUyXKgnVy3fiZmQAFkxKNBLUWH/TBIykLuEB2dXaGt/Eig48IjU9


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-30b1be59a1f7e3d3"></a>


### File `install/helm/agones/certs/server.key`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9fb5933855fe97cff95c59a94fc2ef20df5a0468`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/certs/server.key`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-270c95e5bd0ebba708.** [agones/install/helm/agones/certs/server.key, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/certs/server.key#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9c1e4bd7a02c84a6280b3cd1a428f0467a1e4747e8a84c4f12f82200da474ee7`.

> -----BEGIN RSA PRIVATE KEY-----
> MIIEowIBAAKCAQEAoHZCzigh+fCiQYrQY8mKy9PhPbSLS1xU4KgDktIAAfkVLGNI
> W63lQ8LRyyWqYNJ8uv02u/9BuWaHYXcRUJim1dS32H3XbQZ//IMWfyAoEJ1O+7Y3
> GSlq6op/4McHgZ5MXFypDr7tloQ8GHkCu7xuCHPIbFRqF0no/EP8cHPJAAQ5vGk0
> Uk025Nmf3c6H0ZNEpZLD5FHsqAqQ3hZQ43CkqdN2bloAEN4lv+7D9U1MlyoJ1ct3
> 4mZkABZMSjQS1Fh/0wSMpC7hAdnV2hrfxIoOPCI1PS2i6961SJgq5M7z8lrg3u89
> nibde1x4S1Bq6vGtUx2cMEkWBULKJgjjREgCOwIDAQABAoIBAGMax4YGmX2CVuR+
> NNj82IGtWlc/FC02WKHsg1CB7UlKqv4gT8d37rrcL7D1kXZ9alYfIFgafmBg58qV
> vJb3wUseN3I5vvzyX8f4usNHfBdO4UJrtzoOL5+ubO7KXmN6/pSpEi0uVRV6lJdU
> 3hBPf/IueKIXvPKnToPIBbj6ejFMcHCPbm2zZkSqpkva9QqdsM6jnVAnblbjpwpz
> sFPAgVZLtoqbMwgPTiPgPzPLbvifsa7dzRqYlcL/i6Ov8D2TGkHxbKDoDxK3Vfxu
> YBj6KiI82Gr6FWq5oBilOwz2hhkdTzfgyfYxHLq3H0A5PpG4UBVp4pi3yH1iX7SQ
> VG98N4ECgYEAzOFutVDYr/mGFTP5Nd+C+lzsF8CjD2SaB3vV1tHi7UbttRFgSQIR
> lRwQNsHH8gLVh2UK2riUmXNhDwipEhNsXuDqcmDfeFxrO+L2gpbT6GcGe8ZONOgh
> aGbt6jsyJ1wypK57E5MPyltSsP6dZmMbC0nbM6C4B4R+iIETjly19cECgYEAyH+g


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/certs/allocator` -> module `install::helm::agones::certs::allocator`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-00ebab10bca38117"></a>


### File `install/helm/agones/certs/allocator/server.crt`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `845e92aad27dc8aa8ac93ffcd10c58562a5cd706`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/certs/allocator/server.crt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0745b14dd6db3c745a.** [agones/install/helm/agones/certs/allocator/server.crt, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/certs/allocator/server.crt#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a34abb0f114fa37fc5e9795b0686a68c587c6e0bd2f492b447bc147b927928e3`.

> -----BEGIN CERTIFICATE-----
> MIIC4jCCAcqgAwIBAgIRAOeMg0+RJhPInK9zAi2zhR4wDQYJKoZIhvcNAQELBQAw
> GDEWMBQGA1UEAxMNYWxsb2NhdGlvbi1jYTAeFw0xOTA2MTkxODQzNTVaFw0yOTA2
> MTYxODQzNTVaMAAwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQD6LJnC
> i9FfgsFx1T+zcRrnE4FQ5CPUEP8GkPsqHMbsmDzP+0B+UkE+y5pjVlg+u8qlsdCn
> ztQVU58/lz2SP7AfsH13HIJsi6xk773XREQ46XqJponX764Rj2yf7xOnJ04btlTJ
> t3q8SB/PY7skWtyQN2PD3xAStDEWnu/w3G1LM762XbwkZ7VRTcXEtJTi67uipw/z
> hUiOMpZeOV1WOgz+xr2Pe6f+CNm3XS3UnXcS8Jbiqj8WBc8o7ZKerFre4aLDIetY
> +x93YXX6X63Bd3ogreGpfxWQSxAa0GPKKkWohsDIfQDF0gJJr6RsJk9UNXFrhrj0
> TI4sLdLl+avRLJ1PAgMBAAGjPzA9MA4GA1UdDwEB/wQEAwIFoDAdBgNVHSUEFjAU
> BggrBgEFBQcDAQYIKwYBBQUHAwIwDAYDVR0TAQH/BAIwADANBgkqhkiG9w0BAQsF
> AAOCAQEAJgQ+W6ea7Jf8evjtqcfD/DeELc4KpQpvOMGFFT93A38masxSqUyn8i8F
> ihJe46EfqdDD/qdVH8xNBHwcHr82T5KpY3Mg5jbOXmb0J1dGRLTGJgFwFiQwl3rw
> fxuhebvoi2dqXPlg7/fYfej7ddm0117aD+pPLB7CMPeKvNPHqv7EAFZ09O+F37r9
> 50OdC+JMU+AMs4U35UxFFf4UDuHmc8SItm2kkU7VMSp1ZWUnEVEPLZSORgwYuaMq


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b9dbd97a46afba30"></a>


### File `install/helm/agones/certs/allocator/server.key`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `57bca1fcf12c179aa00e9774905c6f9944b831a8`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/certs/allocator/server.key`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-be658b1dc54fb39bc9.** [agones/install/helm/agones/certs/allocator/server.key, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/certs/allocator/server.key#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b13b5fa409602111fdc2545c8785043e3f85a3cd1fa46c1164fa08a76cd7a0bc`.

> -----BEGIN RSA PRIVATE KEY-----
> MIIEpQIBAAKCAQEA+iyZwovRX4LBcdU/s3Ea5xOBUOQj1BD/BpD7KhzG7Jg8z/tA
> flJBPsuaY1ZYPrvKpbHQp87UFVOfP5c9kj+wH7B9dxyCbIusZO+910REOOl6iaaJ
> 1++uEY9sn+8TpydOG7ZUybd6vEgfz2O7JFrckDdjw98QErQxFp7v8NxtSzO+tl28
> JGe1UU3FxLSU4uu7oqcP84VIjjKWXjldVjoM/sa9j3un/gjZt10t1J13EvCW4qo/
> FgXPKO2Snqxa3uGiwyHrWPsfd2F1+l+twXd6IK3hqX8VkEsQGtBjyipFqIbAyH0A
> xdICSa+kbCZPVDVxa4a49EyOLC3S5fmr0SydTwIDAQABAoIBAQC+RPWclSJkddoS
> bZHJNBi17ovHrfvh6htMLzAXU1On1hFKTVk7ueuNiuSc8Kqk68awRpDfT9rVbYgM
> oUZe2LlnIKAL208GpyQykHPeKTmJ32knD9Z+uPe2u1JuQTK9SpOJWB8cW8OpN2Ga
> JcvLVp+xv67M5fYfg5RiE/eBROSK8AsFE1IS6jRG4Uh/8SWRQGDc9QyMKsyMlSUg
> aj0xeklYFCYytYKB2pZHl47YT+Y3jTzqsuzHq+XH3XdON7l97xvXebOLBYEj+0b5
> ix4BulBHA/VFOwyu6/4JjZmDpuuTi6zMmS2aut9mDcuh9FczmjCFLrp2Zbb0/Zp4
> FKmetIGBAoGBAP0K98Wy27lrVIkCbUNzh6GsyHFIvYav4q2lK2YTgJFWvkKFzaJj
> fadY0tfPuzWwOQOgZIwioxLINTSRkIel+kBqU2CHnVNsqx3NgIi4KH7+YmKfE0Rm
> DZjeta9iMnvkB2AuREHCHJeU8XwbMGbgirMJhefb0DBeclffl+nO0JQRAoGBAP0Z


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/certs/allocator/client-ca` -> module `install::helm::agones::certs::allocator::client-ca`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-3f794832c364701c"></a>


### File `install/helm/agones/certs/allocator/client-ca/ca.crt`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `845e92aad27dc8aa8ac93ffcd10c58562a5cd706`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/certs/allocator/client-ca/ca.crt`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5b5219009cd0ed38e9.** [agones/install/helm/agones/certs/allocator/client-ca/ca.crt, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/certs/allocator/client-ca/ca.crt#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a34abb0f114fa37fc5e9795b0686a68c587c6e0bd2f492b447bc147b927928e3`.

> -----BEGIN CERTIFICATE-----
> MIIC4jCCAcqgAwIBAgIRAOeMg0+RJhPInK9zAi2zhR4wDQYJKoZIhvcNAQELBQAw
> GDEWMBQGA1UEAxMNYWxsb2NhdGlvbi1jYTAeFw0xOTA2MTkxODQzNTVaFw0yOTA2
> MTYxODQzNTVaMAAwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQD6LJnC
> i9FfgsFx1T+zcRrnE4FQ5CPUEP8GkPsqHMbsmDzP+0B+UkE+y5pjVlg+u8qlsdCn
> ztQVU58/lz2SP7AfsH13HIJsi6xk773XREQ46XqJponX764Rj2yf7xOnJ04btlTJ
> t3q8SB/PY7skWtyQN2PD3xAStDEWnu/w3G1LM762XbwkZ7VRTcXEtJTi67uipw/z
> hUiOMpZeOV1WOgz+xr2Pe6f+CNm3XS3UnXcS8Jbiqj8WBc8o7ZKerFre4aLDIetY
> +x93YXX6X63Bd3ogreGpfxWQSxAa0GPKKkWohsDIfQDF0gJJr6RsJk9UNXFrhrj0
> TI4sLdLl+avRLJ1PAgMBAAGjPzA9MA4GA1UdDwEB/wQEAwIFoDAdBgNVHSUEFjAU
> BggrBgEFBQcDAQYIKwYBBQUHAwIwDAYDVR0TAQH/BAIwADANBgkqhkiG9w0BAQsF
> AAOCAQEAJgQ+W6ea7Jf8evjtqcfD/DeELc4KpQpvOMGFFT93A38masxSqUyn8i8F
> ihJe46EfqdDD/qdVH8xNBHwcHr82T5KpY3Mg5jbOXmb0J1dGRLTGJgFwFiQwl3rw
> fxuhebvoi2dqXPlg7/fYfej7ddm0117aD+pPLB7CMPeKvNPHqv7EAFZ09O+F37r9
> 50OdC+JMU+AMs4U35UxFFf4UDuHmc8SItm2kkU7VMSp1ZWUnEVEPLZSORgwYuaMq


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/scripts` -> module `install::helm::agones::scripts`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-900481704de0e471"></a>


### File `install/helm/agones/scripts/delete_agones_resources.sh`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b86042e0ee2e75c84d63ab329d2f2eca3e325612`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/scripts/delete_agones_resources.sh`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f4a8c0ec7c91feef3f.** [agones/install/helm/agones/scripts/delete_agones_resources.sh, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/scripts/delete_agones_resources.sh#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4ec230e11db4c16212ca439887e7d297ab835f1f496690f377c2b2d4e62be932`.

> 
> namespaces=$@
> 
> for ns in $namespaces; do
>   # Building the list of pods we need to ensure are deleted.
>   gs=$(kubectl -n $ns get gs -o jsonpath='{.items[*].metadata.name}')
> 
>   for g in $gs; do
>     pod=$(kubectl -n $ns get po -l agones.dev/gameserver=$g -o jsonpath='{.items[*].metadata.name}')
>     pods="$pods $pod"
>   done
> 
>   # Delete Agones resources to kickstart pod deletion.
>   kubectl -n $ns delete fleetautoscalers --all
>   kubectl -n $ns delete fleets --all


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/templates` -> module `install::helm::agones::templates`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-11ac079369c91f8e"></a>


### File `install/helm/agones/templates/NOTES.txt`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `042d911096958091b2436d1cf08468c74e0229d0`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/NOTES.txt`.


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-6502710c739a6e9ae1.** [agones/install/helm/agones/templates/NOTES.txt, lines 1–13](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/NOTES.txt#L1-L13). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b660f843f7900c818f34912fdd422b92328dfdbd1a0da2bb059df886f67e53dc`.

> The Agones components have been installed in the namespace {{ .Release.Namespace }}.
> 
> You can get their status by running:
> kubectl --namespace {{ .Release.Namespace }} get pods -o wide
> 
> Once ready you can create your first GameServer using our examples https://agones.dev/site/docs/getting-started/create-gameserver/ .
> 
> Finally don't forget to explore our documentation and usage guides on how to develop and host dedicated game servers on top of Agones:
> 
>  - Create a Game Server (https://agones.dev/site/docs/getting-started/create-gameserver/)
>  - Integrating the Game Server SDK (https://agones.dev/site/docs/guides/client-sdks/)
>  - GameServer Health Checking (https://agones.dev/site/docs/guides/health-checking/)
>  - Accessing Agones via the Kubernetes API (https://agones.dev/site/docs/guides/access-api/)


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-bd3c8870164079b4"></a>


### File `install/helm/agones/templates/_helpers.tpl`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `cd19f469f4770d1e899f21e201f581752bc2caa2`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/_helpers.tpl`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0eb84a274ff62eee2a.** [agones/install/helm/agones/templates/_helpers.tpl, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/_helpers.tpl#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b2cb9cff88cb55e1d5b71f8ffcdcde4175fb7e1d930ab3f62f4f60a6f0c198ad`.

> {*
>  Copyright Contributors to Agones a Series of LF Projects, LLC.
> 
>  Licensed under the Apache License, Version 2.0 (the "License");
>  you may not use this file except in compliance with the License.
>  You may obtain a copy of the License at
> 
>      http://www.apache.org/licenses/LICENSE-2.0
> 
>  Unless required by applicable law or agreed to in writing, software
>  distributed under the License is distributed on an "AS IS" BASIS,
>  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
>  See the License for the specific language governing permissions and
>  limitations under the License.
> *}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f4874c57181cfee8"></a>


### File `install/helm/agones/templates/controller-metrics-service.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `9b01fd3b8812f580cf1e3ddda9d495a1bd00bb21`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/controller-metrics-service.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-f05f0d199c96bd4b2e.** [agones/install/helm/agones/templates/controller-metrics-service.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/controller-metrics-service.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `11aa85445bb133ca375493a2dc247a43de7531736555f5784e024d4fdcf979b8`.

> apiVersion: v1
> kind: Service
> metadata:
>   name: agones-controller-metrics-service
>   namespace: {{ .Release.Namespace }}
>   labels:
>     agones.dev/role: controller
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:
>   selector:
>     agones.dev/role: controller
>   ports:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4cdb69f73043e10c"></a>


### File `install/helm/agones/templates/controller.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5079ebdfa1d598940e8cea7f6c6b6a80a4c45539`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/controller.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fc80c82208e571747b.** [agones/install/helm/agones/templates/controller.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/controller.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `22edf46694ca44e4b56ac608d55f7eaf85a02ebc7218e9faa48100ce626e5508`.

> 
> {{- $replicas := (int .Values.agones.controller.replicas) }}
> {{- if lt $replicas 1 }}
> {{- fail "agones.controller.replica field has an invalid value!" }}
> {{- end }}
> apiVersion: apps/v1
> kind: Deployment
> metadata:
>   name: agones-controller
>   namespace: {{ .Release.Namespace }}
>   labels:
>     component: controller
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-d581720da0ed6229"></a>


### File `install/helm/agones/templates/extensions-deployment.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a7e97723cabfdffb9abc6a09748c948619b30e8f`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/extensions-deployment.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-e41d430312ea5bf415.** [agones/install/helm/agones/templates/extensions-deployment.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/extensions-deployment.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `19674431af8d9e337db901d7462ebc334b95668ac570624829141d98a3d2966f`.

> 
> apiVersion: apps/v1
> kind: Deployment
> metadata:
>   name: agones-extensions
>   namespace: {{ .Release.Namespace }}
>   labels:
>     component: extensions
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:
>   selector:
>     matchLabels:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7556b53d1d12d5c5"></a>


### File `install/helm/agones/templates/extensions-metrics-service.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `84b0719701109f83d70b1b919385f60ce2c7867a`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/extensions-metrics-service.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c54880b9cbaa849a37.** [agones/install/helm/agones/templates/extensions-metrics-service.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/extensions-metrics-service.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `453711ad01908ebd17e331c66d6176ce7d37840367f6e826c9d1f19e41a5e30f`.

> 
> apiVersion: v1
> kind: Service
> metadata:
>   name: agones-extensions-metrics-service
>   namespace: {{ .Release.Namespace }}
>   labels:
>     agones.dev/role: extensions
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:
>   selector:
>     agones.dev/role: extensions


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ef24e3fa0013a616"></a>


### File `install/helm/agones/templates/extensions.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `0474bb9c6e22b6427c1a3f9d7f5ee4de95bce46e`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/extensions.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2785047bc92b1df046.** [agones/install/helm/agones/templates/extensions.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/extensions.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e8f35ef66addb988208173e8877ae069e866b5b13fdde9a4fd377925a41da15b`.

> {{- $ca := genCA "admission-controller-ca" 3650 }}
> {{- $cn := printf "agones-controller-service" }}
> {{- $altName1 := printf "agones-controller-service.%s"  .Release.Namespace }}
> {{- $altName2 := printf "agones-controller-service.%s.svc" .Release.Namespace }}
> {{- $cert := genSignedCert $cn nil (list $altName1 $altName2) 3650 $ca }}
> ---
> {{- if .Values.agones.registerApiService }}
> apiVersion: apiregistration.k8s.io/v1
> kind: APIService
> metadata:
>   name: v1.allocation.agones.dev
>   labels:
>     component: controller
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5baaa49594a4d738"></a>


### File `install/helm/agones/templates/pdb.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `70c9b204243d59006905ae6bb110df42aea0b64f`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/pdb.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-127d444bd432dab744.** [agones/install/helm/agones/templates/pdb.yaml, lines 14–27](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/pdb.yaml#L14-L27). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2eabbd32a719ddb3c3a486686160e2956810950a2404f6f799652ba928b80b73`.

> 
> {{- range .Values.gameservers.namespaces }}
> ---
> apiVersion: policy/v1
> kind: PodDisruptionBudget
> metadata:
>   name: agones-gameserver-safe-to-evict-false
>   namespace: {{ . }}
> spec:
>   maxUnavailable: 0%
>   selector:
>     matchLabels:
>       agones.dev/safe-to-evict: "false"
> {{- end }}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a69519fbb80580ca"></a>


### File `install/helm/agones/templates/ping.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6c144e8fb1330bed0af6739a9aba726983135fad`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/ping.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-598054893455b4e715.** [agones/install/helm/agones/templates/ping.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/ping.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `70acad1be8cc884f3978573ae93ee85865c443986f3ff026b0febe6e7da7d877`.

> 
> {{- if .Values.agones.ping.install }}
> apiVersion: apps/v1
> kind: Deployment
> metadata:
>   name: agones-ping
>   namespace: {{ .Release.Namespace }}
>   labels:
>     component: ping
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:
> {{- if .Values.agones.ping.updateStrategy }}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-f76742453a9f7a95"></a>


### File `install/helm/agones/templates/priority-class.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `13f65434ea8718ef6d4ef02647636f01e97b36fa`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/priority-class.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-eeb86cffd4ab736a4e.** [agones/install/helm/agones/templates/priority-class.yaml, lines 1–9](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/priority-class.yaml#L1-L9). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9e1fd804a63985264b929a8616d2b88ddf6c9e92640ec207b4c405648d0a68cb`.

> {{- if .Values.agones.createPriorityClass }}
> apiVersion: scheduling.k8s.io/v1
> kind: PriorityClass
> metadata:
>   name: {{ .Values.agones.priorityClassName }}
> value: 1000000
> globalDefault: false
> description: "This priority class should be used for Agones service pods only."
> {{- end }}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-8ab5fc8000be55c7"></a>


### File `install/helm/agones/templates/processor.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `04dd6e88b320a6cf3a30039bb9656b9bd72005c3`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/processor.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-9a720c9796f898dacd.** [agones/install/helm/agones/templates/processor.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/processor.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bf35bb51ad7e419ca923d32bbae5e49a72a263f01a2107738b3d69843b031942`.

> 
> {{- $featureGates := include "agones.featureGates" . | fromYaml }}
> {{- if $featureGates.ProcessorAllocator }}
> apiVersion: apps/v1
> kind: Deployment
> metadata:
>   name: agones-processor
>   namespace: {{ .Release.Namespace }}
>   labels:
>     component: processor
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-692566287a1fbe5e"></a>


### File `install/helm/agones/templates/service-monitor.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `684200603cc979420618a3800b6ceaf5d80d18be`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/service-monitor.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0108a5d69b725837bc.** [agones/install/helm/agones/templates/service-monitor.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/service-monitor.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `77c89320633d6786a83090bff7b1a73298e84597936805c1e12c8f626f480233`.

> {{- if and (.Values.agones.metrics.prometheusEnabled) (.Values.agones.metrics.serviceMonitor.enabled) }}
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
> apiVersion: monitoring.coreos.com/v1


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-243ff0a1a0d3e8cb"></a>


### File `install/helm/agones/templates/service.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a460d77600ac9f6eca1826b0da9cdc2a0a6edc7b`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/service.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-5caec58a0bff56b9f5.** [agones/install/helm/agones/templates/service.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/service.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f49fa012cc85418d3b4437cd93dcfb768bd55f443e4cd10383ee1987bbe6f791`.

> {{- $featureGates := include "agones.featureGates" . | fromYaml }}
> 
> apiVersion: v1
> kind: Service
> metadata:
>   name: agones-controller-service
>   namespace: {{ .Release.Namespace }}
>   labels:
>     agones.dev/role: extensions
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:
>   selector:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/templates/crds` -> module `install::helm::agones::templates::crds`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ab8be32149c6dca3"></a>


### File `install/helm/agones/templates/crds/_fleetautoscalerpolicy.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a7bf33daa21f882b945eec0850074c215c444840`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/_fleetautoscalerpolicy.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-d41e89d956abbb436d.** [agones/install/helm/agones/templates/crds/_fleetautoscalerpolicy.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/_fleetautoscalerpolicy.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a03998076f15865cf2e849d2dc4aa08c29ef7b2fc38f64c1afe5243e497f7610`.

> 
> ---
> {{/* schema for a URL configuration for webhooks and wasm */}}
> {{- define "url.configuration" }}
> type: object
> nullable: true
> properties:
>   url:
>     type: string
>   service:
>     type: object
>     required:
>       - namespace
>       - name
>     properties:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6824d22cb911d80c"></a>


### File `install/helm/agones/templates/crds/_gameserverspecschema.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `36e5a84b79860d32806d91914bc652b0a545a437`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/_gameserverspecschema.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-a4df8a4908c5e84d40.** [agones/install/helm/agones/templates/crds/_gameserverspecschema.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/_gameserverspecschema.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0c4c57b133371c65b55e287ef0d307c1f0a02b59822d08a100ea059e6470654c`.

> 
> {{/* schema for a gameserver spec */}}
> {{- define "gameserver.schema" }}
> description: 'GameServer is the data structure for a GameServer resource.'
> type: object
> required:
> - spec
> properties:
>   {{- if .metadata | default false }}
>   metadata:
>     {{- include "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta" . | indent 4 }}
>   {{- end}}
>   spec:
>     description: 'GameServerSpec is the spec for a GameServer resource. More info:
>       https://agones.dev/site/docs/reference/agones_crd_api_reference/#agones.dev/v1.GameServer'


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-38304097693315ff"></a>


### File `install/helm/agones/templates/crds/_gameserverstatus.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `606555404c79261ebe140468bee8fe84abc21eba`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/_gameserverstatus.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-620570bca5888bd089.** [agones/install/helm/agones/templates/crds/_gameserverstatus.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/_gameserverstatus.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4063548c3dda1fbbf6b33662738a3c8339e1bf63ff6ec4f810f90e1d94be6bca`.

> 
> {{/* schema for a gameserver status */}}
> {{- define "gameserver.status" }}
> status:
>   description: 'GameServerStatus is the status for a GameServer resource. More info:
>     https://agones.dev/site/docs/reference/agones_crd_api_reference/#agones.dev/v1.GameServer'
>   type: object
>   title: The status values for the GameServer
>   properties:
>     state:
>       type: string
>       enum:
>         - PortAllocation
>         - Creating
>         - Starting


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a447952d1b7b9e07"></a>


### File `install/helm/agones/templates/crds/fleet.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c4ea1be8e1d3393044d00704c1c2008f7d02b2f3`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/fleet.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2350284e17da66dcf2.** [agones/install/helm/agones/templates/crds/fleet.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/fleet.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5531032c76c27f16bb30b0dd879222ec0ee86094ae67b9e89fb025f61418e71f`.

> 
> {{- if .Values.agones.crds.install }}
> 
> apiVersion: apiextensions.k8s.io/v1
> kind: CustomResourceDefinition
> metadata:
>   name: fleets.agones.dev
>   labels:
>     component: crd
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:
>   group: agones.dev


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3e372fc50b700d19"></a>


### File `install/helm/agones/templates/crds/fleetautoscaler.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3eaa691c1567377e3eff8ad7bf2115ba4d01dd6a`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/fleetautoscaler.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-61a261bbe2076aca1f.** [agones/install/helm/agones/templates/crds/fleetautoscaler.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/fleetautoscaler.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a3af4bd36d25ee11e38bf49f541c3993f68e12315ae2b6d632e0360b565732a9`.

> 
> {{- if .Values.agones.crds.install }}
> 
> apiVersion: apiextensions.k8s.io/v1
> kind: CustomResourceDefinition
> metadata:
>   name: fleetautoscalers.autoscaling.agones.dev
>   labels:
>     component: crd
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:
>   group: autoscaling.agones.dev


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-eacd08f676a8a1ef"></a>


### File `install/helm/agones/templates/crds/gameserver.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3fd4d5e8b4a98d794a77520696d8d1d8d7960560`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/gameserver.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-90f8caee35039f612d.** [agones/install/helm/agones/templates/crds/gameserver.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/gameserver.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8ffb3ca501a000e65a4d95c26531b478d37bce8bafe857b8bf7c009d5ce4da47`.

> 
> {{- if .Values.agones.crds.install }}
> 
> apiVersion: apiextensions.k8s.io/v1
> kind: CustomResourceDefinition
> metadata:
>   name: gameservers.agones.dev
>   labels:
>     component: crd
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:
>   group: agones.dev


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ea694e4ac6fc8321"></a>


### File `install/helm/agones/templates/crds/gameserverallocationpolicy.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c319a8e106cc6cd5dbba6184e8a4c0c7d0bf93d8`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/gameserverallocationpolicy.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-35fe75cde855db0c56.** [agones/install/helm/agones/templates/crds/gameserverallocationpolicy.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/gameserverallocationpolicy.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `446ec0bf96a2bbc6ecb913af4ebb58b3da81a719f90faed7b30a0ebc0ff35e72`.

> 
> {{- if .Values.agones.crds.install }}
> 
> apiVersion: apiextensions.k8s.io/v1
> kind: CustomResourceDefinition
> metadata:
>   labels:
>     component: crd
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
>   name: gameserverallocationpolicies.multicluster.agones.dev
> spec:
>   group: multicluster.agones.dev


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3ee992d98fc26f21"></a>


### File `install/helm/agones/templates/crds/gameserverset.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `94b5ce2cbaed949936ff08eb38913450cb078a60`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/gameserverset.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6571e3b4814f7afd80.** [agones/install/helm/agones/templates/crds/gameserverset.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/gameserverset.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4a8bdb0376add124524b056ea6f174b4ec6662bfc04ec1137b48ea01264198ab`.

> 
> {{- if .Values.agones.crds.install }}
> 
> apiVersion: apiextensions.k8s.io/v1
> kind: CustomResourceDefinition
> metadata:
>   name: gameserversets.agones.dev
>   labels:
>     component: crd
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> spec:
>   group: agones.dev


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/templates/crds/k8s` -> module `install::helm::agones::templates::crds::k8s`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-d8da6d80864c9b1c"></a>


### File `install/helm/agones/templates/crds/k8s/_io.k8s.api.core.v1.PodTemplateSpec.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `8dc237c9fce5f8cb0328dee490e37f10d48e7713`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/k8s/_io.k8s.api.core.v1.PodTemplateSpec.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ec65f22992524218ef.** [agones/install/helm/agones/templates/crds/k8s/_io.k8s.api.core.v1.PodTemplateSpec.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/k8s/_io.k8s.api.core.v1.PodTemplateSpec.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9f75e0936055d9237b091c6deb7ce9e555aba4d7a4b2d8521dccb35d90ed8ff4`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Preserve source floating width, narrowing, exceptional values and relevant bits.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-807c9b8c00712dd8"></a>


### File `install/helm/agones/templates/crds/k8s/_io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `28d2f38b84d7d8d56d59fde04a0a39e1a29111bf`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/crds/k8s/_io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-7b6b5436ae014fb9f8.** [agones/install/helm/agones/templates/crds/k8s/_io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/crds/k8s/_io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `419f9e7779f83b07a74bbdd6ae37c905e3023c7584773520eb216c80005a0729`.

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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/templates/hooks` -> module `install::helm::agones::templates::hooks`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-9dce32699e28ca85"></a>


### File `install/helm/agones/templates/hooks/pre_delete_hook.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b2203343cfc60e31ec4b71c0419210900713d389`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/hooks/pre_delete_hook.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8914d9e3a574a0cb56.** [agones/install/helm/agones/templates/hooks/pre_delete_hook.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/hooks/pre_delete_hook.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `42843d2851361bfff89114c924b83c4e400429eb34a851fd2708683f412ac85a`.

> {{- if .Values.agones.crds.cleanupOnDelete }}
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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-73d95c39fe6cf7fd"></a>


### File `install/helm/agones/templates/hooks/sa.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `92d0df7bf3dc04210387a32a859de83bc031777c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/hooks/sa.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-46a1090dd098cc7747.** [agones/install/helm/agones/templates/hooks/sa.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/hooks/sa.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `f9bca2f3e2096ca211a2ebd7f949d83ca8469c2140c7dd7e96fe2faf102faf01`.

> {{- if .Values.agones.crds.cleanupOnDelete }}
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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-9a1efb21739e6786"></a>


### File `install/helm/agones/templates/hooks/scripts.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `79b564c461bfdf1636bb5da1e29a4a50e6aed741`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/hooks/scripts.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-36a9eac0e501776387.** [agones/install/helm/agones/templates/hooks/scripts.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/hooks/scripts.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a02e273acfd585fee7db27257559bf57ca739ace748a491d7218b22fa74b7a5b`.

> {{- if .Values.agones.crds.cleanupOnDelete }}
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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/templates/service` -> module `install::helm::agones::templates::service`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-6d4fa45c74c2baf2"></a>


### File `install/helm/agones/templates/service/allocation.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d4170f6b1a4c1d871cb5083c48c5d5ce8050ee40`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/service/allocation.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-536ff4244ab915d599.** [agones/install/helm/agones/templates/service/allocation.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/service/allocation.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2998f1da5046f4bb41360e1741490bfeccb2d740e56e96934f072517dd0f9812`.

> 
> {{- $useLoadBalancerIP := and (ne .Values.agones.allocator.service.loadBalancerIP "") (eq .Values.agones.allocator.service.serviceType "LoadBalancer") }}
> {{- if .Values.agones.allocator.install }}
> # Define a Service for the agones-allocator
> apiVersion: v1
> kind: Service
> metadata:
>   name: {{ $.Values.agones.allocator.service.name }}
>   namespace: {{ .Release.Namespace }}
>   labels:
>     component: allocator
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/templates/serviceaccounts` -> module `install::helm::agones::templates::serviceaccounts`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-29a5b77037983b67"></a>


### File `install/helm/agones/templates/serviceaccounts/controller.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `727425be75e69510be48edc0d52fb3e2eb34a554`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/serviceaccounts/controller.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3de41c2331f210e785.** [agones/install/helm/agones/templates/serviceaccounts/controller.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/serviceaccounts/controller.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5e54a257578facb7fdbacfd2ef7accae4a7f4dbd5d90cb0ef548ec3645719f21`.

> 
> {{- if .Values.agones.registerServiceAccounts }}
> apiVersion: v1
> kind: ServiceAccount
> metadata:
>   name: {{ .Values.agones.serviceaccount.controller.name }}
>   namespace: {{ .Release.Namespace }}
>   labels:
>     app: {{ template "agones.name" . }}
>     chart: {{ template "agones.chart" . }}
>     release: {{ .Release.Name }}
>     heritage: {{ .Release.Service }}
> {{- if .Values.agones.serviceaccount.controller.annotations }}
>   annotations:
> {{- toYaml .Values.agones.serviceaccount.controller.annotations | nindent 4 }}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-825e572c94aad769"></a>


### File `install/helm/agones/templates/serviceaccounts/sdk.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f53dac61bc39e77991dc48560c25e98a98bf3b46`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/serviceaccounts/sdk.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c4eecbdb29d0bf9a7e.** [agones/install/helm/agones/templates/serviceaccounts/sdk.yaml, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/serviceaccounts/sdk.yaml#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `78b2708f00915b5fd959185d7634ef49f0d0636cc262343cf0d25dd8e617b99d`.

> {{- if .Values.agones.registerServiceAccounts }}
> {{- range .Values.gameservers.namespaces }}
> apiVersion: v1
> kind: ServiceAccount
> metadata:
>   name: {{ $.Values.agones.serviceaccount.sdk.name }}
>   namespace: {{ . }}
>   labels:
>     app: {{ template "agones.name" $ }}
>     chart: {{ template "agones.chart" $ }}
>     release: {{ $.Release.Name }}
>     heritage: {{ $.Release.Service }}
> {{- if hasKey $.Values.agones.serviceaccount.sdk.annotations . }}
>   annotations:
> {{- get $.Values.agones.serviceaccount.sdk.annotations . | toYaml | nindent 4 }}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/helm/agones/templates/tests` -> module `install::helm::agones::templates::tests`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-f9272371e3e198bf"></a>


### File `install/helm/agones/templates/tests/test-runner.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `33861c1f5e07a2cd2abb47d2421cff35381c1dac`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/helm/agones/templates/tests/test-runner.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0ef7dfaa550196402c.** [agones/install/helm/agones/templates/tests/test-runner.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/helm/agones/templates/tests/test-runner.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0b9833f7a99cf27d0a877c3ee4d8b9759a6362d8dc376294c03653824df657f8`.

> {{- if .Values.helm.installTests }}
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


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/terraform/modules/aks` -> module `install::terraform::modules::aks`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-5fa88ae09af1a51b"></a>


### File `install/terraform/modules/aks/aks.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `6929d225ea5c07c7e451cbfb92fa640349c17ca6`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/aks/aks.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-b5974c5855e7031202.** [agones/install/terraform/modules/aks/aks.tf, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/aks/aks.tf#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `53fdefd18e0a0f8e7e06eebef2a2d463cf1208c41c724acb7401c79f7d843789`.

> 
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     azurerm = {
>       source  = "hashicorp/azurerm"
>       version = "~> 2.66"
>     }
>   }
> }
> 
> provider "azurerm" {
>   features {}
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ac9e5490a81c7258"></a>


### File `install/terraform/modules/aks/outputs.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `c33da122fa6ce99988b182456afd8a2d1275a7b1`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/aks/outputs.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ad6afc36531d487398.** [agones/install/terraform/modules/aks/outputs.tf, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/aks/outputs.tf#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c3a2e1ef3760c60c9104cbb3d2ef95b83c88dacea880ccdb5bdac6844e03ce85`.

> 
> output "cluster_ca_certificate" {
>   value = base64decode(azurerm_kubernetes_cluster.agones.kube_config.0.cluster_ca_certificate)
>   depends_on = [
>     # Helm would be invoked only after all node pools would be created
>     # This way taints and tolerations for Agones controller would work properly
>     azurerm_kubernetes_cluster_node_pool.system,
>     azurerm_kubernetes_cluster_node_pool.metrics
>   ]
> }
> 
> output "client_certificate" {
>   value = azurerm_kubernetes_cluster.agones.kube_config.0.client_certificate
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ab16e50042fb599e"></a>


### File `install/terraform/modules/aks/variables.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `a04d903102d721f0d8a4cab04743803a5d642ffd`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/aks/variables.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8c0930a15a3731bae0.** [agones/install/terraform/modules/aks/variables.tf, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/aks/variables.tf#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3fccf9fdff07ff37a506a58d32927c6a9588586deac4e82140dc103ffee05dd5`.

> 
> variable "client_id" {
> }
> 
> variable "client_secret" {
> }
> 
> variable "cluster_name" {
>   default = "test-cluster"
> }
> 
> variable "disk_size" {
>   default = 30
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/terraform/modules/eks` -> module `install::terraform::modules::eks`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-363308e20300dffe"></a>


### File `install/terraform/modules/eks/eks.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `371a24abde8db8a8be16de393d6c0e29a045a450`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/eks/eks.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-11042e5be9ffc4c5d5.** [agones/install/terraform/modules/eks/eks.tf, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/eks/eks.tf#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `795aca15a8d519b217af21428dc5421b6abb8d2a2c7faf2ec30edff8583506b0`.

> 
> 
> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     aws = {
>       source  = "hashicorp/aws"
>       version = "~> 3.0"
>     }
>   }
> }
> 
> provider "aws" {
>   region  = var.region
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4230cdb1163d6b89"></a>


### File `install/terraform/modules/eks/outputs.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `140c8f2ecd10088da7082f80f1c34906eac8435f`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/eks/outputs.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-afd8d69edd4ac79f29.** [agones/install/terraform/modules/eks/outputs.tf, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/eks/outputs.tf#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `212f25d42759b6783a0ac3b19a69b7f5579531467ba1a9184efde8dda12078fa`.

> output "cluster_endpoint" {
>   description = "Endpoint for EKS control plane."
>   value       = module.eks.cluster_endpoint
> }
> 
> output "cluster_security_group_id" {
>   description = "Security group ids attached to the cluster control plane."
>   value       = module.eks.cluster_security_group_id
> }
> 
> output "kubectl_config" {
>   description = "kubectl config as generated by the module."
>   value       = module.eks.kubeconfig
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-3e6978d48513db12"></a>


### File `install/terraform/modules/eks/variables.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `73e1472cdfffc77652f9a29dd7fc4d6ce3b47b23`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/eks/variables.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c3b05f3aa70be4354f.** [agones/install/terraform/modules/eks/variables.tf, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/eks/variables.tf#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `9aab5083d7324b7ff0adb0d754f15c8eb6d144585ebf905f18ffdcd4a326386e`.

> 
> variable "cluster_name" {
>   default = "test-cluster"
> }
> 
> variable "region" {
>   default = "us-west-2"
> }
> 
> variable "machine_type" {
>   default = "t2.large"
> }
> 
> variable "node_count" {
>   default = "4"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/terraform/modules/gke` -> module `install::terraform::modules::gke`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-32027a4d5d405984"></a>


### File `install/terraform/modules/gke/cluster.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3c167ad9f1f7f8a08eef442dc1fd7fb52b97091d`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/gke/cluster.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-959dace208cf9634e6.** [agones/install/terraform/modules/gke/cluster.tf, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/gke/cluster.tf#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `c95f219aa8ff614b8ab1d7c2165016eff635949aebe3b54b2fa9ebb4335e387a`.

> 
> 
> terraform {
>   required_version = ">= 1.0.0"
> }
> 
> data "google_client_config" "default" {}
> 
> # A list of all parameters used in interpolation var.cluster
> # Set values to default if not key was not set in original map
> locals {
>   project                       = lookup(var.cluster, "project", "agones")
>   location                      = lookup(var.cluster, "location", "us-west1-c")
>   zone                          = lookup(var.cluster, "zone", "")
>   name                          = lookup(var.cluster, "name", "test-cluster")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-b5b9d385a6823363"></a>


### File `install/terraform/modules/gke/outputs.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2f2c087b46425f3c64ebddaf37b7e91bf148507a`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/gke/outputs.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-3fb373a10feba0dd77.** [agones/install/terraform/modules/gke/outputs.tf, lines 14–25](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/gke/outputs.tf#L14-L25). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7a67627b445aae2648bcc36e357e8b738c17d936272e58759df945985bc0f7fe`.

> 
> output "cluster_ca_certificate" {
>   value = base64decode(google_container_cluster.primary.master_auth.0.cluster_ca_certificate)
> }
> 
> output "host" {
>   value = "https://${google_container_cluster.primary.endpoint}"
> }
> 
> output "token" {
>   value = data.google_client_config.default.access_token
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-257d13f84acdc018"></a>


### File `install/terraform/modules/gke/variables.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `15d1ca750b7f9f63583099018cc422d330067f13`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/gke/variables.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-cfb32e0a1e235b697d.** [agones/install/terraform/modules/gke/variables.tf, lines 18–32](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/gke/variables.tf#L18-L32). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ecdd5e5dbbfe23cde6332af93efe532a9b1e3bd0265aeef0a31dc3f992a371e1`.

> variable "cluster" {
>   description = "Set of GKE cluster parameters."
>   type        = map(any)
> 
>   default = {
>     "location"                      = "us-west1-c"
>     "name"                          = "test-cluster"
>     "machineType"                   = "e2-standard-4"
>     "initialNodeCount"              = "4"
>     "project"                       = "agones"
>     "network"                       = "default"
>     "subnetwork"                    = ""
>     "releaseChannel"                = "UNSPECIFIED"
>     "kubernetesVersion"             = "1.35"
>     "windowsInitialNodeCount"       = "0"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/terraform/modules/gke-autopilot` -> module `install::terraform::modules::gke-autopilot`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-35544294abaaf72c"></a>


### File `install/terraform/modules/gke-autopilot/cluster.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d467532396304ea37912ac196e002fe2176c02b3`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/gke-autopilot/cluster.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-8296aafe51c63d404c.** [agones/install/terraform/modules/gke-autopilot/cluster.tf, lines 14–28](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/gke-autopilot/cluster.tf#L14-L28). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `6054edaa51d11c37b895ab7fd7d8bb97e263c7fd0694c83b8a3b9704c5bd7e1c`.

> 
> 
> terraform {
>   required_version = ">= 1.0.0"
> }
> 
> data "google_client_config" "default" {}
> 
> # A list of all parameters used in interpolation var.cluster
> # Set values to default if not key was not set in original map
> locals {
>   name                          = lookup(var.cluster, "name", "test-cluster")
>   project                       = lookup(var.cluster, "project", "agones")
>   location                      = lookup(var.cluster, "location", "us-west1")
>   network                       = lookup(var.cluster, "network", "default")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ab79f37e38ef4250"></a>


### File `install/terraform/modules/gke-autopilot/outputs.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2f2c087b46425f3c64ebddaf37b7e91bf148507a`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/gke-autopilot/outputs.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-43175ef65e123de043.** [agones/install/terraform/modules/gke-autopilot/outputs.tf, lines 14–25](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/gke-autopilot/outputs.tf#L14-L25). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `7a67627b445aae2648bcc36e357e8b738c17d936272e58759df945985bc0f7fe`.

> 
> output "cluster_ca_certificate" {
>   value = base64decode(google_container_cluster.primary.master_auth.0.cluster_ca_certificate)
> }
> 
> output "host" {
>   value = "https://${google_container_cluster.primary.endpoint}"
> }
> 
> output "token" {
>   value = data.google_client_config.default.access_token
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-4c99095003b347f1"></a>


### File `install/terraform/modules/gke-autopilot/variables.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `da6c3f5fc7819120c629ad7384f8860b93526ab7`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/gke-autopilot/variables.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-61f1e04f61bcc85597.** [agones/install/terraform/modules/gke-autopilot/variables.tf, lines 18–32](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/gke-autopilot/variables.tf#L18-L32). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `49af1e2339404604635002d9e79b595adfe29bcd60b023aa684e8c28cd7ab03c`.

> variable "cluster" {
>   description = "Set of GKE cluster parameters."
>   type        = map(any)
> 
>   default = {
>     "name"                          = "test-cluster"
>     "project"                       = "agones"
>     "location"                      = "us-west1"
>     "network"                       = "default"
>     "subnetwork"                    = ""
>     "releaseChannel"                = "REGULAR"
>     "kubernetesVersion"             = "1.35"
>     "deletionProtection"            = true
>     "maintenanceExclusionStartTime" = null
>     "maintenanceExclusionEndTime"   = null


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/terraform/modules/helm3` -> module `install::terraform::modules::helm3`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-8da1eb3d52b86251"></a>


### File `install/terraform/modules/helm3/helm.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `bb356667ba5cfd09fa666eeb662d8114cdb45e1c`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/helm3/helm.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-28ed8029a0e271fbd2.** [agones/install/terraform/modules/helm3/helm.tf, lines 18–32](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/helm3/helm.tf#L18-L32). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `4661775c8e235042ae6029e91892dc5e097f1599c08ac89a81a70d2a26d0ecce`.

> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     helm = {
>       version = "~> 2.3"
>       source  = "hashicorp/helm"
>     }
>   }
> }
> 
> provider "helm" {
>   kubernetes {
>     host                   = var.host
>     token                  = var.token
>     cluster_ca_certificate = var.cluster_ca_certificate


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-87bbad3a23483af5"></a>


### File `install/terraform/modules/helm3/variables.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f9b670aac5ffda796313aa5365997e87120283fa`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/helm3/variables.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-79531c72580a6416d6.** [agones/install/terraform/modules/helm3/variables.tf, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/helm3/variables.tf#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `14bf5567caeb3f88a5d47e71b8624a6378b90aa2f1de0f9ddf66832f07f807e7`.

> 
> variable "chart" {
>   default = "agones"
> }
> 
> variable "force_update" {
>   default = "true"
> }
> 
> variable "agones_version" {
>   default = ""
> }
> 
> variable "udp_expose" {
>   default = "true"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/terraform/modules/oke-helm3` -> module `install::terraform::modules::oke-helm3`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-cea17417f879b0b2"></a>


### File `install/terraform/modules/oke-helm3/helm.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `4a1d17d0033681e7f530fac7032a9b502a2fa388`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/oke-helm3/helm.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-2e429b39d6dae15b0b.** [agones/install/terraform/modules/oke-helm3/helm.tf, lines 18–32](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/oke-helm3/helm.tf#L18-L32). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ec15e51836de4e1770145b459c2e41e37474e6b8c09c04028185438e17cb1db0`.

> terraform {
>   required_version = ">= 1.0.0"
>   required_providers {
>     helm = {
>       version = "~> 2.3"
>       source  = "hashicorp/helm"
>     }
>   }
> }
> 
> provider "helm" {
>   kubernetes {
>     host                   = local.cluster_endpoint
>     cluster_ca_certificate = local.cluster_ca_certificate
>     insecure               = local.external_private_endpoint


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ecd3f5311133443f"></a>


### File `install/terraform/modules/oke-helm3/variables.tf`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b9fb750b5a71b778668f430df3315b2ba5ee6859`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/terraform/modules/oke-helm3/variables.tf`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-6f65efa35b9d03e1b8.** [agones/install/terraform/modules/oke-helm3/variables.tf, lines 16–30](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/terraform/modules/oke-helm3/variables.tf#L16-L30). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `21159c0f2624dd197580e5473419cf729237276e967846ef52ead45e15e593c0`.

> 
> variable "chart" {
>   default = "agones"
> }
> 
> variable "force_update" {
>   default = "true"
> }
> 
> variable "agones_version" {
>   default = ""
> }
> 
> variable "udp_expose" {
>   default = "true"


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:install/yaml` -> module `install::yaml`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-c28cd427601ef4cc08.** [agones/site/content/en/docs/Installation/Install Agones/helm.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Installation/Install%20Agones/helm.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `54d3c8a93b3fd19e98ec1e6f3d5e48176ee98ab693ac4a1f46091d6f28e02b3c`.

> 
> ## Prerequisites
> 
> - [Helm](https://helm.sh/) package manager 4.1.3+
> - [Supported Kubernetes Cluster]({{< relref "../_index.md#usage-requirements" >}})
> 
> 
> ## Helm 4
> 
> ### Installing the Chart
> 
> To install the chart with the release name `my-release` using our stable helm repository:


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-3f4efe7362fb9cc0"></a>


### File `install/yaml/install.yaml`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `b1635a11f002fbbeb3cd3783c910ed2395032186`. **Disposition:** `REQUIRED_BUILD_OR_COMPATIBILITY_ARTIFACT`. **Destination:** `agones-rs/install/yaml/install.yaml`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-145f62d0f7bf2e483b.** [agones/install/yaml/install.yaml, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/install/yaml/install.yaml#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `5147234532d408bc8ad9f64a2dccd06e79e6337a2e0191053171ef521fde9ec8`.

> ---
> # Source: agones/templates/priority-class.yaml
> apiVersion: scheduling.k8s.io/v1
> kind: PriorityClass
> metadata:
>   name: agones-system
> value: 1000000
> globalDefault: false
> description: "This priority class should be used for Agones service pods only."
> ---
> # Source: agones/templates/controller.yaml
> apiVersion: policy/v1
> kind: PodDisruptionBudget
> metadata:
>   name: agones-controller-pdb


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Preserve source floating width, narrowing, exceptional values and relevant bits.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
