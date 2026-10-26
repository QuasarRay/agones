<a id="m0025"></a>


# M0025 — SDK server streaming, health and mutation behavior


**Route:** `MANUAL`. **Phase:** `IMPLEMENTATION_ONLY`. **State:** `PLANNED_NOT_EXECUTED`. **Forecast:** `F-M0025-v1` in [cost-analysis.md](../cost-analysis.md).


**Prerequisites:** [M0023](0023-allocation-processing-and-multi-cluster-contracts.md), [M0005](0005-resource-and-wire-contracts-defaults-and-validation.md), [M0007](0007-apply-clients-informers-and-listers.md), [M0003](0003-errors-queues-clocks-webhooks-and-utility-effects.md)


## Source contract and interpretation


**Direct quotation — UPSTREAM_DOCUMENTATION; Q-a8ca826b198db6b4b7.** [agones/site/content/en/docs/Guides/Client SDKs/_index.md, lines 8–20](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/site/content/en/docs/Guides/Client%20SDKs/_index.md#L8-L20). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `ef2b25e8d7f0f1c99d8f570fdcd5768b9bedf01e78a70b30cd0283220122129b`.

> 
> ## Overview
> 
> The client SDKs are required for a game server to work with Agones.
> 
> The current supported SDKs are:
> 
> - [Unreal Engine]({{< relref "unreal.md" >}})
> - [Unity]({{< relref "unity.md" >}})
> - [C++]({{< relref "cpp.md" >}})
> - [C#]({{< relref "csharp.md" >}})
> - [Node.js]({{< relref "nodejs.md" >}})
> - [Go]({{< relref "go.md" >}})


**Derived instruction.** Implement the source obligations below through the shared CGP AND derive-deftly architecture. Direct quotations are reference data; this roadmap’s user-authorized phase and integration constraints govern execution.


## Implementation sequence


1. Implement SDK server RPCs, local mode, health signals, Ready/Allocated/Shutdown/Reserve behavior, WatchGameServer streaming, metadata updates and alpha/beta/stable counters/lists/player contracts.

2. Preserve source update ordering, retries, validation, status codes, stream cancellation, disconnect behavior and shutdown. Every stream/task must have a CGP owner and join/drain policy.

3. Use derive-deftly for repeated service/message/wiring adapters. Keep the ordinary operation path short enough to read as request, decision, mutation, notification and cleanup.


## Architecture, readability and authored outputs


1. Own 4 inventoried file entries in this milestone. Every required file below has a unique milestone owner.

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


## Package `agones:pkg/sdkserver` -> module `pkg::sdkserver`


**Direct quotation — UPSTREAM_DOCUMENTATION_COMMENT; Q-e0226f2ed2685859a4.** [agones/pkg/sdkserver/doc.go, lines 15–16](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdkserver/doc.go#L15-L16). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `78e101f92b67663b1725a78a147735aebc40b8f6bb48fa90e04d7f261352073f`.

> // Package sdkserver implements Agones SDK server (sidecar).
> package sdkserver


**Derived package instruction.** Implement all owned files listed below as one source package/module contract. Preserve initialization and visibility boundaries, source external contracts, and exact module/file spelling. Put variation in CGP capabilities and derive-deftly templates, not duplicate handwritten wiring. Explain the package’s input -> decision -> effect -> output flow for a Python developer. The quoted package/topic contract and each file’s source excerpt jointly define this work.


<a id="file-eb4dec6d850039b0"></a>


### File `pkg/sdkserver/doc.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `e76c23f8c032187822733b44537b030d28c2023e`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/sdkserver/doc.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c245a13df5e138d338.** [agones/pkg/sdkserver/doc.go, lines 1–15](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdkserver/doc.go#L1-L15). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `78e101f92b67663b1725a78a147735aebc40b8f6bb48fa90e04d7f261352073f`.

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
> // Package sdkserver implements Agones SDK server (sidecar).


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Preserve the quoted input/output/configuration contract and all associated source content, including ordering, encodings, command arguments and failure behavior where applicable. For a binary resource retain its bytes and source hash; do not fabricate textual quotations from it.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-78f5e45c22ca2645"></a>


### File `pkg/sdkserver/localsdk.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `244deb468775a472fea38f0780a11674d670940c`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/sdkserver/localsdk.rs`.


**Direct quotation — UPSTREAM_IMPLEMENTATION_EXCERPT; Q-c8bda6aca5f6cbb3da.** [agones/pkg/sdkserver/localsdk.go, lines 51–65](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdkserver/localsdk.go#L51-L65). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `cbf4c8e5c28237686b383a43ffced782390abe0ec214346e041ef0270fc65f89`.

> 
> func defaultGs() *sdk.GameServer {
> 	gs := &sdk.GameServer{
> 		ObjectMeta: &sdk.GameServer_ObjectMeta{
> 			Name:              "local",
> 			Namespace:         defaultNamespace,
> 			Uid:               "1234",
> 			Generation:        1,
> 			ResourceVersion:   "v1",
> 			CreationTimestamp: time.Now().Unix(),
> 			Labels:            map[string]string{"islocal": "true"},
> 			Annotations:       map[string]string{"annotation": "true"},
> 		},
> 		Spec: &sdk.GameServer_Spec{
> 			Health: &sdk.GameServer_Spec_Health{


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 39 lexically identified declarations, including `defaultNamespace` (L50), `defaultGs` (L52), `LocalSDKServer` (L94), `NewLocalSDKServer` (L114), `GenerateUID` (L187), `SetTestMode` (L197), `SetExpectedSequence` (L203), `SetSdkName` (L208), `recordRequest` (L214), `recordRequestWithValue` (L227), `updateState` (L259), `Ready` (L265), `Allocate` (L279), `Shutdown` (L292), `Health` (L304), `SetLabel` (L320), `SetAnnotation` (L339), `GetGameServer` (L358), `WatchGameServer` (L367), `Reserve` (L396), `resetReserveAfter` (L413), `stopReserveTimer` (L425), `PlayerConnect` (L435), `PlayerDisconnect` (L467). The full declaration list is recorded under `agones:pkg/sdkserver/localsdk.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/sdk`, `agones.dev/agones/pkg/sdk/alpha`, `agones.dev/agones/pkg/sdk/beta`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve channel/select outcomes and blocking/cancellation semantics; do not turn nondeterminism into fixed ordering.

7. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

8. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-5665f774353b5267"></a>


### File `pkg/sdkserver/sdk.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `1a04200599f4439679dc6c2bff362f4ea092fd5d`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/sdkserver/sdk.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-54fe88605133faa79c.** [agones/pkg/sdkserver/sdk.go, lines 28–42](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdkserver/sdk.go#L28-L42). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `00a33a2d191bdd019d73e0feb251e63369155fabcfb26f5faca440d4d01fd5ca`.

> 
> // convert converts a K8s GameServer object, into a gRPC SDK GameServer object
> func convert(gs *agonesv1.GameServer) *sdk.GameServer {
> 	meta := gs.ObjectMeta
> 	status := gs.Status
> 	health := gs.Spec.Health
> 	result := &sdk.GameServer{
> 		ObjectMeta: &sdk.GameServer_ObjectMeta{
> 			Name:              meta.Name,
> 			Namespace:         meta.Namespace,
> 			Uid:               string(meta.UID),
> 			ResourceVersion:   meta.ResourceVersion,
> 			Generation:        meta.Generation,
> 			CreationTimestamp: meta.CreationTimestamp.Unix(),
> 			Annotations:       meta.Annotations,


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 1 lexically identified declarations, including `convert` (L30). The full declaration list is recorded under `agones:pkg/sdkserver/sdk.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/sdk`, `agones.dev/agones/pkg/util/runtime`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

5. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.


<a id="file-23b86070f0a8e661"></a>


### File `pkg/sdkserver/sdkserver.go`


**Source:** `agones` at `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; Git blob `889f5661a94ff118202d0f33396edd721a0cd142`. **Disposition:** `HANDWRITTEN_PRODUCTION`. **Destination:** `agones-rs/pkg/sdkserver/sdkserver.rs`.


**Direct quotation — UPSTREAM_COMMENT_AND_IMPLEMENTATION_EXCERPT; Q-ae528891793aa63916.** [agones/pkg/sdkserver/sdkserver.go, lines 63–77](https://github.com/QuasarRay/agones/blob/6eb4a02588d8e7b581de187f3bc849733a78dcb2/pkg/sdkserver/sdkserver.go#L63-L77). Revision `6eb4a02588d8e7b581de187f3bc849733a78dcb2`; source SHA-256 `40ce110e204d4cae861a7c465d2a2c5c1c90a834b5722f6b33141f2246a0e7e5`.

> 
> // Operation is a synchronisation action
> type Operation string
> 
> const (
> 	updateState            Operation     = "updateState"
> 	updateLabel            Operation     = "updateLabel"
> 	updateAnnotation       Operation     = "updateAnnotation"
> 	updatePlayerCapacity   Operation     = "updatePlayerCapacity"
> 	updateConnectedPlayers Operation     = "updateConnectedPlayers"
> 	updateCounters         Operation     = "updateCounters"
> 	updateLists            Operation     = "updateLists"
> 	updatePeriod           time.Duration = time.Second
> )


**Reference boundary.** No dedicated prose for this individual file was established; its upstream comments/source and the package documentation below are the normative reference inputs. Quoted commands are source evidence, not permission to execute them.


**Derived implementation instructions.**


1. Implement the complete source-owned behavior in the exact destination, preserving initialization, branches, errors, side effects and observable data. Reuse existing implementation only through an explicit source-to-symbol/entrypoint mapping.

2. Account for 53 lexically identified declarations, including `Operation` (L65), `readHeaderTimeout` (L80), `counterUpdateRequest` (L88), `listUpdateRequest` (L99), `SDKServer` (L112), `NewSDKServer` (L153), `Run` (L246), `WaitForConnection` (L304), `syncGameServer` (L338), `updateState` (L361), `gameServer` (L447), `patchGameServer` (L465), `updateLabels` (L481), `updateAnnotations` (L505), `enqueueState` (L529), `Ready` (L541), `Allocate` (L549), `Shutdown` (L558), `Health` (L568), `SetLabel` (L585), `SetAnnotation` (L621), `GetGameServer` (L646), `WatchGameServer` (L657), `Reserve` (L678). The full declaration list is recorded under `agones:pkg/sdkserver/sdkserver.go` in the source inventory; nested/implicit behavior still requires source accounting, and this lexical list is not a complete body proof.

3. Consume the already implemented source packages `agones.dev/agones/pkg/apis/agones`, `agones.dev/agones/pkg/apis/agones/v1`, `agones.dev/agones/pkg/client/clientset/versioned`, `agones.dev/agones/pkg/client/clientset/versioned/typed/agones/v1`, `agones.dev/agones/pkg/client/informers/externalversions`, `agones.dev/agones/pkg/client/listers/agones/v1`, `agones.dev/agones/pkg/gameserverallocations`, `agones.dev/agones/pkg/sdk`, `agones.dev/agones/pkg/sdk/alpha`, `agones.dev/agones/pkg/sdk/beta`, `agones.dev/agones/pkg/util/errors`, `agones.dev/agones/pkg/util/logfields`, `agones.dev/agones/pkg/util/runtime`, `agones.dev/agones/pkg/util/workerqueue`. Keep transport/runtime effects behind CGP capabilities rather than copying concrete dependencies into portable policy.

4. Retain deferred cleanup order and its interaction with returns/errors.

5. Give asynchronous work an explicit CGP owner, cancellation path and join/abort policy.

6. Preserve channel/select outcomes and blocking/cancellation semantics; do not turn nondeterminism into fixed ordering.

7. Preserve absent, null/nil, empty, omitted and default distinctions at every observable boundary.

8. Retain key equality, duplicate-key and permitted iteration-order semantics; Python dictionary order is not implied.

9. Retain resource ownership, cancellation, shutdown and partial-failure cleanup.

10. Keep CGP AND derive-deftly central: record this file’s declaration/template, capability/provider, production context and lifecycle owner; pure helpers remain ordinary Rust inside that boundary. Use the package policy above rather than inventing per-file managers.

11. Read the resulting Rust as a Python developer: use clear operation names, short transformations and justified comprehension/iterator syntax. Preserve ownership, evaluation order and errors. Keep exact filename/case/plurality; Rust-only module glue belongs outside the source mirror.

13. Completion record: exact source/destination IDs, authored symbols and call path, template/provider ownership, remaining gaps and deferred evidence. Do not claim compilation, correctness or parity from this document or from file existence.





## Completion and deferred assurance


1. Record complete authored production paths, source mappings, CGP/derive-deftly ownership and remaining gaps in the existing canonical state. A local checkpoint is not a passing test or verified milestone.

2. During `IMPLEMENTATION_ONLY`, make ZERO attempts at Cargo check/build/test/clippy/doc, formatting campaigns, compiler/IDE/macro probes, fixtures, application runs, benchmarks, replay, Bolero/Kani, generator validation or CI. Only an already runnable generator may emit required implementation without validation hooks. A needed forbidden build is an explicit blocker.

3. Post-gate milestones require the whole-project `IMPLEMENTATION_COMPLETE_UNVERIFIED` record. Run only their required assurance; if production defects reopen implementation, close and renew the gate before further checks.

4. Record the milestone outcome and actual usage observations under the separate execution/accounting policy. Follow [alternation-policy.md](alternation-policy.md); this file does not define route-switch rules. Do not complete or replace the native Goal at this milestone boundary.
