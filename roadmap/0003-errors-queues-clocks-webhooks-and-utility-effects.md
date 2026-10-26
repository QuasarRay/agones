<a id="m0003"></a>


# M0003 — Errors, queues, clocks, webhooks and utility effects


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0003-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** The shared contract and pinned source inventory.


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Separate source utility algorithms from effects: error aggregation, log fields, filesystem notifications, HTTP/TLS servers, signal handling, admission hooks and work queues each expose the minimum CGP capability.

2. Preserve error classification, HTTP statuses, TLS material handling, queue retry/shutdown behavior, timer ownership, and callback ordering. Retain the source cancellation and cleanup path for every spawned operation.

3. Use derive-deftly for repeated effect adapters and ownership descriptors. Keep pure transformations short; a comprehension may not reorder side effects or erase errors. Do not run servers, TLS probes, or queue tests.


## Architecture, readability and authored outputs


1. Own 14 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:pkg/util` -> module `pkg::util`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-59ffe282f160cc39a9.** [agones/pkg/util/doc.go, lines 15–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/doc.go#L15-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a7302d530af170a17bd1cf523d7e0da006d8015e315577c0f7f5f75bd9459e57`.

> // Package util is for general libraries that aren't specific to Agones
> package util


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-941b04524a83dee2"></a>


