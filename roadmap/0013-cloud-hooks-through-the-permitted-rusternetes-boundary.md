<a id="m0013"></a>


# M0013 — Cloud hooks through the permitted Rusternetes boundary


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0013-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0011](0011-port-ownership-and-allocation-policy.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-252126bb5962982bee.** [agones/site/content/en/docs/Advanced/controlling-disruption.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/controlling-disruption.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e099ae789c0faba275ee0f35b86a6fa0f646a81b62879e139c47d9ee8c300ee9`.

> 
> ## Disruption in Kubernetes
> 
> [A `Pod` in Kubernetes may be disrupted](https://kubernetes.io/docs/concepts/workloads/pods/disruptions/#voluntary-and-involuntary-disruptions) for involuntary reasons, e.g. hardware failure, or voluntary reasons, such as when nodes are drained for upgrades. 
> 
> By default, Agones assumes your game server should never be disrupted voluntarily and configures the `Pod` appropriately - but this isn't always the ideal setting. Here we discuss how Agones allows you to control the two most significant sources of voluntary `Pod` evictions, node upgrades and Cluster Autoscaler, using the `eviction` API on the `GameServer` object. 
> 
> ## Benefits of Allowing Voluntary Disruption
> 
> It's not always easy to write your game server in a way that allows for disruption, but it can have major benefits:
> 
> * Compaction of your cluster using [Cluster Autoscaler](https://github.com/kubernetes/autoscaler/tree/master/cluster-autoscaler) can lead to considerable cost savings for your infrastructure.
> * Allowing automated node upgrades can save you management toil, and lowers the time it takes to patch security vulnerabilites.


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Separate generic cloud-product policy, eviction behavior and GKE-specific source assumptions by exact symbols. Retain required observable hooks, labels, annotations and shutdown behavior through the Rusternetes context.

2. Preserve source GKE paths and account for their behavior without installing a GKE/Kubernetes integration or inventing an alternative platform. Record an explicit conflict if an indispensable source effect cannot fit the permitted adapter.

3. Compose hook behavior through CGP and derive-deftly ownership declarations. Neither a neutral name nor an unused provider establishes portability or completion.


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


## Package `agones:pkg/cloudproduct` -> module `pkg::cloudproduct`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-027ee0a7b2adf95193.** [agones/pkg/cloudproduct/doc.go, lines 15–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/doc.go#L15-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3304582e8475cba165f11c37a50748583d60b709201db52c19371ea7874a80af`.

> // Package cloudproduct provides an abstraction layer to product specific functionality
> package cloudproduct


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-80c980ae7da9026f"></a>


### File `pkg/cloudproduct/cloudproduct.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `edd81750c6683771d0ef0968be0f22a3d0eb264c`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/cloudproduct/cloudproduct.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-6d283a665143a6641d.** [agones/pkg/cloudproduct/cloudproduct.go, lines 33–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/cloudproduct.go#L33-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `549a3913205c8fa42a9d7526d3d924f8e7ce95edda4defcceef2341045d9ab33`.

> 
> // The Cloud Product abstraction currently consists of disjoint interfaces:
> // * ControllerHooks() is an interface available to the pkg/controller binaries
> // * api hooks are separated into api group and registered directly with the API
> //
> // Users should bind flags with Bind{Flags,Env}, then call Initialize with a
> // k8s client to initialize.
> 
> // ControllerHooksInterface provides a generic interface that abstracts cloud product
> // specific functionality for pkg/controller packages.
> type ControllerHooksInterface interface {
> 	agonesv1.APIHooks
> 
> 	// SyncPodPortsToGameServer runs after a Pod has been assigned to a Node and before we sync
> 	// Pod host ports to the GameServer status.
> 	SyncPodPortsToGameServer(*agonesv1.GameServer, *corev1.Pod) error
> 
> 	// NewPortAllocator creates a PortAllocator. See gameservers.NewPortAllocator for parameters.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 5 lexically identified declarations, including `ControllerHooksInterface` (L43), `BindFlags` (L76), `BindEnv` (L82), `NewFromFlag` (L87), `autoDetect` (L99). The full declaration list is recorded under `agones:pkg/cloudproduct/cloudproduct.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/cloudproduct/generic`, `agones.dev/agones/pkg/cloudproduct/gke`, `agones.dev/agones/pkg/portallocator`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-a9a83cfd9489dba5"></a>


### File `pkg/cloudproduct/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `3e412f6e2af444f506d8cd56d2736385ecc97b5b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/cloudproduct/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-677bfcd8991d05db2b.** [agones/pkg/cloudproduct/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `3304582e8475cba165f11c37a50748583d60b709201db52c19371ea7874a80af`.

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
> // Package cloudproduct provides an abstraction layer to product specific functionality


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/cloudproduct/eviction` -> module `pkg::cloudproduct::eviction`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-252126bb5962982bee.** [agones/site/content/en/docs/Advanced/controlling-disruption.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/controlling-disruption.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e099ae789c0faba275ee0f35b86a6fa0f646a81b62879e139c47d9ee8c300ee9`.

> 
> ## Disruption in Kubernetes
> 
> [A `Pod` in Kubernetes may be disrupted](https://kubernetes.io/docs/concepts/workloads/pods/disruptions/#voluntary-and-involuntary-disruptions) for involuntary reasons, e.g. hardware failure, or voluntary reasons, such as when nodes are drained for upgrades. 
> 
> By default, Agones assumes your game server should never be disrupted voluntarily and configures the `Pod` appropriately - but this isn't always the ideal setting. Here we discuss how Agones allows you to control the two most significant sources of voluntary `Pod` evictions, node upgrades and Cluster Autoscaler, using the `eviction` API on the `GameServer` object. 
> 
> ## Benefits of Allowing Voluntary Disruption
> 
> It's not always easy to write your game server in a way that allows for disruption, but it can have major benefits:
> 
> * Compaction of your cluster using [Cluster Autoscaler](https://github.com/kubernetes/autoscaler/tree/master/cluster-autoscaler) can lead to considerable cost savings for your infrastructure.
> * Allowing automated node upgrades can save you management toil, and lowers the time it takes to patch security vulnerabilites.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ee3889cdc666a4aa"></a>


### File `pkg/cloudproduct/eviction/eviction.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `743ca77d883427a73d10d4c3407920cbd541cdac`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/cloudproduct/eviction/eviction.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-10cc12f8458118e4ea.** [agones/pkg/cloudproduct/eviction/eviction.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/eviction/eviction.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `528d24cc7fce1e737f3538d2925f9711e97b17380eeb8fe62c1466ad6a44f0a7`.

> 
> // SetEviction sets disruptions controls on a Pod based on GameServer.Status.Eviction.
> func SetEviction(eviction *agonesv1.Eviction, pod *corev1.Pod) error {
> 	if eviction == nil {
> 		return errs.New("No eviction value set. Should be the default value")
> 	}
> 	if _, exists := pod.ObjectMeta.Annotations[agonesv1.PodSafeToEvictAnnotation]; !exists {
> 		switch eviction.Safe {
> 		case agonesv1.EvictionSafeAlways:
> 			pod.ObjectMeta.Annotations[agonesv1.PodSafeToEvictAnnotation] = agonesv1.True
> 		case agonesv1.EvictionSafeOnUpgrade, agonesv1.EvictionSafeNever:
> 			// For EvictionSafeOnUpgrade and EvictionSafeNever, we block Cluster Autoscaler
> 			// (on Autopilot, this enables Extended Duration pods, which is equivalent).
> 			pod.ObjectMeta.Annotations[agonesv1.PodSafeToEvictAnnotation] = agonesv1.False
> 		default:


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 2 lexically identified declarations, including `errs` (L24), `SetEviction` (L27). The full declaration list is recorded under `agones:pkg/cloudproduct/eviction/eviction.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/util/errors`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/cloudproduct/generic` -> module `pkg::cloudproduct::generic`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-0c84a486bf1e867dba.** [agones/pkg/cloudproduct/generic/doc.go, lines 15–17](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/generic/doc.go#L15-L17). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e59ec8a3e3d51804c5307c8f932595f9447ac31855b4dcfe9f6d20173ad6bd0d`.

> // Package generic provides the generic cloud provider. The generic cloud provider
> // should be usable on all clouds, but provides no special functionality.
> package generic


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-8815d8114afea5a8"></a>


### File `pkg/cloudproduct/generic/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `d915e05810d4ce93d0bc0885155e623d6c35b941`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/cloudproduct/generic/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-ff3ea08ec3a5200d86.** [agones/pkg/cloudproduct/generic/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/generic/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `e59ec8a3e3d51804c5307c8f932595f9447ac31855b4dcfe9f6d20173ad6bd0d`.

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
> // Package generic provides the generic cloud provider. The generic cloud provider


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-6831449d40dfb7d4"></a>


### File `pkg/cloudproduct/generic/generic.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `796ce1b49f8e9dbc2054301e8c573aeec33570f8`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/cloudproduct/generic/generic.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-f82e120cb5abd97585.** [agones/pkg/cloudproduct/generic/generic.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/generic/generic.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `8446276333915038492f542ebd5ec80a98e7dee2306a267926f29f0788bc627d`.

> 
> // New returns a new generic cloud product
> //
> //nolint:revive // ignore the unexported return; implements ControllerHooksInterface
> func New() *generic { return &generic{} }
> 
> type generic struct{}
> 
> func (*generic) ValidateGameServerSpec(*agonesv1.GameServerSpec, *field.Path) field.ErrorList {
> 	return nil
> }
> func (*generic) ValidateScheduling(apis.SchedulingStrategy, *field.Path) field.ErrorList { return nil }
> func (*generic) MutateGameServerPod(*agonesv1.GameServerSpec, *corev1.Pod) error         { return nil }
> 
> // SetEviction sets disruptions controls based on GameServer.Status.Eviction.


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 9 lexically identified declarations, including `New` (L32), `generic` (L34), `ValidateGameServerSpec` (L36), `ValidateScheduling` (L39), `MutateGameServerPod` (L40), `SetEviction` (L43), `SyncPodPortsToGameServer` (L47), `NewPortAllocator` (L49), `WaitOnFreePorts` (L55). The full declaration list is recorded under `agones:pkg/cloudproduct/generic/generic.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/cloudproduct/eviction`, `agones.dev/agones/pkg/portallocator`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/cloudproduct/gke` -> module `pkg::cloudproduct::gke`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-81f0e0d9c1b92c54ad.** [agones/pkg/cloudproduct/gke/doc.go, lines 15–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/gke/doc.go#L15-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `00ac7513e690ffb3529c55b1ac125673411cf6953903283cf3d368b87e8fbe38`.

> // Package gke provides the GKE cloud product interfaces for GKE Standard and Autopilot
> package gke


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-a7a76c630fb37666"></a>


### File `pkg/cloudproduct/gke/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7510fdc18769f3cf3df6102a9ab76db1265af7bd`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/cloudproduct/gke/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0002168fc6cec993c2.** [agones/pkg/cloudproduct/gke/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/gke/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `00ac7513e690ffb3529c55b1ac125673411cf6953903283cf3d368b87e8fbe38`.

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
> // Package gke provides the GKE cloud product interfaces for GKE Standard and Autopilot


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-055828963a3f93ef"></a>


### File `pkg/cloudproduct/gke/gke.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `26bad0bb387ff99ce1ef4eac617f9777b212fa0c`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/cloudproduct/gke/gke.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-0cd0ca67fc54159228.** [agones/pkg/cloudproduct/gke/gke.go, lines 57–71](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/cloudproduct/gke/gke.go#L57-L71). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `fea226c3547b17154a44fad5354eca22f8502df9bf57105a2a4da3182fa26960`.

> 
> type gkeAutopilot struct {
> 	useExtendedDurationPods bool
> }
> 
> // hostPortAssignment is the JSON structure of the `host-port-assignment` annotation
> //
> //nolint:govet // API-like, keep consistent
> type hostPortAssignment struct {
> 	Min           int32           `json:"min,omitempty"`
> 	Max           int32           `json:"max,omitempty"`
> 	PortsAssigned map[int32]int32 `json:"portsAssigned,omitempty"` // old -> new
> }
> 
> // Detect whether we're running on GKE and/or Autopilot and return the appropriate


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 21 lexically identified declarations, including `gkeAutopilot` (L58), `hostPortAssignment` (L65), `Detect` (L73), `Autopilot` (L100), `SyncPodPortsToGameServer` (L104), `NewPortAllocator` (L127), `WaitOnFreePorts` (L135), `ValidateGameServerSpec` (L137), `ValidateScheduling` (L153), `MutateGameServerPod` (L162), `setPassthroughLabel` (L172), `setPrimaryContainer` (L182), `podSpecSeccompUnconfined` (L193), `SetEviction` (L203), `setEvictionNoExtended` (L217), `hasPortPolicy` (L241), `autopilotPortAllocator` (L250), `Run` (L255), `DeAllocate` (L256), `Allocate` (L258), `validatePortPolicy` (L303). The full declaration list is recorded under `agones:pkg/cloudproduct/gke/gke.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/cloudproduct/eviction`, `agones.dev/agones/pkg/portallocator`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