### File `pkg/util/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `659827f246031538fc40abc173ba01f50772392b`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-fbb001b6388fe08fd8.** [agones/pkg/util/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a7302d530af170a17bd1cf523d7e0da006d8015e315577c0f7f5f75bd9459e57`.

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
> // Package util is for general libraries that aren't specific to Agones


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/apiserver` -> module `pkg::util::apiserver`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-5d3be22817b783ad"></a>


### File `pkg/util/apiserver/apiserver.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `84f47785c726ae04765999b31ad710413963a199`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/apiserver/apiserver.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-512e2171be18c3eba6.** [agones/pkg/util/apiserver/apiserver.go, lines 63–77](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/apiserver/apiserver.go#L63-L77). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `67e4b15b16088940677a899b870053ee32f7aaea40819bb5c8d059820b9f2a68`.

> 
> func init() {
> 	Scheme.AddUnversionedTypes(unversionedVersion, unversionedTypes...)
> }
> 
> // CRDHandler is a http handler, that gets passed the Namespace it's working
> // on, and returns an error if a server error occurs
> type CRDHandler func(http.ResponseWriter, *http.Request, string) error
> 
> // APIServer is a lightweight library for registering, and providing handlers
> // for Kubernetes APIServer extensions.
> type APIServer struct {
> 	logger             *logrus.Entry
> 	errs               *errors.Errors
> 	mux                *http.ServeMux


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 10 lexically identified declarations, including `init` (L64), `CRDHandler` (L70), `APIServer` (L74), `NewAPIServer` (L86), `AddAPIResource` (L137), `resourceHandler` (L165), `addSerializedHandler` (L189), `shallowCopyObjectForTargetKind` (L214), `AcceptedSerializer` (L224), `splitNameSpaceResource` (L245). The full declaration list is recorded under `agones:pkg/util/apiserver/apiserver.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/https`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/crd` -> module `pkg::util::crd`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-f026a87427e43436"></a>


### File `pkg/util/crd/crd.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `33c0fd1919e4383387842e9d8ea834fbda4e75a4`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/crd/crd.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-50b640cb99633f1433.** [agones/pkg/util/crd/crd.go, lines 29–43](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/crd/crd.go#L29-L43). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `a7b1fb654c5e00db12db06c524738ec0eb361c584f5bd6e09b27a3c043ab0bf0`.

> 
> // WaitForEstablishedCRD blocks until CRD comes to an Established state.
> // Has a deadline of 60 seconds for this to occur.
> func WaitForEstablishedCRD(_ context.Context, crdGetter apiextclientv1.CustomResourceDefinitionInterface, name string, logger *logrus.Entry) error {
> 	return wait.PollUntilContextTimeout(context.Background(), time.Second, 60*time.Second, true, func(ctx context.Context) (done bool, err error) {
> 		crd, err := crdGetter.Get(ctx, name, metav1.GetOptions{})
> 		if err != nil {
> 			return false, err
> 		}
> 
> 		for _, cond := range crd.Status.Conditions {
> 			if cond.Type == apiextv1.Established {
> 				if cond.Status == apiextv1.ConditionTrue {
> 					logger.WithField("crd", crd.ObjectMeta.Name).Debug("custom resource definition established")
> 					return true, err


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `WaitForEstablishedCRD` (L32). The full declaration list is recorded under `agones:pkg/util/crd/crd.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/errors` -> module `pkg::util::errors`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-8b4a6c6c3145bf8a"></a>


### File `pkg/util/errors/errors.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ff3c173b9b8130352a16050fbd6f2a5a0dae9e1c`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/errors/errors.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-89626833087806b4e1.** [agones/pkg/util/errors/errors.go, lines 52–66](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/errors/errors.go#L52-L66). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b9b811509cc02053ce498cf012a58ea6fabb44d6fbf69e355d39baf0504dbc29`.

> 
> // Errors creates errors prefixed with their origin, in the form "package: msg",
> // or "package.Struct: msg" when created from a struct. Create one with
> // FromPackage or FromStruct.
> //
> // The zero value is usable, and produces errors without a prefix.
> type Errors struct {
> 	// pkg is the full import path, e.g. "agones.dev/agones/pkg/gameservers".
> 	pkg string
> 	// structName is the struct name, e.g. "Controller". Empty when created via FromPackage.
> 	structName string
> }
> 
> // FromPackage returns an Errors for the package it is called from, determined
> // by inspecting the calling function at runtime. It is intended to be assigned


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 10 lexically identified declarations, including `Errors` (L58), `FromPackage` (L70), `FromStruct` (L87), `New` (L104), `Errorf` (L110), `Wrap` (L118), `Wrapf` (L129), `prefix` (L138), `callerPackage` (L154), `packageFromFunc` (L173). The full declaration list is recorded under `agones:pkg/util/errors/errors.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/fswatch` -> module `pkg::util::fswatch`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ba48ee62a08dd4fe"></a>


### File `pkg/util/fswatch/fswatch.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7c46058c214f79361f457461f2b57df9a4b89cf1`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/fswatch/fswatch.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-ace4c719901a99301c.** [agones/pkg/util/fswatch/fswatch.go, lines 24–38](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/fswatch/fswatch.go#L24-L38). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `23ac1767229cb4699c1e7e1bd41c4be17de45088573a3a8f00bfc9ff9de33d95`.

> 
> // Watch watches the filesystem path `path`. When anything changes, changes are
> // batched for the period `batchFor`, then `processEvent` is called.
> //
> // Returns a cancel() function to terminate the watch.
> func Watch(logger *logrus.Entry, path string, batchFor time.Duration, processEvent func()) (func(), error) {
> 	logger = logger.WithField("path", path)
> 	watcher, err := fsnotify.NewWatcher()
> 	if err != nil {
> 		return nil, err
> 	}
> 	cancelChan := make(chan struct{})
> 	cancel := func() {
> 		close(cancelChan)
> 		_ = watcher.Close()


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 2 lexically identified declarations, including `Watch` (L29), `batchWatch` (L53). The full declaration list is recorded under `agones:pkg/util/fswatch/fswatch.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve channel/select outcomes and blocking/cancellation semantics; do not turn nondeterminism into fixed ordering.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/https` -> module `pkg::util::https`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-363eb9afcbcf4eeb"></a>


### File `pkg/util/https/https.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `f5ca0c6c16c5c5a24fdac6a0b6373a390e9f385f`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/https/https.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-c443a11d52d02042a2.** [agones/pkg/util/https/https.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/https/https.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `feaf1a8c1d5f3d8933a5752142bc574a6d28f15430ecf1f87a3e657e59223a65`.

> 
> // ErrorHandlerFunc is a http handler that can return an error
> // for standard logging and a 500 response
> type ErrorHandlerFunc func(http.ResponseWriter, *http.Request) error
> 
> // sensitiveHeaders lists, in canonical form, the HTTP headers whose values
> // must never be written to logs verbatim, since they routinely carry
> // credentials such as bearer tokens or API keys.
> var sensitiveHeaders = map[string]bool{
> 	"Authorization":       true,
> 	"Proxy-Authorization": true,
> 	"Cookie":              true,
> 	"Set-Cookie":          true,
> 	"X-Api-Key":           true,
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 7 lexically identified declarations, including `errs` (L27), `ErrorHandlerFunc` (L31), `sensitiveHeaders` (L36), `redactSensitiveHeaders` (L48), `FourZeroFour` (L61), `ErrorHTTPHandler` (L80), `LogRequest` (L93). The full declaration list is recorded under `agones:pkg/util/https/https.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-ab123b2c1f6ea4c6"></a>


### File `pkg/util/https/server.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `ddaa46a85d70a4109835dd9fd41302de36599709`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/https/server.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-0faeb549944e5c5094.** [agones/pkg/util/https/server.go, lines 38–52](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/https/server.go#L38-L52). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ba57ba27151f223eabc321c83e295449be5d8d15ba6d0ccf53472cd405e6b951`.

> 
> // tls is a http server interface to enable easier testing
> type tls interface {
> 	Shutdown(context.Context) error
> 	ListenAndServeTLS(certFile, keyFile string) error
> }
> 
> // certServer holds the Server certificate
> type certServer struct {
> 	certs  *cryptotls.Certificate
> 	certMu sync.Mutex
> }
> 
> // Server is a HTTPs server that conforms to the runner interface
> // we use in /cmd/controller, and has a public Mux that can be updated


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 9 lexically identified declarations, including `tls` (L40), `certServer` (L46), `Server` (L54), `NewServer` (L66), `setupServer` (L83), `getCertificate` (L105), `WatchForCertificateChanges` (L112), `Run` (L136), `defaultHandler` (L159). The full declaration list is recorded under `agones:pkg/util/https/server.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/fswatch`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/httpserver` -> module `pkg::util::httpserver`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-3db54e9b8050ecaf"></a>


### File `pkg/util/httpserver/server.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `72b790615ed7744904062fe3f0bc62d468c253c6`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/httpserver/server.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a758de3a6f0285a54b.** [agones/pkg/util/httpserver/server.go, lines 34–48](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/httpserver/server.go#L34-L48). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `0f9b972fc678b0f33c7531a8b19c08a9ffb54ab745014ed800eff19b4157102b`.

> 
> // Server is a HTTPs server that conforms to the runner interface
> // we use in /cmd/controller.
> //
> //nolint:govet // ignore field alignment complaint, this is a singleton
> type Server struct {
> 	http.ServeMux
> 	Port string
> 
> 	Logger *logrus.Entry
> }
> 
> // Run runs an http server on port :8080.
> func (s *Server) Run(ctx context.Context, _ int) error {
> 	s.Logger.Info("Starting http server...")


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 4 lexically identified declarations, including `readHeaderTimeout` (L31), `errs` (L33), `Server` (L39), `Run` (L47). The full declaration list is recorded under `agones:pkg/util/httpserver/server.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/logfields` -> module `pkg::util::logfields`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-74c1cd41fdc48446"></a>


### File `pkg/util/logfields/logfields.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `5ff6d059e60ec57af63763886d954c68cb344227`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/logfields/logfields.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-9b2965535baa80fe47.** [agones/pkg/util/logfields/logfields.go, lines 26–40](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/logfields/logfields.go#L26-L40). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `355671ba746083a3ac152f580af67b7c65ad82fd6568f88b1fa41d36ed1728ba`.

> 
> // ResourceType identifies the type of a resource for the purpose of putting it in structural logs.
> type ResourceType string
> 
> // Identifiers used in logs for identifying Agones objects.
> const (
> 	GameServerKey           ResourceType = "gsKey"
> 	GameServerSetKey        ResourceType = "gssKey"
> 	GameServerAllocationKey ResourceType = "gsaKey"
> 	FleetKey                ResourceType = "fleetKey"
> 	FleetAutoscalerKey      ResourceType = "fasKey"
> )
> 
> // AugmentLogEntry creates derived log entry with a given resource identifier ("namespace/name")
> func AugmentLogEntry(base *logrus.Entry, resourceType ResourceType, resourceID string) *logrus.Entry {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 2 lexically identified declarations, including `ResourceType` (L28), `AugmentLogEntry` (L40). The full declaration list is recorded under `agones:pkg/util/logfields/logfields.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/runtime` -> module `pkg::util::runtime`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-ce68055eacd45678"></a>


### File `pkg/util/runtime/features.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `2103404a206101e55d23d102292adbffb04f9942`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/runtime/features.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-255a26588ddb612ec4.** [agones/pkg/util/runtime/features.go, lines 190–204](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/runtime/features.go#L190-L204). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `05736b26646cf219419626dfca37c4f533302b178ca3bb542330cf7884ddc679`.

> 
> // Feature is a type for defining feature gates.
> type Feature string
> 
> // FeaturesBindFlags does the Viper arguments configuration. Call before running pflag.Parse()
> func FeaturesBindFlags() {
> 	viper.SetDefault(FeatureGateFlag, "")
> 	pflag.String(FeatureGateFlag, viper.GetString(FeatureGateFlag), "Flag to pass in the url query list of feature flags to enable or disable")
> }
> 
> // FeaturesBindEnv binds the environment variables, based on the flags provided.
> // call after viper.SetEnvKeyReplacer(...) if it is being set.
> func FeaturesBindEnv() error {
> 	return viper.BindEnv(FeatureGateFlag)
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 8 lexically identified declarations, including `Feature` (L192), `FeaturesBindFlags` (L195), `FeaturesBindEnv` (L202), `ParseFeaturesFromEnv` (L208), `ParseFeatures` (L214), `EnableAllFeatures` (L249), `FeatureEnabled` (L263), `EncodeFeatures` (L270). The full declaration list is recorded under `agones:pkg/util/runtime/features.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-7ee39c4411befec4"></a>


### File `pkg/util/runtime/runtime.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `7581ddc41945340b97d4561a275bf969a32e4f8f`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/runtime/runtime.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-fac3dc835c54f17d73.** [agones/pkg/util/runtime/runtime.go, lines 36–50](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/runtime/runtime.go#L36-L50). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `d1683d9c5931891d59ae46cbfdbce13b45d2836355097e7e1bd24fd30200271a`.

> 
> // replace the standard glog error logger, with a logrus one
> func init() {
> 	logrus.SetFormatter(&logrus.JSONFormatter{
> 		TimestampFormat: time.RFC3339Nano,
> 		FieldMap: logrus.FieldMap{
> 			logrus.FieldKeyTime:  "time",
> 			logrus.FieldKeyLevel: "severity",
> 			logrus.FieldKeyMsg:   "message",
> 		},
> 	})
> 
> 	runtime.ErrorHandlers[0] = func(_ context.Context, err error, _ string, _ ...any) {
> 		logrus.Error(err)
> 	}


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 10 lexically identified declarations, including `errs` (L33), `sourceKey` (L35), `init` (L38), `SetLevel` (L54), `HandleError` (L60), `Must` (L69), `NewLoggerWithSource` (L76), `NewLoggerWithType` (L82), `NewServerMux` (L89), `InClusterBuildConfig` (L111). The full declaration list is recorded under `agones:pkg/util/runtime/runtime.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve source panic/recover boundaries; do not use an unsupported panic as a translation placeholder.

5. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/signals` -> module `pkg::util::signals`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-7aa019ab7188d7f0"></a>


### File `pkg/util/signals/signals.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `db00f66bfc48af4148a32f13bd653e1d443e2255`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/signals/signals.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-e038b28fd2d8f91e89.** [agones/pkg/util/signals/signals.go, lines 25–39](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/signals/signals.go#L25-L39). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `bddfa941f5235d32ccf6909f211caf491cd536b3e1920a16473430ec1e51e418`.

> 
> // NewSigKillContext returns a Context that cancels when os.Interrupt or os.Kill is received
> // along with a stop function that can be used to unregister the signal behavior.
> func NewSigKillContext() (context.Context, context.CancelFunc) {
> 	return signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
> }
> 
> // NewSigTermHandler creates a channel to listen to SIGTERM and runs the handle function
> func NewSigTermHandler(handle func()) {
> 	c := make(chan os.Signal, 1)
> 	signal.Notify(c, syscall.SIGTERM, syscall.SIGINT)
> 
> 	go func() {
> 		<-c
> 		handle()


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 2 lexically identified declarations, including `NewSigKillContext` (L28), `NewSigTermHandler` (L33). The full declaration list is recorded under `agones:pkg/util/signals/signals.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

4. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

5. Preserve channel/select outcomes and blocking/cancellation semantics; do not turn nondeterminism into fixed ordering.

6. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/webhooks` -> module `pkg::util::webhooks`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-e53796b9dd81500c"></a>


### File `pkg/util/webhooks/webhooks.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1995f206ca2d186af3988703fbe9abda8ef86fe7`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/webhooks/webhooks.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-34db3b341ffeb4daf9.** [agones/pkg/util/webhooks/webhooks.go, lines 29–43](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/webhooks/webhooks.go#L29-L43). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b3a2c84309dae0dab131cac8bb3a1e8798824333ba063f6bb1a388bbeba54673`.

> 
> // WebHook manage Kubernetes webhooks
> type WebHook struct {
> 	logger   *logrus.Entry
> 	errs     *errors.Errors
> 	mux      *http.ServeMux
> 	handlers map[string][]operationHandler
> }
> 
> // operationHandler stores the data for a handler to match against
> type operationHandler struct {
> 	handler   Handler
> 	groupKind schema.GroupKind
> 	operation admissionv1.Operation
> }


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 6 lexically identified declarations, including `WebHook` (L31), `operationHandler` (L39), `Handler` (L47), `NewWebHook` (L50), `AddHandler` (L62), `handle` (L77). The full declaration list is recorded under `agones:pkg/util/webhooks/webhooks.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


## Package `agones:pkg/util/workerqueue` -> module `pkg::util::workerqueue`


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-7a4fa54738e0818f1b.** [agones/site/content/en/docs/Advanced/high-availability-agones.md, lines 9–21](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Advanced/high-availability-agones.md#L9-L21). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `b674bceb41ed3c9d26ca8637afc39cf67df131b785946863201f3c9bbfe41e5d`.

> 
> 
> ## High Availability for Agones Controller
> 
> 
> The `agones-controller` responsibility is split up into `agones-controller`, which enacts the Agones control loop, and `agones-extensions`, which acts as a service endpoint for webhooks and the allocation extension API. Splitting these responsibilities allows the `agones-extensions` pod to be **horizontally scaled**, making the Agones control plane **highly available** and more **resilient to disruption**.
> 
> Multiple `agones-controller` pods enabled, with a primary controller selected via leader election. Having multiple `agones-controller` minimizes downtime of the service from pod disruptions such as deployment updates, autoscaler evictions, and crashes.
> 
> ## Extension Pod Configurations 
> 
> The `agones-extensions` binary has a similar `helm` configuration to `agones-controller`, see [here]({{< relref "/docs/Installation/Install Agones/helm.md" >}}). If you previously overrode `agones.controller.*` settings, you may need to override the same `agones.extensions.*` setting.


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-32753297d4db115d"></a>


### File `pkg/util/workerqueue/workerqueue.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `98b8b2431036013b0764f1cf4be2c30b26db99e9`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/util/workerqueue/workerqueue.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-a23329f903d9600cb3.** [agones/pkg/util/workerqueue/workerqueue.go, lines 40–54](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/util/workerqueue/workerqueue.go#L40-L54). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `2b4fb5a73f89aef140f6ba910b7adf8faf49c11ec34bcd20870ba14d512a05a8`.

> 
> // traceError is a marker type for errors that that should only be logged at a Trace level.
> // Useful if you want a Handler to be retried, but not logged at an Error level.
> type traceError struct {
> 	err error
> }
> 
> // NewTraceError returns a traceError wrapper around an error.
> func NewTraceError(err error) error {
> 	return &traceError{err: err}
> }
> 
> // Error returns the error string
> func (l *traceError) Error() string {
> 	if l.err == nil {


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 22 lexically identified declarations, including `traceError` (L43), `NewTraceError` (L48), `Error` (L53), `isTraceError` (L61), `Handler` (L68), `WorkerQueue` (L75), `FastRateLimiter` (L89), `ConstantRateLimiter` (L97), `NewWorkerQueue` (L105), `NewWorkerQueueWithRateLimiter` (L110), `Enqueue` (L124), `EnqueueImmediately` (L139), `EnqueueAfter` (L152), `runWorker` (L168), `processNextWorkItem` (L175), `Run` (L213), `run` (L225), `Healthy` (L232), `RunCount` (L245), `setWorkerCount` (L251), `inc` (L257), `dec` (L263). The full declaration list is recorded under `agones:pkg/util/workerqueue/workerqueue.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

7. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
